//! Final-document notation measurements. No musical selection, mutation or release authority.
use roxmltree::{Document, Node};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub(crate) const SECTION: &str = "NOTATION-INVARIANT-BALANCE";
pub(crate) const SECTION_RULE: &str = "------------------------";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Decimal {
    numerator: i128,
    denominator: i128,
}

fn greatest_common_divisor(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn decimal(value: &str) -> Result<Decimal, String> {
    let value = value.trim();
    let unsigned = value.strip_prefix(['-', '+']).unwrap_or(value);
    let (whole, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    if (whole.is_empty() && fraction.is_empty())
        || !whole.bytes().all(|b| b.is_ascii_digit())
        || !fraction.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(format!("invalid decimal {value:?}"));
    }
    let denominator = 10_i128
        .checked_pow(u32::try_from(fraction.len()).map_err(|_| "decimal length")?)
        .ok_or("decimal scale overflow")?;
    let whole: i128 = if whole.is_empty() {
        0
    } else {
        whole.parse().map_err(|_| "decimal magnitude")?
    };
    let tail: i128 = if fraction.is_empty() {
        0
    } else {
        fraction.parse().map_err(|_| "decimal fraction")?
    };
    let mut numerator = whole
        .checked_mul(denominator)
        .and_then(|n| n.checked_add(tail))
        .ok_or("decimal overflow")?;
    if value.starts_with('-') {
        numerator = -numerator;
    }
    let divisor = greatest_common_divisor(numerator.abs(), denominator);
    Ok(Decimal {
        numerator: numerator / divisor,
        denominator: denominator / divisor,
    })
}

fn least_common_multiple(a: i128, b: i128) -> Result<i128, String> {
    (a / greatest_common_divisor(a, b))
        .checked_mul(b)
        .ok_or_else(|| "time resolution overflow".to_string())
}

fn child_text<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    node.children()
        .find(|n| n.has_tag_name(name))
        .and_then(|n| n.text())
        .map(str::trim)
}

fn required_text<'a>(node: Node<'a, '_>, name: &str) -> Result<&'a str, String> {
    child_text(node, name).ok_or_else(|| format!("missing {name}"))
}

fn has_child(node: Node<'_, '_>, name: &str) -> bool {
    node.children().any(|n| n.has_tag_name(name))
}

fn integer(value: &str) -> Result<i128, String> {
    value
        .parse()
        .map_err(|_| format!("invalid integer {value:?}"))
}

fn sum_time(a: i128, b: i128) -> Result<i128, String> {
    a.checked_add(b).ok_or_else(|| "time overflow".to_string())
}

// Common exact quarter-note ticks, including decimal durations/offsets and meter denominators.
// An excessive rational resolution is reported unavailable; it is never rounded.
fn time_resolution(doc: &Document<'_>) -> Result<i128, String> {
    let (mut divisions, mut decimals, mut meters) = (1, 1, 1);
    for node in doc.descendants().filter(Node::is_element) {
        match node.tag_name().name() {
            "divisions" => {
                let d = decimal(node.text().ok_or("empty divisions")?)?;
                if d.numerator <= 0 {
                    return Err("nonpositive divisions".to_string());
                }
                divisions = least_common_multiple(divisions, d.numerator)?;
            }
            "duration" | "offset" => {
                decimals = least_common_multiple(
                    decimals,
                    decimal(node.text().ok_or("empty time value")?)?.denominator,
                )?;
            }
            "beat-type" => {
                let d = integer(node.text().ok_or("empty beat-type")?.trim())?;
                if d <= 0 {
                    return Err("nonpositive beat-type".to_string());
                }
                meters = least_common_multiple(meters, d)?;
            }
            _ => {}
        }
    }
    divisions
        .checked_mul(decimals)
        .and_then(|v| v.checked_mul(meters))
        .ok_or_else(|| "time resolution overflow".to_string())
}

fn duration_ticks(value: &str, divisions: Decimal, resolution: i128) -> Result<i128, String> {
    let d = decimal(value)?;
    let denominator = d
        .denominator
        .checked_mul(divisions.numerator)
        .ok_or("time denominator overflow")?;
    if resolution % denominator != 0 {
        return Err("nonintegral time resolution".to_string());
    }
    d.numerator
        .checked_mul(divisions.denominator)
        .and_then(|v| v.checked_mul(resolution / denominator))
        .ok_or_else(|| "time magnitude overflow".to_string())
}

