// v0.10.726 real crate-root module split from the old score grouping.
// No behavior change: original module body, visibility and call paths are preserved.
pub(crate) mod metric_rest_notation;

    use serde::Serialize;
    use std::fmt;
    
    #[derive(Debug, Clone)]
    pub struct Score {
        pub title: Option<String>,
        pub parts: Vec<Part>,
    }
    
    #[derive(Debug, Clone)]
    pub struct Part {
        pub id: String,
        pub name: String,
        pub measures: Vec<Measure>,
    }
    
    #[derive(Debug, Clone)]
    pub struct Measure {
        pub number: String,
        pub divisions: i64,
        pub beats: i64,
        pub beat_type: i64,
        pub expected: ExpectedDuration,
        pub cursor_min: i64,
        pub cursor_max: i64,
        pub events: Vec<Event>,
    }
    
    #[derive(Debug, Clone, Copy)]
    pub struct ExpectedDuration {
        /// Erwartete Taktlänge in MusicXML-Divisions als Bruch: numerator / denominator.
        pub numerator: i64,
        pub denominator: i64,
    }
    
    impl ExpectedDuration {
        pub fn new(divisions: i64, beats: i64, beat_type: i64) -> Self {
            Self {
                numerator: divisions * beats * 4,
                denominator: beat_type.max(1),
            }
        }
    
        pub fn compare_to_actual(&self, actual: i64) -> i64 {
            actual * self.denominator - self.numerator
        }
    
        pub fn as_text(&self) -> String {
            if self.numerator % self.denominator == 0 {
                format!("{} divisions", self.numerator / self.denominator)
            } else {
                format!("{}/{} divisions", self.numerator, self.denominator)
            }
        }
    }
    
    #[derive(Debug, Clone)]
    pub struct Event {
        pub kind: EventKind,
        pub staff: String,
        pub voice: String,
        pub start: i64,
        pub duration: i64,
        pub pitch: Option<Pitch>,
        pub is_chord: bool,
        pub tie_stop: bool,
        pub visible: bool,
        pub note_type: Option<String>,
        pub dots: usize,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum EventKind {
        Note,
        Rest,
    }
    
    #[derive(Debug, Clone, Serialize)]
    pub struct Pitch {
        pub step: String,
        pub alter: i32,
        pub octave: i32,
        pub midi: i32,
        pub label: String,
    }
    
    impl Pitch {
        pub fn new(step: String, alter: i32, octave: i32) -> Self {
            let base = match step.as_str() {
                "C" => 0,
                "D" => 2,
                "E" => 4,
                "F" => 5,
                "G" => 7,
                "A" => 9,
                "B" => 11,
                _ => 0,
            };
            let midi = (octave + 1) * 12 + base + alter;
            let accidental = match alter {
                -2 => "bb".to_string(),
                -1 => "b".to_string(),
                0 => "".to_string(),
                1 => "#".to_string(),
                2 => "x".to_string(),
                n if n > 0 => format!("+{n}"),
                n => n.to_string(),
            };
            let label = format!("{step}{accidental}{octave}");
            Self {
                step,
                alter,
                octave,
                midi,
                label,
            }
        }
    }
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Severity {
        Blocker,
        Warning,
        Hint,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Han03EndpointIdentity {
        pub start: i64,
        pub duration: i64,
        pub midi: i32,
        pub pitch_label: String,
        pub is_chord: bool,
        pub tie_stop: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Han03TemporalEdge {
        pub from_end: i64,
        pub onset_delta: i64,
        pub release_to_attack_gap: i64,
        pub direct_attack_successor: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Han03JumpIdentity {
        pub line_kind: String,
        pub from: Han03EndpointIdentity,
        pub to: Han03EndpointIdentity,
        pub temporal_edge: Han03TemporalEdge,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Issue {
        pub severity: Severity,
        pub issue_type: String,
        pub rule_id: String,
        pub rule_severity: String,
        pub rule_weight: i32,
        pub part_id: String,
        pub part_name: String,
        pub measure: String,
        pub staff: Option<String>,
        pub voice: Option<String>,
        pub expected_divisions: Option<String>,
        #[serde(rename = "actual_value")]
        pub actual_divisions: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub actual_unit: Option<String>,
        pub delta_divisions: Option<String>,
        #[serde(skip)]
        pub han03_jump_identity: Option<Han03JumpIdentity>,
        pub details: String,
        pub message_for_ai: String,
    }
    
    #[derive(Debug, Clone, Serialize)]
    pub struct ReportSummary {
        pub status: ReportStatus,
        pub blockers: usize,
        pub warnings: usize,
        pub hints: usize,
    }
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum ReportStatus {
        Ok,
        Warn,
        Blocked,
    }
    
    impl fmt::Display for ReportStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ReportStatus::Ok => write!(f, "OK"),
                ReportStatus::Warn => write!(f, "WARNUNG"),
                ReportStatus::Blocked => write!(f, "BLOCKIERT"),
            }
        }
    }
    
    #[derive(Debug, Clone, Serialize)]
    pub struct CheckReport {
        pub tool: String,
        pub version: String,
        pub score_title: Option<String>,
        pub summary: ReportSummary,
        pub issues: Vec<Issue>,
    }
    
    impl CheckReport {
        pub fn exit_code(&self) -> i32 {
            match self.summary.status {
                ReportStatus::Ok => 0,
                ReportStatus::Warn => 0,
                ReportStatus::Blocked => 2,
            }
        }
    }
