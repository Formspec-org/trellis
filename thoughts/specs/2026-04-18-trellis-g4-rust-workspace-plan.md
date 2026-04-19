# Trellis G-4 Rust Reference Implementation — Workspace Plan

**Date:** 2026-04-18
**Scope:** Cargo workspace layout, crate split, public API shape, milestone sequencing, and fixture-runner contract for the G-4 Rust reference implementation of Trellis Phase 1.
**Closes:** G-4 (design). The first milestone byte-matches `append/001-minimal-inline-payload`; the second milestone byte-matches the full G-3 corpus once it freezes.
**Unblocks:** G-5 (stranger-test second implementation) — the Rust impl is G-5's negative space; it MUST be unreadable to the stranger implementor.
**Does not cover:** CLI surface, WASM bindings (both in Track A step 8 per `thoughts/product-vision.md`), Phase 2 runtime-integrity library, or G-5 execution itself. Those are separate plans.

## Context

Ratification checklist G-4 pins seven crate names: `trellis-core`, `trellis-cose`, `trellis-store-postgres`, `trellis-store-memory`, `trellis-verify`, `trellis-cli`, `trellis-conformance`. Public API is three functions: `append`, `verify`, `export`. Byte-match on every vector closes the gate.

The G-3 corpus is partial at plan-authoring time — `append/001-minimal-inline-payload` is committed; ~49 vectors remain. G-4's critical-path per `TODO.md` is:

1. Workspace scaffold + `append/001` byte-match (Milestone 1).
2. Full-corpus byte-match (Milestone 2), gated on G-3 close.

Milestone 1 cannot wait for the corpus. The plan treats M1 as a standalone deliverable that proves the architecture works against one worked example, then scales to the corpus in M2 without reshaping the crate graph.

Normative inputs this plan consumes without re-stating: Core §5 (dCBOR), §6 (event format), §7 (COSE_Sign1/Ed25519), §8 (signing-key registry), §9 (hash construction + §9.8 domain-tag registry), §10 (chain), §11 (checkpoint), §15 (snapshot/watermark), §18 (export ZIP), §19 (verification algorithm + `PostureTransitionOutcome`), §28 (CDDL Appendix A). Companion §10/§19 for posture-transition and delegated-compute structural recognition.

The plan is RFC-terse. Rationale belongs here; declarative "MUST" text belongs in Core. Where the two disagree, Core wins and this plan amends.

## Crate graph

Seven crates as pinned, plus two internal-only utility crates to keep the public ones cohesive. Layering is strict: no same-layer or upward dependencies, no cycles. The workspace compiles bottom-up.

```
Layer 0   trellis-cddl           (internal)      dCBOR encoder/decoder + CDDL-validated types
Layer 0   trellis-cose                           COSE_Sign1 Ed25519 profile (§7)
Layer 1   trellis-types          (internal)      Core CDDL structs + domain-tag constants (§9.8)
Layer 2   trellis-core                           append + hash construction + chain + checkpoint
Layer 2   trellis-verify                         verification algorithm (§19) incl. PostureTransitionOutcome
Layer 2   trellis-export         (internal or crate) deterministic ZIP writer (§18)
Layer 3   trellis-store-memory                   in-proc ledger store
Layer 3   trellis-store-postgres                 pg-backed ledger store
Layer 4   trellis-conformance    (internal)      fixture runner walking fixtures/vectors/
Layer 4   trellis-cli                            stub (deferred to Track A step 8 — scaffold only)
```

### Why this graph

