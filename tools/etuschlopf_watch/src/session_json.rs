use crate::observation_protocol::{
    ObservedSourceSnapshotV2, SessionSnapshotV2, SnapshotActivityV2, SnapshotOutcomeColorV2,
    SnapshotProgressV2, SnapshotSourceRoleV2, SnapshotSourceStatusV2, SnapshotStageV2,
    SourceSnapshotV2,
    SESSION_SNAPSHOT_PROTOCOL_VERSION, SOURCE_SNAPSHOT_PROTOCOL_VERSION,
};
use std::collections::BTreeMap;

const SESSION_TOPIC_PREFIX: &str = "etuschlopf/v2/session/";
const SESSION_TOPIC_SUFFIX: &str = "/state";

pub fn mqtt_topic_for_session(session_id: &str) -> Option<String> {
    if session_id.is_empty()
        || session_id
            .bytes()
            .any(|byte| matches!(byte, 0 | b'/' | b'+' | b'#'))
    {
        return None;
    }
    Some(format!(
        "{SESSION_TOPIC_PREFIX}{session_id}{SESSION_TOPIC_SUFFIX}"
    ))
}

pub fn encode_session_snapshot_json_v2(snapshot: &SessionSnapshotV2) -> String {
    let mut json = String::with_capacity(256 + snapshot.sources.len().saturating_mul(192));
    json.push('{');
    push_u64_field(&mut json, "protocol_version", u64::from(snapshot.protocol_version));
    json.push(',');
    push_string_field(
        &mut json,
        "observation_session_id",
        &snapshot.observation_session_id,
    );
    json.push_str(",\"sources\":[");
    for (index, source) in snapshot.sources.iter().enumerate() {
        if index != 0 {
            json.push(',');
        }
        push_observed_source(&mut json, source);
    }
    json.push_str("]}");
    json
}

fn push_observed_source(json: &mut String, observed: &ObservedSourceSnapshotV2) {
    json.push_str("{\"source\":");
    push_source(json, &observed.source);
    json.push(',');
    push_u64_field(json, "last_seen_age_millis", observed.last_seen_age_millis);
    json.push(',');
    push_u64_field(
        json,
        "last_state_change_age_millis",
        observed.last_state_change_age_millis,
    );
    json.push(',');
    push_u64_field(json, "run_elapsed_millis", observed.run_elapsed_millis);
    json.push('}');
}

fn push_source(json: &mut String, source: &SourceSnapshotV2) {
    json.push('{');
    push_u64_field(json, "protocol_version", u64::from(source.protocol_version));
    json.push(',');
    push_string_field(
        json,
        "observation_session_id",
        &source.observation_session_id,
    );
    json.push(',');
    push_string_field(json, "source_id", &source.source_id);
    json.push(',');
    push_optional_string_field(json, "parent_source_id", source.parent_source_id.as_deref());
    json.push(',');
    push_string_field(json, "source_role", source_role_label(source.source_role));
    json.push(',');
    push_u64_field(json, "revision", source.revision);
    json.push(',');
    push_string_field(json, "status", source_status_label(source.status));
    json.push(',');
    push_string_field(json, "stage", stage_label(source.stage));
    json.push(',');
    match source.activity {
        Some(activity) => push_string_field(json, "activity", activity_label(activity)),
        None => json.push_str("\"activity\":null"),
    }
    json.push(',');
    push_optional_string_field(json, "project_label", source.project_label.as_deref());
    json.push_str(",\"progress\":");
    match source.progress {
        Some(progress) => {
            json.push('{');
            push_u64_field(json, "completed", progress.completed);
            json.push(',');
            push_u64_field(json, "total", progress.total);
            json.push(',');
            push_u64_field(json, "active", progress.active);
            json.push('}');
        }
        None => json.push_str("null"),
    }
    json.push(',');
    push_u64_field(json, "started_unix_millis", source.started_unix_millis);
    json.push_str(",\"terminal_duration_millis\":");
    match source.terminal_duration_millis {
        Some(value) => json.push_str(&value.to_string()),
        None => json.push_str("null"),
    }
    json.push_str(",\"outcome_color\":");
    match source.outcome_color {
        Some(color) => push_json_string(json, outcome_color_label(color)),
        None => json.push_str("null"),
    }
    json.push('}');
}

