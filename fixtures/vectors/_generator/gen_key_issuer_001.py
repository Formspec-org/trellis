"""Generate pinned issuer-001 COSE_Key (Ed25519).

Run once; commit output. The seed is literal so regeneration is deterministic
and auditable from this source file alone.

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

SEED = bytes.fromhex(
    "00000000000000000000000000000000000000000000000000000000000000aa"
)

def main() -> None:
    sk = Ed25519PrivateKey.from_private_bytes(SEED)
    pk = sk.public_key()
    pk_raw = pk.public_bytes(
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
    out = Path(__file__).resolve().parent.parent / "_keys" / "issuer-001.cose_key"
    out.write_bytes(cbor2.dumps(cose_key))
    print(f"wrote {out} ({out.stat().st_size} bytes)")

if __name__ == "__main__":
    main()
