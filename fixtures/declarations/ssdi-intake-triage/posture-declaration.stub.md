# Posture Declaration — SSDI Intake Triage (stub)

This is a stub describing the shape of the Posture Declaration that the
companion `declaration.md` references via
`posture_declaration_ref = "urn:example:ssdi-intake-triage/posture-declaration/v1"`.
It exists to make the A.6 cross-check surfaces (rules 1, 2, and 6) narratively
resolvable for a human reader; it is NOT a conformance fixture and does not
carry the full §11 / Appendix A.1–A.4 payload.

The referenced full Posture Declaration would include, at minimum:

- `operator_id = "urn:example:operator/ssa-example-adjudication-unit"` — equal
  to the Delegated-Compute Declaration's `operator_id` (A.6 rule 2).
- An A.2 `access_taxonomy` with rows for `ssdi.intake.questionnaire_response`
  and `ssdi.intake.supporting_document_text`, each declaring
  `access_class = delegated_compute` and
  `delegated_compute_exposure = isolated_enclave` (A.6 rules 1 and 6).
- A custody-model reference (A.4) and a metadata budget (A.3) consistent with
  a confidential-compute enclave posture: provider-operated inference surface
  with sealed plaintext, tenant-held decryption authority, and bounded
  metadata leakage to provider observability.
- A posture-honesty statement per §11 and an operator signature per §11.3.

The orchestrator should replace this stub with a full Posture Declaration
document before any conformance run that touches this Delegated-Compute
Declaration.
