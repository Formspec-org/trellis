"""Generate pinned issuer-002 COSE_Key (Ed25519).

Run once; commit output. The seed is literal so regeneration is deterministic
and auditable from this source file alone.

Role: successor signing key under Core §8 signing-key rotation. Used by
`append/002-rotation-signing-key` to exercise invariant #7 (key-bag /
author_event_hash immutable under rotation; Core §8.4 lifecycle; §8.6 re-wrap
mechanics). The pre-rotation event in `002` is signed by `issuer-001`; the
post-rotation event is signed by this key. The registry snapshot committed in
that vector links them via `SigningKeyEntry.supersedes` (§8.2).

COSE_Key layout (RFC 9052 §7): CBOR map with integer keys:
  1 (kty)  = 1 (OKP)
  3 (alg)  = -8 (EdDSA)
  -1 (crv) = 6 (Ed25519)
  -2 (x)   = 32-byte public key
  -4 (d)   = 32-byte secret seed
"""
from pathlib import Path
import cbor2
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives import serialization

# Distinct seed vs issuer-001 (…00aa) and attestation-authority-cm-b-001 (…00bb)
# so all three pinned keys are visually separable in the registry snapshot.
SEED = bytes.fromhex(
    "00000000000000000000000000000000000000000000000000000000000000cc"
)


def main() -> None:
    sk = Ed25519PrivateKey.from_private_bytes(SEED)
    pk_raw = sk.public_key().public_bytes(
        encoding=serialization.Encoding.Raw,
        format=serialization.PublicFormat.Raw,
    )
    cose_key = {
        1: 1,       # kty = OKP
        3: -8,      # alg = EdDSA
        -1: 6,      # crv = Ed25519
        -2: pk_raw,
        -4: SEED,
    }
    out = Path(__file__).resolve().parent.parent / "_keys" / "issuer-002.cose_key"
    out.write_bytes(cbor2.dumps(cose_key))
    print(f"wrote {out} ({out.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
