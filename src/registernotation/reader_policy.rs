//! Source-carried continuation origins and the accepted cue-only reader decision.
//! This module plans immutable removals. Only the existing E012 owner edits XML.
use super::{RegisternotationsDarstellung, RegisternotationsSichtmaterialisierungsauftrag};
use crate::structural_write_authority::HorizontalLayoutMaterializationDirective;
use anyhow::{Result, ensure};
use roxmltree::Node;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
    sync::{Mutex, OnceLock},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Birth {
    pub(crate) part: String,
    pub(crate) staff: String,
    pub(crate) slot: usize,
    pub(crate) onset: i64,
    pub(crate) kind: String,
    pub(crate) display: RegisternotationsDarstellung,
    pub(crate) line: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CueRequest {
    pub(crate) id: String,
    pub(crate) ordinal: usize,
    pub(crate) slot: usize,
    pub(crate) staff: u8,
    pub(crate) display: RegisternotationsDarstellung,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VisibleLine {
    pub(crate) page: usize,
    /// Exact observed coordinates, not calibrated pixel thresholds.
    pub(crate) stroke: [u64; 4],
    pub(crate) orientation: [u64; 4],
    pub(crate) cell_x: [u64; 2],
}
impl VisibleLine {
    /// The carried horizontal attachment survives vertical engraving reflow.
    /// This comparison does not prove visibility: the current PDF observer
    /// must independently prove the glyph, line and current staff reading side.
    pub(crate) fn same_horizontal_course(&self, other: &Self) -> bool {
        self.page == other.page
            && self.cell_x == other.cell_x
            && [self.stroke[0], self.stroke[2], self.stroke[3]]
                == [other.stroke[0], other.stroke[2], other.stroke[3]]
            && [self.orientation[0], self.orientation[2]]
                == [other.orientation[0], other.orientation[2]]
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct PdfContext {
    pub(crate) xml_sha256: String,
    pub(crate) pdf_sha256: String,
    pub(crate) supported: BTreeMap<String, VisibleLine>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Origin {
    birth: Birth,
    validated: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalPlan {
    source: String,
    directives: String,
    removals: Vec<Range<usize>>,
    omissions: BTreeMap<usize, (CueRequest, VisibleLine)>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalClaim {
    source: String,
    omissions: BTreeMap<usize, (CueRequest, VisibleLine)>,
    adopted: bool,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Session {
    origins: BTreeMap<String, Origin>,
    probes: BTreeMap<(String, String), BTreeMap<String, VisibleLine>>,
    plans: BTreeMap<String, FinalPlan>,
    claims: BTreeMap<String, FinalClaim>,
}
static SESSION: OnceLock<Mutex<Session>> = OnceLock::new();
fn session() -> &'static Mutex<Session> {
    SESSION.get_or_init(|| Mutex::new(Session::default()))
}
fn digest(xml: &str) -> String {
    crate::sha256::hex(xml.as_bytes())
}
fn layout_key(directives: &[HorizontalLayoutMaterializationDirective]) -> String {
    digest(&format!("{directives:?}"))
}
pub(crate) fn reset() {
    crate::master_state_identity::clear_final_layout_observations();
    *session().lock().unwrap_or_else(|e| e.into_inner()) = Session::default();
}

/// Called at the existing creator, before its bound octave-measure writes.
pub(crate) fn issue_origins(
    plan: &RegisternotationsSichtmaterialisierungsauftrag,
    births: &[Birth],
) -> Vec<String> {
    let mut state = session().lock().unwrap_or_else(|e| e.into_inner());
    // Keep the original creator's eager issuance semantics. Preparation itself
    // is immutable, so a later replacement can be validated before committing.
    let prepared = prepare_origin_update(&state, plan, births, None)
        .expect("initial origin issuance has no replacement preconditions");
    commit_origin_update(&mut state, prepared)
        .expect("locked initial origin session cannot change during issuance")
}

/// Registry-only preparation: this neither edits MusicXML nor adopts a course.
/// The original and proposed complete reader sessions remain distinct values.
pub(crate) struct PreparedOriginUpdate {
    before: Session,
    proposed: Session,
    ids: Vec<String>,
}

fn prepare_origin_update(
    state: &Session,
    plan: &RegisternotationsSichtmaterialisierungsauftrag,
    births: &[Birth],
    replacement_source: Option<&str>,
) -> Result<PreparedOriginUpdate> {
    let mut proposed = state.clone();
    if let Some(source) = replacement_source {
        // Once probes or final plans exist, their geometry/claims would need
        // their own proven continuation. Never silently discard those readers.
        ensure!(state.probes.is_empty() && state.plans.is_empty() && state.claims.is_empty(),
            "Ottava replacement has downstream reader observations");
        ensure!(state.origins.values().all(|origin| origin.validated),
            "Ottava replacement has unconfirmed original births");
        let document = roxmltree::Document::parse(source)?;
        validate_materialized_origins(&document, &state.origins)?;
        proposed.origins.clear();
    }
    let ids = births
        .iter()
        .map(|birth| {
            let id = format!(
                "ets-ottava-{}",
                digest(&format!(
                    "{:?}|{:?}|{birth:?}",
                    plan.semantischer_plan_fingerabdruck, plan.sicht_fingerabdruck
                ))
            );
            proposed.origins.insert(
                id.clone(),
                Origin {
                    birth: birth.clone(),
                    validated: false,
                },
            );
            id
        })
        .collect();
    Ok(PreparedOriginUpdate { before: state.clone(), proposed, ids })
}

pub(crate) fn prepare_joint_origins(plan: &RegisternotationsSichtmaterialisierungsauftrag,
    births: &[Birth], master: &str, render: &str) -> Result<PreparedOriginUpdate> {
    let state = session().lock().unwrap_or_else(|error| error.into_inner());
    validate_materialized_origins(&roxmltree::Document::parse(render)?,&state.origins)?;
    prepare_origin_update(&state,plan,births,Some(master))
}
impl PreparedOriginUpdate {
    pub(crate) fn ids(&self) -> &[String] { &self.ids }
    pub(crate) fn validate_documents(mut self, master: &str, render: &str) -> Result<Self> {
        self = validate_origin_update(self,master)?;
        validate_materialized_origins(&roxmltree::Document::parse(render)?,&self.proposed.origins)?;
        Ok(self)
    }
    pub(crate) fn commit_with<T>(self, action: impl FnOnce() -> Result<T>) -> Result<T> {
        let mut state = session().lock().unwrap_or_else(|error| error.into_inner());
        ensure!(*state == self.before,"C3 origin session changed before productive materialization");
        let result = action()?;
        *state = self.proposed;
        Ok(result)
    }
}

fn validate_origin_update(
    mut prepared: PreparedOriginUpdate,
    xml: &str,
) -> Result<PreparedOriginUpdate> {
    let document = roxmltree::Document::parse(xml)?;
    validate_materialized_origins(&document, &prepared.proposed.origins)?;
    for origin in prepared.proposed.origins.values_mut() {
        origin.validated = true;
    }
    Ok(prepared)
}

fn commit_origin_update(
    state: &mut Session,
    prepared: PreparedOriginUpdate,
) -> Result<Vec<String>> {
    ensure!(*state == prepared.before,
        "Ottava reader session changed while materialization was prepared");
    *state = prepared.proposed;
    Ok(prepared.ids)
}
fn child<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    node.children()
        .find(|n| n.has_tag_name(name))
        .and_then(|n| n.text())
}
/// A creator-issued marker travels as a direct nonprinting XML comment.
/// Multiple markers are ambiguous; a readback never mints a replacement.
pub(crate) fn carried_origin<'a>(node: Node<'a, '_>) -> Option<&'a str> {
    let mut markers = node
        .children()
        .filter(|child| child.is_comment())
        .filter_map(|child| child.text())
        .filter(|text| text.starts_with("ets-ottava-"));
    let first = markers.next()?;
    if markers.next().is_some() {
        None
    } else {
        Some(first)
    }
}
fn origin_matches(node: Node<'_, '_>, part: &str, origin: &Origin) -> bool {
    let shifts = node
        .descendants()
        .filter(|n| n.has_tag_name("octave-shift"))
        .collect::<Vec<_>>();
    let [shift] = shifts.as_slice() else {
        return false;
    };
    let place = match origin.birth.display {
        RegisternotationsDarstellung::Achtva | RegisternotationsDarstellung::Fuenfzehnma => "above",
        _ => "below",
    };
    part == origin.birth.part
        && child(node, "staff") == Some(origin.birth.staff.as_str())
        && node.attribute("placement") == Some(place)
        && shift.attribute("type") == Some(origin.birth.kind.as_str())
        && shift.attribute("size").and_then(|v| v.parse::<i32>().ok())
            == Some(origin.birth.display.groesse())
        && shift
            .attribute("number")
            .and_then(|v| v.parse::<usize>().ok())
            == Some(origin.birth.line)
}
pub(crate) fn confirm_materialization(xml: &str) -> Result<()> {
    let mut state = session().lock().unwrap_or_else(|e| e.into_inner());
    let prepared = PreparedOriginUpdate {
        before: state.clone(), proposed: state.clone(), ids: Vec::new(),
    };
    let validated = validate_origin_update(prepared, xml)?;
    commit_origin_update(&mut state, validated)?;
    Ok(())
}

fn validate_materialized_origins(
    document: &roxmltree::Document<'_>,
    origins: &BTreeMap<String, Origin>,
) -> Result<()> {
    let mut found = BTreeSet::new();
    for part in document
        .root_element()
        .children()
        .filter(|n| n.has_tag_name("part"))
    {
        for node in part.descendants().filter(|n| n.has_tag_name("direction")) {
            if let Some(id) = carried_origin(node) {
                let origin = origins.get(id).ok_or_else(|| anyhow::anyhow!(
                    "Ottava creator marker has no registered origin"))?;
                ensure!(
                    found.insert(id.to_owned())
                        && origin_matches(node, part.attribute("id").unwrap_or(""), origin),
                    "Ottava creator origin is duplicated or inconsistent"
                );
            }
        }
    }
    let marker_count = document.descendants().filter(|node| node.is_comment()
        && node.text().is_some_and(|text| text.starts_with("ets-ottava-"))).count();
    ensure!(marker_count == found.len(),
        "Ottava creator marker coverage is incomplete or ambiguous");
    ensure!(
        found.len() == origins.len(),
        "Ottava creator origin coverage is incomplete"
    );
    Ok(())
}

#[derive(Debug)]
struct ConfirmedOriginalOctaveRemoval {
    source: std::sync::Arc<str>,
    range: Range<usize>,
}

impl crate::arrangement::representation_transport::OriginalOctaveRemovalEvidence
    for ConfirmedOriginalOctaveRemoval
{
    fn verifies(&self, source: &str, range: &Range<usize>) -> Result<()> {
        ensure!(self.source.as_ref() == source && self.range == *range,
            "Original octave removal evidence belongs to another document or operation");
        Ok(())
    }
}

/// Attach immutable owner evidence only after the journal has been enclosed
/// into the actual full-document transition. No session or XML is changed.
pub(crate) fn bind_original_octave_removal_evidence(
    step: crate::arrangement::representation_transport::RepresentationStep,
) -> Result<crate::arrangement::representation_transport::RepresentationStep> {
    let state = session().lock().unwrap_or_else(|error| error.into_inner());
    bind_original_removals_against(&state, step)
}

fn bind_original_removals_against(
    state: &Session,
    mut step: crate::arrangement::representation_transport::RepresentationStep,
) -> Result<crate::arrangement::representation_transport::RepresentationStep> {
    let ranges = step.original_removal_ranges();
    if ranges.is_empty() { return Ok(step); }
    let source = std::sync::Arc::<str>::from(step.original_documents().0);
    for range in ranges {
        validate_original_direction_removal(state, &source, range.clone())?;
        step.record_original_octave_removal(range.clone(), std::sync::Arc::new(
            ConfirmedOriginalOctaveRemoval { source: source.clone(), range }))?;
    }
    Ok(step)
}

fn validate_original_direction_removal(
    state: &Session,
    source: &str,
    range: Range<usize>,
) -> Result<()> {
    ensure!(state.probes.is_empty() && state.plans.is_empty() && state.claims.is_empty(),
        "Ottava removal has downstream reader observations");
    let document = roxmltree::Document::parse(source)?;
    let node = document.descendants().find(|node|
        node.has_tag_name("direction") && node.range() == range)
        .ok_or_else(|| anyhow::anyhow!("Ottava removal is not a complete original direction"))?;
    let id = carried_origin(node)
        .ok_or_else(|| anyhow::anyhow!("Ottava removal has no unique creator marker"))?;
    let origin = state.origins.get(id)
        .ok_or_else(|| anyhow::anyhow!("Ottava removal has no registered creator origin"))?;
    ensure!(origin.validated, "Ottava removal has no confirmed original birth");
    let part = node.ancestors().find(|node| node.has_tag_name("part"))
        .and_then(|part| part.attribute("id"))
        .ok_or_else(|| anyhow::anyhow!("Ottava removal has no bound original part"))?;
    ensure!(origin_matches(node, part, origin), "Ottava removal differs from its original birth");
    ensure!(document.descendants().filter(|node|
        node.is_comment() && node.text() == Some(id)).count() == 1,
        "Ottava removal has a duplicated original marker");
    ensure!(node.children().filter(|child| child.has_tag_name("staff")).count() == 1
        && node.children().filter(|child| child.has_tag_name("offset")).count() <= 1,
        "Ottava removal has ambiguous staff or offset content");
    ensure!(node.descendants().filter(|child| child.is_comment())
        .all(|comment| comment.parent() == Some(node) && comment.text() == Some(id)),
        "Ottava removal contains a foreign nested comment");
    ensure!(!node.descendants().any(|child| child.is_pi())
        && node.descendants().filter(|child| child.has_tag_name("words")).count() <= 1,
        "Ottava removal contains unsupported instructions or duplicated cues");
    for child in node.children() {
        if child.is_comment() {
            ensure!(child.text() == Some(id), "Ottava removal contains another comment");
        } else if child.is_text() {
            ensure!(child.text().unwrap_or_default().trim().is_empty(),
                "Ottava removal contains foreign text");
        } else if child.is_element() {
            match child.tag_name().name() {
                "staff" => ensure!(!child.children().any(|node| node.is_element()),
                    "Ottava removal has structured staff content"),
                "offset" => ensure!(!child.children().any(|node| node.is_element())
                    && child.text().is_some_and(|text| text.trim().parse::<i64>().is_ok()),
                    "Ottava removal has unsupported offset content"),
                "direction-type" => {
                    for kind in child.children().filter(|node| node.is_element()) {
                        match kind.tag_name().name() {
                            "octave-shift" => ensure!(kind.children().all(|node| node.is_text()
                                && node.text().unwrap_or_default().trim().is_empty()),
                                "Ottava removal contains structured octave markup"),
                            "words" => ensure!(origin.birth.kind == "continue"
                                && !kind.children().any(|node| node.is_element())
                                && kind.text().map(str::trim) == Some(origin.birth.display.bezeichnung()),
                                "Ottava removal contains words outside its creator cue"),
                            _ => anyhow::bail!("Ottava removal contains non-register direction content"),
                        }
                    }
                    ensure!(child.children().filter(|node| node.is_text())
                        .all(|node| node.text().unwrap_or_default().trim().is_empty()),
                        "Ottava removal contains foreign direction-type text");
                }
                _ => anyhow::bail!("Ottava removal contains a non-register element"),
            }
        } else {
            anyhow::bail!("Ottava removal contains unsupported XML content");
        }
    }
    Ok(())
}

fn requests_with(state: &Session, xml: &str) -> Vec<CueRequest> {
    if let Some(claim) = state
        .claims
        .get(&digest(xml))
        .filter(|c| c.source == xml && c.adopted)
    {
        return claim
            .omissions
            .values()
            .map(|(request, _)| request.clone())
            .collect();
    }
    let Ok(document) = roxmltree::Document::parse(xml) else {
        return Vec::new();
    };
    let mut occurrences = BTreeMap::<&str, usize>::new();
    for node in document.descendants() {
        if node.is_comment()
            && let Some(id) = node.text().filter(|text| text.starts_with("ets-ottava-"))
        {
            *occurrences.entry(id).or_default() += 1;
        }
    }
    let mut requests = Vec::new();
    let mut ordinal = 0;
    for part in document
        .root_element()
        .children()
        .filter(|n| n.has_tag_name("part"))
    {
        // Retain the staff of unowned/ambiguous scopes as well: lack of an
        // origin cannot make a competing line disappear from reader context.
        let mut active = BTreeMap::<usize, (Option<&Origin>, Option<&str>)>::new();
        for (slot, measure) in part
            .children()
            .filter(|n| n.has_tag_name("measure"))
            .enumerate()
        {
            for node in measure.children().filter(|n| n.has_tag_name("direction")) {
                let current = ordinal;
                ordinal += 1;
                let Some(shift) = node.descendants().find(|n| n.has_tag_name("octave-shift"))
                else {
                    continue;
                };
                let Some(line) = shift
                    .attribute("number")
                    .unwrap_or("1")
                    .parse::<usize>()
                    .ok()
                else {
                    continue;
                };
                let staff = child(node, "staff");
                let origin = carried_origin(node).and_then(|id| {
                    state.origins.get(id).filter(|o| {
                        o.validated
                            && occurrences.get(id) == Some(&1)
                            && origin_matches(node, part.attribute("id").unwrap_or(""), o)
                    })
                });
                match shift.attribute("type") {
                    Some("up" | "down") => {
                        let value = if active.contains_key(&line) {
                            None
                        } else {
                            origin
                        };
                        active.insert(line, (value, staff));
                    }
                    Some("stop") => {
                        active.remove(&line);
                    }
                    Some("continue") => {
                        let Some(origin) = origin else {
                            active.insert(line, (None, staff));
                            continue;
                        };
                        let same =
                            active
                                .get(&line)
                                .and_then(|(value, _)| *value)
                                .is_some_and(|start| {
                                    start.birth.staff == origin.birth.staff
                                        && start.birth.display == origin.birth.display
                                });
                        let conflict = active.iter().any(|(number, (_, other_staff))| {
                            *number != line
                                && other_staff.is_none_or(|other| other == origin.birth.staff)
                        });
                        if !same || conflict {
                            continue;
                        }
                        let words = node
                            .descendants()
                            .filter(|n| n.has_tag_name("words"))
                            .collect::<Vec<_>>();
                        let [word] = words.as_slice() else {
                            continue;
                        };
                        if word.text() != Some(origin.birth.display.bezeichnung())
                            || word.attribute("font-style") != Some("italic")
                        {
                            continue;
                        }
                        let Ok(staff) = origin.birth.staff.parse::<u8>() else {
                            continue;
                        };
                        requests.push(CueRequest {
                            id: carried_origin(node).unwrap().to_owned(),
                            ordinal: current,
                            slot: slot + 1,
                            staff,
                            display: origin.birth.display,
                        });
                    }
                    _ => {
                        active.insert(line, (None, staff));
                    }
                }
            }
        }
    }
    requests
}
pub(crate) fn pdf_requests(xml: &str) -> Vec<CueRequest> {
    requests_with(&session().lock().unwrap_or_else(|e| e.into_inner()), xml)
}
pub(crate) fn expected_visible_line(xml: &str, id: &str) -> Option<VisibleLine> {
    let state = session().lock().unwrap_or_else(|e| e.into_inner());
    state
        .claims
        .get(&digest(xml))
        .filter(|c| c.adopted && c.source == xml)?
        .omissions
        .values()
        .find(|(request, _)| request.id == id)
        .map(|(_, line)| line.clone())
}
pub(crate) fn observe_probe(
    source: &str,
    shadow: &str,
    directives: &[HorizontalLayoutMaterializationDirective],
    context: &PdfContext,
) {
    if context.xml_sha256 != digest(shadow) || context.pdf_sha256.is_empty() {
        return;
    }
    let mut state = session().lock().unwrap_or_else(|e| e.into_inner());
    let requests = requests_with(&state, shadow);
    let observed = requests
        .iter()
        .filter_map(|request| {
            context
                .supported
                .get(&request.id)
                .map(|line| (request.id.clone(), line.clone()))
        })
        .collect();
    state
        .probes
        .insert((digest(source), layout_key(directives)), observed);
}

/// Domain decision on the actual selected pre-layout source. Missing observations
/// plan only removal of our non-printing transport markers, preserving all cues.
pub(crate) fn plan_final_document(
    source: &str,
    directives: &[HorizontalLayoutMaterializationDirective],
) -> Result<()> {
    let document = roxmltree::Document::parse(source)?;
    let mut state = session().lock().unwrap_or_else(|e| e.into_inner());
    let requests = requests_with(&state, source);
    let observed = state
        .probes
        .get(&(digest(source), layout_key(directives)))
        .cloned()
        .unwrap_or_default();
    let mut removals = Vec::new();
    let mut omissions = BTreeMap::new();
    for (current, node) in document
        .descendants()
        .filter(|n| n.has_tag_name("direction"))
        .enumerate()
    {
        for marker in node.children().filter(|child| child.is_comment()) {
            if marker
                .text()
                .is_some_and(|id| state.origins.contains_key(id))
            {
                removals.push(marker.range());
            }
        }
        let Some(id) = carried_origin(node).filter(|id| state.origins.contains_key(*id)) else {
            continue;
        };
        let Some(request) = requests.iter().find(|r| r.ordinal == current && r.id == id) else {
            continue;
        };
        let Some(line) = observed.get(id) else {
            continue;
        };
        let word = node
            .children()
            .filter(|n| n.has_tag_name("direction-type"))
            .find(|n| {
                n.children().any(|c| {
                    c.has_tag_name("words") && c.text() == Some(request.display.bezeichnung())
                })
            });
        if let Some(word) = word {
            let expected = format!(
                "<direction-type><words font-style=\"italic\">{}</words></direction-type>",
                request.display.bezeichnung()
            );
            if source.get(word.range()) == Some(expected.as_str()) {
                removals.push(word.range());
                omissions.insert(current, (request.clone(), line.clone()));
            }
        }
    }
    removals.sort_by_key(|range| range.start);
    let plan = FinalPlan {
        source: source.to_owned(),
        directives: layout_key(directives),
        removals,
        omissions,
    };
    state.plans.insert(digest(source), plan);
    Ok(())
}
pub(crate) fn final_removals(
    source: &str,
    directives: &[HorizontalLayoutMaterializationDirective],
) -> Vec<Range<usize>> {
    session()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .plans
        .get(&digest(source))
        .filter(|p| p.source == source && p.directives == layout_key(directives))
        .map(|p| p.removals.clone())
        .unwrap_or_default()
}

/// E012 preserves direction-container order; E007 inserts only phrase notation.
/// The real E007 receipt seals this carried ordinal course to its exact post.
fn observe_final_execution(
    observation: &crate::master_state_identity::CommittedFinalLayoutObservation,
) -> Result<()> {
    let (source, post) = observation.documents();
    let mut state = session().lock().unwrap_or_else(|e| e.into_inner());
    let Some(plan) = state
        .plans
        .get(&digest(source))
        .filter(|p| p.source == source)
        .cloned()
    else {
        return Ok(());
    };
    let before = roxmltree::Document::parse(source)?;
    let after = roxmltree::Document::parse(post)?;
    let old = before
        .descendants()
        .filter(|n| n.has_tag_name("direction"))
        .collect::<Vec<_>>();
    let new = after
        .descendants()
        .filter(|n| n.has_tag_name("direction"))
        .collect::<Vec<_>>();
    ensure!(
        old.len() == new.len(),
        "E012/E007 changed the carried direction-container course"
    );
    for (&index, (request, _)) in &plan.omissions {
        let a = old
            .get(index)
            .ok_or_else(|| anyhow::anyhow!("Missing carried source direction"))?;
        let b = new
            .get(index)
            .ok_or_else(|| anyhow::anyhow!("Missing carried post direction"))?;
        ensure!(
            carried_origin(*a) == Some(request.id.as_str()),
            "Foreign carried origin"
        );
        let shift = |n: Node<'_, '_>| {
            n.descendants()
                .find(|n| n.has_tag_name("octave-shift"))
                .map(|n| {
                    (
                        n.attribute("type").map(str::to_owned),
                        n.attribute("size").map(str::to_owned),
                        n.attribute("number").map(str::to_owned),
                    )
                })
        };
        ensure!(
            shift(*a) == shift(*b)
                && child(*a, "staff") == child(*b, "staff")
                && a.attribute("placement") == b.attribute("placement")
                && !b.descendants().any(|n| n.has_tag_name("words")),
            "Ottava carried post differs outside its approved word"
        );
    }
    if !plan.omissions.is_empty() {
        eprintln!(
            "OttavaReaderDecision: omit={}; source_sha256={}; post_sha256={}; authority=accepted_reader_scope_existing_E012_E007",
            plan.omissions.len(),
            digest(source),
            digest(post)
        );
        state.claims.insert(
            digest(post),
            FinalClaim {
                source: post.to_owned(),
                omissions: plan.omissions,
                adopted: false,
            },
        );
    }
    Ok(())
}
fn observe_adopted_document(xml: &str, domain: crate::ssc_evidence_proof::SscAdoptionDomain) {
    let Some(observation) =
        crate::master_state_identity::committed_final_layout_observation(xml, domain)
    else {
        return;
    };
    if let Err(error) = observe_final_execution(&observation) {
        eprintln!("OttavaReaderReceipt: UNKNOWN; {error}; receipt_and_adoption_unchanged=true");
        return;
    }
    let mut state = session().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(claim) = state
        .claims
        .get_mut(&digest(xml))
        .filter(|c| c.source == xml)
    {
        claim.adopted = true;
    }
}

pub(crate) fn omission_is_adopted(xml: &str, ordinal: usize) -> bool {
    session()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .claims
        .get(&digest(xml))
        .is_some_and(|c| c.source == xml && c.adopted && c.omissions.contains_key(&ordinal))
}
pub(crate) fn validate_final_pdf(xml: &str, context: &PdfContext) -> Result<()> {
    let state = session().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(claim) = state
        .claims
        .get(&digest(xml))
        .filter(|c| c.source == xml && c.adopted)
    {
        ensure!(
            context.xml_sha256 == digest(xml) && !context.pdf_sha256.is_empty(),
            "Ottava final PDF observation is foreign or unavailable"
        );
        for (request, line) in claim.omissions.values() {
            ensure!(
                context
                    .supported
                    .get(&request.id)
                    .is_some_and(|current| current.same_horizontal_course(line)),
                "Ottava final PDF no longer proves the approved orientation and continuous line"
            );
        }
    }
    Ok(())
}

pub(crate) fn final_pdf_observation_required(xml: &str) -> bool {
    session()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .claims
        .get(&digest(xml))
        .is_some_and(|claim| claim.source == xml && claim.adopted && !claim.omissions.is_empty())
}

pub(crate) fn observe_master_adoption(
    master: &crate::structural_write_authority::MasterAdoptedState,
) {
    observe_adopted_document(
        master.document(),
        crate::ssc_evidence_proof::SscAdoptionDomain::Master,
    );
}
pub(crate) fn observe_render_adoption(
    render: &crate::structural_write_authority::RenderSovereignState,
) {
    if let Some(xml) = render.document() {
        observe_adopted_document(xml, crate::ssc_evidence_proof::SscAdoptionDomain::Render);
    }
}
pub(crate) fn append_report(out: &mut String) {
    use std::fmt::Write as _;
    let state = session().lock().unwrap_or_else(|e| e.into_inner());
    let omitted = state
        .claims
        .values()
        .filter(|c| c.adopted)
        .flat_map(|c| c.omissions.values().map(|(r, _)| r.id.as_str()))
        .collect::<BTreeSet<_>>();
    let continuations = state
        .origins
        .iter()
        .filter(|(_, o)| o.validated && o.birth.kind == "continue")
        .collect::<Vec<_>>();
    let _ = writeln!(
        out,
        "OttavaReaderDecision: Schema 1; CapturedContinuations {}; AdoptedOmissions {}; Authority accepted_scope_existing_E012_E007; OtherContinuations baseline_or_UNKNOWN.",
        continuations.len(),
        omitted.len()
    );
    for (id, _) in continuations {
        let _ = writeln!(
            out,
            "OttavaReaderOrigin: {id}; Decision {}.",
            if omitted.contains(id.as_str()) {
                "OMIT_ADDITIONAL_GENERATED_WORD"
            } else {
                "BASELINE_OR_UNKNOWN"
            }
        );
    }
}

#[cfg(test)]
pub(crate) mod synthetic {
    use super::*;
    pub(crate) fn course(
        display: RegisternotationsDarstellung,
        staff: &str,
        part: &str,
        label: &str,
    ) -> String {
        let direction = |kind: &str, onset| Birth {
            part: part.to_owned(),
            staff: staff.to_owned(),
            slot: 1,
            onset,
            kind: kind.to_owned(),
            display,
            line: 1,
        };
        let start = if matches!(
            display,
            RegisternotationsDarstellung::Achtva | RegisternotationsDarstellung::Fuenfzehnma
        ) {
            "down"
        } else {
            "up"
        };
        let births = vec![
            direction(start, 0),
            direction("continue", 2),
            direction("stop", 3),
        ];
        let plan = RegisternotationsSichtmaterialisierungsauftrag {
            plaene: vec![super::super::RegisternotationsMaterialisierungsPlan {
                teil: part.to_owned(),
                beginn_takt_ordinal: 1,
                ende_takt_ordinal: 1,
                system: staff.to_owned(),
                beginn_einsatz: 0,
                ende_einsatz: 3,
                darstellung: display,
                sichtwiederholungen: vec![super::super::RegisternotationsSichtwiederholung {
                    takt_ordinal: 1,
                    einsatz: 2,
                }],
            }],
            semantischer_plan_fingerabdruck: "synthetic_semantic".into(),
            sicht_fingerabdruck: "synthetic_visible".into(),
            semantische_plaene: 2,
            kontaktfusionen: 0,
            stille_bruecken: 1,
        };
        let ids = issue_origins(&plan, &births);
        let place = if start == "down" { "above" } else { "below" };
        let mut body = String::new();
        for (index, (birth, id)) in births.iter().zip(ids).enumerate() {
            let word = if birth.kind == "continue" {
                format!(
                    "<direction-type><words font-style=\"italic\">{}</words></direction-type>",
                    display.bezeichnung()
                )
            } else {
                String::new()
            };
            body.push_str(&format!("<direction placement=\"{place}\"><!--{id}-->{word}<direction-type><octave-shift type=\"{}\" size=\"{}\" number=\"1\"/></direction-type><staff>{staff}</staff></direction>",birth.kind,display.groesse()));
            if index < 2 {
                body.push_str(&format!("<note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><voice>renamed</voice><type>quarter</type><staff>{staff}</staff></note>"));
            }
        }
        let xml = format!(
            "<score-partwise version=\"4.0\"><part-list><score-part id=\"{part}\"><part-name>Piano</part-name></score-part></part-list><part id=\"{part}\"><measure number=\"{label}\"><attributes><divisions>1</divisions><staves>2</staves></attributes>{body}</measure></part></score-partwise>"
        );
        confirm_materialization(&xml).unwrap();
        xml
    }
    pub(crate) fn directives() -> Vec<HorizontalLayoutMaterializationDirective> {
        vec![HorizontalLayoutMaterializationDirective {
            explicit_width: None,
            system_break:
                crate::structural_write_authority::HorizontalSystemBreakMaterialization::None,
            starts_new_page: false,
        }]
    }
    pub(crate) fn signature() -> VisibleLine {
        VisibleLine {
            page: 1,
            stroke: [0; 4],
            orientation: [0; 4],
            cell_x: [0; 2],
        }
    }
}
#[cfg(test)]
#[path = "../../tests/unit/ottava_reader_policy.rs"]
mod tests;

#[cfg(test)]
#[path = "../../tests/unit/ottava_origin_registry_transaction.rs"]
mod origin_registry_transaction_tests;
