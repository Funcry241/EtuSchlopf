//! C3's pure finite-course search. This file has no parser, writer or adoption authority.
//! An eventual owner adapter must prove the supplied graph and incumbent complete.
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Time {
    n: u128,
    d: u128,
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl Time {
    pub const ZERO: Self = Self { n: 0, d: 1 };
    pub const ONE: Self = Self { n: 1, d: 1 };
    pub fn new(n: u128, d: u128) -> Result<Self, Error> {
        if d == 0 {
            return Err(Error::Invalid("zero time denominator"));
        }
        let g = gcd(n, d);
        Ok(Self { n: n / g, d: d / g })
    }
    fn add(self, other: Self) -> Result<Self, Error> {
        let g = gcd(self.d, other.d);
        let a = self.d / g;
        let b = other.d / g;
        Self::new(
            self.n
                .checked_mul(b)
                .and_then(|x| other.n.checked_mul(a).and_then(|y| x.checked_add(y)))
                .ok_or(Error::Arithmetic)?,
            a.checked_mul(other.d).ok_or(Error::Arithmetic)?,
        )
    }
    fn aged(self, duration: Self) -> Result<Self, Error> {
        if self >= Self::ONE || duration >= Self::ONE {
            return Ok(Self::ONE);
        }
        if duration
            >= (Self {
                n: self.d - self.n,
                d: self.d,
            })
        {
            return Ok(Self::ONE);
        }
        Ok(self.add(duration)?.min(Self::ONE))
    }
}

impl Ord for Time {
    fn cmp(&self, rhs: &Self) -> Ordering {
        // Continued fractions compare exactly without overflowing a cross-product.
        let (mut a, mut b, mut c, mut d, mut reverse) = (self.n, self.d, rhs.n, rhs.d, false);
        loop {
            let comparison = (a / b).cmp(&(c / d));
            if comparison != Ordering::Equal {
                return if reverse {
                    comparison.reverse()
                } else {
                    comparison
                };
            }
            let (ra, rc) = (a % b, c % d);
            let terminal = match (ra, rc) {
                (0, 0) => Some(Ordering::Equal),
                (0, _) => Some(Ordering::Less),
                (_, 0) => Some(Ordering::Greater),
                _ => None,
            };
            if let Some(value) = terminal {
                return if reverse { value.reverse() } else { value };
            }
            (a, b, c, d, reverse) = (b, ra, d, rc, !reverse);
        }
    }
}
impl PartialOrd for Time {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

/// Canonical policy order: G/loco, G/8va, G/8vb, G/15ma, G/15mb, then F likewise.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct State(u8);
impl State {
    pub fn new(index: u8) -> Result<Self, Error> {
        if index < 10 {
            Ok(Self(index))
        } else {
            Err(Error::Invalid("register state"))
        }
    }
    pub fn index(self) -> u8 {
        self.0
    }
    fn bass(self) -> bool {
        self.0 >= 5
    }
    fn octave(self) -> i32 {
        [0, 1, -1, 2, -2][usize::from(self.0 % 5)]
    }
    fn unconventional(self) -> bool {
        if self.bass() {
            self.octave() > 0
        } else {
            self.octave() < 0
        }
    }
    fn ledgers(self, diatonic: i32) -> Result<u64, Error> {
        let written = i64::from(diatonic) - 7 * i64::from(self.octave());
        let (bottom, top) = if self.bass() { (18, 26) } else { (30, 38) };
        u64::try_from((bottom - written).max(written - top).max(0) / 2)
            .map_err(|_| Error::Arithmetic)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Head {
    /// Actual spelling, C0=0, D0=1 ... B0=6. Not inferred from MIDI pitch class.
    pub diatonic: i32,
    pub visible: bool,
}

#[derive(Clone, Debug)]
pub struct Layer {
    /// Exact elapsed nominal-bar coordinate, supplied by the temporal owner.
    pub duration: Time,
    pub incumbent: State,
    pub heads: Vec<Head>,
    pub allowed: Vec<State>,
    /// Existing owner-bound group crossing at the incoming boundary.
    pub group_crossing: bool,
    /// Fixed shared state index matrix; no arbitrary new transition is inferred.
    pub incoming: [[bool; 10]; 10],
}

#[derive(Clone, Copy, Debug)]
pub enum Entry {
    Initial,
    /// Bound length of the fixed preceding section, up to the domain boundary.
    /// At least one bar is sufficient; unknown length cannot be encoded as zero.
    Carry {
        state: State,
        elapsed: Time,
    },
}
#[derive(Clone, Copy, Debug)]
pub enum Exit {
    End,
    /// Bound length of the fixed following section, from the domain boundary.
    /// It ends at the next real register boundary (or the complete course end).
    Carry {
        state: State,
        remaining: Time,
    },
}

#[derive(Clone, Debug)]
pub struct Domain {
    pub entry: Entry,
    pub exit: Exit,
    pub layers: Vec<Layer>,
    pub exit_allowed: [bool; 10],
    pub exit_group_crossing: bool,
}

/// h8,h7,h6,h5,h4,ko,uk,ks,gb,h3,ch,h2,h1, exactly in policy order.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cost(pub [u64; 13]);
impl Cost {
    fn count(&mut self, field: usize, amount: u64) -> Result<(), Error> {
        self.0[field] = self.0[field].checked_add(amount).ok_or(Error::Arithmetic)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Invalid(&'static str),
    Arithmetic,
}

#[derive(Clone, Debug)]
struct Label {
    cost: Cost,
    changed_layers: u64,
    path: Vec<State>,
}
impl Label {
    fn precedes(&self, other: &Self) -> bool {
        (self.cost, self.changed_layers, &self.path)
            < (other.cost, other.changed_layers, &other.path)
    }
}

fn transition(cost: &mut Cost, from: State, to: State, group: bool) -> Result<(), Error> {
    let clef = from.bass() != to.bass();
    let octave = from.octave() != to.octave();
    if clef && octave {
        cost.count(5, 1)?;
    }
    if (clef || octave) && group {
        cost.count(8, 1)?;
    }
    cost.count(
        10,
        u64::from(clef)
            + u64::from(octave && from.octave() != 0)
            + u64::from(octave && to.octave() != 0),
    )
}

fn heads(cost: &mut Cost, layer: &Layer, state: State) -> Result<bool, Error> {
    for head in layer.heads.iter().filter(|h| h.visible) {
        let l = state.ledgers(head.diatonic)?;
        let original = layer.incumbent.ledgers(head.diatonic)?;
        if l >= 4 && l > original {
            return Ok(false);
        }
        let index = match l {
            0 => None,
            1 => Some(12),
            2 => Some(11),
            3 => Some(9),
            4 => Some(4),
            5 => Some(3),
            6 => Some(2),
            7 => Some(1),
            _ => Some(0),
        };
        if let Some(field) = index {
            cost.count(field, 1)?;
        }
        if state.unconventional() {
            cost.count(6, 1)?;
        }
    }
    Ok(true)
}

fn first(domain: &Domain, state: State) -> Result<Option<Label>, Error> {
    let layer = &domain.layers[0];
    let mut label = Label {
        cost: Cost::default(),
        changed_layers: 0,
        path: vec![state],
    };
    match domain.entry {
        Entry::Initial => label.cost.count(10, u64::from(state.octave() != 0))?,
        Entry::Carry {
            state: from,
            elapsed,
        } => {
            if !layer.incoming[usize::from(from.0)][usize::from(state.0)] {
                return Ok(None);
            }
            transition(&mut label.cost, from, state, layer.group_crossing)?;
            // Switching at the cut ends this fixed-context section. Its short
            // remainder must not disappear merely because it lies outside.
            if from != state && elapsed < Time::ONE {
                label.cost.count(7, 1)?;
            }
        }
    }
    if !heads(&mut label.cost, layer, state)? {
        return Ok(None);
    }
    label.changed_layers = u64::from(!layer.heads.is_empty() && state != layer.incumbent);
    Ok(Some(label))
}

fn initial_age(domain: &Domain, state: State) -> Result<Time, Error> {
    let duration = domain.layers[0].duration;
    match domain.entry {
        Entry::Carry {
            state: from,
            elapsed,
        } if from == state => elapsed.aged(duration),
        _ => Ok(duration.min(Time::ONE)),
    }
}

fn extend(
    previous: &Label,
    age: Time,
    layer: &Layer,
    to: State,
) -> Result<Option<(Time, Label)>, Error> {
    let from = *previous.path.last().ok_or(Error::Invalid("empty prefix"))?;
    if !layer.incoming[usize::from(from.0)][usize::from(to.0)] {
        return Ok(None);
    }
    let mut label = previous.clone();
    let next_age = if from == to {
        age.aged(layer.duration)?
    } else {
        if age < Time::ONE {
            label.cost.count(7, 1)?;
        }
        transition(&mut label.cost, from, to, layer.group_crossing)?;
        layer.duration.min(Time::ONE)
    };
    if !heads(&mut label.cost, layer, to)? {
        return Ok(None);
    }
    if !layer.heads.is_empty() && to != layer.incumbent {
        label.changed_layers = label
            .changed_layers
            .checked_add(1)
            .ok_or(Error::Arithmetic)?;
    }
    label.path.push(to);
    Ok(Some((next_age, label)))
}

fn finish(domain: &Domain, age: Time, mut label: Label) -> Result<Option<Label>, Error> {
    let last = *label.path.last().ok_or(Error::Invalid("empty course"))?;
    if !domain.exit_allowed[usize::from(last.0)] {
        return Ok(None);
    }
    let completed_age = match domain.exit {
        Exit::Carry {
            state: to,
            remaining,
        } if last == to => age.aged(remaining)?,
        _ => age,
    };
    if completed_age < Time::ONE {
        label.cost.count(7, 1)?;
    }
    match domain.exit {
        Exit::End => label.cost.count(10, u64::from(last.octave() != 0))?,
        Exit::Carry {
            state: to,
            remaining,
        } => {
            transition(&mut label.cost, last, to, domain.exit_group_crossing)?;
            if last != to && remaining < Time::ONE {
                label.cost.count(7, 1)?;
            }
        }
    }
    Ok(Some(label))
}

fn validate(domain: &Domain) -> Result<(), Error> {
    if domain.layers.is_empty() {
        return Err(Error::Invalid("empty domain"));
    }
    if matches!(
        domain.entry,
        Entry::Carry {
            elapsed: Time::ZERO,
            ..
        }
    ) || matches!(
        domain.exit,
        Exit::Carry {
            remaining: Time::ZERO,
            ..
        }
    ) {
        return Err(Error::Invalid(
            "carry context must have a known positive length",
        ));
    }
    for layer in &domain.layers {
        if layer.duration == Time::ZERO {
            return Err(Error::Invalid("unnormalized zero-time layer"));
        }
        if !layer.allowed.contains(&layer.incumbent) {
            return Err(Error::Invalid("incumbent missing"));
        }
        let mut states = layer.allowed.clone();
        states.sort();
        states.dedup();
        if states.len() != layer.allowed.len() {
            return Err(Error::Invalid("duplicate state"));
        }
    }
    Ok(())
}

fn evaluate(domain: &Domain, path: &[State]) -> Result<Label, Error> {
    let mut label = first(domain, path[0])?.ok_or(Error::Invalid("illegal incumbent entry"))?;
    let mut age = initial_age(domain, path[0])?;
    for (layer, state) in domain.layers.iter().skip(1).zip(path.iter().skip(1)) {
        (age, label) =
            extend(&label, age, layer, *state)?.ok_or(Error::Invalid("illegal incumbent edge"))?;
    }
    finish(domain, age, label)?.ok_or(Error::Invalid("illegal incumbent exit"))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outcome {
    pub complete: bool,
    pub incumbent_cost: Cost,
    pub selected_cost: Cost,
    pub selected: Vec<State>,
    pub improved: bool,
    pub expansions: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerMetrics {
    pub visible_heads: u64,
    pub maximum: u64,
    pub sum: u64,
    /// Exact, uncapped histogram; the cost's h8 bin is a separate representation.
    pub histogram: BTreeMap<u64, u64>,
}

/// Evaluate a complete already-given path; never selects an alternative.
pub fn evaluate_course(domain:&Domain,path:&[State])->Result<Cost,Error>{
    validate(domain)?;
    if path.len()!=domain.layers.len(){return Err(Error::Invalid("path length"));}
    Ok(evaluate(domain,path)?.cost)
}

/// Symbolic metrics of this exact graph/path only, not persisted-artifact evidence.
pub fn ledger_metrics(domain: &Domain, path: &[State]) -> Result<LedgerMetrics, Error> {
    if path.len() != domain.layers.len() {
        return Err(Error::Invalid("path length"));
    }
    let mut metrics = LedgerMetrics {
        visible_heads: 0,
        maximum: 0,
        sum: 0,
        histogram: BTreeMap::new(),
    };
    for (layer, state) in domain.layers.iter().zip(path) {
        if !layer.allowed.contains(state) {
            return Err(Error::Invalid("path state unavailable"));
        }
        for head in layer.heads.iter().filter(|head| head.visible) {
            let count = state.ledgers(head.diatonic)?;
            metrics.visible_heads = metrics
                .visible_heads
                .checked_add(1)
                .ok_or(Error::Arithmetic)?;
            metrics.maximum = metrics.maximum.max(count);
            metrics.sum = metrics.sum.checked_add(count).ok_or(Error::Arithmetic)?;
            let bin = metrics.histogram.entry(count).or_default();
            *bin = bin.checked_add(1).ok_or(Error::Arithmetic)?;
        }
    }
    Ok(metrics)
}

/// The optional caller-owned resource bound never turns a partial search into a decision.
pub fn solve(domain: &Domain, expansion_limit: Option<usize>) -> Result<Outcome, Error> {
    validate(domain)?;
    let incumbent: Vec<_> = domain.layers.iter().map(|layer| layer.incumbent).collect();
    let baseline = evaluate(domain, &incumbent)?;
    let retained = |complete, expansions| Outcome {
        complete,
        incumbent_cost: baseline.cost,
        selected_cost: baseline.cost,
        selected: incumbent.clone(),
        improved: false,
        expansions,
    };
    let mut frontier = BTreeMap::new();
    let mut expansions = 0usize;
    for state in &domain.layers[0].allowed {
        if let Some(label) = first(domain, *state)? {
            frontier.insert((*state, initial_age(domain, *state)?), label);
        }
    }
    for layer in domain.layers.iter().skip(1) {
        let mut next: BTreeMap<(State, Time), Label> = BTreeMap::new();
        for ((_, age), label) in frontier {
            for state in &layer.allowed {
                if expansion_limit.is_some_and(|limit| expansions >= limit) {
                    return Ok(retained(false, expansions));
                }
                expansions = expansions.checked_add(1).ok_or(Error::Arithmetic)?;
                if let Some((age, label)) = extend(&label, age, layer, *state)? {
                    let key = (*state, age);
                    if next.get(&key).is_none_or(|old| label.precedes(old)) {
                        next.insert(key, label);
                    }
                }
            }
        }
        frontier = next;
    }
    let mut winner = baseline.clone();
    for ((_, age), label) in frontier {
        if let Some(candidate) = finish(domain, age, label)?
            && candidate.precedes(&winner)
        {
            winner = candidate;
        }
    }
    if winner.cost >= baseline.cost {
        return Ok(retained(true, expansions));
    }
    Ok(Outcome {
        complete: true,
        incumbent_cost: baseline.cost,
        selected_cost: winner.cost,
        selected: winner.path,
        improved: true,
        expansions,
    })
}

#[cfg(test)]
#[path = "joint_reading_c3_course_tests.rs"]
mod tests;
