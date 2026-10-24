//! Writer-authored optional connections; identity is inherited, never recovered
//! from a later beam grouping. This module plans no new voice assignment.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

type OccurrenceKey = (usize, usize, usize);
type Position = (usize, usize);
type BeamImage = Vec<(u8, String)>;
pub(crate) type RestBridgeWithdrawalRows = Vec<(usize, usize, Vec<(usize, &'static str)>)>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Member {
    identity: OccurrenceKey,
    position: Position,
    original: bool,
    anchor: Option<OccurrenceKey>,
    head: bool,
    onset: i64,
    duration: i64,
    pitch: Option<i32>,
    staff: String,
    voice: String,
    sealed_voice: String,
    chord: bool,
    tie_start: bool,
    tie_stop: bool,
    before: BeamImage,
    written: BeamImage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Connection {
    members: Vec<Member>,
    resources: Vec<Member>,
    component: BTreeSet<OccurrenceKey>,
    origin: (usize, usize),
    // r1484: written although the sealed plan splits it; must be decided at
    // withdrawal and may never silently lose custody before that point.
    provisional: bool,
}
impl Connection {
    fn all(&self) -> impl Iterator<Item = &Member> {
        self.members.iter().chain(&self.resources)
    }
    fn diagnostic(&self, project: &str, branch: &str, phase: &str, reason: &str) {
        eprintln!(
            "BridgeExtensionCustody: project={project:?}; branch={branch}; phase={phase}; reason={reason}; origin={:?}; members={:?}; resources={:?}",
            self.origin,
            self.members
                .iter()
                .map(|m| (m.identity, m.position))
                .collect::<Vec<_>>(),
            self.resources
                .iter()
                .map(|m| (m.identity, m.position))
                .collect::<Vec<_>>()
        );
    }
}

/// Pending writer output. No Clone or public fields: a proposal is consumed
/// once at its actual successful Main adoption, or simply discarded.
#[derive(Debug)]
pub(crate) struct RestBridgeExtensionOccurrenceEvidence {
    course: RestBridgeExtensionCourse,
    state_address: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RestBridgeExtensionCourse {
    project: String,
    branch: &'static str,
    xml: String,
    source: std::sync::Arc<[FinalChordCanonicalSourceLocator]>,
    connections: Vec<Connection>,
    // Origins of every provisional connection bound at capture; never narrowed.
    provisional_origins: BTreeSet<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub(super) struct BoundRestBridgeWithdrawal {
    input: String,
    rows: RestBridgeWithdrawalRows,
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn notes(xml: &str) -> io::Result<BTreeMap<Position, FinalChordEncodingNote>> {
    let mut result = BTreeMap::new();
    for measure in final_chord_parse_binding_measures(xml)? {
        for (ordinal, note) in measure
            .notes
            .into_iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .enumerate()
        {
            if result.insert((note.measure, ordinal), note).is_some() {
                return Err(invalid(
                    "Bridge continuation has duplicate physical position",
                ));
            }
        }
    }
    Ok(result)
}

fn beams(xml: &str, note: &FinalChordEncodingNote) -> io::Result<BeamImage> {
    let doc = roxmltree::Document::parse(&xml[note.range.clone()])
        .map_err(|_| invalid("Bridge continuation has invalid note XML"))?;
    let mut result = BTreeMap::new();
    for node in doc
        .root_element()
        .children()
        .filter(|node| node.has_tag_name("beam"))
    {
        let level = node
            .attribute("number")
            .unwrap_or("1")
            .parse::<u8>()
            .map_err(|_| invalid("Bridge continuation has invalid beam level"))?;
        if result
            .insert(level, node.text().unwrap_or_default().trim().to_owned())
            .is_some()
        {
            return Err(invalid("Bridge continuation has duplicate beam level"));
        }
    }
    Ok(result.into_iter().collect())
}

fn complete_resource_image(
    current: &BTreeMap<Position, FinalChordEncodingNote>,
    bindings: &BTreeMap<(usize, usize), OccurrenceKey>,
    source: &[FinalChordCanonicalSourceLocator],
    heads: &BTreeSet<(usize, usize)>,
    component: &BTreeSet<OccurrenceKey>,
) -> Result<BTreeSet<Position>, &'static str> {
    let by_identity = bindings
        .iter()
        .map(|(range, key)| (*key, *range))
        .collect::<BTreeMap<_, _>>();
    if by_identity.len() != bindings.len() {
        return Err("duplicate_bound_occurrence");
    }
    let mut required = component.clone();
    for head in heads {
        required.insert(*bindings.get(head).ok_or("unbound_head")?);
    }
    loop {
        let prior = required.len();
        let attacks = required
            .iter()
            .map(|key| (key.0, key.1))
            .collect::<BTreeSet<_>>();
        for locator in source {
            if attacks.contains(&(locator.source_event_id, locator.attack_id)) {
                required.insert((
                    locator.source_event_id,
                    locator.attack_id,
                    locator.fragment_id,
                ));
            }
        }
        for key in bindings.values() {
            if attacks.contains(&(key.0, key.1)) {
                required.insert(*key);
            }
        }
        let mut anchors = BTreeSet::new();
        for key in &required {
            let range = by_identity.get(key).ok_or("missing_resource_fragment")?;
            let note = current
                .values()
                .find(|n| (n.range.start, n.range.end) == *range)
                .ok_or("missing_resource_note")?;
            anchors.insert((
                note.measure,
                note.anchor_index.unwrap_or(note.raw_note_index),
            ));
        }
        for note in current.values() {
            if anchors.contains(&(
                note.measure,
                note.anchor_index.unwrap_or(note.raw_note_index),
            )) {
                required.insert(
                    *bindings
                        .get(&(note.range.start, note.range.end))
                        .ok_or("unbound_chord_resource")?,
                );
            }
        }
        if required.len() == prior {
            break;
        }
    }
    required
        .iter()
        .map(|key| {
            let range = by_identity.get(key).ok_or("missing_resource_fragment")?;
            current
                .iter()
                .find(|(_, n)| (n.range.start, n.range.end) == *range)
                .map(|(p, _)| *p)
                .ok_or("missing_resource_note")
        })
        .collect()
}

impl Member {
    fn matches(&self, xml: &str, note: &FinalChordEncodingNote, projection: bool) -> bool {
        self.onset == note.onset
            && self.duration == note.duration
            && self.pitch == note.pitch_midi
            && self.staff == note.staff
            && self.chord == note.chord_member
            && self.tie_start == note.tie_start
            && self.tie_stop == note.tie_stop
            && (projection || self.voice == note.voice)
            && beams(xml, note).ok().as_ref() == Some(&self.written)
    }
}

fn attachments_match(
    connection: &Connection,
    current: &BTreeMap<Position, FinalChordEncodingNote>,
) -> bool {
    for member in connection.all() {
        let Some(note) = current.get(&member.position) else {
            return false;
        };
        if let Some(identity) = member.anchor {
            let Some(anchor) = connection.all().find(|m| m.identity == identity) else {
                return false;
            };
            let Some(head) = current.get(&anchor.position) else {
                return false;
            };
            if head.measure != note.measure || note.anchor_index != Some(head.raw_note_index) {
                return false;
            }
        } else if note.anchor_index.is_some() {
            return false;
        }
        if !member.chord {
            let actual = current
                .iter()
                .filter(|(_, candidate)| {
                    candidate.measure == note.measure
                        && candidate.chord_member
                        && candidate.anchor_index == Some(note.raw_note_index)
                })
                .map(|(position, _)| *position)
                .collect::<BTreeSet<_>>();
            let carried = connection
                .all()
                .filter(|m| m.anchor == Some(member.identity))
                .map(|m| m.position)
                .collect::<BTreeSet<_>>();
            if actual != carried {
                return false;
            }
        }
    }
    true
}

impl Connection {
    /// Exact restoration rows of this connection's own written heads.
    fn restoration_rows(
        &self,
        xml: &str,
        current: &BTreeMap<Position, FinalChordEncodingNote>,
        claimed: &mut BTreeSet<Position>,
    ) -> io::Result<RestBridgeWithdrawalRows> {
        let connection = self;
        let mut rows = Vec::new();
        for member in &connection.members {
            let note = current
                .get(&member.position)
                .ok_or_else(|| invalid("Bridge withdrawal lost member"))?;
            if !member.matches(xml, note, false) {
                return Err(invalid("Bridge withdrawal has drifted member"));
            }
            if !member.head || member.before == member.written {
                continue;
            }
            if !claimed.insert(member.position) {
                return Err(invalid("Bridge withdrawal overlapping batch"));
            }
            let previous = member
                .before
                .iter()
                .filter(|(level, _)| *level <= 2)
                .map(|(level, state)| {
                    let state = match state.as_str() {
                        "begin" => "begin",
                        "continue" => "continue",
                        "end" => "end",
                        "forward hook" => "forward hook",
                        "backward hook" => "backward hook",
                        _ => {
                            return Err(invalid(
                                "Bridge withdrawal unknown original beam state",
                            ));
                        }
                    };
                    Ok((*level as usize, state))
                })
                .collect::<io::Result<Vec<_>>>()?;
            rows.push((note.range.start, note.range.end, previous));
        }
        Ok(rows)
    }
}

impl RestBridgeExtensionCourse {
    /// Every provisional connection bound at capture must still be carried.
    fn require_provisional_custody(&self) -> io::Result<()> {
        let carried = self.connections.iter().map(|c| c.origin).collect::<BTreeSet<_>>();
        if self.provisional_origins.is_subset(&carried) {
            return Ok(());
        }
        eprintln!(
            "BridgeExtensionCustody: project={:?}; branch={}; phase=provisional_custody_lost; lost={:?}",
            self.project,
            self.branch,
            self.provisional_origins.difference(&carried).collect::<Vec<_>>()
        );
        Err(invalid("Provisional rest bridge lost custody before its withdrawal decision"))
    }
    fn has_provisional(&self) -> bool {
        !self.provisional_origins.is_empty()
    }
}

impl RestBridgeExtensionCourse {
    pub(super) fn current_xml(&self) -> &str {
        &self.xml
    }
    // Every supported ordinary transition proves the whole pitched sequence and
    // separately the participating connection. Relevant drift retires a whole
    // connection permanently; an unchanged later image cannot recreate it.
    pub(super) fn through(&mut self, before: &str, after: &str) -> io::Result<()> {
        if self.xml != before {
            return Err(invalid(
                "Bridge continuation has a skipped/stale owner transition",
            ));
        }
        let topology = final_chord_verify_pitched_sequence_transition(before, after, false).is_ok();
        let current = notes(after)?;
        let prior_count = self.connections.len();
        self.connections.retain(|connection| {
            let retained = topology
                && connection.all().all(|member| {
                    current
                        .get(&member.position)
                        .is_some_and(|note| member.matches(after, note, false))
                })
                && attachments_match(connection, &current);
            if !retained {
                connection.diagnostic(
                    &self.project,
                    self.branch,
                    "owner_transition",
                    if topology {
                        "connection_resource_drift"
                    } else {
                        "global_pitched_sequence_drift"
                    },
                );
            }
            retained
        });
        if prior_count != self.connections.len() {
            eprintln!(
                "BridgeExtensionCustody: phase=owner_transition; retained={}; retired={}",
                self.connections.len(),
                prior_count - self.connections.len()
            );
        }
        self.xml = after.to_owned();
        Ok(())
    }

    pub(super) fn projected(
        &self,
        input: &str,
        output: &str,
        occurrences: &[FinalChordTransactionOutputOccurrenceWitness],
    ) -> io::Result<Self> {
        if self.xml != input {
            return Err(invalid("Bridge FinalChord handover has stale source"));
        }
        let current = notes(output)?;
        let mut result = self.clone();
        result.connections.retain_mut(|connection| {
            let owned = connection
                .all()
                .map(|m| m.identity)
                .collect::<BTreeSet<_>>();
            if occurrences.iter().any(|row| {
                connection
                    .all()
                    .any(|m| (m.identity.0, m.identity.1) == (row.source_event_id, row.attack_id))
                    && !owned.contains(&(row.source_event_id, row.attack_id, row.fragment_id))
            }) {
                return false;
            }
            let mut updated = Vec::new();
            let mut positions = BTreeSet::new();
            for member in connection.all() {
                let rows = occurrences
                    .iter()
                    .filter(|row| {
                        (row.source_event_id, row.attack_id, row.fragment_id) == member.identity
                    })
                    .collect::<Vec<_>>();
                let [row] = rows.as_slice() else {
                    return false;
                };
                let position = (row.measure, row.pitched_ordinal);
                if !positions.insert(position) {
                    return false;
                }
                let Some(note) = current.get(&position) else {
                    return false;
                };
                if !member.matches(output, note, true) || member.sealed_voice != note.voice.trim() {
                    return false;
                }
                let mut carried = member.clone();
                carried.position = position;
                carried.voice = note.voice.clone();
                updated.push(carried);
            }
            let resources = updated.split_off(connection.members.len());
            connection.members = updated;
            connection.resources = resources;
            attachments_match(connection, &current)
        });
        for connection in &self.connections {
            if !result
                .connections
                .iter()
                .any(|c| c.origin == connection.origin)
            {
                connection.diagnostic(
                    &self.project,
                    self.branch,
                    "final_chord_handover",
                    "incomplete_or_changed_projected_resources",
                );
            }
        }
        if result.connections.len() != self.connections.len() {
            eprintln!(
                "BridgeExtensionCustody: phase=final_chord_handover; retained={}; retired={}",
                result.connections.len(),
                self.connections.len() - result.connections.len()
            );
        }
        result.xml = output.to_owned();
        Ok(result)
    }
}

impl CanonicalFinalChordStructuralPlan {
    /// Binds only the local custody of this pending Main operation. The address
    /// is not a source identity or selector and is never persisted or ranked.
    pub(crate) fn bind_rest_bridge_main_state(
        &mut self,
        state: &super::super::RenderSovereignState,
    ) {
        self.rest_bridge_main_state = Some((
            std::ptr::from_ref(state) as usize,
            state.artifact_path().display().to_string(),
        ));
    }

    #[cfg(test)]
    pub(crate) fn capture_rest_bridge_extension_output(
        &self,
        input: &str,
        output: &str,
        selected: &[Vec<(usize, usize)>],
        receipt: &crate::master_state_identity::PreservingTransformationReceipt,
    ) -> io::Result<Option<RestBridgeExtensionOccurrenceEvidence>> {
        self.capture_rest_bridge_extension_output_offered(input, output, selected, &[], receipt)
            .map(|(evidence, _)| evidence)
    }

    /// As above; `offered[i]` marks a provisional group (split in the sealed plan).
    /// Returns the indices of all groups whose complete evidence was bound.
    pub(crate) fn capture_rest_bridge_extension_output_offered(
        &self,
        input: &str,
        output: &str,
        selected: &[Vec<(usize, usize)>],
        offered: &[bool],
        receipt: &crate::master_state_identity::PreservingTransformationReceipt,
    ) -> io::Result<(Option<RestBridgeExtensionOccurrenceEvidence>, BTreeSet<usize>)> {
        let mut bound_offers = BTreeSet::new();
        receipt.validate_preserving_transition::<crate::pipeline::StructuralRestBridgeVariantAPreservingMint>(input, output)
            .map_err(|_| invalid("Bridge companion lacks the actual E050 pre/post receipt"))?;
        let Some((state_address, project)) = &self.rest_bridge_main_state else {
            return Ok((None, bound_offers));
        };
        let guard = self
            .bridge_voice_guard
            .as_ref()
            .ok_or_else(|| invalid("Bridge companion lacks bound guard"))?;
        if !guard.matches_input(input) {
            return Err(invalid("Bridge companion input is stale"));
        }
        final_chord_verify_pitched_sequence_transition(input, output, false)?;
        let before = notes(input)?;
        let after = notes(output)?;
        let bindings = guard.occurrence_bindings();
        let mut connections = Vec::new();
        let mut claimed = BTreeSet::new();
        for (offer_index, heads) in selected.iter().enumerate() {
            let skipped = |reason: &str| {
                eprintln!(
                    "BridgeExtensionCustody: project={project:?}; branch=main; phase=capture_declined; reason={reason}; writer_heads={heads:?}; bound_identities={:?}",
                    heads
                        .iter()
                        .map(|range| bindings.get(range))
                        .collect::<Vec<_>>()
                )
            };
            let selected_heads = heads.iter().copied().collect::<BTreeSet<_>>();
            if selected_heads.len() != heads.len() {
                return Err(invalid("Bridge companion duplicate selected head"));
            }
            let identities = heads
                .iter()
                .map(|range| bindings.get(range).copied())
                .collect::<Option<BTreeSet<_>>>();
            let Some(identities) = identities else {
                skipped("unbound_identity");
                continue;
            };
            let originals = self
                .original_beam_voice_groups
                .iter()
                .enumerate()
                .flat_map(|(component, group)| {
                    group
                        .closed_bridge_origins(&identities)
                        .into_iter()
                        .map(move |(origin, members)| (component, origin, members))
                })
                .collect::<Vec<_>>();
            let [(component_index, origin_index, original)] = originals.as_slice() else {
                skipped("no_unique_closed_positive_origin");
                continue;
            };
            if original.len() >= identities.len() {
                skipped("no_extension_head");
                continue;
            }
            let component =
                self.original_beam_voice_groups[*component_index].bridge_component_occurrences();
            let positions = match complete_resource_image(
                &before,
                bindings,
                &self.source_locators,
                &selected_heads,
                &component,
            ) {
                Ok(positions) => positions,
                Err(reason) => {
                    skipped(reason);
                    continue;
                }
            };
            let mut members = Vec::new();
            let mut resources = Vec::new();
            let mut complete = true;
            for (position, note) in &before {
                let is_head = selected_heads.contains(&(note.range.start, note.range.end));
                let attached = note.chord_member
                    && before.values().any(|head| {
                        head.measure == note.measure
                            && Some(head.raw_note_index) == note.anchor_index
                            && selected_heads.contains(&(head.range.start, head.range.end))
                    });
                if !positions.contains(position) {
                    continue;
                }
                let Some(identity) = bindings.get(&(note.range.start, note.range.end)).copied()
                else {
                    complete = false;
                    break;
                };
                let Some(post) = after.get(position) else {
                    complete = false;
                    break;
                };
                let prior = beams(input, note)?;
                let written = beams(output, post)?;
                let anchor = if note.chord_member {
                    before
                        .values()
                        .find(|head| {
                            head.measure == note.measure
                                && Some(head.raw_note_index) == note.anchor_index
                        })
                        .and_then(|head| bindings.get(&(head.range.start, head.range.end)))
                        .copied()
                } else {
                    None
                };
                if note.chord_member && anchor.is_none() {
                    complete = false;
                    break;
                }
                let original = (is_head || attached)
                    && (original.contains(&identity)
                        || anchor.is_some_and(|anchor| original.contains(&anchor)));
                // We can retract an extension only when its extra heads had no
                // pre-existing beam at the levels owned by this operation.
                if is_head && !original && prior.iter().any(|(level, _)| *level <= 2) {
                    complete = false;
                    break;
                }
                if note.onset != post.onset
                    || note.duration != post.duration
                    || note.chord_member != post.chord_member
                    || note.voice != post.voice
                    || note.staff != post.staff
                {
                    complete = false;
                    break;
                }
                let member = Member {
                    identity,
                    position: *position,
                    original,
                    anchor,
                    head: is_head,
                    onset: note.onset,
                    duration: note.duration,
                    pitch: note.pitch_midi,
                    staff: note.staff.clone(),
                    voice: note.voice.clone(),
                    sealed_voice: guard
                        .occurrence_labels()
                        .get(&(note.range.start, note.range.end))
                        .ok_or_else(|| invalid("Bridge companion lost sealed label"))?
                        .clone(),
                    chord: note.chord_member,
                    tie_start: note.tie_start,
                    tie_stop: note.tie_stop,
                    before: prior,
                    written,
                };
                if is_head || attached {
                    members.push(member);
                } else {
                    resources.push(member);
                }
            }
            if !complete
                || members.iter().filter(|m| m.head).count() != heads.len()
                || members.iter().all(|m| m.before == m.written)
            {
                skipped("incomplete_or_unchanged_connection");
                continue;
            }
            let primary = members
                .iter()
                .filter(|m| m.head)
                .map(|m| {
                    m.written
                        .iter()
                        .find(|(level, _)| *level == 1)
                        .map(|(_, state)| state.as_str())
                })
                .collect::<Vec<_>>();
            if primary.iter().enumerate().any(|(index, state)| {
                *state
                    != Some(if index == 0 {
                        "begin"
                    } else if index + 1 == primary.len() {
                        "end"
                    } else {
                        "continue"
                    })
            }) {
                continue;
            }
            let writes = members
                .iter()
                .filter(|m| m.head && m.before != m.written)
                .map(|m| m.identity)
                .collect::<BTreeSet<_>>();
            if writes.iter().any(|id| !claimed.insert(*id)) {
                return Err(invalid("Bridge companion overlapping operations"));
            }
            let connection = Connection {
                members,
                resources,
                component,
                origin: (*component_index, *origin_index),
                provisional: offered.get(offer_index).copied().unwrap_or(false),
            };
            connection.diagnostic(
                project,
                "main",
                "capture_bound",
                "writer_output_with_complete_resources",
            );
            connections.push(connection);
            bound_offers.insert(offer_index);
        }
        if connections.is_empty() {
            return Ok((None, bound_offers));
        }
        let provisional_origins = connections
            .iter()
            .filter(|connection| connection.provisional)
            .map(|connection| connection.origin)
            .collect();
        Ok((Some(RestBridgeExtensionOccurrenceEvidence {
            state_address: *state_address,
            course: RestBridgeExtensionCourse {
                project: project.clone(),
                branch: "main",
                xml: output.to_owned(),
                source: self.source_locators.clone(),
                connections,
                provisional_origins,
            },
        }), bound_offers))
    }

    pub(crate) fn adopt_rest_bridge_extension_output(
        &mut self,
        evidence: Option<RestBridgeExtensionOccurrenceEvidence>,
        state: &super::super::RenderSovereignState,
    ) -> io::Result<()> {
        let Some(evidence) = evidence else {
            return Ok(());
        };
        if evidence.state_address != std::ptr::from_ref(state) as usize
            || state.document() != Some(evidence.course.xml.as_str())
            || !std::sync::Arc::ptr_eq(&self.source_locators, &evidence.course.source)
        {
            return Err(invalid(
                "Bridge companion is not the adopted Main writer output",
            ));
        }
        eprintln!(
            "BridgeExtensionCustody: project={:?}; branch=main; phase=main_adopted; connections={}",
            evidence.course.project,
            evidence.course.connections.len()
        );
        self.rest_bridge_extension_course = Some(evidence.course);
        Ok(())
    }

    pub(crate) fn continue_rest_bridge_extensions_at_state(
        &mut self,
        before: &str,
        after: Option<&str>,
    ) -> io::Result<()> {
        match (self.rest_bridge_extension_course.as_mut(), after) {
            (None, _) => Ok(()),
            (Some(course), Some(after)) => course.through(before, after),
            (Some(_), None) => Err(invalid("Active bridge custody lost its adopted Main state")),
        }
    }

    pub(crate) fn continue_rest_bridge_extensions(
        &mut self,
        before: &str,
        after: &str,
    ) -> io::Result<()> {
        if let Some(course) = &mut self.rest_bridge_extension_course {
            course.through(before, after)?;
        }
        Ok(())
    }

    pub(crate) fn bind_rest_bridge_withdrawal(
        &self,
        source: &FinalPdfSourceOccurrenceContinuation,
        plan: &PrimaryBeamVoicePartitionBoundPlan,
    ) -> io::Result<Self> {
        if !plan.matches_source_xml(source.xml()) || !source.has_authentic_beam_surface()? {
            return Err(invalid("Bridge withdrawal has stale source or assignment"));
        }
        let current = notes(source.xml())?;
        let mut rows = Vec::new();
        let mut claimed = BTreeSet::new();
        if let Some(course) = &source.receipt.rest_bridge_extension_course {
            if course.xml != source.xml()
                || !std::sync::Arc::ptr_eq(&course.source, &self.source_locators)
            {
                return Err(invalid("Bridge withdrawal lost the original course"));
            }
            course.require_provisional_custody()?;
            for connection in &course.connections {
                if connection.provisional {
                    // A provisional bridge stays only when the bound final
                    // assignment keeps every member in one lane; anything else,
                    // including an unknown label, withdraws it.
                    let lanes = connection
                        .members
                        .iter()
                        .map(|member| plan.repaired_voice_plan.label_for_source(member.identity.0))
                        .collect::<Option<BTreeSet<_>>>();
                    if lanes.is_some_and(|lanes| lanes.len() == 1) {
                        connection.diagnostic(&course.project, course.branch,
                            "withdrawal_declined", "provisional_same_final_lane");
                        continue;
                    }
                    connection.diagnostic(&course.project, course.branch,
                        "withdrawal_selected", "provisional_separated_or_unknown_final_lane");
                    rows.extend(connection.restoration_rows(source.xml(), &current, &mut claimed)?);
                    continue;
                }
                let component_labels = connection
                    .component
                    .iter()
                    .map(|key| plan.repaired_voice_plan.label_for_source(key.0))
                    .collect::<Option<BTreeSet<_>>>();
                if component_labels.is_none_or(|labels| labels.len() != 1) {
                    connection.diagnostic(
                        &course.project,
                        course.branch,
                        "withdrawal_declined",
                        "original_component_not_together",
                    );
                    continue;
                }
                let labels = connection
                    .members
                    .iter()
                    .map(|member| {
                        plan.repaired_voice_plan
                            .label_for_source(member.identity.0)
                            .map(str::to_owned)
                    })
                    .collect::<Option<BTreeSet<_>>>();
                let Some(labels) = labels else {
                    continue;
                };
                if labels.len() <= 1 {
                    connection.diagnostic(
                        &course.project,
                        course.branch,
                        "withdrawal_declined",
                        "connection_same_final_lane",
                    );
                    continue;
                }
                let original_labels = connection
                    .members
                    .iter()
                    .filter(|m| m.original)
                    .map(|member| plan.repaired_voice_plan.label_for_source(member.identity.0))
                    .collect::<Option<BTreeSet<_>>>();
                if original_labels.is_none_or(|labels| labels.len() != 1) {
                    continue;
                }
                connection.diagnostic(
                    &course.project,
                    course.branch,
                    "withdrawal_selected",
                    "connection_separated_with_original_component_together",
                );
                rows.extend(connection.restoration_rows(source.xml(), &current, &mut claimed)?);
            }
        }
        eprintln!(
            "BridgeExtensionCustody: phase=withdrawal_bound; beam_rows={}; voice_write=false",
            rows.len()
        );
        let mut bound = self.clone();
        bound.rest_bridge_withdrawal = Some(BoundRestBridgeWithdrawal {
            input: source.xml().to_owned(),
            rows,
        });
        Ok(bound)
    }

    /// Without a bound late assignment no voice changes after the sealed
    /// projection, so every provisional connection that is split in the current
    /// document is withdrawn; ordinary connections are left untouched.
    pub(crate) fn bind_provisional_rest_bridge_fallback(
        &self,
        source: &FinalPdfSourceOccurrenceContinuation,
    ) -> io::Result<Self> {
        if !source.has_authentic_beam_surface()? {
            return Err(invalid("Provisional bridge fallback has unauthentic source"));
        }
        let current = notes(source.xml())?;
        let mut rows = Vec::new();
        let mut claimed = BTreeSet::new();
        if let Some(course) = &source.receipt.rest_bridge_extension_course {
            if course.xml != source.xml()
                || !std::sync::Arc::ptr_eq(&course.source, &self.source_locators)
            {
                return Err(invalid("Provisional bridge fallback lost the original course"));
            }
            course.require_provisional_custody()?;
            for connection in course.connections.iter().filter(|c| c.provisional) {
                let lanes = connection
                    .members
                    .iter()
                    .map(|member| current.get(&member.position).map(|note| note.voice.trim().to_owned()))
                    .collect::<Option<BTreeSet<_>>>();
                if lanes.is_some_and(|lanes| lanes.len() == 1) {
                    connection.diagnostic(&course.project, course.branch,
                        "withdrawal_declined", "provisional_same_current_lane_without_late_plan");
                    continue;
                }
                connection.diagnostic(&course.project, course.branch,
                    "withdrawal_selected", "provisional_split_without_late_plan");
                rows.extend(connection.restoration_rows(source.xml(), &current, &mut claimed)?);
            }
        }
        eprintln!(
            "BridgeExtensionCustody: phase=withdrawal_bound; beam_rows={}; voice_write=false; late_plan=absent",
            rows.len()
        );
        let mut bound = self.clone();
        bound.rest_bridge_withdrawal = Some(BoundRestBridgeWithdrawal {
            input: source.xml().to_owned(),
            rows,
        });
        Ok(bound)
    }

    /// True while adopted custody still holds provisional bridges that no
    /// withdrawal decision has consumed.
    pub(crate) fn has_provisional_rest_bridges(&self) -> bool {
        self.rest_bridge_extension_course.as_ref().is_some_and(RestBridgeExtensionCourse::has_provisional)
    }

    pub(crate) fn rest_bridge_withdrawal_rows(
        &self,
        input: &str,
    ) -> io::Result<Option<&RestBridgeWithdrawalRows>> {
        self.rest_bridge_withdrawal
            .as_ref()
            .map(|bound| {
                if bound.input == input {
                    Ok(&bound.rows)
                } else {
                    Err(invalid("Bridge withdrawal input changed"))
                }
            })
            .transpose()
    }
}

impl FinalPdfSourceOccurrenceContinuation {
    pub(crate) fn has_provisional_rest_bridges(&self) -> bool {
        self.receipt.rest_bridge_extension_course.as_ref().is_some_and(RestBridgeExtensionCourse::has_provisional)
    }
    pub(crate) fn with_rest_bridge_shadow_diagnostics(&self) -> Self {
        let mut copy = self.clone();
        if let Some(course) = &mut copy.receipt.rest_bridge_extension_course {
            course.branch = "shadow";
        }
        copy
    }
    pub(crate) fn through_rest_bridge_withdrawal(
        &self,
        post: &str,
        guard: &CanonicalFinalChordStructuralPlan,
        receipt: &crate::master_state_identity::PreservingTransformationReceipt,
    ) -> io::Result<Self> {
        receipt.validate_preserving_transition::<crate::pipeline::StructuralRestBridgeVariantAPreservingMint>(self.xml(),post)
            .map_err(|_|invalid("Bridge withdrawal lacks E050 receipt"))?;
        if guard.rest_bridge_withdrawal_rows(self.xml())?.is_none() {
            return Err(invalid("Bridge withdrawal has no bound operation"));
        }
        final_chord_verify_pitched_sequence_transition(self.xml(), post, false)?;
        let before = notes(self.xml())?;
        let mut expected = current_occurrence_beam_surface(self.xml())?;
        for (start, end, rows) in guard
            .rest_bridge_withdrawal_rows(self.xml())?
            .expect("checked bound operation")
        {
            let positions = before
                .iter()
                .filter(|(_, note)| note.range.start == *start && note.range.end == *end)
                .collect::<Vec<_>>();
            let [(position, _)] = positions.as_slice() else {
                return Err(invalid("Bridge withdrawal lost exact footprint"));
            };
            let levels = expected
                .get_mut(position)
                .ok_or_else(|| invalid("Bridge withdrawal lost beam surface"))?;
            levels.retain(|(level, _)| *level > 2);
            levels.extend(
                rows.iter()
                    .map(|(level, state)| (*level as u8, (*state).to_owned())),
            );
            levels.sort();
        }
        if current_occurrence_beam_surface(post)? != expected {
            return Err(invalid(
                "Bridge withdrawal differs from the complete bound beam operation",
            ));
        }
        Self::from_original_transport(post.to_owned(), self.receipt.clone())
    }
}

#[cfg(test)]
impl CanonicalFinalChordStructuralPlan {
    pub(crate) fn synthetic_adopted_extension_guard(input: &str) -> (String, Self) {
        let (original, receipt, capture) =
            crate::pipeline::actual_beam_owner_capture_tests::execute(input);
        capture.binds_original_receipt(&receipt).unwrap();
        let evidence = final_chord_declassified_evidence_from_xml(&original).unwrap();
        let (mut canonical, _) =
            plan_canonical_final_chord_structure_once(&original, &evidence).unwrap();
        let notation = crate::pipeline::actual_beam_owner_capture_tests::notation_voice_bindings(
            &original,
            canonical
                .source_locators
                .iter()
                .map(|l| ((l.source_event_id, l.attack_id), "2".to_owned())),
        );
        canonical.bind_original_beam_voice_groups(&original, Some(&Ok(capture)), &notation);
        assert!(!canonical.original_beam_voice_groups.is_empty());
        let mut voices = FrozenSerializedVoicePlan {
            authority_bound: true,
            ..Default::default()
        };
        for l in canonical.source_locators.iter() {
            voices
                .source_to_label
                .insert(l.source_event_id, "shared".to_owned());
        }
        let guard = canonical
            .bind_bridge_voice_guard(&original, &voices, &BTreeSet::new())
            .unwrap();
        (original, guard)
    }

    pub(crate) fn synthetic_assert_capture_contract(
        &self,
        input: &str,
        output: &str,
        receipt: &crate::master_state_identity::PreservingTransformationReceipt,
        evidence: &RestBridgeExtensionOccurrenceEvidence,
    ) {
        let before = notes(input).unwrap();
        let bindings = self
            .bridge_voice_guard
            .as_ref()
            .unwrap()
            .occurrence_bindings();
        let selected = evidence
            .course
            .connections
            .iter()
            .map(|connection| {
                let heads = connection
                    .members
                    .iter()
                    .filter(|m| m.head)
                    .map(|m| {
                        let n = &before[&m.position];
                        (n.range.start, n.range.end)
                    })
                    .collect::<Vec<_>>();
                assert!(!connection.resources.is_empty());
                for resource in &connection.resources {
                    let mut missing = bindings.clone();
                    let n = &before[&resource.position];
                    missing.remove(&(n.range.start, n.range.end));
                    assert!(
                        complete_resource_image(
                            &before,
                            &missing,
                            &self.source_locators,
                            &heads.iter().copied().collect(),
                            &connection.component
                        )
                        .is_err()
                    );
                }
                heads
            })
            .collect::<Vec<_>>();
        let mut overlapping = selected.clone();
        overlapping.push(selected[0].clone());
        assert!(
            self.capture_rest_bridge_extension_output(input, output, &overlapping, receipt)
                .is_err()
        );
        let mut ambiguous = self.clone();
        ambiguous
            .original_beam_voice_groups
            .extend(self.original_beam_voice_groups.clone());
        assert!(
            ambiguous
                .capture_rest_bridge_extension_output(input, output, &selected, receipt)
                .unwrap()
                .is_none()
        );
    }
    pub(crate) fn synthetic_assert_shared_resource_retirement(&self) {
        let mut course = self.rest_bridge_extension_course.clone().unwrap();
        let mut dependencies = BTreeMap::<OccurrenceKey, Vec<usize>>::new();
        for (index, connection) in course.connections.iter().enumerate() {
            for resource in &connection.resources {
                dependencies
                    .entry(resource.identity)
                    .or_default()
                    .push(index);
            }
        }
        let (identity, dependent) = dependencies
            .iter()
            .find(|(_, dependent)| dependent.len() > 1)
            .expect("two captured bridges must share a read-only resource");
        let resource = course.connections[dependent[0]]
            .resources
            .iter()
            .find(|m| m.identity == *identity)
            .unwrap();
        let before = course.xml.clone();
        let n = &notes(&before).unwrap()[&resource.position];
        let mut after = before.clone();
        after.replace_range(
            n.range.clone(),
            &before[n.range.clone()]
                .replace("</note>", "<beam number=\"1\">forward hook</beam></note>"),
        );
        let total = course.connections.len();
        let expected = total - dependent.len();
        course.through(&before, &after).unwrap();
        assert_eq!(course.connections.len(), expected);
        course.through(&after, &before).unwrap();
        assert_eq!(course.connections.len(), expected);
    }
    fn synthetic_merged_origin_voices(&self) -> FrozenSerializedVoicePlan {
        let mut plan = FrozenSerializedVoicePlan {
            authority_bound: true,
            semantic_fingerprint_fnv64: 1,
            ..Default::default()
        };
        let original = self
            .written_projection
            .as_ref()
            .expect("original fixture")
            .original_xml
            .as_ref();
        let secondary = notes(original)
            .unwrap()
            .values()
            .find(|n| n.staff == "2")
            .expect("lower fixture")
            .voice
            .clone();
        let primary = self
            .source_locators
            .iter()
            .filter(|l| l.staff == "2" && l.onset == 0 && !l.chord_syntax)
            .map(|l| l.source_event_id)
            .collect::<BTreeSet<_>>();
        for l in self.source_locators.iter() {
            let (channel, label, role) = if l.staff == "1" {
                (3, "3", SerializedVoicePresentationRole::UpperPrimary)
            } else if primary.contains(&l.source_event_id) {
                (1, "7", SerializedVoicePresentationRole::LowerPrimary)
            } else {
                (
                    2,
                    secondary.as_str(),
                    SerializedVoicePresentationRole::LowerSecondary,
                )
            };
            plan.source_to_channel.insert(l.source_event_id, channel);
            plan.source_to_label
                .insert(l.source_event_id, label.to_owned());
            plan.source_to_role.insert(l.source_event_id, role);
            plan.channel_to_label.insert(channel, label.to_owned());
            plan.channel_to_role.insert(channel, role);
            plan.label_to_role.insert(label.to_owned(), role);
        }
        // The shift harness prepends an independent rest-only lane. Its label
        // is registered without allocating any pitched source to it.
        plan.channel_to_label.insert(4, "1".to_owned());
        plan.channel_to_role
            .insert(4, SerializedVoicePresentationRole::UpperAuxiliary);
        plan.label_to_role.insert(
            "1".to_owned(),
            SerializedVoicePresentationRole::UpperAuxiliary,
        );
        plan
    }
    /// Fixture: the same adopted-extension setup with explicitly authored
    /// sealed lanes, cycled over the canonical source order.
    pub(crate) fn synthetic_adopted_extension_guard_with_sealed(input: &str, sealed: &[&str]) -> (String, Self) {
        let (original, canonical) = Self::synthetic_adopted_extension_guard(input);
        let mut voices = FrozenSerializedVoicePlan { authority_bound: true, ..Default::default() };
        for (locator, label) in canonical.source_locators.iter().zip(sealed.iter().cycle()) {
            voices.source_to_label.insert(locator.source_event_id, (*label).to_owned());
        }
        let guard = canonical.bind_bridge_voice_guard(&original, &voices, &BTreeSet::new()).unwrap();
        (original, guard)
    }
    pub(crate) fn synthetic_without_original_groups(mut self) -> Self {
        self.original_beam_voice_groups.clear();
        self
    }
    fn synthetic_source_for(&self, course: &RestBridgeExtensionCourse) -> FinalPdfSourceOccurrenceContinuation {
        let occurrences = course
            .connections
            .iter()
            .flat_map(Connection::all)
            .map(|m| FinalChordTransactionOutputOccurrenceWitness {
                source_event_id: m.identity.0,
                attack_id: m.identity.1,
                fragment_id: m.identity.2,
                fragment_ordinal: 0,
                materialization_kind: FinalChordProjectionMaterializationKind::SurvivingSourceNote,
                measure: m.position.0,
                pitched_ordinal: m.position.1,
            })
            .collect();
        let receipt = FinalChordTransactionLocalOutputAlphaReceipt {
            rest_bridge_extension_course: Some(course.clone()),
            phrase_origin: std::sync::Arc::new(()),
            canonical_semantic_fingerprint_fnv64: 0,
            bound_source_fingerprint_fnv64: 0,
            bound_source_len: course.xml.len(),
            output_topology_fingerprint_fnv64: 0,
            occurrences,
            certified_omitted_fragment_ids: BTreeSet::new(),
            original_beam_voice_groups: self.original_beam_voice_groups.clone(),
        };
        FinalPdfSourceOccurrenceContinuation::from_original_transport(course.xml.clone(), receipt).unwrap()
    }
    /// Withdrawal against a bound late plan, without unwrapping the binding.
    pub(crate) fn synthetic_try_extension_withdrawal(&self, labels: &[&str]) -> io::Result<Self> {
        let course = self.rest_bridge_extension_course.as_ref().expect("adopted extension");
        let source = self.synthetic_source_for(course);
        let mut voices = FrozenSerializedVoicePlan { authority_bound: true, ..Default::default() };
        for (locator, label) in self.source_locators.iter().zip(labels) {
            voices.source_to_label.insert(locator.source_event_id, (*label).to_owned());
        }
        let plan = PrimaryBeamVoicePartitionBoundPlan {
            source_fingerprint_fnv64: fingerprint(source.xml()),
            repaired_voice_plan: voices,
            edits: Vec::new(),
            selected_members: Vec::new(),
            scope_candidates: 1,
            original_group_alignment: Vec::new(),
            historical_phrase_decision_only: false,
        };
        self.bind_rest_bridge_withdrawal(&source, &plan)
    }
    /// Applies the sealed lanes as the actual FinalChord projection would (via
    /// `projected`), then binds the no-late-plan fallback on that document.
    pub(crate) fn synthetic_provisional_fallback_after_sealed_projection(
        &self,
    ) -> (Self, FinalPdfSourceOccurrenceContinuation) {
        let course = self.rest_bridge_extension_course.as_ref().expect("adopted extension");
        let current = notes(&course.xml).unwrap();
        let mut output = course.xml.clone();
        let mut edits = course
            .connections
            .iter()
            .flat_map(Connection::all)
            .map(|m| (current[&m.position].range.clone(), m.sealed_voice.clone()))
            .collect::<Vec<_>>();
        edits.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
        for (range, label) in edits {
            let note = output[range.clone()].to_owned();
            let open = note.find("<voice>").unwrap() + "<voice>".len();
            let close = note.find("</voice>").unwrap();
            output.replace_range(range.start + open..range.start + close, &label);
        }
        let occurrences = course
            .connections
            .iter()
            .flat_map(Connection::all)
            .map(|m| FinalChordTransactionOutputOccurrenceWitness {
                source_event_id: m.identity.0,
                attack_id: m.identity.1,
                fragment_id: m.identity.2,
                fragment_ordinal: 0,
                materialization_kind: FinalChordProjectionMaterializationKind::SurvivingSourceNote,
                measure: m.position.0,
                pitched_ordinal: m.position.1,
            })
            .collect::<Vec<_>>();
        let projected = course.projected(&course.xml, &output, &occurrences).unwrap();
        assert_eq!(projected.connections.len(), course.connections.len(), "projection keeps custody");
        let source = self.synthetic_source_for(&projected);
        (self.bind_provisional_rest_bridge_fallback(&source).unwrap(), source)
    }
    pub(crate) fn synthetic_merged_origin_extension_guard(input: &str) -> (String, Self) {
        let (original, canonical) = Self::synthetic_adopted_extension_guard(input);
        assert!(canonical.original_beam_voice_groups.iter().any(|g| {
            g.bridge_component_occurrences().len()
                > g.closed_bridge_origins(&g.bridge_component_occurrences())[0]
                    .1
                    .len()
        }));
        let voices = canonical.synthetic_merged_origin_voices();
        let guard = canonical
            .bind_bridge_voice_guard(&original, &voices, &BTreeSet::new())
            .unwrap();
        (original, guard)
    }
    pub(crate) fn synthetic_project_extension_for_alignment(
        &self,
        post: &str,
    ) -> (
        FinalPdfSourceOccurrenceContinuation,
        PrimaryBeamVoicePartitionBoundPlan,
    ) {
        let voices = self.synthetic_merged_origin_voices();
        assert!(
            audit_musicxml_voice_label_binding_alpha_for_render_projection(
                post,
                self,
                &voices,
                &BTreeSet::new()
            )
            .unwrap()
            .green()
        );
        let projection = project_canonical_final_chord_render_occurrence_continuation(
            post,
            self,
            &voices,
            &BTreeSet::new(),
        )
        .unwrap();
        let projected_xml = projection.xml().to_owned();
        assert_ne!(
            post, projected_xml,
            "fixture must exercise actual voice projection"
        );
        let source = advance_final_pdf_source_occurrence_after_cursor_commit(
            projection,
            &projected_xml,
            &projected_xml,
            false,
            true,
            FinalPdfSourceOccurrenceValidationContext {
                canonical: self,
                voice_plan: &voices,
                render_omissions: Some(&BTreeSet::new()),
            },
        )
        .unwrap();
        let course = source
            .receipt
            .rest_bridge_extension_course
            .as_ref()
            .unwrap();
        assert!(!course.connections.is_empty());
        assert!(course.connections.iter().all(|c| !c.resources.is_empty()));
        let plan =
            original_beam_voice_alignment::bridge_test_actual_alignment_plan(&voices, &source);
        assert!(plan.scope_candidates() > 0);
        assert!(!plan.edits.is_empty());
        (source, plan)
    }
    pub(crate) fn synthetic_extension_withdrawal(
        &self,
        labels: &[&str],
    ) -> (Self, FinalPdfSourceOccurrenceContinuation) {
        let course = self
            .rest_bridge_extension_course
            .as_ref()
            .expect("adopted extension");
        let occurrences = course
            .connections
            .iter()
            .flat_map(Connection::all)
            .map(|m| FinalChordTransactionOutputOccurrenceWitness {
                source_event_id: m.identity.0,
                attack_id: m.identity.1,
                fragment_id: m.identity.2,
                fragment_ordinal: 0,
                materialization_kind: FinalChordProjectionMaterializationKind::SurvivingSourceNote,
                measure: m.position.0,
                pitched_ordinal: m.position.1,
            })
            .collect();
        let receipt = FinalChordTransactionLocalOutputAlphaReceipt {
            rest_bridge_extension_course: Some(course.clone()),
            phrase_origin: std::sync::Arc::new(()),
            canonical_semantic_fingerprint_fnv64: 0,
            bound_source_fingerprint_fnv64: 0,
            bound_source_len: course.xml.len(),
            output_topology_fingerprint_fnv64: 0,
            occurrences,
            certified_omitted_fragment_ids: BTreeSet::new(),
            original_beam_voice_groups: self.original_beam_voice_groups.clone(),
        };
        let source = FinalPdfSourceOccurrenceContinuation::from_original_transport(
            course.xml.clone(),
            receipt,
        )
        .unwrap();
        let mut voices = FrozenSerializedVoicePlan {
            authority_bound: true,
            ..Default::default()
        };
        for (locator, label) in self.source_locators.iter().zip(labels) {
            voices
                .source_to_label
                .insert(locator.source_event_id, (*label).to_owned());
        }
        let plan = PrimaryBeamVoicePartitionBoundPlan {
            source_fingerprint_fnv64: fingerprint(source.xml()),
            repaired_voice_plan: voices,
            edits: Vec::new(),
            selected_members: Vec::new(),
            scope_candidates: 1,
            original_group_alignment: Vec::new(),
            historical_phrase_decision_only: false,
        };
        (
            self.bind_rest_bridge_withdrawal(&source, &plan).unwrap(),
            source,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn course(chord: bool) -> RestBridgeExtensionCourse {
        let attached = if chord {
            "<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>24</duration><voice>same</voice><staff>2</staff></note>"
        } else {
            ""
        };
        let synthetic_measure = "79";
        let xml = format!(
            "<score-partwise><part id=\"P\"><measure number=\"{synthetic_measure}\"><attributes><divisions>96</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>same</voice><staff>2</staff><beam number=\"1\">begin</beam></note>{attached}<note><pitch><step>G</step><octave>4</octave></pitch><duration>24</duration><voice>same</voice><staff>2</staff><beam number=\"1\">end</beam></note></measure></part></score-partwise>"
        );
        let current = notes(&xml).unwrap();
        let members = current
            .iter()
            .map(|(position, note)| Member {
                identity: (position.1 + 11, position.1 + 31, position.1 + 51),
                position: *position,
                original: position.1 == 0,
                anchor: note.chord_member.then_some((11, 31, 51)),
                head: !note.chord_member,
                onset: note.onset,
                duration: note.duration,
                pitch: note.pitch_midi,
                staff: note.staff.clone(),
                voice: note.voice.clone(),
                sealed_voice: note.voice.clone(),
                chord: note.chord_member,
                tie_start: note.tie_start,
                tie_stop: note.tie_stop,
                before: Vec::new(),
                written: beams(&xml, note).unwrap(),
            })
            .collect();
        RestBridgeExtensionCourse {
            project: "synthetic".to_owned(),
            branch: "main",
            xml,
            source: std::sync::Arc::from(Vec::<FinalChordCanonicalSourceLocator>::new()),
            connections: vec![Connection {
                members,
                resources: Vec::new(),
                component: BTreeSet::new(),
                origin: (0, 0),
                provisional: false,
            }],
            provisional_origins: BTreeSet::new(),
        }
    }
    fn witnesses(
        course: &RestBridgeExtensionCourse,
    ) -> Vec<FinalChordTransactionOutputOccurrenceWitness> {
        course
            .connections
            .iter()
            .flat_map(Connection::all)
            .map(|m| FinalChordTransactionOutputOccurrenceWitness {
                source_event_id: m.identity.0,
                attack_id: m.identity.1,
                fragment_id: m.identity.2,
                measure: m.position.0,
                pitched_ordinal: m.position.1,
                fragment_ordinal: 0,
                materialization_kind: FinalChordProjectionMaterializationKind::SurvivingSourceNote,
            })
            .collect()
    }
    #[test]
    fn final_chord_handover_requires_unique_complete_actual_occurrences() {
        let c = course(true);
        let rows = witnesses(&c);
        assert_eq!(
            c.projected(&c.xml, &c.xml, &rows)
                .unwrap()
                .connections
                .len(),
            1
        );
        let mut duplicate = rows.clone();
        duplicate.push(rows[0]);
        assert!(
            c.projected(&c.xml, &c.xml, &duplicate)
                .unwrap()
                .connections
                .is_empty()
        );
        assert!(
            c.projected(&c.xml, &c.xml, &rows[..rows.len() - 1])
                .unwrap()
                .connections
                .is_empty()
        );
        let mut generated = rows.clone();
        let mut extra = rows[0];
        extra.fragment_id += 1000;
        generated.push(extra);
        assert!(
            c.projected(&c.xml, &c.xml, &generated)
                .unwrap()
                .connections
                .is_empty()
        );
        let mut foreign = rows;
        foreign[0].source_event_id += 1000;
        assert!(
            c.projected(&c.xml, &c.xml, &foreign)
                .unwrap()
                .connections
                .is_empty()
        );
        assert!(c.projected("stale", &c.xml, &foreign).is_err());
    }
    #[test]
    fn beam_and_attachment_drift_retire_whole_connection() {
        for chord in [false, true] {
            let c = course(chord);
            let mut carried = c.clone();
            let drift = c.xml.replacen(">begin</beam>", ">continue</beam>", 1);
            carried.through(&c.xml, &drift).unwrap();
            assert!(carried.connections.is_empty());
            carried.through(&drift, &c.xml).unwrap();
            assert!(carried.connections.is_empty());
            let mut carried = c.clone();
            let drift = c.xml.replacen("<note>", "<note><chord/>", 1);
            carried.through(&c.xml, &drift).unwrap();
            assert!(carried.connections.is_empty());
        }
    }
    #[test]
    fn complete_chord_resource_cannot_be_hidden_or_reassigned() {
        let c = course(true);
        let current = notes(&c.xml).unwrap();
        assert!(attachments_match(&c.connections[0], &current));
        let mut incomplete = c.connections[0].clone();
        incomplete.members.retain(|m| !m.chord);
        assert!(!attachments_match(&incomplete, &current));
        let mut wrong = c.connections[0].clone();
        wrong.members[1].anchor = Some(wrong.members[2].identity);
        assert!(!attachments_match(&wrong, &current));
    }
    #[test]
    fn harmless_serialization_and_enharmonic_spelling_preserve_custody() {
        let mut c = course(true);
        let old = c.xml.clone();
        let post = old
            .replace("<step>C</step>", "<step>B</step><alter>1</alter>")
            .replacen("<octave>4</octave>", "<octave>3</octave>", 1);
        c.through(&old, &post).unwrap();
        assert_eq!(c.connections.len(), 1);
        let old = c.xml.clone();
        let post = old.replace("<voice>same</voice>", "<voice>same</voice>\n");
        c.through(&old, &post).unwrap();
        assert_eq!(c.connections.len(), 1);
    }
    #[test]
    fn real_final_chord_projection_carries_existing_occurrence_course() {
        let synthetic_measure = "83";
        let input = crate::pipeline::actual_beam_owner_capture_tests::input()
            .replace("probe", synthetic_measure);
        let (mut canonical, voices, source, _) =
            original_beam_voice_alignment::original_beam_alignment_test_fixture(&input);
        let current = notes(source.xml()).unwrap();
        let members = current
            .iter()
            .map(|(position, note)| {
                let row = source
                    .receipt
                    .occurrences
                    .iter()
                    .find(|r| (r.measure, r.pitched_ordinal) == *position)
                    .unwrap();
                Member {
                    identity: (row.source_event_id, row.attack_id, row.fragment_id),
                    position: *position,
                    original: true,
                    anchor: None,
                    head: true,
                    onset: note.onset,
                    duration: note.duration,
                    pitch: note.pitch_midi,
                    staff: note.staff.clone(),
                    voice: note.voice.clone(),
                    sealed_voice: note.voice.clone(),
                    chord: note.chord_member,
                    tie_start: note.tie_start,
                    tie_stop: note.tie_stop,
                    before: beams(source.xml(), note).unwrap(),
                    written: beams(source.xml(), note).unwrap(),
                }
            })
            .collect();
        canonical.rest_bridge_extension_course = Some(RestBridgeExtensionCourse {
            project: "synthetic".to_owned(),
            branch: "main",
            xml: source.xml().to_owned(),
            source: canonical.source_locators.clone(),
            connections: vec![Connection {
                members,
                resources: Vec::new(),
                component: BTreeSet::new(),
                origin: (0, 0),
                provisional: false,
            }],
            provisional_origins: BTreeSet::new(),
        });
        let projected = project_canonical_final_chord_render_occurrence_continuation(
            source.xml(),
            &canonical,
            &voices,
            &BTreeSet::new(),
        )
        .unwrap();
        let carried = projected
            .projected
            .receipt
            .rest_bridge_extension_course
            .as_ref()
            .unwrap();
        assert_eq!(carried.current_xml(), projected.xml());
        assert_eq!(carried.connections.len(), 1);
        assert!(std::sync::Arc::ptr_eq(
            &carried.source,
            &canonical.source_locators
        ));
    }
    #[test]
    fn absent_render_is_a_noop_only_without_adopted_extension_custody() {
        let mut canonical = CanonicalFinalChordStructuralPlan::default();
        canonical
            .continue_rest_bridge_extensions_at_state("original branch document", None)
            .unwrap();
        let c = course(false);
        let pre = c.xml.clone();
        canonical.rest_bridge_extension_course = Some(c);
        assert!(
            canonical
                .continue_rest_bridge_extensions_at_state(&pre, None)
                .is_err()
        );
        canonical
            .continue_rest_bridge_extensions_at_state(&pre, Some(&pre))
            .unwrap();
    }
    #[test]
    fn shared_read_only_beam_resource_retires_only_dependent_connections() {
        let base = course(false);
        let measure = base
            .xml
            .split("<part id=\"P\">")
            .nth(1)
            .unwrap()
            .split("</part>")
            .next()
            .unwrap();
        let mut measures = String::new();
        for synthetic_measure in [101, 102, 103, 104] {
            measures.push_str(
                &measure.replace("number=\"79\"", &format!("number=\"{synthetic_measure}\"")),
            );
        }
        let xml = format!("<score-partwise><part id=\"P\">{measures}</part></score-partwise>");
        let current = notes(&xml).unwrap();
        let make = |position: Position| {
            let n = &current[&position];
            Member {
                identity: (
                    position.0 * 10 + position.1,
                    position.0 * 10 + position.1,
                    position.0 * 10 + position.1,
                ),
                position,
                original: true,
                anchor: None,
                head: true,
                onset: n.onset,
                duration: n.duration,
                pitch: n.pitch_midi,
                staff: n.staff.clone(),
                voice: n.voice.clone(),
                sealed_voice: n.voice.clone(),
                chord: false,
                tie_start: false,
                tie_stop: false,
                before: beams(&xml, n).unwrap(),
                written: beams(&xml, n).unwrap(),
            }
        };
        let mut resource = make((104, 0));
        resource.head = false;
        resource.original = false;
        let connections = [101, 102, 103]
            .into_iter()
            .enumerate()
            .map(|(index, m)| Connection {
                members: vec![make((m, 0)), make((m, 1))],
                resources: if index < 2 {
                    vec![resource.clone()]
                } else {
                    Vec::new()
                },
                component: BTreeSet::new(),
                origin: (index, 0),
                provisional: false,
            })
            .collect();
        let mut c = RestBridgeExtensionCourse {
            project: "synthetic".to_owned(),
            branch: "main",
            xml: xml.clone(),
            source: std::sync::Arc::from(Vec::<FinalChordCanonicalSourceLocator>::new()),
            connections,
            provisional_origins: BTreeSet::new(),
        };
        let n = &current[&(104, 0)];
        let mut post = xml.clone();
        post.replace_range(
            n.range.clone(),
            &xml[n.range.clone()].replace(">begin</beam>", ">continue</beam>"),
        );
        c.through(&xml, &post).unwrap();
        assert_eq!(c.connections.len(), 1);
        assert_eq!(c.connections[0].origin, (2, 0));
        c.through(&post, &xml).unwrap();
        assert_eq!(
            c.connections.len(),
            1,
            "retired dependencies may not resurrect"
        );
    }
}
