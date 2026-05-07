use std::collections::BTreeMap;

use ciborium::Value;
use serde_json::Value as JsonValue;
use trellis_types::{map_lookup_map, map_lookup_optional_value, map_lookup_text, sha256_bytes};

use crate::kinds::VerificationFailureKind;
use crate::parse::{decode_event_details, decode_value, readable_payload_bytes};
use crate::types::{
    ExportArchive, OpenClockCatalog, OpenClockCatalogRow, OpenClocksExportExtension, ParsedSign1,
    TrellisTimestamp, VerificationFailure, VerificationReport,
};
use crate::util::{hex_decode, hex_string};

const OPEN_CLOCKS_MEMBER: &str = "open-clocks.json";
const CLOCK_STARTED_RECORD_KIND: &str = "clockStarted";
const CLOCK_RESOLVED_RECORD_KIND: &str = "clockResolved";
const CLOCK_RESOLUTION_PAUSED: &str = "paused";

pub(crate) fn verify_open_clocks(
    archive: &ExportArchive,
    extension: &OpenClocksExportExtension,
    sealed_at: TrellisTimestamp,
    report: &mut VerificationReport,
) {
    let Some(catalog_bytes) = archive.members.get(OPEN_CLOCKS_MEMBER) else {
        report.event_failures.push(VerificationFailure::new(
            VerificationFailureKind::ArchiveIntegrityFailure,
            OPEN_CLOCKS_MEMBER,
        ));
        return;
    };

    if sha256_bytes(catalog_bytes) != extension.open_clocks_digest {
        report.event_failures.push(VerificationFailure::new(
            VerificationFailureKind::ArchiveIntegrityFailure,
            OPEN_CLOCKS_MEMBER,
        ));
        return;
    }

    let catalog = match parse_open_clocks_catalog(catalog_bytes) {
        Ok(catalog) => catalog,
        Err(error) => {
            report.event_failures.push(VerificationFailure::new(
                VerificationFailureKind::ManifestPayloadInvalid,
                format!("{OPEN_CLOCKS_MEMBER}/{error}"),
            ));
            return;
        }
    };

    if catalog.open_clocks.len() as u64 != extension.open_clock_count {
        report.event_failures.push(VerificationFailure::new(
            VerificationFailureKind::ManifestPayloadInvalid,
            OPEN_CLOCKS_MEMBER,
        ));
    }
    if catalog.sealed_at != sealed_at {
        report.event_failures.push(VerificationFailure::new(
            VerificationFailureKind::ManifestPayloadInvalid,
            format!("{OPEN_CLOCKS_MEMBER}/sealed_at"),
        ));
    }

    for row in &catalog.open_clocks {
        if row.computed_deadline < catalog.sealed_at {
            report.warnings.push(format!(
                "open_clock_overdue:{}:{}",
                row.clock_id,
                hex_string(&row.origin_event_hash)
            ));
        }
    }
}

pub(crate) fn verify_unbound_open_clocks(
    archive: &ExportArchive,
    extension_present: bool,
    report: &mut VerificationReport,
) {
    if !extension_present && archive.members.contains_key(OPEN_CLOCKS_MEMBER) {
        report.event_failures.push(VerificationFailure::new(
            VerificationFailureKind::ManifestPayloadInvalid,
            OPEN_CLOCKS_MEMBER,
        ));
    }
}

