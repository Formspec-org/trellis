# append/004-hpke-wrapped-inline derivation

**What this vector exercises.** This is the first append vector with real
payload encryption and a populated `KeyBag`. The payload plaintext is
`../../_inputs/sample-payload-001.bin`; the fixture encrypts it with
ChaCha20-Poly1305 using the pinned 32-byte DEK
`000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f`,
nonce `040404040404040404040404`, and `aad = h''` per Core §6.4. The
resulting ciphertext is the bytes embedded in `PayloadInline.ciphertext`, and
`content_hash = e31a0f28ac9e1867b938866c8f2a02c7fa4df5144d11450ec2940bce515f6e01`
is the `trellis-content-v1` hash over those ciphertext bytes per Core §9.3.

**HPKE wrap.** The DEK is wrapped to recipient `ledger-service` with HPKE
suite 1, pinned in Core §9.4 as RFC 9180 Base mode with
`DHKEM(X25519, HKDF-SHA256)`, `HKDF-SHA256`, and `ChaCha20-Poly1305`.
Suite 1 uses `info = h''` and wrap `aad = h''`. The recipient key is
`../../_keys/recipient-004-ledger-service.cose_key`; the pinned fixture-only
ephemeral key is `../../_keys/ephemeral-004-recipient-001.cose_key`.

The HPKE encapsulated public key is:

```text
34e42d4af5ef94a07a3a84201b889d4cd1a743cb27b11b6a10438a8feb8e5847
```

The HPKE sealed DEK is:

```text
9f89d135c1594b3a52a9854609e8ac9387ec1d9a82865e8ab35fd43a2cf77028f848c833e9871ae9f43fef0b28b743fa
```

**Fixture-only ephemeral carve-out.** Core §9.4 requires production producers
to generate a fresh X25519 ephemeral keypair per `KeyBagEntry`, use the
private key exactly once, and destroy it. This vector pins the ephemeral
private key only because Core §5.2 and §27 require fixture bytes to reproduce
exactly across independent implementations. No production `Fact Producer`,
`Canonical Append Service`, or `Verifier` may rely on the pinned-key behavior.

## Construction

1. Load issuer signing key `issuer-001.cose_key` and derive `kid =
   af9dff525391faa75c8e8da4808b1743` per Core §8.3.
2. Encrypt the 64-byte sample payload with ChaCha20-Poly1305 under the pinned
   DEK and nonce above. Commit the AEAD ciphertext as `PayloadInline.ciphertext`.
3. Compute `content_hash` over the ciphertext bytes under the
   `trellis-content-v1` domain tag.
4. Seal the DEK with HPKE suite 1 using the pinned recipient and ephemeral
   X25519 keys. Store the encapsulated public key as
   `KeyBagEntry.ephemeral_pubkey` and the ciphertext+tag as
   `KeyBagEntry.wrapped_dek`.
5. Build `AuthorEventHashPreimage` with `sequence = 0`, `prev_hash = null`,
   `event_type = "x-trellis-test/append-hpke-inline"`, the encrypted
   `PayloadInline`, and the populated `KeyBag`. Its dCBOR bytes are committed
   as `input-author-event-hash-preimage.cbor` (704 bytes).
6. Hash the authored bytes under `trellis-author-event-v1` to produce
   `author_event_hash = ed098176b1925e92ccfad9cd0615e2a81c5dd1a5bf138330fbf89d7e9cc7a3a1`.
7. Add `author_event_hash` to form `EventPayload`; commit dCBOR as
   `expected-event-payload.cbor` (756 bytes).
8. Build the COSE protected header `{1: -8, 4: kid, -65537: 1}` and
   `Sig_structure = ["Signature1", protected, h'', payload]`.
9. Sign the Sig_structure with issuer-001 Ed25519 and assemble COSE_Sign1 tag
   18 as `expected-event.cbor`.
10. Compute `canonical_event_hash =
    213bbf2a1ca95059e0df9f082ebfb8f9446a8611f49872386477a964c2415f99` and
    emit `expected-append-head.cbor`.

## Coverage mapping

| Requirement | Evidence |
|---|---|
| TR-CORE-031 / invariant #4 | `content_hash` is over real AEAD ciphertext bytes, not plaintext. |
| TR-CORE-038 / invariant #7 | `key_bag` is populated and included in `author_event_hash`; any re-wrap would change the signed event and is therefore forbidden outside the append-only service-wrap path. |

## Regeneration

Run:

```sh
python3 fixtures/vectors/_generator/gen_append_004.py
```

The generator writes the X25519 COSE_Key fixtures, all CBOR artifacts, and the
hash/signature targets listed above.
