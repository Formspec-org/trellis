"""Generate supersession-graph export vectors for ADR 0066 / TR-CORE-170.

Outputs:
- verify/017-export-015-supersession-graph
- tamper/046-supersession-graph-linkage-mismatch

The positive export is a one-event export built from `append/015-supersession`.
The tamper vector keeps the graph manifest-bound and well-formed but changes
the predecessor checkpoint hash so Core §19 step 6e must raise
`supersession_graph_linkage_mismatch`.
"""

from __future__ import annotations

import hashlib
import json
import sys
import zipfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import cbor2  # noqa: E402
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey  # noqa: E402

from _lib.byte_utils import (  # noqa: E402
    ALG_EDDSA,
    CBOR_TAG_COSE_SIGN1,
    COSE_LABEL_ALG,
    COSE_LABEL_KID,
    COSE_LABEL_SUITE_ID,
    SUITE_ID_PHASE_1,
    dcbor,
    deterministic_zipinfo,
    domain_separated_sha256,
    ts,
)


ROOT = Path(__file__).resolve().parent.parent
SOURCE_EVENT_DIR = ROOT / "append" / "015-supersession"
KEY_FILE = ROOT / "_keys" / "issuer-001.cose_key"

OUT_VERIFY = ROOT / "verify" / "017-export-015-supersession-graph"
OUT_TAMPER = ROOT / "tamper" / "046-supersession-graph-linkage-mismatch"

TAG_TRELLIS_EVENT_V1 = "trellis-event-v1"
TAG_TRELLIS_CHECKPOINT_V1 = "trellis-checkpoint-v1"
TAG_TRELLIS_MERKLE_LEAF_V1 = "trellis-merkle-leaf-v1"
TAG_TRELLIS_EXPORT_MANIFEST_V1 = "trellis-export-manifest-v1"

SUPERSEDES_CHAIN_ID_EVENT_EXTENSION = "trellis.supersedes-chain-id.v1"
SUPERSESSION_GRAPH_EXPORT_EXTENSION = "trellis.export.supersession-graph.v1"

GENERATED_AT = ts(1777000090)
CHECKPOINT_TIMESTAMP = ts(1777000020)


