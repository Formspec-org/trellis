"""Offline export ZIP and tamper-ledger verification (Core §19)."""

from __future__ import annotations

import io
import zipfile
from collections import defaultdict
from dataclasses import dataclass, field
from typing import Any, Optional

import cbor2
from cbor2 import CBORTag
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

from trellis_py.codec import (
    domain_separated_sha256,
    encode_bstr,
    encode_tstr,
    encode_uint,
    sig_structure_bytes,
)
from trellis_py.constants import (
    ALG_EDDSA,
    AUTHOR_EVENT_DOMAIN,
    CHECKPOINT_DOMAIN,
    CONTENT_DOMAIN,
    COSE_LABEL_ALG,
    COSE_LABEL_KID,
    COSE_LABEL_SUITE_ID,
    EVENT_DOMAIN,
    MERKLE_INTERIOR_DOMAIN,
    MERKLE_LEAF_DOMAIN,
    POSTURE_DECLARATION_DOMAIN,
    SUITE_ID_PHASE_1,
)

ATTACHMENT_EXPORT_EXTENSION = "trellis.export.attachments.v1"
ATTACHMENT_EVENT_EXTENSION = "trellis.evidence-attachment-binding.v1"
SIGNATURE_EXPORT_EXTENSION = "trellis.export.signature-affirmations.v1"
INTAKE_EXPORT_EXTENSION = "trellis.export.intake-handoffs.v1"
WOS_SIGNATURE_AFFIRMATION_EVENT_TYPE = "wos.kernel.signatureAffirmation"
WOS_INTAKE_ACCEPTED_EVENT_TYPE = "wos.kernel.intakeAccepted"
WOS_CASE_CREATED_EVENT_TYPE = "wos.kernel.caseCreated"


@dataclass
class VerificationFailure:
    kind: str
    location: str


@dataclass
class PostureTransitionOutcome:
    transition_id: str
    kind: str
    event_index: int
    from_state: str
    to_state: str
    continuity_verified: bool
    declaration_resolved: bool
    attestations_verified: bool
    failures: list[str] = field(default_factory=list)


@dataclass
class VerificationReport:
    structure_verified: bool = False
    integrity_verified: bool = False
    readability_verified: bool = False
    event_failures: list[VerificationFailure] = field(default_factory=list)
    checkpoint_failures: list[VerificationFailure] = field(default_factory=list)
    proof_failures: list[VerificationFailure] = field(default_factory=list)
    posture_transitions: list[PostureTransitionOutcome] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)

    @staticmethod
    def fatal(kind: str, warning: str) -> VerificationReport:
        return VerificationReport(
            structure_verified=False,
            integrity_verified=False,
            readability_verified=False,
            event_failures=[VerificationFailure(kind, "structure")],
            warnings=[warning],
        )


class VerifyError(Exception):
    pass


@dataclass
class ParsedSign1:
    protected_bytes: bytes
    kid: bytes
    alg: int
    suite_id: int
    payload: Optional[bytes]
    signature: bytes


@dataclass
class SigningKeyEntry:
    public_key: bytes
    status: int
    valid_to: Optional[int]


@dataclass
class NonSigningKeyEntry:
    """A reserved non-signing `KeyEntry` (Core §8.7 / ADR 0006).

    Phase-1 verifiers track these so a signature attempt under a kid registered
    as `tenant-root`, `scope`, `subject`, or `recovery` can be flagged with
    `key_class_mismatch` (Core §8.7.3 step 4) rather than the generic
    `unresolvable_manifest_kid` failure.
    """

    class_: str  # `class` is a Python keyword; trailing underscore by convention.


# Reserved non-signing class literals (Core §8.7).
_RESERVED_NON_SIGNING_KIND = frozenset({"tenant-root", "scope", "subject", "recovery"})


@dataclass
class RegistryBindingInfo:
    digest_hex: str
    bound_at_sequence: int


@dataclass
class BoundRegistry:
    event_types: list[str]
    classifications: list[str]


@dataclass
class AttachmentBindingDetails:
    attachment_id: str
    slot_path: str
    media_type: str
    byte_length: int
    attachment_sha256: bytes
    payload_content_hash: bytes
    filename: Optional[str]
    prior_binding_hash: Optional[bytes]


@dataclass
class EventDetails:
    scope: bytes
    sequence: int
    authored_at: int
    event_type: str
    classification: str
    prev_hash: Optional[bytes]
    author_event_hash: bytes
    content_hash: bytes
    canonical_event_hash: bytes
    payload_ref_inline: Optional[bytes]
    payload_ref_external: bool
    transition: Optional["TransitionDetails"]
    attachment_binding: Optional[AttachmentBindingDetails] = None


@dataclass
class TransitionDetails:
    # "custody-model" or "disclosure-profile" — routes shadow-state lookup
    # and attestation rule per Companion Appendix A.5.3. Mirrors the Rust
    # TransitionKind enum in trellis-verify/src/lib.rs.
    kind: str
    transition_id: str
    from_state: str
    to_state: str
    declaration_digest: bytes
    attestation_classes: list[str]
    # Only populated for disclosure-profile transitions. Drives the
    # Narrowing / Widening / Orthogonal attestation rule (A.5.3 step 4).
    scope_change: Optional[str] = None


def _sha256(b: bytes) -> bytes:
    import hashlib

    return hashlib.sha256(b).digest()


def _hex(b: bytes) -> str:
    return b.hex()


def _hex_decode(text: str) -> bytes:
    if len(text) % 2:
        raise VerifyError("hex string must have even length")
    out = bytearray()
    for i in range(0, len(text), 2):
        out.append(int(text[i : i + 2], 16))
    return bytes(out)


def _decode_value(data: bytes) -> Any:
    try:
        return cbor2.loads(data)
    except Exception as exc:  # noqa: BLE001
        raise VerifyError(f"failed to decode CBOR: {exc}") from exc


def _map_lookup_str(m: dict, key: str) -> Any:
    if key not in m:
        raise VerifyError(f"missing `{key}`")
    return m[key]


def _map_lookup_optional_str(m: dict, key: str) -> Any:
    return m.get(key)


def _map_lookup_bytes(m: dict, key: str) -> bytes:
    v = _map_lookup_str(m, key)
    if not isinstance(v, bytes):
        raise VerifyError(f"`{key}` is not bytes")
    return v


def _map_lookup_fixed_bytes(m: dict, key: str, n: int) -> bytes:
    b = _map_lookup_bytes(m, key)
    if len(b) != n:
        raise VerifyError(f"`{key}` must be {n} bytes")
    return b


def _map_lookup_u64(m: dict, key: str) -> int:
    v = _map_lookup_str(m, key)
    if not isinstance(v, int) or v < 0:
        raise VerifyError(f"`{key}` is not an unsigned integer")
    return v


def _map_lookup_bool(m: dict, key: str) -> bool:
    v = _map_lookup_str(m, key)
    if not isinstance(v, bool):
        raise VerifyError(f"`{key}` is not a bool")
    return v


def _map_lookup_optional_fixed_bytes(m: dict, key: str, n: int) -> Optional[bytes]:
    v = m.get(key)
    if v is None:
        return None
    if not isinstance(v, bytes) or len(v) != n:
        raise VerifyError(f"`{key}` must be {n} bytes or absent")
    return v


def _parse_sha256_prefix_text(value: str) -> bytes:
    if not value.startswith("sha256:"):
        raise VerifyError("hash text must use sha256: prefix")
    hx = value[len("sha256:") :]
    raw = _hex_decode(hx)
    if len(raw) != 32:
        raise VerifyError("sha256 hash text must be 32 bytes")
    return raw


def _map_lookup_optional_bytes(m: dict, key: str) -> Optional[bytes]:
    v = _map_lookup_optional_str(m, key)
    if v is None:
        return None
    if v is False:  # cbor2 undefined? unlikely
        pass
    if isinstance(v, bytes):
        return v
    if v is None or (isinstance(v, type(None))):  # noqa: E721
        return None
    raise VerifyError(f"`{key}` is neither bytes nor null")


def _map_lookup_int_label(m: dict, label: int) -> int:
    if label not in m:
        raise VerifyError(f"missing COSE label {label} integer")
    v = m[label]
    if not isinstance(v, int):
        raise VerifyError("not int")
    return v


def _map_lookup_int_label_bytes(m: dict, label: int) -> bytes:
    if label not in m:
        raise VerifyError(f"missing COSE label {label} bytes")
    v = m[label]
    if not isinstance(v, bytes):
        raise VerifyError("not bytes")
    return v


def _parse_sign1_value(value: Any) -> ParsedSign1:
    if not isinstance(value, CBORTag) or value.tag != 18:
        raise VerifyError("value is not a tag-18 COSE_Sign1 item")
    body = value.value
    if not isinstance(body, list) or len(body) != 4:
        raise VerifyError("COSE_Sign1 body does not contain four fields")
    protected_bytes = body[0]
    unprotected = body[1]
    payload_field = body[2]
    sig_field = body[3]
    if not isinstance(protected_bytes, bytes):
        raise VerifyError("protected header is not a byte string")
    if not isinstance(unprotected, dict) or len(unprotected) != 0:
        raise VerifyError("unprotected header map must be empty")
    if isinstance(payload_field, bytes):
        payload: Optional[bytes] = payload_field
    elif payload_field is None:
        payload = None
    else:
        raise VerifyError("payload is neither bytes nor null")
    if not isinstance(sig_field, bytes) or len(sig_field) != 64:
        raise VerifyError("signature is not 64 bytes")
    protected_value = _decode_value(protected_bytes)
    if not isinstance(protected_value, dict):
        raise VerifyError("protected header does not decode to a map")
    kid = _map_lookup_int_label_bytes(protected_value, COSE_LABEL_KID)
    alg = _map_lookup_int_label(protected_value, COSE_LABEL_ALG)
    suite_id = _map_lookup_int_label(protected_value, COSE_LABEL_SUITE_ID)
    return ParsedSign1(
        protected_bytes=protected_bytes,
        kid=kid,
        alg=alg,
        suite_id=suite_id,
        payload=payload,
        signature=sig_field,
    )


def _parse_sign1_bytes(data: bytes) -> ParsedSign1:
    return _parse_sign1_value(_decode_value(data))


def _parse_sign1_array(data: bytes) -> list[ParsedSign1]:
    v = _decode_value(data)
    if not isinstance(v, list):
        raise VerifyError("expected a dCBOR array")
    return [_parse_sign1_value(item) for item in v]


