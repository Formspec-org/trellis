// Rust guideline compliant 2026-02-21
//! Fixture-oriented CLI for Trellis smoke checks and export verification.

#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use integrity_verify::trellis::{Severity, verify_single_event};
use trellis_cddl::parse_ed25519_cose_key;
use trellis_core::{AuthoredEvent, SigningKeyMaterial, append_event};
use trellis_store_memory::MemoryStore;

#[derive(Clone, Copy)]
struct FixtureCase {
    label: &'static str,
    vector_dir: &'static str,
    key_file: &'static str,
}

const FIXTURE_001: FixtureCase = FixtureCase {
    label: "001",
    vector_dir: "001-minimal-inline-payload",
    key_file: "issuer-001.cose_key",
};

const FIXTURE_002: FixtureCase = FixtureCase {
    label: "002",
    vector_dir: "002-rotation-signing-key",
    key_file: "issuer-002.cose_key",
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(message) = run(args.as_slice()) {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

fn usage_top_level() -> String {
    "usage: trellis-cli <append-001|append-002|verify-001|verify-002|verify-export <bundle.zip>>\n\
     \n\
     Fixture commands mirror a small smoke subset of the Trellis append corpus.\n\
     `verify-export` verifies a Trellis/WOS export ZIP through trellis-verify-wos.\n\
     Run the full committed vector set via the `trellis-conformance` binary."
        .to_string()
}

fn run(args: &[String]) -> Result<(), String> {
    let command = args
        .get(1)
        .map(String::as_str)
        .ok_or_else(usage_top_level)?;
    dispatch_command(args, command)
}

fn dispatch_command(args: &[String], command: &str) -> Result<(), String> {
    match command {
        "append-001" => append_fixture_command(&FIXTURE_001),
        "append-002" => append_fixture_command(&FIXTURE_002),
        "verify-001" => verify_fixture_command(&FIXTURE_001),
        "verify-002" => verify_fixture_command(&FIXTURE_002),
        "verify-export" => verify_export_command(args),
        _ => Err(format!("unknown command `{command}`")),
    }
}

fn append_fixture_command(fixture: &FixtureCase) -> Result<(), String> {
    let (authored_event, signing_key) = fixture_inputs(fixture)?;
    let mut store = MemoryStore::new();
    let artifacts = append_event(
        &mut store,
        &SigningKeyMaterial::new(signing_key),
        &AuthoredEvent::new(authored_event),
    )
    .map_err(|error| error.to_string())?;

    println!(
        "append/{} canonical={} signed={} append_head={}",
        fixture.label,
        artifacts.canonical_event.len(),
        artifacts.signed_event.len(),
        artifacts.append_head.len()
    );
    Ok(())
}

fn verify_fixture_command(fixture: &FixtureCase) -> Result<(), String> {
    let key_bytes = fs::read(key_path(fixture.key_file))
        .map_err(|error| format!("failed to read key fixture: {error}"))?;
    let parsed_key = parse_ed25519_cose_key(&key_bytes).map_err(|error| error.to_string())?;
    let signed_event = fs::read(fixture_root(fixture.vector_dir).join("expected-event.cbor"))
        .map_err(|error| format!("failed to read event fixture: {error}"))?;

    let report = verify_single_event(parsed_key.public_key, &signed_event)
        .map_err(|error| error.to_string())?;
    println!(
        "structure_verified={} integrity_verified={} readability_verified={}",
        report.structure_verified, report.integrity_verified, report.readability_verified
    );
    Ok(())
}

fn verify_export_command(args: &[String]) -> Result<(), String> {
    let path = args
        .get(2)
        .ok_or_else(|| "usage: trellis-cli verify-export <bundle.zip>".to_string())?;
    let bytes = fs::read(path)
        .map_err(|error| format!("failed to read export bundle `{path}`: {error}"))?;
    let report = trellis_verify_wos::verify_export_zip(&bytes);
    let trellis_failures = report.trellis.event_failures.len()
        + report.trellis.checkpoint_failures.len()
        + report.trellis.proof_failures.len();
    let wos_failures = report
        .wos_findings
        .iter()
        .filter(|finding| finding.severity == Severity::Failure)
        .count();
    let failures = trellis_failures + wos_failures;

    println!(
        "verified={} trellis_failures={} wos_findings={}",
        report.trellis.structure_verified && report.trellis.integrity_verified && failures == 0,
        trellis_failures,
        report.wos_findings.len()
    );
    if report.trellis.structure_verified && report.trellis.integrity_verified && failures == 0 {
        Ok(())
    } else {
        Err(format!(
            "export verification failed with {failures} failure finding(s)"
        ))
    }
}

fn fixture_inputs(fixture: &FixtureCase) -> Result<(Vec<u8>, Vec<u8>), String> {
    let authored_event =
        fs::read(fixture_root(fixture.vector_dir).join("input-author-event-hash-preimage.cbor"))
            .map_err(|error| format!("failed to read authored fixture: {error}"))?;
    let signing_key = fs::read(key_path(fixture.key_file))
        .map_err(|error| format!("failed to read key fixture: {error}"))?;
    Ok((authored_event, signing_key))
}

fn fixture_root(dir: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../fixtures/vectors/append/{dir}"))
}

fn key_path(file: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../fixtures/vectors/_keys/{file}"))
}

#[cfg(test)]
mod tests {
    use super::{dispatch_command, run};

    #[test]
    fn dispatch_rejects_unknown_command() {
        assert_eq!(
            dispatch_command(
                &["trellis-cli".into(), "not-a-real-command".into()],
                "not-a-real-command"
            )
            .unwrap_err(),
            "unknown command `not-a-real-command`"
        );
    }

    #[test]
    fn dispatch_accepts_fixture_command_names() {
        for cmd in ["append-001", "append-002", "verify-001", "verify-002"] {
            assert!(
                dispatch_command(&["trellis-cli".into(), cmd.into()], cmd).is_ok(),
                "fixture command `{cmd}` should run against committed vectors"
            );
        }
    }

    #[test]
    fn verify_export_requires_path() {
        let err = run(&["trellis-cli".into(), "verify-export".into()]).unwrap_err();
        assert!(err.contains("verify-export <bundle.zip>"), "{err}");
    }
}