pub(crate) fn verify_clock_segments(
    events: &[ParsedSign1],
    payload_blobs: &BTreeMap<[u8; 32], Vec<u8>>,
    report: &mut VerificationReport,
) {
    let mut active = BTreeMap::<String, ClockSegment>::new();
    let mut paused = BTreeMap::<String, ClockSegment>::new();

    for event in events {
        let Ok(details) = decode_event_details(event) else {
            continue;
        };
        let Some(payload_bytes) = readable_payload_bytes(&details, payload_blobs) else {
            continue;
        };
        let Ok(clock_record) = parse_clock_record(&payload_bytes) else {
            continue;
        };
        let Some(clock_record) = clock_record else {
            continue;
        };

        match clock_record {
            ClockRecord::Started(started) => {
                if let Some(paused_segment) = paused.remove(&started.clock_id)
                    && (paused_segment.clock_kind != started.clock_kind
                        || paused_segment.calendar_ref != started.calendar_ref)
                {
                    report.event_failures.push(VerificationFailure::new(
                        VerificationFailureKind::ClockCalendarMismatch,
                        hex_string(&details.canonical_event_hash),
                    ));
                }
                active.insert(
                    started.clock_id.clone(),
                    ClockSegment {
                        clock_kind: started.clock_kind,
                        calendar_ref: started.calendar_ref,
                    },
                );
            }
            ClockRecord::Resolved(resolved) => {
                if resolved.resolution == CLOCK_RESOLUTION_PAUSED {
                    if let Some(segment) = active.remove(&resolved.clock_id) {
                        paused.insert(resolved.clock_id, segment);
                    }
                } else {
                    active.remove(&resolved.clock_id);
                    paused.remove(&resolved.clock_id);
                }
            }
        }
    }
}

pub(crate) fn parse_open_clocks_catalog(bytes: &[u8]) -> Result<OpenClockCatalog, String> {
    if bytes.starts_with(b"\xef\xbb\xbf") {
        return Err("BOM is forbidden".to_string());
    }
    let text = std::str::from_utf8(bytes).map_err(|_| "catalog is not UTF-8".to_string())?;
    if !text.ends_with('\n') || text[..text.len() - 1].contains('\n') {
        return Err("catalog must have one trailing newline".to_string());
    }
    let value: JsonValue =
        serde_json::from_str(text).map_err(|error| format!("invalid JSON: {error}"))?;
    let catalog = parse_catalog_value(&value)?;
    validate_order(&catalog.open_clocks)?;
    let canonical = render_open_clocks_catalog(&catalog);
    if canonical.as_bytes() != bytes {
        return Err("catalog is not Trellis canonical JSON".to_string());
    }
    Ok(catalog)
}

#[derive(Clone, Debug)]
struct ClockSegment {
    clock_kind: String,
    calendar_ref: Option<String>,
}

enum ClockRecord {
    Started(ClockStartedRecord),
    Resolved(ClockResolvedRecord),
}

struct ClockStartedRecord {
    clock_id: String,
    clock_kind: String,
    calendar_ref: Option<String>,
}

struct ClockResolvedRecord {
    clock_id: String,
    resolution: String,
}

fn parse_clock_record(payload_bytes: &[u8]) -> Result<Option<ClockRecord>, String> {
    let value = decode_value(payload_bytes).map_err(|error| error.to_string())?;
    let map = value
        .as_map()
        .ok_or_else(|| "clock record root is not a map".to_string())?;
    let record_kind = map_lookup_text(map, "recordKind").map_err(|error| error.to_string())?;
    match record_kind.as_str() {
        CLOCK_STARTED_RECORD_KIND => Ok(Some(ClockRecord::Started(parse_clock_started(map)?))),
        CLOCK_RESOLVED_RECORD_KIND => Ok(Some(ClockRecord::Resolved(parse_clock_resolved(map)?))),
        _ => Ok(None),
    }
}

fn parse_clock_started(map: &[(Value, Value)]) -> Result<ClockStartedRecord, String> {
    let data = map_lookup_map(map, "data").map_err(|error| error.to_string())?;
    Ok(ClockStartedRecord {
        clock_id: map_lookup_text(data, "clockId").map_err(|error| error.to_string())?,
        clock_kind: map_lookup_text(data, "clockKind").map_err(|error| error.to_string())?,
        calendar_ref: optional_text(data, "calendarRef")?,
    })
}