def _verify_signature(item: ParsedSign1, public_key_bytes: bytes) -> bool:
    if item.payload is None:
        return False
    try:
        vk = Ed25519PublicKey.from_public_bytes(public_key_bytes)
    except Exception:  # noqa: BLE001
        return False
    msg = sig_structure_bytes(item.protected_bytes, item.payload)
    try:
        vk.verify(item.signature, msg)
        return True
    except Exception:  # noqa: BLE001
        return False


def _encode_tstr_cbor(s: str) -> bytes:
    return encode_tstr(s)


def _authored_preimage_from_canonical(canonical_event_bytes: bytes) -> Optional[bytes]:
    key = _encode_tstr_cbor("author_event_hash")
    key_pos = canonical_event_bytes.rfind(key)
    if key_pos < 0:
        return None
    value_pos = key_pos + len(key)
    if len(canonical_event_bytes) != value_pos + 34:
        return None
    if canonical_event_bytes[value_pos] != 0x58 or canonical_event_bytes[value_pos + 1] != 0x20:
        return None
    new_prefix = canonical_event_bytes[0] - 1
    if new_prefix < 0:
        return None
    authored = bytearray()
    authored.append(new_prefix)
    authored.extend(canonical_event_bytes[1:key_pos])
    return bytes(authored)


def _recompute_author_event_hash(canonical_event_bytes: bytes) -> Optional[bytes]:
    authored = _authored_preimage_from_canonical(canonical_event_bytes)
    if authored is None:
        return None
    return domain_separated_sha256(AUTHOR_EVENT_DOMAIN, authored)


def _recompute_canonical_event_hash(scope: bytes, canonical_event_bytes: bytes) -> bytes:
    preimage = bytearray()
    preimage.append(0xA3)
    preimage.extend(encode_tstr("version"))
    preimage.extend(encode_uint(1))
    preimage.extend(encode_tstr("ledger_scope"))
    preimage.extend(encode_bstr(scope))
    preimage.extend(encode_tstr("event_payload"))
    preimage.extend(canonical_event_bytes)
    return domain_separated_sha256(EVENT_DOMAIN, bytes(preimage))


def _decode_transition_details(extensions: dict) -> Optional[TransitionDetails]:
    ext_custody = extensions.get("trellis.custody-model-transition.v1")
    ext_disclosure = extensions.get("trellis.disclosure-profile-transition.v1")
    if ext_custody is not None and ext_disclosure is not None:
        raise VerifyError(
            "extensions MUST NOT contain both trellis.custody-model-transition.v1 and "
            "trellis.disclosure-profile-transition.v1 on the same event"
        )
    if ext_custody is not None:
        return _decode_custody_model_transition(ext_custody)
    if ext_disclosure is not None:
        return _decode_disclosure_profile_transition(ext_disclosure)
    return None


def _decode_custody_model_transition(ext: object) -> TransitionDetails:
    if not isinstance(ext, dict):
        raise VerifyError("custody-model transition extension is not a map")
    tid = str(_map_lookup_str(ext, "transition_id"))
    fs = str(_map_lookup_str(ext, "from_custody_model"))
    ts = str(_map_lookup_str(ext, "to_custody_model"))
    _ = _map_lookup_u64(ext, "effective_at")
    dd = _map_lookup_fixed_bytes(ext, "declaration_doc_digest", 32)
    classes = _decode_attestation_classes(ext)
    return TransitionDetails(
        kind="custody-model",
        transition_id=tid,
        from_state=fs,
        to_state=ts,
        declaration_digest=dd,
        attestation_classes=classes,
        scope_change=None,
    )


def _decode_disclosure_profile_transition(ext: object) -> TransitionDetails:
    if not isinstance(ext, dict):
        raise VerifyError("disclosure-profile transition extension is not a map")
    tid = str(_map_lookup_str(ext, "transition_id"))
    fs = str(_map_lookup_str(ext, "from_disclosure_profile"))
    ts = str(_map_lookup_str(ext, "to_disclosure_profile"))
    _ = _map_lookup_u64(ext, "effective_at")
    dd = _map_lookup_fixed_bytes(ext, "declaration_doc_digest", 32)
    sc = str(_map_lookup_str(ext, "scope_change"))
    classes = _decode_attestation_classes(ext)
    return TransitionDetails(
        kind="disclosure-profile",
        transition_id=tid,
        from_state=fs,
        to_state=ts,
        declaration_digest=dd,
        attestation_classes=classes,
        scope_change=sc,
    )


def _decode_attestation_classes(ext: dict) -> list[str]:
    atts = _map_lookup_str(ext, "attestations")
    if not isinstance(atts, list):
        raise VerifyError("attestations not array")
    classes: list[str] = []
    for item in atts:
        if not isinstance(item, dict):
            continue
        ac = item.get("authority_class")
        if isinstance(ac, str):
            classes.append(ac)
    return classes


def _decode_attachment_binding_details(exts: dict) -> Optional[AttachmentBindingDetails]:
    ext = exts.get(ATTACHMENT_EVENT_EXTENSION)
    if ext is None:
        return None
    if not isinstance(ext, dict):
        raise VerifyError("attachment binding extension is not a map")
    attachment_id = str(_map_lookup_str(ext, "attachment_id"))
    slot_path = str(_map_lookup_str(ext, "slot_path"))
    media_type = str(_map_lookup_str(ext, "media_type"))
    byte_length = _map_lookup_u64(ext, "byte_length")
    att_txt = str(_map_lookup_str(ext, "attachment_sha256"))
    pay_txt = str(_map_lookup_str(ext, "payload_content_hash"))
    attachment_sha256 = _parse_sha256_prefix_text(att_txt)
    payload_content_hash = _parse_sha256_prefix_text(pay_txt)
    fn_raw = _map_lookup_optional_str(ext, "filename")
    filename: Optional[str]
    if fn_raw is None:
        filename = None
    elif isinstance(fn_raw, str):
        filename = fn_raw
    else:
        raise VerifyError("filename invalid")
    prior_binding_hash: Optional[bytes]
    pb = ext.get("prior_binding_hash")
    if pb is None:
        prior_binding_hash = None
    elif isinstance(pb, bytes) and len(pb) == 32:
        prior_binding_hash = pb
    else:
        raise VerifyError("prior_binding_hash is neither 32 bytes nor null")
    return AttachmentBindingDetails(
        attachment_id=attachment_id,
        slot_path=slot_path,
        media_type=media_type,
        byte_length=byte_length,
        attachment_sha256=attachment_sha256,
        payload_content_hash=payload_content_hash,
        filename=filename,
        prior_binding_hash=prior_binding_hash,
    )


def _normalize_cbor_compare(obj: Any) -> Any:
    """RFC8949-style map key ordering for semantic nested-map equality."""
    if isinstance(obj, dict):
        parts: list[tuple[bytes, Any, Any]] = []
        for k, v in obj.items():
            nk = _normalize_cbor_compare(k)
            nv = _normalize_cbor_compare(v)
            kb = cbor2.dumps(nk, canonical=True)
            parts.append((kb, nk, nv))
        parts.sort(key=lambda x: x[0])
        return tuple((k, v) for _, k, v in parts)
    if isinstance(obj, list):
        return tuple(_normalize_cbor_compare(x) for x in obj)
    if isinstance(obj, CBORTag):
        return ("tag", obj.tag, _normalize_cbor_compare(obj.value))
    return obj


def _cbor_nested_semantic_eq(a: Any, b: Any) -> bool:
    return _normalize_cbor_compare(a) == _normalize_cbor_compare(b)


def _decode_event_details(event: ParsedSign1) -> EventDetails:
    if event.payload is None:
        raise VerifyError("detached event payloads are out of scope")
    payload_value = _decode_value(event.payload)
    if not isinstance(payload_value, dict):
        raise VerifyError("event payload root is not a map")
    scope = _map_lookup_bytes(payload_value, "ledger_scope")
    sequence = _map_lookup_u64(payload_value, "sequence")
    prev_raw = _map_lookup_optional_bytes(payload_value, "prev_hash")
    author_event_hash = _map_lookup_fixed_bytes(payload_value, "author_event_hash", 32)
    content_hash = _map_lookup_fixed_bytes(payload_value, "content_hash", 32)
    canonical_event_hash = _recompute_canonical_event_hash(scope, event.payload)
    header = _map_lookup_str(payload_value, "header")
    if not isinstance(header, dict):
        raise VerifyError("header not map")
    authored_at = _map_lookup_u64(header, "authored_at")
    et = _map_lookup_bytes(header, "event_type")
    cl = _map_lookup_bytes(header, "classification")
    if not isinstance(et, bytes) or not isinstance(cl, bytes):
        raise VerifyError("event_type/classification")
    event_type = et.decode("utf-8")
    classification = cl.decode("utf-8")
    pr = _map_lookup_str(payload_value, "payload_ref")
    if not isinstance(pr, dict):
        raise VerifyError("payload_ref")
    rt = pr.get("ref_type")
    if rt == "inline":
        inline = _map_lookup_bytes(pr, "ciphertext")
        external = False
    elif rt == "external":
        inline = None
        external = True
    else:
        raise VerifyError("payload_ref.ref_type unsupported")
    exts = payload_value.get("extensions")
    transition: Optional[TransitionDetails] = None
    attachment_binding: Optional[AttachmentBindingDetails] = None
    if isinstance(exts, dict):
        transition = _decode_transition_details(exts)
        attachment_binding = _decode_attachment_binding_details(exts)
    elif exts is not None:
        raise VerifyError("extensions not map")
    return EventDetails(
        scope=scope,
        sequence=sequence,
        authored_at=authored_at,
        event_type=event_type,
        classification=classification,
        prev_hash=prev_raw,
        author_event_hash=author_event_hash,
        content_hash=content_hash,
        canonical_event_hash=canonical_event_hash,
        payload_ref_inline=inline,
        payload_ref_external=external,
        transition=transition,
        attachment_binding=attachment_binding,
    )


def _parse_signing_key_registry(data: bytes) -> dict[bytes, SigningKeyEntry]:
    """Backwards-compat wrapper that drops the non-signing map.

    Call sites that need to detect class-confusion attacks per Core §8.7.3
    step 4 should use ``_parse_key_registry`` directly.
    """

    signing, _non_signing = _parse_key_registry(data)
    return signing


