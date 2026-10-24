//! CanonicalMeasurementSnapshotV1 projects a deliberately small, read-only measurement surface
//! from already authoritative runtime owners. It owns no metric registry, history, repair,
//! selection, writer, release, MusicXML or assistant-export policy.

use crate::corpus_run_manifest::json_escape;
use crate::model::CheckReport;
use crate::source::SourcePlan;
use crate::source_selection::SourceSelectionFullCausalProjection;
use anyhow::{anyhow, Context};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: u32 = 1;
const MEASUREMENT_CONTRACT_VERSION: &str = "CMS-1";
const PRODUCER_VERSION_ENV: &str = "ETUSCHLOPF_RUNTIME_PRODUCER_VERSION";
const PRODUCER_SOURCE_SIGNATURE_ENV: &str = "ETUSCHLOPF_RUNTIME_PRODUCER_SOURCE_SIGNATURE";
const PRODUCER_BINARY_FINGERPRINT_ENV: &str = "ETUSCHLOPF_RUNTIME_PRODUCER_BINARY_FINGERPRINT";
const PRODUCER_BINDING_STATE_ENV: &str = "ETUSCHLOPF_RUNTIME_PRODUCER_BINDING_STATE";
const OBSERVATION_SESSION_ENV: &str = "ETUSCHLOPF_OBSERVATION_SESSION";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProducerBindingState {
    Bound,
    PartiallyBound,
    Unbound,
}

impl ProducerBindingState {
    fn parse(value: Option<String>) -> Self {
        match value.as_deref().map(str::trim) {
            Some("bound") => Self::Bound,
            Some("partially_bound") => Self::PartiallyBound,
            _ => Self::Unbound,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Bound => "bound",
            Self::PartiallyBound => "partially_bound",
            Self::Unbound => "unbound",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeProducerMeasurementBinding {
    state: ProducerBindingState,
    version: Option<String>,
    source_signature: Option<String>,
    binary_fingerprint: Option<String>,
}

impl RuntimeProducerMeasurementBinding {
    fn from_environment() -> anyhow::Result<Self> {
        let state = ProducerBindingState::parse(std::env::var(PRODUCER_BINDING_STATE_ENV).ok());
        let version = nonempty_env(PRODUCER_VERSION_ENV);
        let source_signature = nonempty_env(PRODUCER_SOURCE_SIGNATURE_ENV);
        let binary_fingerprint = nonempty_env(PRODUCER_BINARY_FINGERPRINT_ENV);

        match state {
            ProducerBindingState::Bound => {
                if version.is_none() || source_signature.is_none() || binary_fingerprint.is_none() {
                    return Err(anyhow!(
                        "CanonicalMeasurementSnapshotV1: producer binding_state=bound ohne vollstaendige Version+SourceSignature+BinaryFingerprint-Bindung"
                    ));
                }
            }
            ProducerBindingState::PartiallyBound => {
                if binary_fingerprint.is_none() {
                    return Err(anyhow!(
                        "CanonicalMeasurementSnapshotV1: producer binding_state=partially_bound ohne BinaryFingerprint"
                    ));
                }
                if version.is_some() || source_signature.is_some() {
                    return Err(anyhow!(
                        "CanonicalMeasurementSnapshotV1: partially_bound darf keine unbewiesene ProducerVersion oder ProducerSourceSignature transportieren"
                    ));
                }
            }
            ProducerBindingState::Unbound => {
                if version.is_some() || source_signature.is_some() || binary_fingerprint.is_some() {
                    return Err(anyhow!(
                        "CanonicalMeasurementSnapshotV1: unbound darf keine Producer-Identitaetsfelder behaupten"
                    ));
                }
            }
        }

        Ok(Self {
            state,
            version,
            source_signature,
            binary_fingerprint,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NativeCounts {
    blockers: usize,
    warnings: usize,
    hints: usize,
    canonical_source_events: usize,
    retained_source_events: usize,
    removed_source_events: usize,
}

impl NativeCounts {
    fn project(
        report: &CheckReport,
        canonical_source_event_count: usize,
        source_selection: &SourceSelectionFullCausalProjection,
    ) -> anyhow::Result<Self> {
        if !source_selection.productive_plan_authority
            || !source_selection.productive_plan_equivalence_verified
        {
            return Err(anyhow!(
                "CanonicalMeasurementSnapshotV1: produktiver Source-Selection-Checkpoint ist nicht authority-bound/equivalence-verified"
            ));
        }
        if source_selection.productive_plan_retained_event_count
            != source_selection.retained_event_count
        {
            return Err(anyhow!(
                "CanonicalMeasurementSnapshotV1: ProductivePlanRetainedEvents {} != retained_event_count {}",
                source_selection.productive_plan_retained_event_count,
                source_selection.retained_event_count
            ));
        }
        if source_selection.canonical_source_event_count != canonical_source_event_count {
            return Err(anyhow!(
                "CanonicalMeasurementSnapshotV1: CanonicalSourceEventProjection {} != SourceSelectionProjection {}",
                canonical_source_event_count,
                source_selection.canonical_source_event_count
            ));
        }
        let retained_plus_removed = source_selection
            .productive_plan_retained_event_count
            .checked_add(source_selection.removed_event_count)
            .ok_or_else(|| anyhow!("CanonicalMeasurementSnapshotV1: Source-Count-Summe ueberlaeuft usize"))?;
        if retained_plus_removed != canonical_source_event_count {
            return Err(anyhow!(
                "CanonicalMeasurementSnapshotV1: CanonicalSourceEvents {} != Retained {} + Removed {}",
                canonical_source_event_count,
                source_selection.productive_plan_retained_event_count,
                source_selection.removed_event_count
            ));
        }

        Ok(Self {
            blockers: report.summary.blockers,
            warnings: report.summary.warnings,
            hints: report.summary.hints,
            canonical_source_events: canonical_source_event_count,
            retained_source_events: source_selection.productive_plan_retained_event_count,
            removed_source_events: source_selection.removed_event_count,
        })
    }
}

pub(crate) fn project_snapshot_path(source: &SourcePlan) -> PathBuf {
    let parent = source.report_out.parent().unwrap_or_else(|| Path::new("."));
    let report_name = source
        .report_out
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("etuschlopf.result.txt");
    let stem = report_name
        .strip_suffix(".result.txt")
        .unwrap_or(report_name);
    parent.join(format!("{stem}.canonical_measurement_snapshot_v1.json"))
}

pub(crate) fn write_project_snapshot(
    source: &SourcePlan,
    report: &CheckReport,
    canonical_source_event_count: usize,
    source_selection: &SourceSelectionFullCausalProjection,
) -> anyhow::Result<PathBuf> {
    let producer = RuntimeProducerMeasurementBinding::from_environment()?;
    let counts = NativeCounts::project(report, canonical_source_event_count, source_selection)?;
    let run_id = nonempty_env(OBSERVATION_SESSION_ENV).unwrap_or_else(|| "none".to_string());
    let json = render_snapshot_json(&source.input_label, &run_id, &producer, counts);
    let target = project_snapshot_path(source);
    let partial = target.with_extension("json.partial");

    match std::fs::remove_file(&partial) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "CanonicalMeasurementSnapshotV1 konnte alte Partial-Datei nicht entfernen: {}",
                    partial.display()
                )
            });
        }
    }
    std::fs::write(&partial, json.as_bytes()).with_context(|| {
        format!(
            "CanonicalMeasurementSnapshotV1 konnte Partial-Datei nicht schreiben: {}",
            partial.display()
        )
    })?;
    match std::fs::remove_file(&target) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "CanonicalMeasurementSnapshotV1 konnte alten Snapshot nicht entfernen: {}",
                    target.display()
                )
            });
        }
    }
    std::fs::rename(&partial, &target).with_context(|| {
        format!(
            "CanonicalMeasurementSnapshotV1 konnte Partial-Datei nicht atomar uebernehmen: {} -> {}",
            partial.display(),
            target.display()
        )
    })?;
    Ok(target)
}

