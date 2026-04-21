# G-5 Package Handoff

This directory holds local handoff material for commissioning the G-5 stranger
implementation.

The implementor should receive only:

- `trellis-g5-allowed-readset-2026-04-21.tar.gz`

The reviewer or project owner may use:

- `allowed-readset-files.txt` — tracked repo paths included in the archive.
- `allowed-readset-sha256.txt` — per-file SHA-256 checksums for the archive
  source files.
- `archive-sha256.txt` — SHA-256 checksum for the archive itself.

Do not give this README or the surrounding `ratification/` directory to the
independent implementor. The allowed read set is defined in
`thoughts/specs/2026-04-21-trellis-g5-stranger-commission-brief.md`; this
directory only packages that set for handoff.

The archive was built from tracked files only. Untracked workspace files are
not included.