fn push_u64_field(json: &mut String, name: &str, value: u64) {
    push_json_string(json, name);
    json.push(':');
    json.push_str(&value.to_string());
}

fn push_string_field(json: &mut String, name: &str, value: &str) {
    push_json_string(json, name);
    json.push(':');
    push_json_string(json, value);
}

fn push_optional_string_field(json: &mut String, name: &str, value: Option<&str>) {
    push_json_string(json, name);
    json.push(':');
    match value {
        Some(value) => push_json_string(json, value),
        None => json.push_str("null"),
    }
}

fn push_json_string(json: &mut String, value: &str) {
    json.push('"');
    for ch in value.chars() {
        match ch {
            '"' => json.push_str("\\\""),
            '\\' => json.push_str("\\\\"),
            '\n' => json.push_str("\\n"),
            '\r' => json.push_str("\\r"),
            '\t' => json.push_str("\\t"),
            ch if ch <= '\u{001f}' => {
                json.push_str(&format!("\\u{:04x}", u32::from(ch)));
            }
            ch => json.push(ch),
        }
    }
    json.push('"');
}

const fn source_role_label(role: SnapshotSourceRoleV2) -> &'static str {
    match role {
        SnapshotSourceRoleV2::Launcher => "launcher",
        SnapshotSourceRoleV2::Corpus => "corpus",
        SnapshotSourceRoleV2::Project => "project",
    }
}

const fn source_status_label(status: SnapshotSourceStatusV2) -> &'static str {
    match status {
        SnapshotSourceStatusV2::Running => "running",
        SnapshotSourceStatusV2::Completed => "completed",
        SnapshotSourceStatusV2::Failed => "failed",
        SnapshotSourceStatusV2::Cancelled => "cancelled",
    }
}

const fn outcome_color_label(color: SnapshotOutcomeColorV2) -> &'static str {
    match color {
        SnapshotOutcomeColorV2::DarkGreen => "dark_green",
        SnapshotOutcomeColorV2::LightGreen => "light_green",
        SnapshotOutcomeColorV2::Yellow => "yellow",
        SnapshotOutcomeColorV2::Red => "red",
    }
}

const fn activity_label(activity: SnapshotActivityV2) -> &'static str {
    match activity {
        SnapshotActivityV2::ClefPlanning => "clef_planning",
        SnapshotActivityV2::BeamPlanning => "beam_planning",
    }
}

const fn stage_label(stage: SnapshotStageV2) -> &'static str {
    match stage {
        SnapshotStageV2::LauncherPreparing => "launcher_preparing",
        SnapshotStageV2::LauncherBuilding => "launcher_building",
        SnapshotStageV2::LauncherTests => "launcher_tests",
        SnapshotStageV2::LauncherRunningTarget => "launcher_running_target",
        SnapshotStageV2::LauncherFinalizing => "launcher_finalizing",
        SnapshotStageV2::ProjectRunPreparation => "project_run_preparation",
        SnapshotStageV2::ProjectArrangementSolver => "project_arrangement_solver",
        SnapshotStageV2::ProjectMasterNotationAndSalience => {
            "project_master_notation_and_salience"
        }
        SnapshotStageV2::ProjectInitialRenderTransaction => {
            "project_initial_render_transaction"
        }
        SnapshotStageV2::ProjectMetricBeamAndFinalNotation => {
            "project_metric_beam_and_final_notation"
        }
        SnapshotStageV2::ProjectReleaseReportAndExit => "project_release_report_and_exit",
        SnapshotStageV2::CorpusPlanning => "corpus_planning",
        SnapshotStageV2::CorpusPreparingProjects => "corpus_preparing_projects",
        SnapshotStageV2::CorpusRunningProjects => "corpus_running_projects",
        SnapshotStageV2::CorpusCollectingProjectResult => {
            "corpus_collecting_project_result"
        }
        SnapshotStageV2::CorpusFinalizingCorpusReport => "corpus_finalizing_corpus_report",
    }
}


