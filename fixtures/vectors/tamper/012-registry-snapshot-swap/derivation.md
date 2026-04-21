# Derivation — `tamper/012-registry-snapshot-swap`

Registry-binding tamper over `export/003-three-event-transition-chain`. Replaces the bound registry snapshot with the minimal append-only registry from `append/009-signing-key-revocation`, updates the manifest binding digest and member path, and re-signs the manifest. Digest bindings remain valid, but the transition event types are no longer resolvable under the embedded registry snapshot, so Core §19 step 4.j records `registry_digest_mismatch`.
