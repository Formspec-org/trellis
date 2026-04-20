"""Generate verify negative vectors for export/001.

Generates:
- verify/002-export-001-manifest-sigflip
- verify/003-export-001-missing-registry-snapshot

Authoring aid only. This script is NOT normative; the vectors' derivation.md
documents cite Core § prose as the reproduction authority.

Determinism: two runs produce byte-identical ZIP outputs.
"""

from __future__ import annotations

from pathlib import Path

import cbor2


ROOT = Path(__file__).resolve().parent.parent  # fixtures/vectors/
SOURCE_EXPORT_DIR = ROOT / "export" / "001-two-event-chain"
LEDGER_STATE_FILE = SOURCE_EXPORT_DIR / "input-ledger-state.cbor"

OUT_SIGFLIP = ROOT / "verify" / "002-export-001-manifest-sigflip"
OUT_MISSING_REGISTRY = ROOT / "verify" / "003-export-001-missing-registry-snapshot"


ZIP_FIXED_DATETIME = (1980, 1, 1, 0, 0, 0)


def zipinfo(name: str):
    import zipfile

    info = zipfile.ZipInfo(filename=name, date_time=ZIP_FIXED_DATETIME)
    info.compress_type = zipfile.ZIP_STORED
    info.external_attr = 0
    info.extra = b""
    info.flag_bits = 0
    info.create_system = 0
    return info


def flip_last_byte(data: bytes) -> bytes:
    if not data:
        raise ValueError("cannot flip last byte of empty input")
    last = data[-1] ^ 0x01
    return data[:-1] + bytes([last])


def write_zip(out_dir: Path, *, root_dir: str, members: list[str], overrides: dict[str, bytes]):
    import zipfile

    out_dir.mkdir(parents=True, exist_ok=True)
    zip_path = out_dir / "input-export.zip"
    with zipfile.ZipFile(zip_path, "w") as zf:
        for member in sorted(members):
            if member in overrides:
                payload = overrides[member]
            else:
                payload = (SOURCE_EXPORT_DIR / member).read_bytes()
            zf.writestr(zipinfo(f"{root_dir}/{member}"), payload)


def main() -> None:
    ledger_state = cbor2.loads(LEDGER_STATE_FILE.read_bytes())
    root_dir = ledger_state["root_dir"]
    members = list(ledger_state["members"])

    # verify/002: flip one bit in the manifest signature (COSE_Sign1 last byte).
    manifest_bytes = (SOURCE_EXPORT_DIR / "000-manifest.cbor").read_bytes()
    tampered_manifest = flip_last_byte(manifest_bytes)
    write_zip(
        OUT_SIGFLIP,
        root_dir=root_dir,
        members=members,
        overrides={"000-manifest.cbor": tampered_manifest},
    )

    # verify/003: omit the required registry snapshot file referenced by the manifest.
    registry_members = [m for m in members if m.startswith("050-registries/")]
    if len(registry_members) != 1:
        raise ValueError(f"expected exactly 1 registry member, found {registry_members}")
    missing_members = [m for m in members if m not in set(registry_members)]
    write_zip(
        OUT_MISSING_REGISTRY,
        root_dir=root_dir,
        members=missing_members,
        overrides={},
    )


if __name__ == "__main__":
    main()

