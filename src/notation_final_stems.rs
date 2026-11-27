use crate::clef_readability_observation::{PianoClef, observe_pitch};
use roxmltree::{Document, Node};
use std::collections::BTreeMap;

#[path = "notation_final_stems_tie_chain.rs"]
mod tie_chain;

pub(super) type StemEdit = (usize, usize, &'static str);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    fn xml(self) -> &'static str {
        match self {
            Self::Up => "<stem>up</stem>",
            Self::Down => "<stem>down</stem>",
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    fn outward(self, distance: i64) -> i64 {
        match self {
            Self::Up => distance,
            Self::Down => -distance,
        }
    }
}

struct Head {
    distance: i64,
    stem: Option<StemEdit>,
    written_pitch: Option<i64>,
    ties: Option<StemTies>,
    source: (usize, usize),
    ordinary_stem: bool,
}

struct Attack<'a> {
    start: i64,
    end: i64,
    staff: usize,
    voice: &'a str,
    beam: Option<&'a str>,
    flagged: bool,
    primary_only: bool,
    ordinary_beam_levels: Option<i64>,
    tie: Option<TieEndpoint>,
    clef: Option<PianoClef>,
    isolation_supported: bool,
    heads: Vec<Head>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TieEndpoint {
    Start(i32),
    Stop(i32),
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct TieSides {
    start: Option<u32>,
    stop: Option<u32>,
}

#[derive(Clone, Copy)]
struct StemTies {
    sides: TieSides,
    alteration: i32,
}

fn tie_sides<'a, 'input: 'a>(marks: impl Iterator<Item = Node<'a, 'input>>) -> Option<TieSides> {
    let mut sides = TieSides::default();
    for mark in marks {
        if mark.attribute("time-only").is_some() {
            return None;
        }
        let side = match mark.attribute("type")? {
            "start" => &mut sides.start,
            "stop" => &mut sides.stop,
            _ => return None,
        };
        let number = mark.attribute("number").unwrap_or("1").parse().ok()?;
        if side.replace(number).is_some() {
            return None;
        }
    }
    Some(sides)
}

fn stem_ties(note: Node<'_, '_>) -> Option<StemTies> {
    let sound = tie_sides(note.children().filter(|n| n.has_tag_name("tie")))?;
    let print = tie_sides(note.children().filter(|n| n.has_tag_name("notations"))
        .flat_map(|notations| notations.children().filter(|n| n.has_tag_name("tied"))))?;
    let empty = TieSides::default();
    if sound != empty && print != empty
        && (sound.start.is_some(), sound.stop.is_some()) != (print.start.is_some(), print.stop.is_some())
    {
        return None;
    }
    let sides = if print == empty { sound } else { print };
    let alteration = if sides == empty { 0 } else {
        field(child(note, "pitch")?, "alter").unwrap_or("0").parse().ok()?
    };
    Some(StemTies { sides, alteration })
}

struct Group<'a> {
    attacks: Vec<Attack<'a>>,
    complete: bool,
}

impl Group<'_> {
    fn staff(&self) -> usize {
        self.attacks[0].staff
    }
    fn voice(&self) -> &str {
        self.attacks[0].voice
    }

    fn topology_writable(&self) -> bool {
        self.complete
            && self
                .attacks
                .windows(2)
                .all(|pair| pair[0].end == pair[1].start)
            && self.attacks.iter().all(|attack| {
                attack.start < attack.end
                    && attack.heads.iter().all(|head| head.stem.is_some() && head.written_pitch.is_some())
            })
    }

    fn writable(&self) -> bool {
        let clef = self.attacks[0].clef;
        clef.is_some() && self.topology_writable()
            && self.attacks.iter().all(|attack| attack.clef == clef)
    }
}

fn child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
    node.children().find(|n| n.has_tag_name(name))
}

fn field<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    child(node, name)?.text().map(str::trim)
}

fn tie_endpoint(note: Node<'_, '_>) -> Option<TieEndpoint> {
    let mut ties = note.children().filter(|node| node.has_tag_name("tie"));
    let endpoint = ties.next()?.attribute("type")?;
    if ties.next().is_some() {
        return None;
    }
    let alteration = field(child(note, "pitch")?, "alter")
        .unwrap_or("0")
        .parse()
        .ok()?;
    match endpoint {
        "start" => Some(TieEndpoint::Start(alteration)),
        "stop" => Some(TieEndpoint::Stop(alteration)),
        _ => None,
    }
}

fn staff_index(staff: &str) -> Option<usize> {
    match staff {
        "1" => Some(0),
        "2" => Some(1),
        _ => None,
    }
}

fn printed_clef(node: Node<'_, '_>) -> Option<PianoClef> {
    if !matches!(field(node, "clef-octave-change"), None | Some("0")) {
        return None;
    }
    match (field(node, "sign"), field(node, "line")) {
        (Some("G"), Some("2")) => Some(PianoClef::TrebleG2),
        (Some("F"), Some("4")) => Some(PianoClef::BassF4),
        _ => None,
    }
}

#[derive(Clone, Copy)]
struct OctaveSpan {
    start: i64,
    stop: i64,
    steps: i64,
}

struct OctaveEvent<'a, 'input> {
    node: Node<'a, 'input>,
    at: Option<i64>,
}

type ActiveOctaves = BTreeMap<String, Option<i64>>;

fn octave_diatonic_span(size: &str) -> Option<i64> {
    match size {
        "8" => Some(7),
        "15" => Some(14),
        "22" => Some(21),
        _ => None,
    }
}

fn known_octave_shift(active: &ActiveOctaves) -> Option<i64> {
    match active.len() {
        0 => Some(0),
        1 => active.values().next().copied().flatten(),
        _ => None,
    }
}

fn measure_octave_course(
    mut active: ActiveOctaves,
    events: &[OctaveEvent<'_, '_>],
) -> Option<Vec<OctaveSpan>> {
    let mut events = events.iter().collect::<Vec<_>>();
    events.sort_by_key(|event| (event.at, event.node.attribute("type") != Some("stop")));
    let mut course: Vec<OctaveSpan> = Vec::new();
    let mut start = 0;
    let mut steps = known_octave_shift(&active)?;
    for event in events {
        let at = event.at?;
        let number = event.node.attribute("number").unwrap_or("1");
        let size = event.node.attribute("size");
        match event.node.attribute("type")? {
            "stop" | "continue" => {
                let old = (*active.get(number)?)?;
                if size.is_some_and(|size| octave_diatonic_span(size) != Some(old.abs())) {
                    return None;
                }
                if event.node.attribute("type") == Some("stop") {
                    active.remove(number);
                }
            }
            kind @ ("up" | "down") => {
                let magnitude = octave_diatonic_span(size.unwrap_or("8"))?;
                let shift = if kind == "up" { magnitude } else { -magnitude };
                if active.insert(number.to_owned(), Some(shift)).is_some() {
                    return None;
                }
            }
            _ => return None,
        }
        let next = known_octave_shift(&active)?;
        if next != steps {
            if start < at {
                course.push(OctaveSpan { start, stop: at, steps });
            }
            start = at;
            steps = next;
        }
    }
    course.push(OctaveSpan { start, stop: i64::MAX, steps });
    Some(course)
}

struct StaffPrint {
    clefs: BTreeMap<i64, Option<PianoClef>>,
    octave: Vec<OctaveSpan>,
    has_octave_context: bool,
}

impl StaffPrint {
    fn clef_during(&self, start: i64, end: i64) -> Option<PianoClef> {
        use std::ops::Bound::Excluded;
        if start >= end || self.clefs.range((Excluded(start), Excluded(end))).next().is_some() {
            return None;
        }
        *self.clefs.range(..=start).next_back()?.1
    }
}

fn rhythmic_duration(node: Node<'_, '_>) -> Option<i64> {
    match field(node, "duration") {
        Some(value) => value.parse().ok().filter(|&duration| duration >= 0),
        None if node.has_tag_name("note") => Some(0),
        None => None,
    }
}

