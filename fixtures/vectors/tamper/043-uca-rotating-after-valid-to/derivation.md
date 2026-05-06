# Derivation — `tamper/043-uca-rotating-after-valid-to`

3-event chain on `ledger_scope = b'trellis-uca-tamper:043-rotating-after-valid-to'`:

* seq 0: identity-attestation event.
* seq 1: host event.
* seq 2: user-content-attestation event signed under kid
  `af9dff525391faa75c8e8da4808b1743`.

The registry marks that kid as `Rotating` (`SigningKeyStatus = 1`) with:

* `valid_from = [1776899000, 0]`
* `valid_to = [1776899999, 0]`
* UCA `attested_at = [1776900000, 0]`

Core §8.4 admits `Rotating` only during the declared rotation-grace overlap.
Because this attestation's `attested_at` is after `valid_to`, verifier step 6
flips `key_active = false` and emits
`user_content_attestation_key_not_active` with `failing_event_id` =
`8f5f7e7c179e289d1554972fc35d482234a48b096b02efaebed164f4f340a3e2`.

Generator: `_generator/gen_tamper_043.py`.
