// v0.10.728 real validation rules ownership module; no wrapper/include shell.
// --- mechanical module boundary: former src/rules.rs ---
use crate::model::{
    CheckReport, Event, EventKind, Han03EndpointIdentity, Han03JumpIdentity, Han03TemporalEdge,
    Issue, Part, ReportStatus, ReportSummary, Score, Severity,
};
use crate::central_rulebook::rule_by_id;
use crate::rule_id::{runtime_severity_for_issue_kind, validator_issue_authority, ValidatorIssueKind};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct CheckOptions {
    pub strict: bool,
}

pub fn run_checks(score: &Score, options: &CheckOptions) -> CheckReport {
    let mut issues = Vec::new();

    for part in &score.parts {
        for measure in &part.measures {
            check_measure_cursor(part, measure, &mut issues);
            check_voice_durations(part, measure, options, &mut issues);
            check_visible_voice_durations(part, measure, &mut issues);
            check_duration_type_consistency(part, measure, &mut issues);
            check_rhythm_ledger_visibility(part, measure, &mut issues);

            // v0.1.140: Note-Events pro Takt nur einmal materialisieren.
            // Die eigentlichen Validatoren bekommen dieselbe Slice-Sicht; das
            // reduziert wiederholtes Filtern und macht die folgende
            // Hand-/Sprunglogik nachvollziehbarer.
            let visible_measure_note_events: Vec<&Event> = visible_note_events(measure).collect();
            check_piano_range(part, measure, &visible_measure_note_events, &mut issues);
            check_piano_chords(part, measure, &visible_measure_note_events, &mut issues);
            check_piano_jumps(part, measure, &visible_measure_note_events, &mut issues);
            check_staff_register(part, measure, &visible_measure_note_events, &mut issues);
        }
    }

    let blockers = issues.iter().filter(|i| i.severity == Severity::Blocker).count();
    let warnings = issues.iter().filter(|i| i.severity == Severity::Warning).count();
    let hints = issues.iter().filter(|i| i.severity == Severity::Hint).count();
    let status = if blockers > 0 {
        ReportStatus::Blocked
    } else if warnings > 0 {
        ReportStatus::Warn
    } else {
        ReportStatus::Ok
    };

    CheckReport {
        tool: "etuschlopf".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        score_title: score.title.clone(),
        summary: ReportSummary {
            status,
            blockers,
            warnings,
            hints,
        },
        issues,
    }
}

fn check_measure_cursor(part: &Part, measure: &crate::model::Measure, issues: &mut Vec<Issue>) {
    if measure.cursor_min < 0 {
        issues.push(issue(
            false,
            ValidatorIssueKind::BackupBeforeMeasureStart,
            part,
            measure,
            None,
            None,
            Some(measure.expected.as_text()),
            Some(measure.cursor_min),
            None,
            format!(
                "backup/forward-Timeline läuft vor den Taktanfang: minimale Cursor-Position {} divisions.",
                measure.cursor_min
            ),
            "MusicXML backup/forward prüfen: Der Cursor darf innerhalb eines Takts nicht vor 0 laufen.".to_string(),
        ));
    }

    let delta_scaled = measure.expected.compare_to_actual(measure.cursor_max);
    if delta_scaled < 0 {
        let missing = -delta_scaled;
        issues.push(issue(
            false,
            ValidatorIssueKind::MeasureCursorUnderfull,
            part,
            measure,
            None,
            None,
            Some(measure.expected.as_text()),
            Some(measure.cursor_max),
            Some(format_scaled(missing, measure.expected.denominator)),
            format!(
                "Die MusicXML-Cursor-Timeline erreicht nur {} divisions; erwartet ist {}.",
                measure.cursor_max,
                measure.expected.as_text()
            ),
            format!(
                "Takt {} Part {} ist global unterfüllt. Ergänze exakt {} Dauer oder korrigiere backup/forward.",
                measure.number,
                part.name,
                format_scaled(missing, measure.expected.denominator)
            ),
        ));
    } else if delta_scaled != 0 {
        issues.push(issue(
            false,
            ValidatorIssueKind::MeasureCursorOverfull,
            part,
            measure,
            None,
            None,
            Some(measure.expected.as_text()),
            Some(measure.cursor_max),
            Some(format_scaled(delta_scaled, measure.expected.denominator)),
            format!(
                "Die MusicXML-Cursor-Timeline läuft bis {} divisions; erwartet ist {}.",
                measure.cursor_max,
                measure.expected.as_text()
            ),
            format!(
                "Takt {} Part {} ist global überfüllt. Entferne oder verkürze exakt {} Dauer.",
                measure.number,
                part.name,
                format_scaled(delta_scaled, measure.expected.denominator)
            ),
        ));
    }
}


