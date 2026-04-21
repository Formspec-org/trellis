# Derivation — `verify/010-export-002-revoked-key-after-valid-to`

Negative-non-tamper verify vector for Core §19 step 4.a. Starts from `export/002-revoked-key-history`, moves the embedded revoked signing-key entry's `valid_to` earlier than the event's `authored_at`, updates the manifest digest binding, and re-signs the manifest so the verifier reaches the event-level `revoked_authority` branch instead of failing archive integrity earlier.
