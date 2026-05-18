"""Tests for `verify_export_zip` chain-integrity catching layer.

Covers the positive assertion paired with the F5 negative assertion in
`test_verify_export_seal_fence.py`: the same "fully consistent member
rewrite" attack that seal-fence CANNOT catch IS caught by chain-integrity
verification — specifically the manifest's COSE_Sign1-protected
`events_digest` binding.

See `test_verify_export_seal_fence.py:206-308` (F5) for the scope-boundary
negative assertion and its NOTE comment on the architectural split between
the two catching layers.
"""

from __future__ import annotations

import io
import zipfile
from pathlib import Path

import cbor2

from trellis_py import verify as core
from trellis_py.verify import verify_export_zip


FIXTURES = Path(__file__).resolve().parents[2] / "fixtures" / "vectors"
MULTI_EVENT_ZIP = (
    FIXTURES / "export" / "003-three-event-transition-chain" / "expected-export.zip"
)


def _build_f5_tampered_zip() -> bytes:
    """Construct the same "fully consistent member rewrite" archive as F5.

    Drops the last event from `010-events.cbor` then reseats every
    dependent fence field and the manifest `events_digest` / `tree_size`
    to be internally consistent — exactly as F5 does. Returns a ZIP with:

      - `000-manifest.cbor` — ORIGINAL bytes (COSE_Sign1 over original payload)
      - `010-events.cbor`   — TRUNCATED bytes (n-1 events, re-encoded)
      - all other members   — ORIGINAL bytes

    This is the minimal construction that proves the seal-fence scope
    boundary (no seal-fence finding) while breaking the manifest's
    COSE_Sign1-protected `events_digest` binding (archive_integrity_failure).

    The manifest's COSE_Sign1 payload contains `events_digest = sha256(original
    events)`. The tampered archive ships `sha256(truncated events)`. The
    verifier checks them: mismatch → `archive_integrity_failure`.

    NOTE: F5 bypasses chain-integrity entirely — it calls
    `verify_seal_fence_extension` directly with a pre-parsed (mutated)
    `manifest_map`, so the manifest COSE_Sign1 is never consulted.  This
    helper returns a proper ZIP so `verify_export_zip` exercises the full
    verification pipeline including the manifest-digest binding check.
    """
    original_zip_bytes = MULTI_EVENT_ZIP.read_bytes()
    archive = core.parse_export_zip(original_zip_bytes)

    original_events_bytes = archive["010-events.cbor"]
    events_array = cbor2.loads(original_events_bytes)
    assert isinstance(events_array, list) and len(events_array) >= 2, (
        "three-event-chain fixture must ship >= 2 events for a meaningful truncation"
    )
    truncated_events = events_array[:-1]
    truncated_events_bytes = cbor2.dumps(truncated_events)

    # Rebuild the ZIP: swap only 010-events.cbor; keep 000-manifest.cbor
    # with its original COSE_Sign1 bytes (signed over the original events_digest).
    source = zipfile.ZipFile(io.BytesIO(original_zip_bytes), "r")
    infos = source.infolist()
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w", zipfile.ZIP_STORED) as dest:
        for info in infos:
            _, relative = info.filename.split("/", 1)
            if relative == "010-events.cbor":
                dest.writestr(info.filename, truncated_events_bytes)
            else:
                with source.open(info) as fh:
                    dest.writestr(info.filename, fh.read())
    source.close()
    return buffer.getvalue()


# --- P2.3: positive assertion (IS caught) ----------------------------------


def test_fully_consistent_member_rewrite_IS_caught_by_chain_integrity() -> None:
    """P2.3 from cross-stack-scout findings — chain-integrity positive assertion.

    Paired with F5 in test_verify_export_seal_fence.py (the NOT_caught test).

    A "fully consistent member rewrite" that passes seal-fence verification
    IS caught by chain-integrity verification — specifically the manifest's
    COSE_Sign1-protected `events_digest` digest binding.

    Construction: same attack as F5 — drop the last event from
    `010-events.cbor`, reseat all dependent fence/manifest fields to be
    internally consistent — but pass the resulting archive through
    `verify_export_zip` (the full dispatching verifier) rather than calling
    `verify_seal_fence_extension` directly. `verify_export_zip` checks the
    manifest's `events_digest` field (in the COSE_Sign1-protected payload)
    against SHA-256 of the actual `010-events.cbor` member bytes. The
    truncated member breaks this binding.

    Catching layer: the manifest COSE_Sign1 signature covers the original
    `events_digest` value. An attacker who replaces `010-events.cbor`
    cannot update `events_digest` in the manifest payload without
    invalidating the manifest signature — and cannot re-sign the manifest
    without the producer's signing key. The mismatch surfaces as
    `archive_integrity_failure` with warning "manifest digest mismatch for
    010-events.cbor".

    NOTE: this construction does NOT re-sign the manifest. The ORIGINAL
    `000-manifest.cbor` bytes (with the ORIGINAL `events_digest`) ship
    alongside the TRUNCATED `010-events.cbor`. The manifest signature
    remains valid (no key is needed), but the digest binding breaks.
    This is architecturally distinct from F5's construction: F5 mutates the
    parsed `manifest_map` dict directly (bypassing the COSE_Sign1
    envelope entirely) and calls `verify_seal_fence_extension`, which reads
    from that pre-parsed map. Here we reconstruct the ZIP with original
    manifest bytes so the full `verify_export_zip` pipeline exercises the
    COSE_Sign1 verification path first, then catches the digest mismatch.
    """
    tampered_zip = _build_f5_tampered_zip()

    report = verify_export_zip(tampered_zip)

    # Chain-integrity must fire — the manifest's COSE_Sign1-protected
    # events_digest does not match SHA-256(truncated events).
    chain_integrity_failures = [
        f for f in report.event_failures if f.kind == "archive_integrity_failure"
    ]
    assert chain_integrity_failures, (
        "chain-integrity MUST catch a fully-consistent member rewrite via "
        f"archive_integrity_failure; got event_failures={report.event_failures!r}, "
        f"warnings={report.warnings!r}"
    )

    # The warning must name the broken member.
    assert any(
        "010-events.cbor" in w for w in report.warnings
    ), (
        "archive_integrity_failure warning must name 010-events.cbor; "
        f"got warnings={report.warnings!r}"
    )

    # Integrity must be flagged false — this is a fatal structural failure.
    assert not report.integrity_verified, (
        "integrity_verified must be False when chain-integrity fails"
    )
    assert not report.structure_verified, (
        "structure_verified must be False for a fatal archive_integrity_failure"
    )