fn cross_staff_voices_in_measure(measure: &crate::model::Measure) -> BTreeSet<String> {
    let mut staffs_by_voice: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for event in &measure.events {
        if event.duration <= 0 || event.is_chord {
            continue;
        }
        staffs_by_voice
            .entry(event.voice.clone())
            .or_default()
            .insert(event.staff.clone());
    }
    staffs_by_voice
        .into_iter()
        .filter_map(|(voice, staffs)| if staffs.len() > 1 { Some(voice) } else { None })
        .collect()
}

fn duration_validation_key(event: &Event, cross_staff_voices: &BTreeSet<String>) -> (String, String) {
    if cross_staff_voices.contains(&event.voice) {
        ("cross-staff".to_string(), event.voice.clone())
    } else {
        (event.staff.clone(), event.voice.clone())
    }
}

fn check_voice_durations(
    part: &Part,
    measure: &crate::model::Measure,
    options: &CheckOptions,
    issues: &mut Vec<Issue>,
) {
    // v0.1.111: Cross-staff ist in MusicXML eine Eigenschaft einer Stimme,
    // nicht zwingend eine neue Takt-Timeline. Wenn dieselbe Voice sichtbar auf
    // mehreren Systemen notiert wird, validieren wir ihre Dauer als gemeinsame
    // Voice-Spur. Das verhindert, dass reine Staff-Umschreibungen fälschlich als
    // getrennte Staff/Voice-Takte über- oder unterfüllt gelten.
    let cross_staff_voices = cross_staff_voices_in_measure(measure);
    let mut by_voice: BTreeMap<(String, String), Vec<(i64, i64)>> = BTreeMap::new();
    for event in &measure.events {
        if event.duration <= 0 {
            continue;
        }
        let start = event.start;
        let end = event.start + event.duration;
        by_voice
            .entry(duration_validation_key(event, &cross_staff_voices))
            .or_default()
            .push((start, end));
    }

    for ((staff, voice), mut intervals) in by_voice {
        let actual = union_length(&mut intervals);
        let delta_scaled = measure.expected.compare_to_actual(actual);
        if delta_scaled == 0 {
            continue;
        }

        let (strict_context, issue_kind, verb, magnitude, ai_hint) = if delta_scaled < 0 {
            let missing = -delta_scaled;
            (
                options.strict,
                ValidatorIssueKind::VoiceDurationUnderfull,
                "unterfüllt",
                missing,
                format!(
                    "Takt {} Part {} Staff {} Voice {}: Ergänze exakt {} Dauer, bevorzugt als explizite Pause, sofern musikalisch nichts anderes beabsichtigt ist.",
                    measure.number,
                    part.name,
                    staff,
                    voice,
                    format_scaled(missing, measure.expected.denominator)
                ),
            )
        } else {
            (
                false,
                ValidatorIssueKind::VoiceDurationOverfull,
                "überfüllt",
                delta_scaled,
                format!(
                    "Takt {} Part {} Staff {} Voice {}: Entferne oder verkürze exakt {} Dauer.",
                    measure.number,
                    part.name,
                    staff,
                    voice,
                    format_scaled(delta_scaled, measure.expected.denominator)
                ),
            )
        };

        issues.push(issue(
            strict_context,
            issue_kind,
            part,
            measure,
            Some(staff.clone()),
            Some(voice.clone()),
            Some(measure.expected.as_text()),
            Some(actual),
            Some(format_scaled(magnitude, measure.expected.denominator)),
            format!(
                "Staff {}, Voice {} ist {}: tatsächliche Abdeckung {} divisions, erwartet {}.",
                staff,
                voice,
                verb,
                actual,
                measure.expected.as_text()
            ),
            ai_hint,
        ));
    }
}