fn meter_ticks(time: Node<'_, '_>, resolution: i128) -> Result<i128, String> {
    if time.attribute("number").is_some() || has_child(time, "senza-misura") {
        return Err("staff-specific or unmetered time unsupported".to_string());
    }
    let mut beats: Option<i128> = None;
    let mut total = 0;
    for node in time.children().filter(Node::is_element) {
        match node.tag_name().name() {
            "beats" => {
                if beats.is_some() {
                    return Err("unpaired beats".to_string());
                }
                let mut count = 0;
                for term in node.text().ok_or("empty beats")?.split('+') {
                    let term = integer(term.trim())?;
                    if term <= 0 {
                        return Err("nonpositive beats".to_string());
                    }
                    count = sum_time(count, term)?;
                }
                beats = Some(count);
            }
            "beat-type" => {
                let denominator = integer(node.text().ok_or("empty beat-type")?.trim())?;
                let count = beats.take().ok_or("beat-type without beats")?;
                let value = count
                    .checked_mul(4)
                    .and_then(|v| v.checked_mul(resolution / denominator))
                    .ok_or("meter overflow")?;
                total = sum_time(total, value)?;
            }
            _ => {}
        }
    }
    if total <= 0 || beats.is_some() {
        return Err("incomplete meter".to_string());
    }
    Ok(total)
}