- **`trellis-cddl` separate from `trellis-types`.** The encoder/decoder is crypto-adjacent plumbing; the typed structs are the spec surface. Mixing them makes it impossible to swap dCBOR library later (e.g., if `ciborium` diverges from dCBOR strict-mode requirements). `trellis-cddl` owns encoder discipline; `trellis-types` owns the struct shapes and can be regenerated from the CDDL.
- **`trellis-cose` at Layer 0, not Layer 1.** It depends only on crypto crates, not on Trellis types. Event-level signing composes COSE + types at Layer 2. Keeping COSE independent lets a `trellis-core` refactor leave it untouched.
- **`trellis-verify` separate from `trellis-core`.** The append path and the verify path share types but not code paths. A verifier is frequently embedded in contexts (export consumers, auditors) that do not need the append path. Splitting lets a caller depend on `trellis-verify` + `trellis-cddl` alone. Shared helpers (hash preimage construction) live in `trellis-types`.
- **`trellis-export` as its own crate.** Deterministic ZIP discipline (§18) is load-bearing and narrow. Embedding it in `trellis-core` couples append to ZIP concerns it does not own. Public or internal? Internal for now — no Phase-1 caller needs it apart from the three-function façade. Promote to public if Phase 2 agency-log code needs it.
- **`trellis-store-memory` + `trellis-store-postgres` as separate crates, not a backend trait in `trellis-core`.**

  **Tradeoff considered:** fold both into one `trellis-store` crate with a `LedgerStore` trait. Fewer crates, less Cargo.toml noise, one dependency for callers.

  **Decision:** keep them separate, but expose the trait from `trellis-core` (not from a separate `trellis-store` crate). Rationale: (a) Postgres pulls `tokio-postgres`, `deadpool-postgres`, or `sqlx` — heavy deps that an in-memory or WASM embedding must not inherit; (b) Cargo feature flags on one combined crate would create a combinatorial feature matrix and conditional compilation pain; (c) the G-4 ratification language names both crates by hand, so keeping them split matches the spec-visible surface. The trait lives next to append logic in `trellis-core::store`; each backend crate is a pure implementation of that trait with no reverse dependency.

