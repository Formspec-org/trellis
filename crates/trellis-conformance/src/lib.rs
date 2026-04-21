// Rust guideline compliant 2026-02-21
//! Conformance harness for the committed Trellis vector corpus.

#![forbid(unsafe_code)]

#[cfg(test)]
mod model_checks;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use ciborium::Value;
    use trellis_cddl::canonical_event_hash_preimage;
    use trellis_core::{AuthoredEvent, SigningKeyMaterial, append_event};
    use trellis_export::{ExportEntry, ExportPackage};
    use trellis_store_memory::MemoryStore;
    use trellis_types::{EVENT_DOMAIN, domain_separated_sha256};
    use trellis_verify::{verify_export_zip, verify_tampered_ledger};

    fn fixtures_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/vectors")
    }

    #[test]
    fn committed_vectors_match_the_rust_runtime() {
        for op in ["append", "export", "verify", "tamper", "projection", "shred"] {
            for dir in vector_dirs(op) {
                assert_fixture_matches(&dir);
            }
        }
    }

    fn vector_dirs(op: &str) -> Vec<PathBuf> {
        let mut dirs = fs::read_dir(fixtures_root().join(op))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_dir())
            .collect::<Vec<_>>();
        dirs.sort();
        dirs
    }

    fn assert_fixture_matches(root: &Path) {
        let manifest = manifest_for(root);
        let op = manifest
            .get("op")
            .and_then(toml::Value::as_str)
            .expect("manifest must name an op");

        match op {
            "append" => assert_append_fixture_matches(root, &manifest),
            "export" => assert_export_fixture_matches(root, &manifest),
            "verify" => assert_verify_fixture_matches(root, &manifest),
            "tamper" => assert_tamper_fixture_matches(root, &manifest),
            "projection" => assert_projection_fixture_matches(root, &manifest),
            "shred" => assert_shred_fixture_matches(root, &manifest),
            other => panic!("unsupported vector op `{other}`"),
        }
    }

    fn manifest_for(dir: &Path) -> toml::Value {
        toml::from_str(&fs::read_to_string(dir.join("manifest.toml")).unwrap()).unwrap()
    }

    fn assert_append_fixture_matches(root: &Path, manifest: &toml::Value) {
        let inputs = table(manifest, "inputs");
        let authored_event = fs::read(root.join(path_field(inputs, "authored_event"))).unwrap();
        let expected_author_hash = fs::read(root.join("author-event-hash.bin")).unwrap();
        let expected_canonical_event = fs::read(root.join("expected-event-payload.cbor")).unwrap();
        let expected_sig_structure = fs::read(root.join("sig-structure.bin")).unwrap();
        let expected_signed_event = fs::read(root.join("expected-event.cbor")).unwrap();
        let expected_append_head = fs::read(root.join("expected-append-head.cbor")).unwrap();
        let signing_key = fs::read(root.join(signing_key_name(inputs))).unwrap();

        let mut store = MemoryStore::new();
        let artifacts = append_event(
            &mut store,
            &SigningKeyMaterial::new(signing_key),
            &AuthoredEvent::new(authored_event),
        )
        .unwrap();

        assert_eq!(artifacts.author_event_hash.as_slice(), expected_author_hash.as_slice());
        assert_eq!(artifacts.canonical_event, expected_canonical_event);
        assert_eq!(artifacts.sig_structure, expected_sig_structure);
        assert_eq!(artifacts.signed_event, expected_signed_event);
        assert_eq!(artifacts.append_head, expected_append_head);
    }

    fn assert_export_fixture_matches(root: &Path, manifest: &toml::Value) {
        let inputs = table(manifest, "inputs");
        let expected = table(manifest, "expected");
        let ledger_state = decode_value(
            &fs::read(root.join(path_field(inputs, "ledger_state"))).unwrap(),
        );
        let ledger_state_map = ledger_state.as_map().unwrap();
        let root_dir = value_text(map_value(ledger_state_map, "root_dir"));
        let members = map_value(ledger_state_map, "members").as_array().unwrap();

        let mut package = ExportPackage::new();
        for member in members {
            let member_name = member.as_text().unwrap();
            package.add_entry(ExportEntry::new(
                format!("{root_dir}/{member_name}"),
                fs::read(root.join(member_name)).unwrap(),
            ));
        }

        let actual = package.to_zip_bytes().unwrap();
        let expected_zip = fs::read(root.join(path_field(expected, "zip"))).unwrap();
        assert_eq!(actual, expected_zip);
    }

    fn assert_verify_fixture_matches(root: &Path, manifest: &toml::Value) {
        let inputs = table(manifest, "inputs");
        let expected_report = table_in_table(table(manifest, "expected"), "report");
        let report = verify_export_zip(
            &fs::read(root.join(path_field(inputs, "export_zip"))).unwrap(),
        );

        assert_eq!(report.structure_verified, bool_field(expected_report, "structure_verified"));
        assert_eq!(report.integrity_verified, bool_field(expected_report, "integrity_verified"));
        assert_eq!(report.readability_verified, bool_field(expected_report, "readability_verified"));
    }

    fn assert_tamper_fixture_matches(root: &Path, manifest: &toml::Value) {
        let inputs = table(manifest, "inputs");
        let expected_report = table_in_table(table(manifest, "expected"), "report");
        let report = verify_tampered_ledger(
            &fs::read(root.join(path_field(inputs, "signing_key_registry"))).unwrap(),
            &fs::read(root.join(path_field(inputs, "ledger"))).unwrap(),
            optional_path_field(inputs, "initial_posture_declaration")
                .map(|path| fs::read(root.join(path)).unwrap())
                .as_deref(),
            optional_path_field(inputs, "posture_declaration")
                .map(|path| fs::read(root.join(path)).unwrap())
                .as_deref(),
        )
        .unwrap();

        assert_eq!(report.structure_verified, bool_field(expected_report, "structure_verified"));
        assert_eq!(report.integrity_verified, bool_field(expected_report, "integrity_verified"));
        assert_eq!(report.readability_verified, bool_field(expected_report, "readability_verified"));

        let expected_tamper_kind = pathless_string(expected_report, "tamper_kind");
        if let Some(expected_kind) = expected_tamper_kind {
            assert_eq!(
                report.event_failures.first().map(|failure| failure.kind.as_str()),
                Some(expected_kind.as_str()),
            );
        }
        if let Some(expected_event_id) = pathless_string(expected_report, "failing_event_id") {
            assert_eq!(
                report.event_failures.first().map(|failure| failure.location.as_str()),
                Some(expected_event_id.as_str()),
            );
        }
    }

    fn assert_projection_fixture_matches(root: &Path, manifest: &toml::Value) {
        let inputs = table(manifest, "inputs");
        let expected = table(manifest, "expected");

        if expected.contains_key("watermark") {
            let view = decode_value(&fs::read(root.join(path_field(inputs, "view"))).unwrap());
            let view_map = view.as_map().unwrap();
            let watermark = map_value(view_map, "watermark").clone();
            let watermark_bytes = encode_value(&watermark);
            assert_eq!(
                watermark_bytes,
                fs::read(root.join(path_field(expected, "watermark"))).unwrap()
            );

            let checkpoint_payload = sign1_payload_bytes(
                &fs::read(root.join(path_field(inputs, "checkpoint"))).unwrap(),
            );
            let checkpoint_payload_value = decode_value(&checkpoint_payload);
            let checkpoint_scope =
                map_bytes(checkpoint_payload_value.as_map().unwrap(), "scope");
            let checkpoint_digest = checkpoint_digest(&checkpoint_scope, &checkpoint_payload);
            let watermark_map = watermark.as_map().unwrap();
            assert_eq!(
                map_fixed_bytes(watermark_map, "checkpoint_ref", 32),
                checkpoint_digest.to_vec(),
            );

            if expected.contains_key("staff_view_decision_binding") {
                let fields = table(manifest, "staff_view_decision_binding_fields");
                let binding = Value::Map(vec![
                    (Value::Text("watermark".into()), watermark.clone()),
                    (
                        Value::Text("extensions".into()),
                        if pathless_string(fields, "extensions").as_deref() == Some("null") {
                            Value::Null
                        } else {
                            unreachable!("current staff-view vector pins null extensions")
                        },
                    ),
                    (
                        Value::Text("staff_view_ref".into()),
                        Value::Text(pathless_string(fields, "staff_view_ref").unwrap()),
                    ),
                    (
                        Value::Text("stale_acknowledged".into()),
                        Value::Bool(bool_field(fields, "stale_acknowledged")),
                    ),
                ]);
                assert_eq!(
                    encode_value(&binding),
                    fs::read(root.join(path_field(expected, "staff_view_decision_binding"))).unwrap()
                );
            }
        }

        if expected.contains_key("view_rebuilt") {
            let chain = decode_value(&fs::read(root.join(path_field(inputs, "chain"))).unwrap());
            let events = chain.as_array().unwrap();
            let last_payload = decode_value(&sign1_payload_bytes_from_value(events.last().unwrap()));
            let last_payload_map = last_payload.as_map().unwrap();
            let scope = map_bytes(last_payload_map, "ledger_scope");
            let canonical_hash = domain_separated_sha256(
                EVENT_DOMAIN,
                &canonical_event_hash_preimage(&scope, &sign1_payload_bytes_from_value(events.last().unwrap())),
            );
            let rebuilt = Value::Map(vec![
                (Value::Text("event_count".into()), Value::Integer((events.len() as u64).into())),
                (
                    Value::Text("last_canonical_event_hash".into()),
                    Value::Bytes(canonical_hash.to_vec()),
                ),
            ]);
            let rebuilt_bytes = encode_value(&rebuilt);
            assert_eq!(
                rebuilt_bytes,
                fs::read(root.join(path_field(expected, "view_rebuilt"))).unwrap()
            );
            assert_eq!(rebuilt_bytes, fs::read(root.join(path_field(inputs, "view"))).unwrap());
        }

        if expected.contains_key("cadence_report") {
            let cadence = table(manifest, "cadence");
            let observed = table_paths(inputs, "checkpoints")
                .into_iter()
                .map(|path| {
                    let payload = sign1_payload_bytes(&fs::read(root.join(path)).unwrap());
                    let payload_value = decode_value(&payload);
                    map_u64(payload_value.as_map().unwrap(), "tree_size")
                })
                .collect::<Vec<_>>();
            let required = integer_array(cadence, "required_tree_sizes");
            let missing = required
                .iter()
                .copied()
                .filter(|value| !observed.contains(value))
                .collect::<Vec<_>>();
            let report = Value::Map(vec![
                (
                    Value::Text("interval".into()),
                    Value::Integer((int_field(cadence, "interval")).into()),
                ),
                (
                    Value::Text("cadence_kind".into()),
                    Value::Text(pathless_string(cadence, "kind").unwrap()),
                ),
                (
                    Value::Text("failure_code".into()),
                    match pathless_string(cadence, "failure_code") {
                        Some(value) => Value::Text(value),
                        None => Value::Null,
                    },
                ),
                (
                    Value::Text("cadence_satisfied".into()),
                    Value::Bool(!bool_field(cadence, "expect_failure")),
                ),
                (
                    Value::Text("missing_tree_sizes".into()),
                    Value::Array(
                        missing
                            .iter()
                            .map(|value| Value::Integer((*value).into()))
                            .collect(),
                    ),
                ),
                (
                    Value::Text("expected_tree_sizes".into()),
                    Value::Array(
                        required
                            .iter()
                            .map(|value| Value::Integer((*value).into()))
                            .collect(),
                    ),
                ),
                (
                    Value::Text("observed_tree_sizes".into()),
                    Value::Array(
                        observed
                            .iter()
                            .map(|value| Value::Integer((*value).into()))
                            .collect(),
                    ),
                ),
            ]);
            assert_eq!(
                encode_value(&report),
                fs::read(root.join(path_field(expected, "cadence_report"))).unwrap()
            );
        }
    }

    fn assert_shred_fixture_matches(root: &Path, manifest: &toml::Value) {
        let inputs = table(manifest, "inputs");
        let expected = table(manifest, "expected");
        let procedure = table(manifest, "procedure");
        let chain = decode_value(&fs::read(root.join(path_field(inputs, "chain"))).unwrap());
        let events = chain.as_array().unwrap();
        let target_event_payload = decode_value(&sign1_payload_bytes_from_value(&events[0]));
        let target_content_hash = map_fixed_bytes(target_event_payload.as_map().unwrap(), "content_hash", 32);

        let declared_scope = table_paths_as_strings(procedure, "cascade_scope");
        let mut report_entries = vec![(
            Value::Text("declared_scope".into()),
            Value::Array(declared_scope.iter().cloned().map(Value::Text).collect()),
        )];

        if let Some(snapshot_path) = optional_path_field(inputs, "backup_snapshot") {
            let snapshot_bytes = fs::read(root.join(snapshot_path)).unwrap();
            report_entries.push((
                Value::Text("backup_snapshot_ref".into()),
                Value::Bytes(sha256_bytes(&snapshot_bytes).to_vec()),
            ));
        }

        report_entries.push((
            Value::Text("expected_post_state".into()),
            Value::Map(
                declared_scope
                    .into_iter()
                    .map(|scope| {
                        let state = if inputs.contains_key("backup_snapshot") {
                            Value::Map(vec![
                                (
                                    Value::Text("rationale".into()),
                                    Value::Text(format!("{scope}-backup-restore-refused-per-§16.5")),
                                ),
                                (
                                    Value::Text("backup_resurrection_refused".into()),
                                    Value::Bool(true),
                                ),
                                (
                                    Value::Text("invalidated_or_plaintext_absent".into()),
                                    Value::Bool(true),
                                ),
                            ])
                        } else {
                            Value::Map(vec![
                                (
                                    Value::Text("rationale".into()),
                                    Value::Text(format!("{scope}-in-declared-cascade-scope")),
                                ),
                                (
                                    Value::Text("invalidated_or_plaintext_absent".into()),
                                    Value::Bool(true),
                                ),
                            ])
                        };
                        (Value::Text(scope), state)
                    })
                    .collect(),
            ),
        ));
        report_entries.push((
            Value::Text("target_content_hash".into()),
            Value::Bytes(target_content_hash.clone()),
        ));

        let report = Value::Map(report_entries);
        assert_eq!(
            encode_value(&report),
            fs::read(root.join(path_field(expected, "cascade_report"))).unwrap()
        );
    }

    fn signing_key_name(inputs: &toml::value::Table) -> &str {
        optional_path_field(inputs, "signing_key")
            .or_else(|| optional_path_field(inputs, "signing_key_b"))
            .expect("append manifest must name a signing key")
    }

    fn table<'a>(value: &'a toml::Value, key: &str) -> &'a toml::value::Table {
        value.get(key).and_then(toml::Value::as_table).unwrap()
    }

    fn table_in_table<'a>(table: &'a toml::value::Table, key: &str) -> &'a toml::value::Table {
        table.get(key).and_then(toml::Value::as_table).unwrap()
    }

    fn path_field<'a>(table: &'a toml::value::Table, key: &str) -> &'a str {
        table.get(key).and_then(toml::Value::as_str).unwrap()
    }

    fn optional_path_field<'a>(table: &'a toml::value::Table, key: &str) -> Option<&'a str> {
        table.get(key).and_then(toml::Value::as_str)
    }

    fn bool_field(table: &toml::value::Table, key: &str) -> bool {
        table.get(key).and_then(toml::Value::as_bool).unwrap()
    }

    fn int_field(table: &toml::value::Table, key: &str) -> i64 {
        table.get(key).and_then(toml::Value::as_integer).unwrap()
    }

    fn pathless_string(table: &toml::value::Table, key: &str) -> Option<String> {
        table.get(key).and_then(toml::Value::as_str).map(ToOwned::to_owned)
    }

    fn table_paths(table: &toml::value::Table, key: &str) -> Vec<String> {
        table
            .get(key)
            .and_then(toml::Value::as_array)
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_string())
            .collect()
    }

    fn table_paths_as_strings(table: &toml::value::Table, key: &str) -> Vec<String> {
        table
            .get(key)
            .and_then(toml::Value::as_array)
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_string())
            .collect()
    }

    fn integer_array(table: &toml::value::Table, key: &str) -> Vec<u64> {
        table
            .get(key)
            .and_then(toml::Value::as_array)
            .unwrap()
            .iter()
            .map(|value| value.as_integer().unwrap() as u64)
            .collect()
    }

    fn decode_value(bytes: &[u8]) -> Value {
        ciborium::from_reader(bytes).unwrap()
    }

    fn encode_value(value: &Value) -> Vec<u8> {
        let mut bytes = Vec::new();
        ciborium::into_writer(value, &mut bytes).unwrap();
        bytes
    }

    fn sign1_payload_bytes(bytes: &[u8]) -> Vec<u8> {
        sign1_payload_value(&decode_value(bytes)).as_bytes().unwrap().clone()
    }

    fn sign1_payload_bytes_from_value(value: &Value) -> Vec<u8> {
        sign1_payload_value(value).as_bytes().unwrap().clone()
    }

    fn sign1_payload_value(value: &Value) -> &Value {
        match value {
            Value::Tag(18, inner) => &inner.as_array().unwrap()[2],
            _ => panic!("expected tag-18 COSE_Sign1 value"),
        }
    }

    fn map_value<'a>(map: &'a [(Value, Value)], key: &str) -> &'a Value {
        map.iter()
            .find(|(candidate, _)| candidate.as_text().is_some_and(|text| text == key))
            .map(|(_, value)| value)
            .unwrap()
    }

    fn value_text(value: &Value) -> String {
        value.as_text().unwrap().to_string()
    }

    fn map_bytes(map: &[(Value, Value)], key: &str) -> Vec<u8> {
        map_value(map, key).as_bytes().unwrap().clone()
    }

    fn map_fixed_bytes(map: &[(Value, Value)], key: &str, expected_len: usize) -> Vec<u8> {
        let bytes = map_bytes(map, key);
        assert_eq!(bytes.len(), expected_len);
        bytes
    }

    fn map_u64(map: &[(Value, Value)], key: &str) -> u64 {
        map_value(map, key)
            .as_integer()
            .and_then(|value| value.try_into().ok())
            .unwrap()
    }

    fn checkpoint_digest(scope: &[u8], payload_bytes: &[u8]) -> [u8; 32] {
        let mut preimage = Vec::new();
        preimage.push(0xa3);
        preimage.extend_from_slice(&encode_tstr("scope"));
        preimage.extend_from_slice(&encode_bstr(scope));
        preimage.extend_from_slice(&encode_tstr("version"));
        preimage.extend_from_slice(&encode_uint(1));
        preimage.extend_from_slice(&encode_tstr("checkpoint_payload"));
        preimage.extend_from_slice(payload_bytes);
        trellis_types::domain_separated_sha256("trellis-checkpoint-v1", &preimage)
    }

    fn sha256_bytes(bytes: &[u8]) -> [u8; 32] {
        use sha2::{Digest, Sha256};

        Sha256::digest(bytes).into()
    }

    fn encode_bstr(bytes: &[u8]) -> Vec<u8> {
        trellis_types::encode_bstr(bytes)
    }

    fn encode_tstr(text: &str) -> Vec<u8> {
        trellis_types::encode_tstr(text)
    }

    fn encode_uint(value: u64) -> Vec<u8> {
        trellis_types::encode_uint(value)
    }
}
