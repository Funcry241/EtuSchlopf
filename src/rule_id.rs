// v0.10.728 real rule-id ownership module; no wrapper/include shell.
// --- mechanical module boundary: former src/rule_id.rs ---
use crate::model::Severity;

macro_rules! define_rule_ids {
    ($($variant:ident => $id:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum RuleId {
            $($variant,)+
        }

        impl RuleId {
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(RuleId::$variant => $id,)+
                }
            }

            pub fn from_str(id: &str) -> Option<Self> {
                match id {
                    $($id => Some(RuleId::$variant),)+
                    _ => None,
                }
            }

        }
    };
}

define_rule_ids! {
    Rhy01 => "RHY01",
    Rhy02 => "RHY02",
    Rhy03 => "RHY03",
    Rhy04 => "RHY04",
    Rhy05 => "RHY05",
    Rhy06 => "RHY06",
    Rhy07 => "RHY07",
    Rhy08 => "RHY08",
    Rhy09 => "RHY09",
    Rst01 => "RST01",
    Bea01 => "BEA01",
    Bea02 => "BEA02",
    Bea03 => "BEA03",
    Bea04 => "BEA04",
    Bea05 => "BEA05",
    Bea06 => "BEA06",
    Voi01 => "VOI01",
    Voi02 => "VOI02",
    Voi03 => "VOI03",
    Voi04 => "VOI04",
    Voi05 => "VOI05",
    Voi06 => "VOI06",
    Han01 => "HAN01",
    Han02 => "HAN02",
    Han03 => "HAN03",
    Han04 => "HAN04",
    Han05 => "HAN05",
    Han06 => "HAN06",
    Han07 => "HAN07",
    Han08 => "HAN08",
    Reg01 => "REG01",
    Reg02 => "REG02",
    Reg03 => "REG03",
    Reg04 => "REG04",
    Reg05 => "REG05",
    Reg06 => "REG06",
    Reg07 => "REG07",
    Oct01 => "OCT01",
    Oct02 => "OCT02",
    Oct03 => "OCT03",
    Oct04 => "OCT04",
    Oct05 => "OCT05",
    Oct06 => "OCT06",
    Oct07 => "OCT07",
    Sub01 => "SUB01",
    Sub02 => "SUB02",
    Sub03 => "SUB03",
    Sub04 => "SUB04",
    Sub05 => "SUB05",
    Sub06 => "SUB06",
    Sub07 => "SUB07",
    Sub08 => "SUB08",
    Sub09 => "SUB09",
    Sub10 => "SUB10",
    Lay01 => "LAY01",
    Lay03 => "LAY03",
    Lay04 => "LAY04",
    Lay05 => "LAY05",
    Lay06 => "LAY06",
    Lay07 => "LAY07",
    Lay08 => "LAY08",
    Lay09 => "LAY09",
    Lay10 => "LAY10",
    Slr01 => "SLR01",
    Clf01 => "CLF01",
    Clf03 => "CLF03",
    Clf04 => "CLF04",
    Clf05 => "CLF05",
    Clf06 => "CLF06",
    Clf09 => "CLF09",
    Col01 => "COL01",
    Col02 => "COL02",
    Sty01 => "STY01",
    Sty02 => "STY02",
    Sty03 => "STY03",
    Sty04 => "STY04",
    Sty05 => "STY05",
    Ref01 => "REF01",
    Ref02 => "REF02",
    Ref03 => "REF03",
    Ref04 => "REF04",
    Ref05 => "REF05",
    Exp01 => "EXP01",
    Exp02 => "EXP02",
    Exp03 => "EXP03",
    Exp04 => "EXP04",
    Exp05 => "EXP05",
    Grd01 => "GRD01",
    Grd02 => "GRD02",
    Grd03 => "GRD03",
    Grd04 => "GRD04",
    Grd05 => "GRD05",
    Grd06 => "GRD06",
    Grd07 => "GRD07",
    Grd08 => "GRD08",
    Grd09 => "GRD09",
    Grd10 => "GRD10",
    Opt01 => "OPT01",
    Gdl01 => "GDL01",
    Mat01 => "MAT01",
    Mat02 => "MAT02",
    Grf01 => "GRF01",
    Par01 => "PAR01",
    Top01 => "TOP01",
    Ent01 => "ENT01",
    Cat01 => "CAT01",
    Gau01 => "GAU01",
    Gau02 => "GAU02",
    Gau03 => "GAU03",
}

