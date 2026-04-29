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

## Out of scope (intentional)

The Python G-5 oracle implements **path-(b)** of the ADR 0008 interop-
sidecar dispatched verifier — digest-binds only, no `source_ref`
resolution, no C2PA manifest decode (Wave 25). That is the same
discipline `trellis-verify` follows in Rust; both implementations
treat the manifest store as opaque bytes whose SHA-256 (under
`trellis-content-v1`) is the only verifiable surface.

The **C2PA-tooling-path consumer** (read manifest from PDF/JPEG,
decode the `org.formspec.trellis.certificate-of-completion.v1`
assertion, run the five-field cross-check against the canonical
chain) is **not ported to Python**. That path lives in Rust under
`trellis-interop-c2pa` and is consumer-tier per ADR 0008 §"`c2pa-manifest`"
(an adopter picks a C2PA SDK and integrates the assertion bytes into
their PDF rendering pipeline). Porting it to Python would force every
G-5 oracle deployment to ship a C2PA SDK; the path-(b) discipline
sidesteps that by checking only the bytes the export ZIP catalogues.