def _parse_key_registry(
    data: bytes,
) -> tuple[dict[bytes, SigningKeyEntry], dict[bytes, NonSigningKeyEntry]]:
    """Parses the unified key registry per Core §8 (ADR 0006).

    Verifier dispatch follows Core §8.7.3 step 1: an entry whose top-level map
    carries a `kind` field is `KeyEntry` (§8.7.1); an entry without `kind` is
    the legacy `SigningKeyEntry` flat shape (§8.2). Both paths populate the
    signing-key map identically for `kind = "signing"` and the legacy shape.

    Reserved non-signing classes (`tenant-root`, `scope`, `subject`,
    `recovery`) and unknown extension `tstr` kinds are NOT inserted into the
    signing-key map — they cannot resolve a COSE_Sign1 protected-header `kid`.
    They are returned in the second map so the caller can emit
    `key_class_mismatch` (Core §8.7.3 step 4) when an event tries to sign
    under such a kid.

    Per Core §8.7.6 the wire string `"wrap"` is a deprecated synonym for
    `"subject"`; this parser normalizes the stored class label.
    """

    v = _decode_value(data)
    if not isinstance(v, list):
        raise VerifyError("signing-key registry root is not an array")

    reg: dict[bytes, SigningKeyEntry] = {}
    non_signing: dict[bytes, NonSigningKeyEntry] = {}

    for entry in v:
        if not isinstance(entry, dict):
            raise VerifyError("registry entry not map")

        # Core §8.7.3 step 1: dispatch on presence of `kind`.
        kind_raw = entry.get("kind")
        if kind_raw is not None and not isinstance(kind_raw, str):
            raise VerifyError("registry entry `kind` is not a text string")
        # Core §8.7.6: normalize legacy `"wrap"` → `"subject"`.
        kind_norm: Optional[str] = (
            "subject" if kind_raw == "wrap" else kind_raw
        )

        kid = _map_lookup_bytes(entry, "kid")

        if kind_norm is None or kind_norm == "signing":
            # Legacy `SigningKeyEntry` (no `kind`) OR new `KeyEntrySigning`.
            pubkey = _map_lookup_fixed_bytes(entry, "pubkey", 32)
            status = _map_lookup_u64(entry, "status")
            vt_raw = _map_lookup_optional_str(entry, "valid_to")
            valid_to: Optional[int]
            if vt_raw is None:
                valid_to = None
            elif isinstance(vt_raw, int):
                valid_to = vt_raw
            else:
                raise VerifyError("valid_to invalid")
            reg[kid] = SigningKeyEntry(
                public_key=pubkey, status=status, valid_to=valid_to
            )
        elif kind_norm in _RESERVED_NON_SIGNING_KIND:
            # Core §8.7.3 step 3: reserved non-signing class. Phase-1
            # verifier does not validate class-specific inner fields (the
            # deep validation rides Phase-2+ activation per ADR 0006), but
            # it DOES enforce the structural-shape gate of §8.7.1: the entry
            # MUST carry an `attributes` map. Absent or wrong-typed →
            # `key_entry_attributes_shape_mismatch` (TR-CORE-048).
            attrs = entry.get("attributes")
            if not isinstance(attrs, dict):
                raise VerifyError(
                    "key_entry_attributes_shape_mismatch: KeyEntry of "
                    f"kind=\"{kind_norm}\" missing required `attributes` map "
                    "(Core §8.7.1)"
                )
            non_signing[kid] = NonSigningKeyEntry(class_=kind_norm)
        else:
            # Core §8.7.3 step 4 *Unknown `kind`*: forward-compatibility floor.
            non_signing[kid] = NonSigningKeyEntry(class_=kind_norm)

    return reg, non_signing


def _parse_bound_registry(data: bytes) -> BoundRegistry:
    v = _decode_value(data)
    if not isinstance(v, dict):
        raise VerifyError("bound registry root is not a map")
    et = _map_lookup_str(v, "event_types")
    if not isinstance(et, dict):
        raise VerifyError("event_types")
    event_types = [str(k) for k in et.keys() if isinstance(k, str)]
    cl_arr = _map_lookup_str(v, "classifications")
    if not isinstance(cl_arr, list):
        raise VerifyError("classifications")
    classifications = [str(x) for x in cl_arr if isinstance(x, str)]
    return BoundRegistry(event_types=event_types, classifications=classifications)


def _event_identity(event: ParsedSign1) -> tuple[bytes, bytes]:
    d = _decode_event_details(event)
    return d.scope, d.canonical_event_hash


def _parse_custody_model(data: bytes) -> str:
    v = _decode_value(data)
    if not isinstance(v, dict):
        raise VerifyError("posture declaration root is not a map")
    cm = _map_lookup_str(v, "custody_model")
    if not isinstance(cm, dict):
        raise VerifyError("custody_model")
    return str(_map_lookup_str(cm, "custody_model_id"))


def _parse_disclosure_profile(data: bytes) -> str:
    v = _decode_value(data)
    if not isinstance(v, dict):
        raise VerifyError("posture declaration root is not a map")
    return str(_map_lookup_str(v, "disclosure_profile"))


def _custody_rank(value: str) -> int:
    return {"CM-A": 3, "CM-B": 2, "CM-C": 1}.get(value, 0)


def _requires_dual_attestation(from_state: str, to_state: str) -> bool:
    return _custody_rank(to_state) > _custody_rank(from_state)


def _verify_event_set(
    events: list[ParsedSign1],
    registry: dict[bytes, SigningKeyEntry],
    initial_posture_declaration: Optional[bytes],
    posture_declaration: Optional[bytes],
    classify_tamper: bool,
    expected_ledger_scope: Optional[bytes],
    payload_blobs: Optional[dict[bytes, bytes]],
    non_signing_registry: Optional[dict[bytes, NonSigningKeyEntry]] = None,
) -> VerificationReport:
    event_failures: list[VerificationFailure] = []
    posture_transitions: list[PostureTransitionOutcome] = []
    previous_hash: Optional[bytes] = None
    skip_prev = initial_posture_declaration is not None and len(events) == 1
    shadow_custody_model: Optional[str] = None
    shadow_disclosure_profile: Optional[str] = None
    if initial_posture_declaration is not None:
        try:
            shadow_custody_model = _parse_custody_model(initial_posture_declaration)
        except VerifyError:
            shadow_custody_model = None
        try:
            shadow_disclosure_profile = _parse_disclosure_profile(initial_posture_declaration)
        except VerifyError:
            shadow_disclosure_profile = None

    suite_i = SUITE_ID_PHASE_1

    for index, event in enumerate(events):
        key_entry = registry.get(event.kid)
        if key_entry is None:
            # Core §8.7.3 step 4: a kid resolving to a reserved non-signing
            # class fails with `key_class_mismatch` rather than the generic
            # unresolvable-kid path. Recovery-only / tenant-root / scope /
            # subject kids signing canonical events is the canonical
            # class-confusion attack.
            if non_signing_registry is not None:
                ns_entry = non_signing_registry.get(event.kid)
                if ns_entry is not None:
                    return VerificationReport.fatal(
                        "key_class_mismatch",
                        (
                            f"event signed under a `{ns_entry.class_}`-class "
                            "kid; only `signing` keys may sign canonical events "
                            "(Core §8.7.3 step 4)"
                        ),
                    )
            return VerificationReport.fatal(
                "unresolvable_manifest_kid",
                "event kid is not resolvable via the provided signing-key registry",
            )
        if event.alg != ALG_EDDSA or event.suite_id != suite_i:
            return VerificationReport.fatal(
                "unsupported_suite",
                "event protected header does not match the Trellis Phase-1 suite",
            )
        if not _verify_signature(event, key_entry.public_key):
            try:
                loc = _hex(_event_identity(event)[1])
            except Exception:  # noqa: BLE001
                loc = f"event[{index}]"
            event_failures.append(VerificationFailure("signature_invalid", loc))
            continue

        try:
            details = _decode_event_details(event)
        except VerifyError:
            return VerificationReport.fatal(
                "malformed_cose",
                "event payload does not decode as a canonical Trellis event",
            )

        if key_entry.status == 3:
            if key_entry.valid_to is None:
                return VerificationReport.fatal(
                    "signing_key_registry_invalid",
                    "revoked signing-key registry entry is missing valid_to",
                )
            if details.authored_at > key_entry.valid_to:
                event_failures.append(
                    VerificationFailure("revoked_authority", _hex(details.canonical_event_hash))
                )

        if expected_ledger_scope is not None and details.scope != expected_ledger_scope:
            event_failures.append(
                VerificationFailure("scope_mismatch", _hex(details.canonical_event_hash))
            )

        if details.payload_ref_inline is not None:
            expected_ch = domain_separated_sha256(CONTENT_DOMAIN, details.payload_ref_inline)
            if expected_ch != details.content_hash:
                event_failures.append(
                    VerificationFailure("content_hash_mismatch", _hex(details.canonical_event_hash))
                )
        elif details.payload_ref_external:
            if payload_blobs is not None:
                pb = payload_blobs.get(details.content_hash)
                if pb is not None:
                    expected_ch = domain_separated_sha256(CONTENT_DOMAIN, pb)
                    if expected_ch != details.content_hash:
                        event_failures.append(
                            VerificationFailure(
                                "content_hash_mismatch", _hex(details.canonical_event_hash)
                            )
                        )

        payload_bytes = event.payload
        assert payload_bytes is not None
        rah = _recompute_author_event_hash(payload_bytes)
        if rah is None:
            event_failures.append(
                VerificationFailure("author_preimage_invalid", _hex(details.canonical_event_hash))
            )
        elif rah != details.author_event_hash:
            event_failures.append(
                VerificationFailure("hash_mismatch", _hex(details.canonical_event_hash))
            )

        if skip_prev:
            pass
        elif details.sequence == 0:
            if details.prev_hash is not None:
                kind = "event_reorder" if classify_tamper else "prev_hash_mismatch"
                event_failures.append(VerificationFailure(kind, _hex(details.canonical_event_hash)))
        elif previous_hash != details.prev_hash:
            if classify_tamper:
                if previous_hash is None and len(events) == 1:
                    kind = "event_truncation"
                elif previous_hash is None:
                    kind = "event_reorder"
                else:
                    kind = "prev_hash_break"
            else:
                kind = "prev_hash_mismatch"
            event_failures.append(VerificationFailure(kind, _hex(details.canonical_event_hash)))
        previous_hash = details.canonical_event_hash

        if details.transition is not None:
            tr = details.transition
            outcome = PostureTransitionOutcome(
                transition_id=tr.transition_id,
                kind=tr.kind,
                event_index=index,
                from_state=tr.from_state,
                to_state=tr.to_state,
                continuity_verified=True,
                declaration_resolved=True,
                attestations_verified=True,
                failures=[],
            )
            shadow_state = (
                shadow_custody_model
                if tr.kind == "custody-model"
                else shadow_disclosure_profile
            )
            if shadow_state is not None and tr.from_state != shadow_state:
                outcome.continuity_verified = False
                outcome.failures.append("state_continuity_mismatch")
            if posture_declaration is not None:
                exp_dd = domain_separated_sha256(POSTURE_DECLARATION_DOMAIN, posture_declaration)
                if exp_dd != tr.declaration_digest:
                    outcome.continuity_verified = False
                    outcome.declaration_resolved = False
                    outcome.failures.append("posture_declaration_digest_mismatch")
            if tr.kind == "custody-model":
                dual_required = _requires_dual_attestation(tr.from_state, tr.to_state)
            else:
                # Appendix A.5.3 step 4: Narrowing MAY be attested alone;
                # Widening and Orthogonal MUST be dually attested. Unknown
                # values fall through to dual as the conservative default.
                dual_required = tr.scope_change != "Narrowing"
            if dual_required and not (
                any(x == "prior" for x in tr.attestation_classes)
                and any(x == "new" for x in tr.attestation_classes)
            ):
                outcome.attestations_verified = False
                outcome.failures.append("attestation_insufficient")
            if outcome.failures:
                event_failures.append(
                    VerificationFailure(
                        outcome.failures[0], _hex(details.canonical_event_hash)
                    )
                )
            if tr.kind == "custody-model":
                shadow_custody_model = tr.to_state
            else:
                shadow_disclosure_profile = tr.to_state
            posture_transitions.append(outcome)

    posture_ok = all(
        o.continuity_verified and o.declaration_resolved and o.attestations_verified
        for o in posture_transitions
    )
    return VerificationReport(
        structure_verified=True,
        integrity_verified=not event_failures and posture_ok,
        readability_verified=True,
        event_failures=event_failures,
        checkpoint_failures=[],
        proof_failures=[],
        posture_transitions=posture_transitions,
        warnings=[],
    )


