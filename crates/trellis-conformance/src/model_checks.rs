// Rust guideline compliant 2026-02-21
//! Property and replay tests backing Trellis model-check evidence.

use std::collections::{BTreeMap, BTreeSet};
use std::convert::Infallible;
use std::fs;
use std::path::{Path, PathBuf};

use proptest::prelude::*;
use trellis_core::{AuthoredEvent, LedgerStore, SigningKeyMaterial, append_event};
use trellis_store_memory::MemoryStore;
use trellis_types::{AppendArtifacts, StoredEvent};

#[derive(Clone, Debug, PartialEq, Eq)]
struct CandidateSpec {
    scope: u8,
    parent_mask: u8,
    tie_breaker: u8,
    payload: u8,
    fact_group: Option<u8>,
    received_at: u8,
    worker_id: u8,
    queue_depth: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Candidate {
    id: u8,
    scope: u8,
    dependencies: Vec<u8>,
    tie_breaker: u8,
    payload: u8,
    fact_group: Option<u8>,
    accident: Accident,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Accident {
    received_at: u8,
    worker_id: u8,
    queue_depth: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CanonicalRef {
    scope: u8,
    position: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AppendCall {
    record_id: u8,
    scope: u8,
    dependencies: Vec<u8>,
    idempotency_key: u8,
    payload: u8,
    fact_group: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppendOutcome {
    Deferred,
    Accepted { canonical_ref: CanonicalRef },
    Replay { canonical_ref: CanonicalRef },
    RejectedDifferentPayload,
    RejectedConflict,
}

#[derive(Default, Debug)]
struct AppendContractModel {
    admitted_records: BTreeSet<u8>,
    next_position_by_scope: BTreeMap<u8, usize>,
    idempotency_by_scope: BTreeMap<(u8, u8), (u8, CanonicalRef)>,
    fact_values: BTreeMap<(u8, u8), u8>,
}

impl AppendContractModel {
    fn append(&mut self, call: &AppendCall) -> AppendOutcome {
        if let Some((known_payload, canonical_ref)) = self
            .idempotency_by_scope
            .get(&(call.scope, call.idempotency_key))
        {
            return if *known_payload == call.payload {
                AppendOutcome::Replay {
                    canonical_ref: *canonical_ref,
                }
            } else {
                AppendOutcome::RejectedDifferentPayload
            };
        }

        if !call
            .dependencies
            .iter()
            .all(|dependency| self.admitted_records.contains(dependency))
        {
            return AppendOutcome::Deferred;
        }

        if let Some(fact_group) = call.fact_group {
            if let Some(existing_payload) = self.fact_values.get(&(call.scope, fact_group)) {
                if *existing_payload != call.payload {
                    return AppendOutcome::RejectedConflict;
                }
            }
        }

        let position = self.next_position_by_scope.entry(call.scope).or_insert(0);
        let canonical_ref = CanonicalRef {
            scope: call.scope,
            position: *position,
        };
        *position += 1;

        self.admitted_records.insert(call.record_id);
        self.idempotency_by_scope
            .insert((call.scope, call.idempotency_key), (call.payload, canonical_ref));
        if let Some(fact_group) = call.fact_group {
            self.fact_values
                .entry((call.scope, fact_group))
                .or_insert(call.payload);
        }

        AppendOutcome::Accepted { canonical_ref }
    }
}

#[derive(Default, Debug)]
struct IndexedStore {
    events: BTreeMap<(Vec<u8>, u64), StoredEvent>,
}

impl IndexedStore {
    fn events_in_order(&self) -> Vec<StoredEvent> {
        self.events.values().cloned().collect()
    }
}

impl LedgerStore for IndexedStore {
    type Error = Infallible;

    fn append_event(&mut self, event: StoredEvent) -> Result<(), Self::Error> {
        self.events
            .insert((event.scope().to_vec(), event.sequence()), event);
        Ok(())
    }
}

fn candidate_specs_strategy() -> impl Strategy<Value = Vec<CandidateSpec>> {
    prop::collection::vec(
        (
            0u8..=2,
            any::<u8>(),
            0u8..=7,
            any::<u8>(),
            prop_oneof![Just(None), (0u8..=2).prop_map(Some)],
            any::<u8>(),
            0u8..=3,
            0u8..=15,
        )
            .prop_map(
                |(
                    scope,
                    parent_mask,
                    tie_breaker,
                    payload,
                    fact_group,
                    received_at,
                    worker_id,
                    queue_depth,
                )| CandidateSpec {
                    scope,
                    parent_mask,
                    tie_breaker,
                    payload,
                    fact_group,
                    received_at,
                    worker_id,
                    queue_depth,
                },
            ),
        1..=5,
    )
}

fn materialize_candidates(specs: &[CandidateSpec]) -> Vec<Candidate> {
    specs
        .iter()
        .enumerate()
        .map(|(index, spec)| {
            let dependency_mask = if index == 0 {
                0
            } else {
                u16::from(spec.parent_mask) & ((1u16 << index) - 1)
            };
            let dependencies = (0..index)
                .filter(|prior| dependency_mask & (1u16 << prior) != 0)
                .filter(|prior| specs[*prior].scope == spec.scope)
                .map(|prior| prior as u8)
                .collect::<Vec<_>>();
            Candidate {
                id: index as u8,
                scope: spec.scope,
                dependencies,
                tie_breaker: spec.tie_breaker,
                payload: spec.payload,
                fact_group: spec.fact_group,
                accident: Accident {
                    received_at: spec.received_at,
                    worker_id: spec.worker_id,
                    queue_depth: spec.queue_depth,
                },
            }
        })
        .collect()
}

fn canonical_order_by_scope(candidates: &[Candidate]) -> BTreeMap<u8, Vec<u8>> {
    let mut by_scope: BTreeMap<u8, Vec<&Candidate>> = BTreeMap::new();
    for candidate in candidates {
        by_scope.entry(candidate.scope).or_default().push(candidate);
    }

    let mut order = BTreeMap::new();
    for (scope, scoped_candidates) in by_scope {
        let mut remaining = scoped_candidates;
        let mut admitted = BTreeSet::new();
        let mut scoped_order = Vec::with_capacity(remaining.len());

        while !remaining.is_empty() {
            let next_index = remaining
                .iter()
                .enumerate()
                .filter(|(_, candidate)| {
                    candidate
                        .dependencies
                        .iter()
                        .all(|dependency| admitted.contains(dependency))
                })
                .min_by_key(|(_, candidate)| (candidate.tie_breaker, candidate.id))
                .map(|(index, _)| index)
                .expect("same-scope dependencies only point backward, so at least one candidate is ready");
            let next = remaining.remove(next_index);
            admitted.insert(next.id);
            scoped_order.push(next.id);
        }

        order.insert(scope, scoped_order);
    }

    order
}

fn perturb_operational_accidents(candidates: &[Candidate]) -> Vec<Candidate> {
    let mut perturbed = candidates
        .iter()
        .enumerate()
        .map(|(index, candidate)| {
            let mut clone = candidate.clone();
            clone.accident = Accident {
                received_at: candidate.accident.received_at.wrapping_add(17 + index as u8),
                worker_id: candidate.accident.worker_id.wrapping_add(3),
                queue_depth: candidate.accident.queue_depth.wrapping_add(9),
            };
            clone
        })
        .collect::<Vec<_>>();
    perturbed.reverse();
    perturbed
}

/// Visits all permutations via Heap's method — **O(n!)** in the candidate count.
///
/// Permutation count is capped by `candidate_specs_strategy` (`1..=5`
/// candidates → at most `5! = 120` permutations per proptest case).
fn visit_permutations<T: Clone>(items: &mut [T], start: usize, visitor: &mut dyn FnMut(&[T])) {
    if start == items.len() {
        visitor(items);
        return;
    }

    for index in start..items.len() {
        items.swap(start, index);
        visit_permutations(items, start + 1, visitor);
        items.swap(start, index);
    }
}

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/vectors")
}

fn append_fixture_inputs() -> (Vec<u8>, Vec<u8>) {
    let root = fixtures_root().join("append/001-minimal-inline-payload");
    let manifest: toml::Value =
        toml::from_str(&fs::read_to_string(root.join("manifest.toml")).unwrap()).unwrap();
    let inputs = manifest.get("inputs").and_then(toml::Value::as_table).unwrap();
    let authored_event = fs::read(root.join(inputs["authored_event"].as_str().unwrap())).unwrap();
    let signing_key = fs::read(root.join(inputs["signing_key"].as_str().unwrap())).unwrap();
    (authored_event, signing_key)
}

fn replay_append_fixture_with_store<S: LedgerStore>(
    store: &mut S,
) -> Result<AppendArtifacts, trellis_core::AppendError> {
    let (authored_event, signing_key) = append_fixture_inputs();
    append_event(
        store,
        &SigningKeyMaterial::new(signing_key),
        &AuthoredEvent::new(authored_event),
    )
}

fn same_ref(outcome: AppendOutcome, expected: CanonicalRef) {
    match outcome {
        AppendOutcome::Accepted { canonical_ref } | AppendOutcome::Replay { canonical_ref } => {
            assert_eq!(canonical_ref, expected);
        }
        other => panic!("expected an attested outcome, got {other:?}"),
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn tr_core_020_single_canonical_order_per_scope(specs in candidate_specs_strategy()) {
        let candidates = materialize_candidates(&specs);
        let baseline = canonical_order_by_scope(&candidates);
        let mut permutations = candidates.clone();
        visit_permutations(&mut permutations, 0, &mut |candidate_order| {
            assert_eq!(canonical_order_by_scope(candidate_order), baseline);
        });
    }

    #[test]
    fn tr_core_023_order_is_independent_of_operational_accidents(specs in candidate_specs_strategy()) {
        let candidates = materialize_candidates(&specs);
        let baseline = canonical_order_by_scope(&candidates);
        let perturbed = perturb_operational_accidents(&candidates);
        prop_assert_eq!(canonical_order_by_scope(&perturbed), baseline);
    }

    #[test]
    fn tr_core_025_concurrency_uses_deterministic_tie_breaking(
        tie_breakers in prop::collection::vec(0u8..=7, 2..=5),
    ) {
        let candidates = tie_breakers
            .iter()
            .enumerate()
            .map(|(index, tie_breaker)| Candidate {
                id: index as u8,
                scope: 0,
                dependencies: Vec::new(),
                tie_breaker: *tie_breaker,
                payload: index as u8,
                fact_group: None,
                accident: Accident {
                    received_at: index as u8,
                    worker_id: index as u8,
                    queue_depth: index as u8,
                },
            })
            .collect::<Vec<_>>();

        let expected = candidates
            .iter()
            .map(|candidate| (candidate.tie_breaker, candidate.id))
            .collect::<Vec<_>>();
        let actual = canonical_order_by_scope(&candidates)
            .remove(&0)
            .unwrap()
            .into_iter()
            .map(|id| {
                let candidate = candidates.iter().find(|candidate| candidate.id == id).unwrap();
                (candidate.tie_breaker, candidate.id)
            })
            .collect::<Vec<_>>();

        let mut sorted_expected = expected;
        sorted_expected.sort();
        prop_assert_eq!(actual, sorted_expected);
    }

    #[test]
    fn tr_core_046_prerequisites_gate_attestation(
        scope in 0u8..=2,
        parent_count in 1usize..=4,
        child_payload in any::<u8>(),
        child_key in any::<u8>(),
    ) {
        let mut model = AppendContractModel::default();
        let child_record_id = parent_count as u8;
        let child_dependencies = (0..parent_count as u8).collect::<Vec<_>>();
        let child = AppendCall {
            record_id: child_record_id,
            scope,
            dependencies: child_dependencies,
            idempotency_key: child_key,
            payload: child_payload,
            fact_group: None,
        };

        prop_assert_eq!(model.append(&child), AppendOutcome::Deferred);

        for parent_index in 0..parent_count {
            let parent = AppendCall {
                record_id: parent_index as u8,
                scope,
                dependencies: Vec::new(),
                idempotency_key: child_key.wrapping_add(parent_index as u8).wrapping_add(1),
                payload: child_payload.wrapping_add(parent_index as u8),
                fact_group: None,
            };
            assert!(matches!(
                model.append(&parent),
                AppendOutcome::Accepted { canonical_ref }
                if canonical_ref.position == parent_index
            ));
        }

        assert!(matches!(
            model.append(&child),
            AppendOutcome::Accepted { canonical_ref }
            if canonical_ref.position == parent_count
        ));
    }

    #[test]
    fn tr_core_050_idempotency_keys_are_stable_across_retries(
        scope in 0u8..=2,
        idempotency_key in any::<u8>(),
        payload in any::<u8>(),
        alternate_payload in any::<u8>(),
        unrelated_count in 0usize..=3,
    ) {
        prop_assume!(payload != alternate_payload);

        let mut model = AppendContractModel::default();
        let first = model.append(&AppendCall {
            record_id: 0,
            scope,
            dependencies: Vec::new(),
            idempotency_key,
            payload,
            fact_group: None,
        });
        let canonical_ref = match first {
            AppendOutcome::Accepted { canonical_ref } => canonical_ref,
            other => panic!("expected first append to admit, got {other:?}"),
        };

        same_ref(
            model.append(&AppendCall {
                record_id: 1,
                scope,
                dependencies: Vec::new(),
                idempotency_key,
                payload,
                fact_group: None,
            }),
            canonical_ref,
        );

        for unrelated_index in 0..unrelated_count {
            let unrelated = AppendCall {
                record_id: (10 + unrelated_index) as u8,
                scope,
                dependencies: Vec::new(),
                idempotency_key: idempotency_key.wrapping_add(unrelated_index as u8).wrapping_add(1),
                payload: payload.wrapping_add(unrelated_index as u8).wrapping_add(1),
                fact_group: None,
            };
            assert!(matches!(
                model.append(&unrelated),
                AppendOutcome::Accepted { .. } | AppendOutcome::Replay { .. }
            ));
        }

        same_ref(
            model.append(&AppendCall {
                record_id: 2,
                scope,
                dependencies: Vec::new(),
                idempotency_key,
                payload,
                fact_group: None,
            }),
            canonical_ref,
        );
        prop_assert_eq!(
            model.append(&AppendCall {
                record_id: 3,
                scope,
                dependencies: Vec::new(),
                idempotency_key,
                payload: alternate_payload,
                fact_group: None,
            }),
            AppendOutcome::RejectedDifferentPayload
        );
    }

    #[test]
    fn tr_op_061_conflicts_stay_scoped_to_affected_facts(
        scope_a in 0u8..=1,
        fact_group in 0u8..=2,
        initial_payload in any::<u8>(),
        conflicting_payload in any::<u8>(),
    ) {
        prop_assume!(initial_payload != conflicting_payload);

        let scope_b = scope_a + 1;
        let mut model = AppendContractModel::default();

        assert!(matches!(
            model.append(&AppendCall {
                record_id: 0,
                scope: scope_a,
                dependencies: Vec::new(),
                idempotency_key: 1,
                payload: initial_payload,
                fact_group: Some(fact_group),
            }),
            AppendOutcome::Accepted { canonical_ref }
            if canonical_ref.position == 0
        ));
        prop_assert_eq!(
            model.append(&AppendCall {
                record_id: 1,
                scope: scope_a,
                dependencies: Vec::new(),
                idempotency_key: 2,
                payload: conflicting_payload,
                fact_group: Some(fact_group),
            }),
            AppendOutcome::RejectedConflict
        );
        assert!(matches!(
            model.append(&AppendCall {
                record_id: 2,
                scope: scope_b,
                dependencies: Vec::new(),
                idempotency_key: 3,
                payload: conflicting_payload,
                fact_group: Some(fact_group),
            }),
            AppendOutcome::Accepted { canonical_ref }
            if canonical_ref.position == 0
        ));
        assert!(matches!(
            model.append(&AppendCall {
                record_id: 3,
                scope: scope_a,
                dependencies: Vec::new(),
                idempotency_key: 4,
                payload: conflicting_payload,
                fact_group: Some(fact_group.wrapping_add(1)),
            }),
            AppendOutcome::Accepted { canonical_ref }
            if canonical_ref.position == 1
        ));
    }
}

/// Locks byte-identical append replay for the committed `append/001` fixture
/// across the in-memory `MemoryStore` and `IndexedStore` harnesses (same
/// canonical artifacts and persisted event ordering). This does not exercise
/// distinct storage backends beyond those two adapters.
#[test]
fn tr_core_001_append_fixture_replay_is_identical_across_memory_and_indexed_stores() {
    let mut memory_store = MemoryStore::new();
    let mut indexed_store = IndexedStore::default();

    let memory_artifacts = replay_append_fixture_with_store(&mut memory_store).unwrap();
    let indexed_artifacts = replay_append_fixture_with_store(&mut indexed_store).unwrap();

    assert_eq!(memory_artifacts, indexed_artifacts);
    assert_eq!(memory_store.events(), indexed_store.events_in_order().as_slice());
}

#[test]
fn tr_op_111_replay_and_property_battery_are_live() {
    let candidates = materialize_candidates(&[
        CandidateSpec {
            scope: 0,
            parent_mask: 0,
            tie_breaker: 2,
            payload: 10,
            fact_group: None,
            received_at: 9,
            worker_id: 1,
            queue_depth: 4,
        },
        CandidateSpec {
            scope: 0,
            parent_mask: 0,
            tie_breaker: 1,
            payload: 11,
            fact_group: None,
            received_at: 1,
            worker_id: 3,
            queue_depth: 2,
        },
    ]);
    assert_eq!(canonical_order_by_scope(&candidates).get(&0).cloned(), Some(vec![1, 0]));

    let mut memory_store = MemoryStore::new();
    let mut indexed_store = IndexedStore::default();
    let memory_artifacts = replay_append_fixture_with_store(&mut memory_store).unwrap();
    let indexed_artifacts = replay_append_fixture_with_store(&mut indexed_store).unwrap();

    assert_eq!(memory_artifacts, indexed_artifacts);
    assert_eq!(memory_store.events(), indexed_store.events_in_order().as_slice());
}