fn render_snapshot_json(
    project: &str,
    run_id: &str,
    producer: &RuntimeProducerMeasurementBinding,
    counts: NativeCounts,
) -> String {
    let mut out = String::new();
    writeln!(out, "{{").expect("String write cannot fail");
    writeln!(out, "  \"schema_version\": {SCHEMA_VERSION},").expect("String write cannot fail");
    writeln!(
        out,
        "  \"measurement_contract_version\": \"{MEASUREMENT_CONTRACT_VERSION}\","
    )
    .expect("String write cannot fail");
    writeln!(out, "  \"producer\": {{").expect("String write cannot fail");
    writeln!(
        out,
        "    \"binding_state\": \"{}\",",
        producer.state.as_str()
    )
    .expect("String write cannot fail");
    push_optional_json_string(&mut out, "version", producer.version.as_deref(), true, 4);
    push_optional_json_string(
        &mut out,
        "source_signature",
        producer.source_signature.as_deref(),
        true,
        4,
    );
    push_optional_json_string(
        &mut out,
        "binary_fingerprint",
        producer.binary_fingerprint.as_deref(),
        false,
        4,
    );
    writeln!(out, "  }},").expect("String write cannot fail");
    writeln!(out, "  \"run\": {{").expect("String write cannot fail");
    writeln!(
        out,
        "    \"run_id\": \"{}\",",
        json_escape(run_id)
    )
    .expect("String write cannot fail");
    writeln!(out, "    \"scope\": \"project\",").expect("String write cannot fail");
    writeln!(out, "    \"full_corpus\": false,").expect("String write cannot fail");
    writeln!(
        out,
        "    \"expected_projects\": [\"{}\"],",
        json_escape(project)
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "    \"measured_projects\": [\"{}\"]",
        json_escape(project)
    )
    .expect("String write cannot fail");
    writeln!(out, "  }},").expect("String write cannot fail");
    writeln!(out, "  \"metrics\": [").expect("String write cannot fail");
    push_metric(
        &mut out,
        "final_findings.blocker.count",
        "FINDING",
        counts.blockers,
        "final_validator",
        true,
    );
    push_metric(
        &mut out,
        "final_findings.warning.count",
        "FINDING",
        counts.warnings,
        "final_validator",
        true,
    );
    push_metric(
        &mut out,
        "final_findings.hint.count",
        "FINDING",
        counts.hints,
        "final_validator",
        true,
    );
    push_metric(
        &mut out,
        "source.canonical_event.count",
        "POPULATION",
        counts.canonical_source_events,
        "canonical_source_event_projection",
        true,
    );
    push_metric(
        &mut out,
        "source.selection.retained.count",
        "OUTCOME",
        counts.retained_source_events,
        "productive_source_selection_plan_bound",
        true,
    );
    push_metric(
        &mut out,
        "source.selection.removed.count",
        "OUTCOME",
        counts.removed_source_events,
        "productive_source_selection_plan_bound",
        false,
    );
    writeln!(out, "  ]").expect("String write cannot fail");
    writeln!(out, "}}").expect("String write cannot fail");
    out
}