fn parse_clock_resolved(map: &[(Value, Value)]) -> Result<ClockResolvedRecord, String> {
    let data = map_lookup_map(map, "data").map_err(|error| error.to_string())?;
    Ok(ClockResolvedRecord {
        clock_id: map_lookup_text(data, "clockId").map_err(|error| error.to_string())?,
        resolution: map_lookup_text(data, "resolution").map_err(|error| error.to_string())?,
    })
}

fn optional_text(map: &[(Value, Value)], key: &str) -> Result<Option<String>, String> {
    let Some(value) = map_lookup_optional_value(map, key) else {
        return Ok(None);
    };
    match value {
        Value::Text(text) => Ok(Some(text.clone())),
        Value::Null => Ok(None),
        _ => Err(format!("{key} must be text or null")),
    }
}

fn parse_catalog_value(value: &JsonValue) -> Result<OpenClockCatalog, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "catalog root is not an object".to_string())?;
    let keys = object.keys().map(String::as_str).collect::<Vec<_>>();
    if keys.as_slice() != ["open_clocks", "sealed_at"] {
        return Err("catalog root keys are not exactly open_clocks/sealed_at".to_string());
    }
    let open_clock_values = object
        .get("open_clocks")
        .and_then(JsonValue::as_array)
        .ok_or_else(|| "open_clocks is not an array".to_string())?;
    let mut open_clocks = Vec::with_capacity(open_clock_values.len());
    for value in open_clock_values {
        open_clocks.push(parse_open_clock_row(value)?);
    }
    Ok(OpenClockCatalog {
        open_clocks,
        sealed_at: parse_json_timestamp(
            object
                .get("sealed_at")
                .ok_or_else(|| "sealed_at is missing".to_string())?,
            "sealed_at",
        )?,
    })
}

fn parse_open_clock_row(value: &JsonValue) -> Result<OpenClockCatalogRow, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "open clock row is not an object".to_string())?;
    let keys = object.keys().map(String::as_str).collect::<Vec<_>>();
    if keys.as_slice()
        != [
            "clock_id",
            "clock_kind",
            "computed_deadline",
            "origin_event_hash",
        ]
    {
        return Err(
            "open clock row keys are not exactly clock_id/clock_kind/computed_deadline/origin_event_hash"
                .to_string(),
        );
    }
    let origin_event_hash = parse_lower_hex_string(
        object
            .get("origin_event_hash")
            .and_then(JsonValue::as_str)
            .ok_or_else(|| "origin_event_hash is not a string".to_string())?,
        "origin_event_hash",
    )?;
    let origin_event_hash: [u8; 32] = origin_event_hash
        .as_slice()
        .try_into()
        .map_err(|_| "origin_event_hash must decode to 32 bytes".to_string())?;
    Ok(OpenClockCatalogRow {
        clock_id: object
            .get("clock_id")
            .and_then(JsonValue::as_str)
            .ok_or_else(|| "clock_id is not a string".to_string())?
            .to_string(),
        clock_kind: object
            .get("clock_kind")
            .and_then(JsonValue::as_str)
            .ok_or_else(|| "clock_kind is not a string".to_string())?
            .to_string(),
        computed_deadline: parse_json_timestamp(
            object
                .get("computed_deadline")
                .ok_or_else(|| "computed_deadline is missing".to_string())?,
            "computed_deadline",
        )?,
        origin_event_hash,
    })
}

fn parse_json_timestamp(value: &JsonValue, field: &str) -> Result<TrellisTimestamp, String> {
    let array = value
        .as_array()
        .ok_or_else(|| format!("{field} is not an array"))?;
    if array.len() != 2 {
        return Err(format!("{field} must have exactly two elements"));
    }
    let seconds = array[0]
        .as_u64()
        .ok_or_else(|| format!("{field}.seconds is not a uint"))?;
    let nanos_u64 = array[1]
        .as_u64()
        .ok_or_else(|| format!("{field}.nanos is not a uint"))?;
    let nanos = u32::try_from(nanos_u64).map_err(|_| format!("{field}.nanos out of u32 range"))?;
    if nanos > 999_999_999 {
        return Err(format!("{field}.nanos must be <= 999999999"));
    }
    Ok(TrellisTimestamp { seconds, nanos })
}