def _merkle_leaf_hash(canonical_hash: bytes) -> bytes:
    return domain_separated_sha256(MERKLE_LEAF_DOMAIN, canonical_hash)


def _merkle_interior_hash(left: bytes, right: bytes) -> bytes:
    joined = left + right
    return domain_separated_sha256(MERKLE_INTERIOR_DOMAIN, joined)


def _merkle_root(leaves: list[bytes]) -> bytes:
    if len(leaves) == 0:
        return bytes(32)
    if len(leaves) == 1:
        return leaves[0]
    level = list(leaves)
    while len(level) > 1:
        nxt: list[bytes] = []
        i = 0
        while i < len(level):
            if i + 1 == len(level):
                nxt.append(level[i])
            else:
                nxt.append(_merkle_interior_hash(level[i], level[i + 1]))
            i += 2
        level = nxt
    return level[0]


def _digest_path_from_values(nodes: list[Any]) -> list[bytes]:
    out: list[bytes] = []
    for node in nodes:
        if not isinstance(node, bytes) or len(node) != 32:
            raise ValueError("bad node")
        out.append(node)
    return out


def _inner_proof_size(index: int, size: int) -> int:
    xor = index ^ (size - 1)
    if xor == 0:
        return 0
    return xor.bit_length()


def _decomp_inclusion_proof(index: int, size: int) -> tuple[int, int]:
    inner = _inner_proof_size(index, size)
    border = (index >> inner).bit_count()
    return inner, border


def _chain_inner_merkle(seed: bytes, proof: list[bytes], index: int) -> bytes:
    for i, sibling in enumerate(proof):
        if (index >> i) & 1 == 0:
            seed = _merkle_interior_hash(seed, sibling)
        else:
            seed = _merkle_interior_hash(sibling, seed)
    return seed


def _chain_inner_right_merkle(seed: bytes, proof: list[bytes], index: int) -> bytes:
    for i, sibling in enumerate(proof):
        if (index >> i) & 1 == 1:
            seed = _merkle_interior_hash(sibling, seed)
    return seed


def _chain_border_right_merkle(seed: bytes, proof: list[bytes]) -> bytes:
    for sibling in proof:
        seed = _merkle_interior_hash(sibling, seed)
    return seed


def _root_from_inclusion_proof(
    leaf_index: int, tree_size: int, leaf_hash: bytes, proof: list[bytes]
) -> Optional[bytes]:
    if tree_size == 0 or leaf_index >= tree_size:
        return None
    inner, border = _decomp_inclusion_proof(leaf_index, tree_size)
    if len(proof) != inner + border:
        return None
    node = _chain_inner_merkle(leaf_hash, proof[:inner], leaf_index)
    node = _chain_border_right_merkle(node, proof[inner:])
    return node


def _root_from_consistency_proof(
    size1: int, size2: int, root1: bytes, proof: list[bytes]
) -> Optional[bytes]:
    if size2 < size1:
        return None
    if size1 == size2:
        if proof:
            return None
        return root1
    if size1 == 0:
        return None
    if not proof:
        return None
    inner, border = _decomp_inclusion_proof(size1 - 1, size2)
    shift = (size1 & -size1).bit_length() - 1  # trailing_zeros
    if inner < shift:
        return None
    inner -= shift
    seed = proof[0]
    start = 1
    if size1 == 1 << shift:
        seed = root1
        start = 0
    if len(proof) != start + inner + border:
        return None
    suffix = proof[start:]
    mask = (size1 - 1) >> shift
    hash1 = _chain_inner_right_merkle(seed, suffix[:inner], mask)
    hash1 = _chain_border_right_merkle(hash1, suffix[inner:])
    if hash1 != root1:
        return None
    hash2 = _chain_inner_merkle(seed, suffix[:inner], mask)
    return _chain_border_right_merkle(hash2, suffix[inner:])


def _checkpoint_digest(scope: bytes, payload_bytes: bytes) -> bytes:
    preimage = bytearray()
    preimage.append(0xA3)
    preimage.extend(encode_tstr("scope"))
    preimage.extend(encode_bstr(scope))
    preimage.extend(encode_tstr("version"))
    preimage.extend(encode_uint(1))
    preimage.extend(encode_tstr("checkpoint_payload"))
    preimage.extend(payload_bytes)
    return domain_separated_sha256(CHECKPOINT_DOMAIN, bytes(preimage))


def _map_lookup_array(m: dict, key: str) -> list[Any]:
    v = _map_lookup_str(m, key)
    if not isinstance(v, list):
        raise VerifyError(f"missing or invalid `{key}` array")
    return v


def _map_lookup_map(m: dict, key: str) -> dict:
    v = _map_lookup_str(m, key)
    if not isinstance(v, dict):
        raise VerifyError(f"missing or invalid `{key}` map")
    return v


def _map_lookup_optional_map(m: dict, key: str) -> Optional[dict]:
    v = m.get(key)
    if v is None:
        return None
    if isinstance(v, dict):
        return v
    raise VerifyError(f"`{key}` is not a map")


def _map_lookup_optional_extensions(manifest_map: dict) -> Optional[dict]:
    ext = manifest_map.get("extensions")
    if ext is None:
        return None
    if isinstance(ext, dict):
        return ext
    raise VerifyError("manifest extensions is not a map")


def _parse_attachment_export_extension(
    manifest_map: dict,
) -> Optional[tuple[bytes, bool]]:
    exts = _map_lookup_optional_extensions(manifest_map)
    if exts is None:
        return None
    ext = exts.get(ATTACHMENT_EXPORT_EXTENSION)
    if ext is None:
        return None
    if not isinstance(ext, dict):
        raise VerifyError("attachment export extension is not a map")
    digest = _map_lookup_fixed_bytes(ext, "attachment_manifest_digest", 32)
    inline = _map_lookup_bool(ext, "inline_attachments")
    return digest, inline


def _parse_signature_export_extension(manifest_map: dict) -> Optional[bytes]:
    exts = _map_lookup_optional_extensions(manifest_map)
    if exts is None:
        return None
    ext = exts.get(SIGNATURE_EXPORT_EXTENSION)
    if ext is None:
        return None
    if not isinstance(ext, dict):
        raise VerifyError("signature export extension is not a map")
    return _map_lookup_fixed_bytes(ext, "signature_catalog_digest", 32)


def _parse_intake_export_extension(manifest_map: dict) -> Optional[bytes]:
    exts = _map_lookup_optional_extensions(manifest_map)
    if exts is None:
        return None
    ext = exts.get(INTAKE_EXPORT_EXTENSION)
    if ext is None:
        return None
    if not isinstance(ext, dict):
        raise VerifyError("intake export extension is not a map")
    return _map_lookup_fixed_bytes(ext, "intake_catalog_digest", 32)


def _map_lookup_optional_text(m: dict, key: str) -> Optional[str]:
    v = m.get(key)
    if v is None:
        return None
    if isinstance(v, str):
        return v
    raise VerifyError(f"`{key}` is neither text nor null")


def _parse_intake_handoff_details(value: Any) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise VerifyError("handoff is not a map")
    initiation_mode = str(_map_lookup_str(value, "initiationMode"))
    case_ref = _map_lookup_optional_text(value, "caseRef")
    if initiation_mode == "workflowInitiated" and case_ref is None:
        raise VerifyError("workflowInitiated handoff is missing caseRef")
    if initiation_mode == "publicIntake" and case_ref is not None:
        raise VerifyError("publicIntake handoff caseRef must be null or absent")
    if initiation_mode not in ("workflowInitiated", "publicIntake"):
        raise VerifyError("handoff initiationMode is unsupported")
    definition_ref = _map_lookup_map(value, "definitionRef")
    response_hash = str(_map_lookup_str(value, "responseHash"))
    _parse_sha256_prefix_text(response_hash)
    return {
        "handoff_id": str(_map_lookup_str(value, "handoffId")),
        "initiation_mode": initiation_mode,
        "case_ref": case_ref,
        "definition_url": str(_map_lookup_str(definition_ref, "url")),
        "definition_version": str(_map_lookup_str(definition_ref, "version")),
        "response_ref": str(_map_lookup_str(value, "responseRef")),
        "response_hash": response_hash,
        "validation_report_ref": str(_map_lookup_str(value, "validationReportRef")),
        "ledger_head_ref": str(_map_lookup_str(value, "ledgerHeadRef")),
    }


def _first_array_text(outputs: list[Any]) -> Optional[str]:
    if not outputs:
        return None
    first = outputs[0]
    if isinstance(first, str):
        return first
    return None


