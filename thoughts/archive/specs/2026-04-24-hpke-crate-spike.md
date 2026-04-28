# HPKE crate selection spike — `trellis-core`

**Date:** 2026-04-24
**Status:** **Superseded — non-normative archive.** Promoted to [ADR 0009 — HPKE Crate Selection](../adr/0009-hpke-crate-selection.md) (Wave 18, 2026-04-27). Cite ADR 0009 — never this spike — for normative HPKE crate-selection authority going forward. This file is kept for historical context only.
**Lifecycle:** Closed. The promote-on-bump discipline now lives in ADR 0009 §Lifecycle.
**Owner:** Trellis center.
**Unblocks:** Sequence item #6 (HPKE wrap/unwrap in Rust) in [`../../TODO.md`](../../TODO.md) — landed Wave 16. Also item #7 (duplicate-ephemeral lint) — landed Wave 17.

## Decision

Adopt the [`hpke`](https://github.com/rozbb/rust-hpke) crate (published on crates.io as `hpke`, repo `rozbb/rust-hpke`) for the `trellis-core` HPKE wrap/unwrap path. Pin to a single version in `Cargo.toml`; document that version in [`specs/trellis-requirements-matrix.md`](../../specs/trellis-requirements-matrix.md) as load-bearing for byte conformance with the committed `append/004-hpke-wrapped-inline` oracle (and with `fixtures/vectors/_generator/gen_append_004.py`, which implements RFC 9180 Base inline—not the PyPI `hpke` package).

## Context

Core §9.4 pins the HPKE suite to X25519-HKDF-SHA256-ChaCha20Poly1305 (RFC 9180 KEM ID `0x0020`, KDF ID `0x0001`, AEAD ID `0x0003`) and specifies per-`KeyBagEntry` ephemeral freshness. `append/004-hpke-wrapped-inline` exists as the byte-exact reference vector; its bytes were authored with real HPKE via the inline RFC 9180 Base implementation in `fixtures/vectors/_generator/gen_append_004.py`. G-5 closed with `trellis-py` matching every committed vector byte-for-byte, but the Rust reference implementation still round-trips `append/004` HPKE fields without performing the crypto work. Sequence item #6 closes that asymmetry so Rust independently derives the same wrap bytes as the generator (and thus the fixture).

The G-5 stranger-test claim weakens while the Rust side short-circuits: vectors match because bytes match, not because both implementations independently derive them. Any new HPKE-using vector widens the gap.

## Options considered

### Option 1 — `hpke` (rozbb/rust-hpke) — **selected**

- Pure Rust, no C dependencies. Apache-2.0 / MIT dual license.
- Implements RFC 9180 Base, Auth, PSK, and AuthPSK modes. Phase-1 needs only Base.
- Supports the exact Trellis suite: X25519, HKDF-SHA256, ChaCha20Poly1305.
- Actively maintained; last release within the freshness window for production crypto libraries.
- Used in production in several ECH (Encrypted Client Hello for TLS 1.3) and MLS-adjacent implementations.
- `no_std` friendly — leaves the door open for the Open Item #6 in the archived G-4 workspace plan even though we haven't committed to it.
- Clean `Serializable`/`Deserializable` traits over ephemeral pubkeys, encapsulated keys, and AEAD ciphertexts. Single-shot API (`single_shot_seal_in_place_detached` / `single_shot_open_in_place_detached`) matches our per-`KeyBagEntry` usage exactly. Resolves the archived G-4 workspace plan open item on pinned-ephemeral feasibility for `append/004` ([`../archive/specs/2026-04-18-trellis-g4-rust-workspace-plan.md`](../archive/specs/2026-04-18-trellis-g4-rust-workspace-plan.md) — “Hpke wrap semantics for `append/004`”) via `wrap_dek_with_pinned_ephemeral` below.

### Option 2 — `hpke-rs` (franziskuskiefer/hpke-rs)

- Pure Rust, MPL-2.0.
- More generic backend abstraction (can swap crypto providers).
- Less directly aligned with our single fixed suite; the abstraction tax buys flexibility we don't need.
- Used in OpenMLS.

### Option 3 — hand-rolled

- We already hand-roll COSE_Sign1 in `trellis-cose` (stricter-than-coset discipline). Hand-rolling HPKE is also achievable — RFC 9180 §5–6 pin the construction fully.
- Rejected: HPKE involves KDF expansion, HPKE context construction, AEAD key derivation, and nonce sequence management. Five more surfaces for subtle byte bugs than COSE_Sign1 has. The stranger test catches divergence but hand-rolling invites it.
- Keep as a fallback if the selected crate develops a blocking issue.

## Rationale for the selection

- **Byte-match risk minimization.** The `hpke` crate name on crates.io is unrelated to any PyPI package name collision—only RFC 9180 semantics matter. Phase 1 suite 1 pins empty `info` and empty wrap `aad` (Core §9.4); the oracle is the committed fixture plus the generator's inline RFC implementation. The Rust crate is a second RFC-backed implementation; divergence surfaces as failing the `append/004` integration test or a spec clarification, not as a “same branded library on both sides” guarantee.
- **Single-suite fit.** We pin exactly one ciphersuite in Phase 1. Option 1's API surface covers this without leaving unused knobs exposed.
- **Dependency weight.** Pure Rust, no OpenSSL, no `ring`. Fits the existing `trellis-core` / `trellis-cose` posture.
- **Maintenance trajectory.** The crate has steady maintenance and an active issue tracker. Not abandonware.
- **PQ-suite migration path.** The crate has KEM-generic traits; if Phase-2+ ever uses a PQ HPKE suite (Kyber-based, per `suite_id` registry reservation), the same API shape applies.

## Interface sketch for `trellis-core`

A new `hpke.rs` module inside `trellis-core` (or a thin `trellis-hpke` sibling crate if isolation is preferred — TBD during implementation, not load-bearing for this spike). Sketch:

**Suite 1 inputs (Core §9.4, `append/004` derivation):** `SetupBaseS` / `Seal` with `info = h''` and wrap `aad = h''`. A future suite MAY define non-empty `info` or `aad`; keep parameters on the API so callers are not hard-wired to empty only, but Phase 1 production paths pass empty slices for both.

**Wire shape:** `KeyBagEntry.wrapped_dek` is the single AEAD ciphertext **concatenated with** the 16-byte Poly1305 tag (same bytes as `WrapResult.ciphertext` + `WrapResult.aead_tag` after seal). Callers assembling CBOR join those two; `unwrap_dek` accepts the wire `wrapped_dek` as one slice and splits internally for `single_shot_open_in_place_detached` if required.

```rust
use hpke::{
    aead::ChaCha20Poly1305,
    kdf::HkdfSha256,
    kem::X25519HkdfSha256,
    Kem, OpModeR, OpModeS, Serializable, Deserializable,
};

/// Phase 1 suite 1: MUST be empty (Core §9.4). Non-empty only when a future suite
/// registers different `info` in the spec.
pub const HPKE_SUITE1_INFO: &[u8] = &[];

pub struct WrapResult {
    pub ephemeral_pubkey: [u8; 32],
    pub ciphertext: Vec<u8>,
    pub aead_tag: [u8; 16],
}

pub fn wrap_dek(
    recipient_pubkey: &[u8; 32],
    dek_plaintext: &[u8],
    info: &[u8],
    aad: &[u8],
) -> Result<WrapResult, HpkeError> {
    // Load recipient pubkey; single_shot_seal with info/aad; return ephemeral + ct + tag.
}

pub fn unwrap_dek(
    recipient_privkey: &[u8; 32],
    ephemeral_pubkey: &[u8; 32],
    wrapped_dek: &[u8],
    info: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, HpkeError> {
    // Split wrapped_dek into ciphertext || tag (last 16 bytes are tag per RFC 9180
    // ChaCha20Poly1305); single_shot_open; return plaintext DEK.
}

/// Test-vector path: accept a pinned ephemeral privkey (per Core §9.4 carve-out)
/// instead of generating one. Callable only when TRELLIS_TEST_VECTORS env
/// allows it, mirroring the existing append/004 fixture discipline.
pub fn wrap_dek_with_pinned_ephemeral(
    recipient_pubkey: &[u8; 32],
    ephemeral_privkey: &[u8; 32],
    dek_plaintext: &[u8],
    info: &[u8],
    aad: &[u8],
) -> Result<WrapResult, HpkeError> { /* ... */ }
```

Integration points:

- `trellis-core::append_event` gains a wrap call on the path that builds `KeyBagEntry` values. Currently the key-bag for `append/001` etc. is empty (structural-only vectors); `append/004` is the only positive HPKE vector and its key-bag is produced by the Python generator.
- Full integration means `append_event` can produce the `append/004` bytes from scratch given the same inputs. One integration test in `trellis-core/tests/` that byte-matches `append/004`'s key-bag entry end-to-end.

## Out of scope for this spike

- Multi-recipient wrap. Trellis invariant is per-`KeyBagEntry` single-recipient; Base mode is correct.
- PSK / Auth / AuthPSK modes. Phase-1 suite does not use them.
- `no_std` compilation. Keep the option open but don't commit.
- Key-encapsulation caching / pooling. Each wrap uses a fresh ephemeral; no caching.

## Verification approach

1. Import the crate at the pinned version.
2. Implement `wrap_dek_with_pinned_ephemeral` first (matches the test-vector carve-out).
3. Integration test: feed `append/004-hpke-wrapped-inline`'s input recipient pubkey + pinned ephemeral privkey + DEK plaintext + **empty `info` and empty wrap `aad`** (suite 1) through the new wrap path; assert output `ephemeral_pubkey` + `ciphertext` + `aead_tag` concatenated byte-match `KeyBagEntry.wrapped_dek` in the committed vector (and match `ephemeral_pubkey` separately).
4. Once the test-vector path matches, implement `wrap_dek` (real ephemeral generation) + one round-trip test that generates, wraps, unwraps, and checks equality against the original plaintext.
5. `trellis-py` (G-5 stranger) compares against committed vector bytes; it needs no dependency change for this work. After landing, Rust independently reproduces the same HPKE wrap bytes as `gen_append_004.py`, strengthening the claim beyond “round-trip opaque `wrapped_dek`.”

## Risk

- **Crate abandonment or breaking change.** The primary mitigation is version-pinning in `Cargo.toml`. If the crate stalls, Option 3 (hand-roll) remains viable — the fallback path is a 1-week spike using RFC 9180 directly, and the integration test corpus stays the same.
- **RFC-compliance divergence.** The Rust `hpke` crate, the inline generator (`gen_append_004.py`), and any future third-party HPKE library used in CI are independent implementations of the same RFC. A real divergence is an RFC-compliance bug in one path or a spec ambiguity. Discovery surface: the `append/004` Rust integration test (and regeneration checks against the generator) fail; resolution is spec clarification and/or upstream issue.
- **PQ migration.** Not a current risk. When Phase-2+ or later phases add a PQ HPKE suite, the crate's trait-generic design supports adding it; separate implementation work.

## Follow-ons triggered

Once the Rust HPKE path lands:

- Sequence item #7 (HPKE duplicate-ephemeral detection lint) becomes actionable — the lint walks the committed key-bag entries across a ledger scope and verifies ephemeral-pubkey uniqueness. No ambiguity that wasn't already resolved by Core §9.4's uniqueness obligation; just needs a surface to run against.
- The latent `§8.6 / §9.4` alignment note in the 2026-04-23 spec update becomes strictly binding: a verifier reading §9.4 who also verifies LAK rotation chains will enforce the `unique per wrap` constraint against actual Rust wrap output.

## Decision log

- 2026-04-24 — Selected `hpke` (rozbb/rust-hpke). Rationale above.
- 2026-04-27 — Executed against TODO item #2 (post-Wave-15 renumber).
  Pinned to `=0.13.0` (latest stable), not `0.14.0-pre.2` — the pre-release
  pulls in `sha3 0.11.0-rc.7` whose `keccak::p1600` API drift breaks the
  build chain. Crate selection is what is load-bearing; the pinned
  version is a solvable downstream concern. Landed in a new
  `trellis-hpke` sibling crate (not in `trellis-core` / `trellis-cose`)
  to preserve Core §16 verification independence — `trellis-verify`'s
  dep tree does not pull HPKE crypto crates. The fixture-only
  `wrap_dek_with_pinned_ephemeral` path bypasses `hpke::setup_sender`
  (which always derives the ephemeral via DeriveKeyPair on fresh
  randomness, RFC 9180 §7.1) and reaches the lower-level public KDF
  helpers (`labeled_extract`, `extract_and_expand`) plus
  `x25519-dalek` + `chacha20poly1305` directly — the only way to feed
  a raw 32-byte X25519 scalar as the ephemeral, which is the carve-out
  shape the Python generator and the committed fixture commit to. The
  production `wrap_dek` and verifier-side `unwrap_dek` go through
  `hpke`'s RFC-blessed public API (`single_shot_seal`,
  `setup_receiver` + `AeadCtxR::open`).

---

*End of spike memo. Implementation landed against sequence item #2 in the Trellis TODO (post-Wave-15 renumber); see `trellis/COMPLETED.md` Wave 16.*