fn check_visible_voice_durations(
    part: &Part,
    measure: &crate::model::Measure,
    issues: &mut Vec<Issue>,
) {
    let cross_staff_voices = cross_staff_voices_in_measure(measure);
    let mut by_voice: BTreeMap<(String, String), Vec<(i64, i64)>> = BTreeMap::new();
    for event in &measure.events {
        if event.duration <= 0 || !event.visible || event.is_chord {
            continue;
        }
        by_voice
            .entry(duration_validation_key(event, &cross_staff_voices))
            .or_default()
            .push((event.start, event.start + event.duration));
    }

    for ((staff, voice), mut intervals) in by_voice {
        let actual = union_length(&mut intervals);
        let delta_scaled = measure.expected.compare_to_actual(actual);
        if delta_scaled == 0 {
            continue;
        }

        let (issue_kind, verb, magnitude, ai_hint) = if delta_scaled < 0 {
            let missing = -delta_scaled;
            (
                ValidatorIssueKind::VisibleVoiceDurationUnderfull,
                "sichtbar unterfüllt",
                missing,
                format!(
                    "Takt {} Part {} Staff {} Voice {}: Sichtbar fehlen {}. Ergänze echte sichtbare Pausen; versteckte Events dürfen die lesbare Taktfüllung nicht retten.",
                    measure.number,
                    part.name,
                    staff,
                    voice,
                    format_scaled(missing, measure.expected.denominator)
                ),
            )
        } else {
            (
                ValidatorIssueKind::VisibleVoiceDurationOverfull,
                "sichtbar überfüllt",
                delta_scaled,
                format!(
                    "Takt {} Part {} Staff {} Voice {}: Sichtbar sind {} zu viel notiert. Dauern/Bindungen/Pausen korrigieren.",
                    measure.number,
                    part.name,
                    staff,
                    voice,
                    format_scaled(delta_scaled, measure.expected.denominator)
                ),
            )
        };

        issues.push(issue(
            false,
            issue_kind,
            part,
            measure,
            Some(staff.clone()),
            Some(voice.clone()),
            Some(measure.expected.as_text()),
            Some(actual),
            Some(format_scaled(magnitude, measure.expected.denominator)),
            format!(
                "VisibleRhythmLedger: Staff {}, Voice {} ist {}: sichtbare Abdeckung {} divisions, erwartet {}.",
                staff,
                voice,
                verb,
                actual,
                measure.expected.as_text()
            ),
            ai_hint,
        ));
    }
}

#[derive(Debug, Clone, Copy)]
enum DurationExpectation {
    Exact(i64),
    Inexact { numerator: i64, denominator: i64 },
}

impl DurationExpectation {
    fn as_text(&self) -> String {
        match *self {
            DurationExpectation::Exact(value) => format!("{value} divisions"),
            DurationExpectation::Inexact { numerator, denominator } => {
                format!("{numerator}/{denominator} divisions")
            }
        }
    }
}

fn check_duration_type_consistency(part: &Part, measure: &crate::model::Measure, issues: &mut Vec<Issue>) {
    for event in &measure.events {
        if event.duration <= 0 || !event.visible || event.is_chord {
            continue;
        }
        let Some(note_type) = event.note_type.as_deref() else {
            issues.push(issue(
                false,
                ValidatorIssueKind::VisibleDurationTypeMissing,
                part,
                measure,
                Some(event.staff.clone()),
                Some(event.voice.clone()),
                Some(measure.expected.as_text()),
                Some(event.duration),
                None,
                format!(
                    "VisibleRhythmLedger: sichtbares Ereignis Staff {}, Voice {} mit {} divisions hat keinen MusicXML-Notenwert.",
                    event.staff,
                    event.voice,
                    event.duration
                ),
                format!(
                    "Takt {} Part {} Staff {} Voice {}: <type> passend zur Dauer ergänzen.",
                    measure.number,
                    part.name,
                    event.staff,
                    event.voice
                ),
            ));
            continue;
        };
        let Some(expected_duration) = duration_for_type_and_dots(measure.divisions, note_type, event.dots) else {
            issues.push(issue(
                false,
                ValidatorIssueKind::VisibleDurationTypeUnknown,
                part,
                measure,
                Some(event.staff.clone()),
                Some(event.voice.clone()),
                Some(measure.expected.as_text()),
                Some(event.duration),
                Some(note_type.to_string()),
                format!(
                    "VisibleRhythmLedger: unbekannter MusicXML-Notenwert '{note_type}' bei sichtbarem Ereignis."
                ),
                format!(
                    "Takt {} Part {}: Notenwert '{}' durch gültigen MusicXML-Wert ersetzen.",
                    measure.number,
                    part.name,
                    note_type
                ),
            ));
            continue;
        };

        match expected_duration {
            DurationExpectation::Exact(expected_duration) => {
                if expected_duration != event.duration {
                    issues.push(issue(
                        false,
                        ValidatorIssueKind::VisibleDurationTypeMismatch,
                        part,
                        measure,
                        Some(event.staff.clone()),
                        Some(event.voice.clone()),
                        Some(format!("{} + {} Punkt(e)", note_type, event.dots)),
                        Some(event.duration),
                        Some(expected_duration.to_string()),
                        format!(
                            "VisibleRhythmLedger: Dauer {} divisions passt nicht zu Notenwert '{}' mit {} Punkt(en) (= {} divisions).",
                            event.duration,
                            note_type,
                            event.dots,
                            expected_duration
                        ),
                        format!(
                            "Takt {} Part {} Staff {} Voice {}: duration/type/dot synchronisieren oder Dauer in gebundene Teilwerte/Pausen zerlegen.",
                            measure.number,
                            part.name,
                            event.staff,
                            event.voice
                        ),
                    ));
                }
            }
            DurationExpectation::Inexact { numerator, denominator } => {
                issues.push(issue(
                    false,
                    ValidatorIssueKind::VisibleDurationTypeInexact,
                    part,
                    measure,
                    Some(event.staff.clone()),
                    Some(event.voice.clone()),
                    Some(format!("{} + {} Punkt(e)", note_type, event.dots)),
                    Some(event.duration),
                    Some(format!("{numerator}/{denominator} divisions")),
                    format!(
                        "VisibleRhythmLedger: Notenwert '{}' mit {} Punkt(en) ergibt bei divisions={} keine ganzzahlige Dauer ({}). Abschneidende Ganzzahldivision wird nicht mehr akzeptiert.",
                        note_type,
                        event.dots,
                        measure.divisions,
                        (DurationExpectation::Inexact { numerator, denominator }).as_text()
                    ),
                    format!(
                        "Takt {} Part {} Staff {} Voice {}: divisions-Auflösung erhöhen oder Dauer als exakte gebundene Teilwerte/tuplets notieren; keine gerundete duration/type-Zuordnung verwenden.",
                        measure.number,
                        part.name,
                        event.staff,
                        event.voice
                    ),
                ));
            }
        }
    }
}

