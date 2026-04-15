# Task 11 — Removed ULCOMP-R rows

Working file consumed by Task 14 (cross-reference-map). Delete after Task 14 completes.

Removed from `unified-ledger-companion-requirements-matrix.md` in Plan 3 (2026-04-15). These IDs are permanently retired per §1 "These IDs are stable"; removed IDs are not reused.

| ULCOMP-R-ID | Concept | Upstream home |
|---|---|---|
| ULCOMP-R-067 | User-held reuse — MUST support reuse/reference of prior user-held records | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-068 | User-held reuse — MUST bind exactly what was reused when entering canonical workflow | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-069 | User-held reuse — MUST distinguish reusable prior records from canonical workflow state | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-070 | User-held reuse — MUST distinguish workflow submission from prior-record possession | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-071 | User-held reuse — MUST avoid treating user-held record layer as canonical workflow by default | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-072 | User-held reuse — SHOULD favor selective submission over bulk transfer | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-073 | User-held reuse — MUST bind what was introduced when reused material enters canonical truth | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-074 | User-held reuse — SHOULD bind reuse context where relevant | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-075 | User-held reuse — MUST preserve provenance distinctions among user-held / canonical / disclosure | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-076 | Respondent history — MUST scope to respondent-originated/visible material | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-077 | Respondent history — MAY support listed moments (draft, save, submit) | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-078 | Respondent history — MUST treat views as projections over canonical truth | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-079 | Respondent history — MUST NOT define a second canonical append model | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-080 | Respondent history — MUST NOT imply broader workflow/governance/custody coverage | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-081 | Respondent history materiality — MUST prioritize material state changes over UI telemetry | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-082 | Respondent history materiality — MUST NOT require ephemeral interface event capture | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-083 | Respondent history materiality — SHOULD expose validation/submission/amendment boundaries | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-084 | Respondent history materiality — MAY define profile-specific change-set semantics | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-085 | Respondent history export — MAY present profile-specific timeline/delta | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-086 | Respondent history export — MUST preserve provenance distinctions | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-087 | Respondent history export — MUST NOT imply broader coverage than scope | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-135 | Identity & attestation — SHOULD represent provider-neutrally where feasible | [WOS Assurance §3 identity/attestation semantics] |
| ULCOMP-R-136 | Identity & attestation — provider-specific issuers/adapters operational only | [WOS Assurance §3 identity/attestation semantics] |
| ULCOMP-R-137 | User signing — SHOULD support user-originated signatures | [WOS Assurance §3 authentication/signing] |
| ULCOMP-R-138 | User signing — MAY support offline user-originated signing | [WOS Assurance §3 authentication/signing] |
| ULCOMP-R-140 | Assurance & disclosure — MUST distinguish assurance from disclosure posture | [WOS Assurance §2 assurance taxonomy, §4 Invariant 6], [Formspec Respondent Ledger §6.6.1 assuranceLevel] |
| ULCOMP-R-141 | Assurance & disclosure — MUST NOT treat higher assurance as requiring greater identity disclosure | [WOS Assurance §4 Invariant 6], [Formspec Respondent Ledger §6.6A Subject Continuity] |
| ULCOMP-R-142 | Assurance & disclosure — MAY support subject continuity without full identity disclosure | [WOS Assurance §3 subject continuity], [Formspec Respondent Ledger §6.6A Subject Continuity] |
| ULCOMP-R-155 | Lifecycle — MAY define lifecycle facts (retention, hold, archival, etc.) | [WOS Governance §2.9 Schema Upgrade, §7.15 Legal Hold] |
| ULCOMP-R-156 | Lifecycle — MAY support subset or none of lifecycle operations | [WOS Governance §2.9, §7.15] |
| ULCOMP-R-157 | Lifecycle — MUST represent compliance-relevant operations as lifecycle facts | [WOS Governance §2.9, §7.15] |
| ULCOMP-R-158 | Lifecycle — compliance/retention/recoverability claims MUST be canonical facts | [WOS Governance §2.9, §7.15] |
| ULCOMP-R-161 | Sealing — MUST define whether sealed scopes permit later lifecycle/governance facts | [WOS Governance §7.15 Legal Hold, sealing/precedence] |
| ULCOMP-R-162 | Sealing — MUST define retention vs hold precedence | [WOS Governance §7.15 Legal Hold, retention precedence] |
| ULCOMP-R-181 | Forms sidecar — stable form-path semantics | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-182 | Forms sidecar — item-key semantics | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-183 | Forms sidecar — validation snapshot semantics | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-184 | Forms sidecar — amendment-cycle semantics | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-185 | Forms sidecar — migration outcome semantics | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-186 | Forms sidecar — change-set semantics | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-187 | Forms sidecar — history moments reproducible from canonical/profile material | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-188 | Forms sidecar — respondent export views remain derived/disclosure-oriented | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCOMP-R-189 | Workflow sidecar — workflow mapping distinctions (operational vs canonical) | [WOS Governance §7 workflow governance] |
| ULCOMP-R-190 | Workflow sidecar — governance fact-family canonical-vs-operational classification | [WOS Governance §7] |
| ULCOMP-R-191 | Workflow sidecar — review-stage semantics | [WOS Governance §7] |
| ULCOMP-R-192 | Workflow sidecar — approval/recovery/compensating semantics | [WOS Governance §7] |
| ULCOMP-R-193 | Workflow sidecar — operational sequencing MUST NOT be mistaken for canonical order | [WOS Governance §7] |
| ULCOMP-R-194 | Workflow sidecar — provenance family (trace derived to canonical) | [WOS Governance §7] |
| ULCOMP-R-195 | Workflow sidecar — conflict family distinctions | [WOS Governance §7] |
| ULCOMP-R-196 | Workflow sidecar — export views preserve provenance/scope honesty | [WOS Governance §7] |
| ULCOMP-R-197 | Appendix A — versioned registries for identifier/kind categories | [WOS Governance §2 registry conventions] |
