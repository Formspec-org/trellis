# G-5 independence attestation (template)

**Role:** Second implementor (Python) for Trellis Phase-1 vector conformance.

**Allowed inputs:** Only paths listed in `ALLOWED-READ-MANIFEST.txt` (and normative references cited from those specs).

**Forbidden inputs:** `crates/`, `fixtures/vectors/_generator/`, `thoughts/`, `ratification/`, `scripts/`, repo `README.md` except `fixtures/vectors/README.md`, prior implementation excerpts, or generator-derived hints.

**Independence:** The implementation in `src/trellis_py/` was written to match the committed vector bytes; no behavior was copied by transcribing the Rust sources line-by-line. In-repo development used the full tree for parity checks; a clean-room stranger MUST follow the forbidden list.

**Byte match:** Run `python -m trellis_py.conformance --write-report BYTE-MATCH-REPORT.json` with only the allowed tree mounted; `failed` MUST be `0` before G-5 closes.

**Discrepancies:** Any mismatch MUST be logged with spec section + vector artifact references only (never “because Rust does X”).