fn duration_for_type_and_dots(divisions: i64, note_type: &str, dots: usize) -> Option<DurationExpectation> {
    // v0.1.140: exakte rationale Dauerrechnung.
    // Früher wurde bei eighth/16th/32nd/64th per Ganzzahldivision
    // abgeschnitten. Das kann bei kleinen oder tupletnahen divisions-Werten
    // falsche duration/type-Mismatches erzeugen oder verdecken.
    let note_denominator = match note_type.trim() {
        "whole" => 1_i64,
        "half" => 2_i64,
        "quarter" => 4_i64,
        "eighth" => 8_i64,
        "16th" => 16_i64,
        "32nd" => 32_i64,
        "64th" => 64_i64,
        _ => return None,
    };
    if divisions <= 0 {
        return None;
    }

    let dot_factor_den = 1_i64.checked_shl(dots as u32)?;
    let dot_factor_num = dot_factor_den.checked_mul(2)?.checked_sub(1)?;
    let numerator = divisions
        .checked_mul(4)?
        .checked_mul(dot_factor_num)?;
    let denominator = note_denominator.checked_mul(dot_factor_den)?;
    let (numerator, denominator) = reduce_fraction(numerator, denominator);

    if denominator == 1 {
        Some(DurationExpectation::Exact(numerator))
    } else {
        Some(DurationExpectation::Inexact { numerator, denominator })
    }
}

fn reduce_fraction(mut numerator: i64, mut denominator: i64) -> (i64, i64) {
    if denominator < 0 {
        numerator = -numerator;
        denominator = -denominator;
    }
    let divisor = gcd_i64(numerator, denominator);
    if divisor == 0 {
        (numerator, denominator)
    } else {
        (numerator / divisor, denominator / divisor)
    }
}

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}


fn check_rhythm_ledger_visibility(part: &Part, measure: &crate::model::Measure, issues: &mut Vec<Issue>) {
    for event in &measure.events {
        if event.duration <= 0 || event.visible || event.is_chord {
            continue;
        }
        let pitch_text = event
            .pitch
            .as_ref()
            .map_or_else(|| "Pause".to_string(), |p| p.label.clone());
        issues.push(issue(
            false,
            ValidatorIssueKind::RhythmLedgerHiddenTimelineEvent,
            part,
            measure,
            Some(event.staff.clone()),
            Some(event.voice.clone()),
            Some(measure.expected.as_text()),
            Some(event.duration),
            None,
            format!(
                "Staff {}, Voice {} enthält ein unsichtbares takttragendes Ereignis ({}) mit {} divisions. Dadurch wirkt der Takt im Notenbild rhythmisch unvollständig.",
                event.staff,
                event.voice,
                pitch_text,
                event.duration
            ),
            format!(
                "Takt {} Part {} Staff {} Voice {}: Unsichtbares rhythmisches Ereignis sichtbar als Pause notieren oder als sichtbare Note führen; nicht nur verstecken.",
                measure.number,
                part.name,
                event.staff,
                event.voice
            ),
        ));
    }
}

fn check_piano_range(part: &Part, measure: &crate::model::Measure, note_events: &[&Event], issues: &mut Vec<Issue>) {
    for event in note_events.iter().copied() {
        if let Some(pitch) = event.pitch.as_ref()
            && (pitch.midi < 21 || pitch.midi > 108) {
                issues.push(issue(
                    false,
                    ValidatorIssueKind::PianoRangeOutsideA0C8,
                    part,
                    measure,
                    Some(event.staff.clone()),
                    Some(event.voice.clone()),
                    None,
                    None,
                    None,
                    format!(
                        "Note {} liegt außerhalb des Klavierumfangs A0-C8.",
                        pitch.label
                    ),
                    format!(
                        "Takt {}: Note {} verschieben oder umschreiben; der Klavierumfang ist A0-C8.",
                        measure.number, pitch.label
                    ),
                ));
            }
    }
}

