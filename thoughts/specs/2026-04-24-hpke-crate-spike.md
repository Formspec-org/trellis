# HPKE crate selection spike — `trellis-core`

**Date:** 2026-04-24
**Status:** Decided; pending implementation landing.
**Owner:** Trellis center.
**Unblocks:** Sequence item #6 (HPKE wrap/unwrap in Rust) in [`../../TODO.md`](../../TODO.md). Also item #7 (duplicate-ephemeral lint), which hangs off the Rust HPKE infrastructure this item stands up.

## Decision

Adopt the [`hpke`](https://github.com/rozbb/rust-hpke) crate (published on crates.io as `hpke`, repo `rozbb/rust-hpke`) for the `trellis-core` HPKE wrap/unwrap path. Pin to a single version in `Cargo.toml`; document the version in the Trellis requirements matrix as load-bearing for byte conformance with the Python stranger (which uses `hpke` from PyPI, RFC 9180 compliant).

## Context

Core §9.4 pins the HPKE suite to X25519-HKDF-SHA256-ChaCha20Poly1305 (RFC 9180 KEM ID `0x0020`, KDF ID `0x0001`, AEAD ID `0x0003`) and specifies per-`KeyBagEntry` ephemeral freshness. `append/004-hpke-wrapped-inline` exists as the byte-exact reference vector; the Python stranger byte-matches it by performing real HPKE wrap using the Python `hpke` package; the Rust reference implementation currently round-trips the pinned bytes without performing the crypto work. Sequence item #6 closes that asymmetry.

The G-5 stranger-test claim weakens while the Rust side short-circuits: vectors match because bytes match, not because both implementations independently derive them. Any new HPKE-using vector widens the gap.

## Options considered

### Option 1 — `hpke` (rozbb/rust-hpke) — **selected**

- Pure Rust, no C dependencies. Apache-2.0 / MIT dual license.
- Implements RFC 9180 Base, Auth, PSK, and AuthPSK modes. Phase-1 needs only Base.
- Supports the exact Trellis suite: X25519, HKDF-SHA256, ChaCha20Poly1305.
- Actively maintained; last release within the freshness window for production crypto libraries.
- Used in production in several ECH (Encrypted Client Hello for TLS 1.3) and MLS-adjacent implementations.
- `no_std` friendly — leaves the door open for the Open Item #6 in the archived G-4 workspace plan even though we haven't committed to it.
- Clean `Serializable`/`Deserializable` traits over ephemeral pubkeys, encapsulated keys, and AEAD ciphertexts. Single-shot API (`single_shot_seal_in_place_detached` / `single_shot_open_in_place_detached`) matches our per-`KeyBagEntry` usage exactly.

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

- **Byte-match risk minimization.** Both Rust and Python use the same library family (the `hpke` name is shared across the Rust and Python implementations though they're separate codebases). Both implement RFC 9180 straight. Byte divergence between them would be an RFC-compliance bug in one of them, not a design-level mismatch.
- **Single-suite fit.** We pin exactly one ciphersuite in Phase 1. Option 1's API surface covers this without leaving unused knobs exposed.
- **Dependency weight.** Pure Rust, no OpenSSL, no `ring`. Fits the existing `trellis-core` / `trellis-cose` posture.
- **Maintenance trajectory.** The crate has steady maintenance and an active issue tracker. Not abandonware.
- **PQ-suite migration path.** The crate has KEM-generic traits; if Phase-2+ ever uses a PQ HPKE suite (Kyber-based, per `suite_id` registry reservation), the same API shape applies.

## Interface sketch for `trellis-core`

A new `hpke.rs` module inside `trellis-core` (or a thin `trellis-hpke` sibling crate if isolation is preferred — TBD during implementation, not load-bearing for this spike). Sketch:

```rust
use hpke::{
    aead::ChaCha20Poly1305,
    kdf::HkdfSha256,
    kem::X25519HkdfSha256,
    Kem, OpModeR, OpModeS, Serializable, Deserializable,
};

const HPKE_INFO: &[u8] = b"trellis-hpke-v1";

pub struct WrapResult {
    pub ephemeral_pubkey: [u8; 32],
    pub ciphertext: Vec<u8>,
    pub aead_tag: [u8; 16],
}

pub fn wrap_dek(
    recipient_pubkey: &[u8; 32],
    dek_plaintext: &[u8],
    aad: &[u8],
) -> Result<WrapResult, HpkeError> {
    // Load recipient pubkey; single_shot_seal; return ephemeral + ct + tag.
}

pub fn unwrap_dek(
    recipient_privkey: &[u8; 32],
    ephemeral_pubkey: &[u8; 32],
    ciphertext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, HpkeError> {
    // Load recipient privkey; single_shot_open; return plaintext.
}

/// Test-vector path: accept a pinned ephemeral privkey (per Core §9.4 carve-out)
/// instead of generating one. Callable only when TRELLIS_TEST_VECTORS env
/// allows it, mirroring the existing append/004 fixture discipline.
pub fn wrap_dek_with_pinned_ephemeral(
    recipient_pubkey: &[u8; 32],
    ephemeral_privkey: &[u8; 32],
    dek_plaintext: &[u8],
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
3. Integration test: feed `append/004-hpke-wrapped-inline`'s input recipient pubkey + pinned ephemeral privkey + DEK plaintext + AAD through the new wrap path; assert output `ephemeral_pubkey` + `ciphertext` + `aead_tag` byte-match the committed vector.
4. Once the test-vector path matches, implement `wrap_dek` (real ephemeral generation) + one round-trip test that generates, wraps, unwraps, and checks equality against the original plaintext.
5. Python stranger needs no changes; it already does real HPKE via the Python `hpke` package. The byte-match conformance test (currently `trellis-py/` stranger vs. committed bytes) extends transparently: after landing, the Rust side independently produces the same bytes as the Python stranger.

## Risk

- **Crate abandonment or breaking change.** The primary mitigation is version-pinning in `Cargo.toml`. If the crate stalls, Option 3 (hand-roll) remains viable — the fallback path is a 1-week spike using RFC 9180 directly, and the integration test corpus stays the same.
- **RFC-compliance divergence.** Rust and Python implementations of `hpke` are independent. A real divergence would be an RFC-compliance bug in one. Discovery surface: the stranger test on `append/004` would fail after landing the Rust path if such a bug existed. That IS the purpose of the stranger test and the byte-match claim; a divergence would generate a spec-clarification commit and a filed issue upstream.
- **PQ migration.** Not a current risk. When Phase-2+ or later phases add a PQ HPKE suite, the crate's trait-generic design supports adding it; separate implementation work.

## Follow-ons triggered

Once the Rust HPKE path lands:

- Sequence item #7 (HPKE duplicate-ephemeral detection lint) becomes actionable — the lint walks the committed key-bag entries across a ledger scope and verifies ephemeral-pubkey uniqueness. No ambiguity that wasn't already resolved by Core §9.4's uniqueness obligation; just needs a surface to run against.
- The latent `§8.6 / §9.4` alignment note in the 2026-04-23 spec update becomes strictly binding: a verifier reading §9.4 who also verifies LAK rotation chains will enforce the `unique per wrap` constraint against actual Rust wrap output.

## Decision log

- 2026-04-24 — Selected `hpke` (rozbb/rust-hpke). Rationale above.

---

*End of spike memo. Implementation lands against sequence item #6 in the Trellis TODO.*