pub fn decode_session_snapshot_json_v2(bytes: &[u8]) -> Result<SessionSnapshotV2, String> {
    let value = JsonParser::new(bytes).parse_document()?;
    let root = expect_object(&value, "session root")?;
    let protocol_version = expect_u16(field(root, "protocol_version")?, "protocol_version")?;
    if protocol_version != SESSION_SNAPSHOT_PROTOCOL_VERSION {
        return Err(format!("unsupported SessionSnapshotV2 protocol {protocol_version}"));
    }
    let observation_session_id = expect_string(field(root, "observation_session_id")?, "observation_session_id")?.to_string();
    let source_values = expect_array(field(root, "sources")?, "sources")?;
    let mut sources = Vec::with_capacity(source_values.len());
    for value in source_values {
        let observed = expect_object(value, "observed source")?;
        let source = decode_source(field(observed, "source")?)?;
        if source.observation_session_id != observation_session_id {
            return Err("source observation_session_id differs from session".to_string());
        }
        let last_seen_age_millis =
            expect_u64(field(observed, "last_seen_age_millis")?, "last_seen_age_millis")?;
        let last_state_change_age_millis = expect_u64(
            field(observed, "last_state_change_age_millis")?,
            "last_state_change_age_millis",
        )?;
        let run_elapsed_millis = optional_u64(observed, "run_elapsed_millis")?
            .unwrap_or(last_state_change_age_millis);
        sources.push(ObservedSourceSnapshotV2 {
            source,
            last_seen_age_millis,
            last_state_change_age_millis,
            run_elapsed_millis,
        });
    }
    Ok(SessionSnapshotV2 {
        protocol_version,
        observation_session_id,
        sources,
    })
}

fn optional_u64(object: &BTreeMap<String, JsonValue>, name: &str) -> Result<Option<u64>, String> {
    match object.get(name) {
        Some(value) => expect_u64(value, name).map(Some),
        None => Ok(None),
    }
}

fn decode_source(value: &JsonValue) -> Result<SourceSnapshotV2, String> {
    let source = expect_object(value, "source")?;
    let protocol_version = expect_u16(field(source, "protocol_version")?, "source.protocol_version")?;
    if protocol_version != SOURCE_SNAPSHOT_PROTOCOL_VERSION {
        return Err(format!("unsupported SourceSnapshotV2 protocol {protocol_version}"));
    }
    Ok(SourceSnapshotV2 {
        protocol_version,
        observation_session_id: expect_string(
            field(source, "observation_session_id")?,
            "source.observation_session_id",
        )?
        .to_string(),
        source_id: expect_string(field(source, "source_id")?, "source.source_id")?.to_string(),
        parent_source_id: expect_optional_string(
            field(source, "parent_source_id")?,
            "source.parent_source_id",
        )?,
        source_role: decode_role(expect_string(field(source, "source_role")?, "source.source_role")?)?,
        revision: expect_u64(field(source, "revision")?, "source.revision")?,
        status: decode_status(expect_string(field(source, "status")?, "source.status")?)?,
        stage: decode_stage(expect_string(field(source, "stage")?, "source.stage")?)?,
        activity: decode_activity(field(source, "activity")?)?,
        project_label: expect_optional_string(field(source, "project_label")?, "source.project_label")?,
        progress: decode_progress(field(source, "progress")?)?,
        started_unix_millis: expect_u64(
            field(source, "started_unix_millis")?,
            "source.started_unix_millis",
        )?,
        terminal_duration_millis: decode_optional_u64_value(
            field(source, "terminal_duration_millis")?,
            "source.terminal_duration_millis",
        )?,
        outcome_color: decode_outcome_color(field(source, "outcome_color")?)?,
    })
}

