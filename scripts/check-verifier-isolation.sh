#!/usr/bin/env bash
# Trellis verifier-isolation CI assertion.
#
# Asserts that `trellis-verify`'s dependency graph does NOT include any
# HPKE-or-related crypto crate. The Phase-1 verifier MUST stay free of
# HPKE so an offline core-bytes verify (Core §16 — Verification
# Independence) does not pull in the HPKE / X25519 / AEAD / HKDF
# transitive graph.
#
# Why the sibling-crate architecture rests on this:
#   - `trellis-hpke` is a sibling crate at the same level as
#     `trellis-core` / `trellis-cose`. The boundary is what makes Core
#     §16 enforceable structurally (not just by prose discipline).
#   - A future change to `trellis-cose` that pulls `trellis-hpke` in as
#     a dep would silently breach the verifier-isolation invariant
#     because every consumer of `trellis-cose` (including
#     `trellis-verify`) would inherit HPKE.
#   - This script is the loud-fail gate: `cargo tree -p trellis-verify`
#     MUST NOT mention `hpke`, `x25519-dalek`, `chacha20poly1305`, or
#     `hkdf`. Run via `make check-verifier-isolation` (Trellis-local)
#     or directly in CI.
#
# Authority:
#   - Core §16 (Verification Independence)
#   - ADR 0009 §"Architectural posture" (sibling-crate boundary)
#   - ADR 0008 §ISC-05 (same hygiene contract for ecosystem libs)
#   - `crates/trellis-hpke/Cargo.toml` `# DO NOT BUMP` block (cites
#     this script as the firewall)

set -euo pipefail

# Resolve the trellis root: this script lives at <root>/scripts/, and
# `cargo tree` resolves the manifest from the workspace root.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# The forbidden crates. `trellis-verify` MUST NOT list any of these in
# its dependency tree. ADR 0009 names these exactly.
FORBIDDEN_RE='hpke|x25519-dalek|chacha20poly1305|hkdf'

# `cargo tree` from the parent workspace root (the trellis crates live
# under `trellis/crates/*` in the parent `Cargo.toml`'s workspace
# members list). Run from $ROOT_DIR/.. so cargo finds the workspace.
PARENT_DIR="$(cd "$ROOT_DIR/.." && pwd)"

echo "Asserting trellis-verify is HPKE-clean (Core §16; ADR 0009)..."
echo "  workspace root: $PARENT_DIR"
echo "  forbidden: $FORBIDDEN_RE"

# `cargo tree -p trellis-verify` lists every dep + transitive in the
# graph. We grep for any forbidden crate name; a hit (exit 0) is a
# regression. We invert by treating a hit as failure and absence
# (`grep -E ... || true` returning empty) as success.
TREE_OUTPUT="$(cd "$PARENT_DIR" && cargo tree -p trellis-verify --edges normal,build,dev 2>&1)"

# Filter the tree to lines that mention any of the forbidden crates.
# Words may appear in `name vX.Y.Z` form or `(*)` repetition lines;
# either form is a regression.
HITS="$(printf '%s\n' "$TREE_OUTPUT" | grep -E "\b($FORBIDDEN_RE)\b" || true)"

if [ -n "$HITS" ]; then
    echo
    echo "FAIL: trellis-verify dependency graph includes a forbidden HPKE-related crate." >&2
    echo "      Core §16 (Verification Independence) requires the offline verifier path" >&2
    echo "      to not depend on HPKE / X25519 / AEAD / HKDF. ADR 0009 §'Architectural" >&2
    echo "      posture' explains why; the sibling-crate firewall is what enforces it." >&2
    echo >&2
    echo "Hits:" >&2
    printf '%s\n' "$HITS" | sed 's/^/  /' >&2
    echo >&2
    echo "Diagnose: probably a transitive add to trellis-cose or trellis-types pulled" >&2
    echo "an HPKE-related crate into the graph. Move that work into trellis-hpke (or" >&2
    echo "a new sibling crate) and re-run." >&2
    exit 1
fi

echo "OK: trellis-verify is HPKE-clean."
exit 0
