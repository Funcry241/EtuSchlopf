//! C3 adapter: same-epoch joint reading input -> checked finite-course kernel.
//! Read-only. No writer, no adoption, no origin issue, no identity minting.
//! Every unsupported or unproved fact keeps the complete incumbent course of the
//! affected staff domain and is reported as `Unknown`.
//!
//! Temporal semantics follow the existing current-document observer
//! (`clef_readability_observation/temporal.rs`): an octave shift applies to every
//! note whose onset is at or after its effective time (`range(..=at)`), with
//! `<offset>` added to the cursor; `continue` is a reminder, not a state change.
//! Same-time ordering that this rule cannot decide is Unknown, not guessed.
use super::c3_course as k;
use std::collections::{BTreeMap, BTreeSet};

/// Exact signed rational in quarter notes; negative values are offsets only.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Q {
    n: i128,
    d: i128,
}
fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
impl Q {
    pub const ZERO: Q = Q { n: 0, d: 1 };
    pub fn new(n: i128, d: i128) -> Option<Q> {
        if d <= 0 {
            return None;
        }
        let g = gcd(n.checked_abs()?, d).max(1);
        Some(Q { n: n / g, d: d / g })
    }
    fn add(self, o: Q) -> Option<Q> {
        Q::new(self.n.checked_mul(o.d)?.checked_add(o.n.checked_mul(self.d)?)?, self.d.checked_mul(o.d)?)
    }
    fn sub(self, o: Q) -> Option<Q> {
        Q::new(self.n.checked_mul(o.d)?.checked_sub(o.n.checked_mul(self.d)?)?, self.d.checked_mul(o.d)?)
    }
    fn div(self, o: Q) -> Option<Q> {
        if o.n <= 0 {
            return None;
        }
        Q::new(self.n.checked_mul(o.d)?, self.d.checked_mul(o.n)?)
    }
    pub fn parts(self) -> (i128, i128) {
        (self.n, self.d)
    }
}
impl PartialOrd<Q> for Q {
    fn partial_cmp(&self, o: &Q) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Q {
    fn cmp(&self, o: &Q) -> std::cmp::Ordering {
        let signs = self.n.signum().cmp(&o.n.signum());
        if signs != std::cmp::Ordering::Equal {
            return signs;
        }
        // Reuse the kernel's overflow-free continued-fraction comparison.
        // Q's private fields and constructor guarantee positive denominators.
        let a = k::Time::new(self.n.unsigned_abs(), self.d as u128)
            .expect("Q has a positive denominator");
        let b = k::Time::new(o.n.unsigned_abs(), o.d as u128)
            .expect("Q has a positive denominator");
        let order = a.cmp(&b);
        if self.n < 0 { order.reverse() } else { order }
    }
}

// ---------------------------------------------------------------- input ----

/// Plain projection of one `JointReadingOwnerHead`. Built only by the glue from
/// the carried input (and by synthetic tests); never reconstructed from XML.
#[derive(Clone, Debug)]
pub struct AdapterHead {
    pub span: std::ops::Range<usize>,
    pub measure: usize,
    /// Existing carrier field: measure-local raw `<duration>` cursor.
    pub onset: i64,
    pub staff: String,
    pub visible: bool,
    pub diatonic: i32,
    pub continuation: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdapterGroupKind {
    PrimaryBeam,
    Chord,
    TiedAttack,
}

#[derive(Clone, Debug)]
pub struct AdapterGroup {
    pub kind: AdapterGroupKind,
    pub members: Vec<usize>,
}

/// Capabilities of the existing writers at one exact musical boundary.
/// Start and stop are separate because the octave writer uses different anchors.
#[derive(Clone, Copy, Debug, Default)]
pub struct BoundaryCapability {
    pub clef: bool,
    pub octave_start: bool,
    pub octave_stop: bool,
}

/// Evidence-bearing materialization capability of one staff. There is no
/// default and no "all states" constructor: the existing writers must supply it.
#[derive(Clone, Debug)]
pub struct StaffCapability {
    /// Name of the existing owner/receipt that proves the capability.
    pub evidence: String,
    /// Register states the existing materializers can write on this staff.
    pub writable: Vec<k::State>,
    /// Exact source-bound boundary permissions, supplied by the actual owners.
    pub boundaries: BTreeMap<Q, BoundaryCapability>,
    /// States executable at the beginning of the complete comparison domain.
    pub entry: Vec<k::State>,
    /// A non-loco final state needs the actual octave owner's terminal stop.
    pub exit_octave_stop: bool,
    /// Existing clef surfaces the original owner cannot remove.
    pub fixed_clefs: BTreeMap<Q, u8>,
    pub clef_change_inside_primary_beam: bool,
    pub clef_change_inside_tie: bool,
    pub octave_change_inside_primary_beam: bool,
    pub octave_change_inside_tie: bool,
    /// A new boundary at an existing register event in silence (no head onset).
    pub boundary_in_silence: bool,
    /// A new boundary whose only anchor is a hidden head.
    pub boundary_at_hidden_head_only: bool,
}

#[derive(Clone, Debug)]
pub struct Capabilities {
    source: String,
    pub staves: BTreeMap<String, StaffCapability>,
}
impl Capabilities {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_owned(), staves: BTreeMap::new() }
    }
}