def sha256(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def load_issuer_key() -> tuple[bytes, bytes]:
    cose_key = cbor2.loads(KEY_FILE.read_bytes())
    seed = cose_key[-4]
    pubkey = cose_key[-2]
    assert isinstance(seed, bytes) and len(seed) == 32
    assert isinstance(pubkey, bytes) and len(pubkey) == 32
    return seed, pubkey


def derive_kid(pubkey_raw: bytes) -> bytes:
    return hashlib.sha256(dcbor(SUITE_ID_PHASE_1) + pubkey_raw).digest()[:16]


def cose_sign1(seed: bytes, kid: bytes, payload_bytes: bytes) -> bytes:
    protected = dcbor({
        COSE_LABEL_ALG: ALG_EDDSA,
        COSE_LABEL_KID: kid,
        COSE_LABEL_SUITE_ID: SUITE_ID_PHASE_1,
    })
    sig_structure = dcbor(["Signature1", protected, b"", payload_bytes])
    signature = Ed25519PrivateKey.from_private_bytes(seed).sign(sig_structure)
    return dcbor(cbor2.CBORTag(CBOR_TAG_COSE_SIGN1, [protected, {}, payload_bytes, signature]))


def canonical_event_hash(ledger_scope: bytes, event_payload: dict) -> bytes:
    preimage = {"version": 1, "ledger_scope": ledger_scope, "event_payload": event_payload}
    return domain_separated_sha256(TAG_TRELLIS_EVENT_V1, dcbor(preimage))


def merkle_leaf_hash(canonical_hash: bytes) -> bytes:
    return domain_separated_sha256(TAG_TRELLIS_MERKLE_LEAF_V1, canonical_hash)


def checkpoint_digest(ledger_scope: bytes, checkpoint_payload: dict) -> bytes:
    preimage = {"version": 1, "scope": ledger_scope, "checkpoint_payload": checkpoint_payload}
    return domain_separated_sha256(TAG_TRELLIS_CHECKPOINT_V1, dcbor(preimage))


def export_manifest_digest(ledger_scope: bytes, manifest_payload: dict) -> bytes:
    preimage = {"version": 1, "scope": ledger_scope, "manifest_payload": manifest_payload}
    return domain_separated_sha256(TAG_TRELLIS_EXPORT_MANIFEST_V1, dcbor(preimage))


def write_bytes(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)


def canonical_graph_json(
    *,
    head_chain_id: bytes,
    predecessor_chain_id: bytes,
    predecessor_checkpoint_hash: bytes,
) -> bytes:
    # Trellis canonical JSON for this member sorts row keys as
    # bundle_path, chain_id, checkpoint_hash and carries one trailing LF.
    return (
        '{"head_chain_id":"'
        + head_chain_id.hex()
        + '","predecessors":[{"bundle_path":null,"chain_id":"'
        + predecessor_chain_id.hex()
        + '","checkpoint_hash":"'
        + predecessor_checkpoint_hash.hex()
        + '"}]}\n'
    ).encode("utf-8")


def write_zip(out_dir: Path, *, root_dir: str, members: list[str]) -> None:
    with zipfile.ZipFile(out_dir / "input-export.zip", "w") as zf:
        for member in sorted(members):
            arcname = f"{root_dir}/{member}"
            assert arcname.isascii(), arcname
            zf.writestr(deterministic_zipinfo(arcname), (out_dir / member).read_bytes())
        for info in zf.filelist:
            info.external_attr = 0


def build_export(out_dir: Path, *, graph_checkpoint_hash: bytes) -> tuple[str, bytes, bytes]:
    out_dir.mkdir(parents=True, exist_ok=True)

    event_bytes = (SOURCE_EVENT_DIR / "expected-event.cbor").read_bytes()
    event_payload = cbor2.loads((SOURCE_EVENT_DIR / "expected-event-payload.cbor").read_bytes())
    ledger_scope = event_payload["ledger_scope"]
    extension = event_payload["extensions"][SUPERSEDES_CHAIN_ID_EVENT_EXTENSION]
    predecessor_chain_id = extension["chain_id"]
    predecessor_checkpoint_hash = extension["checkpoint_hash"]
    assert isinstance(predecessor_chain_id, bytes)
    assert isinstance(predecessor_checkpoint_hash, bytes) and len(predecessor_checkpoint_hash) == 32

    events_cbor = dcbor([cbor2.loads(event_bytes)])
    canon_hash = canonical_event_hash(ledger_scope, event_payload)
    leaf_hash = merkle_leaf_hash(canon_hash)

    seed, pubkey_raw = load_issuer_key()
    kid = derive_kid(pubkey_raw)
    signing_key_registry_cbor = dcbor([{
        "kid": kid,
        "pubkey": pubkey_raw,
        "suite_id": SUITE_ID_PHASE_1,
        "status": 0,
        "valid_from": event_payload["header"]["authored_at"],
        "valid_to": None,
        "supersedes": None,
        "attestation": None,
    }])

    domain_registry = {
        "governance": {
            "ruleset_id": "x-trellis-test/governance-ruleset-v1",
            "ruleset_digest": sha256(b"adr0066-supersession-graph-ruleset"),
        },
        "event_types": {
            "wos.case.supersessionStarted": {
                "privacy_class": "public",
                "commitment_schema": "x-trellis-test/adr0066-supersession-started-v1",
            }
        },
        "classifications": ["x-trellis-test/unclassified"],
        "role_vocabulary": ["x-trellis-test/role-author"],
    }
    domain_registry_cbor = dcbor(domain_registry)
    registry_digest = sha256(domain_registry_cbor)
    registry_binding = {
        "registry_digest": registry_digest,
        "registry_format": 1,
        "registry_version": "x-trellis-test/adr0066-registry-v1",
        "bound_at_sequence": 0,
    }

    checkpoint_payload = {
        "version": 1,
        "scope": ledger_scope,
        "tree_size": 1,
        "tree_head_hash": leaf_hash,
        "timestamp": CHECKPOINT_TIMESTAMP,
        "anchor_ref": None,
        "prev_checkpoint_hash": None,
        "extensions": None,
    }
    head_checkpoint_digest = checkpoint_digest(ledger_scope, checkpoint_payload)
    checkpoint_bytes = cose_sign1(seed, kid, dcbor(checkpoint_payload))
    checkpoints_cbor = dcbor([cbor2.loads(checkpoint_bytes)])

    inclusion_proofs_cbor = dcbor({0: {
        "leaf_index": 0,
        "tree_size": 1,
        "leaf_hash": leaf_hash,
        "audit_path": [],
    }})
    consistency_proofs_cbor = dcbor([])
    graph_json = canonical_graph_json(
        head_chain_id=ledger_scope,
        predecessor_chain_id=predecessor_chain_id,
        predecessor_checkpoint_hash=graph_checkpoint_hash,
    )
    graph_digest = sha256(graph_json)

    write_bytes(out_dir / "010-events.cbor", events_cbor)
    write_bytes(out_dir / "020-inclusion-proofs.cbor", inclusion_proofs_cbor)
    write_bytes(out_dir / "025-consistency-proofs.cbor", consistency_proofs_cbor)
    write_bytes(out_dir / "030-signing-key-registry.cbor", signing_key_registry_cbor)
    write_bytes(out_dir / "040-checkpoints.cbor", checkpoints_cbor)
    write_bytes(out_dir / "050-registries" / f"{registry_digest.hex()}.cbor", domain_registry_cbor)
    write_bytes(out_dir / "064-supersession-graph.json", graph_json)

    readme = (
        "# Trellis Export (Fixture) - verify/017 supersession graph\n"
        "\n"
        f"- scope: `{ledger_scope.decode('utf-8')}`\n"
        f"- head_checkpoint_digest: `{head_checkpoint_digest.hex()}`\n"
        f"- superseded_chain_id: `{predecessor_chain_id.hex()}`\n"
        f"- graph_digest: `{graph_digest.hex()}`\n"
    ).encode("utf-8")
    verify_sh = b"#!/bin/sh\nset -eu\necho \"Run a Trellis verifier against this export.\" >&2\n"
    write_bytes(out_dir / "090-verify.sh", verify_sh)
    write_bytes(out_dir / "098-README.md", readme)

    manifest_payload = {
        "format": "trellis-export/1",
        "version": 1,
        "generator": "x-trellis-test/supersession-graph-generator-015",
        "generated_at": GENERATED_AT,
        "scope": ledger_scope,
        "tree_size": 1,
        "head_checkpoint_digest": head_checkpoint_digest,
        "registry_bindings": [registry_binding],
        "signing_key_registry_digest": sha256(signing_key_registry_cbor),
        "events_digest": sha256(events_cbor),
        "checkpoints_digest": sha256(checkpoints_cbor),
        "inclusion_proofs_digest": sha256(inclusion_proofs_cbor),
        "consistency_proofs_digest": sha256(consistency_proofs_cbor),
        "payloads_inlined": False,
        "external_anchors": [],
        "posture_declaration": {
            "provider_readable": True,
            "reader_held": False,
            "delegated_compute": False,
            "external_anchor_required": False,
            "external_anchor_name": None,
            "recovery_without_user": True,
            "metadata_leakage_summary": "ADR 0066 supersession graph fixture.",
        },
        "head_format_version": 1,
        "omitted_payload_checks": [],
        "extensions": {
            SUPERSESSION_GRAPH_EXPORT_EXTENSION: {
                "graph_digest": graph_digest,
                "predecessor_count": 1,
            }
        },
    }
    manifest_bytes = cose_sign1(seed, kid, dcbor(manifest_payload))
    write_bytes(out_dir / "000-manifest.cbor", manifest_bytes)

    root_dir = f"trellis-export-{ledger_scope.decode('utf-8')}-1-{leaf_hash.hex()[:8]}"
    members = [
        "000-manifest.cbor",
        "010-events.cbor",
        "020-inclusion-proofs.cbor",
        "025-consistency-proofs.cbor",
        "030-signing-key-registry.cbor",
        "040-checkpoints.cbor",
        f"050-registries/{registry_digest.hex()}.cbor",
        "064-supersession-graph.json",
        "090-verify.sh",
        "098-README.md",
    ]
    write_zip(out_dir, root_dir=root_dir, members=members)
    write_bytes(out_dir / "input-ledger-state.cbor", dcbor({
        "version": 1,
        "scope": ledger_scope,
        "tree_size": 1,
        "root_dir": root_dir,
        "members": members,
    }))
    return root_dir, predecessor_checkpoint_hash, canon_hash


def write_manifest(out_dir: Path, *, op: str, title: str, tamper_kind: str | None, event_hash: bytes | None) -> None:
    coverage = """[coverage]
tr_core = [
    "TR-CORE-001",
    "TR-CORE-018",
    "TR-CORE-030",
    "TR-CORE-050",
    "TR-CORE-068",
    "TR-CORE-169",
    "TR-CORE-170",
]
"""
    report_lines = [
        "[expected.report]",
        "structure_verified  = true",
        f"integrity_verified  = {'false' if tamper_kind else 'true'}",
        "readability_verified = true",
    ]
    if tamper_kind is not None:
        report_lines.append(f'tamper_kind = "{tamper_kind}"')
    if event_hash is not None:
        report_lines.append(f'failing_event_id = "{event_hash.hex()}"')
    manifest = (
        f'id          = "{out_dir.parent.name}/{out_dir.name}"\n'
        f'op          = "{op}"\n'
        'status      = "active"\n'
        f'description = """{title}"""\n'
        "\n"
        + coverage
        + "\n[inputs]\n"
        + 'export_zip = "input-export.zip"\n'
        + "\n"
        + "\n".join(report_lines)
        + "\n\n[derivation]\n"
        + 'document = "derivation.md"\n'
    )
    write_bytes(out_dir / "manifest.toml", manifest.encode("utf-8"))


def write_derivation(out_dir: Path, *, tampered: bool) -> None:
    body = (
        f"# Derivation - `{out_dir.parent.name}/{out_dir.name}`\n\n"
        "This vector is generated by `fixtures/vectors/_generator/"
        "gen_supersession_graph_export_015.py` from `append/015-supersession`.\n\n"
        "It packages the supersession-start event into a deterministic Trellis "
        "export ZIP with `064-supersession-graph.json` bound by "
        "`ExportManifestPayload.extensions[\"trellis.export.supersession-graph.v1\"].graph_digest`.\n\n"
    )
    if tampered:
        body += (
            "The graph remains canonical JSON and the manifest digest is recomputed, "
            "but the predecessor `checkpoint_hash` is replaced with 32 zero bytes. "
            "The verifier must therefore reject the export with "
            "`supersession_graph_linkage_mismatch` because the graph row no longer "
            "byte-matches the event's `trellis.supersedes-chain-id.v1` extension.\n"
        )
    else:
        body += (
            "The graph `head_chain_id` equals `manifest.scope`, and its only "
            "predecessor row byte-matches the event's "
            "`trellis.supersedes-chain-id.v1` extension.\n"
        )
    write_bytes(out_dir / "derivation.md", body.encode("utf-8"))


def main() -> None:
    _, expected_checkpoint_hash, canon_hash = build_export(
        OUT_VERIFY,
        graph_checkpoint_hash=cbor2.loads(
            (SOURCE_EVENT_DIR / "input-supersedes-chain-id.cbor").read_bytes()
        )["checkpoint_hash"],
    )
    assert expected_checkpoint_hash == cbor2.loads(
        (SOURCE_EVENT_DIR / "input-supersedes-chain-id.cbor").read_bytes()
    )["checkpoint_hash"]
    write_manifest(
        OUT_VERIFY,
        op="verify",
        title="Positive ADR 0066 export with manifest-bound supersession graph.",
        tamper_kind=None,
        event_hash=None,
    )
    write_derivation(OUT_VERIFY, tampered=False)

    build_export(OUT_TAMPER, graph_checkpoint_hash=b"\x00" * 32)
    write_manifest(
        OUT_TAMPER,
        op="tamper",
        title="Manifest-bound supersession graph whose predecessor row does not byte-match the event extension.",
        tamper_kind="supersession_graph_linkage_mismatch",
        event_hash=canon_hash,
    )
    write_derivation(OUT_TAMPER, tampered=True)

    print(f"verify/017 zip_sha256={sha256((OUT_VERIFY / 'input-export.zip').read_bytes()).hex()}")
    print(f"tamper/046 zip_sha256={sha256((OUT_TAMPER / 'input-export.zip').read_bytes()).hex()}")
    print(f"export_manifest_digest={export_manifest_digest(cbor2.loads((SOURCE_EVENT_DIR / 'expected-event-payload.cbor').read_bytes())['ledger_scope'], cbor2.loads(cbor2.loads((OUT_VERIFY / '000-manifest.cbor').read_bytes()).value[2])).hex()}")


if __name__ == "__main__":
    main()
