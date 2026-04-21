# Derivation — `tamper/011-wrong-scope`

Manifest-only scope tamper. Re-signs `export/001-two-event-chain` after changing `manifest.scope` to a different ledger-scope byte string, leaving event, checkpoint, and proof material untouched so the first localizable failure is the verifier's scope check.