fn check_piano_chords(part: &Part, measure: &crate::model::Measure, note_events: &[&Event], issues: &mut Vec<Issue>) {
    let mut by_staff_time: BTreeMap<(String, i64), Vec<&Event>> = BTreeMap::new();
    for event in note_events.iter().copied() {
        by_staff_time
            .entry((event.staff.clone(), event.start))
            .or_default()
            .push(event);
    }

    for ((staff, start), events) in by_staff_time {
        let pitches: Vec<_> = events.iter().filter_map(|e| e.pitch.as_ref()).collect();
        if pitches.len() < 2 {
            continue;
        }
        let min = pitches.iter().map(|p| p.midi).min().unwrap();
        let max = pitches.iter().map(|p| p.midi).max().unwrap();
        let span = max - min;
        let low = pitches.iter().min_by_key(|p| p.midi).unwrap();
        let high = pitches.iter().max_by_key(|p| p.midi).unwrap();

        if span > 12 {
            issues.push(issue(
                false,
                ValidatorIssueKind::PianoHandSpanLarge,
                part,
                measure,
                Some(staff.clone()),
                None,
                None,
                Some(span as i64),
                None,
                format!(
                    "Gleichzeitigkeit auf Staff {} bei Start {}: Spannweite {}-{} = {} Halbtöne. Für normale Spielbarkeit kritisch.",
                    staff, start, low.label, high.label, span
                ),
                format!(
                    "Takt {} Staff {}: große Spannweite {}-{} ({} Halbtöne). Zwischen den Händen umverteilen, Akkord ausdünnen oder Arpeggio prüfen.",
                    measure.number, staff, low.label, high.label, span
                ),
            ));
        }

        if pitches.len() > 5 {
            issues.push(issue(
                false,
                ValidatorIssueKind::PianoTooManySimultaneousNotesOneStaff,
                part,
                measure,
                Some(staff.clone()),
                None,
                None,
                Some(pitches.len() as i64),
                None,
                format!(
                    "Staff {} hat bei Start {} insgesamt {} gleichzeitige Noten. Das kann pianistisch überladen sein.",
                    staff,
                    start,
                    pitches.len()
                ),
                format!(
                    "Takt {} Staff {}: {} gleichzeitige Noten. Ausdünnen oder zwischen den Händen umverteilen.",
                    measure.number,
                    staff,
                    pitches.len()
                ),
            ));
        }
    }
}

#[derive(Clone, Copy)]
struct JumpPoint<'a> {
    event: &'a Event,
    midi: i32,
}

fn check_piano_jumps(part: &Part, measure: &crate::model::Measure, note_events: &[&Event], issues: &mut Vec<Issue>) {
    // v0.1.140: Sprungprüfung nicht mehr aus der tiefsten Note pro Startzeit
    // ableiten. Akkorde werden nach Voice und Außenstimmen betrachtet, damit
    // Oberstimmensprünge sichtbar bleiben und tiefe Zusatznoten nicht als
    // einzige Melodielinie missverstanden werden.
    let mut emitted: BTreeSet<(String, i64, i64, i32, i32)> = BTreeSet::new();

    let mut by_staff_voice: BTreeMap<(String, String), Vec<&Event>> = BTreeMap::new();
    let mut by_staff: BTreeMap<String, Vec<&Event>> = BTreeMap::new();
    for event in note_events.iter().copied() {
        if event.pitch.is_none() {
            continue;
        }
        by_staff_voice
            .entry((event.staff.clone(), event.voice.clone()))
            .or_default()
            .push(event);
        by_staff.entry(event.staff.clone()).or_default().push(event);
    }

    for ((staff, voice), events) in by_staff_voice {
        check_jump_line(
            JumpLineContext {
                part,
                measure,
                staff: &staff,
                voice: Some(&voice),
                line_kind: "voice-high",
            },
            build_extreme_line(events.iter().copied(), true),
            &mut emitted,
            issues,
        );
        check_jump_line(
            JumpLineContext {
                part,
                measure,
                staff: &staff,
                voice: Some(&voice),
                line_kind: "voice-low",
            },
            build_extreme_line(events.iter().copied(), false),
            &mut emitted,
            issues,
        );
    }

    for (staff, events) in by_staff {
        check_jump_line(
            JumpLineContext {
                part,
                measure,
                staff: &staff,
                voice: None,
                line_kind: "staff-outer-high",
            },
            build_extreme_line(events.iter().copied(), true),
            &mut emitted,
            issues,
        );
        check_jump_line(
            JumpLineContext {
                part,
                measure,
                staff: &staff,
                voice: None,
                line_kind: "staff-outer-low",
            },
            build_extreme_line(events.iter().copied(), false),
            &mut emitted,
            issues,
        );
    }
}

