// Rust guideline compliant 2026-02-21
//! WOS rescission terminality checks.

#![forbid(unsafe_code)]

use trellis_verify::{DomainEvent, DomainFinding, Severity};

use crate::event_types::{
    WOS_GOVERNANCE_DETERMINATION_PREFIX, WOS_GOVERNANCE_DETERMINATION_RESCINDED_EVENT_TYPE,
    WOS_GOVERNANCE_REINSTATED_EVENT_TYPE,
};

pub(crate) fn validate_rescission_terminality(events: &[DomainEvent]) -> Vec<DomainFinding> {
    let mut terminal = false;
    let mut findings = Vec::new();
    for event in events {
        if event.event_type == WOS_GOVERNANCE_DETERMINATION_RESCINDED_EVENT_TYPE {
            terminal = true;
        } else if event.event_type == WOS_GOVERNANCE_REINSTATED_EVENT_TYPE {
            terminal = false;
        } else if terminal
            && event
                .event_type
                .starts_with(WOS_GOVERNANCE_DETERMINATION_PREFIX)
        {
            findings.push(DomainFinding::new(
                "rescission_terminality_violation",
                Some(event.canonical_event_hash),
                Severity::Failure,
                "determination event appears after rescission without reinstatement",
            ));
        }
    }
    findings
}
