//! Current-document musical time for clef observation and protection, no writer.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
struct Note {
    start: usize,
    at: i128,
    end: i128,
    head: usize,
    lower: bool,
    visible: bool,
    attachment_known: bool,
    voice: String,
    key: String,
    tie_start: bool,
    tie_stop: bool,
    beam: Option<String>,
}

pub(crate) struct TemporalPart {
    positions: BTreeMap<usize, i128>,
    shifts: BTreeMap<i128, i64>,
    notes: Vec<Note>,
    length: usize,
    ambiguous_voices: BTreeSet<String>,
}

fn text<'a>(n: roxmltree::Node<'a, 'a>, name: &str) -> Option<&'a str> {
    n.children()
        .find(|c| c.has_tag_name(name))?
        .text()
        .map(str::trim)
}

fn number(n: roxmltree::Node<'_, '_>, name: &str) -> Option<i128> {
    text(n, name)?.parse().ok()
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl TemporalPart {
    pub(crate) fn observe(xml: &str) -> Option<Self> {
        let doc = roxmltree::Document::parse(xml).ok()?;
        let root = doc.root_element();
        let part = if root.has_tag_name("part") {
            root
        } else {
            let mut parts = root.children().filter(|node| node.has_tag_name("part"));
            let part = parts.next()?;
            if parts.next().is_some() {
                return None;
            }
            part
        };
        let mut unit = 1i128;
        for d in part.descendants().filter(|n| n.has_tag_name("divisions")) {
            let d = d.text()?.trim().parse::<i128>().ok().filter(|d| *d > 0)?;
            unit = unit.checked_div(gcd(unit, d))?.checked_mul(d)?;
        }
        let mut result = Self {
            positions: BTreeMap::new(),
            shifts: BTreeMap::new(),
            notes: Vec::new(),
            length: xml.len(),
            ambiguous_voices: BTreeSet::new(),
        };
        let mut divisions = 1i128;
        let mut base = 0i128;
        let mut clefs = BTreeMap::<i128, (String, String, usize, usize, bool)>::new();
        let mut shift_domains = BTreeMap::<i128, (usize, usize, i64)>::new();
        for (measure_index, measure) in part
            .children()
            .filter(|n| n.has_tag_name("measure"))
            .enumerate()
        {
            let mut stream = 0usize;
            let mut cursor = 0i128;
            let mut extent = 0i128;
            let mut head = None;
            let mut attachment = super::LowerChordObservation::default();
            let mut previous_note_end = measure.range().start;
            for n in measure.children().filter(|n| n.is_element()) {
                let at = base.checked_add(cursor)?;
                let range = n.range();
                result.positions.insert(range.start, at);
                result.positions.insert(range.end, at);
                match n.tag_name().name() {
                    "attributes" => {
                        if let Some(d) = number(n, "divisions") {
                            if d <= 0 {
                                return None;
                            }
                            divisions = d;
                        }
                        for c in n.children().filter(|c| {
                            c.has_tag_name("clef") && c.attribute("number") == Some("2")
                        }) {
                            let value =
                                (text(c, "sign")?.to_string(), text(c, "line")?.to_string());
                            if !matches!(
                                (value.0.as_str(), value.1.as_str()),
                                ("G", "2") | ("F", "4")
                            ) || c.children().any(|c| c.has_tag_name("clef-octave-change"))
                                || clefs.get(&at).is_some_and(|old| {
                                    (old.0.as_str(), old.1.as_str())
                                        != (value.0.as_str(), value.1.as_str())
                                        && old.2 == measure_index
                                        && old.3 != stream
                                        && !old.4
                                })
                            {
                                return None;
                            }
                        }
                        let default = clefs.is_empty() && result.notes.is_empty();
                        // Initial declarations and a later ordered surface can
                        // share time; incomparable stream declarations cannot.
                        for c in n.children().filter(|c| {
                            c.has_tag_name("clef") && c.attribute("number") == Some("2")
                        }) {
                            clefs.insert(
                                at,
                                (
                                    text(c, "sign")?.into(),
                                    text(c, "line")?.into(),
                                    measure_index,
                                    stream,
                                    default,
                                ),
                            );
                        }
                    }
                    "backup" | "forward" => {
                        let duration =
                            number(n, "duration")?.checked_mul(unit.checked_div(divisions)?)?;
                        if duration < 0 {
                            return None;
                        }
                        cursor = if n.has_tag_name("backup") {
                            stream = stream.checked_add(1)?;
                            cursor.checked_sub(duration)?
                        } else {
                            cursor.checked_add(duration)?
                        };
                        if cursor < 0 {
                            return None;
                        }
                        extent = extent.max(cursor);
                        head = None;
                    }
                    "direction" => {
                        if text(n, "staff") == Some("2") {
                            let offset =
                                if n.children().any(|child| child.has_tag_name("offset")) {
                                    number(n, "offset")?
                                } else {
                                    0
                                }
                                .checked_mul(unit.checked_div(divisions)?)?;
                            let effective = at.checked_add(offset)?;
                            if effective < base {
                                return None;
                            }
                            for shift in n.descendants().filter(|c| c.has_tag_name("octave-shift"))
                            {
                                let kind = shift.attribute("type")?;
                                let size = shift.attribute("size").unwrap_or("8");
                                let distance = match size {
                                    "8" => 7,
                                    "15" => 14,
                                    _ => return None,
                                };
                                let value = match kind {
                                    "up" => distance,
                                    "down" => -distance,
                                    "stop" => 0,
                                    "continue" => continue,
                                    _ => return None,
                                };
                                if shift_domains.get(&effective).is_some_and(
                                    |&(measure, run, previous)| {
                                        measure == measure_index
                                            && run != stream
                                            && previous != value
                                    },
                                ) {
                                    return None;
                                }
                                shift_domains.insert(effective, (measure_index, stream, value));
                                result.shifts.insert(effective, value);
                            }
                        }
                    }
                    "note" => {
                        // Grace and unmeasured notes need a separate ordering proof.
                        if n.children().any(|c| c.has_tag_name("grace")) {
                            return None;
                        }
                        let duration =
                            number(n, "duration")?.checked_mul(unit.checked_div(divisions)?)?;
                        if duration <= 0 {
                            return None;
                        }
                        let member = n.children().any(|c| c.has_tag_name("chord"));
                        let voice = text(n, "voice").unwrap_or("1").to_string();
                        let (onset, origin) = if member {
                            head?
                        } else {
                            head = Some((at, range.start));
                            (at, range.start)
                        };
                        let end = onset.checked_add(duration)?;
                        result.positions.insert(range.start, onset);
                        result.positions.insert(range.end, onset);
                        let staff = text(n, "staff").unwrap_or("1");
                        let lower = staff == "2";
                        let visible = n.attribute("print-object") != Some("no");
                        let pitch = n.children().find(|c| c.has_tag_name("pitch"));
                        let relation = attachment.observe(
                            &xml[previous_note_end..range.start],
                            range.start,
                            &xml[range.clone()],
                            lower && visible && pitch.is_some(),
                        );
                        previous_note_end = range.end;
                        let key = if let Some(p) = pitch {
                            format!(
                                "{}|{}|{}|{}",
                                voice,
                                text(p, "step")?,
                                text(p, "alter").unwrap_or(""),
                                text(p, "octave")?
                            )
                        } else {
                            String::new()
                        };
                        let tie = |kind| {
                            n.descendants().any(|c| {
                                (c.has_tag_name("tie") || c.has_tag_name("tied"))
                                    && c.attribute("type") == Some(kind)
                            })
                        };
                        let beam = n
                            .children()
                            .find(|c| {
                                c.has_tag_name("beam")
                                    && c.attribute("number").unwrap_or("1") == "1"
                            })
                            .map(|c| c.text().unwrap_or("").to_string());
                        let attachment_known = relation.head().is_some();
                        result.notes.push(Note {
                            start: range.start,
                            at: onset,
                            end,
                            head: origin,
                            lower,
                            visible,
                            attachment_known,
                            voice,
                            key,
                            tie_start: tie("start"),
                            tie_stop: tie("stop"),
                            beam,
                        });
                        extent = extent.max(end.checked_sub(base)?);
                        if !member {
                            cursor = cursor.checked_add(duration)?;
                        }
                    }
                    _ => {}
                }
            }
            if extent <= 0 {
                return None;
            }
            base = base.checked_add(extent)?;
        }
        // Representation order is not voice chronology. Detect actual overlap
        // in musical time; the pitch scanner traverses the same ordered heads.
        let mut heads = result.notes.iter()
            .filter(|note| note.visible && note.start == note.head)
            .collect::<Vec<_>>();
        heads.sort_by_key(|note| (note.at, note.start));
        let mut voice_ends = BTreeMap::<&str, i128>::new();
        for note in heads {
            let end = result
                .notes
                .iter()
                .filter(|member| member.visible && member.head == note.head)
                .map(|member| member.end)
                .max()?;
            if voice_ends
                .insert(&note.voice, end)
                .is_some_and(|previous| previous > note.at)
            {
                result.ambiguous_voices.insert(note.voice.clone());
            }
        }
        Some(result)
    }

    pub(crate) fn origin_supported(&self, position: usize) -> bool {
        self.notes
            .iter()
            .find(|note| note.start == position)
            .is_some_and(|note| {
                note.attachment_known && !self.ambiguous_voices.contains(&note.voice)
            })
    }

    pub(crate) fn at(&self, position: usize) -> Option<i128> {
        if position == self.length {
            Some(i128::MAX)
        } else {
            self.positions.get(&position).copied()
        }
    }

    pub(crate) fn shift(&self, position: usize) -> Option<i64> {
        let at = self.at(position)?;
        Some(
            self.shifts
                .range(..=at)
                .next_back()
                .map_or(0, |(_, value)| *value),
        )
    }

    pub(crate) fn scope(&self, position: usize) -> Option<i128> {
        let at = self.at(position)?;
        Some(
            self.shifts
                .range(..=at)
                .next_back()
                .map_or(-1, |(time, _)| *time),
        )
    }

    pub(crate) fn tie_open(&self, position: usize) -> bool {
        let Some(at) = self.at(position) else {
            return true;
        };
        let mut streams = BTreeMap::<&str, Vec<&Note>>::new();
        for n in self
            .notes
            .iter()
            .filter(|n| n.lower && (n.tie_start || n.tie_stop))
        {
            streams.entry(&n.key).or_default().push(n);
        }
        for notes in streams.values_mut() {
            notes.sort_by_key(|n| (n.at, n.start));
            let mut open: Option<(i128, i128)> = None;
            for n in notes.iter() {
                if n.tie_stop {
                    if let Some((start, end)) = open {
                        if end != n.at {
                            if start < at {
                                return true;
                            }
                        } else if start < at && at < n.end {
                            return true;
                        }
                    } else if at < n.end {
                        return true;
                    }
                    open = None;
                }
                if n.tie_start {
                    open = Some((n.at, n.end));
                }
            }
            if open.is_some_and(|(start, _)| start < at) {
                return true;
            }
        }
        false
    }

    /// None denotes unsupported/incomplete evidence, never a fresh permission.
    pub(crate) fn beam_inside(&self, position: usize) -> Option<bool> {
        let at = self.at(position)?;
        let scope = self.scope(position)?;
        if self.notes.iter().any(|note| {
            note.visible
                && note.lower
                && (!note.attachment_known || self.ambiguous_voices.contains(&note.voice))
                && note.at <= at
                && at < note.end
        }) {
            return None;
        }
        let mut streams = BTreeMap::<&str, Vec<&Note>>::new();
        for n in self.notes.iter().filter(|n| n.visible && n.start == n.head) {
            streams.entry(&n.voice).or_default().push(n);
        }
        for notes in streams.values_mut() {
            notes.sort_by_key(|n| (n.at, n.start));
            let mut open: Option<(i128, bool)> = None;
            for n in notes.iter() {
                let lower = self
                    .notes
                    .iter()
                    .any(|member| member.visible && member.head == n.start && member.lower);
                match n.beam.as_deref() {
                    Some("begin") => {
                        if open.is_some_and(|(start, touches)| {
                            touches && scope <= start && start < at && at <= n.at
                        }) {
                            return None;
                        }
                        open = Some((n.at, lower));
                    }
                    Some("continue") => {
                        if let Some((_, touches)) = open.as_mut() {
                            *touches |= lower;
                        } else if lower && n.at <= at && at < n.end {
                            return None;
                        }
                    }
                    Some("end") => {
                        if let Some((start, touches)) = open.take() {
                            if (touches || lower) && n.at <= start {
                                return None;
                            }
                            if (touches || lower) && scope <= start && start < at && at < n.end {
                                return Some(true);
                            }
                        } else if lower && n.at <= at && at < n.end {
                            return None;
                        }
                    }
                    Some("forward hook" | "backward hook") => {}
                    None => {
                        if open.take().is_some_and(|(start, touches)| {
                            touches && scope <= start && start < at && at <= n.at
                        }) {
                            return None;
                        }
                    }
                    Some(_) => return None,
                }
            }
            if open.is_some_and(|(start, touches)| touches && scope <= start && start < at) {
                return None;
            }
        }
        Some(false)
    }
}