def _parse_intake_accepted_record(payload_bytes: bytes) -> dict[str, Any]:
    v = _decode_value(payload_bytes)
    if not isinstance(v, dict):
        raise VerifyError("intake accepted payload root is not a map")
    record_kind = str(_map_lookup_str(v, "recordKind"))
    if record_kind != "intakeAccepted":
        raise VerifyError("intake accepted payload recordKind is not intakeAccepted")
    data = _map_lookup_map(v, "data")
    case_ref = str(_map_lookup_str(data, "caseRef"))
    outputs = _map_lookup_array(v, "outputs")
    output_case_ref = _first_array_text(outputs)
    if output_case_ref is None:
        raise VerifyError("intake accepted outputs array is missing or empty")
    if output_case_ref != case_ref:
        raise VerifyError("intake accepted outputs[0] does not match data.caseRef")
    return {
        "intake_id": str(_map_lookup_str(data, "intakeId")),
        "case_intent": str(_map_lookup_str(data, "caseIntent")),
        "case_disposition": str(_map_lookup_str(data, "caseDisposition")),
        "case_ref": case_ref,
        "definition_url": _map_lookup_optional_text(data, "definitionUrl"),
        "definition_version": _map_lookup_optional_text(data, "definitionVersion"),
    }


def _parse_case_created_record(payload_bytes: bytes) -> dict[str, Any]:
    v = _decode_value(payload_bytes)
    if not isinstance(v, dict):
        raise VerifyError("case created payload root is not a map")
    record_kind = str(_map_lookup_str(v, "recordKind"))
    if record_kind != "caseCreated":
        raise VerifyError("case created payload recordKind is not caseCreated")
    data = _map_lookup_map(v, "data")
    case_ref = str(_map_lookup_str(data, "caseRef"))
    outputs = _map_lookup_array(v, "outputs")
    output_case_ref = _first_array_text(outputs)
    if output_case_ref is None:
        raise VerifyError("case created outputs array is missing or empty")
    if output_case_ref != case_ref:
        raise VerifyError("case created outputs[0] does not match data.caseRef")
    return {
        "case_ref": case_ref,
        "intake_handoff_ref": str(_map_lookup_str(data, "intakeHandoffRef")),
        "formspec_response_ref": str(_map_lookup_str(data, "formspecResponseRef")),
        "validation_report_ref": str(_map_lookup_str(data, "validationReportRef")),
        "ledger_head_ref": str(_map_lookup_str(data, "ledgerHeadRef")),
        "initiation_mode": str(_map_lookup_str(data, "initiationMode")),
    }


def _parse_intake_manifest_entries(data: bytes) -> list[dict[str, Any]]:
    v = _decode_value(data)
    if not isinstance(v, list):
        raise VerifyError("intake handoff catalog root is not an array")
    out: list[dict[str, Any]] = []
    for entry in v:
        if not isinstance(entry, dict):
            raise VerifyError("intake handoff catalog entry is not a map")
        handoff_raw = entry.get("handoff")
        if handoff_raw is None:
            raise VerifyError("missing `handoff`")
        handoff = _parse_intake_handoff_details(handoff_raw)
        out.append(
            {
                "intake_event_hash": _map_lookup_fixed_bytes(entry, "intake_event_hash", 32),
                "case_created_event_hash": _map_lookup_optional_fixed_bytes(
                    entry, "case_created_event_hash", 32
                ),
                "handoff": handoff,
                "response_bytes": _map_lookup_bytes(entry, "response_bytes"),
            }
        )
    return out


def _intake_entry_matches_record(entry: dict[str, Any], record: dict[str, Any]) -> bool:
    handoff = entry["handoff"]
    if handoff["handoff_id"] != record["intake_id"]:
        return False
    mode = handoff["initiation_mode"]
    if mode == "workflowInitiated":
        return (
            handoff.get("case_ref") == record["case_ref"]
            and record["case_intent"] == "attachToExistingCase"
            and record["case_disposition"] == "attachToExistingCase"
        )
    if mode == "publicIntake":
        return (
            record["case_intent"] == "requestGovernedCaseCreation"
            and record["case_disposition"] == "createGovernedCase"
            and record.get("definition_url") == handoff["definition_url"]
            and record.get("definition_version") == handoff["definition_version"]
        )
    return False


def _case_created_record_matches_handoff(
    entry: dict[str, Any], intake_record: dict[str, Any], case_record: dict[str, Any]
) -> bool:
    handoff = entry["handoff"]
    return (
        case_record["case_ref"] == intake_record["case_ref"]
        and case_record["intake_handoff_ref"] == handoff["handoff_id"]
        and case_record["formspec_response_ref"] == handoff["response_ref"]
        and case_record["validation_report_ref"] == handoff["validation_report_ref"]
        and case_record["ledger_head_ref"] == handoff["ledger_head_ref"]
        and case_record["initiation_mode"] == handoff["initiation_mode"]
    )


def _response_hash_matches(value: str, response_bytes: bytes) -> tuple[bool, Optional[str]]:
    try:
        expected = _parse_sha256_prefix_text(value)
    except VerifyError as exc:
        return False, str(exc)
    actual = _sha256(response_bytes)
    return (actual == expected), None


def _parse_attachment_manifest_entries(data: bytes) -> list[dict[str, Any]]:
    v = _decode_value(data)
    if not isinstance(v, list):
        raise VerifyError("attachment manifest root is not an array")
    out: list[dict[str, Any]] = []
    for entry in v:
        if not isinstance(entry, dict):
            raise VerifyError("attachment manifest entry is not a map")
        out.append(
            {
                "binding_event_hash": _map_lookup_fixed_bytes(entry, "binding_event_hash", 32),
                "attachment_id": str(_map_lookup_str(entry, "attachment_id")),
                "slot_path": str(_map_lookup_str(entry, "slot_path")),
                "media_type": str(_map_lookup_str(entry, "media_type")),
                "byte_length": _map_lookup_u64(entry, "byte_length"),
                "attachment_sha256": _map_lookup_fixed_bytes(entry, "attachment_sha256", 32),
                "payload_content_hash": _map_lookup_fixed_bytes(entry, "payload_content_hash", 32),
                "filename": entry.get("filename"),
                "prior_binding_hash": _map_lookup_optional_fixed_bytes(
                    entry, "prior_binding_hash", 32
                ),
            }
        )
    return out


def _binding_lineage_graph_has_cycle(adj: dict[bytes, list[bytes]]) -> bool:
    class Color:
        WHITE = 0
        GRAY = 1
        BLACK = 2

    nodes: set[bytes] = set()
    for frm, tos in adj.items():
        nodes.add(frm)
        nodes.update(tos)
    color: dict[bytes, int] = {n: Color.WHITE for n in nodes}

    def dfs(node: bytes) -> bool:
        c = color.get(node, Color.WHITE)
        if c == Color.GRAY:
            return True
        if c == Color.BLACK:
            return False
        color[node] = Color.GRAY
        for nxt in adj.get(node, []):
            if dfs(nxt):
                return True
        color[node] = Color.BLACK
        return False

    for n in list(nodes):
        if color.get(n, Color.WHITE) == Color.WHITE and dfs(n):
            return True
    return False


def _attachment_manifest_topology_failures(
    entries: list[dict[str, Any]], hash_to_index: dict[bytes, int]
) -> list[VerificationFailure]:
    failures: list[VerificationFailure] = []
    seen: set[bytes] = set()
    for e in entries:
        bh = e["binding_event_hash"]
        if bh in seen:
            failures.append(
                VerificationFailure("attachment_manifest_duplicate_binding", _hex(bh))
            )
        seen.add(bh)
    adj: dict[bytes, list[bytes]] = defaultdict(list)
    for e in entries:
        prior = e.get("prior_binding_hash")
        if prior is None:
            continue
        bh = e["binding_event_hash"]
        if bh in hash_to_index and prior in hash_to_index:
            adj[bh].append(prior)
    if _binding_lineage_graph_has_cycle(dict(adj)):
        failures.append(
            VerificationFailure("attachment_binding_lineage_cycle", "061-attachments.cbor")
        )
    for e in entries:
        prior = e.get("prior_binding_hash")
        if prior is None:
            continue
        bh = e["binding_event_hash"]
        cur_i = hash_to_index.get(bh)
        pri_i = hash_to_index.get(prior)
        if cur_i is None or pri_i is None:
            failures.append(
                VerificationFailure("attachment_prior_binding_unresolved", _hex(bh))
            )
            continue
        if pri_i >= cur_i:
            failures.append(
                VerificationFailure(
                    "attachment_prior_binding_forward_reference", _hex(bh)
                )
            )
    return failures


def _attachment_entry_matches_binding(
    entry: dict[str, Any], binding: AttachmentBindingDetails
) -> bool:
    if entry["attachment_id"] != binding.attachment_id:
        return False
    if entry["slot_path"] != binding.slot_path:
        return False
    if entry["media_type"] != binding.media_type:
        return False
    if entry["byte_length"] != binding.byte_length:
        return False
    if entry["attachment_sha256"] != binding.attachment_sha256:
        return False
    if entry["payload_content_hash"] != binding.payload_content_hash:
        return False
    fn = entry.get("filename")
    if fn != binding.filename:
        return False
    if entry.get("prior_binding_hash") != binding.prior_binding_hash:
        return False
    return True


def _verify_attachment_manifest(
    archive: dict[str, bytes],
    events: list[ParsedSign1],
    manifest_digest: bytes,
    inline_attachments: bool,
    report: VerificationReport,
) -> None:
    man_bytes = archive.get("061-attachments.cbor")
    if man_bytes is None:
        report.event_failures.append(
            VerificationFailure("missing_attachment_manifest", "061-attachments.cbor")
        )
        return
    if _sha256(man_bytes) != manifest_digest:
        report.event_failures.append(
            VerificationFailure("attachment_manifest_digest_mismatch", "061-attachments.cbor")
        )
    try:
        entries = _parse_attachment_manifest_entries(man_bytes)
    except VerifyError as exc:
        report.event_failures.append(
            VerificationFailure("attachment_manifest_invalid", f"061-attachments.cbor/{exc}")
        )
        return
    details_list: list[EventDetails] = []
    for ev in events:
        try:
            details_list.append(_decode_event_details(ev))
        except VerifyError:
            continue
    hash_to_index: dict[bytes, int] = {}
    for index, ev in enumerate(events):
        try:
            d = _decode_event_details(ev)
            hash_to_index[d.canonical_event_hash] = index
        except VerifyError:
            continue
    for f in _attachment_manifest_topology_failures(entries, hash_to_index):
        report.event_failures.append(f)
    for e in entries:
        bh = e["binding_event_hash"]
        matches = [d for d in details_list if d.canonical_event_hash == bh]
        if len(matches) != 1:
            report.event_failures.append(
                VerificationFailure("attachment_binding_event_unresolved", _hex(bh))
            )
            continue
        det = matches[0]
        binding = det.attachment_binding
        if binding is None:
            report.event_failures.append(
                VerificationFailure("attachment_binding_missing", _hex(bh))
            )
            continue
        if not _attachment_entry_matches_binding(e, binding):
            report.event_failures.append(
                VerificationFailure("attachment_binding_mismatch", _hex(bh))
            )
            continue
        if e["payload_content_hash"] != det.content_hash or binding.payload_content_hash != det.content_hash:
            report.event_failures.append(
                VerificationFailure("attachment_payload_hash_mismatch", _hex(bh))
            )
            continue
        if inline_attachments:
            member = f"060-payloads/{_hex(e['payload_content_hash'])}.bin"
            if member not in archive:
                report.event_failures.append(
                    VerificationFailure("missing_attachment_body", member)
                )