#[derive(Clone, Debug)]
struct Place {
    part: usize,
    measure: usize,
    label: String,
    staff: String,
    at: i128,
    local: i128,
    order: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct ClefValue {
    sign: String,
    line: Option<String>,
    octave: i128,
}
#[derive(Clone, Debug)]
enum SurfaceKind {
    Clef(ClefValue),
    OctaveStart,
    OctaveStop,
}
#[derive(Clone, Debug)]
struct Surface {
    place: Place,
    kind: SurfaceKind,
}
#[derive(Clone, Debug)]
struct Tone {
    place: Place,
    duration: i128,
    pitch: Decimal,
    voice: String,
    visible: bool,
    tie_start: bool,
    tie_stop: bool,
}
#[derive(Debug)]
struct Notation {
    resolution: i128,
    tones: Vec<Tone>,
    surfaces: Vec<Surface>,
    ends: BTreeMap<usize, i128>,
    excluded_grace: usize,
}

fn pitch_value(pitch: Node<'_, '_>) -> Result<Decimal, String> {
    let step = match required_text(pitch, "step")? {
        "C" => 0,
        "D" => 2,
        "E" => 4,
        "F" => 5,
        "G" => 7,
        "A" => 9,
        "B" => 11,
        _ => return Err("invalid pitch step".to_string()),
    };
    let octave = integer(required_text(pitch, "octave")?)?;
    let alter = decimal(child_text(pitch, "alter").unwrap_or("0"))?;
    let base = octave
        .checked_add(1)
        .and_then(|v| v.checked_mul(12))
        .and_then(|v| v.checked_add(step))
        .ok_or("pitch overflow")?;
    let numerator = base
        .checked_mul(alter.denominator)
        .and_then(|v| v.checked_add(alter.numerator))
        .ok_or("pitch overflow")?;
    // alter is normalized, adding an integer preserves its reduced denominator.
    Ok(Decimal {
        numerator,
        denominator: alter.denominator,
    })
}

fn collect_notation(xml: &str) -> Result<Notation, String> {
    let doc = Document::parse(xml).map_err(|e| format!("XML: {e}"))?;
    if !doc.root_element().has_tag_name("score-partwise") {
        return Err("expected score-partwise".to_string());
    }
    let resolution = time_resolution(&doc)?;
    let mut notation = Notation {
        resolution,
        tones: Vec::new(),
        surfaces: Vec::new(),
        ends: BTreeMap::new(),
        excluded_grace: 0,
    };
    let parts: Vec<_> = doc
        .root_element()
        .children()
        .filter(|n| n.has_tag_name("part"))
        .collect();
    if parts.is_empty() {
        return Err("no parts".to_string());
    }
    for (part, node) in parts.into_iter().enumerate() {
        let (mut start, mut meter) = (0, None);
        let mut divisions = None;
        for (measure, bar) in node
            .children()
            .filter(|n| n.has_tag_name("measure"))
            .enumerate()
        {
            let (mut cursor, mut maximum) = (0, 0);
            let mut previous_onset = None;
            for el in bar.children().filter(Node::is_element) {
                let place = |staff: String, local: i128| -> Result<Place, String> {
                    let at = sum_time(start, local)?;
                    if at < 0 {
                        return Err("negative absolute time".to_string());
                    }
                    Ok(Place {
                        part,
                        measure,
                        label: bar.attribute("number").unwrap_or("?").to_string(),
                        staff,
                        at,
                        local,
                        order: el.range().start,
                    })
                };
                match el.tag_name().name() {
                    "attributes" => {
                        if let Some(d) = child_text(el, "divisions") {
                            divisions = Some(decimal(d)?);
                        }
                        for time in el.children().filter(|n| n.has_tag_name("time")) {
                            meter = Some(meter_ticks(time, resolution)?);
                        }
                        for clef in el.children().filter(|n| n.has_tag_name("clef")) {
                            let mut p =
                                place(clef.attribute("number").unwrap_or("1").to_string(), cursor)?;
                            p.order = clef.range().start;
                            notation.surfaces.push(Surface {
                                place: p,
                                kind: SurfaceKind::Clef(ClefValue {
                                    sign: required_text(clef, "sign")?.to_string(),
                                    line: child_text(clef, "line").map(str::to_string),
                                    octave: integer(
                                        child_text(clef, "clef-octave-change").unwrap_or("0"),
                                    )?,
                                }),
                            });
                        }
                    }
                    "backup" | "forward" => {
                        let duration = duration_ticks(
                            required_text(el, "duration")?,
                            divisions.ok_or("missing divisions")?,
                            resolution,
                        )?;
                        if duration < 0 {
                            return Err("negative movement duration".to_string());
                        }
                        cursor = sum_time(
                            cursor,
                            if el.has_tag_name("backup") {
                                -duration
                            } else {
                                duration
                            },
                        )?;
                        if cursor < 0 {
                            return Err("backup before measure start".to_string());
                        }
                        maximum = maximum.max(cursor);
                        previous_onset = None;
                    }
                    "direction" => {
                        let offset = child_text(el, "offset")
                            .map(|v| {
                                duration_ticks(v, divisions.ok_or("missing divisions")?, resolution)
                            })
                            .transpose()?
                            .unwrap_or(0);
                        let local = sum_time(cursor, offset)?;
                        for octave in el
                            .children()
                            .filter(|n| n.has_tag_name("direction-type"))
                            .flat_map(|n| n.children())
                            .filter(|n| n.has_tag_name("octave-shift"))
                        {
                            let kind = match octave.attribute("type") {
                                Some("up" | "down") => SurfaceKind::OctaveStart,
                                Some("stop") => SurfaceKind::OctaveStop,
                                Some("continue") => continue,
                                _ => return Err("invalid octave-shift type".to_string()),
                            };
                            let mut p =
                                place(child_text(el, "staff").unwrap_or("1").to_string(), local)?;
                            p.order = octave.range().start;
                            notation.surfaces.push(Surface { place: p, kind });
                        }
                    }
                    "note" => {
                        if has_child(el, "grace") {
                            notation.excluded_grace += 1;
                            continue;
                        }
                        let duration = duration_ticks(
                            required_text(el, "duration")?,
                            divisions.ok_or("missing divisions")?,
                            resolution,
                        )?;
                        if duration <= 0 {
                            return Err("nonpositive measured note duration".to_string());
                        }
                        let chord = has_child(el, "chord");
                        let onset = if chord {
                            previous_onset.ok_or("chord without preceding note")?
                        } else {
                            cursor
                        };
                        if let Some(pitch) = el.children().find(|n| n.has_tag_name("pitch")) {
                            let ties: BTreeSet<_> = el
                                .children()
                                .filter(|n| n.has_tag_name("tie"))
                                .filter_map(|n| n.attribute("type"))
                                .collect();
                            notation.tones.push(Tone {
                                place: place(
                                    child_text(el, "staff").unwrap_or("1").to_string(),
                                    onset,
                                )?,
                                duration,
                                pitch: pitch_value(pitch)?,
                                voice: child_text(el, "voice").unwrap_or("1").to_string(),
                                visible: el.attribute("print-object") != Some("no"),
                                tie_start: ties.contains("start"),
                                tie_stop: ties.contains("stop"),
                            });
                        }
                        maximum = maximum.max(sum_time(onset, duration)?);
                        if !chord {
                            previous_onset = Some(onset);
                            cursor = sum_time(cursor, duration)?;
                        }
                    }
                    _ => {}
                }
            }
            let length = if bar.attribute("implicit") == Some("yes") {
                maximum
            } else {
                maximum.max(meter.unwrap_or(0))
            };
            if length <= 0 {
                return Err("unknown measure length".to_string());
            }
            start = sum_time(start, length)?;
        }
        notation.ends.insert(part, start);
    }
    notation.tones.sort_by_key(|t| (t.place.at, t.place.order));
    notation
        .surfaces
        .sort_by_key(|s| (s.place.at, s.place.order));
    Ok(notation)
}

#[derive(Debug)]
struct Finding {
    rule: usize,
    reason: &'static str,
    place: Place,
    voice: Option<String>,
    pitch: Option<Decimal>,
}
#[derive(Debug, PartialEq, Eq)]
struct Demand {
    part: usize,
    measure: usize,
    label: String,
    staff: String,
    all: usize,
    visible: usize,
    used_all: usize,
    used_visible: usize,
}
#[derive(Debug, Default)]
struct Balance {
    counts: [usize; 5],
    findings: Vec<Finding>,
    demands: Vec<Demand>,
    hidden: usize,
    with_partner: usize,
    without_partner: usize,
    excluded_grace: usize,
    resolution: i128,
}

fn add_finding(
    balance: &mut Balance,
    rule: usize,
    reason: &'static str,
    place: &Place,
    tone: Option<&Tone>,
) {
    balance.counts[rule - 1] += 1;
    balance.findings.push(Finding {
        rule,
        reason,
        place: place.clone(),
        voice: tone.map(|t| t.voice.clone()),
        pitch: tone.map(|t| t.pitch),
    });
}

fn measure_rules(notation: &Notation, balance: &mut Balance) {
    let mut surfaces: BTreeMap<(usize, &str), Vec<&Surface>> = BTreeMap::new();
    let mut heads: BTreeMap<(usize, &str), BTreeSet<i128>> = BTreeMap::new();
    for surface in &notation.surfaces {
        surfaces
            .entry((surface.place.part, &surface.place.staff))
            .or_default()
            .push(surface);
    }
    for tone in notation.tones.iter().filter(|t| t.visible) {
        heads
            .entry((tone.place.part, &tone.place.staff))
            .or_default()
            .insert(tone.place.at);
    }
    for (key, events) in surfaces {
        let head_times = heads.get(&key);
        let mut clef: Option<(&ClefValue, &Place)> = None;
        let mut octave: Option<&Place> = None;
        for event in events {
            match &event.kind {
                SurfaceKind::Clef(value) => {
                    if let Some((previous, place)) = clef {
                        if previous == value {
                            add_finding(balance, 1, "redundant_clef", &event.place, None);
                            continue;
                        }
                        if !head_times.is_some_and(|times| {
                            times.range(place.at..event.place.at).next().is_some()
                        }) {
                            add_finding(
                                balance,
                                2,
                                "clef_section_without_visible_head",
                                place,
                                None,
                            );
                        }
                    }
                    clef = Some((value, &event.place));
                }
                SurfaceKind::OctaveStart => {
                    if let Some(place) = octave {
                        add_finding(balance, 3, "octave_start_before_stop", place, None);
                    }
                    octave = Some(&event.place);
                }
                SurfaceKind::OctaveStop => {
                    if let Some(place) = octave {
                        if !head_times.is_some_and(|times| {
                            times.range(place.at..=event.place.at).next().is_some()
                        }) {
                            add_finding(
                                balance,
                                4,
                                "octave_section_without_visible_head",
                                place,
                                None,
                            );
                        }
                    } else {
                        add_finding(balance, 3, "octave_stop_without_start", &event.place, None);
                    }
                    octave = None;
                }
            }
        }
        if let Some((_, place)) = clef {
            let end = notation.ends[&place.part];
            if !head_times.is_some_and(|times| times.range(place.at..end).next().is_some()) {
                add_finding(balance, 2, "clef_section_without_visible_head", place, None);
            }
        }
        if let Some(place) = octave {
            add_finding(balance, 3, "octave_open_at_end", place, None);
        }
    }
    let mut ties: BTreeMap<(usize, &str, &str, Decimal), &Tone> = BTreeMap::new();
    for tone in &notation.tones {
        let key = (
            tone.place.part,
            tone.place.staff.as_str(),
            tone.voice.as_str(),
            tone.pitch,
        );
        if tone.tie_stop {
            if ties.remove(&key).is_none() {
                add_finding(
                    balance,
                    5,
                    "tie_stop_without_start",
                    &tone.place,
                    Some(tone),
                );
            }
        } else if ties.remove(&key).is_some() {
            add_finding(
                balance,
                5,
                "new_attack_before_tie_stop",
                &tone.place,
                Some(tone),
            );
        }
        if tone.tie_start {
            ties.insert(key, tone);
        }
    }
    for tone in ties.into_values() {
        add_finding(balance, 5, "tie_open_at_end", &tone.place, Some(tone));
    }
    balance
        .findings
        .sort_by_key(|f| (f.place.part, f.place.at, f.place.order, f.rule));
}

fn maximum_overlap(events: &BTreeSet<(i128, i128)>) -> Result<usize, String> {
    let mut points = Vec::with_capacity(events.len() * 2);
    for &(start, duration) in events {
        points.push((start, 1_i64));
        points.push((sum_time(start, duration)?, -1_i64));
    }
    points.sort_unstable(); // -1 (end) precedes +1 (begin) at the same instant.
    let (mut count, mut maximum) = (0_i64, 0_i64);
    for (_, delta) in points {
        count += delta;
        maximum = maximum.max(count);
    }
    usize::try_from(maximum).map_err(|_| "overlap count overflow".to_string())
}

fn measure_chords(notation: &Notation, balance: &mut Balance) -> Result<(), String> {
    let mut rows: BTreeMap<(usize, usize, &str), Vec<&Tone>> = BTreeMap::new();
    for tone in &notation.tones {
        rows.entry((tone.place.part, tone.place.measure, &tone.place.staff))
            .or_default()
            .push(tone);
    }
    for ((part, measure, staff), tones) in rows {
        let all: BTreeSet<_> = tones.iter().map(|t| (t.place.local, t.duration)).collect();
        let visible: BTreeSet<_> = tones
            .iter()
            .filter(|t| t.visible)
            .map(|t| (t.place.local, t.duration))
            .collect();
        let used_all: BTreeSet<_> = tones.iter().map(|t| &t.voice).collect();
        let used_visible: BTreeSet<_> = tones
            .iter()
            .filter(|t| t.visible)
            .map(|t| &t.voice)
            .collect();
        balance.demands.push(Demand {
            part,
            measure,
            label: tones[0].place.label.clone(),
            staff: staff.to_string(),
            all: maximum_overlap(&all)?,
            visible: maximum_overlap(&visible)?,
            used_all: used_all.len(),
            used_visible: used_visible.len(),
        });
        let partners: BTreeSet<_> = tones
            .iter()
            .filter(|t| t.visible)
            .map(|t| (&t.voice, t.place.local, t.duration))
            .collect();
        for tone in tones.iter().filter(|t| !t.visible) {
            balance.hidden += 1;
            if partners.contains(&(&tone.voice, tone.place.local, tone.duration)) {
                balance.with_partner += 1;
            } else {
                balance.without_partner += 1;
            }
        }
    }
    Ok(())
}

fn calculate_balance(xml: &str) -> Result<Balance, String> {
    let notation = collect_notation(xml)?;
    let mut balance = Balance {
        resolution: notation.resolution,
        excluded_grace: notation.excluded_grace,
        ..Balance::default()
    };
    measure_rules(&notation, &mut balance);
    measure_chords(&notation, &mut balance)?;
    Ok(balance)
}

fn quarter_text(ticks: i128, resolution: i128) -> String {
    let divisor = greatest_common_divisor(ticks.abs(), resolution);
    format!("{}/{}", ticks / divisor, resolution / divisor)
}

fn append_level(text: &mut String, level: &str, xml: &str) {
    let balance = match calculate_balance(xml) {
        Ok(value) => value,
        Err(reason) => {
            let _ = writeln!(
                text,
                "NotationBalance: level {level}; status MODEL_INPUT_UNAVAILABLE; reason {reason}; counts unknown."
            );
            return;
        }
    };
    let [r1, r2, r3, r4, r5] = balance.counts;
    let _ = writeln!(
        text,
        "NotationInvariants: level {level}; status measured; R1 {r1}; R2 {r2}; R3 {r3}; R4 {r4}; R5 {r5}; excluded_grace {}.",
        balance.excluded_grace
    );
    for finding in &balance.findings {
        let _ = writeln!(
            text,
            "NotationFinding: level {level}; R{}; reason {}; part {}; measure_index {}; measure_label {:?}; staff {}; onset_quarters {}; absolute_quarters {}; voice {}; midi {}.",
            finding.rule,
            finding.reason,
            finding.place.part + 1,
            finding.place.measure + 1,
            finding.place.label,
            finding.place.staff,
            quarter_text(finding.place.local, balance.resolution),
            quarter_text(finding.place.at, balance.resolution),
            finding.voice.as_deref().unwrap_or("n/a"),
            finding.pitch.map_or_else(
                || "n/a".to_string(),
                |p| format!("{}/{}", p.numerator, p.denominator)
            )
        );
    }
    for (visibility, is_visible) in [("all", false), ("visible", true)] {
        let mut histogram = BTreeMap::<usize, usize>::new();
        let mut used = BTreeMap::<usize, usize>::new();
        let mut excess = 0;
        for row in &balance.demands {
            let (omega, labels) = if is_visible {
                (row.visible, row.used_visible)
            } else {
                (row.all, row.used_all)
            };
            *histogram.entry(omega).or_default() += 1;
            *used.entry(labels).or_default() += 1;
            excess += usize::from(labels > omega);
        }
        let _ = writeln!(
            text,
            "VoiceDemand: level {level}; visibility {visibility}; pitched_staff_measures {}; omega {histogram:?}; used_labels {used:?}; used_gt_omega {excess}.",
            balance.demands.len()
        );
    }
    for row in &balance.demands {
        let _ = writeln!(
            text,
            "VoiceDemandMeasure: level {level}; part {}; measure_index {}; measure_label {:?}; staff {}; omega_all {}; omega_visible {}; used_all {}; used_visible {}.",
            row.part + 1,
            row.measure + 1,
            row.label,
            row.staff,
            row.all,
            row.visible,
            row.used_all,
            row.used_visible
        );
    }
    let _ = writeln!(
        text,
        "ChordVisibility: level {level}; hidden_fragments {}; with_visible_partner {}; without_visible_partner {}.",
        balance.hidden, balance.with_partner, balance.without_partner
    );
}

pub(crate) fn append_notation_balance(text: &mut String, master: &str, render: Option<&str>) {
    text.push_str("Documents: final in-memory Master and Render measured independently; grace excluded; R1-R4 visible pitched heads; R5 all pitched fragments per pitch/staff/voice; MusicalAuthority none; ReleaseAuthority none.\n");
    text.push_str("VoiceModel: per part/staff/measure, equal onset and duration across voices form one event; half-open intervals; end before begin; population contains staff-measures with pitched non-grace fragments; partner classes also require equal voice; formal demand is no permission to merge musical voices.\n");
    append_level(text, "Master", master);
    if let Some(xml) = render {
        append_level(text, "Render", xml);
    } else {
        text.push_str(
            "NotationBalance: level Render; status DOCUMENT_UNAVAILABLE; counts unknown.\n",
        );
    }
}

#[cfg(test)]
#[path = "notation_invariants_tests.rs"]
mod tests;

/// Printed fragments for the material report; observation carries no source identity.
#[derive(Debug, Clone)]
pub(crate) struct PitchedFragment {
    pub(crate) part: usize,
    pub(crate) measure: usize,
    pub(crate) onset: (i128, i128),
    pub(crate) duration: (i128, i128),
    pub(crate) pitch: (i128, i128),
    pub(crate) staff: String,
    pub(crate) voice: String,
    pub(crate) visible: bool,
    pub(crate) tie_stop: bool,
}
pub(crate) fn material_fragments(xml: &str) -> Result<Vec<PitchedFragment>, String> {
    let notation = collect_notation(xml)?;
    let ratio = |value: i128| {
        let divisor = greatest_common_divisor(value.abs(), notation.resolution);
        (value / divisor, notation.resolution / divisor)
    };
    Ok(notation
        .tones
        .into_iter()
        .map(|tone| PitchedFragment {
            part: tone.place.part + 1,
            measure: tone.place.measure + 1,
            onset: ratio(tone.place.local),
            duration: ratio(tone.duration),
            pitch: (tone.pitch.numerator, tone.pitch.denominator),
            staff: tone.place.staff,
            voice: tone.voice,
            visible: tone.visible,
            tie_stop: tone.tie_stop,
        })
        .collect())
}

/// Exact attack ticks on the metric notation axis, for explicit entry-axis comparison only.
pub(crate) fn material_attack_ticks(xml: &str) -> Result<Vec<i64>, String> {
    let notation = collect_notation(xml)?;
    notation.tones.iter().filter(|tone| !tone.tie_stop).map(|tone| {
        let value = tone.place.at.checked_mul(96).ok_or("attack tick overflow")?;
        if value % notation.resolution != 0 { return Err("nonexact attack time in 96 quarter ticks".to_string()); }
        i64::try_from(value / notation.resolution).map_err(|_| "attack tick outside i64".to_string())
    }).collect()
}
