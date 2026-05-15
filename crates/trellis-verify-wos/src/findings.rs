// Rust guideline compliant 2026-02-21
//! WOS verification report types.

#![forbid(unsafe_code)]

pub type WosFinding = integrity_verify::trellis::DomainFinding;

/// WOS verification report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WosVerificationReport {
    pub trellis: integrity_verify::trellis::VerificationReport,
    pub wos_findings: Vec<WosFinding>,
}

impl From<integrity_verify::trellis::VerificationWithDomain> for WosVerificationReport {
    fn from(value: integrity_verify::trellis::VerificationWithDomain) -> Self {
        Self {
            trellis: value.trellis,
            wos_findings: value.domain_findings,
        }
    }
}