def _readable_payload_bytes(
    details: EventDetails, payload_blobs: dict[bytes, bytes]
) -> Optional[bytes]:
    if details.payload_ref_inline is not None:
        return details.payload_ref_inline
    if details.payload_ref_external:
        return payload_blobs.get(details.content_hash)
    return None


def _parse_signature_catalog_entries(data: bytes) -> list[dict[str, Any]]:
    v = _decode_value(data)
    if not isinstance(v, list):
        raise VerifyError("signature catalog root is not an array")
    rows: list[dict[str, Any]] = []
    for entry in v:
        if not isinstance(entry, dict):
            raise VerifyError("signature catalog entry is not a map")
        pr = entry.get("profile_ref")
        pk = entry.get("profile_key")
        ib = entry.get("identity_binding")
        cr = entry.get("consent_reference")
        if not isinstance(ib, dict) or not isinstance(cr, dict):
            raise VerifyError("identity_binding or consent_reference is not a map")
        rows.append(
            {
                "canonical_event_hash": _map_lookup_fixed_bytes(
                    entry, "canonical_event_hash", 32
                ),
                "signer_id": str(_map_lookup_str(entry, "signer_id")),
                "role_id": str(_map_lookup_str(entry, "role_id")),
                "role": str(_map_lookup_str(entry, "role")),
                "document_id": str(_map_lookup_str(entry, "document_id")),
                "document_hash": str(_map_lookup_str(entry, "document_hash")),
                "document_hash_algorithm": str(_map_lookup_str(entry, "document_hash_algorithm")),
                "signed_at": str(_map_lookup_str(entry, "signed_at")),
                "identity_binding": ib,
                "consent_reference": cr,
                "signature_provider": str(_map_lookup_str(entry, "signature_provider")),
                "ceremony_id": str(_map_lookup_str(entry, "ceremony_id")),
                "profile_ref": str(pr) if isinstance(pr, str) else None,
                "profile_key": str(pk) if isinstance(pk, str) else None,
                "formspec_response_ref": str(_map_lookup_str(entry, "formspec_response_ref")),
            }
        )
    return rows


def _parse_signature_affirmation_record(payload_bytes: bytes) -> dict[str, Any]:
    v = _decode_value(payload_bytes)
    if not isinstance(v, dict):
        raise VerifyError("signature affirmation payload root is not a map")
    rk = str(_map_lookup_str(v, "recordKind"))
    if rk != "signatureAffirmation":
        raise VerifyError("recordKind is not signatureAffirmation")
    data = _map_lookup_map(v, "data")
    pr = data.get("profileRef")
    pk = data.get("profileKey")
    ib = _map_lookup_map(data, "identityBinding")
    cr = _map_lookup_map(data, "consentReference")
    return {
        "signer_id": str(_map_lookup_str(data, "signerId")),
        "role_id": str(_map_lookup_str(data, "roleId")),
        "role": str(_map_lookup_str(data, "role")),
        "document_id": str(_map_lookup_str(data, "documentId")),
        "document_hash": str(_map_lookup_str(data, "documentHash")),
        "document_hash_algorithm": str(_map_lookup_str(data, "documentHashAlgorithm")),
        "signed_at": str(_map_lookup_str(data, "signedAt")),
        "identity_binding": ib,
        "consent_reference": cr,
        "signature_provider": str(_map_lookup_str(data, "signatureProvider")),
        "ceremony_id": str(_map_lookup_str(data, "ceremonyId")),
        "profile_ref": str(pr) if isinstance(pr, str) else None,
        "profile_key": str(pk) if isinstance(pk, str) else None,
        "formspec_response_ref": str(_map_lookup_str(data, "formspecResponseRef")),
    }


def _signature_entry_matches_record(entry: dict[str, Any], record: dict[str, Any]) -> bool:
    if entry["signer_id"] != record["signer_id"]:
        return False
    if entry["role_id"] != record["role_id"]:
        return False
    if entry["role"] != record["role"]:
        return False
    if entry["document_id"] != record["document_id"]:
        return False
    if entry["document_hash"] != record["document_hash"]:
        return False
    if entry["document_hash_algorithm"] != record["document_hash_algorithm"]:
        return False
    if entry["signed_at"] != record["signed_at"]:
        return False
    if not _cbor_nested_semantic_eq(entry["identity_binding"], record["identity_binding"]):
        return False
    if not _cbor_nested_semantic_eq(entry["consent_reference"], record["consent_reference"]):
        return False
    if entry["signature_provider"] != record["signature_provider"]:
        return False
    if entry["ceremony_id"] != record["ceremony_id"]:
        return False
    if entry["profile_ref"] != record["profile_ref"]:
        return False
    if entry["profile_key"] != record["profile_key"]:
        return False
    if entry["formspec_response_ref"] != record["formspec_response_ref"]:
        return False
    return True


def _index_events_by_canonical_hash(
    events: list[ParsedSign1],
) -> tuple[dict[bytes, EventDetails], list[VerificationFailure]]:
    """Single pass for signature + intake catalog verifiers; duplicate failures once."""
    event_by_hash: dict[bytes, EventDetails] = {}
    duplicate_failures: list[VerificationFailure] = []
    for ev in events:
        try:
            d = _decode_event_details(ev)
        except VerifyError:
            continue
        if d.canonical_event_hash in event_by_hash:
            duplicate_failures.append(
                VerificationFailure(
                    "export_events_duplicate_canonical_hash",
                    _hex(d.canonical_event_hash),
                )
            )
            continue
        event_by_hash[d.canonical_event_hash] = d
    return event_by_hash, duplicate_failures


def _verify_signature_catalog(
    archive: dict[str, bytes],
    payload_blobs: dict[bytes, bytes],
    catalog_digest: bytes,
    report: VerificationReport,
    event_by_hash: dict[bytes, EventDetails],
) -> None:
    cat_bytes = archive.get("062-signature-affirmations.cbor")
    if cat_bytes is None:
        report.event_failures.append(
            VerificationFailure("missing_signature_catalog", "062-signature-affirmations.cbor")
        )
        return
    if _sha256(cat_bytes) != catalog_digest:
        report.event_failures.append(
            VerificationFailure(
                "signature_catalog_digest_mismatch", "062-signature-affirmations.cbor"
            )
        )
    try:
        entries = _parse_signature_catalog_entries(cat_bytes)
    except VerifyError as exc:
        report.event_failures.append(
            VerificationFailure(
                "signature_catalog_invalid", f"062-signature-affirmations.cbor/{exc}"
            )
        )
        return
    seen_row: set[bytes] = set()
    for row in entries:
        h = row["canonical_event_hash"]
        if h in seen_row:
            report.event_failures.append(
                VerificationFailure("signature_catalog_duplicate_event", _hex(h))
            )
        seen_row.add(h)
    for row in entries:
        h = row["canonical_event_hash"]
        det = event_by_hash.get(h)
        if det is None:
            report.event_failures.append(
                VerificationFailure("signature_catalog_event_unresolved", _hex(h))
            )
            continue
        if det.event_type != WOS_SIGNATURE_AFFIRMATION_EVENT_TYPE:
            report.event_failures.append(
                VerificationFailure("signature_catalog_event_type_mismatch", _hex(h))
            )
            continue
        payload = _readable_payload_bytes(det, payload_blobs)
        if payload is None:
            report.event_failures.append(
                VerificationFailure("signature_affirmation_payload_unreadable", _hex(h))
            )
            continue
        try:
            record = _parse_signature_affirmation_record(payload)
        except VerifyError as exc:
            report.event_failures.append(
                VerificationFailure(
                    "signature_affirmation_payload_invalid", f"{_hex(h)}/{exc}"
                )
            )
            continue
        if not _signature_entry_matches_record(row, record):
            report.event_failures.append(
                VerificationFailure("signature_catalog_mismatch", _hex(h))
            )


