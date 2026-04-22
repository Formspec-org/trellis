#!/bin/sh
set -eu

if command -v trellis-verify >/dev/null 2>&1; then
  exec trellis-verify "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
fi

echo "trellis-verify not found in PATH (export/005-attachments-inline)." >&2
exit 2
