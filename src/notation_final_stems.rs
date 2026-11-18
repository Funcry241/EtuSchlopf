use crate::clef_readability_observation::{PianoClef, observe_pitch};
use roxmltree::{Document, Node};
use std::collections::BTreeMap;

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
}

struct Head {
    distance: i64,
    stem: Option<StemEdit>,
    written_pitch: Option<i64>,
}

struct Attack<'a> {
    start: i64,
    end: i64,
    staff: usize,
    voice: &'a str,
    beam: Option<&'a str>,
    flagged: bool,
    tie: Option<TieEndpoint>,
    valid: bool,
    isolation_supported: bool,
    heads: Vec<Head>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TieEndpoint {
    Start(i32),
    Stop(i32),
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
        self.topology_writable() && self.attacks.iter().all(|attack| attack.valid)
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
    clef: PianoClef,
    octave: Vec<OctaveSpan>,
    has_octave_context: bool,
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
        let mut stable = [true; 2];
        let mut cursor = measure
            .children()
            .filter(|node| node.has_tag_name("direction"))
            .any(|node| octave_markers(node).next().is_some())
            .then_some(0);
        let mut sounding = false;
        for node in measure.children().filter(Node::is_element) {
            match node.tag_name().name() {
                "note" | "forward" | "backup" => {
                    sounding = true;
                    cursor = cursor.and_then(|at| rhythmic_cursor(node, at));
                }
                "attributes" => {
                    if sounding && child(node, "divisions").is_some() {
                        stable = [false; 2];
                    }
                    for clef in node.children().filter(|n| n.has_tag_name("clef")) {
                        if let Some(staff) = staff_index(clef.attribute("number").unwrap_or("1")) {
                            let next = printed_clef(clef);
                            stable[staff] &= !sounding || self.clefs[staff] == next;
                            self.clefs[staff] = next;
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
            let clef = self.clefs[staff].filter(|_| stable[staff])?;
            let has_octave_context = !carried[staff].is_empty() || !events[staff].is_empty();
            let octave = measure_octave_course(carried[staff].clone(), &events[staff])?;
            Some(StaffPrint { clef, octave, has_octave_context })
        })
    }
}

fn collect_attacks<'a>(
    xml: &str,
    measure: Node<'a, '_>,
    print: [Option<StaffPrint>; 2],
) -> Option<Vec<Attack<'a>>> {
    let mut attacks: Vec<Attack<'a>> = Vec::new();
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
        if child(node, "grace").is_some() {
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
        if child(node, "pitch").is_none() || node.attribute("print-object") == Some("no") {
            continue;
        }
        let staff = staff_index(field(node, "staff").unwrap_or("1"))?;
        let voice = field(node, "voice").unwrap_or("1");
        let start = if is_chord {
            attacks[attachment?].start
        } else {
            start
        };
        let end = start.checked_add(duration)?;
        let pitch = observe_pitch(xml.get(node.range())?);
        let distance = print[staff].as_ref().and_then(|context| {
            let steps = context.octave.iter()
                .find(|span| span.start <= start && end <= span.stop)?.steps;
            Some(context.clef.distance_from_middle_line(pitch?.shifted(steps)))
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
        let beam = match primary {
            Some(primary) => Some(primary.text()?.trim()),
            None if beams.next().is_some() => Some("unbound"),
            None => None,
        };
        let head = Head {
            distance: distance.unwrap_or(0),
            stem,
            written_pitch: pitch.map(|pitch| PianoClef::TrebleG2.distance_from_middle_line(pitch)),
        };
        let valid = distance.is_some() && child(node, "grace").is_none();
        if is_chord {
            let i = attachment?;
            let attack = &mut attacks[i];
            attack.valid &= valid
                && attack.staff == staff
                && attack.voice == voice
                && attack.end == end
                && beam.is_none();
            if attack.staff != staff || attack.voice != voice || attack.end != end || beam.is_some() {
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
                flagged: beam.is_none() && matches!(field(node, "type"),
                    Some("eighth" | "16th" | "32nd" | "64th" | "128th" | "256th")),
                tie: tie_endpoint(node),
                valid,
                isolation_supported: print[staff].as_ref().is_some_and(|context| !context.has_octave_context),
                heads: vec![head],
            });
        }
    }
    Some(attacks)
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
        if !flag.flagged || !flag.valid || !group.topology_writable()
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

fn measure_edits(groups: &[Group<'_>]) -> Vec<StemEdit> {
    let relations = compare_groups(groups);
    let mut visited = vec![false; groups.len()];
    let mut edits = Vec::new();
    for root in 0..groups.len() {
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
            isolated_direction(&groups[root]).map(|direction| vec![direction])
        } else {
            overlapping_directions(groups, &relations, &component)
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
    separate_flag_from_beam(groups, &mut edits, false);
    edits
}

pub(super) fn plan_countervoice_stem_directions(xml: &str) -> Vec<StemEdit> {
    plan_stem_directions(xml, |groups| {
        let mut edits = Vec::new();
        separate_flag_from_beam(groups, &mut edits, true);
        edits
    })
}

pub(super) fn plan_final_stem_directions(xml: &str) -> Vec<StemEdit> {
    plan_stem_directions(xml, measure_edits)
}

fn plan_stem_directions(xml: &str, decide: impl Fn(&[Group<'_>]) -> Vec<StemEdit>) -> Vec<StemEdit> {
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
                && let Some(attacks) = collect_attacks(xml, measure, print)
            {
                edits.extend(decide(&beam_groups(attacks)));
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
}