fn build_extreme_line<'a, I>(events: I, take_highest: bool) -> Vec<JumpPoint<'a>>
where
    I: Iterator<Item = &'a Event>,
{
    let mut by_start: BTreeMap<i64, JumpPoint<'a>> = BTreeMap::new();
    for event in events {
        if event.tie_stop {
            continue;
        }
        let Some(pitch) = event.pitch.as_ref() else { continue; };
        let point = JumpPoint { event, midi: pitch.midi };
        by_start
            .entry(event.start)
            .and_modify(|current| {
                let replace = if take_highest {
                    point.midi > current.midi
                } else {
                    point.midi < current.midi
                };
                if replace {
                    *current = point;
                }
            })
            .or_insert(point);
    }
    by_start.into_values().collect()
}

struct JumpLineContext<'a> {
    part: &'a Part,
    measure: &'a crate::model::Measure,
    staff: &'a str,
    voice: Option<&'a str>,
    line_kind: &'a str,
}

fn canonical_han03_temporal_edge(from: &Event, to: &Event) -> Han03TemporalEdge {
    let from_end = from.start + from.duration.max(1);
    Han03TemporalEdge {
        from_end,
        onset_delta: to.start - from.start,
        release_to_attack_gap: to.start - from_end,
        direct_attack_successor: from.staff == to.staff
            && from.voice == to.voice
            && to.start == from_end,
    }
}

fn check_jump_line(
    context: JumpLineContext<'_>,
    line: Vec<JumpPoint<'_>>,
    emitted: &mut BTreeSet<(String, i64, i64, i32, i32)>,
    issues: &mut Vec<Issue>,
) {
    let JumpLineContext {
        part,
        measure,
        staff,
        voice,
        line_kind,
    } = context;
    for pair in line.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let (Some(pa), Some(pb)) = (a.event.pitch.as_ref(), b.event.pitch.as_ref()) else { continue; };
        let interval = (pb.midi - pa.midi).abs();
        let temporal_edge = canonical_han03_temporal_edge(a.event, b.event);
        let time_gap = temporal_edge.onset_delta.abs();
        // HAN03 owns only large jumps whose source endpoint leaves no positive
        // release-to-attack window. Once A is released before B, the score still
        // carries a transition demand, but calling it "fast" needs tempo/capability
        // evidence from the existing demand audits rather than a release-blocking
        // score-only warning. This does not claim whole-hand freedom or playability.
        // Overlap and contiguous attacks stay
        // in HAN03 because release_to_attack_gap <= 0.
        if interval < 19
            || time_gap > measure.divisions.max(1)
            || temporal_edge.release_to_attack_gap > 0
        {
            continue;
        }

        let dedupe_key = (
            staff.to_string(),
            a.event.start,
            b.event.start,
            pa.midi,
            pb.midi,
        );
        if !emitted.insert(dedupe_key) {
            continue;
        }

        let voice_text = voice.unwrap_or("*");
        let mut jump_issue = issue(
            false,
            ValidatorIssueKind::PianoFastLargeJump,
            part,
            measure,
            Some(staff.to_string()),
            voice.map(str::to_string),
            None,
            Some(interval as i64),
            None,
            format!(
                "Schneller Sprung auf Staff {} Voice {} Linie {}: {} -> {}. Sprungweite: {} Halbtöne; Zeitabstand: {} divisions.",
                staff, voice_text, line_kind, pa.label, pb.label, interval, time_gap
            ),
            format!(
                "Takt {} Staff {} Voice {} Linie {}: schneller großer Sprung {} -> {}. VoiceRun-/StaffFiber-Kontext beweisen; keine automatische Umverteilung.",
                measure.number, staff, voice_text, line_kind, pa.label, pb.label
            ),
        );
        jump_issue.han03_jump_identity = Some(Han03JumpIdentity {
            line_kind: line_kind.to_string(),
            from: Han03EndpointIdentity {
                start: a.event.start,
                duration: a.event.duration,
                midi: pa.midi,
                pitch_label: pa.label.clone(),
                is_chord: a.event.is_chord,
                tie_stop: a.event.tie_stop,
            },
            to: Han03EndpointIdentity {
                start: b.event.start,
                duration: b.event.duration,
                midi: pb.midi,
                pitch_label: pb.label.clone(),
                is_chord: b.event.is_chord,
                tie_stop: b.event.tie_stop,
            },
            temporal_edge,
        });
        issues.push(jump_issue);
    }
}