- **`trellis-conformance` and `trellis-cli` at Layer 4.** Both are terminal consumers. `trellis-conformance` is fixture-runner plumbing, not a library other crates link against. `trellis-cli` is scaffold-only for G-4 (a `main.rs` that calls `trellis-core`'s three façade functions and prints); the rich CLI (argument surface, WASM bindings) is Track A step 8.

### Publishable vs internal

| Crate | Publishable | Rationale |
|---|---|---|
| `trellis-cddl` | internal | No independent consumer. Keeping it internal avoids committing to a dCBOR public API before we know what Phase 2 needs. |
| `trellis-types` | internal | Struct shapes are Core CDDL; if an external consumer needs them they should depend on the façade. |
| `trellis-cose` | **yes** | Has external reuse value — generic COSE_Sign1 Ed25519 is not Trellis-specific. |
| `trellis-core` | **yes** | The façade. Re-exports `append` and public types. |
| `trellis-verify` | **yes** | Re-exports `verify` and `VerificationReport`. A verifier-only consumer can depend on this + `trellis-cddl` without pulling store backends. |
| `trellis-export` | internal initially; **publishable** if Phase 2 needs it | Keeps the deterministic-ZIP invariants in one place. |
| `trellis-store-memory` | **yes** | Useful for tests and light embeds. |
| `trellis-store-postgres` | **yes** | Production backend. |
| `trellis-conformance` | internal | Binds to on-disk `fixtures/vectors/` layout in this repo. Not a library for anyone else. |
| `trellis-cli` | internal for G-4; publishable in Track A step 8 | Current scope is façade exercise only. |

## Public API contract

Three functions per §19/§10/§18. All live in `trellis-core` and re-export from `trellis-verify` where appropriate.

### `append`

```rust
// (comment-only signature; not code)
// pub fn append(
//     store: &mut impl LedgerStore,
//     signing_key: &SigningKey,
//     registry: &RegistryBinding,
//     authored_event: AuthoredEvent,
// ) -> Result<AppendHead, AppendError>;
```

- Takes a mutable store (async wrappers — see below), a signing key resolved via `kid`, the bound registry snapshot (§14), and the authored event surface (§6.8).
- Returns `AppendHead` per §10.6 — `{ scope, sequence, canonical_event_hash, signed_event_cid, append_head_digest }`.
- Failure kinds per Core §6/§10: `IdempotencyCollision`, `PrevHashMismatch`, `ScopeMismatch`, `SignatureFailed`, `StoreError(…)`.

### `verify`

```rust
// pub fn verify(
//     export: &ExportPackage,
//     policy: VerifyPolicy,
// ) -> VerificationReport;
```

- Infallible return — all failures surface in the report, not as `Err`. This matches §19.2 ("MUST NOT silently skip checks") and §19's three-boolean discipline: no Rust-level panic erases the structured report.
- `ExportPackage` is constructed by parsing a ZIP (via `trellis-export`) or by direct construction for in-memory testing.
- `VerificationReport` shape is pinned per Core §19:

  ```rust
  // pub struct VerificationReport {
  //     pub structure_verified: bool,
  //     pub integrity_verified: bool,
  //     pub readability_verified: bool,
  //     pub event_failures: Vec<VerificationFailure>,
  //     pub checkpoint_failures: Vec<VerificationFailure>,
  //     pub proof_failures: Vec<VerificationFailure>,
  //     pub posture_transitions: Vec<PostureTransitionOutcome>,
  //     pub omitted_payload_checks: Vec<OmittedPayloadCheck>,
  //     pub warnings: Vec<String>,
  // }
  //
  // pub struct PostureTransitionOutcome {
  //     pub transition_id: String,
  //     pub kind: PostureTransitionKind,    // "custody-model" | "disclosure-profile"
  //     pub event_index: u64,
  //     pub from_state: String,
  //     pub to_state: String,
  //     pub continuity_verified: bool,
  //     pub declaration_resolved: bool,
  //     pub attestations_verified: bool,
  //     pub failures: Vec<String>,
  // }
  ```

- `VerifyPolicy` is the Rust-side knob for strict-superset behavior: does this verifier reject unknown top-level fields (Phase 1 normative per C-5) or tolerate them (Phase 2 forward-compat mode)? G-4 default: reject. See Open Items.

### `export`

```rust
// pub fn export(
//     store: &impl LedgerStore,
//     scope: &LedgerScope,
//     policy: ExportPolicy,
// ) -> Result<ExportPackage, ExportError>;
```

- Read-only on the store.
- Returns a logical `ExportPackage` plus, via `ExportPackage::to_zip_bytes(&self) -> Vec<u8>`, the deterministic ZIP per §18.
- `ExportPolicy` controls whether ciphertext payloads are included (affects `readability_verified` on the consumer side).

### Async vs sync

**Decision: sync core, async-shim at the store boundary.**

- `trellis-core`, `trellis-verify`, `trellis-export`, `trellis-cose`, `trellis-cddl`, `trellis-types` are all synchronous. No executor dependency, no `async fn` in public APIs, no colored-function friction. These crates are pure transform logic — bytes in, bytes out — and should run identically in a blocking server, a background thread, or a WASM module.
- `LedgerStore` trait has a sync base (`fn get_head`, `fn append_event`, etc.) that `trellis-store-memory` implements directly. `trellis-store-postgres` exposes both sync (blocking via `tokio::runtime::Handle::block_on` internally) and async variants; the async variant uses a parallel `AsyncLedgerStore` trait. Call sites that want async pair it with an async façade, but `trellis-core::append` itself does not need to be async — it operates on a `&mut impl LedgerStore`.

Rationale: async propagates. Making `append` async forces every caller (fixture runner, CLI, future WASM binding) to be async. The store is the only place real I/O happens; isolate async there.

### no_std feasibility

**Decision: `trellis-cddl`, `trellis-cose`, `trellis-types`, `trellis-verify` target `no_std` + `alloc`. `trellis-core` (append) and `trellis-export` are `std`. Stores are `std`.**

Motivation: a WASM browser-side verifier (Track A step 8) benefits from a no_std verify path. Append and export require heavier machinery (deterministic ZIP streaming, store I/O) that does not fit the no_std envelope cleanly. Split on the verify/append seam.

## Dependency policy

### Allowed external crates

| Crate | Used for | Notes |
|---|---|---|
| `ed25519-dalek` | Ed25519 signing/verification | RustCrypto. Pin to ≥2.1. |
| `coset` | COSE_Sign1 encode/decode | Google-maintained; integers-first header labels line up with Core §7.4's pinned labels. |
| `ciborium` | dCBOR encode/decode | Standards-compliant CBOR. `trellis-cddl` wraps it with dCBOR strict-mode discipline (§5.1). |
| `hpke` | HPKE wrap (key-bag entries, §9.4) | RustCrypto. |
| `sha2` | SHA-256 for domain-tagged hashes (§9) | RustCrypto. |
| `zip` | Deterministic ZIP writer (§18) | Must support stored (no-compression) mode and explicit member ordering. If `zip` crate's LFH fields aren't pinnable, drop it and hand-write the archive — the spec pins local-file-header fields. |
| `tokio-postgres` or `sqlx` | Postgres store | Store-crate only. |
| `serde`, `serde_bytes` | CDDL struct derive | Only in `trellis-types`. |
| `thiserror` | Error derive | All crates. |
| `proptest` | Property tests | dev-dep in `trellis-cddl`, `trellis-core`. |

### Forbidden

- **No Trellis-derived crates for cryptographic primitives.** Per fixture-system design's generator rule (`_generator/` allowed-imports list), the Rust impl must not consume Trellis-derived code as a building block. This means no internal "Trellis CBOR" fork that the generator also uses — the test harness and the impl must independently reach the same bytes. `ciborium` and `coset` satisfy this; a shared-with-generator internal crate does not.
- **No hand-rolled crypto.** Ed25519, SHA-256, HPKE come from RustCrypto or equivalent audited libraries. No Bouncy-Rust equivalents, no "simplified" implementations.
- **No async runtime in Layer 0–2.** `tokio` may appear only in `trellis-store-postgres` and `trellis-cli`.
- **No `serde_cbor`.** Abandoned upstream. Use `ciborium`.
- **No unsafe outside `trellis-cddl` low-level encoder paths, and even there only when benched necessary.**

## Milestone split

### M1 — byte-match `append/001-minimal-inline-payload`

Six runnable sub-milestones. Each has a failing test (the vector's expected bytes) that the work makes pass. Each commits independently.

#### M1.a — CDDL + dCBOR

Deliverable: `trellis-cddl` encodes/decodes the minimal event structs from `trellis-types` in dCBOR and round-trips byte-for-byte against `expected-event-payload.cbor`. A property test generates random `EventPayload` structs and confirms encode→decode→encode yields a fixed point. First failing test: load `expected-event-payload.cbor`, decode to `EventPayload`, re-encode, assert byte equality.

#### M1.b — COSE_Sign1 signing/verification

Deliverable: `trellis-cose` produces a COSE_Sign1 structure with pinned protected-header integer labels (§7.4) and Ed25519 signature matching `sig-structure.bin` and `expected-event.cbor` from the vector. First failing test: construct the Sig_structure from the vector's inputs, compare bytes to `sig-structure.bin`; sign with the pinned Ed25519 key from `_keys/issuer-001.cose_key`, compare signed bytes to `expected-event.cbor`.

#### M1.c — Hash construction + domain separation

Deliverable: `trellis-types` exports domain-tag constants from §9.8 (`trellis-event-v1`, `trellis-content-v1`, `trellis-checkpoint-v1`, `trellis-export-manifest-v1`, `trellis-posture-declaration-v1`, `trellis-transition-attestation-v1`); `trellis-core::hash` produces `author_event_hash` bytes matching `author-event-hash.bin`. First failing test: load `author-event-preimage.bin`, hash with domain tag per §9.5, compare to `author-event-hash.bin`.

#### M1.d — Event construction (append path skeleton)

Deliverable: `trellis-core::append` — called with the vector's inputs — emits all three event surfaces (authored / canonical / signed) matching the vector's expected bytes end-to-end. First failing test: call `append(&mut MemoryStore::new(), &signing_key, &registry, authored_event)`; assert the returned `AppendHead` matches `expected-append-head.cbor` and the store contains an event whose canonical encoding matches `expected-event-payload.cbor`.

#### M1.e — Verification algorithm (single-event path)

Deliverable: `trellis-verify::verify` — given an `ExportPackage` containing the single event from `append/001` — returns `VerificationReport { structure_verified: true, integrity_verified: true, readability_verified: true, .. }`. The full §19 algorithm is not yet implemented — steps that do not apply to single-event, no-posture-transition, no-checkpoint exports short-circuit. First failing test: construct an `ExportPackage` from the vector's bytes; verify; assert all three booleans true and all failure arrays empty.

#### M1.f — Export ZIP layout

Deliverable: `trellis-export::to_zip_bytes` produces a ZIP matching the §18 layout for a minimal single-event export. `append/001` does not ship an expected ZIP today; this sub-milestone is gated on the first `export/*` fixture landing. Until then, the deliverable is structural: `ExportPackage::to_zip_bytes` produces a ZIP that parses back to the original `ExportPackage` via the inverse path, and the ZIP bytes are reproducible across runs (two calls produce byte-identical output). First failing test: build an `ExportPackage` in memory, write to ZIP twice, assert byte equality.

**M1 closure:** all six sub-milestones green + the full `trellis-conformance` runner (see below) executes `append/001` and passes. At this point the workspace compiles, the API surface is real, and the first vector byte-matches. One vector does not close G-4 — M2 does.

### M2 — full corpus byte-match (G-4 closure)

No new sub-milestones; this is execution discipline. As each G-3 vector batch lands (`append/002..005` + first tamper, then `append/` residue, then `verify/`, `export/`, expanded `tamper/`), the Rust impl runs the fixture runner, finds the new vector(s), fails, implements the missing path, lands a commit. M2 is complete when:

- The fixture runner walks `fixtures/vectors/` and returns zero failures.
- `_pending-invariants.toml` is empty (every byte-testable invariant has a green vector; the G-3 lint and the conformance runner agree).
- `trellis-cli` scaffold compiles and emits the three façade outputs for a worked example.

M2 is explicitly expected to surface Core amendments. Each amendment is tracked like any other G-3-surfaced gap (see `thoughts/specs/2026-04-18-trellis-core-gaps-surfaced-by-g3.md`).

## Fixture runner

`trellis-conformance` is the Rust half of G-3's allowlist discipline. It walks `fixtures/vectors/{append,verify,export,tamper}/*/` (ignoring `_`-prefixed scaffolding per the fixture-system design), parses each `manifest.toml`, dispatches on `op`, and byte-compares outputs.

Responsibilities:

1. **Walk.** Use `walkdir` or equivalent; skip `_`-prefixed entries; skip vectors with `status = "deprecated"` per F6 (honored by reading the manifest, not by directory inspection).
2. **Parse.** `toml` crate to read manifest; resolve sibling paths and `../../_keys`, `../../_inputs` relative references per the fixture-system design.
3. **Dispatch.** One dispatch function per `op`:
   - `append` → call `trellis-core::append`, byte-compare `canonical_event`, `signed_event`, `append_head` against declared expected siblings (manifest `[expected]` map).
   - `verify` → call `trellis-verify::verify`, compare `structure_verified` / `integrity_verified` / `readability_verified` against `[expected.report]`.
   - `export` → call `trellis-core::export`, byte-compare the ZIP bytes against the expected ZIP.
   - `tamper` → same as verify, but expects at least one `*_verified = false` and a matching `tamper_kind` / `failing_event_id`.
4. **Fail-fast on pending allowlist drift.** Read `fixtures/vectors/_pending-invariants.toml`. If any row listed as pending is now covered by a passing vector, the runner MUST fail — the entry must be removed from the allowlist in the same commit that adds the vector. This is the Rust-side mirror of the G-3 lint's allowlist rule: the lint checks the spec is honest about what is not yet covered; the runner checks the allowlist is honest about what has now been covered. Together they trap both drift directions.
5. **Report.** Per-vector pass/fail, with byte-diff hex output on mismatch (first 64 bytes before and after the divergence). A reviewer diagnosing a byte mismatch should not need to write their own hex differ.

The runner is a library crate + an integration-test harness, not a binary. `cargo test -p trellis-conformance` is the full-corpus run. A CLI subcommand `trellis conformance run` is deferred to Track A step 8.

### Edge: vectors that exercise sparse-only behavior

Some vectors (e.g., `append/004-hpke-wrapped-inline`) require HPKE primitives. If the corresponding code path is not yet implemented, the runner MUST fail the vector (not skip), and the operator removes the failure by implementing the path — never by skipping the vector. `status = "deprecated"` is the only skip mechanism; there is no `ignore` or `xfail` in manifests.

## Stranger-test isolation

G-5 requires an independent implementor who has read only Core + Companion + Agreement to produce byte-identical output on every vector. The Rust impl MUST be structurally unreadable by that implementor.

### Directory layout

Place the Rust workspace **outside** the `specs/` subtree. Proposed:

```
trellis/
├── specs/              # what G-5 reads
├── fixtures/           # what G-5's impl tests against
├── ratification/       # what G-5 targets
├── rust/               # G-4 Rust workspace (THIS plan's output)
│   ├── Cargo.toml      # workspace root
│   ├── crates/
│   │   ├── trellis-cddl/
│   │   ├── trellis-cose/
│   │   ├── trellis-types/
│   │   ├── trellis-core/
│   │   ├── trellis-verify/
│   │   ├── trellis-export/
│   │   ├── trellis-store-memory/
│   │   ├── trellis-store-postgres/
│   │   ├── trellis-conformance/
│   │   └── trellis-cli/
│   └── target/         # gitignored
└── thoughts/           # not in the G-5 read set anyway
```

Rationale: `specs/` is the read set. `fixtures/` is the interop target and necessarily readable. Everything else is isolation discipline. `rust/` is a sibling of `specs/`, not a child — a stranger told "read only `specs/`" has no reason to wander into `rust/`, and tooling that enumerates the read set can whitelist `specs/` + `fixtures/` without leakage.

### Enforcement

- **Documentation-level.** `ratification/stranger-test-brief.md` (future, not in this plan) tells the implementor exactly what they may read: `specs/`, `ratification/`, `fixtures/vectors/**` (except `_generator/`). Explicitly excludes `rust/`, `thoughts/`, `fixtures/vectors/_generator/`.
- **Tooling-level.** Repo-scoped agent context files (e.g., `CLAUDE.md` for Claude Code sessions, or an equivalent `.g5-readlist` for a human with a grep discipline) encode the allowed set. G-5 session has a pre-commit hook that rejects diffs touching `rust/` by the stranger author.
- **Review-level.** G-5 PRs are reviewed by someone who can confirm the stranger's implementation does not recapitulate Rust-side idioms — bytes are the only evidence, but "independent derivation" is the stronger claim.

Not in this plan: actually writing that brief. It is a G-5 prerequisite; this plan names the discipline.

## Testing strategy

Layered per the CLAUDE.md testing philosophy: one well-chosen test at the right layer beats ten redundant ones.

### Unit tests (per crate, Rust-native)

- `trellis-cddl` — dCBOR encode/decode round-trip; deterministic-ordering property tests; malformed-input failure modes.
- `trellis-cose` — Sig_structure preimage correctness; pinned integer label placement (§7.4); Ed25519 sign/verify known-answer tests from RFC 8032 appendix A.
- `trellis-types` — struct field presence (compile-time via derive); domain-tag constant values.
- `trellis-core::hash` — hash preimage construction for each domain tag; length-prefix discipline (§9.1).
- `trellis-core::append` — idempotency behavior (§6.1); `prev_hash` chaining (§10.2); scope validation.
- `trellis-verify` — each `*_verified` boolean flipped in isolation by a constructed input; `PostureTransitionOutcome` emission for each failure mode named in §19.
- `trellis-export` — ZIP reproducibility (two runs byte-equal); prefix-ordered filenames (§18).

**Which are normative (must pass for G-4 closure):** CDDL round-trip, Sig_structure bytes, hash preimage bytes, idempotency, `prev_hash`, ZIP reproducibility. These correspond directly to Core MUSTs.

**Which are recommended (hygiene):** property-test coverage, RFC 8032 KATs (redundant with `ed25519-dalek`'s own tests but cheap).

### Integration tests (in `trellis-conformance`)

The fixture-runner integration is the load-bearing test. Every vector is a test case. Passing the corpus = G-4 closed.

### Property tests (`proptest`)

- `trellis-cddl`: encode→decode→encode fixed-point.
- `trellis-core::hash`: length-prefix discipline holds under arbitrary byte lengths including 0 and `u64::MAX`.
- `trellis-export`: ZIP reproducibility under arbitrary member orderings that normalize to the same prefix order.

Property tests are recommended, not normative. They catch regressions in boundaries the fixtures don't reach.

### CI discipline

- `cargo test --workspace` green.
- `cargo fmt --check`, `cargo clippy -- -D warnings`.
- `cargo deny check` against the allowed external-crate list; a dependency-graph change requires a plan update.
- Fixture-runner pass against the full corpus.
- Repo-level `python3 scripts/check-specs.py` unaffected by Rust changes — the G-3 lint and the Rust runner are independent halves of the allowlist discipline.

## Non-goals

- **CLI argument surface.** `trellis-cli` in M1 is a scaffold calling the three façade functions for a canned input. Rich CLI (subcommands, flags, human-friendly output, stdin streaming) is Track A step 8.
- **WASM bindings.** Same — Track A step 8. The no_std split in the crate graph leaves the door open; building the bindings is out of scope.
- **Phase 2 runtime-integrity library.** This impl is not designed as a shared runtime-integrity component for WOS + Formspec embedding. Phase 1 closure first; Phase 2 reshape later.
- **Phase 2 extension key acceptance.** Verifier rejects unknown top-level fields per C-5 (Phase 1 normative). Strict-superset verification of Phase 2 bytes is a forward-compat mode we may add later but not now — see Open Items.
- **Commissioning the G-5 stranger impl.** Out of scope; separate plan. This plan only establishes the isolation discipline.
- **Performance tuning.** §19.3 names an engineering target (1M events / laptop / 60s). M1 and M2 do not instrument or tune for it. Add a bench harness in a follow-on.
- **Database migration tooling for `trellis-store-postgres`.** Schema ships as SQL files under `crates/trellis-store-postgres/migrations/`. Migration runner integration is out of scope for G-4.
- **Formal verification or fuzzing harness.** Property tests and KATs only. Fuzzing (`cargo-fuzz`) is a hygiene follow-on.

## Open items

Flagged for orchestrator review before M1 execution starts.

1. **Seven crates vs five.** The G-4 ratification language pins seven names; this plan adds two internal (`trellis-cddl`, `trellis-types`) + one provisional (`trellis-export`) for ten total. Argument against: ten crates for Phase 1 is a lot of `Cargo.toml` surface; a five-crate version (`trellis-core` absorbing cddl + types + export, `trellis-store-memory`, `trellis-store-postgres`, `trellis-verify`, `trellis-cli`, `trellis-conformance`) would work. **Recommendation: keep the ten-crate split.** The internal utility crates are where the discipline lives — a single `trellis-core` doing CDDL + types + append + export loses the seam that lets us swap the CBOR library or extract a verifier-only build. But this is a genuine tradeoff worth naming.
2. **Strict-superset verification of Phase 2 bytes.** The format is designed strict-superset-compatible — a Phase 2 event with extension keys is still a valid Phase 1 envelope if the extensions are in the reserved container. Should the G-4 verifier eagerly tolerate Phase 2 extensions (preserve them through export, surface them in warnings), or refuse them (per Phase 1 C-5 MUST reject unknowns)? **Recommendation: refuse by default, with a `VerifyPolicy::tolerate_phase2_extensions = false` knob that stays false through G-5.** Strict behavior is what the stranger test asserts; tolerance behavior is a Phase-2 problem. But the opposite choice (tolerate by default) is defensible on the grounds that strict refusal locks in a Phase-1-only byte surface and we then have to re-audit when Phase 2 lands.
3. **`ciborium` vs custom dCBOR.** `ciborium` implements CBOR per RFC 8949, not dCBOR strict-mode per §5.1. The strict discipline (canonical ordering, shortest-form integers, no indefinite-length items) has to be layered on top. If we find `ciborium` can't be wrapped cleanly — e.g., it accepts non-canonical inputs silently — we need to either fork it or hand-write a dCBOR encoder. Decide during M1.a.
4. **`coset` protected-header integer label behavior.** Core §7.4 pins specific integer labels; `coset` may serialize them differently than we need. First test of M1.b proves it or forces a workaround. If `coset` can't produce byte-exact output, the fallback is manual COSE_Sign1 assembly — still using `ed25519-dalek` for the signature, but hand-rolling the CBOR structure via `trellis-cddl`.
5. **Postgres schema.** Not designed in this plan. The `LedgerStore` trait shape is the contract; the Postgres schema is an implementation detail of `trellis-store-postgres` and will be designed in a follow-on plan once M1 closes and we know the access patterns append and export actually exercise.
6. **Hpke wrap semantics for `append/004`.** The fixture-system first-batch brainstorm names `append/004-hpke-wrapped-inline` with a pinned X25519 ephemeral keypair under `_keys/`. The Rust impl must consume the ephemeral keypair rather than generating one — this may require a non-default `hpke` crate API. Confirm feasibility before M2 starts that batch.
7. **Fixture-runner output format for CI.** Human-readable hex-diff vs structured JSON vs JUnit-XML. Decide before M1 closes; probably human-readable + a `--format json` flag for CI consumers.
8. **Worktree vs main.** Per CLAUDE.md, default is no worktree. G-4 execution is a multi-session L effort; a dedicated worktree is probably justified for the M1 sprint to isolate `rust/target/` from any parallel fixture-authoring sessions. Orchestrator call.
