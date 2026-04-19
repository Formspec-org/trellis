"""Generate pinned attestation-authority-cm-b-001 COSE_Key (Ed25519).

Run once; commit output. The seed is literal so regeneration is deterministic
and auditable from this source file alone.

Role: prior-custody-model attestation authority for O-5 posture-transition
fixtures. Companion Appendix A.5.3 step 4 (and OC-11) requires Posture-widening
transitions — e.g. CM-B (Reader-Held w/ Recovery Assistance) → CM-A
(Provider-Readable Custodial) — to be dually attested by BOTH the prior and
new authorities. The issuer signing key (issuer-001) represents the new
authority-class signer; this key represents the prior authority-class signer
for CM-B. Held entirely inside the fixture corpus; does NOT sign any ledger
event — only attestation preimages under domain tag
`trellis-transition-attestation-v1` (Core §9.8).

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

# Distinct seed vs issuer-001 (…00aa) so the two keys cannot be confused at a
# glance. No further constraint; opaque 32 bytes to Ed25519.
SEED = bytes.fromhex(
    "00000000000000000000000000000000000000000000000000000000000000bb"
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
    out = (
        Path(__file__).resolve().parent.parent
        / "_keys"
        / "attestation-authority-cm-b-001.cose_key"
    )
    out.write_bytes(cbor2.dumps(cose_key))
    print(f"wrote {out} ({out.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