fn rhythmic_cursor(node: Node<'_, '_>, cursor: i64) -> Option<i64> {
    let is_note = node.has_tag_name("note");
    let duration = rhythmic_duration(node)?;
    if is_note && child(node, "grace").is_some() {
        return (duration == 0).then_some(cursor);
    }
    let next = if is_note && child(node, "chord").is_some() {
        cursor
    } else if node.has_tag_name("backup") {
        cursor.checked_sub(duration)?
    } else {
        cursor.checked_add(duration)?
    };
    (next >= 0).then_some(next)
}

fn octave_markers<'a, 'input>(
    direction: Node<'a, 'input>,
) -> impl Iterator<Item = Node<'a, 'input>> {
    direction
        .children()
        .filter(|node| node.has_tag_name("direction-type"))
        .flat_map(|node| node.children())
        .filter(|node| node.has_tag_name("octave-shift"))
}

#[derive(Default)]
struct PrintContext {
    clefs: [Option<PianoClef>; 2],
    octave_shifts: [ActiveOctaves; 2],
}

impl PrintContext {
    fn for_measure(&mut self, measure: Node<'_, '_>) -> [Option<StaffPrint>; 2] {
        let carried = self.octave_shifts.clone();
        let mut events = [Vec::new(), Vec::new()];
        let mut clef_events: [BTreeMap<i64, Option<PianoClef>>; 2] = Default::default();
        let mut unpositioned_clef = [None; 2];
        let mut stable_divisions = true;
        let mut cursor = measure.children().any(|node|
            (node.has_tag_name("attributes") && child(node, "clef").is_some())
                || (node.has_tag_name("direction") && octave_markers(node).next().is_some())
        ).then_some(0);
        let mut sounding = false;
        for node in measure.children().filter(Node::is_element) {
            match node.tag_name().name() {
                "note" | "forward" | "backup" => {
                    sounding = true;
                    cursor = cursor.and_then(|at| rhythmic_cursor(node, at));
                }
                "attributes" => {
                    if sounding && child(node, "divisions").is_some() {
                        stable_divisions = false;
                    }
                    for clef in node.children().filter(|n| n.has_tag_name("clef")) {
                        if let Some(staff) = staff_index(clef.attribute("number").unwrap_or("1")) {
                            let next = printed_clef(clef);
                            let agreed = match cursor {
                                Some(at) => clef_events[staff].entry(at).or_insert(next),
                                None => unpositioned_clef[staff].get_or_insert(next),
                            };
                            // Parallel voices may restate the same boundary. Conflicting
                            // declarations leave it unresolved in either XML order.
                            if *agreed != next {
                                *agreed = None;
                            }
                        }
                    }
                }
                "direction" => {
                    let Some(staff) = staff_index(field(node, "staff").unwrap_or("1")) else {
                        continue;
                    };
                    for shift in octave_markers(node) {
                        let at = cursor.filter(|_| {
                            child(node, "offset").is_none() && child(node, "voice").is_none()
                        });
                        events[staff].push(OctaveEvent { node: shift, at });
                        if shift.attribute("type") == Some("stop") {
                            self.octave_shifts[staff]
                                .remove(shift.attribute("number").unwrap_or("1"));
                        } else {
                            if shift.attribute("type") == Some("continue") {
                                continue;
                            }
                            let amount = octave_diatonic_span(shift.attribute("size").unwrap_or("8"));
                            let steps = match shift.attribute("type") {
                                Some("up") => amount,
                                Some("down") => amount.map(|value| -value),
                                _ => None,
                            };
                            self.octave_shifts[staff].insert(
                                shift.attribute("number").unwrap_or("1").to_owned(),
                                at.and(steps),
                            );
                        }
                    }
                }
                _ => {}
            }
        }
        std::array::from_fn(|staff| {
            let mut clefs = std::mem::take(&mut clef_events[staff]);
            if !stable_divisions || unpositioned_clef[staff].is_some() {
                // If time positions cannot be compared, equal declarations still
                // determine the next measure's clef independently of their order.
                let mut declarations = clefs.values().copied().chain(unpositioned_clef[staff]);
                if let Some(first) = declarations.next() {
                    self.clefs[staff] = first.filter(|clef| declarations.all(|other| other == Some(*clef)));
                }
                return None;
            }
            clefs.entry(0).or_insert(self.clefs[staff]);
            let mut previous = None;
            clefs.retain(|_, value| {
                let changed = previous != *value;
                previous = *value;
                changed
            });
            self.clefs[staff] = previous;
            if clefs.is_empty() {
                return None;
            }
            let has_octave_context = !carried[staff].is_empty() || !events[staff].is_empty();
            let octave = measure_octave_course(carried[staff].clone(), &events[staff])?;
            Some(StaffPrint { clefs, octave, has_octave_context })
        })
    }
}

struct PositionedSymbol {
    staff: Option<usize>,
    onset: i64,
}

struct PrintedMeasure<'a> {
    attacks: Vec<Attack<'a>>,
    other_symbols: Vec<PositionedSymbol>,
}

fn collect_attacks<'a>(
    xml: &str,
    measure: Node<'a, '_>,
    print: [Option<StaffPrint>; 2],
) -> Option<PrintedMeasure<'a>> {
    let mut attacks: Vec<Attack<'a>> = Vec::new();
    let mut other_symbols = Vec::new();
    let mut cursor = 0_i64;
    let mut attachment: Option<usize> = None;
    for node in measure.children().filter(Node::is_element) {
        let tag = node.tag_name().name();
        if matches!(tag, "forward" | "backup") {
            cursor = rhythmic_cursor(node, cursor)?;
            attachment = None;
        }
        if tag != "note" {
            continue;
        }
        let visible = node.attribute("print-object") != Some("no");
        if child(node, "grace").is_some() {
            if visible {
                other_symbols.push(PositionedSymbol {
                    staff: staff_index(field(node, "staff").unwrap_or("1")),
                    onset: cursor,
                });
            }
            attachment = None;
            continue;
        }
        let duration = rhythmic_duration(node)?;
        let is_chord = child(node, "chord").is_some();
        let start = cursor;
        if !is_chord {
            cursor = cursor.checked_add(duration)?;
            attachment = None;
        }
        if !visible {
            continue;
        }
        let staff = staff_index(field(node, "staff").unwrap_or("1"));
        if child(node, "pitch").is_none() {
            other_symbols.push(PositionedSymbol { staff, onset: start });
            continue;
        }
        let staff = staff?;
        let voice = field(node, "voice").unwrap_or("1");
        let start = if is_chord {
            attacks[attachment?].start
        } else {
            start
        };
        let end = start.checked_add(duration)?;
        let pitch = observe_pitch(xml.get(node.range())?);
        let printed = print[staff].as_ref().and_then(|context| {
            let clef = context.clef_during(start, end)?;
            let steps = context.octave.iter()
                .find(|span| span.start <= start && end <= span.stop)?.steps;
            Some((clef, clef.distance_from_middle_line(pitch?.shifted(steps))))
        });
        let stem = child(node, "stem").and_then(|stem| {
            let range = stem.range();
            let original = match xml.get(range.clone())? {
                "<stem>up</stem>" => Direction::Up.xml(),
                "<stem>down</stem>" => Direction::Down.xml(),
                _ => return None,
            };
            Some((range.start, range.end, original))
        });
        let mut beams = node.children().filter(|n| n.has_tag_name("beam"));
        let primary = beams
            .clone()
            .find(|n| n.attribute("number").unwrap_or("1") == "1");
        let primary_only = primary.is_some() && beams.clone().count() == 1;
        let ordinary_beam_levels = if primary.is_some()
            && !node.children().any(|child| matches!(child.tag_name().name(),
                "tie" | "notations" | "time-modification" | "cue" | "notehead"))
        {
            match beams.clone().count() {
                1 => Some(1),
                2 if beams.clone().any(|beam| beam.attribute("number") == Some("2")
                    && matches!(beam.text().map(str::trim), Some("begin" | "continue" | "end"))) => Some(2),
                _ => None,
            }
        } else {
            None
        };
        let beam = match primary {
            Some(primary) => Some(primary.text()?.trim()),
            None if beams.next().is_some() => Some("unbound"),
            None => None,
        };
        let source = node.range();
        let head = Head {
            distance: printed.map_or(0, |(_, distance)| distance),
            stem,
            written_pitch: pitch.map(|pitch| PianoClef::TrebleG2.distance_from_middle_line(pitch)),
            ties: stem_ties(node),
            source: (source.start, source.end),
            ordinary_stem: child(node, "cue").is_none()
                && child(node, "notehead").is_none_or(|head| head.text().map(str::trim) == Some("normal"))
                && matches!(field(node, "type"), Some("half" | "quarter" | "eighth" | "16th"
                    | "32nd" | "64th" | "128th" | "256th" | "512th" | "1024th")),
        };
        let clef = printed.map(|(clef, _)| clef);
        if is_chord {
            let i = attachment?;
            let attack = &mut attacks[i];
            if attack.clef != clef {
                attack.clef = None;
            }
            if attack.staff != staff || attack.voice != voice || attack.end != end || beam.is_some() {
                attack.clef = None;
                for head in &mut attack.heads {
                    head.written_pitch = None;
                }
            }
            attack.end = attack.end.max(end);
            attack.heads.push(head);
        } else {
            attachment = Some(attacks.len());
            attacks.push(Attack {
                start,
                end: cursor,
                staff,
                voice,
                beam,
                primary_only,
                ordinary_beam_levels,
                flagged: beam.is_none() && matches!(field(node, "type"),
                    Some("eighth" | "16th" | "32nd" | "64th" | "128th" | "256th")),
                tie: tie_endpoint(node),
                clef,
                isolation_supported: print[staff].as_ref().is_some_and(|context| !context.has_octave_context),
                heads: vec![head],
            });
        }
    }
    Some(PrintedMeasure { attacks, other_symbols })
}