impl std::fmt::Display for RuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RuntimeSeverityProfile {
    Blocker,
    Warning,
    Hint,
    StrictBlockerElseWarning,
}

impl RuntimeSeverityProfile {
    pub fn severity(self, strict_context: bool) -> Severity {
        match self {
            RuntimeSeverityProfile::Blocker => Severity::Blocker,
            RuntimeSeverityProfile::Warning => Severity::Warning,
            RuntimeSeverityProfile::Hint => Severity::Hint,
            RuntimeSeverityProfile::StrictBlockerElseWarning => {
                if strict_context {
                    Severity::Blocker
                } else {
                    Severity::Warning
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IssuePriorityProfile {
    ActualValue,
    RhythmCritical,
    SafetyCritical,
    HandSpan,
    FastLargeJump,
    DenseStaff,
    Register,
}

impl IssuePriorityProfile {
    pub fn score(self, actual_divisions: Option<i64>) -> i32 {
        let value = actual_divisions.unwrap_or(0) as i32;
        match self {
            IssuePriorityProfile::ActualValue => value,
            IssuePriorityProfile::RhythmCritical => 400 + value.abs(),
            IssuePriorityProfile::SafetyCritical => 350 + value.abs(),
            IssuePriorityProfile::HandSpan => {
                if value >= 19 {
                    200 + value
                } else {
                    100 + value
                }
            }
            IssuePriorityProfile::FastLargeJump => {
                if value >= 24 {
                    190 + value
                } else {
                    90 + value
                }
            }
            IssuePriorityProfile::DenseStaff => 80 + value,
            IssuePriorityProfile::Register => 20 + value,
        }
    }
}

macro_rules! define_validator_issue_authority {
    (
        $(
            $kind:ident => {
                issue_type: $issue_type:literal,
                rule_id: $rule_id:expr,
                actual_unit: $actual_unit:expr,
                runtime_severity_profile: $runtime_severity_profile:expr,
                priority_profile: $priority_profile:expr,
                repair_hint: $repair_hint:expr $(,)?
            }
        ),+ $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum ValidatorIssueKind {
            $($kind,)+
        }

        impl ValidatorIssueKind {
            pub fn as_str(self) -> &'static str {
                match self {
                    $(ValidatorIssueKind::$kind => $issue_type,)+
                }
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct ValidatorIssueAuthority {
            pub kind: ValidatorIssueKind,
            pub rule_id: RuleId,
            pub actual_unit: Option<&'static str>,
            pub runtime_severity_profile: RuntimeSeverityProfile,
            pub priority_profile: IssuePriorityProfile,
            pub repair_hint: Option<&'static str>,
        }

        pub const VALIDATOR_ISSUE_AUTHORITY: &[ValidatorIssueAuthority] = &[
            $(
                ValidatorIssueAuthority {
                    kind: ValidatorIssueKind::$kind,
                    rule_id: $rule_id,
                    actual_unit: $actual_unit,
                    runtime_severity_profile: $runtime_severity_profile,
                    priority_profile: $priority_profile,
                    repair_hint: $repair_hint,
                },
            )+
        ];
    };
}

define_validator_issue_authority! {
    BackupBeforeMeasureStart => { issue_type: "BACKUP_BEFORE_MEASURE_START", rule_id: RuleId::Rhy02, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::SafetyCritical, repair_hint: None },
    MeasureCursorUnderfull => { issue_type: "MEASURE_CURSOR_UNDERFULL", rule_id: RuleId::Rhy01, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::RhythmCritical, repair_hint: None },
    MeasureCursorOverfull => { issue_type: "MEASURE_CURSOR_OVERFULL", rule_id: RuleId::Rhy01, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::RhythmCritical, repair_hint: None },
    VoiceDurationUnderfull => { issue_type: "VOICE_DURATION_UNDERFULL", rule_id: RuleId::Rhy01, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::StrictBlockerElseWarning, priority_profile: IssuePriorityProfile::RhythmCritical, repair_hint: None },
    VoiceDurationOverfull => { issue_type: "VOICE_DURATION_OVERFULL", rule_id: RuleId::Rhy01, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::RhythmCritical, repair_hint: None },
    VoiceDurationMismatch => { issue_type: "VOICE_DURATION_MISMATCH", rule_id: RuleId::Rhy01, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    VisibleVoiceDurationUnderfull => { issue_type: "VISIBLE_VOICE_DURATION_UNDERFULL", rule_id: RuleId::Rhy01, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    VisibleVoiceDurationOverfull => { issue_type: "VISIBLE_VOICE_DURATION_OVERFULL", rule_id: RuleId::Rhy01, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    VisibleVoiceDurationMismatch => { issue_type: "VISIBLE_VOICE_DURATION_MISMATCH", rule_id: RuleId::Rhy01, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    VisibleDurationTypeMissing => { issue_type: "VISIBLE_DURATION_TYPE_MISSING", rule_id: RuleId::Rhy03, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    VisibleDurationTypeUnknown => { issue_type: "VISIBLE_DURATION_TYPE_UNKNOWN", rule_id: RuleId::Rhy03, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    VisibleDurationTypeMismatch => { issue_type: "VISIBLE_DURATION_TYPE_MISMATCH", rule_id: RuleId::Rhy03, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    VisibleDurationTypeInexact => { issue_type: "VISIBLE_DURATION_TYPE_INEXACT", rule_id: RuleId::Rhy03, actual_unit: Some("divisions"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    RhythmLedgerHiddenTimelineEvent => { issue_type: "RHYTHM_LEDGER_HIDDEN_TIMELINE_EVENT", rule_id: RuleId::Rhy08, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    PianoRangeOutsideA0C8 => { issue_type: "PIANO_RANGE_OUTSIDE_A0_C8", rule_id: RuleId::Rhy01, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::SafetyCritical, repair_hint: None },
    PianoHandSpanLarge => { issue_type: "PIANO_HAND_SPAN_LARGE", rule_id: RuleId::Han02, actual_unit: Some("semitones"), runtime_severity_profile: RuntimeSeverityProfile::Warning, priority_profile: IssuePriorityProfile::HandSpan, repair_hint: Some("Umverteilen / ausdünnen / Arpeggio prüfen.") },
    PianoFastLargeJump => { issue_type: "PIANO_FAST_LARGE_JUMP", rule_id: RuleId::Han03, actual_unit: Some("semitones"), runtime_severity_profile: RuntimeSeverityProfile::Warning, priority_profile: IssuePriorityProfile::FastLargeJump, repair_hint: Some("VoiceRun-/StaffFiber-Beweis prüfen; keine automatische Umverteilung.") },
    PianoTooManySimultaneousNotesOneStaff => { issue_type: "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF", rule_id: RuleId::Han06, actual_unit: Some("notes"), runtime_severity_profile: RuntimeSeverityProfile::Warning, priority_profile: IssuePriorityProfile::DenseStaff, repair_hint: Some("Ausdünnen / auf beide Hände verteilen.") },
    RightHandLowRegister => { issue_type: "RIGHT_HAND_LOW_REGISTER", rule_id: RuleId::Reg01, actual_unit: Some("midi"), runtime_severity_profile: RuntimeSeverityProfile::Hint, priority_profile: IssuePriorityProfile::Register, repair_hint: Some("Handverteilung prüfen.") },
    LeftHandHighRegister => { issue_type: "LEFT_HAND_HIGH_REGISTER", rule_id: RuleId::Reg01, actual_unit: Some("midi"), runtime_severity_profile: RuntimeSeverityProfile::Hint, priority_profile: IssuePriorityProfile::Register, repair_hint: Some("Handverteilung prüfen.") },
    OrphanSlurEndpoint => { issue_type: "ORPHAN_SLUR_ENDPOINT", rule_id: RuleId::Slr01, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Warning, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    OrphanTieEndpoint => { issue_type: "ORPHAN_TIE_ENDPOINT", rule_id: RuleId::Slr01, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Warning, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    EmptyTemporaryClefRange => { issue_type: "EMPTY_TEMPORARY_CLEF_RANGE", rule_id: RuleId::Clf01, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Hint, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    TrailingRedundantClef => { issue_type: "TRAILING_REDUNDANT_CLEF", rule_id: RuleId::Clf03, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Hint, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    DenseTemporaryClefCluster => { issue_type: "DENSE_TEMPORARY_CLEF_CLUSTER", rule_id: RuleId::Clf04, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Hint, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    RedundantClefChange => { issue_type: "REDUNDANT_CLEF_CHANGE", rule_id: RuleId::Clf05, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Hint, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    UnneededAdvanceClef => { issue_type: "UNNEEDED_ADVANCE_CLEF", rule_id: RuleId::Clf06, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Hint, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    VisibleNoteheadCollision => { issue_type: "VISIBLE_NOTEHEAD_COLLISION", rule_id: RuleId::Col01, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Warning, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    VisibleNoteRestCollision => { issue_type: "VISIBLE_NOTE_REST_COLLISION", rule_id: RuleId::Col02, actual_unit: None, runtime_severity_profile: RuntimeSeverityProfile::Warning, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
    RawCursorInvariantHardGate => { issue_type: "RAW_CURSOR_INVARIANT_HARD_GATE", rule_id: RuleId::Rhy01, actual_unit: Some("raw_cursor_max"), runtime_severity_profile: RuntimeSeverityProfile::Blocker, priority_profile: IssuePriorityProfile::ActualValue, repair_hint: None },
}

pub fn validator_issue_authority(kind: ValidatorIssueKind) -> &'static ValidatorIssueAuthority {
    VALIDATOR_ISSUE_AUTHORITY
        .iter()
        .find(|entry| entry.kind == kind)
        .expect("ValidatorIssueKind must be listed in ValidatorIssueAuthority")
}

pub fn validator_issue_authority_by_issue_type(issue_type: &str) -> Option<&'static ValidatorIssueAuthority> {
    VALIDATOR_ISSUE_AUTHORITY
        .iter()
        .find(|entry| entry.kind.as_str() == issue_type)
}

pub fn priority_score_for_issue_type(issue_type: &str, actual_divisions: Option<i64>) -> i32 {
    validator_issue_authority_by_issue_type(issue_type)
        .map_or_else(|| actual_divisions.unwrap_or(0) as i32, |entry| entry.priority_profile.score(actual_divisions))
}


pub fn runtime_severity_for_issue_kind(kind: ValidatorIssueKind, strict_context: bool) -> Severity {
    validator_issue_authority(kind)
        .runtime_severity_profile
        .severity(strict_context)
}

pub fn repair_hint_for_issue_type(issue_type: &str) -> Option<&'static str> {
    validator_issue_authority_by_issue_type(issue_type).and_then(|entry| entry.repair_hint)
}

pub fn validator_issue_authority_invariant_status() -> String {
    let mut seen_issue_types = std::collections::BTreeSet::new();
    let mut duplicate_issue_types = Vec::new();
    let mut empty_issue_types = Vec::new();
    let mut empty_units = Vec::new();
    let mut empty_repair_hints = Vec::new();
    let mut issue_type_roundtrip_errors = Vec::new();
    let mut rule_id_roundtrip_errors = Vec::new();

    for entry in VALIDATOR_ISSUE_AUTHORITY {
        let issue_type = entry.kind.as_str();
        if issue_type.trim().is_empty() {
            empty_issue_types.push(issue_type);
        }
        if !seen_issue_types.insert(issue_type) {
            duplicate_issue_types.push(issue_type);
        }
        if validator_issue_authority_by_issue_type(issue_type).map(|candidate| candidate.kind) != Some(entry.kind) {
            issue_type_roundtrip_errors.push(issue_type);
        }
        if RuleId::from_str(entry.rule_id.as_str()) != Some(entry.rule_id) {
            rule_id_roundtrip_errors.push(issue_type);
        }
        if entry.actual_unit.is_some_and(|unit| unit.trim().is_empty()) {
            empty_units.push(issue_type);
        }
        if entry.repair_hint.is_some_and(|hint| hint.trim().is_empty()) {
            empty_repair_hints.push(issue_type);
        }
    }

    if duplicate_issue_types.is_empty()
        && empty_issue_types.is_empty()
        && empty_units.is_empty()
        && empty_repair_hints.is_empty()
        && issue_type_roundtrip_errors.is_empty()
        && rule_id_roundtrip_errors.is_empty()
    {
        format!(
            "OK: {} Validator-IssueTypes mit eindeutiger IssueType-Roundtrip-, RuleId-Roundtrip-, Einheit- und Reparaturhinweis-Invariante",
            VALIDATOR_ISSUE_AUTHORITY.len()
        )
    } else {
        format!(
            "FEHLER: ValidatorIssueAuthority-Invariante verletzt; doppelteIssueTypes=[{}], leereIssueTypes=[{}], leereEinheiten=[{}], leereReparaturhinweise=[{}], issueTypeRoundtrip=[{}], ruleIdRoundtrip=[{}]",
            duplicate_issue_types.join(","),
            empty_issue_types.join(","),
            empty_units.join(","),
            empty_repair_hints.join(","),
            issue_type_roundtrip_errors.join(","),
            rule_id_roundtrip_errors.join(",")
        )
    }
}