fn check_staff_register(part: &Part, measure: &crate::model::Measure, note_events: &[&Event], issues: &mut Vec<Issue>) {
    for event in note_events.iter().copied() {
        let Some(pitch) = event.pitch.as_ref() else { continue; };
        if event.staff == "1" && pitch.midi < 48 {
            issues.push(issue(
                false,
                ValidatorIssueKind::RightHandLowRegister,
                part,
                measure,
                Some(event.staff.clone()),
                Some(event.voice.clone()),
                None,
                Some(pitch.midi as i64),
                None,
                format!(
                    "Staff 1 enthält tiefe Note {}. Das kann korrekt sein, sollte aber als Handverteilung geprüft werden.",
                    pitch.label
                ),
                format!(
                    "Takt {} Staff 1 enthält tiefe Note {}. Handverteilung prüfen.",
                    measure.number, pitch.label
                ),
            ));
        }
        if event.staff == "2" && pitch.midi > 72 {
            issues.push(issue(
                false,
                ValidatorIssueKind::LeftHandHighRegister,
                part,
                measure,
                Some(event.staff.clone()),
                Some(event.voice.clone()),
                None,
                Some(pitch.midi as i64),
                None,
                format!(
                    "Staff 2 enthält hohe Note {}. Das kann korrekt sein, sollte aber als Handverteilung geprüft werden.",
                    pitch.label
                ),
                format!(
                    "Takt {} Staff 2 enthält hohe Note {}. Handverteilung prüfen.",
                    measure.number, pitch.label
                ),
            ));
        }
    }
}

fn note_events(measure: &crate::model::Measure) -> impl Iterator<Item = &Event> {
    measure.events.iter().filter(|e| e.kind == EventKind::Note)
}

fn visible_note_events(measure: &crate::model::Measure) -> impl Iterator<Item = &Event> {
    // v0.1.144: Pianistische Spielbarkeitsprüfungen bewerten die gedruckte
    // Klavierfassung. Unsichtbare Stütz-/Akkordtöne mit print-object="no"
    // dürfen nicht als physisch zu spielende Handspanne oder Sprunglinie zählen.
    note_events(measure).filter(|e| e.visible)
}

fn union_length(intervals: &mut Vec<(i64, i64)>) -> i64 {
    intervals.retain(|(a, b)| b > a);
    intervals.sort_unstable();
    let mut total = 0_i64;
    let mut current: Option<(i64, i64)> = None;

    for &(start, end) in intervals.iter() {
        match current {
            None => current = Some((start, end)),
            Some((a, b)) if start <= b => current = Some((a, b.max(end))),
            Some((a, b)) => {
                total += b - a;
                current = Some((start, end));
            }
        }
    }
    if let Some((a, b)) = current {
        total += b - a;
    }
    total
}

#[allow(clippy::too_many_arguments)]
fn issue(
    strict_context: bool,
    issue_kind: ValidatorIssueKind,
    part: &Part,
    measure: &crate::model::Measure,
    staff: Option<String>,
    voice: Option<String>,
    expected_divisions: Option<String>,
    actual_divisions: Option<i64>,
    delta_divisions: Option<String>,
    details: String,
    message_for_ai: String,
) -> Issue {
    let issue_authority = validator_issue_authority(issue_kind);
    let issue_type = issue_kind.as_str();
    let rule_id = issue_authority.rule_id;
    let rule_entry = rule_by_id(rule_id.as_str());
    let severity = runtime_severity_for_issue_kind(issue_kind, strict_context);

    Issue {
        severity,
        issue_type: issue_type.to_string(),
        rule_id: rule_id.as_str().to_string(),
        rule_severity: rule_entry
            .map_or_else(|| "UNMAPPED".to_string(), |rule| rule.severity.to_string()),
        rule_weight: rule_entry.map_or(0, |rule| rule.weight),
        part_id: part.id.clone(),
        part_name: part.name.clone(),
        measure: measure.number.clone(),
        staff,
        voice,
        expected_divisions,
        actual_divisions,
        actual_unit: issue_authority.actual_unit.map(str::to_string),
        delta_divisions,
        han03_jump_identity: None,
        details,
        message_for_ai,
    }
}

fn format_scaled(scaled: i64, denominator: i64) -> String {
    if denominator == 0 {
        return format!("{scaled} divisions");
    }
    if scaled % denominator == 0 {
        format!("{} divisions", scaled / denominator)
    } else {
        format!("{scaled}/{denominator} divisions")
    }
}


#[cfg(test)]
mod han03_attack_semantics_tests {
    use super::*;

    fn event(start: i64, midi: i32, tie_stop: bool) -> Event {
        Event {
            kind: EventKind::Note,
            staff: "2".to_string(),
            voice: "2".to_string(),
            start,
            duration: 48,
            pitch: Some(crate::model::Pitch::new(
                "G".to_string(),
                1,
                (midi / 12) - 1,
            )),
            is_chord: false,
            tie_stop,
            visible: true,
            note_type: Some("eighth".to_string()),
            dots: 0,
        }
    }