fn parse_lower_hex_string(value: &str, field: &str) -> Result<Vec<u8>, String> {
    let decoded = hex_decode(value).map_err(|error| format!("{field} is invalid hex: {error}"))?;
    if hex_string(&decoded) != value {
        return Err(format!("{field} must be lowercase hexadecimal"));
    }
    Ok(decoded)
}

fn validate_order(rows: &[OpenClockCatalogRow]) -> Result<(), String> {
    for pair in rows.windows(2) {
        let left = (&pair[0].origin_event_hash, pair[0].clock_id.as_bytes());
        let right = (&pair[1].origin_event_hash, pair[1].clock_id.as_bytes());
        if left > right {
            return Err(
                "open_clocks rows must be ordered by origin_event_hash then clock_id".to_string(),
            );
        }
    }
    Ok(())
}

fn render_open_clocks_catalog(catalog: &OpenClockCatalog) -> String {
    let mut text = String::new();
    text.push_str("{\"open_clocks\":[");
    for (index, row) in catalog.open_clocks.iter().enumerate() {
        if index > 0 {
            text.push(',');
        }
        text.push_str("{\"clock_id\":");
        push_json_string(&mut text, &row.clock_id);
        text.push_str(",\"clock_kind\":");
        push_json_string(&mut text, &row.clock_kind);
        text.push_str(",\"computed_deadline\":");
        push_timestamp(&mut text, row.computed_deadline);
        text.push_str(",\"origin_event_hash\":\"");
        text.push_str(&hex_string(&row.origin_event_hash));
        text.push_str("\"}");
    }
    text.push_str("],\"sealed_at\":");
    push_timestamp(&mut text, catalog.sealed_at);
    text.push_str("}\n");
    text
}

fn push_json_string(text: &mut String, value: &str) {
    text.push_str(&serde_json::to_string(value).expect("serializing a string cannot fail"));
}

fn push_timestamp(text: &mut String, timestamp: TrellisTimestamp) {
    text.push('[');
    text.push_str(&timestamp.seconds.to_string());
    text.push(',');
    text.push_str(&timestamp.nanos.to_string());
    text.push(']');
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(value: u8) -> [u8; 32] {
        [value; 32]
    }

    #[test]
    fn parse_open_clocks_catalog_accepts_canonical_json() {
        let bytes = br#"{"open_clocks":[{"clock_id":"review:123","clock_kind":"statutory-review","computed_deadline":[10,0],"origin_event_hash":"0101010101010101010101010101010101010101010101010101010101010101"}],"sealed_at":[11,0]}
"#;
        let catalog = parse_open_clocks_catalog(bytes).unwrap();
        assert_eq!(catalog.open_clocks.len(), 1);
        assert_eq!(catalog.open_clocks[0].clock_id, "review:123");
    }

    #[test]
    fn verify_open_clocks_adds_overdue_warning_without_failure() {
        let bytes = br#"{"open_clocks":[{"clock_id":"review:123","clock_kind":"statutory-review","computed_deadline":[10,0],"origin_event_hash":"0101010101010101010101010101010101010101010101010101010101010101"}],"sealed_at":[11,0]}
"#;
        let mut archive = ExportArchive {
            members: Default::default(),
        };
        archive
            .members
            .insert(OPEN_CLOCKS_MEMBER.to_string(), bytes.to_vec());
        let extension = OpenClocksExportExtension {
            open_clocks_digest: sha256_bytes(bytes),
            open_clock_count: 1,
        };
        let mut report = VerificationReport::default();

        verify_open_clocks(
            &archive,
            &extension,
            TrellisTimestamp {
                seconds: 11,
                nanos: 0,
            },
            &mut report,
        );

        assert!(report.event_failures.is_empty());
        assert_eq!(
            report.warnings,
            vec![format!(
                "open_clock_overdue:review:123:{}",
                hex_string(&digest(1))
            )]
        );
    }
}
