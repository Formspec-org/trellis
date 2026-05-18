# Derivation — `verify/027-signed-acts-manifest-derivation-precondition-failure`

Starts from `export/009-signed-acts-manifest-only` — the minimal Trellis
export binding the substrate-anchored 068 signed-acts manifest with no
066 render projection, no 062 signature catalog, and no 063 intake
catalog. Forges `010-events.cbor` by appending a byte-identical copy of
the sole `wos.kernel.signature_affirmation` event to the dCBOR array,
then re-signs `000-manifest.cbor` with the recomputed
`events_digest = SHA-256(duplicated 010-events.cbor)` so the substrate's
pre-WOS manifest-binding check passes and the deriver runs.

The deriver call site
`validate_bound_signed_acts_manifest_extension`
(Rust `crates/trellis-verify-wos/src/signed_acts.rs:167-176`; Python
mirror in `verify_wos.py::_validate_signed_acts_manifest_extension`)
pre-filters `export.events` for the closed WOS signed-acts allowlist
and feeds the candidates to `derive_signed_acts_manifest_v1`. The
deriver's Wave 3 Task 2.c third rejection branch — duplicate
`(canonical_event_hash, event_type)` tuples — fires with the
byte-identical error string

```
signed-acts manifest has duplicate (canonical_event_hash, event_type) tuple for event_type wos.kernel.signature_affirmation
```

The verifier wraps that error into one `signed_acts_manifest_extension_invalid`
finding (Severity::Failure) with detail

```
signed acts manifest derivation failed: signed-acts manifest has duplicate (canonical_event_hash, event_type) tuple for event_type wos.kernel.signature_affirmation
```

Both runtimes converge on the same detail bytes; the
`check_cross_runtime_parity.py` `signed-acts-projection` gate pins
that parity via the Python verifier-vector assertion below.

Source-export choice
--------------------

Fixture 009 carries neither `trellis.export.signature-affirmations.v1`
nor `trellis.export.intake-handoffs.v1` manifest extensions, so the
WOS catalog validator's `event_by_hash` helper (which emits the
`export_events_duplicate_canonical_hash` finding kind on duplicate
canonical event hashes) is not invoked. Rust gates that call inside
`validate_signature_catalog` / `validate_intake_catalog`
(`crates/trellis-verify-wos/src/catalog.rs:66`, `:169`); Python is
aligned with Rust (`_validate_export` gates
`_index_events_by_canonical_hash` on extension presence — drift fix
co-landed with this fixture). The deriver-rejection finding is therefore
the only WOS failure that surfaces, which is the load-bearing
parity claim this fixture pins.

Substrate side-effect
---------------------

The duplicated event is absent from the fixture-009 inclusion proof
tree, so substrate reports `inclusion_proof_invalid` in
`proof_failures`. `integrity_verified` becomes false on the substrate
report and stays false in the composed WOS report. The fixture's
`first_failure_kind` is the WOS-routed
`signed_acts_manifest_extension_invalid`, so the conformance harness
(`crates/trellis-conformance/src/lib.rs::assert_verify_fixture_matches`)
dispatches through `trellis_verify_wos::verify_export_zip` and asserts on
`first_wos_failure`, which IS the deriver-rejection finding in both
runtimes.

Closes TR-CORE-180 evidence-pending subcase (d) — derivation
precondition failure — via test vector. Subcase (e) (canonical-CBOR
re-encoding failure) was deferred as structurally inert in Wave 4
commit `ad746bf` and remains so.