fn beam_groups(attacks: Vec<Attack<'_>>) -> Vec<Group<'_>> {
    let mut lanes = BTreeMap::<_, Vec<_>>::new();
    for attack in attacks {
        lanes
            .entry((attack.staff, attack.voice))
            .or_default()
            .push(attack);
    }
    let mut groups = Vec::new();
    for lane in lanes.into_values() {
        let mut pending = Vec::new();
        for attack in lane {
            match attack.beam {
                Some("continue" | "end") if !pending.is_empty() => {
                    let complete = attack.beam == Some("end");
                    pending.push(attack);
                    if complete {
                        groups.push(Group {
                            attacks: std::mem::take(&mut pending),
                            complete,
                        });
                    }
                }
                _ => {
                    if !pending.is_empty() {
                        groups.push(Group {
                            attacks: std::mem::take(&mut pending),
                            complete: false,
                        });
                    }
                    match attack.beam {
                        Some("begin") => pending.push(attack),
                        beam => groups.push(Group {
                            attacks: vec![attack],
                            complete: beam.is_none(),
                        }),
                    }
                }
            }
        }
        if !pending.is_empty() {
            groups.push(Group {
                attacks: pending,
                complete: false,
            });
        }
    }
    groups
}

#[derive(Default)]
struct Relation {
    neighbors: Vec<usize>,
    above: bool,
    below: bool,
    ambiguous: bool,
    distinct_timing: bool,
    flag_against_beam: bool,
}

fn overlap(a: &Attack<'_>, b: &Attack<'_>) -> bool {
    a.start < b.end && b.start < a.end
}

fn entirely_above(a: &Attack<'_>, b: &Attack<'_>) -> bool {
    a.heads
        .iter()
        .all(|upper| b.heads.iter().all(|lower| upper.distance > lower.distance))
}

fn mixed_stem_order(a: &Attack<'_>, b: &Attack<'_>) -> Option<Direction> {
    if a.heads.iter().any(|x| b.heads.iter().any(|y| x.distance == y.distance)) {
        return None;
    }
    let midpoint_twice = |attack: &Attack<'_>| {
        let distances = attack.heads.iter().map(|head| i128::from(head.distance));
        distances.clone().min().unwrap_or_default() + distances.max().unwrap_or_default()
    };
    match midpoint_twice(a).cmp(&midpoint_twice(b)) {
        std::cmp::Ordering::Greater => Some(Direction::Up),
        std::cmp::Ordering::Less => Some(Direction::Down),
        std::cmp::Ordering::Equal => None,
    }
}