fn decode_optional_u64_value(value: &JsonValue, name: &str) -> Result<Option<u64>, String> {
    match value {
        JsonValue::Null => Ok(None),
        _ => expect_u64(value, name).map(Some),
    }
}

fn decode_outcome_color(value: &JsonValue) -> Result<Option<SnapshotOutcomeColorV2>, String> {
    match value {
        JsonValue::Null => Ok(None),
        JsonValue::String(value) => match value.as_str() {
            "dark_green" => Ok(Some(SnapshotOutcomeColorV2::DarkGreen)),
            "light_green" => Ok(Some(SnapshotOutcomeColorV2::LightGreen)),
            "yellow" => Ok(Some(SnapshotOutcomeColorV2::Yellow)),
            "red" => Ok(Some(SnapshotOutcomeColorV2::Red)),
            _ => Err(format!("unknown outcome_color {value}")),
        },
        _ => Err("source.outcome_color must be string or null".to_string()),
    }
}

fn decode_progress(value: &JsonValue) -> Result<Option<SnapshotProgressV2>, String> {
    if matches!(value, JsonValue::Null) {
        return Ok(None);
    }
    let progress = expect_object(value, "source.progress")?;
    Ok(Some(SnapshotProgressV2 {
        completed: expect_u64(field(progress, "completed")?, "source.progress.completed")?,
        total: expect_u64(field(progress, "total")?, "source.progress.total")?,
        active: expect_u64(field(progress, "active")?, "source.progress.active")?,
    }))
}

fn decode_role(value: &str) -> Result<SnapshotSourceRoleV2, String> {
    match value {
        "launcher" => Ok(SnapshotSourceRoleV2::Launcher),
        "corpus" => Ok(SnapshotSourceRoleV2::Corpus),
        "project" => Ok(SnapshotSourceRoleV2::Project),
        _ => Err(format!("unknown source_role {value}")),
    }
}

fn decode_status(value: &str) -> Result<SnapshotSourceStatusV2, String> {
    match value {
        "running" => Ok(SnapshotSourceStatusV2::Running),
        "completed" => Ok(SnapshotSourceStatusV2::Completed),
        "failed" => Ok(SnapshotSourceStatusV2::Failed),
        "cancelled" => Ok(SnapshotSourceStatusV2::Cancelled),
        _ => Err(format!("unknown source status {value}")),
    }
}

fn decode_activity(value: &JsonValue) -> Result<Option<SnapshotActivityV2>, String> {
    match value {
        JsonValue::Null => Ok(None),
        JsonValue::String(value) => match value.as_str() {
            "clef_planning" => Ok(Some(SnapshotActivityV2::ClefPlanning)),
            "beam_planning" => Ok(Some(SnapshotActivityV2::BeamPlanning)),
            _ => Err(format!("unknown activity {value}")),
        },
        _ => Err("source.activity must be string or null".to_string()),
    }
}