def _verify_intake_catalog(
    archive: dict[str, bytes],
    payload_blobs: dict[bytes, bytes],
    catalog_digest: bytes,
    report: VerificationReport,
    event_by_hash: dict[bytes, EventDetails],
) -> None:
    cat_bytes = archive.get("063-intake-handoffs.cbor")
    if cat_bytes is None:
        report.event_failures.append(
            VerificationFailure("missing_intake_handoff_catalog", "063-intake-handoffs.cbor")
        )
        return
    if _sha256(cat_bytes) != catalog_digest:
        report.event_failures.append(
            VerificationFailure(
                "intake_handoff_catalog_digest_mismatch", "063-intake-handoffs.cbor"
            )
        )
    try:
        entries = _parse_intake_manifest_entries(cat_bytes)
    except VerifyError as exc:
        report.event_failures.append(
            VerificationFailure("intake_handoff_catalog_invalid", f"063-intake-handoffs.cbor/{exc}")
        )
        return

    seen_row: set[bytes] = set()
    for entry in entries:
        h = entry["intake_event_hash"]
        if h in seen_row:
            report.event_failures.append(
                VerificationFailure("intake_handoff_catalog_duplicate_event", _hex(h))
            )
        seen_row.add(h)

    for entry in entries:
        intake_h = entry["intake_event_hash"]
        det = event_by_hash.get(intake_h)
        if det is None:
            report.event_failures.append(
                VerificationFailure("intake_event_unresolved", _hex(intake_h))
            )
            continue
        if det.event_type != WOS_INTAKE_ACCEPTED_EVENT_TYPE:
            report.event_failures.append(
                VerificationFailure("intake_event_type_mismatch", _hex(intake_h))
            )
            continue
        payload = _readable_payload_bytes(det, payload_blobs)
        if payload is None:
            report.event_failures.append(
                VerificationFailure("intake_payload_unreadable", _hex(intake_h))
            )
            continue
        try:
            intake_record = _parse_intake_accepted_record(payload)
        except VerifyError as exc:
            report.event_failures.append(
                VerificationFailure("intake_payload_invalid", f"{_hex(intake_h)}/{exc}")
            )
            continue
        if not _intake_entry_matches_record(entry, intake_record):
            report.event_failures.append(
                VerificationFailure("intake_handoff_mismatch", _hex(intake_h))
            )
        ok, err_detail = _response_hash_matches(
            entry["handoff"]["response_hash"], entry["response_bytes"]
        )
        if err_detail is not None:
            report.event_failures.append(
                VerificationFailure(
                    "intake_handoff_catalog_invalid",
                    f"{_hex(intake_h)}/{err_detail}",
                )
            )
        elif not ok:
            report.event_failures.append(
                VerificationFailure("intake_response_hash_mismatch", _hex(intake_h))
            )

        handoff = entry["handoff"]
        mode = handoff["initiation_mode"]
        case_created_hash = entry["case_created_event_hash"]
        if mode == "workflowInitiated":
            if case_created_hash is not None:
                report.event_failures.append(
                    VerificationFailure("case_created_handoff_mismatch", _hex(intake_h))
                )
            continue
        if mode == "publicIntake":
            if case_created_hash is None:
                report.event_failures.append(
                    VerificationFailure("case_created_handoff_mismatch", _hex(intake_h))
                )
                continue
            case_details = event_by_hash.get(case_created_hash)
            if case_details is None:
                report.event_failures.append(
                    VerificationFailure(
                        "case_created_event_unresolved", _hex(case_created_hash)
                    )
                )
                continue
            if case_details.event_type != WOS_CASE_CREATED_EVENT_TYPE:
                report.event_failures.append(
                    VerificationFailure(
                        "case_created_event_type_mismatch", _hex(case_created_hash)
                    )
                )
                continue
            case_payload = _readable_payload_bytes(case_details, payload_blobs)
            if case_payload is None:
                report.event_failures.append(
                    VerificationFailure(
                        "case_created_payload_unreadable", _hex(case_created_hash)
                    )
                )
                continue
            try:
                case_record = _parse_case_created_record(case_payload)
            except VerifyError as exc:
                report.event_failures.append(
                    VerificationFailure(
                        "case_created_payload_invalid", f"{_hex(case_created_hash)}/{exc}"
                    )
                )
                continue
            if not _case_created_record_matches_handoff(entry, intake_record, case_record):
                report.event_failures.append(
                    VerificationFailure(
                        "case_created_handoff_mismatch", _hex(case_created_hash)
                    )
                )
            continue


def parse_export_zip(data: bytes) -> dict[str, bytes]:
    try:
        zf = zipfile.ZipFile(io.BytesIO(data), "r")
    except Exception as exc:  # noqa: BLE001
        raise VerifyError(f"failed to parse ZIP: {exc}") from exc
    members: dict[str, bytes] = {}
    try:
        for info in zf.infolist():
            name = info.filename
            if "/" not in name:
                raise VerifyError("ZIP member does not live under one export root")
            _, relative = name.split("/", 1)
            with zf.open(info) as fh:
                members[relative] = fh.read()
    finally:
        zf.close()
    return members


