//! Read-only temporal precondition at the former #56 writer boundary.
//! Causal class: docs/selector_derivation_contract_cursor_temporal_precondition_r1445.md.
//! A successful proof binds immutable original bytes and grants no writer authority.
use crate::audit_only::serialized_temporal_replay::observe_serialized_temporal_xml;

#[derive(Debug, Clone, Default)]
pub(crate) struct CursorLaneTemporalCheckAudit {
    pub(crate) source: String,
    pub(crate) input_sha256: String,
    pub(crate) checked_measures: usize,
    pub(crate) temporal_violation_measures: usize,
    pub(crate) unsupported_inputs: usize,
    pub(crate) partial_serializations_observed: usize,
}

impl std::fmt::Display for CursorLaneTemporalCheckAudit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CursorLaneTemporalCheck v1 {}: checked {}, temporalViolations {}, unsupported {}, partialSerializations {}; inputSha256 {}; writes 0",
            self.source, self.checked_measures, self.temporal_violation_measures,
            self.unsupported_inputs, self.partial_serializations_observed, self.input_sha256)
    }
}

#[derive(Debug)]
pub(crate) struct CursorLaneTemporalCheckFailure {
    pub(crate) audit: CursorLaneTemporalCheckAudit,
    reason: String,
}
impl std::fmt::Display for CursorLaneTemporalCheckFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}; rejected: {}", self.audit, self.reason)
    }
}
impl std::error::Error for CursorLaneTemporalCheckFailure {}

#[derive(Debug)]
pub(crate) struct CheckedCursorTemporalInput<'a> {
    original: &'a str,
    sha256: String,
    audit: CursorLaneTemporalCheckAudit,
}
impl CheckedCursorTemporalInput<'_> {
    pub(crate) fn audit(&self) -> &CursorLaneTemporalCheckAudit { &self.audit }
    pub(crate) fn binds(&self, document: &str) -> bool {
        self.original == document && self.sha256 == crate::sha256::hex(document.as_bytes())
    }
}

/// Coverage is deliberately explicit: one part, explicit initial positive simple
/// meter/divisions, inherited context thereafter, integer durations and no mid-
/// measure context changes. Unsupported temporal grammars fail before replay.
fn supported_temporal_coverage(document: &roxmltree::Document<'_>) -> bool {
    let root = document.root_element();
    if root.tag_name().name() != "score-partwise" { return false; }
    let parts = root.children().filter(|n| n.has_tag_name("part")).collect::<Vec<_>>();
    if parts.len() != 1 { return false; }
    let measures = parts[0].children().filter(|n| n.has_tag_name("measure")).collect::<Vec<_>>();
    if measures.is_empty() { return false; }
    let mut divisions = None::<i64>;
    let mut beats = None::<i64>;
    let mut beat_type = None::<i64>;
    for measure in measures {
        let mut saw_event = false;
        let mut attributes_seen = false;
        for event in measure.children().filter(roxmltree::Node::is_element) {
            if event.has_tag_name("attributes") {
                let changes_time = event.children().any(|n| n.has_tag_name("divisions") || n.has_tag_name("time"));
                if !changes_time { continue; }
                if attributes_seen || saw_event { return false; }
                attributes_seen = true;
                let nodes = event.children().filter(|n| n.has_tag_name("divisions")).collect::<Vec<_>>();
                if nodes.len() > 1 { return false; }
                if let Some(node) = nodes.first() {
                    let value = node.text().and_then(|s| s.trim().parse::<i64>().ok()).filter(|v| *v > 0);
                    if value.is_none() { return false; }
                    divisions = value;
                }
                let times = event.children().filter(|n| n.has_tag_name("time")).collect::<Vec<_>>();
                if times.len() > 1 { return false; }
                if let Some(time) = times.first() {
                    let values = time.children().filter(roxmltree::Node::is_element).collect::<Vec<_>>();
                    if values.len() != 2 || !values[0].has_tag_name("beats") || !values[1].has_tag_name("beat-type") { return false; }
                    beats = values[0].text().and_then(|s| s.trim().parse::<i64>().ok()).filter(|v| *v > 0);
                    beat_type = values[1].text().and_then(|s| s.trim().parse::<i64>().ok()).filter(|v| *v > 0);
                }
            } else if event.has_tag_name("note") || event.has_tag_name("backup") || event.has_tag_name("forward") {
                saw_event = true;
                let durations = event.children().filter(|n| n.has_tag_name("duration")).collect::<Vec<_>>();
                let grace = event.has_tag_name("note") && event.children().any(|n| n.has_tag_name("grace"));
                if grace && durations.is_empty() { continue; }
                if durations.len() != 1 || durations[0].text().and_then(|s| s.trim().parse::<i64>().ok()).is_none_or(|v| v <= 0) { return false; }
            }
        }
        let (Some(d), Some(b), Some(t)) = (divisions, beats, beat_type) else { return false; };
        let Some(numerator) = d.checked_mul(b).and_then(|v| v.checked_mul(4)) else { return false; };
        if numerator % t != 0 || numerator / t <= 0 { return false; }
    }
    true
}

pub(crate) fn check_cursor_lane_temporal_precondition<'a>(source: &str, xml: &'a str)
    -> Result<CheckedCursorTemporalInput<'a>, CursorLaneTemporalCheckFailure>
{
    let mut audit = CursorLaneTemporalCheckAudit { source: source.to_owned(), input_sha256: crate::sha256::hex(xml.as_bytes()), ..Default::default() };
    let parsed = roxmltree::Document::parse(xml);
    let reason = match &parsed {
        Err(_) => Some("unparseable_xml"),
        Ok(document) if !supported_temporal_coverage(document) => Some("unsupported_temporal_coverage"),
        Ok(_) => None,
    };
    if let Some(reason) = reason {
        audit.unsupported_inputs = 1;
        return Err(CursorLaneTemporalCheckFailure { audit, reason: reason.to_owned() });
    }
    let observation = observe_serialized_temporal_xml(source, xml);
    if !observation.parse_ok || !observation.score_partwise || observation.rows.is_empty() {
        audit.unsupported_inputs = 1;
        return Err(CursorLaneTemporalCheckFailure { audit, reason: "missing_replay_coverage".to_owned() });
    }
    audit.checked_measures = observation.rows.len();
    audit.partial_serializations_observed = observation.rows.iter().map(|r| r.partial_backup_count).sum();
    audit.temporal_violation_measures = observation.rows.iter().filter(|r| r.temporal_violation).count();
    if audit.temporal_violation_measures > 0 {
        let reasons = observation.rows.iter().filter(|r| r.temporal_violation)
            .map(|r| format!("measure {}: {}", r.measure, r.first_bad.split('+').filter(|reason| *reason != "BackupToNonzeroLegacy").collect::<Vec<_>>().join("+"))).collect::<Vec<_>>().join("; ");
        return Err(CursorLaneTemporalCheckFailure { audit, reason: reasons });
    }
    Ok(CheckedCursorTemporalInput { original: xml, sha256: crate::sha256::hex(xml.as_bytes()), audit })
}
