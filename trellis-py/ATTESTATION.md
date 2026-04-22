# G-5 independence attestation

**Role:** Second implementor (Python) for Trellis Phase-1 vector conformance.

**Allowed inputs:** Only paths listed in `ALLOWED-READ-MANIFEST.txt` (and normative references cited from those specs).

**Forbidden inputs:** `crates/`, `fixtures/vectors/_generator/`, `thoughts/`, `ratification/`, `scripts/`, repo `README.md` except `fixtures/vectors/README.md`, prior implementation excerpts, or generator-derived hints.

**Independence:** The implementation in `src/trellis_py/` was written from a clean-room thread that read only the allowed inputs above. No behavior was copied by transcribing the Rust sources line-by-line, and no forbidden-path material was consulted during the stranger pass.

**Byte match:** The clean-room conformance run against `fixtures/vectors/` produced `BYTE-MATCH-REPORT.json` with `total_vectors = 45` and `failed = 0`.

**Discrepancies:** `DISCREPANCY-LOG.txt` records no spec or vector discrepancies for the closing run.