fn push_metric(
    out: &mut String,
    metric_id: &str,
    count_class: &str,
    count: usize,
    checkpoint: &str,
    comma: bool,
) {
    writeln!(out, "    {{").expect("String write cannot fail");
    writeln!(out, "      \"metric_id\": \"{metric_id}\",").expect("String write cannot fail");
    writeln!(out, "      \"count_class\": \"{count_class}\",").expect("String write cannot fail");
    writeln!(out, "      \"measurement_state\": \"measured\",").expect("String write cannot fail");
    writeln!(out, "      \"count\": {count},").expect("String write cannot fail");
    writeln!(out, "      \"checkpoint\": \"{checkpoint}\",").expect("String write cannot fail");
    writeln!(out, "      \"scope\": \"project\",").expect("String write cannot fail");
    writeln!(out, "      \"aggregation\": \"additive_disjoint_project_reports\"").expect("String write cannot fail");
    writeln!(out, "    }}{}", if comma { "," } else { "" }).expect("String write cannot fail");
}

fn push_optional_json_string(
    out: &mut String,
    name: &str,
    value: Option<&str>,
    comma: bool,
    indent: usize,
) {
    let padding = " ".repeat(indent);
    match value {
        Some(value) => writeln!(
            out,
            "{padding}\"{name}\": \"{}\"{}",
            json_escape(value),
            if comma { "," } else { "" }
        ),
        None => writeln!(
            out,
            "{padding}\"{name}\": null{}",
            if comma { "," } else { "" }
        ),
    }
    .expect("String write cannot fail");
}

fn nonempty_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}


#[cfg(test)]
mod tests {
    use super::{
        json_escape, render_snapshot_json, NativeCounts, ProducerBindingState,
        RuntimeProducerMeasurementBinding,
    };

    #[test]
    fn project_snapshot_has_exact_six_metric_ids_and_no_physical_source() {
        let producer = RuntimeProducerMeasurementBinding {
            state: ProducerBindingState::Bound,
            version: Some("0.10.1839".to_string()),
            source_signature: Some("fnv64:source".to_string()),
            binary_fingerprint: Some("fnv64:binary".to_string()),
        };
        let json = render_snapshot_json(
            "Synthetic Project Nr. 6",
            "obs-test",
            &producer,
            NativeCounts {
                blockers: 0,
                warnings: 0,
                hints: 0,
                canonical_source_events: 101,
                retained_source_events: 100,
                removed_source_events: 1,
            },
        );

        assert_eq!(json.matches("\"metric_id\"").count(), 6);
        assert!(json.contains("\"binding_state\": \"bound\""));
        assert!(json.contains("\"count\": 0"));
        assert!(json.contains("\"count\": 101"));
        assert!(json.contains("\"count\": 100"));
        assert!(json.contains("\"count\": 1"));
        assert!(!json.contains("physical_source"));
        assert!(!json.contains("revision"));
        assert!(!json.contains("retention_percent"));
    }

    #[test]
    fn producer_binding_state_parser_is_fail_closed() {
        assert_eq!(
            ProducerBindingState::parse(Some("bound".to_string())),
            ProducerBindingState::Bound
        );
        assert_eq!(
            ProducerBindingState::parse(Some("partially_bound".to_string())),
            ProducerBindingState::PartiallyBound
        );
        assert_eq!(
            ProducerBindingState::parse(Some("unexpected".to_string())),
            ProducerBindingState::Unbound
        );
        assert_eq!(
            ProducerBindingState::parse(None),
            ProducerBindingState::Unbound
        );
    }

    #[test]
    fn json_escape_preserves_project_names_without_raw_controls() {
        assert_eq!(json_escape("Synthetic Project \"6\"\nX"), "Synthetic Project \\\"6\\\"\\nX");
    }
}
