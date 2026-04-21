# Derivation — `tamper/009-prev-hash-break`

Mutates `append/005-prior-head-chain`'s `prev_hash`, recomputes the dependent `author_event_hash`, and re-signs the event so the verifier reaches the dedicated `prev_hash_break` branch instead of failing signature or author-hash integrity earlier.