fn decode_stage(value: &str) -> Result<SnapshotStageV2, String> {
    match value {
        "launcher_preparing" => Ok(SnapshotStageV2::LauncherPreparing),
        "launcher_building" => Ok(SnapshotStageV2::LauncherBuilding),
        "launcher_tests" => Ok(SnapshotStageV2::LauncherTests),
        "launcher_running_target" => Ok(SnapshotStageV2::LauncherRunningTarget),
        "launcher_finalizing" => Ok(SnapshotStageV2::LauncherFinalizing),
        "project_run_preparation" => Ok(SnapshotStageV2::ProjectRunPreparation),
        "project_arrangement_solver" => Ok(SnapshotStageV2::ProjectArrangementSolver),
        "project_master_notation_and_salience" => Ok(SnapshotStageV2::ProjectMasterNotationAndSalience),
        "project_initial_render_transaction" => Ok(SnapshotStageV2::ProjectInitialRenderTransaction),
        "project_metric_beam_and_final_notation" => Ok(SnapshotStageV2::ProjectMetricBeamAndFinalNotation),
        "project_release_report_and_exit" => Ok(SnapshotStageV2::ProjectReleaseReportAndExit),
        "corpus_planning" => Ok(SnapshotStageV2::CorpusPlanning),
        "corpus_preparing_projects" => Ok(SnapshotStageV2::CorpusPreparingProjects),
        "corpus_running_projects" => Ok(SnapshotStageV2::CorpusRunningProjects),
        "corpus_collecting_project_result" => Ok(SnapshotStageV2::CorpusCollectingProjectResult),
        "corpus_finalizing_corpus_report" => Ok(SnapshotStageV2::CorpusFinalizingCorpusReport),
        _ => Err(format!("unknown stage {value}")),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum JsonValue {
    Null,
    Bool,
    Number(u64),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

struct JsonParser<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> JsonParser<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn parse_document(mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        let value = self.parse_value()?;
        self.skip_whitespace();
        if self.cursor != self.bytes.len() {
            return Err("trailing JSON bytes".to_string());
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => self.parse_string().map(JsonValue::String),
            Some(b'n') => {
                self.expect_literal(b"null")?;
                Ok(JsonValue::Null)
            }
            Some(b't') => {
                self.expect_literal(b"true")?;
                Ok(JsonValue::Bool)
            }
            Some(b'f') => {
                self.expect_literal(b"false")?;
                Ok(JsonValue::Bool)
            }
            Some(b'0'..=b'9') => self.parse_number().map(JsonValue::Number),
            _ => Err("invalid JSON value".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.expect_byte(b'{')?;
        self.skip_whitespace();
        let mut object = BTreeMap::new();
        if self.consume_byte(b'}') {
            return Ok(JsonValue::Object(object));
        }
        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect_byte(b':')?;
            let value = self.parse_value()?;
            if object.insert(key, value).is_some() {
                return Err("duplicate JSON object field".to_string());
            }
            self.skip_whitespace();
            if self.consume_byte(b'}') {
                break;
            }
            self.expect_byte(b',')?;
        }
        Ok(JsonValue::Object(object))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.expect_byte(b'[')?;
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.consume_byte(b']') {
            return Ok(JsonValue::Array(values));
        }
        loop {
            values.push(self.parse_value()?);
            self.skip_whitespace();
            if self.consume_byte(b']') {
                break;
            }
            self.expect_byte(b',')?;
        }
        Ok(JsonValue::Array(values))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.expect_byte(b'"')?;
        let mut text = String::new();
        loop {
            let byte = self.next().ok_or_else(|| "truncated JSON string".to_string())?;
            match byte {
                b'"' => return Ok(text),
                b'\\' => {
                    let escaped = self.next().ok_or_else(|| "truncated JSON escape".to_string())?;
                    match escaped {
                        b'"' => text.push('"'),
                        b'\\' => text.push('\\'),
                        b'/' => text.push('/'),
                        b'b' => text.push('\u{0008}'),
                        b'f' => text.push('\u{000c}'),
                        b'n' => text.push('\n'),
                        b'r' => text.push('\r'),
                        b't' => text.push('\t'),
                        b'u' => {
                            let value = self.parse_hex4()?;
                            let ch = char::from_u32(u32::from(value))
                                .ok_or_else(|| "invalid JSON unicode escape".to_string())?;
                            text.push(ch);
                        }
                        _ => return Err("invalid JSON escape".to_string()),
                    }
                }
                0x00..=0x1f => return Err("control byte in JSON string".to_string()),
                0x20..=0x7f => text.push(char::from(byte)),
                _ => {
                    let start = self.cursor.saturating_sub(1);
                    let width = utf8_width(byte).ok_or_else(|| "invalid JSON UTF-8".to_string())?;
                    let end = start.saturating_add(width);
                    let bytes = self
                        .bytes
                        .get(start..end)
                        .ok_or_else(|| "truncated JSON UTF-8".to_string())?;
                    let chunk = std::str::from_utf8(bytes)
                        .map_err(|_| "invalid JSON UTF-8".to_string())?;
                    text.push_str(chunk);
                    self.cursor = end;
                }
            }
        }
    }

    fn parse_hex4(&mut self) -> Result<u16, String> {
        let bytes = self
            .bytes
            .get(self.cursor..self.cursor.saturating_add(4))
            .ok_or_else(|| "truncated JSON unicode escape".to_string())?;
        self.cursor = self.cursor.saturating_add(4);
        let text = std::str::from_utf8(bytes).map_err(|_| "invalid JSON unicode escape".to_string())?;
        u16::from_str_radix(text, 16).map_err(|_| "invalid JSON unicode escape".to_string())
    }

    fn parse_number(&mut self) -> Result<u64, String> {
        let start = self.cursor;
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            self.cursor = self.cursor.saturating_add(1);
        }
        let bytes = self
            .bytes
            .get(start..self.cursor)
            .ok_or_else(|| "invalid JSON number".to_string())?;
        let text = std::str::from_utf8(bytes).map_err(|_| "invalid JSON number".to_string())?;
        text.parse::<u64>().map_err(|_| "invalid JSON number".to_string())
    }

    fn expect_literal(&mut self, literal: &[u8]) -> Result<(), String> {
        let end = self.cursor.saturating_add(literal.len());
        if self.bytes.get(self.cursor..end) != Some(literal) {
            return Err("invalid JSON literal".to_string());
        }
        self.cursor = end;
        Ok(())
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), String> {
        if self.consume_byte(expected) {
            Ok(())
        } else {
            Err(format!("expected JSON byte {}", char::from(expected)))
        }
    }

    fn consume_byte(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.cursor = self.cursor.saturating_add(1);
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while self
            .peek()
            .is_some_and(|byte| matches!(byte, b' ' | b'\n' | b'\r' | b'\t'))
        {
            self.cursor = self.cursor.saturating_add(1);
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.cursor).copied()
    }

    fn next(&mut self) -> Option<u8> {
        let value = self.peek()?;
        self.cursor = self.cursor.saturating_add(1);
        Some(value)
    }
}

const fn utf8_width(first: u8) -> Option<usize> {
    match first {
        0xc2..=0xdf => Some(2),
        0xe0..=0xef => Some(3),
        0xf0..=0xf4 => Some(4),
        _ => None,
    }
}

fn field<'a>(object: &'a BTreeMap<String, JsonValue>, name: &str) -> Result<&'a JsonValue, String> {
    object.get(name).ok_or_else(|| format!("missing JSON field {name}"))
}