fn compare_groups(groups: &[Group<'_>]) -> Vec<Relation> {
    let mut relations = (0..groups.len())
        .map(|_| Relation::default())
        .collect::<Vec<_>>();
    for (i, a) in groups.iter().enumerate() {
        for (j, b) in groups.iter().enumerate().skip(i + 1) {
            let same_staff = a.staff() == b.staff();
            let same_voice = a.voice() == b.voice();
            if !same_staff && !same_voice {
                continue;
            }
            for x in &a.attacks {
                for y in &b.attacks {
                    if !overlap(x, y) {
                        continue;
                    }
                    if same_staff && !same_voice {
                        let flag_against_beam = (x.flagged && b.attacks.len() > 1)
                            || (y.flagged && a.attacks.len() > 1);
                        let mixed_order = flag_against_beam.then(|| mixed_stem_order(x, y)).flatten();
                        if entirely_above(x, y) || mixed_order == Some(Direction::Up) {
                            relations[i].above = true;
                            relations[j].below = true;
                        } else if entirely_above(y, x) || mixed_order == Some(Direction::Down) {
                            relations[j].above = true;
                            relations[i].below = true;
                        } else {
                            relations[i].ambiguous = true;
                            relations[j].ambiguous = true;
                        }
                        if flag_against_beam {
                            relations[i].flag_against_beam = true;
                            relations[j].flag_against_beam = true;
                        }
                        if (x.start, x.end) != (y.start, y.end) {
                            relations[i].distinct_timing = true;
                            relations[j].distinct_timing = true;
                        }
                    } else {
                        // A same-voice overlap cannot establish two independent roles.
                        relations[i].ambiguous = true;
                        relations[j].ambiguous = true;
                    }
                    relations[i].neighbors.push(j);
                    relations[j].neighbors.push(i);
                }
            }
        }
    }
    for (index, pair) in groups.windows(2).enumerate() {
        let Some(boundary) = tied_boundary(&pair[0], &pair[1]) else {
            continue;
        };
        let (Some(left), Some(right)) = (
            relations[index].single_neighbor(),
            relations[index + 1].single_neighbor(),
        ) else {
            continue;
        };
        if relations[left].single_neighbor() == Some(index)
            && relations[right].single_neighbor() == Some(index + 1)
            && relations[index].ordered_role().is_some()
            && relations[index].ordered_role() == relations[index + 1].ordered_role()
            && groups[left].writable()
            && groups[right].writable()
            && pair.iter().all(Group::writable)
            && groups[left].attacks.len() > 1
            && groups[right].attacks.len() > 1
            && groups[left].voice() == groups[right].voice()
            // A continuation in a beam must already have independent timing.
            && (pair[1].attacks.len() == 1 || relations[index + 1].distinct_timing)
            && groups[left]
                .attacks
                .last()
                .is_some_and(|attack| attack.end == boundary)
            && groups[right].attacks[0].start == boundary
        {
            // The sustained attack crosses the other voice's beam boundary.
            // Decide both fragments and both opposing beams as one component.
            relations[index].distinct_timing = true;
            relations[index].neighbors.push(index + 1);
            relations[index + 1].neighbors.push(index);
        }
    }
    relations
}

fn tied_boundary(left: &Group<'_>, right: &Group<'_>) -> Option<i64> {
    let ([start], [stop, ..]) = (left.attacks.as_slice(), right.attacks.as_slice()) else {
        return None;
    };
    let ([first], [second]) = (start.heads.as_slice(), stop.heads.as_slice()) else {
        return None;
    };
    let Some(TieEndpoint::Start(alteration)) = start.tie else {
        return None;
    };
    let boundary = start.end;
    (stop.tie == Some(TieEndpoint::Stop(alteration))
        && start.beam.is_none()
        && matches!(stop.beam, None | Some("begin"))
        && start.staff == stop.staff
        && start.voice == stop.voice
        && boundary == stop.start
        && first.distance == second.distance)
        .then_some(boundary)
}

impl Relation {
    fn single_neighbor(&self) -> Option<usize> {
        let (&first, rest) = self.neighbors.split_first()?;
        rest.iter()
            .all(|&neighbor| neighbor == first)
            .then_some(first)
    }

    fn ordered_role(&self) -> Option<Direction> {
        match (self.above, self.below, self.ambiguous) {
            (true, false, false) => Some(Direction::Up),
            (false, true, false) => Some(Direction::Down),
            _ => None,
        }
    }
}

fn isolated_direction(group: &Group<'_>) -> Option<Direction> {
    // An octave line alone does not settle the stem of an isolated beam.
    if !group.attacks[0].isolation_supported {
        return None;
    }
    let heads = group.attacks.iter().flat_map(|a| &a.heads);
    let low = heads.clone().map(|head| head.distance).min()?;
    let high = heads.map(|head| head.distance).max()?;
    if low >= 0 && high > 0 {
        Some(Direction::Down)
    } else if high <= 0 && low < 0 {
        Some(Direction::Up)
    } else {
        None
    }
}

// The middle-line rule alone cannot decide one fragment of a tie independently
// of its printed partner. Each endpoint binds by musical time and written pitch.
fn isolated_directions(groups: &[Group<'_>], relations: &[Relation]) -> Vec<Option<Direction>> {
    let mut directions: Vec<_> = groups.iter().zip(relations).map(|(group, relation)| {
        (relation.neighbors.is_empty() && group.writable()).then(|| isolated_direction(group)).flatten()
    }).collect();
    let mut endpoints = BTreeMap::new();
    for (index, group) in groups.iter().enumerate() {
        for attack in &group.attacks {
            for head in &attack.heads {
                let Some(tie) = head.ties else {
                    directions[index] = None;
                    continue;
                };
                for (number, at, start) in [
                    (tie.sides.start, attack.end, true),
                    (tie.sides.stop, attack.start, false),
                ] {
                    let Some(number) = number else { continue; };
                    let Some(pitch) = head.written_pitch else {
                        directions[index] = None;
                        continue;
                    };
                    let key = (attack.staff, attack.voice, at, pitch, tie.alteration, number);
                    let (starts, stops) = endpoints.entry(key).or_insert_with(|| (Vec::new(), Vec::new()));
                    if start { starts.push(index); } else { stops.push(index); }
                }
            }
        }
    }
    if endpoints.is_empty() {
        return directions;
    }
    let mut neighbors = vec![Vec::new(); groups.len()];
    for (starts, stops) in endpoints.into_values() {
        if let ([left], [right]) = (starts.as_slice(), stops.as_slice()) {
            neighbors[*left].push(*right);
            neighbors[*right].push(*left);
        } else {
            for index in starts.into_iter().chain(stops) {
                directions[index] = None;
            }
        }
    }
    let mut visited = vec![false; groups.len()];
    for root in 0..groups.len() {
        if visited[root] || neighbors[root].is_empty() { continue; }
        let mut pending = vec![root];
        let mut connected = Vec::new();
        while let Some(index) = pending.pop() {
            if std::mem::replace(&mut visited[index], true) { continue; }
            connected.push(index);
            pending.extend(&neighbors[index]);
        }
        if connected.iter().any(|&index| directions[index] != directions[root]) {
            for index in connected { directions[index] = None; }
        }
    }
    directions
}

fn overlapping_directions(
    groups: &[Group<'_>],
    relations: &[Relation],
    indices: &[usize],
) -> Option<Vec<Direction>> {
    if !indices.iter().any(|&i| relations[i].distinct_timing || relations[i].flag_against_beam) {
        return None;
    }
    let mut roles = BTreeMap::new();
    let mut result = Vec::new();
    for &i in indices {
        let direction = relations[i].ordered_role()?;
        let voice = groups[i].voice();
        if roles
            .insert(voice, direction)
            .is_some_and(|old| old != direction)
        {
            return None;
        }
        result.push(direction);
    }
    (roles.values().any(|role| *role == Direction::Up)
        && roles.values().any(|role| *role == Direction::Down)).then_some(result)
}

struct BeamFootprint<'group, 'source> {
    attacks: &'group [Attack<'source>],
    start: i64,
    end: i64,
    direction: Direction,
    levels: i64,
    low: i64,
    high: i64,
}

impl<'group, 'source> BeamFootprint<'group, 'source> {
    const LEVEL_STEP: i64 = 2;

    fn new(group: &'group Group<'source>) -> Option<Self> {
        let [first, .., last] = group.attacks.as_slice() else {
            return None;
        };
        let direction = match first.heads[0].stem?.2 {
            "<stem>up</stem>" => Direction::Up,
            "<stem>down</stem>" => Direction::Down,
            _ => return None,
        };
        let mut low = i64::MAX;
        let mut high = i64::MIN;
        let mut levels = 1;
        for attack in &group.attacks {
            for head in &attack.heads {
                if head.stem?.2 != direction.xml() || !(-8..=8).contains(&head.distance) {
                    return None;
                }
                low = low.min(head.distance);
                high = high.max(head.distance);
            }
            levels = levels.max(attack.ordinary_beam_levels?);
        }
        Some(Self { attacks: &group.attacks, start: first.start, end: last.start,
            direction, levels, low, high })
    }

    fn outer_head(&self, direction: Direction) -> i64 {
        match direction {
            Direction::Up => self.high,
            Direction::Down => -self.low,
        }
    }

    fn crosses_other_stem(&self, other: &Self) -> bool {
        // Compare nominal beam heights in staff steps (half-line spaces).
        // A stem at an interior onset can run through the other voice's band
        // on its way to its own beam. Endpoint alignment alone is insufficient.
        const NOMINAL_REACH: i64 = 5;
        let height = self.outer_head(self.direction) + NOMINAL_REACH;
        let other_height = other.outer_head(self.direction) + NOMINAL_REACH;
        height - Self::LEVEL_STEP * (self.levels - 1) <= other_height
            && other.attacks.iter().any(|attack| {
                self.start < attack.start && attack.start < self.end
                    && attack.heads.iter().any(|head| self.direction.outward(head.distance) < height)
            })
    }

    fn clears(&self, other: &Self, direction: Direction) -> bool {
        // For an opposite-facing pair, a beam beyond every other head also
        // clears the other stems. Reserve room on the inward side for slope,
        // beam thickness and secondary levels of an ordinary beam.
        const PRIMARY_INNER_EDGE: i64 = 1;
        let inside_beam_edge = self.outer_head(direction) + PRIMARY_INNER_EDGE
            - Self::LEVEL_STEP * (self.levels - 1);
        // At a later onset the beam has already ended, even if that attack
        // and the beam's last note still sound together.
        other.attacks.iter().filter(|attack| (self.start..=self.end).contains(&attack.start))
            .flat_map(|attack| &attack.heads)
            .all(|head| inside_beam_edge > direction.outward(head.distance))
    }

    fn can_reverse(&self) -> bool {
        self.attacks.iter().all(|attack| attack.heads.len() == 1)
    }
}

fn separated_beam_directions(
    groups: &[Group<'_>],
    component: &[usize],
    other_symbols: &[PositionedSymbol],
) -> Option<Vec<Direction>> {
    let &[first, second] = component else { return None; };
    let left = &groups[first];
    let right = &groups[second];
    if left.staff() != right.staff() || left.voice() == right.voice()
        || left.attacks[0].clef != right.attacks[0].clef
    {
        return None;
    }
    let a = BeamFootprint::new(left)?;
    let b = BeamFootprint::new(right)?;
    let direction = a.direction;
    if direction != b.direction || !(a.crosses_other_stem(&b) || b.crosses_other_stem(&a)) {
        return None;
    }
    let occupied_span = a.start.min(b.start)..=a.end.max(b.end);
    if other_symbols.iter().any(|symbol| {
        symbol.staff.is_none_or(|staff| staff == left.staff()) && occupied_span.contains(&symbol.onset)
    }) {
        return None;
    }
    let opposite = direction.opposite();
    match (b.can_reverse() && a.clears(&b, direction) && b.clears(&a, opposite),
        a.can_reverse() && a.clears(&b, opposite) && b.clears(&a, direction))
    {
        (true, false) => Some(vec![direction, opposite]),
        (false, true) => Some(vec![opposite, direction]),
        _ => None,
    }
}

// If a larger component has no consistent pitch-role ordering, an explicit
// flag can still be separated from an adjacent beam of a different voice.
fn separate_flag_from_beam(groups: &[Group<'_>], edits: &mut Vec<StemEdit>, require_pitch_order: bool) {
    let planned: BTreeMap<_, _> = edits.iter().map(|&(start, _, value)| (start, value)).collect();
    let beam_directions = groups.iter().map(|group| {
        if group.attacks.len() < 2 || !group.topology_writable() {
            return None;
        }
        let mut directions = group.attacks.iter().filter_map(|attack| attack.heads[0].stem);
        let (mut up, mut down) = (0, 0);
        for (start, _, existing) in &mut directions {
            if planned.get(&start).copied().unwrap_or(existing) == Direction::Up.xml() {
                up += 1;
            } else {
                down += 1;
            }
        }
        if up > down { Some(Direction::Up) } else if down > up { Some(Direction::Down) } else { None }
    }).collect::<Vec<_>>();
    for group in groups {
        let [flag] = group.attacks.as_slice() else {
            continue;
        };
        if !flag.flagged || !group.writable()
            || flag.heads.iter().any(|head| head.stem.is_some_and(|(at, _, _)| planned.contains_key(&at)))
        {
            continue;
        }
        let Some((_, _, original)) = flag.heads[0].stem else {
            continue;
        };
        let mut directions = groups.iter().zip(&beam_directions).filter_map(|(beam, &direction)| {
            let direction = direction?;
            if beam.staff() != flag.staff || beam.voice() == flag.voice || !beam.writable() {
                return None;
            }
            let mut overlapping = beam.attacks.iter().filter(|attack| overlap(attack, flag)).peekable();
            if require_pitch_order && (overlapping.peek().is_none() || !overlapping.clone().all(|attack| {
                match direction {
                    Direction::Up => entirely_above(attack, flag),
                    Direction::Down => entirely_above(flag, attack),
                }
            })) {
                return None;
            }
            overlapping.any(|attack| flag.heads.iter().any(|head| {
                attack.heads.iter().any(|beam_head| {
                    head.written_pitch.zip(beam_head.written_pitch)
                        .is_some_and(|(a, b)| a.abs_diff(b) <= 7 && a != b)
                })
            })).then_some(direction)
        });
        let Some(direction) = directions.next() else { continue; };
        if original != direction.xml() || directions.any(|other| other != direction) {
            continue;
        }
        let opposite = match direction { Direction::Up => Direction::Down, Direction::Down => Direction::Up };
        edits.extend(flag.heads.iter().filter_map(|head| {
            let (start, end, _) = head.stem?;
            Some((start, end, opposite.xml()))
        }));
    }
}

// A horizontal primary beam lies on the stem side of its equal-height
// anchors. Compare other printed heads over the closed onset span, where
// their physical positions can interact with the beam. Earlier held notes
// still inform voice roles, but their heads are outside this footprint.
fn interleaved_beam_side(
    group_index: usize,
    groups: &[Group<'_>],
    other_symbols: &[PositionedSymbol],
) -> Option<Direction> {
    let group = &groups[group_index];
    let [left, .., right] = group.attacks.as_slice() else {
        return None;
    };
    let [anchor] = left.heads.as_slice() else {
        return None;
    };
    // The renderer probe qualifies the five-line staff's ordinary
    // single-level beam. Ledger-line and stacked-beam offsets vary.
    if !group.writable()
        || !(-4..=4).contains(&anchor.distance)
        || !group.attacks.iter().all(|attack| {
            attack.isolation_supported && attack.primary_only
                && matches!(attack.heads.as_slice(), [head] if head.distance == anchor.distance)
        })
        || other_symbols.iter().any(|symbol| {
            symbol.staff.is_none_or(|staff| staff == group.staff())
                && (left.start..=right.start).contains(&symbol.onset)
        })
    {
        return None;
    }

    let mut side = None;
    let mut near_interior_head = false;
    for (index, other) in groups.iter().enumerate() {
        if index == group_index || other.staff() != group.staff() {
            continue;
        }
        for attack in other.attacks.iter().filter(|attack| {
            (left.start..=right.start).contains(&attack.start)
        }) {
            if attack.clef.is_none() || other.voice() == group.voice() {
                return None;
            }
            for head in &attack.heads {
                let delta = head.distance.checked_sub(anchor.distance)?;
                let direction = match delta.cmp(&0) {
                    std::cmp::Ordering::Greater => Direction::Up,
                    std::cmp::Ordering::Less => Direction::Down,
                    std::cmp::Ordering::Equal => return None,
                };
                if side.is_some_and(|prior| prior != direction) {
                    return None;
                }
                side = Some(direction);
                // In staff steps (half-spaces), cover the observed beam height
                // plus its band and the head contour. This is a risk envelope;
                // the one-sided clearance after flipping is the invariant.
                near_interior_head |= left.start < attack.start && attack.start < right.start
                    && (4..=8).contains(&delta.unsigned_abs());
            }
        }
    }
    side.filter(|_| near_interior_head)
}

fn clear_interleaved_horizontal_beams(
    groups: &[Group<'_>],
    other_symbols: &[PositionedSymbol],
    edits: &mut Vec<StemEdit>,
) {
    let direction_candidates: Vec<_> = groups.iter().enumerate().filter_map(|(index, group)| {
        interleaved_beam_side(index, groups, other_symbols).map(|side| (group, side))
    }).collect();
    if direction_candidates.is_empty() {
        return;
    }
    let mut planned: BTreeMap<_, _> = edits.drain(..).map(|edit| (edit.0, edit)).collect();
    for (group, side) in direction_candidates {
        let affected = group.attacks.iter().filter_map(|attack| attack.heads[0].stem);
        if !affected.clone().all(|(start, _, old)| {
            planned.get(&start).map_or(old, |edit| edit.2) == side.xml()
        }) {
            continue;
        }
        let away = match side { Direction::Up => Direction::Down, Direction::Down => Direction::Up };
        for (start, end, old) in affected {
            if old == away.xml() {
                planned.remove(&start);
            } else {
                planned.insert(start, (start, end, away.xml()));
            }
        }
    }
    edits.extend(planned.into_values());
}

fn measure_edits(groups: &[Group<'_>], other_symbols: &[PositionedSymbol]) -> Vec<StemEdit> {
    let relations = compare_groups(groups);
    let isolated = isolated_directions(groups, &relations);
    let mut visited = vec![false; groups.len()];
    let mut edits = Vec::new();
    for (root, &isolated_direction) in isolated.iter().enumerate() {
        let mut pending = vec![root];
        let mut component = Vec::new();
        while let Some(i) = pending.pop() {
            if std::mem::replace(&mut visited[i], true) {
                continue;
            }
            component.push(i);
            pending.extend(relations[i].neighbors.iter().copied());
        }
        if component.is_empty() || !component.iter().all(|&i| groups[i].writable()) {
            continue;
        }
        let directions = if component.len() == 1 {
            isolated_direction.map(|direction| vec![direction])
        } else {
            overlapping_directions(groups, &relations, &component)
                .or_else(|| separated_beam_directions(groups, &component, other_symbols))
        };
        let Some(directions) = directions else {
            continue;
        };
        for (i, direction) in component.into_iter().zip(directions) {
            edits.extend(
                groups[i]
                    .attacks
                    .iter()
                    .flat_map(|attack| &attack.heads)
                    .filter_map(|head| {
                        let (start, end, existing) = head.stem?;
                        (existing != direction.xml()).then_some((start, end, direction.xml()))
                    }),
            );
        }
    }
    clear_interleaved_horizontal_beams(groups, other_symbols, &mut edits);
    separate_flag_from_beam(groups, &mut edits, false);
    edits
}

pub(super) fn plan_countervoice_stem_directions(xml: &str) -> Vec<StemEdit> {
    plan_stem_directions(xml, |groups, _| {
        let mut edits = Vec::new();
        separate_flag_from_beam(groups, &mut edits, true);
        edits
    })
}

pub(super) fn plan_final_stem_directions(xml: &str) -> Vec<StemEdit> {
    plan_stem_directions(xml, measure_edits)
}

pub(super) fn plan_tied_stem_chains(
    xml: &str,
    authenticated_chains: &[Vec<std::ops::Range<usize>>],
) -> Vec<Vec<StemEdit>> {
    tie_chain::plan(xml, authenticated_chains)
}

fn plan_stem_directions(xml: &str, decide: impl Fn(&[Group<'_>], &[PositionedSymbol]) -> Vec<StemEdit>) -> Vec<StemEdit> {
    let Ok(document) = Document::parse(xml) else {
        return Vec::new();
    };
    let mut edits = Vec::new();
    for part in document
        .root_element()
        .children()
        .filter(|n| n.has_tag_name("part"))
    {
        let mut context = PrintContext::default();
        for measure in part.children().filter(|n| n.has_tag_name("measure")) {
            let print = context.for_measure(measure);
            if print.iter().any(Option::is_some)
                && let Some(printed) = collect_attacks(xml, measure, print)
            {
                edits.extend(decide(&beam_groups(printed.attacks), &printed.other_symbols));
            }
        }
    }
    edits.sort_unstable_by_key(|edit| edit.0);
    edits
}

pub(super) fn matches_planned_stem_edit(source: &str, post: &str, edits: &[StemEdit]) -> bool {
    let mut old = 0;
    let mut new = 0;
    for &(start, end, replacement) in edits {
        let unchanged = start - old;
        if source.get(old..start) != post.get(new..new + unchanged) {
            return false;
        }
        new += unchanged;
        if post.get(new..new + replacement.len()) != Some(replacement) {
            return false;
        }
        new += replacement.len();
        old = end;
    }
    source.get(old..) == post.get(new..)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(body: &str) -> String {
        format!(
            "<score-partwise><part id=\"P\"><measure number=\"synthetic\"><attributes><divisions>8</divisions><clef number=\"1\"><sign>G</sign><line>2</line></clef></attributes>{body}</measure></part></score-partwise>"
        )
    }

    fn note(step: &str, duration: u8, voice: u8, stem: &str, beam: &str) -> String {
        format!(
            "<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><stem>{stem}</stem>{beam}</note>"
        )
    }

    fn preview(source: &str) -> String {
        super::super::project_final_stem_directions(source).unwrap()
            .map_or_else(|| source.to_owned(), |(output, receipt)| {
                receipt.validate_preserving_transition::<super::super::NotationPolishPreservingMint>(source, &output).unwrap();
                output
            })
    }

    #[test]
    fn duration_defaults_only_apply_to_notes_without_duration() {
        for (xml, expected_duration, expected_cursor) in [
            ("<note><grace/></note>", Some(0), Some(4)),
            ("<note><chord/><duration>2</duration></note>", Some(2), Some(4)),
            ("<note><duration>2</duration></note>", Some(2), Some(6)),
            ("<note><duration>-1</duration></note>", None, None),
            ("<note><duration>invalid</duration></note>", None, None),
            ("<forward/>", None, None),
            ("<backup/>", None, None),
            ("<backup><duration>5</duration></backup>", Some(5), None),
        ] {
            let document = Document::parse(xml).unwrap();
            let node = document.root_element();
            assert_eq!(rhythmic_duration(node), expected_duration, "{xml}");
            assert_eq!(rhythmic_cursor(node, 4), expected_cursor, "{xml}");
        }
    }

    #[test]
    fn final_stem_direction_treats_the_complete_beam_with_its_countervoice() {
        let beam_a = note("F", 2, 7, "down", "<beam number=\"1\">begin</beam>");
        let beam_b = note("E", 2, 7, "up", "<beam number=\"1\">end</beam>");
        let lower = note("C", 4, 2, "up", "");
        let xml = score(&format!(
            "{beam_a}{beam_b}<backup><duration>4</duration></backup>{lower}"
        ));
        let expected = score(&format!(
            "{}{beam_b}<backup><duration>4</duration></backup>{}",
            beam_a.replace("<stem>down</stem>", "<stem>up</stem>"),
            lower.replace("<stem>up</stem>", "<stem>down</stem>")
        ));
        assert_eq!(preview(&xml), expected);
        let edits = plan_final_stem_directions(&xml);
        assert!(matches_planned_stem_edit(&xml, &expected, &edits));
        assert!(!matches_planned_stem_edit(
            &xml,
            &expected.replace("<voice>2</voice>", "<voice>4</voice>"),
            &edits,
        ));
        assert_eq!(preview(&expected), expected);
    }

    #[test]
    fn final_stem_direction_joins_isolated_heads_and_guards_unknown_roles() {
        let first = note("F", 2, 1, "up", "<beam number=\"1\">begin</beam>");
        let last = note("E", 2, 1, "up", "<beam number=\"1\">end</beam>");
        let xml = score(&format!("{first}{last}"));
        assert_eq!(
            preview(&xml),
            xml.replace("<stem>up</stem>", "<stem>down</stem>")
        );
        for unresolved in [
            score(&format!(
                "{first}<forward><duration>2</duration></forward>{last}"
            )),
            score(&format!(
                "{first}{last}<backup><duration>4</duration></backup>{}",
                note("F", 4, 2, "up", "")
            )),
            score(&format!(
                "<direction><direction-type><octave-shift type=\"down\"/></direction-type></direction>{first}{last}"
            )),
        ] {
            assert_eq!(preview(&unresolved), unresolved);
        }
    }

    #[test]
    fn flagged_countervoice_uses_opposite_stem_with_equal_attacks_under_carried_octave() {
        let first = note("F", 2, 7, "up", "<beam number=\"1\">begin</beam>");
        let last = note("E", 2, 7, "up", "<beam number=\"1\">end</beam>");
        let flag = note("C", 2, 2, "up", "<type>16th</type>");
        let body = format!("{first}{last}<backup><duration>4</duration></backup>{flag}");
        let base = score(&body);
        let carried = base.replace(&body, &format!(
            "<direction><direction-type><octave-shift type=\"down\" size=\"8\"/></direction-type></direction>{}</measure><measure number=\"next\">{body}",
            note("G", 2, 7, "up", ""),
        ));
        for input in [base, carried] {
            let expected = input.replace(&flag, &flag.replace("<stem>up</stem>", "<stem>down</stem>"));
            assert_eq!(preview(&input), expected);
            let [(start, end, direction)] = plan_countervoice_stem_directions(&input)[..] else {
                panic!("only the independent flagged stem must change");
            };
            assert_eq!(format!("{}{direction}{}", &input[..start], &input[end..]), expected);
            assert!(plan_countervoice_stem_directions(&expected).is_empty());
        }
    }

    #[test]
    fn flat_beam_uses_the_clear_side_of_interleaved_heads() {
        for (pitch, counter, held, toward, away) in [
            (("G", 4), ("F", 5), ("C", 4), "up", "down"),
            (("E", 5), ("F", 4), ("C", 6), "down", "up"),
        ] {
            let at = |(step, octave), duration, voice, stem, notation| {
                note(step, duration, voice, stem, notation)
                    .replace("<octave>5</octave>", &format!("<octave>{octave}</octave>"))
            };
            let beam = [
                "<beam number=\"1\">begin</beam>",
                "<beam number=\"1\">continue</beam>",
                "<beam number=\"1\">end</beam>",
            ].map(|mark| at(pitch, 3, 7, toward, mark)).concat();
            let sustained = at(held, 11, 3, away, "");
            let nearby = at(counter, 1, 5, toward, "");
            let prefix = format!(
                "<forward><duration>2</duration></forward>{beam}<backup><duration>11</duration></backup>{sustained}<backup><duration>7</duration></backup>"
            );
            let input = score(&format!("{prefix}{nearby}"));
            let fixed_beam = beam.replace(
                &format!("<stem>{toward}</stem>"),
                &format!("<stem>{away}</stem>"),
            );
            let expected = input.replace(&beam, &fixed_beam);
            assert_eq!(preview(&input), expected);
            assert_eq!(preview(&expected), expected);
            let repeated = score(&format!(
                "{prefix}{nearby}</measure><measure number=\"copy\">{prefix}{nearby}",
            ));
            assert_eq!(preview(&repeated), repeated.replace(&beam, &fixed_beam));

            let opposite_head = at(held, 1, 5, away, "");
            for blocked in [
                score(&format!("{prefix}{nearby}{opposite_head}")),
                score(&format!("{prefix}{nearby}<note><rest/><duration>1</duration><voice>5</voice></note>")),
                input.replace("<duration>7</duration></backup>", "<duration>9</duration></backup>"),
            ] {
                assert!(preview(&blocked).contains(&beam));
            }
        }
    }

    #[test]
    fn independent_beams_separate_on_the_clear_side() {
        for (mirror, reverse_serialization, fast_voice, slow_voice) in
            [(false, false, 3, 7), (true, true, 8, 2)]
        {
            let toward = if mirror { "down" } else { "up" };
            let away = if mirror { "up" } else { "down" };
            let build = |pitches: &[(&str, &str)], duration, voice, direction, double: bool| {
                pitches.iter().enumerate().map(|(index, &(pitch, reflected))| {
                    let pitch = if mirror { reflected } else { pitch };
                    let mark = match index {
                        0 => "begin",
                        i if i + 1 == pitches.len() => "end",
                        _ => "continue",
                    };
                    let secondary = if !double { "" } else if index % 2 == 0 {
                        "<beam number=\"2\">begin</beam>"
                    } else {
                        "<beam number=\"2\">end</beam>"
                    };
                    note(&pitch[..1], duration, voice, direction, &format!(
                        "<beam number=\"1\">{mark}</beam>{secondary}"
                    )).replace("<octave>5</octave>", &format!("<octave>{}</octave>", &pitch[1..]))
                }).collect::<String>()
            };
            let slow = build(&[("D5", "G4"), ("C5", "A4"), ("E5", "F4")], 2, slow_voice, toward, false);
            let changed = slow.replace(&format!("<stem>{toward}</stem>"), &format!("<stem>{away}</stem>"));
            let fast_pitches = [("E5", "F4"), ("G5", "D4"), ("D5", "G4"),
                ("D5", "G4"), ("C5", "A4"), ("E5", "F4")];
            let fast = build(&fast_pitches, 1, fast_voice, toward, true);
            let overlap = |fast: &str, slow: &str| {
                let (first, second) = if reverse_serialization { (slow, fast) } else { (fast, slow) };
                format!("{first}<backup><duration>6</duration></backup>{second}")
            };
            let input = score(&overlap(&fast, &slow));
            let expected = score(&overlap(&fast, &changed));
            assert_eq!(preview(&input), expected);
            assert_eq!(preview(&expected), expected);

            let mut blocked_pitches = fast_pitches;
            blocked_pitches[3] = ("G4", "D5");
            let blocked_fast = build(&blocked_pitches, 1, fast_voice, toward, true);
            let incomplete = fast.replace("<beam number=\"1\">end</beam>", "<beam number=\"1\">continue</beam>");
            let rest = "<backup><duration>4</duration></backup><note><rest/><duration>1</duration><voice>9</voice></note>";
            for blocked in [
                score(&overlap(&blocked_fast, &slow)),
                score(&overlap(&incomplete, &slow)),
                score(&format!("{}{rest}", overlap(&fast, &slow))),
            ] {
                assert_eq!(preview(&blocked), blocked);
            }
        }
    }

    #[test]
    fn a_tie_across_another_voices_beam_boundary_establishes_independent_timing() {
        let begin = "<beam number=\"1\">begin</beam>";
        let end = "<beam number=\"1\">end</beam>";
        let upper = format!(
            "{}{}{}{}",
            note("F", 2, 7, "up", begin),
            note("E", 2, 7, "up", end),
            note("F", 2, 7, "up", begin),
            note("E", 2, 7, "up", end),
        );
        let start = "<tie type=\"start\"/>";
        let stop = "<tie type=\"stop\"/>";
        let fragment = |step, stem, tie| {
            note(step, 2, 2, stem, "").replace("<voice>", &format!("{tie}<voice>"))
        };
        let lower = |stem, start, stop| {
            format!("{}{}", fragment("C", stem, start), fragment("C", stem, stop))
        };
        let voices = |countervoice| {
            score(&format!(
                "{upper}<backup><duration>6</duration></backup>{countervoice}"
            ))
        };
        let source = voices(lower("up", start, stop));
        let expected = voices(lower("down", start, stop));
        assert_eq!(preview(&source), expected);
        assert_eq!(preview(&expected), expected);
        for unproven in [
            voices(lower("up", "", "")),
            voices(lower("up", start, "")),
            source.replace("<tie type=\"stop\"/>", "<tie type=\"start\"/>"),
            voices(format!(
                "{}{}",
                fragment("C", "up", start),
                fragment("D", "up", stop),
            )),
        ] {
            assert_eq!(preview(&unproven), unproven);
        }
    }

    #[test]
    fn bounded_octave_shift_preserves_the_order_of_overlapping_voices() {
        let start = "<direction><direction-type><octave-shift type=\"down\" size=\"8\"/></direction-type></direction>";
        let stop = "<direction><direction-type><octave-shift type=\"stop\"/></direction-type></direction>";
        let first = note("F", 2, 7, "up", "<beam number=\"1\">begin</beam>");
        let last = note("E", 2, 7, "up", "<beam number=\"1\">end</beam>");
        let lower = note("C", 4, 2, "up", "");
        let voices = format!("{first}{last}<backup><duration>4</duration></backup>{lower}");
        let source = score(&format!("{start}{voices}{stop}"));
        let expected = source.replace(&lower, &lower.replace("<stem>up</stem>", "<stem>down</stem>"));
        assert_eq!(preview(&source), expected);
        for unresolved in [
            score(&format!("{start}{first}{stop}{last}<backup><duration>4</duration></backup>{lower}")),
            source.replace("type=\"stop\"", "type=\"stop\" number=\"2\""),
            source.replace("</direction-type>", "</direction-type><offset>1</offset>"),
            score(&format!("{start}{first}{last}{stop}")).replace("<stem>up</stem>", "<stem>down</stem>"),
        ] {
            assert_eq!(preview(&unresolved), unresolved);
        }
    }

    #[test]
    fn a_tie_into_an_independent_beam_resolves_its_leading_note() {
        let begin = "<beam number=\"1\">begin</beam>";
        let end = "<beam number=\"1\">end</beam>";
        let upper = format!(
            "{}{}{}{}",
            note("F", 2, 7, "up", begin),
            note("E", 2, 7, "up", end),
            note("F", 2, 7, "up", begin),
            note("E", 4, 7, "up", end),
        );
        let lead = note("C", 2, 2, "up", "")
            .replace("<voice>", "<tie type=\"start\"/><voice>");
        let continuation = note("C", 2, 2, "down", begin)
            .replace("<voice>", "<tie type=\"stop\"/><voice>");
        let lower_end = note("D", 3, 2, "down", end);
        let source = score(&format!(
            "{upper}<backup><duration>8</duration></backup>{lead}{continuation}{lower_end}"
        ));
        let expected = source.replace(&lead, &lead.replace("<stem>up</stem>", "<stem>down</stem>"));
        assert_eq!(preview(&source), expected);
        assert_eq!(preview(&expected), expected);
        for unresolved in [
            source.replace("<tie type=\"stop\"/>", ""),
            source.replace(&lower_end, &note("D", 4, 2, "down", end)),
        ] {
            assert_eq!(preview(&unresolved), unresolved);
        }
    }

    const BASS: &str = "<attributes><clef><sign>F</sign><line>4</line></clef></attributes>";
    const TREBLE: &str = "<attributes><clef><sign>G</sign><line>2</line></clef></attributes>";

    fn beam_at_pitch(step: &str, octave: u8, duration: u8, stem: &str) -> String {
        ["begin", "end"].map(|mark| {
            note(step, duration, 1, stem, &format!("<beam>{mark}</beam>"))
                .replace("<octave>5</octave>", &format!("<octave>{octave}</octave>"))
        }).concat()
    }

    #[test]
    fn independent_beams_use_the_clef_at_their_own_time_and_carry_it_forward() {
        // All pitches lie between the two middle lines. Changing clef must reverse
        // the isolated direction even when the written pitch stays the same.
        for (step, octave, duration) in [("E", 3, 1), ("A", 3, 3), ("G", 4, 2)] {
            let down = beam_at_pitch(step, octave, duration, "down");
            let up = beam_at_pitch(step, octave, duration, "up");
            assert_eq!(preview(&score(&down)), score(&up));
            for copies in [1, 2] {
                let early_input = down.repeat(copies);
                let early_result = up.repeat(copies);
                let next_measure = "</measure><measure number=\"continuation\">";
                let elapsed = (copies + 1) * 2 * usize::from(duration);
                // A later XML voice may restate the opening clef at musical time zero.
                for rewind in [String::new(), format!(
                    "<backup><duration>{elapsed}</duration></backup>{TREBLE}<forward><duration>{elapsed}</duration></forward>"
                )] {
                    let input = score(&format!("{early_input}{BASS}{up}{rewind}{next_measure}{up}"));
                    let expected = score(&format!("{early_result}{BASS}{down}{rewind}{next_measure}{down}"));
                    assert_eq!(preview(&input), expected);
                    assert_eq!(preview(&expected), expected);
                    assert!(super::super::project_master_countervoice_stems(&input).unwrap().is_none());
                }
            }
        }
    }

    #[test]
    fn countervoice_roles_follow_musical_time_across_serialized_voices() {
        for (beam_voice, flag_voice, duration) in [(2, 6, 1), (8, 3, 3)] {
            let beam = format!("{}{}", note("F", duration, beam_voice, "up", "<beam>begin</beam>"),
                note("E", duration, beam_voice, "up", "<beam>end</beam>"));
            let flag = note("C", duration, flag_voice, "up", "").replace("<stem>", "<type>16th</type><stem>");
            let forward = format!("<forward><duration>{duration}</duration></forward>");
            let rewind = format!("<backup><duration>{}</duration></backup>", 2 * duration);
            for body in [
                format!("{beam}{BASS}{rewind}{flag}{forward}"),
                format!("{flag}{forward}{BASS}{rewind}{beam}"),
            ] {
                let source = score(&body);
                let expected = source.replace(&flag, &flag.replace("<stem>up</stem>", "<stem>down</stem>"));
                let (master, receipt) = super::super::project_master_countervoice_stems(&source)
                    .unwrap().expect("lower independent flag needs the opposite direction");
                receipt.validate_preserving_transition::<super::super::NotationPolishPreservingMint>(&source, &master).unwrap();
                assert_eq!(master, expected);
                assert_eq!(preview(&source), expected);
                assert!(super::super::project_master_countervoice_stems(&master).unwrap().is_none());
                assert_eq!(preview(&expected), expected);
            }
        }
    }

    #[test]
    fn unclear_clef_and_crossing_beam_leave_separate_material_writable() {
        let early = beam_at_pitch("G", 4, 2, "down");
        let fixed_early = beam_at_pitch("G", 4, 2, "up");
        let after = beam_at_pitch("A", 3, 2, "up");
        let fixed_after = beam_at_pitch("A", 3, 2, "down");
        let uncertain = beam_at_pitch("E", 5, 3, "up");
        let unsupported = "<attributes><clef><sign>C</sign><line>3</line></clef></attributes>";
        for middle in [
            format!("{unsupported}{uncertain}{BASS}"),
            format!("{TREBLE}{BASS}{uncertain}{BASS}"),
            format!("{}{BASS}{}", note("F", 3, 1, "up", "<beam>begin</beam>"),
                note("E", 3, 1, "up", "<beam>end</beam>")),
        ] {
            let source = score(&format!("{early}{middle}{after}"));
            let expected = score(&format!("{fixed_early}{middle}{fixed_after}"));
            assert_eq!(preview(&source), expected);
            assert_eq!(preview(&expected), expected);
        }
    }

    #[test]
    fn divisions_redeclaration_preserves_known_clefs_for_the_following_measure() {
        let treble_wrong = beam_at_pitch("G", 4, 2, "down").replacen("<stem>down</stem>", "<stem>up</stem>", 1);
        let treble_fixed = beam_at_pitch("G", 4, 2, "up");
        let bass_wrong = beam_at_pitch("G", 4, 2, "up")
            .replace("<voice>1</voice>", "<voice>3</voice>")
            .replace("<beam>", "<staff>2</staff><beam>");
        let bass_fixed = bass_wrong.replace("<stem>up</stem>", "<stem>down</stem>");
        let second_staff = BASS.replace("<clef>", "<staves>2</staves><clef number=\"2\">");
        let voices = format!("{treble_wrong}<backup><duration>4</duration></backup>{bass_wrong}");
        let next_measure = "</measure><measure number=\"continuation\">";
        for divisions in [8, 12] {
            let changed = format!("<attributes><divisions>{divisions}</divisions></attributes>");
            for (late_clef, next_treble) in [("", &treble_fixed), (TREBLE, &treble_fixed), (BASS, &treble_wrong)] {
                let prefix = format!("{second_staff}{voices}{changed}{late_clef}{next_measure}");
                let input = score(&format!("{prefix}{voices}"));
                let expected = score(&format!(
                    "{prefix}{next_treble}<backup><duration>4</duration></backup>{bass_fixed}"
                ));
                assert_eq!(preview(&input), expected);
                assert_eq!(preview(&expected), expected);
            }
        }
    }

    #[test]
    fn isolated_stem_evidence_does_not_move_one_endpoint_of_a_tie_between_groups() {
        for (step, octave, other_octave, toward, away, duration) in [
            ("E", 5, 4, "up", "down", 1),
            ("G", 4, 5, "down", "up", 3),
        ] {
            let at = |octave: u8, beam: &str| {
                note(step, duration, 4, toward, beam)
                    .replace("<octave>5</octave>", &format!("<octave>{octave}</octave>"))
                    .replace("<stem>", "<type>eighth</type><stem>")
            };
            let tied = |head: &str, end: &str| {
                head.replace("<voice>", &format!("<tie type=\"{end}\"/><voice>"))
                    .replace("</note>", &format!("<notations><tied type=\"{end}\"/></notations></note>"))
            };
            let single = at(octave, "");
            let free = single.replace("<voice>4</voice>", "<voice>6</voice>");
            let opposite = |head: &str| head.replace(
                &format!("<stem>{toward}</stem>"), &format!("<stem>{away}</stem>"),
            );
            // The beam straddles the middle line, so neither of its two heads
            // supplies an isolated direction for the complete group.
            for single_first in [true, false] {
                let (plain, changed, connected) = if single_first {
                    let anchor = at(octave, "<beam>begin</beam>");
                    let remote = at(other_octave, "<beam>end</beam>");
                    (format!("{single}{anchor}{remote}"), format!("{}{anchor}{remote}", opposite(&single)),
                     format!("{}{}{remote}", tied(&single, "start"), tied(&anchor, "stop")))
                } else {
                    let remote = at(other_octave, "<beam>begin</beam>");
                    let anchor = at(octave, "<beam>end</beam>");
                    (format!("{remote}{anchor}{single}"), format!("{remote}{anchor}{}", opposite(&single)),
                     format!("{remote}{}{}", tied(&anchor, "start"), tied(&single, "stop")))
                };
                let projected = |phrase: &str, tail: &str| score(&format!("{phrase}{tail}"))
                    .replace("<divisions>8</divisions>", &format!("<divisions>{}</divisions>", 2 * duration));
                let fixed_free = opposite(&free);
                assert_eq!(preview(&projected(&plain, &free)), projected(&changed, &fixed_free));
                let input = projected(&connected, &free);
                let expected = projected(&connected, &fixed_free);
                assert_eq!(preview(&input), expected);
                assert_eq!(preview(&expected), expected);

                // Both independent courses can agree on a shared direction.
                let same_side = connected.replace(
                    &format!("<octave>{other_octave}</octave>"),
                    &format!("<octave>{octave}</octave>"),
                );
                let consistent = projected(&same_side, &free);
                let both = opposite(&consistent);
                assert_eq!(preview(&consistent), both);
                assert_eq!(preview(&both), both);
            }
        }

        // On opposite sides of a clef boundary both local courses can be
        // decisive, yet prescribe incompatible stems to the same tied pitch.
        let low = |stem: &str, beam: &str, ties: &str| note("A", 4, 1, stem, beam)
            .replace("<octave>5</octave>", "<octave>3</octave>")
            .replace("<voice>", &format!("{ties}<voice>"))
            .replace("<stem>", "<type>eighth</type><stem>");
        let left = low("down", "", "<tie type=\"start\"/>");
        let anchor = low("up", "<beam>begin</beam>", "<tie type=\"stop\"/>");
        let end = low("up", "<beam>end</beam>", "");
        let disagreement = score(&format!("{left}{BASS}{anchor}{end}"));
        assert_eq!(preview(&disagreement), disagreement);
    }
}
