// Rust guideline compliant 2026-02-21
//! Fixture-oriented CLI for the current Trellis Rust scaffold.

#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use trellis_cddl::parse_ed25519_cose_key;
use trellis_core::{AuthoredEvent, SigningKeyMaterial, append_event};
use trellis_export::{ExportEntry, ExportPackage};
use trellis_store_memory::MemoryStore;
use trellis_verify::verify_single_event;

fn main() {
    if let Err(message) = run() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let command = std::env::args().nth(1).ok_or_else(|| {
        "usage: trellis-cli <append-001|append-002|verify-001|verify-002|export-001|export-002>"
            .to_string()
    })?;
    dispatch_command(command.as_str())
}

fn dispatch_command(command: &str) -> Result<(), String> {
    match command {
        "append-001" => append_001_command(),
        "append-002" => append_002_command(),
        "verify-001" => verify_001_command(),
        "verify-002" => verify_002_command(),
        "export-001" => export_001_command(),
        "export-002" => export_002_command(),
        _ => Err(format!("unknown command `{command}`")),
    }
}

fn append_001_command() -> Result<(), String> {
    let (authored_event, signing_key) =
        fixture_inputs("001-minimal-inline-payload", "issuer-001.cose_key")?;
    let mut store = MemoryStore::new();
    let artifacts = append_event(
        &mut store,
        &SigningKeyMaterial::new(signing_key),
        &AuthoredEvent::new(authored_event),
    )
    .map_err(|error| error.to_string())?;

    println!(
        "append/001 canonical={} signed={} append_head={}",
        artifacts.canonical_event.len(),
        artifacts.signed_event.len(),
        artifacts.append_head.len()
    );
    Ok(())
}

fn append_002_command() -> Result<(), String> {
    let (authored_event, signing_key) =
        fixture_inputs("002-rotation-signing-key", "issuer-002.cose_key")?;
    let mut store = MemoryStore::new();
    let artifacts = append_event(
        &mut store,
        &SigningKeyMaterial::new(signing_key),
        &AuthoredEvent::new(authored_event),
    )
    .map_err(|error| error.to_string())?;

    println!(
        "append/002 canonical={} signed={} append_head={}",
        artifacts.canonical_event.len(),
        artifacts.signed_event.len(),
        artifacts.append_head.len()
    );
    Ok(())
}

fn verify_001_command() -> Result<(), String> {
    let key_bytes = fs::read(key_path("issuer-001.cose_key"))
        .map_err(|error| format!("failed to read key fixture: {error}"))?;
    let parsed_key = parse_ed25519_cose_key(&key_bytes).map_err(|error| error.to_string())?;
    let signed_event =
        fs::read(fixture_root("001-minimal-inline-payload").join("expected-event.cbor"))
            .map_err(|error| format!("failed to read event fixture: {error}"))?;

    let report = verify_single_event(parsed_key.public_key, &signed_event)
        .map_err(|error| error.to_string())?;
    println!(
        "structure_verified={} integrity_verified={} readability_verified={}",
        report.structure_verified, report.integrity_verified, report.readability_verified
    );
    Ok(())
}

fn verify_002_command() -> Result<(), String> {
    let key_bytes = fs::read(key_path("issuer-002.cose_key"))
        .map_err(|error| format!("failed to read key fixture: {error}"))?;
    let parsed_key = parse_ed25519_cose_key(&key_bytes).map_err(|error| error.to_string())?;
    let signed_event =
        fs::read(fixture_root("002-rotation-signing-key").join("expected-event.cbor"))
            .map_err(|error| format!("failed to read event fixture: {error}"))?;

    let report = verify_single_event(parsed_key.public_key, &signed_event)
        .map_err(|error| error.to_string())?;
    println!(
        "structure_verified={} integrity_verified={} readability_verified={}",
        report.structure_verified, report.integrity_verified, report.readability_verified
    );
    Ok(())
}

fn export_001_command() -> Result<(), String> {
    let (authored_event, signing_key) =
        fixture_inputs("001-minimal-inline-payload", "issuer-001.cose_key")?;
    let mut store = MemoryStore::new();
    let artifacts = append_event(
        &mut store,
        &SigningKeyMaterial::new(signing_key),
        &AuthoredEvent::new(authored_event),
    )
    .map_err(|error| error.to_string())?;

    let mut package = ExportPackage::new();
    package.add_entry(ExportEntry::new(
        "010-canonical-event.cbor",
        artifacts.canonical_event,
    ));
    package.add_entry(ExportEntry::new(
        "020-signed-event.cbor",
        artifacts.signed_event,
    ));
    package.add_entry(ExportEntry::new(
        "030-append-head.cbor",
        artifacts.append_head,
    ));

    let zip_bytes = package.to_zip_bytes().map_err(|error| error.to_string())?;
    println!("zip_bytes={}", zip_bytes.len());
    Ok(())
}

fn export_002_command() -> Result<(), String> {
    let (authored_event, signing_key) =
        fixture_inputs("002-rotation-signing-key", "issuer-002.cose_key")?;
    let mut store = MemoryStore::new();
    let artifacts = append_event(
        &mut store,
        &SigningKeyMaterial::new(signing_key),
        &AuthoredEvent::new(authored_event),
    )
    .map_err(|error| error.to_string())?;

    let mut package = ExportPackage::new();
    package.add_entry(ExportEntry::new(
        "010-canonical-event.cbor",
        artifacts.canonical_event,
    ));
    package.add_entry(ExportEntry::new(
        "020-signed-event.cbor",
        artifacts.signed_event,
    ));
    package.add_entry(ExportEntry::new(
        "030-append-head.cbor",
        artifacts.append_head,
    ));

    let zip_bytes = package.to_zip_bytes().map_err(|error| error.to_string())?;
    println!("zip_bytes={}", zip_bytes.len());
    Ok(())
}

fn fixture_inputs(dir: &str, key_file: &str) -> Result<(Vec<u8>, Vec<u8>), String> {
    let authored_event = fs::read(fixture_root(dir).join("input-author-event-hash-preimage.cbor"))
        .map_err(|error| format!("failed to read authored fixture: {error}"))?;
    let signing_key = fs::read(key_path(key_file))
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
    use super::dispatch_command;

    #[test]
    fn dispatch_rejects_unknown_command() {
        assert_eq!(
            dispatch_command("not-a-real-command").unwrap_err(),
            "unknown command `not-a-real-command`"
        );
    }

    #[test]
    fn dispatch_accepts_fixture_command_names() {
        for cmd in [
            "append-001",
            "append-002",
            "verify-001",
            "verify-002",
            "export-001",
            "export-002",
        ] {
            assert!(
                dispatch_command(cmd).is_ok(),
                "fixture command `{cmd}` should run against committed vectors"
            );
        }
    }
}