    #[test]
    fn tie_stop_continuation_is_not_a_han03_jump_endpoint() {
        let continuation = event(96, 44, true);
        let attack = event(144, 68, false);
        let line = build_extreme_line([&continuation, &attack].into_iter(), true);
        assert_eq!(line.len(), 1);
        assert_eq!(line[0].event.start, 144);
        assert_eq!(line[0].midi, 68);
    }

    #[test]
    fn genuine_attacks_remain_han03_jump_endpoints() {
        let first_attack = event(96, 44, false);
        let second_attack = event(144, 68, false);
        let line = build_extreme_line([&first_attack, &second_attack].into_iter(), true);
        assert_eq!(line.len(), 2);
        assert_eq!(line[0].midi, 44);
        assert_eq!(line[1].midi, 68);
    }

    #[test]
    fn mixed_onset_keeps_new_attack_and_ignores_higher_tie_continuation() {
        let attack = event(96, 56, false);
        let continuation = event(96, 68, true);
        let line = build_extreme_line([&attack, &continuation].into_iter(), true);
        assert_eq!(line.len(), 1);
        assert_eq!(line[0].midi, 56);
    }

    #[test]
    fn canonical_temporal_edge_distinguishes_contiguous_attack_from_gap() {
        let first = event(48, 44, false);
        let contiguous = event(96, 68, false);
        let gapped = event(144, 68, false);

        let direct = canonical_han03_temporal_edge(&first, &contiguous);
        assert_eq!(direct.from_end, 96);
        assert_eq!(direct.onset_delta, 48);
        assert_eq!(direct.release_to_attack_gap, 0);
        assert!(direct.direct_attack_successor);

        let gap = canonical_han03_temporal_edge(&first, &gapped);
        assert_eq!(gap.from_end, 96);
        assert_eq!(gap.onset_delta, 96);
        assert_eq!(gap.release_to_attack_gap, 48);
        assert!(!gap.direct_attack_successor);
    }

    #[test]
    fn canonical_temporal_edge_requires_same_voice_for_direct_successor() {
        let first = event(48, 44, false);
        let mut other_voice = event(96, 68, false);
        other_voice.voice = "1".to_string();

        let edge = canonical_han03_temporal_edge(&first, &other_voice);
        assert_eq!(edge.release_to_attack_gap, 0);
        assert!(!edge.direct_attack_successor);
    }

    fn han03_issues_for_pair(first: &Event, second: &Event) -> Vec<Issue> {
        let part = Part {
            id: "P1".to_string(),
            name: "Piano".to_string(),
            measures: Vec::new(),
        };
        let measure = crate::model::Measure {
            number: "1".to_string(),
            divisions: 96,
            beats: 4,
            beat_type: 4,
            expected: crate::model::ExpectedDuration::new(96, 4, 4),
            cursor_min: 0,
            cursor_max: 384,
            events: Vec::new(),
        };
        let line = build_extreme_line([first, second].into_iter(), true);
        let mut emitted = BTreeSet::new();
        let mut issues = Vec::new();
        check_jump_line(
            JumpLineContext {
                part: &part,
                measure: &measure,
                staff: "2",
                voice: Some("2"),
                line_kind: "voice-high",
            },
            line,
            &mut emitted,
            &mut issues,
        );
        issues
    }

    #[test]
    fn han03_does_not_claim_fast_jump_when_release_precedes_target_attack() {
        let first = event(48, 44, false);
        let second = event(144, 68, false);
        let edge = canonical_han03_temporal_edge(&first, &second);
        assert_eq!(edge.onset_delta, 96);
        assert_eq!(edge.release_to_attack_gap, 48);
        assert!(han03_issues_for_pair(&first, &second).is_empty());
    }

    #[test]
    fn han03_keeps_contiguous_large_jump_warning() {
        let first = event(48, 44, false);
        let second = event(96, 68, false);
        let issues = han03_issues_for_pair(&first, &second);
        assert_eq!(issues.len(), 1);
        let identity = issues[0]
            .han03_jump_identity
            .as_ref()
            .expect("HAN03 warning must carry canonical jump identity");
        assert_eq!(identity.temporal_edge.release_to_attack_gap, 0);
    }

    #[test]
    fn han03_keeps_overlapping_large_jump_warning() {
        let mut first = event(48, 44, false);
        first.duration = 96;
        let second = event(96, 68, false);
        let issues = han03_issues_for_pair(&first, &second);
        assert_eq!(issues.len(), 1);
        let identity = issues[0]
            .han03_jump_identity
            .as_ref()
            .expect("HAN03 warning must carry canonical jump identity");
        assert_eq!(identity.temporal_edge.release_to_attack_gap, -48);
    }
}
