// Rust guideline compliant 2026-02-21
//! WOS verification report types.

#![forbid(unsafe_code)]

pub type WosFinding = trellis_verify::DomainFinding;

/// WOS verification report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WosVerificationReport {
    pub trellis: trellis_verify::VerificationReport,
    pub wos_findings: Vec<WosFinding>,
}

impl From<trellis_verify::VerificationWithDomain> for WosVerificationReport {
    fn from(value: trellis_verify::VerificationWithDomain) -> Self {
        Self {
            trellis: value.trellis,
            wos_findings: value.domain_findings,
        }
    }
}
