# trellis-interop-c2pa

C2PA interop adapter for Trellis (ADR 0008, Wave 25).

This crate emits and parses the **Trellis assertion** that ADR 0008's
`c2pa-manifest` kind binds to a Certificate of Completion's
presentation artifact (PDF or HTML, per ADR 0007). The assertion is
a five-field dCBOR map carrying `certificate_id`, `canonical_event_hash`,
`presentation_artifact.content_hash`, `kid`, and `cose_sign1_ref` —
the cross-binding between the Trellis canonical chain and the C2PA
manifest store.

## Two verification paths

ADR 0008 §"`c2pa-manifest`" defines two **independent and additive**
verification paths. Both can run; neither replaces canonical bytes
(ISC-01).

### Path-(b) — canonical bytes only (Phase-1 core)

The Phase-1 core verifier (`trellis-verify::verify_export_zip`) walks
`manifest.interop_sidecars`, recomputes `content_digest` against the
on-disk file under `trellis-content-v1`, and validates `kind` /
`derivation_version` / `path` against the closed registry. It does
**NOT** decode the assertion or resolve `source_ref`. Path-(b)
detects sidecar-bytes tampering — a stranger walking only the export
ZIP catches mutation by digest cross-check.

This path runs offline, uses no ecosystem libraries, and depends only
on `trellis-verify`. It does not import this crate.

### Path-(a) — C2PA-tooling consumer

A consumer that wants to verify the Trellis ↔ C2PA cross-binding from
a real C2PA manifest does the following:

1. Open the PDF (or JPEG / etc.) and extract the C2PA manifest store
   using their preferred C2PA SDK (`c2pa-rs`, `c2patool`, etc.).
2. Validate the manifest under C2PA conventions (signature
   verification, claim chain integrity, manifest-store layout).
3. Read the assertion bytes for label
   `org.formspec.trellis.certificate-of-completion.v1`.
4. Pass those bytes to [`extract_trellis_assertion`].
5. Build a [`CanonicalChainContext`] from the verified Trellis chain
   (the certificate event's canonical hash, the presentation
   artifact's content hash, the signer kid, the COSE_Sign1 ref).
6. Call [`TrellisAssertion::verify_against_canonical_chain`].

The path-(a) consumer is **not** in this crate. This crate provides
only the assertion emitter, parser, and cross-check; the consumer is
responsible for picking and managing their own C2PA SDK plus the
PDF/JPEG embedding pipeline. Keeping the SDK boundary outside this
crate preserves Core §16 verification independence (ISC-05) — neither
`trellis-verify` nor `trellis-types` pulls a 328-crate ecosystem
dep tree, and a deployment that doesn't ship C2PA at all has zero
adapter footprint.

## Why hand-rolled CBOR

The `c2pa-rs` crate is large (≈ 328 transitive dependencies as of
v0.80, including image parsers, ASN.1 decoders, RDF infrastructure,
and network-capable certificate-validation paths). That weight is
appropriate for the C2PA-tooling consumer — which already needs
PDF/JPEG embedding and C2PA-conventional signing — but inappropriate
as a workspace base dep.

The Trellis *assertion* is a 5-field dCBOR map; the assertion bytes
are byte-exact under any conforming dCBOR encoder. This crate emits
and parses the assertion directly via `ciborium`, with deterministic
map-key ordering per Core §5.1. Consumers feed the assertion bytes
into their preferred C2PA SDK (or read them out after the SDK extracts
the assertion from the manifest store).

If a future design needs end-to-end PDF-embedding round-trip vectors
in the workspace conformance corpus, the right design move is to add
a `trellis-interop-c2pa-tooling` adapter crate that depends on `c2pa`
and exposes the embedding/extraction pipeline. The current crate
intentionally stops at the assertion boundary so the SDK weight is
opt-in per deployment.

## Vendor-prefix label

Per ADR 0008 Open Q3 resolution (Wave 25): the assertion ships under
the vendor-prefix label
`org.formspec.trellis.certificate-of-completion.v1`. This sidesteps
C2PA coalition gating without forfeiting interop — consumers parse
the assertion identically regardless of label namespace. A formal
short-form C2PA-registry registration is a follow-on ADR; that ADR
will rename the label and bump the kind's `derivation_version` per
ISC-06.

## API surface

```rust,ignore
pub fn emit_c2pa_manifest_for_certificate(
    certificate_id: &str,
    canonical_event_hash: &[u8; 32],
    presentation_artifact_content_hash: &[u8; 32],
    kid: &[u8; 16],
    cose_sign1_ref: &[u8; 32],
) -> Result<Vec<u8>, AdapterError>;

pub fn extract_trellis_assertion(
    c2pa_bytes: &[u8],
) -> Result<TrellisAssertion, AdapterError>;

impl TrellisAssertion {
    pub fn verify_against_canonical_chain(
        &self,
        context: &CanonicalChainContext,
    ) -> Result<(), AdapterError>;
}

pub fn compute_cose_sign1_ref(cose_sign1_bytes: &[u8]) -> [u8; 32];
```

## ISC-05 hygiene

`deny.toml` (workspace root) binds the `c2pa` crate to wrappers
`["trellis-interop-c2pa"]` only. Wave 25 ships this crate WITHOUT a
direct `c2pa` dep — the hand-rolled CBOR path makes the dep
unnecessary for the assertion shape. If a future
`trellis-interop-c2pa-tooling` adapter activates, that crate would
add the `c2pa` dep and the cargo-deny rule already permits it.

## See also

- [ADR 0008 — Interop Sidecar Discipline](../../thoughts/adr/0008-interop-sidecar-discipline.md)
- [ADR 0007 — Certificate of Completion](../../thoughts/adr/0007-certificate-of-completion-composition.md)
- [Core §18.3a + §19.1](../../specs/trellis-core.md) — manifest reservation + tamper enum
