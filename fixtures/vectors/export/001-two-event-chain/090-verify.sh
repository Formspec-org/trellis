#!/bin/sh
set -eu

# Trellis Phase-1 export verifier invocation (§18.8).
#
# This fixture export does not bundle a 099-* verifier binary.
# If you have a verifier installed as `trellis-verify`, this script
# invokes it against the directory containing this script.

if command -v trellis-verify >/dev/null 2>&1; then
  exec trellis-verify "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
fi

echo "trellis-verify not found in PATH (fixture export/001)." >&2
echo "Run your verifier against this export directory." >&2
exit 2
