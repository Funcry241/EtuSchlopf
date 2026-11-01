//! Same-input counterfactual metrics, never a musical plan or write permission.
//! The early register proposal/occurrence binding is deliberately UNKNOWN.
use crate::clef_readability_observation::{
    ClefPitchObservation, LowerChordObservation, PianoClef, observe_pitch, temporal::TemporalPart,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::Write as _;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Costs {
    pub(crate) transition: i64,
    pub(crate) redundant: i64,
    pub(crate) short: i64,
    pub(crate) short_attacks: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Clef {
    Bass,
    Treble,
}
impl Clef {
    fn ledger(self, pitch: ClefPitchObservation) -> i64 {
        match self {
            Self::Bass => PianoClef::BassF4,
            Self::Treble => PianoClef::TrebleG2,
        }
        .ledger_lines(pitch)
    }
}
#[derive(Clone, Copy)]
struct Tone {
    at: i128,
    end: i128,
    pitch: ClefPitchObservation,
    shift: i64,
    visible: bool,
    counted: bool,
}
#[derive(Clone, Copy)]
struct Surface {
    at: i128,
    kind: Clef,
    visible: bool,
}
#[derive(Clone, Copy)]
struct OctaveSurface {
    at: i128,
    shift: Option<i64>,
}
struct Context {
    octaves: Vec<OctaveSurface>,
    tones: Vec<Tone>,
    surfaces: Vec<Surface>,
    bounds: Vec<i128>,
    end: i128,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Metrics {
    objects: usize,
    ledger_sum: i64,
    ledger_max: i64,
    pressure: i64,
    transitions: usize,
    redundant: usize,
    short: usize,
    octave_changes: usize,
    octave_reminders: usize,
}
impl Metrics {
    fn total(&self, c: Costs) -> Option<i64> {
        self.pressure
            .checked_add(
                i64::try_from(self.transitions)
                    .ok()?
                    .checked_mul(c.transition)?,
            )?
            .checked_add(
                i64::try_from(self.redundant)
                    .ok()?
                    .checked_mul(c.redundant)?,
            )?
            .checked_add(i64::try_from(self.short).ok()?.checked_mul(c.short)?)
    }
}
#[derive(Clone, Copy, Debug)]
struct Alternative {
    start: i128,
    end: i128,
    clef: Clef,
}
struct Observation {
    baseline: Metrics,
    best: Option<Metrics>,
    compared: usize,
    improving: usize,
}
fn child<'a>(n: roxmltree::Node<'a, 'a>, name: &str) -> Option<&'a str> {
    n.children()
        .find(|c| c.has_tag_name(name))?
        .text()
        .map(str::trim)
}
fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn context(xml: &str) -> Option<Context> {
    let doc = roxmltree::Document::parse(xml).ok()?;
    let part = doc.root_element();
    if !part.has_tag_name("part") {
        return None;
    }
    let temporal = TemporalPart::observe(xml)?;
    let mut end = 0i128;
    let mut unit = 1i128;
    for d in part.descendants().filter(|n| n.has_tag_name("divisions")) {
        let v = d.text()?.trim().parse::<i128>().ok().filter(|v| *v > 0)?;
        unit = unit.checked_div(gcd(unit, v))?.checked_mul(v)?;
    }
    let mut division = 1i128;
    let mut tones = Vec::new();
    let mut surfaces = Vec::new();
    let mut octaves = Vec::new();
    let mut bounds = BTreeSet::from([0]);
    let mut attachment = LowerChordObservation::default();
    let mut previous_end = 0;
    for measure in part.children().filter(|n| n.has_tag_name("measure")) {
        for n in measure.children().filter(|n| n.is_element()) {
            let at = temporal.at(n.range().start)?;
            if n.has_tag_name("attributes") {
                if let Some(v) = child(n, "divisions") {
                    division = v.parse::<i128>().ok().filter(|v| *v > 0)?;
                }
                for clef in n
                    .children()
                    .filter(|c| c.has_tag_name("clef") && c.attribute("number") == Some("2"))
                {
                    if clef
                        .children()
                        .any(|c| c.has_tag_name("clef-octave-change"))
                    {
                        return None;
                    }
                    let kind = match (child(clef, "sign")?, child(clef, "line")?) {
                        ("F", "4") => Clef::Bass,
                        ("G", "2") => Clef::Treble,
                        _ => return None,
                    };
                    surfaces.push(Surface {
                        at,
                        kind,
                        visible: clef.attribute("print-object") != Some("no"),
                    });
                    bounds.insert(at);
                }
            } else if n.has_tag_name("direction") && child(n, "staff") == Some("2") {
                if n.descendants().any(|c| c.has_tag_name("octave-shift")) {
                    // TemporalPart owns offsets and simultaneous-state validation.
                    let offset = child(n, "offset")
                        .unwrap_or("0")
                        .parse::<i128>()
                        .ok()?
                        .checked_mul(unit.checked_div(division)?)?;
                    let pos = at.checked_add(offset)?;
                    let shift=crate::clef_readability_observation::observe_lower_staff_octave_display_shift(&xml[n.range()]);
                    let typ = n
                        .descendants()
                        .find(|c| c.has_tag_name("octave-shift"))?
                        .attribute("type")?;
                    if shift.is_none() && typ != "continue" {
                        return None;
                    }
                    octaves.push(OctaveSurface { at: pos, shift });
                }
            } else if n.has_tag_name("forward") {
                let duration = child(n, "duration")?
                    .parse::<i128>()
                    .ok()?
                    .checked_mul(unit.checked_div(division)?)?;
                if duration < 0 {
                    return None;
                }
                end = end.max(at.checked_add(duration)?);
            } else if n.has_tag_name("note") {
                let duration = child(n, "duration")?
                    .parse::<i128>()
                    .ok()?
                    .checked_mul(unit.checked_div(division)?)?;
                if duration <= 0 {
                    return None;
                }
                end = end.max(at.checked_add(duration)?);
                let raw = &xml[n.range()];
                let lower = child(n, "staff") == Some("2");
                let pitch = lower.then(|| observe_pitch(raw)).flatten();
                let visible = n.attribute("print-object") != Some("no");
                let relation = attachment.observe(
                    &xml[previous_end..n.range().start],
                    n.range().start,
                    raw,
                    visible && pitch.is_some(),
                );
                previous_end = n.range().end;
                if let Some(pitch) = pitch {
                    tones.push(Tone {
                        at,
                        end: at.checked_add(duration)?,
                        pitch,
                        shift: temporal.shift(n.range().start)?,
                        visible,
                        counted: relation.counted(),
                    });
                }
            }
        }
    }
    surfaces.sort_by_key(|s| s.at);
    tones.sort_by_key(|t| t.at);
    if surfaces.is_empty() || tones.is_empty() || surfaces[0].at != 0 || end <= 0 {
        return None;
    }
    octaves.sort_by_key(|s| s.at);
    let mut active_shift = 0;
    for s in &octaves {
        if let Some(shift) = s.shift {
            if shift != active_shift {
                bounds.insert(s.at);
            }
            active_shift = shift;
        }
    }
    bounds.insert(end);
    if bounds.iter().any(|b| *b < 0 || *b > end) {
        return None;
    }
    Some(Context {
        octaves,
        tones,
        surfaces,
        bounds: bounds.into_iter().collect(),
        end,
    })
}
fn original_at(c: &Context, at: i128) -> Option<Clef> {
    c.surfaces.iter().rev().find(|s| s.at <= at).map(|s| s.kind)
}
fn metric(c: &Context, costs: Costs, alt: Option<Alternative>) -> Option<Metrics> {
    let mut result = Metrics::default();
    let original_shift = |at: i128| {
        c.octaves
            .iter()
            .rev()
            .find(|s| s.at <= at && s.shift.is_some())
            .and_then(|s| s.shift)
            .unwrap_or(0)
    };
    let mut shifts = BTreeMap::<i128, Vec<Option<i64>>>::new();
    for s in &c.octaves {
        if alt.is_none_or(|a| s.at < a.start || s.at >= a.end) {
            shifts.entry(s.at).or_default().push(s.shift);
        }
    }
    if let Some(a) = alt {
        shifts.insert(a.start, vec![Some(0)]);
        if a.end < c.end && !shifts.contains_key(&a.end) {
            shifts.insert(a.end, vec![Some(original_shift(a.end))]);
        }
    }
    let mut previous_shift = 0;
    for at in shifts.values() {
        for shift in at {
            if let Some(shift) = shift {
                result.octave_changes += usize::from(*shift != previous_shift);
                previous_shift = *shift;
            } else {
                result.octave_reminders += 1;
            }
        }
    }
    let mut events = BTreeMap::<i128, Vec<Surface>>::new();
    for s in &c.surfaces {
        if alt.is_none_or(|a| s.at < a.start || s.at >= a.end) {
            events.entry(s.at).or_default().push(*s);
        }
    }
    if let Some(a) = alt {
        let incoming = c
            .surfaces
            .iter()
            .rev()
            .find(|s| s.at < a.start)
            .map(|s| s.kind);
        if a.start == 0 || incoming != Some(a.clef) {
            events.insert(
                a.start,
                vec![Surface {
                    at: a.start,
                    kind: a.clef,
                    visible: true,
                }],
            );
        }
        // At the closed right edge restore the incumbent state, but retain real
        // surfaces there verbatim rather than inventing an extra reminder.
        if a.end < c.end && !events.contains_key(&a.end) && original_at(c, a.end)? != a.clef {
            events.insert(
                a.end,
                vec![Surface {
                    at: a.end,
                    kind: original_at(c, a.end)?,
                    visible: true,
                }],
            );
        }
    }
    let points = events.keys().copied().collect::<Vec<_>>();
    let mut state = None;
    let mut origin = None;
    let mut attacks = 0usize;
    for (i, at) in points.iter().copied().enumerate() {
        for s in &events[&at] {
            if let Some(old) = state {
                if !s.visible {
                    origin = None;
                    attacks = 0;
                } else if old == s.kind {
                    result.redundant += 1;
                } else {
                    result.transitions += 1;
                    if origin == Some(s.kind) && attacks <= costs.short_attacks {
                        result.short += 1;
                        origin = None;
                        attacks = 0;
                    } else {
                        origin = Some(old);
                        attacks = 0;
                    }
                }
            }
            state = Some(s.kind);
        }
        let stop = points.get(i + 1).copied().unwrap_or(c.end);
        let active = state?;
        for t in c
            .tones
            .iter()
            .filter(|t| t.at >= at && t.at < stop && t.visible)
        {
            let shifted = if alt.is_some_and(|a| t.at >= a.start && t.at < a.end) {
                t.pitch
            } else {
                t.pitch.shifted(t.shift)
            };
            let ledger = active.ledger(shifted);
            result.objects += 1;
            result.ledger_sum = result.ledger_sum.checked_add(ledger)?;
            result.ledger_max = result.ledger_max.max(ledger);
            let pressure = ledger
                .checked_mul(ledger)?
                .checked_add((ledger - 3).max(0).checked_mul(8)?)?;
            result.pressure = result.pressure.checked_add(pressure)?;
            if origin.is_some() && t.counted {
                attacks = attacks
                    .saturating_add(1)
                    .min(costs.short_attacks.saturating_add(1));
            }
        }
    }
    Some(result)
}
fn observe(c: &Context, costs: Costs) -> Option<Observation> {
    if [costs.transition, costs.redundant, costs.short]
        .iter()
        .any(|x| *x < 0)
    {
        return None;
    }
    let baseline = metric(c, costs, None)?;
    let total = baseline.total(costs)?;
    let mut best: Option<Metrics> = None;
    let mut compared = 0;
    let mut improving = 0;
    for (i, &start) in c.bounds.iter().enumerate() {
        for &end in c.bounds.iter().skip(i + 1) {
            // These are representation counterfactuals, not valid write scopes.
            // Do not even measure a mapping cut through a held lower note.
            if c.tones
                .iter()
                .any(|t| (t.at < start && start < t.end) || (t.at < end && end < t.end))
            {
                continue;
            }
            for clef in [Clef::Bass, Clef::Treble] {
                let m = metric(c, costs, Some(Alternative { start, end, clef }))?;
                if m.objects != baseline.objects {
                    return None;
                }
                compared += 1;
                let candidate_total = m.total(costs)?;
                if candidate_total < total {
                    improving += 1;
                    // All alternatives here replace this same complete part
                    // course. No ranking ever crosses independent parts/calls.
                    if best
                        .as_ref()
                        .is_none_or(|b| candidate_total < b.total(costs).unwrap_or(i64::MIN))
                    {
                        best = Some(m);
                    }
                }
            }
        }
    }
    Some(Observation {
        baseline,
        best,
        compared,
        improving,
    })
}
pub(crate) fn report(xml: &str, costs: Costs) -> String {
    let hash = crate::sha256::hex(xml.as_bytes());
    let mut out = format!(
        "JointRegisterReadingBegin: Schema=1; InputSha256={hash}; SourceRole=actual_clef_planner_input; EarlyRegisterProposalBinding=UNKNOWN; ClassMembership=UNKNOWN; Feasibility=NOT_PROVEN; Authority=none\n"
    );
    let parsed = roxmltree::Document::parse(xml);
    if let Ok(doc) = parsed {
        let parts = doc
            .root_element()
            .children()
            .filter(|p| p.has_tag_name("part"))
            .collect::<Vec<_>>();
        if parts.is_empty() {
            out.push_str("JointRegisterReading: Status=UNAVAILABLE; Reason=missing_part\n");
        }
        for (index, part) in parts.iter().enumerate() {
            let row = context(&xml[part.range()]).and_then(|c| observe(&c, costs));
            if let Some(row) = row {
                let _ = writeln!(
                    out,
                    "JointRegisterReading: PartOrdinal={}; Objects={}; BaselineCost={:?}; BaselineMetrics={:?}; Compared={}; StrictlyLowerCounterfactuals={}; LowestCounterfactualCost={:?}; LowestCounterfactualMetrics={:?}; MetricOnly=true; MusicalDecision=none",
                    index + 1,
                    row.baseline.objects,
                    row.baseline.total(costs),
                    row.baseline,
                    row.compared,
                    row.improving,
                    row.best.as_ref().and_then(|m| m.total(costs)),
                    row.best
                );
            } else {
                let _ = writeln!(
                    out,
                    "JointRegisterReading: PartOrdinal={}; Status=UNAVAILABLE; Reason=unsupported_or_incomplete_context",
                    index + 1
                );
            }
        }
    } else {
        out.push_str("JointRegisterReading: Status=UNAVAILABLE; Reason=invalid_xml\n");
    }
    let _ = writeln!(out, "JointRegisterReadingEnd: InputSha256={hash}");
    out
}
pub(crate) fn emit(xml: &str, costs: Costs) {
    // An observational write to the already existing diagnostic sink. No state
    // returned here can be consumed by the musical planner or materializer.
    let _ = std::io::stderr()
        .lock()
        .write_all(report(xml, costs).as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    const COSTS: Costs = Costs {
        transition: 24,
        redundant: 1,
        short: 24,
        short_attacks: 2,
    };
    fn note(step: &str, octave: i32, duration: i32) -> String {
        format!(
            "<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>7</voice><staff>2</staff></note>"
        )
    }
    fn clef(kind: &str) -> String {
        let (sign, line) = if kind == "bass" { ("F", 4) } else { ("G", 2) };
        format!(
            "<attributes><clef number=\"2\"><sign>{sign}</sign><line>{line}</line></clef></attributes>"
        )
    }
    fn octave(kind: &str, size: i32) -> String {
        format!(
            "<direction><direction-type><octave-shift type=\"{kind}\" size=\"{size}\"/></direction-type><staff>2</staff></direction>"
        )
    }
    fn part(body: &str) -> String {
        format!(
            "<part id=\"sample\"><measure number=\"alpha\"><attributes><divisions>1</divisions></attributes>{body}</measure></part>"
        )
    }
    fn stable() -> String {
        part(&(clef("treble") + &note("G", 4, 1) + &note("A", 4, 1) + &note("B", 4, 1)))
    }
    fn fragmented() -> String {
        part(
            &(clef("bass")
                + &note("D", 4, 1)
                + &clef("treble")
                + &note("D", 4, 1)
                + &octave("down", 8)
                + &note("D", 5, 1)
                + &octave("stop", 8)
                + &clef("bass")
                + &note("D", 4, 1)
                + &clef("treble")
                + &note("D", 4, 1)),
        )
    }
    #[test]
    fn full_context_baseline_and_counterfactual_use_same_objects() {
        let c = context(&fragmented()).unwrap();
        let o = observe(&c, COSTS).unwrap();
        assert!(o.improving > 0);
        assert_eq!(o.baseline.objects, 5);
        assert_eq!(o.best.as_ref().unwrap().objects, o.baseline.objects);
        assert!(o.best.unwrap().total(COSTS).unwrap() < o.baseline.total(COSTS).unwrap());
    }
    #[test]
    fn ordinary_stable_course_is_not_strictly_improved() {
        let c = context(&stable()).unwrap();
        let o = observe(&c, COSTS).unwrap();
        assert_eq!(o.improving, 0);
        assert!(o.best.is_none());
    }
    #[test]
    fn time_scaling_and_label_changes_preserve_costs() {
        let source = fragmented();
        let scaled = source
            .replace("<divisions>1</divisions>", "<divisions>3</divisions>")
            .replace("<duration>1</duration>", "<duration>3</duration>")
            .replace("sample", "renamed")
            .replace("alpha", "unrelated")
            .replace("<voice>7</voice>", "<voice>other</voice>");
        let a = observe(&context(&source).unwrap(), COSTS).unwrap();
        let b = observe(&context(&scaled).unwrap(), COSTS).unwrap();
        assert_eq!(a.baseline, b.baseline);
        assert_eq!(a.best, b.best);
        assert_eq!(a.improving, b.improving);
    }
    #[test]
    fn independent_parts_do_not_choose_a_global_winner() {
        let p = fragmented();
        let one = format!("<score-partwise>{p}</score-partwise>");
        let two = format!(
            "<score-partwise>{p}{}</score-partwise>",
            p.replace("sample", "duplicate")
        );
        let a = report(&one, COSTS);
        let b = report(&two, COSTS);
        let first = a
            .lines()
            .find(|l| l.starts_with("JointRegisterReading: PartOrdinal=1"))
            .unwrap();
        assert!(b.contains(first));
        assert_eq!(b.matches("MetricOnly=true").count(), 2);
    }
    #[test]
    fn malformed_and_unsupported_context_stays_unavailable() {
        for xml in [
            "<broken".to_owned(),
            "<score-partwise/>".to_owned(),
            format!(
                "<score-partwise>{}</score-partwise>",
                stable().replace("<sign>G</sign>", "<sign>C</sign>")
            ),
        ] {
            let r = report(&xml, COSTS);
            assert!(r.contains("UNAVAILABLE"));
            assert!(r.contains("ClassMembership=UNKNOWN"));
            assert!(r.contains("Authority=none"));
        }
    }
    #[test]
    fn required_high_register_can_make_loco_alternative_worse() {
        let xml = part(
            &(clef("treble")
                + &octave("down", 15)
                + &note("C", 8, 1)
                + &note("D", 8, 1)
                + &octave("stop", 15)),
        );
        let c = context(&xml).unwrap();
        let a = metric(&c, COSTS, None).unwrap();
        let b = metric(
            &c,
            COSTS,
            Some(Alternative {
                start: 0,
                end: c.end,
                clef: Clef::Treble,
            }),
        )
        .unwrap();
        assert!(b.ledger_max > a.ledger_max);
        assert!(b.total(COSTS) > a.total(COSTS));
        assert!(observe(&c, COSTS).unwrap().best.is_none());
    }
    #[test]
    fn full_context_includes_low_margin_and_following_carry() {
        let xml = part(
            &(clef("bass")
                + &note("C", 2, 1)
                + &clef("treble")
                + &note("G", 4, 1)
                + &note("A", 4, 1)
                + &clef("bass")
                + &note("D", 2, 1)),
        );
        let c = context(&xml).unwrap();
        let a = metric(&c, COSTS, None).unwrap();
        let b = metric(
            &c,
            COSTS,
            Some(Alternative {
                start: 1,
                end: 3,
                clef: Clef::Treble,
            }),
        )
        .unwrap();
        assert_eq!(a, b);
        assert_eq!(b.objects, 4);
    }
    #[test]
    fn hidden_pitched_objects_protect_cut_boundaries_but_are_not_reading_heads() {
        let xml = part(
            &(clef("treble")
                + &note("G", 4, 4).replace("<note>", "<note print-object=\"no\">")
                + "<backup><duration>3</duration></backup>"
                + &clef("bass")
                + &note("D", 3, 3)),
        );
        let c = context(&xml).unwrap();
        assert_eq!(metric(&c, COSTS, None).unwrap().objects, 1);
        assert!(c.tones.iter().any(|t| !t.visible && t.at < 1 && 1 < t.end));
    }
    #[test]
    fn chords_and_ties_are_not_extra_musical_mutations() {
        let xml = part(
            &(clef("treble")
                + &note("G", 4, 2).replace("</note>", "<tie type=\"start\"/></note>")
                + &note("B", 4, 2).replace("<note>", "<note><chord/>")
                + &note("G", 4, 1).replace("</note>", "<tie type=\"stop\"/></note>")),
        );
        let original = xml.clone();
        let c = context(&xml).unwrap();
        let o = observe(&c, COSTS).unwrap();
        assert_eq!(o.baseline.objects, 3);
        assert_eq!(xml, original);
    }
    #[test]
    fn changing_divisions_and_offsets_use_existing_temporal_units() {
        let a = part(
            &(clef("treble")
                + &octave("down", 8)
                + &note("G", 5, 2)
                + &octave("stop", 8)
                + &note("A", 4, 2)),
        );
        let b = a
            .replace("<divisions>1</divisions>", "<divisions>2</divisions>")
            .replace("<duration>2</duration>", "<duration>4</duration>");
        assert_eq!(
            metric(&context(&a).unwrap(), COSTS, None),
            metric(&context(&b).unwrap(), COSTS, None)
        );
    }
    #[test]
    fn same_pitch_different_spelling_uses_diatonic_ledgers() {
        let a = part(
            &(clef("treble") + &note("C", 4, 1).replace("<octave>", "<alter>-1</alter><octave>")),
        );
        let b = part(&(clef("treble") + &note("B", 3, 1)));
        let x = metric(&context(&a).unwrap(), COSTS, None).unwrap();
        let y = metric(&context(&b).unwrap(), COSTS, None).unwrap();
        assert_eq!(x.ledger_sum, 1);
        assert_eq!(y.ledger_sum, 1);
        assert_ne!(
            context(&a).unwrap().tones[0].pitch,
            context(&b).unwrap().tones[0].pitch
        );
    }
    #[test]
    fn input_hash_and_unknown_authority_survive_diagnostic_report() {
        let xml = format!("<score-partwise>{}</score-partwise>", fragmented());
        let before = xml.clone();
        let r = report(&xml, COSTS);
        assert!(r.contains(&crate::sha256::hex(xml.as_bytes())));
        assert!(r.contains("EarlyRegisterProposalBinding=UNKNOWN"));
        assert!(r.contains("Feasibility=NOT_PROVEN"));
        assert!(r.contains("MusicalDecision=none"));
        assert_eq!(xml, before);
    }
}