// --------------------------------------------------------------- output ----

#[derive(Clone, Debug)]
pub struct LayerAnchor {
    /// Exact quarter-note time from the piece start.
    pub at: Q,
    /// Structural measure ordinal (1-based) containing `at`.
    pub measure: usize,
    /// Input head indices whose onset is this layer start.
    pub heads: Vec<usize>,
    /// Byte starts of existing register elements effective at this time.
    pub incumbent_events: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct SolvedStaff {
    pub domain: k::Domain,
    pub anchors: Vec<LayerAnchor>,
    pub outcome: k::Outcome,
}

#[derive(Clone, Debug)]
pub enum StaffVerdict {
    Solved(Box<SolvedStaff>),
    Unknown { reason: String },
}

#[derive(Clone, Debug)]
pub struct C3Proposal {
    pub staves: BTreeMap<String, StaffVerdict>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdapterError {
    /// The carried input does not bind this document; the whole input is Unknown.
    Binding(String),
}

// ------------------------------------------------------ document observer ----

#[derive(Clone, Debug)]
struct Measure {
    start: Q,
    extent: Q,
    nominal: Q,
}

#[derive(Clone, Debug)]
struct ObservedNote {
    span: std::ops::Range<usize>,
    visible: bool,
    measure: usize,
    raw_onset: i64,
    at: Q,
    staff: String,
    pitched: bool,
    after: Q,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EventKind {
    Clef(u8),
    Octave(i32),
}

#[derive(Clone, Debug)]
struct RegisterEvent {
    at: Q,
    pos: usize,
    staff: String,
    kind: EventKind,
    initial: bool,
}

struct Observed {
    measures: Vec<Measure>,
    notes: BTreeMap<usize, ObservedNote>,
    events: Vec<RegisterEvent>,
    unsupported: BTreeMap<String, String>,
    end: Q,
}

fn child_text<'a>(n: roxmltree::Node<'a, 'a>, name: &str) -> Option<&'a str> {
    n.children().find(|c| c.has_tag_name(name))?.text().map(str::trim)
}

fn observe(source: &str) -> Result<Observed, String> {
    let doc = roxmltree::Document::parse(source).map_err(|e| format!("xml: {e}"))?;
    let mut parts = doc.root_element().children().filter(|n| n.has_tag_name("part"));
    let part = parts.next().ok_or("no part")?;
    if parts.next().is_some() {
        return Err("more than one part".into());
    }
    let mut divisions: Option<i128> = None;
    let mut beats: Option<(i128, i128)> = None;
    let mut base = Q::ZERO;
    let mut out = Observed {
        measures: Vec::new(),
        notes: BTreeMap::new(),
        events: Vec::new(),
        unsupported: BTreeMap::new(),
        end: Q::ZERO,
    };
    let mut any_note = false;
    for (mi, m) in part.children().filter(|n| n.has_tag_name("measure")).enumerate() {
        let ordinal = mi + 1;
        let mut cursor = Q::ZERO;
        let mut raw_cursor = 0i64;
        let mut extent = Q::ZERO;
        let mut last_head: Option<(Q, i64)> = None;
        for n in m.children().filter(|n| n.is_element()) {
            let pos = n.range().start;
            match n.tag_name().name() {
                "attributes" => {
                    if let Some(d) = child_text(n, "divisions") {
                        let d: i128 = d.parse().map_err(|_| "divisions")?;
                        if d <= 0 {
                            return Err("divisions".into());
                        }
                        divisions = Some(d);
                    }
                    if n.children().filter(|c| c.has_tag_name("time")).count() > 1 {
                        return Err("multiple simultaneous time signatures need a staff-bound clock".into());
                    }
                    if let Some(t) = n.children().find(|c| c.has_tag_name("time")) {
                        let b: i128 = child_text(t, "beats").and_then(|v| v.parse().ok()).ok_or("time beats")?;
                        let bt: i128 = child_text(t, "beat-type").and_then(|v| v.parse().ok()).ok_or("time beat-type")?;
                        if b <= 0 || bt <= 0 {
                            return Err("time".into());
                        }
                        if extent > Q::ZERO && beats != Some((b, bt)) {
                            return Err("mid-measure meter change has no bound piecewise clock".into());
                        }
                        beats = Some((b, bt));
                    }
                    let at = base.add(cursor).ok_or("overflow")?;
                    for c in n.children().filter(|c| c.has_tag_name("clef")) {
                        let staff = c.attribute("number").unwrap_or("1").to_string();
                        let kind = match (child_text(c, "sign"), child_text(c, "line")) {
                            (Some("G"), Some("2")) => Some(0u8),
                            (Some("F"), Some("4")) => Some(1u8),
                            _ => None,
                        };
                        match kind {
                            Some(kind) if !c.children().any(|x| x.has_tag_name("clef-octave-change")) => {
                                out.events.push(RegisterEvent { at, pos, staff, kind: EventKind::Clef(kind), initial: !any_note && at == Q::ZERO });
                            }
                            _ => {
                                out.unsupported.insert(staff, "clef outside G2/F4 or with octave change".into());
                            }
                        }
                    }
                }
                "backup" | "forward" => {
                    let d = divisions.ok_or("duration before divisions")?;
                    let raw: i64 = child_text(n, "duration").and_then(|v| v.parse().ok()).ok_or("duration")?;
                    if raw <= 0 {
                        return Err("nonpositive rhythmic duration".into());
                    }
                    let q = Q::new(i128::from(raw), d).ok_or("duration")?;
                    if n.has_tag_name("backup") {
                        cursor = cursor.sub(q).ok_or("overflow")?;
                        raw_cursor = raw_cursor.checked_sub(raw).ok_or("raw clock overflow")?;
                        if cursor < Q::ZERO {
                            return Err("backup before measure start".into());
                        }
                    } else {
                        cursor = cursor.add(q).ok_or("overflow")?;
                        raw_cursor = raw_cursor.checked_add(raw).ok_or("raw clock overflow")?;
                    }
                    extent = extent.max(cursor);
                    last_head = None;
                }
                "direction" => {
                    let staff = child_text(n, "staff").unwrap_or("1").to_string();
                    let d = divisions.ok_or("direction before divisions")?;
                    let offset = match child_text(n, "offset") {
                        Some(v) => Q::new(v.parse::<i128>().map_err(|_| "offset")?, d).ok_or("offset")?,
                        None => Q::ZERO,
                    };
                    let at = base.add(cursor).and_then(|v| v.add(offset)).ok_or("overflow")?;
                    if at < base {
                        out.unsupported.insert(staff.clone(), "direction offset before measure start".into());
                    }
                    for s in n.descendants().filter(|c| c.has_tag_name("octave-shift")) {
                        let distance = match s.attribute("size").unwrap_or("8") {
                            "8" => 1,
                            "15" => 2,
                            _ => {
                                out.unsupported.insert(staff.clone(), "octave-shift size".into());
                                continue;
                            }
                        };
                        let value = match s.attribute("type") {
                            // MusicXML pitch is sounding; "down" displays lower (8va/15ma).
                            Some("down") => distance,
                            Some("up") => -distance,
                            Some("stop") => 0,
                            Some("continue") => continue,
                            _ => {
                                out.unsupported.insert(staff.clone(), "octave-shift type".into());
                                continue;
                            }
                        };
                        out.events.push(RegisterEvent { at, pos, staff: staff.clone(), kind: EventKind::Octave(value), initial: false });
                    }
                }
                "note" => {
                    let d = divisions.ok_or("note before divisions")?;
                    if n.children().any(|c| c.has_tag_name("grace")) {
                        return Err("grace note needs a separate ordering proof".into());
                    }
                    let raw: i64 = child_text(n, "duration").and_then(|v| v.parse().ok()).ok_or("note duration")?;
                    if raw <= 0 {
                        return Err("nonpositive rhythmic duration".into());
                    }
                    let q = Q::new(i128::from(raw), d).ok_or("duration")?;
                    let chord = n.children().any(|c| c.has_tag_name("chord"));
                    let (at_local, raw_local) = if chord {
                        last_head.ok_or("chord member without head")?
                    } else {
                        (cursor, raw_cursor)
                    };
                    if !chord {
                        last_head = Some((cursor, raw_cursor));
                    }
                    let staff = child_text(n, "staff").unwrap_or("1").to_string();
                    out.notes.insert(pos, ObservedNote {
                        span: n.range(),
                        visible: n.attribute("print-object") != Some("no"),
                        measure: ordinal,
                        raw_onset: raw_local,
                        at: base.add(at_local).ok_or("overflow")?,
                        staff,
                        pitched: n.children().any(|c| c.has_tag_name("pitch")),
                        after: base.add(if chord { cursor } else { cursor.add(q).ok_or("overflow")? }).ok_or("overflow")?,
                    });
                    any_note = true;
                    if !chord {
                        cursor = cursor.add(q).ok_or("overflow")?;
                        raw_cursor = raw_cursor.checked_add(raw).ok_or("raw clock overflow")?;
                    }
                    extent = extent.max(cursor).max(at_local.add(q).ok_or("overflow")?);
                }
                _ => {}
            }
        }
        let (b, bt) = beats.ok_or("measure without time signature")?;
        let nominal = Q::new(b.checked_mul(4).ok_or("time overflow")?, bt).ok_or("time")?;
        if extent == Q::ZERO {
            return Err("empty measure has no time".into());
        }
        out.measures.push(Measure { start: base, extent, nominal });
        base = base.add(extent).ok_or("overflow")?;
    }
    out.end = base;
    Ok(out)
}

impl Observed {
    /// Nominal-bar coordinate: integral of time over each measure's own length.
    fn coordinate(&self, t: Q) -> Option<Q> {
        let mut c = Q::ZERO;
        for m in &self.measures {
            let end = m.start.add(m.extent)?;
            if t < end {
                return c.add(t.sub(m.start)?.div(m.nominal)?);
            }
            c = c.add(m.extent.div(m.nominal)?)?;
        }
        if t == self.end { Some(c) } else { None }
    }
    fn measure_of(&self, t: Q) -> usize {
        self.measures.iter().position(|m| m.start <= t && t < m.start.add(m.extent).unwrap_or(m.start))
            .map_or(self.measures.len(), |i| i + 1)
    }
}

fn state_of(clef: u8, octave: i32) -> Option<k::State> {
    let slot = match octave {
        0 => 0,
        1 => 1,
        -1 => 2,
        2 => 3,
        -2 => 4,
        _ => return None,
    };
    k::State::new(clef * 5 + slot).ok()
}

fn clef_of(s: k::State) -> u8 {
    s.index() / 5
}

/// Before/after are distinct even at the same byte: a chord member can start
/// exactly where its head ends, but has the head's earlier musical onset.
#[derive(Debug)]
pub struct WriterClock {
    pub times: BTreeMap<(usize, bool), Q>,
    pub entries: BTreeMap<String, (Q, k::State)>,
    pub end: Q,
}
/// Original writer anchors on the exact source; no material identity is minted.
pub fn observe_clock(source: &str) -> Result<WriterClock, AdapterError> {
    let observed = observe(source).map_err(AdapterError::Binding)?;
    let mut times = BTreeMap::new();
    let mut starts = BTreeMap::new();
    for note in observed.notes.values() {
        for (key, time) in [((note.span.start, false), note.at), ((note.span.end, true), note.after)] {
            if times.insert(key, time).is_some_and(|previous| previous != time) {
                return Err(AdapterError::Binding("writer anchor has conflicting exact times".into()));
            }
        }
        if note.pitched {
            starts.entry(note.staff.clone()).and_modify(|at: &mut Q| *at = (*at).min(note.at)).or_insert(note.at);
        }
    }
    for event in &observed.events {
        if times.insert((event.pos, false), event.at).is_some_and(|previous| previous != event.at) {
            return Err(AdapterError::Binding("register anchor has conflicting times".into()));
        }
    }
    let mut events = observed.events;
    events.sort_by_key(|event| (event.at, event.pos));
    let mut entries = BTreeMap::new();
    for (staff, start) in starts {
        let mut clef = None;
        let mut octave = 0;
        for event in events.iter().filter(|event| event.staff == staff && event.at <= start) {
            match event.kind { EventKind::Clef(value) => clef = Some(value), EventKind::Octave(value) => octave = value }
        }
        if let Some(state) = clef.and_then(|clef| state_of(clef, octave)) {
            entries.insert(staff, (start, state));
        }
    }
    Ok(WriterClock { times, entries, end: observed.end })
}

// ---------------------------------------------------------------- adapter ----

/// Builds and solves one independent domain per staff. Cross-staff dependencies
/// and every unproved capability keep the incumbent (`Unknown`).
pub fn propose(
    source: &str,
    heads: &[AdapterHead],
    groups: &[AdapterGroup],
    capabilities: &Capabilities,
    expansion_limit: Option<usize>,
) -> Result<C3Proposal, AdapterError> {
    if capabilities.source != source {
        return Err(AdapterError::Binding("writer capabilities belong to another source epoch".into()));
    }
    let observed = observe(source).map_err(AdapterError::Binding)?;
    // Field checks: carried locators must bind exactly the observed containers.
    let mut head_at = Vec::with_capacity(heads.len());
    let mut bound_containers = BTreeSet::new();
    for (i, h) in heads.iter().enumerate() {
        let note = observed.notes.get(&h.span.start).ok_or_else(|| AdapterError::Binding(format!("head {i}: no note container at its locator")))?;
        if h.span != note.span
            || !bound_containers.insert(h.span.start)
            || h.visible != note.visible
            || note.measure != h.measure
            || note.raw_onset != h.onset
            || note.staff != h.staff
            || !note.pitched
        {
            return Err(AdapterError::Binding(format!("head {i}: carried fields disagree with the bound container")));
        }
        head_at.push(note.at);
    }
    let pitched = observed.notes.values().filter(|n| n.pitched).count();
    if pitched != heads.len() {
        return Err(AdapterError::Binding("pitched containers and carried heads differ in number".into()));
    }
    for (gi, g) in groups.iter().enumerate() {
        if g.members.iter().any(|&m| m >= heads.len()) || g.members.len() < 2
            || g.members.iter().copied().collect::<BTreeSet<_>>().len() != g.members.len()
        {
            return Err(AdapterError::Binding(format!("group {gi}: invalid members")));
        }
        if g.kind == AdapterGroupKind::Chord && g.members.iter().any(|&m| head_at[m] != head_at[g.members[0]]) {
            return Err(AdapterError::Binding(format!("group {gi}: chord members differ in onset")));
        }
    }

    // A carried tie continuation must belong to a carried tie group; otherwise the
    // tie obligation of that fragment is not bound and nothing may be decided.
    let tied: BTreeSet<usize> = groups.iter().filter(|g| g.kind == AdapterGroupKind::TiedAttack)
        .flat_map(|g| g.members.iter().copied()).collect();
    if let Some(i) = (0..heads.len()).find(|&i| heads[i].continuation && !tied.contains(&i)) {
        return Err(AdapterError::Binding(format!("head {i}: tie continuation without a carried tie group")));
    }
    let staves: BTreeSet<String> = heads.iter().map(|h| h.staff.clone()).collect();
    // Cross-staff coupling of any carried group: no product state exists here.
    let mut coupled = BTreeSet::new();
    for g in groups {
        let s: BTreeSet<&str> = g.members.iter().map(|&m| heads[m].staff.as_str()).collect();
        if s.len() > 1 {
            coupled.extend(s.into_iter().map(str::to_string));
        }
    }
    let mut result = BTreeMap::new();
    for staff in staves {
        let verdict = if coupled.contains(&staff) {
            StaffVerdict::Unknown { reason: "cross-staff group dependency is not closed".into() }
        } else if let Some(r) = observed.unsupported.get(&staff) {
            StaffVerdict::Unknown { reason: format!("unsupported register markup: {r}") }
        } else {
            match staff_domain(&observed, heads, &head_at, groups, &staff, capabilities.staves.get(&staff)) {
                Ok((domain, anchors)) => match k::solve(&domain, expansion_limit) {
                    Ok(outcome) if outcome.complete => StaffVerdict::Solved(Box::new(SolvedStaff { domain, anchors, outcome })),
                    Ok(_) => StaffVerdict::Unknown { reason: "search resource bound reached".into() },
                    Err(e) => StaffVerdict::Unknown { reason: format!("kernel: {e:?}") },
                },
                Err(reason) => StaffVerdict::Unknown { reason },
            }
        };
        result.insert(staff, verdict);
    }
    Ok(C3Proposal { staves: result })
}

fn staff_domain(
    obs: &Observed,
    heads: &[AdapterHead],
    head_at: &[Q],
    groups: &[AdapterGroup],
    staff: &str,
    cap: Option<&StaffCapability>,
) -> Result<(k::Domain, Vec<LayerAnchor>), String> {
    let mine: Vec<usize> = (0..heads.len()).filter(|&i| heads[i].staff == staff).collect();
    let start = mine.iter().map(|&i| head_at[i]).min().ok_or("no head")?;
    let mut events: Vec<&RegisterEvent> = obs.events.iter().filter(|e| e.staff == staff).collect();
    events.sort_by_key(|e| (e.at, e.pos));
    if events.iter().any(|e| e.at < Q::ZERO || e.at > obs.end) {
        return Err("register event outside the bound piece interval".into());
    }
    // Exit::End represents an octave's necessary return to loco only. It
    // cannot erase an actual terminal clef change or a new terminal octave.
    if events.iter().any(|e| e.at == obs.end && e.kind != EventKind::Octave(0)) {
        return Err("terminal register event needs an explicitly bound exit model".into());
    }
    // Same-time order the time rule cannot decide: a non-initial register element
    // effective at t but written after a head of this staff at the same t.
    for e in events.iter().filter(|e| !e.initial) {
        if mine.iter().any(|&i| head_at[i] == e.at && heads[i].span.start < e.pos) {
            return Err("register element follows a same-time head in document order".into());
        }
    }
    let initial_clef = events.iter().find_map(|e| match e.kind {
        EventKind::Clef(c) if e.at == Q::ZERO => Some(c),
        _ => None,
    });
    let mut clef = initial_clef.ok_or("no initial clef for this staff")?;
    let mut octave = 0i32;
    // State after all events with effective time <= t (zero-time intermediates normalized).
    let mut timeline: Vec<(Q, k::State, Vec<usize>)> = Vec::new();
    let mut i = 0;
    while i < events.len() {
        let t = events[i].at;
        let mut positions = Vec::new();
        while i < events.len() && events[i].at == t {
            match events[i].kind {
                EventKind::Clef(c) => clef = c,
                EventKind::Octave(o) => octave = o,
            }
            positions.push(events[i].pos);
            i += 1;
        }
        let s = state_of(clef, octave).ok_or("state")?;
        timeline.push((t, s, positions));
    }
    let state_at = |t: Q| -> Option<k::State> {
        timeline.iter().rev().find(|(at, _, _)| *at <= t).map(|(_, s, _)| *s)
    };
    // Cuts: every head onset of this staff and every incumbent register time inside the domain.
    let mut cuts: BTreeSet<Q> = mine.iter().map(|&i| head_at[i]).collect();
    for (t, _, _) in &timeline {
        if start < *t && *t < obs.end {
            cuts.insert(*t);
        }
    }
    if let Some(capability) = cap {
        cuts.extend(capability.boundaries.keys().copied().filter(|t| start <= *t && *t < obs.end));
    }
    let cuts: Vec<Q> = cuts.into_iter().collect();
    let incumbent: Vec<k::State> = cuts.iter().map(|&t| state_at(t)).collect::<Option<_>>().ok_or("no state before first head")?;

    // Capability: missing evidence or an incumbent state outside it keeps the incumbent.
    let cap = match cap {
        Some(c) if !c.evidence.trim().is_empty() => c,
        _ => return Err("no evidenced materialization capability for this staff".into()),
    };
    if let Some(s) = incumbent.iter().find(|s| !cap.writable.contains(s)) {
        return Err(format!("incumbent state {} is outside the evidenced capability", s.index()));
    }

    // Group intervals on this staff: first onset < boundary <= last onset.
    let interval = |kind: AdapterGroupKind| -> Vec<(Q, Q)> {
        groups.iter().filter(|g| g.kind == kind && heads[g.members[0]].staff == staff)
            .map(|g| {
                let ts = g.members.iter().map(|&m| head_at[m]);
                (ts.clone().min().unwrap_or(Q::ZERO), ts.max().unwrap_or(Q::ZERO))
            }).collect()
    };
    let beams = interval(AdapterGroupKind::PrimaryBeam);
    let ties = interval(AdapterGroupKind::TiedAttack);
    let inside = |b: Q, v: &[(Q, Q)]| v.iter().any(|&(f, l)| f < b && b <= l);

    let mut layers = Vec::with_capacity(cuts.len());
    let mut anchors = Vec::with_capacity(cuts.len());
    for (li, &t) in cuts.iter().enumerate() {
        let next = cuts.get(li + 1).copied().unwrap_or(obs.end);
        let duration = obs.coordinate(next).and_then(|b| obs.coordinate(t).and_then(|a| b.sub(a))).ok_or("time coordinate")?;
        let (n, d) = duration.parts();
        let duration = k::Time::new(u128::try_from(n).map_err(|_| "time")?, u128::try_from(d).map_err(|_| "time")?).map_err(|e| format!("{e:?}"))?;
        let at_heads: Vec<usize> = mine.iter().copied().filter(|&i| head_at[i] == t).collect();
        let kernel_heads = at_heads.iter().map(|&i| k::Head { diatonic: heads[i].diatonic, visible: heads[i].visible }).collect();
        let inc = incumbent[li];
        let mut allowed = cap.writable.clone();
        allowed.sort();
        allowed.dedup();
        if let Some(fixed) = cap.fixed_clefs.get(&t) {
            allowed.retain(|state| clef_of(*state) == *fixed);
        }
        if li == 0 {
            allowed.retain(|state| *state == inc || cap.entry.contains(state));
        }
        let boundary = cap.boundaries.get(&t).copied().unwrap_or_default();
        let in_beam = inside(t, &beams);
        let in_tie = inside(t, &ties);
        let silent = at_heads.is_empty();
        let hidden_only = !silent && at_heads.iter().all(|&i| !heads[i].visible);
        let mut incoming = [[false; 10]; 10];
        if li > 0 {
            let prev = incumbent[li - 1];
            for &a in &allowed {
                for &b in &allowed {
                    let clef_change = clef_of(a) != clef_of(b);
                    let octave_change = a.index() % 5 != b.index() % 5;
                    let existing = a == b || (a == prev && b == inc);
                    let writer_ok = (!clef_change || boundary.clef)
                        && (!octave_change || ((a.index() % 5 == 0 || boundary.octave_stop)
                            && (b.index() % 5 == 0 || boundary.octave_start)));
                    let new_ok = writer_ok && (!silent || cap.boundary_in_silence)
                        && (!hidden_only || cap.boundary_at_hidden_head_only)
                        && !(clef_change && in_beam && !cap.clef_change_inside_primary_beam)
                        && !(clef_change && in_tie && !cap.clef_change_inside_tie)
                        && !(octave_change && in_beam && !cap.octave_change_inside_primary_beam)
                        && !(octave_change && in_tie && !cap.octave_change_inside_tie);
                    incoming[usize::from(a.index())][usize::from(b.index())] = existing || new_ok;
                }
            }
        }
        layers.push(k::Layer { duration, incumbent: inc, heads: kernel_heads, allowed, group_crossing: in_beam || in_tie, incoming });
        anchors.push(LayerAnchor {
            at: t,
            measure: obs.measure_of(t),
            heads: at_heads,
            incumbent_events: timeline.iter().filter(|(at, _, _)| *at == t).flat_map(|(_, _, p)| p.clone()).collect(),
        });
    }
    let mut exit_allowed = [false; 10];
    for s in &cap.writable {
        exit_allowed[usize::from(s.index())] = s.index() % 5 == 0
            || cap.exit_octave_stop
            || incumbent.last() == Some(s);
    }
    Ok((k::Domain { entry: k::Entry::Initial, exit: k::Exit::End, layers, exit_allowed, exit_group_crossing: false }, anchors))
}

#[cfg(test)]
#[path = "joint_reading_c3_adapter_tests.rs"]
mod tests;

/// Read-only canonical state courses. It exports neither edits nor permission.
pub fn read_register_courses(source:&str) -> Result<BTreeMap<String,Vec<(Q,k::State)>>,AdapterError> {
    let observed=observe(source).map_err(AdapterError::Binding)?;
    let mut events=observed.events;events.sort_by_key(|event|(event.at,event.pos));
    let mut state=BTreeMap::<String,(Option<u8>,i32)>::new();
    let mut courses=BTreeMap::<String,Vec<(Q,k::State)>>::new();
    for event in events {
        if observed.unsupported.contains_key(&event.staff){continue;}
        let entry=state.entry(event.staff.clone()).or_insert((None,0));
        match event.kind {EventKind::Clef(clef)=>entry.0=Some(clef),EventKind::Octave(octave)=>entry.1=octave}
        if let Some(current)=entry.0.and_then(|clef|state_of(clef,entry.1)) {
            let course=courses.entry(event.staff).or_default();
            if course.last().is_some_and(|(at,_)|*at==event.at){course.pop();}
            course.push((event.at,current));
        }
    }
    for course in courses.values_mut(){course.dedup_by(|right,left|right.1==left.1);}
    Ok(courses)
}
pub fn course_state_at(course:&[(Q,k::State)],at:Q)->Option<k::State>{
    course.iter().rev().find(|(time,_)|*time<=at).map(|(_,state)|*state)
}

/// Negative final-artifact readback only. These XML observations create no
/// source identity, group authority, plan, state alternative or writer permit.
pub fn compare_actual_output(source:&str,baseline:&str,staff:&str)->Result<(k::Cost,k::Cost,k::LedgerMetrics),AdapterError>{
    let fail=|text:&str|AdapterError::Binding(text.to_owned());
    let observed=observe(source).map_err(AdapterError::Binding)?;
    let actual_courses=read_register_courses(source)?;
    let baseline_courses=read_register_courses(baseline)?;
    let actual_course=actual_courses.get(staff).ok_or_else(||fail("Missing final actual course"))?;
    let baseline_course=baseline_courses.get(staff).ok_or_else(||fail("Missing bound incumbent course"))?;
    let document=roxmltree::Document::parse(source).map_err(|error|fail(&error.to_string()))?;
    let mut heads=Vec::<(Q,k::Head)>::new();
    let mut groups=Vec::<(Q,Q)>::new();
    // Negative readback of explicit serialized group grammar only. A voice
    // label may change within an original physical group; it is not a group ID.
    // Competing open beams or same-pitch ties are unsupported, never guessed.
    let mut beam: Option<(Q,Q)> = None;
    let mut ties=BTreeMap::<i32,(Q,Q)>::new();
    for node in document.descendants().filter(|node|node.has_tag_name("note")){
        let note=observed.notes.get(&node.range().start).ok_or_else(||fail("Unbound final note observation"))?;
        if note.staff!=staff {continue;}
        let chord=node.children().any(|child|child.has_tag_name("chord"));
        if !chord && let Some(tag)=node.children().find(|child|child.has_tag_name("beam")&&child.attribute("number").unwrap_or("1")=="1") {
            match tag.text().map(str::trim) {
                Some("begin")=>{
                    if beam.replace((note.at,note.at)).is_some(){return Err(fail("Competing actual primary beams have no unique serialized binding"));}
                },
                Some("continue")=>{
                    let span=beam.as_mut().ok_or_else(||fail("Unbound actual primary beam continuation"))?;
                    if note.at<=span.1{return Err(fail("Actual primary beam does not advance in time"));}
                    span.1=note.at;
                },
                Some("end")=>{
                    let (first,last)=beam.take().ok_or_else(||fail("Unbound actual primary beam end"))?;
                    if note.at<=last{return Err(fail("Actual primary beam does not advance in time"));}
                    groups.push((first,note.at));
                },
                Some("forward hook"|"backward hook")=>{},
                _=>return Err(fail("Unsupported actual primary beam state")),
            }
        }
        let Some(pitch)=node.children().find(|child|child.has_tag_name("pitch")) else {continue;};
        let step=child_text(pitch,"step").ok_or_else(||fail("Missing final spelling"))?;
        let offset=match step {"C"=>0,"D"=>1,"E"=>2,"F"=>3,"G"=>4,"A"=>5,"B"=>6,_=>return Err(fail("Unsupported actual spelling"))};
        let octave=child_text(pitch,"octave").and_then(|text|text.parse::<i32>().ok()).ok_or_else(||fail("Invalid final octave"))?;
        let diatonic=octave.checked_mul(7).and_then(|value|value.checked_add(offset)).ok_or_else(||fail("Final spelling overflow"))?;
        heads.push((note.at,k::Head {diatonic,visible:note.visible}));
        let alter=child_text(pitch,"alter").unwrap_or("0").parse::<i32>().map_err(|_|fail("Unsupported sounding alter"))?;
        let semitone=match step {"C"=>0,"D"=>2,"E"=>4,"F"=>5,"G"=>7,"A"=>9,_=>11};
        let midi=octave.checked_add(1).and_then(|value|value.checked_mul(12)).and_then(|value|value.checked_add(semitone)).and_then(|value|value.checked_add(alter)).ok_or_else(||fail("Sounding pitch overflow"))?;
        let start=node.children().any(|child|child.has_tag_name("tie")&&child.attribute("type")==Some("start"));
        let stop=node.children().any(|child|child.has_tag_name("tie")&&child.attribute("type")==Some("stop"));
        let key=midi;
        if stop {
            let span=ties.get_mut(&key).ok_or_else(||fail("Actual tie has no unambiguous beginning"))?;
            if note.at<=span.1{return Err(fail("Actual tied chain does not advance in time"));}
            span.1=note.at;
            if !start {groups.push(ties.remove(&key).expect("just observed actual tie"));}
        } else if start && ties.insert(key,(note.at,note.at)).is_some(){return Err(fail("Ambiguous actual tie observation"));}
    }
    if beam.is_some()||!ties.is_empty(){return Err(fail("Incomplete final group observations"));}
    let start=heads.iter().map(|(at,_)|*at).min().ok_or_else(||fail("No actual heads"))?;
    let mut cuts=heads.iter().map(|(at,_)|*at).collect::<BTreeSet<_>>();
    cuts.extend(actual_course.iter().chain(baseline_course).map(|(at,_)|*at).filter(|at|start<=*at&&*at<observed.end));
    let cuts=cuts.into_iter().collect::<Vec<_>>();let mut layers=Vec::new();let mut path=Vec::new();
    for (index,&at) in cuts.iter().enumerate(){
        let next=cuts.get(index+1).copied().unwrap_or(observed.end);
        let elapsed=observed.coordinate(next).and_then(|last|observed.coordinate(at).and_then(|first|last.sub(first))).ok_or_else(||fail("Unbound actual nominal time"))?;
        let (n,d)=elapsed.parts();
        let duration=k::Time::new(u128::try_from(n).map_err(|_|fail("Negative final interval"))?,u128::try_from(d).map_err(|_|fail("Invalid final denominator"))?).map_err(|error|fail(&format!("{error:?}")))?;
        let incumbent=course_state_at(baseline_course,at).ok_or_else(||fail("Unbound incumbent final entry"))?;
        let actual=course_state_at(actual_course,at).ok_or_else(||fail("Unbound actual final entry"))?;
        path.push(actual);
        layers.push(k::Layer{duration,incumbent,heads:heads.iter().filter(|(time,_)|*time==at).map(|(_,head)|*head).collect(),
            allowed:(0..10).map(|index|k::State::new(index).expect("canonical state")).collect(),
            group_crossing:groups.iter().any(|&(first,last)|first<at&&at<=last),incoming:[[true;10];10]});
    }
    let domain=k::Domain{entry:k::Entry::Initial,exit:k::Exit::End,layers,exit_allowed:[true;10],exit_group_crossing:false};
    let original=domain.layers.iter().map(|layer|layer.incumbent).collect::<Vec<_>>();
    let baseline=k::evaluate_course(&domain,&original).map_err(|error|fail(&format!("{error:?}")))?;
    let actual=k::evaluate_course(&domain,&path).map_err(|error|fail(&format!("Actual final head/course protection: {error:?}")))?;
    let metrics=k::ledger_metrics(&domain,&path).map_err(|error|fail(&format!("{error:?}")))?;
    Ok((baseline,actual,metrics))
}