def verify_export_zip(export_zip: bytes) -> VerificationReport:
    try:
        archive = parse_export_zip(export_zip)
    except VerifyError as exc:
        return VerificationReport.fatal("export_zip_invalid", f"failed to open export ZIP: {exc}")

    reg_bytes = archive.get("030-signing-key-registry.cbor")
    if reg_bytes is None:
        return VerificationReport.fatal(
            "missing_signing_key_registry", "export is missing 030-signing-key-registry.cbor"
        )
    try:
        registry, non_signing_registry = _parse_key_registry(reg_bytes)
    except VerifyError as exc:
        return VerificationReport.fatal(
            "signing_key_registry_invalid", f"failed to decode signing-key registry: {exc}"
        )

    manifest_bytes = archive.get("000-manifest.cbor")
    if manifest_bytes is None:
        return VerificationReport.fatal("missing_manifest", "export is missing 000-manifest.cbor")
    try:
        manifest = _parse_sign1_bytes(manifest_bytes)
    except VerifyError as exc:
        return VerificationReport.fatal(
            "manifest_structure_invalid", f"manifest is not a valid COSE_Sign1 envelope: {exc}"
        )

    if manifest.alg != ALG_EDDSA or manifest.suite_id != SUITE_ID_PHASE_1:
        return VerificationReport.fatal(
            "unsupported_suite", "manifest protected header does not match the Trellis Phase-1 suite"
        )

    manifest_entry = registry.get(manifest.kid)
    if manifest_entry is None:
        return VerificationReport.fatal(
            "unresolvable_manifest_kid",
            "manifest kid is not resolvable via the embedded signing-key registry",
        )
    if not _verify_signature(manifest, manifest_entry.public_key):
        return VerificationReport.fatal("manifest_signature_invalid", "manifest COSE signature is invalid")

    if manifest.payload is None:
        return VerificationReport.fatal(
            "manifest_payload_missing", "manifest payload is detached, which is out of scope for Phase 1"
        )
    try:
        manifest_payload = _decode_value(manifest.payload)
    except VerifyError as exc:
        return VerificationReport.fatal(
            "manifest_payload_invalid", f"failed to decode manifest payload: {exc}"
        )
    if not isinstance(manifest_payload, dict):
        return VerificationReport.fatal("manifest_payload_invalid", "manifest payload root is not a map")
    manifest_map = manifest_payload

    required_digests = [
        ("010-events.cbor", "events_digest"),
        ("020-inclusion-proofs.cbor", "inclusion_proofs_digest"),
        ("025-consistency-proofs.cbor", "consistency_proofs_digest"),
        ("030-signing-key-registry.cbor", "signing_key_registry_digest"),
        ("040-checkpoints.cbor", "checkpoints_digest"),
    ]
    for member_name, field_name in required_digests:
        try:
            expected = _map_lookup_fixed_bytes(manifest_map, field_name, 32)
        except VerifyError as exc:
            return VerificationReport.fatal(
                "manifest_payload_invalid", f"manifest is missing {field_name}: {exc}"
            )
        actual_bytes = archive.get(member_name)
        if actual_bytes is None:
            return VerificationReport.fatal(
                "archive_integrity_failure", f"export is missing required member {member_name}"
            )
        if expected != _sha256(actual_bytes):
            return VerificationReport.fatal(
                "archive_integrity_failure", f"manifest digest mismatch for {member_name}"
            )

    try:
        registry_bindings = _map_lookup_array(manifest_map, "registry_bindings")
    except VerifyError as exc:
        return VerificationReport.fatal(
            "manifest_payload_invalid", f"manifest registry_bindings are invalid: {exc}"
        )
    parsed_bindings: list[RegistryBindingInfo] = []
    for binding in registry_bindings:
        if not isinstance(binding, dict):
            return VerificationReport.fatal(
                "manifest_payload_invalid", "registry binding is not a map"
            )
        try:
            digest = _map_lookup_fixed_bytes(binding, "registry_digest", 32)
            member_name = f"050-registries/{_hex(digest)}.cbor"
            actual = archive.get(member_name)
            if actual is None:
                return VerificationReport.fatal(
                    "archive_integrity_failure",
                    f"export is missing bound registry member {member_name}",
                )
            if _sha256(actual) != digest:
                return VerificationReport.fatal(
                    "archive_integrity_failure",
                    f"bound registry digest mismatch for {member_name}",
                )
            bound_at_sequence = _map_lookup_u64(binding, "bound_at_sequence")
        except VerifyError as exc:
            return VerificationReport.fatal(
                "manifest_payload_invalid", f"registry binding digest is invalid: {exc}"
            )
        parsed_bindings.append(RegistryBindingInfo(digest_hex=_hex(digest), bound_at_sequence=bound_at_sequence))
    parsed_bindings.sort(key=lambda b: b.bound_at_sequence)

    parsed_registries: dict[str, BoundRegistry] = {}
    for binding in parsed_bindings:
        member_name = f"050-registries/{binding.digest_hex}.cbor"
        registry_member_bytes = archive[member_name]
        try:
            parsed_registries[binding.digest_hex] = _parse_bound_registry(registry_member_bytes)
        except VerifyError as exc:
            return VerificationReport.fatal(
                "bound_registry_invalid", f"failed to decode {member_name}: {exc}"
            )

    try:
        scope = _map_lookup_bytes(manifest_map, "scope")
    except VerifyError as exc:
        return VerificationReport.fatal("manifest_payload_invalid", f"manifest scope is invalid: {exc}")

    events_bytes = archive["010-events.cbor"]
    try:
        events = _parse_sign1_array(events_bytes)
    except VerifyError as exc:
        return VerificationReport.fatal("events_invalid", f"failed to decode 010-events.cbor: {exc}")

    payload_blobs: dict[bytes, bytes] = {}
    for name, blob in archive.items():
        if not name.startswith("060-payloads/") or not name.endswith(".bin"):
            continue
        digest_hex = name[len("060-payloads/") : -len(".bin")]
        try:
            d = _hex_decode(digest_hex)
        except VerifyError:
            continue
        if len(d) == 32:
            payload_blobs[d] = blob

    report = _verify_event_set(
        events,
        registry,
        None,
        None,
        False,
        scope,
        payload_blobs,
        non_signing_registry=non_signing_registry,
    )
    try:
        attachment_export = _parse_attachment_export_extension(manifest_map)
    except VerifyError as exc:
        return VerificationReport.fatal(
            "manifest_payload_invalid", f"attachment export extension is invalid: {exc}"
        )
    if attachment_export is not None:
        att_digest, att_inline = attachment_export
        _verify_attachment_manifest(archive, events, att_digest, att_inline, report)
    try:
        signature_catalog_digest = _parse_signature_export_extension(manifest_map)
    except VerifyError as exc:
        return VerificationReport.fatal(
            "manifest_payload_invalid", f"signature export extension is invalid: {exc}"
        )
    try:
        intake_catalog_digest = _parse_intake_export_extension(manifest_map)
    except VerifyError as exc:
        return VerificationReport.fatal(
            "manifest_payload_invalid", f"intake export extension is invalid: {exc}"
        )
    shared_event_by_hash: dict[bytes, EventDetails] = {}
    if signature_catalog_digest is not None or intake_catalog_digest is not None:
        shared_event_by_hash, dup_failures = _index_events_by_canonical_hash(events)
        report.event_failures.extend(dup_failures)
    if signature_catalog_digest is not None:
        _verify_signature_catalog(
            archive, payload_blobs, signature_catalog_digest, report, shared_event_by_hash
        )
    if intake_catalog_digest is not None:
        _verify_intake_catalog(
            archive, payload_blobs, intake_catalog_digest, report, shared_event_by_hash
        )

    for failure in report.event_failures:
        if failure.kind == "scope_mismatch":
            failure.location = f"manifest-scope/{failure.location}"

    for event in events:
        try:
            details = _decode_event_details(event)
        except VerifyError:
            continue
        eligible = [b for b in parsed_bindings if b.bound_at_sequence <= details.sequence]
        if not eligible:
            report.event_failures.append(
                VerificationFailure("registry_digest_mismatch", _hex(details.canonical_event_hash))
            )
            continue
        binding = max(eligible, key=lambda b: b.bound_at_sequence)
        bound_registry = parsed_registries.get(binding.digest_hex)
        if bound_registry is None:
            report.event_failures.append(
                VerificationFailure("registry_digest_mismatch", _hex(details.canonical_event_hash))
            )
            continue
        if (details.event_type not in bound_registry.event_types) or (
            details.classification not in bound_registry.classifications
        ):
            report.event_failures.append(
                VerificationFailure("registry_digest_mismatch", _hex(details.canonical_event_hash))
            )

    canonical_hashes = []
    for event in events:
        try:
            _, ch = _event_identity(event)
            canonical_hashes.append(ch)
        except Exception:  # noqa: BLE001
            continue
    leaf_hashes = [_merkle_leaf_hash(h) for h in canonical_hashes]

    checkpoints_bytes = archive["040-checkpoints.cbor"]
    try:
        checkpoints = _parse_sign1_array(checkpoints_bytes)
    except VerifyError as exc:
        return VerificationReport.fatal("checkpoints_invalid", f"failed to decode 040-checkpoints.cbor: {exc}")

    prior_checkpoint_digest: Optional[bytes] = None
    head_checkpoint_root: Optional[bytes] = None
    for checkpoint in checkpoints:
        ck_entry = registry.get(checkpoint.kid)
        if ck_entry is None:
            return VerificationReport.fatal(
                "unresolvable_manifest_kid",
                "checkpoint kid is not resolvable via the embedded signing-key registry",
            )
        if not _verify_signature(checkpoint, ck_entry.public_key):
            return VerificationReport.fatal(
                "checkpoint_signature_invalid", "checkpoint COSE signature is invalid"
            )
        payload_bytes_chk = checkpoint.payload
        if payload_bytes_chk is None:
            return VerificationReport.fatal("checkpoint_payload_invalid", "detached checkpoint")
        try:
            payload = _decode_value(payload_bytes_chk)
        except VerifyError as exc:
            return VerificationReport.fatal(
                "checkpoint_payload_invalid", f"failed to decode checkpoint payload: {exc}"
            )
        if not isinstance(payload, dict):
            return VerificationReport.fatal(
                "checkpoint_payload_invalid", "checkpoint payload root is not a map"
            )
        try:
            checkpoint_scope = _map_lookup_bytes(payload, "scope")
        except VerifyError as exc:
            return VerificationReport.fatal(
                "checkpoint_payload_invalid", f"checkpoint scope is invalid: {exc}"
            )
        if checkpoint_scope != scope:
            report.checkpoint_failures.append(
                VerificationFailure("scope_mismatch", "checkpoint/scope")
            )
            continue
        try:
            tree_size = _map_lookup_u64(payload, "tree_size")
        except VerifyError as exc:
            return VerificationReport.fatal(
                "checkpoint_payload_invalid", f"checkpoint tree_size is invalid: {exc}"
            )
        if tree_size == 0 or tree_size > len(leaf_hashes):
            report.checkpoint_failures.append(
                VerificationFailure("tree_size_invalid", f"checkpoint/tree_size/{tree_size}")
            )
            continue
        try:
            actual_root = _map_lookup_fixed_bytes(payload, "tree_head_hash", 32)
        except VerifyError as exc:
            return VerificationReport.fatal(
                "checkpoint_payload_invalid", f"checkpoint tree_head_hash is invalid: {exc}"
            )
        expected_root = _merkle_root(leaf_hashes[:tree_size])
        if expected_root != actual_root:
            report.checkpoint_failures.append(
                VerificationFailure("checkpoint_root_mismatch", f"checkpoint/tree_size/{tree_size}")
            )

        digest = _checkpoint_digest(scope, payload_bytes_chk)
        if prior_checkpoint_digest is not None:
            try:
                actual_prev = _map_lookup_fixed_bytes(payload, "prev_checkpoint_hash", 32)
            except VerifyError as exc:
                return VerificationReport.fatal(
                    "checkpoint_payload_invalid",
                    f"checkpoint prev_checkpoint_hash is invalid: {exc}",
                )
            if prior_checkpoint_digest != actual_prev:
                report.checkpoint_failures.append(
                    VerificationFailure(
                        "prev_checkpoint_hash_mismatch", f"checkpoint/tree_size/{tree_size}"
                    )
                )
        prior_checkpoint_digest = digest
        head_checkpoint_root = actual_root

    try:
        head_checkpoint_digest = _map_lookup_fixed_bytes(manifest_map, "head_checkpoint_digest", 32)
    except VerifyError as exc:
        return VerificationReport.fatal(
            "manifest_payload_invalid", f"manifest head_checkpoint_digest is invalid: {exc}"
        )
    if prior_checkpoint_digest != head_checkpoint_digest:
        report.checkpoint_failures.append(
            VerificationFailure("head_checkpoint_digest_mismatch", "manifest/head_checkpoint_digest")
        )

    inclusion_map = _decode_value(archive["020-inclusion-proofs.cbor"])
    if isinstance(inclusion_map, dict):
        expected_root = head_checkpoint_root if head_checkpoint_root is not None else bytes(32)
        for _k, proof_value in inclusion_map.items():
            if not isinstance(proof_value, dict):
                report.proof_failures.append(VerificationFailure("inclusion_proof_invalid", "proof/map"))
                continue
            pm = proof_value
            try:
                tree_size_p = _map_lookup_u64(pm, "tree_size")
                leaf_index = _map_lookup_u64(pm, "leaf_index")
                leaf_hash = _map_lookup_fixed_bytes(pm, "leaf_hash", 32)
                audit_path_values = _map_lookup_array(pm, "audit_path")
                audit_path = _digest_path_from_values(audit_path_values)
            except (VerifyError, ValueError):
                report.proof_failures.append(VerificationFailure("inclusion_proof_invalid", "proof/map"))
                continue
            if tree_size_p != len(leaf_hashes):
                report.proof_failures.append(
                    VerificationFailure("inclusion_proof_invalid", f"proof/tree_size/{tree_size_p}")
                )
                continue
            if leaf_index >= len(leaf_hashes):
                report.proof_failures.append(
                    VerificationFailure("inclusion_proof_invalid", f"proof/index/{leaf_index}")
                )
                continue
            matches_leaf = leaf_hash == leaf_hashes[leaf_index]
            rr = _root_from_inclusion_proof(leaf_index, tree_size_p, leaf_hash, audit_path)
            matches_root = rr is not None and rr == expected_root
            if not matches_leaf or not matches_root:
                report.proof_failures.append(
                    VerificationFailure("inclusion_proof_mismatch", f"proof/index/{leaf_index}")
                )

    consistency_value = _decode_value(archive["025-consistency-proofs.cbor"])
    if isinstance(consistency_value, list):
        for record in consistency_value:
            if not isinstance(record, dict):
                report.proof_failures.append(
                    VerificationFailure("consistency_proof_invalid", "consistency/map")
                )
                continue
            rm = record
            from_tree_size = int(rm.get("from_tree_size", 0) or 0)
            to_tree_size = int(rm.get("to_tree_size", 0) or 0)
            location = f"consistency/{from_tree_size}-{to_tree_size}"
            try:
                proof_path_values = _map_lookup_array(rm, "proof_path")
                proof_path = _digest_path_from_values(proof_path_values)
            except (VerifyError, ValueError):
                report.proof_failures.append(
                    VerificationFailure(
                        "consistency_proof_invalid", f"{location}/proof_path"
                    )
                )
                continue
            if from_tree_size == 0:
                report.proof_failures.append(
                    VerificationFailure("consistency_proof_invalid", f"{location}/from_zero")
                )
                continue
            if from_tree_size >= to_tree_size or to_tree_size > len(leaf_hashes):
                report.proof_failures.append(
                    VerificationFailure("consistency_proof_invalid", location)
                )
                continue
            root_old = _merkle_root(leaf_hashes[:from_tree_size])
            root_new = _merkle_root(leaf_hashes[:to_tree_size])
            cr = _root_from_consistency_proof(from_tree_size, to_tree_size, root_old, proof_path)
            if cr != root_new:
                report.proof_failures.append(
                    VerificationFailure("consistency_proof_mismatch", location)
                )

    report.structure_verified = True
    report.integrity_verified = (
        not report.event_failures
        and not report.checkpoint_failures
        and not report.proof_failures
        and all(
            o.continuity_verified and o.declaration_resolved and o.attestations_verified
            for o in report.posture_transitions
        )
    )
    report.readability_verified = True
    return report


def verify_tampered_ledger(
    signing_key_registry: bytes,
    ledger: bytes,
    initial_posture_declaration: Optional[bytes] = None,
    posture_declaration: Optional[bytes] = None,
) -> VerificationReport:
    try:
        registry, non_signing_registry = _parse_key_registry(signing_key_registry)
    except VerifyError as exc:
        return VerificationReport.fatal(
            "signing_key_registry_invalid", f"failed to decode signing-key registry: {exc}"
        )
    try:
        events = _parse_sign1_array(ledger)
    except Exception:  # noqa: BLE001
        events = []
    if not events:
        return VerificationReport.fatal(
            "malformed_cose", "ledger is not a non-empty dCBOR array of COSE_Sign1 events"
        )
    return _verify_event_set(
        events,
        registry,
        initial_posture_declaration,
        posture_declaration,
        True,
        None,
        None,
        non_signing_registry=non_signing_registry,
    )


def verify_single_event(public_key_bytes: bytes, signed_event: bytes) -> VerificationReport:
    try:
        parsed = _parse_sign1_bytes(signed_event)
    except VerifyError as exc:
        return VerificationReport.fatal("malformed_cose", str(exc))
    registry = {parsed.kid: SigningKeyEntry(public_key=public_key_bytes, status=0, valid_to=None)}
    return _verify_event_set([parsed], registry, None, None, False, None, None)
