# trellis-py

Standalone Python implementation of Trellis Phase-1 **append**, **verify**, and **export** (deterministic ZIP), plus a local vector conformance runner. Used for G-5 ratification: behavior is fixed by `../specs/trellis-core.md` and the committed corpus under `../fixtures/vectors/`.

## Install

```bash
cd trellis-py
pip install -e .
```

## APIs

- `trellis_py.append_event(signing_key_cose: bytes, authored_event: bytes) -> AppendArtifacts`
- `trellis_py.export_to_zip_bytes(entries: list[ExportEntry]) -> bytes`
- `trellis_py.verify_export_zip(export_zip: bytes) -> VerificationReport`
- `trellis_py.verify_tampered_ledger(registry: bytes, ledger: bytes, ...) -> VerificationReport`

## Conformance

```bash
# default vectors root: ../fixtures/vectors (from this package location)
python -m trellis_py.conformance

# explicit path + JSON report
python -m trellis_py.conformance --vectors /path/to/fixtures/vectors --write-report BYTE-MATCH-REPORT.json
```

Exit code `0` means every vector under `append/`, `export/`, `verify/`, `tamper/`, `projection/`, and `shred/` passed.

## Dependencies

`cbor2`, `cryptography` (Ed25519). No Rust runtime is required at import time.