fn expect_object<'a>(value: &'a JsonValue, name: &str) -> Result<&'a BTreeMap<String, JsonValue>, String> {
    match value {
        JsonValue::Object(value) => Ok(value),
        _ => Err(format!("{name} must be JSON object")),
    }
}

fn expect_array<'a>(value: &'a JsonValue, name: &str) -> Result<&'a [JsonValue], String> {
    match value {
        JsonValue::Array(value) => Ok(value),
        _ => Err(format!("{name} must be JSON array")),
    }
}

fn expect_string<'a>(value: &'a JsonValue, name: &str) -> Result<&'a str, String> {
    match value {
        JsonValue::String(value) => Ok(value),
        _ => Err(format!("{name} must be JSON string")),
    }
}

fn expect_optional_string(value: &JsonValue, name: &str) -> Result<Option<String>, String> {
    match value {
        JsonValue::Null => Ok(None),
        JsonValue::String(value) => Ok(Some(value.clone())),
        _ => Err(format!("{name} must be JSON string or null")),
    }
}

fn expect_u64(value: &JsonValue, name: &str) -> Result<u64, String> {
    match value {
        JsonValue::Number(value) => Ok(*value),
        _ => Err(format!("{name} must be JSON unsigned integer")),
    }
}

fn expect_u16(value: &JsonValue, name: &str) -> Result<u16, String> {
    let value = expect_u64(value, name)?;
    u16::try_from(value).map_err(|_| format!("{name} exceeds u16"))
}

