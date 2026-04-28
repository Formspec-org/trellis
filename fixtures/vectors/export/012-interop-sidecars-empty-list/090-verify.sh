#!/bin/sh
set -eu

# Trellis Phase-1 export verifier invocation (§18.8).
#
# Placeholder: this script only becomes runnable once the G-4 Rust
# `trellis-verify` binary lands per
# `thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md`.
# Until then the fixture deliberately ships no `099-*` bundled
# verifier and this script exits 2 with a human-facing pointer.
#
# If you have a verifier installed as `trellis-verify`, this script
# invokes it against the directory containing this script.

if command -v trellis-verify >/dev/null 2>&1; then
  exec trellis-verify "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
fi

echo "trellis-verify not found in PATH (fixture export/001)." >&2
echo "Run your verifier against this export directory." >&2
exit 2
