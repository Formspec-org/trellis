"""Cross-implementation byte oracle for the C2PA Trellis assertion.

Pairs with the Rust unit test
``trellis-interop-c2pa::tests::emit_matches_canonical_dcbor_fixture_bytes``
(Wave 26, FINDING 1 close-out). Together they form the
ISC-02 byte-determinism cross-impl oracle for ADR 0008
``c2pa-manifest@v1``:

* Rust (``ciborium``) emits the assertion bytes and asserts byte
  equality with the on-disk fixture
  ``fixtures/vectors/export/014-…/cert-wave25-001.c2pa``.
* Python (``cbor2.dumps(..., canonical=True)``) below produces the
  same bytes for the same logical input, against the same fixture.

If either encoder drifts from RFC 8949 §4.2.2 / Core §5.1 dCBOR
canonical map-key ordering, exactly one of the two halves fails —
localizing the regression to the offending implementation. Cross-impl
divergence on the 5-field assertion is the load-bearing claim ADR
0008's path-(b) verifier rests on.

The brief audited adding ``c2pa-rs`` as a dev-dep for a third oracle
(C2PA-tooling round-trip). The audit found 285 transitive crates plus
unused tokio/openssl/reqwest/hyper — too heavy for an assertion-only
oracle. Skipped in Wave 26; this Python oracle is the cross-impl
enforcement mechanism. (See COMPLETED.md Wave 26 entry.)
"""

from __future__ import annotations

from pathlib import Path

import cbor2

# Path resolution: this file lives at trellis-py/tests/, the fixture
# lives at fixtures/vectors/export/014-…/.
_REPO_ROOT = Path(__file__).resolve().parent.parent.parent
_FIXTURE_PATH = (
    _REPO_ROOT
    / "fixtures"
    / "vectors"
    / "export"
    / "014-interop-sidecar-c2pa-manifest"
    / "interop-sidecars"
    / "c2pa-manifest"
    / "cert-wave25-001.c2pa"
)

# The five fields exactly as in
# ``fixtures/vectors/_generator/gen_interop_sidecar_c2pa_037_to_040.py``.
# Mirrored here so the test is self-contained — if the generator's
# logical input ever drifts from the Rust unit-test inputs, the Rust
# oracle in `trellis-interop-c2pa::tests` and this oracle catch the
# drift independently.
_FIXTURE_LOGICAL_INPUT: dict[str, bytes | str] = {
    "trellis.canonical_event_hash": bytes([0x11] * 32),
    "trellis.certificate_id": "cert-wave25-001",
    "trellis.cose_sign1_ref": bytes([0x44] * 32),
    "trellis.kid": bytes([0x33] * 16),
    "trellis.presentation_artifact.content_hash": bytes([0x22] * 32),
}

# Length-then-bytes canonical key order per RFC 8949 §4.2.2 over the
# encoded `tstr` bytes. Locked by name so a future field rename must
# revisit canonical ordering deliberately rather than silently regress.
_CANONICAL_KEY_ORDER = [
    "trellis.kid",                                      # 11 bytes
    "trellis.certificate_id",                           # 22 bytes
    "trellis.cose_sign1_ref",                           # 22 bytes
    "trellis.canonical_event_hash",                     # 28 bytes
    "trellis.presentation_artifact.content_hash",       # 42 bytes
]


def test_fixture_is_dcbor_canonical_via_cbor2() -> None:
    """The on-disk fixture MUST be byte-equal to cbor2's canonical
    re-encoding of the same logical input. Fails on any drift in
    fixture content vs. canonical ordering."""
    fixture_bytes = _FIXTURE_PATH.read_bytes()
    re_encoded = cbor2.dumps(_FIXTURE_LOGICAL_INPUT, canonical=True)
    assert re_encoded == fixture_bytes, (
        "fixture cert-wave25-001.c2pa diverges from cbor2(canonical=True) "
        "encoding of its declared logical input — fixture is no longer canonical "
        "or generator inputs drifted"
    )


def test_fixture_key_order_is_length_then_bytes() -> None:
    """Decoding the fixture MUST surface keys in the canonical
    length-then-bytes order the Rust emitter is now contracted to
    produce. This is the cross-impl agreement point with
    ``trellis-interop-c2pa::tests::emit_canonical_key_order_…``."""
    fixture_bytes = _FIXTURE_PATH.read_bytes()
    decoded = cbor2.loads(fixture_bytes)
    assert list(decoded.keys()) == _CANONICAL_KEY_ORDER, (
        "fixture key order is not dCBOR canonical (length-then-bytes); "
        f"got {list(decoded.keys())!r}, expected {_CANONICAL_KEY_ORDER!r}"
    )


def test_distinct_logical_input_round_trips_canonically() -> None:
    """Vary every field from the fixture's input to exercise the
    canonical-ordering invariant independent of any specific byte
    pattern. The fields and their lengths are fixed, so the canonical
    key order is invariant under input variation — only the values
    change. cbor2(canonical=True) MUST emit the same key sequence."""
    distinct_input: dict[str, bytes | str] = {
        "trellis.canonical_event_hash": bytes([0xAA] * 32),
        "trellis.certificate_id": "cert-other-id",
        "trellis.cose_sign1_ref": bytes([0xBB] * 32),
        "trellis.kid": bytes([0xCC] * 16),
        "trellis.presentation_artifact.content_hash": bytes([0xDD] * 32),
    }
    encoded = cbor2.dumps(distinct_input, canonical=True)
    decoded = cbor2.loads(encoded)
    assert list(decoded.keys()) == _CANONICAL_KEY_ORDER

    # And a second encode of the same logical input must be byte-
    # identical (ISC-02 deterministic derivation, Python half).
    encoded_again = cbor2.dumps(distinct_input, canonical=True)
    assert encoded == encoded_again, "cbor2(canonical=True) is not deterministic"