#[cfg(test)]
mod tests {
    use super::{decode_session_snapshot_json_v2, encode_session_snapshot_json_v2, mqtt_topic_for_session};
    use crate::observation_protocol::{
        ObservedSourceSnapshotV2, SessionSnapshotV2, SnapshotOutcomeColorV2, SnapshotSourceRoleV2,
        SnapshotSourceStatusV2, SnapshotStageV2, SourceSnapshotV2,
        SESSION_SNAPSHOT_PROTOCOL_VERSION, SOURCE_SNAPSHOT_PROTOCOL_VERSION,
    };

    #[test]
    fn topic_is_session_scoped_and_rejects_mqtt_wildcards() {
        assert_eq!(
            mqtt_topic_for_session("obs-1").as_deref(),
            Some("etuschlopf/v2/session/obs-1/state")
        );
        assert!(mqtt_topic_for_session("bad/session").is_none());
        assert!(mqtt_topic_for_session("bad+session").is_none());
        assert!(mqtt_topic_for_session("bad#session").is_none());
    }

    #[test]
    fn session_snapshot_json_is_deterministic_and_escaped() {
        let snapshot = SessionSnapshotV2 {
            protocol_version: SESSION_SNAPSHOT_PROTOCOL_VERSION,
            observation_session_id: "obs-1".to_string(),
            sources: vec![ObservedSourceSnapshotV2 {
                source: SourceSnapshotV2 {
                    protocol_version: SOURCE_SNAPSHOT_PROTOCOL_VERSION,
                    observation_session_id: "obs-1".to_string(),
                    source_id: "project:1".to_string(),
                    parent_source_id: Some("corpus".to_string()),
                    source_role: SnapshotSourceRoleV2::Project,
                    revision: 3,
                    status: SnapshotSourceStatusV2::Running,
                    stage: SnapshotStageV2::ProjectInitialRenderTransaction,
                    activity: None,
                    project_label: Some("Etüde \"A\"".to_string()),
                    progress: None,
                    started_unix_millis: 1_000,
                    terminal_duration_millis: Some(12),
                    outcome_color: Some(SnapshotOutcomeColorV2::DarkGreen),
                },
                last_seen_age_millis: 4,
                last_state_change_age_millis: 9,
                run_elapsed_millis: 12,
            }],
        };

        let json = encode_session_snapshot_json_v2(&snapshot);
        assert!(json.starts_with("{\"protocol_version\":2,\"observation_session_id\":\"obs-1\""));
        assert!(json.contains("\"revision\":3"));
        assert!(json.contains("\"project_label\":\"Etüde \\\"A\\\"\""));
        assert!(json.contains("\"last_seen_age_millis\":4"));
        assert!(json.contains("\"last_state_change_age_millis\":9"));
        assert!(json.contains("\"run_elapsed_millis\":12"));
        assert!(json.contains("\"started_unix_millis\":1000"));
        assert!(json.contains("\"terminal_duration_millis\":12"));
        assert!(json.contains("\"outcome_color\":\"dark_green\""));
        assert_eq!(json, encode_session_snapshot_json_v2(&snapshot));
        assert_eq!(
            decode_session_snapshot_json_v2(json.as_bytes()).expect("decode SessionSnapshotV2"),
            snapshot
        );
    }

    #[test]
    fn decoder_accepts_whitespace_and_rejects_unknown_protocol() {
        let empty = br#" { "sources": [], "observation_session_id": "obs-2", "protocol_version": 2 } "#;
        let decoded = decode_session_snapshot_json_v2(empty).expect("decode reordered JSON fields");
        assert_eq!(decoded.observation_session_id, "obs-2");
        assert!(decode_session_snapshot_json_v2(
            br#"{"protocol_version":1,"observation_session_id":"obs","sources":[]}"#
        )
        .is_err());
    }
}
