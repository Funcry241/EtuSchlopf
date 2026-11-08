// Same-epoch input of the existing register/clef owners. This carries the
// original material identities; positions only locate already bound fragments.
#[derive(Debug, Clone)]
pub(crate) struct JointReadingOwnerInput {
    source: std::sync::Arc<str>,
    heads: Vec<JointReadingOwnerHead>,
    groups: Vec<JointReadingOwnerGroup>,
    tonal: TonalSpellingDecision,
    unknown_groups: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum JointReadingGroupKind {
    PrimaryBeam,
    Chord,
    TiedAttack,
}

#[derive(Debug, Clone)]
pub(crate) struct JointReadingOwnerGroup {
    pub(crate) kind: JointReadingGroupKind,
    pub(crate) members: Vec<usize>,
}

#[derive(Debug, Clone)]
pub(crate) struct JointReadingOwnerHead {
    source_event_id: MaterialSourceEventId,
    attack_id: SoundingAttackId,
    pub(crate) measure: usize,
    pub(crate) onset: i64,
    pub(crate) duration: i64,
    pub(crate) staff: String,
    pub(crate) visible: bool,
    pub(crate) diatonic: i32,
    pub(crate) continuation: bool,
    source_span: std::ops::Range<usize>,
}

impl JointReadingOwnerInput {
    fn from_carried_material(candidate: &SolverCandidate) -> anyhow::Result<Self> {
        anyhow::ensure!(candidate.material_lineage.authority_bound,
            "Joint reading input has no carried material authority");
        // This observation precedes the one-shot Master epoch and Freeze.
        // It requires the existing material carrier and actual E053 custody,
        // not a frozen voice-label plan which does not exist at this boundary.
        let source = candidate.layout_xml();
        let document = roxmltree::Document::parse(source)?;
        // The current material carrier has no Part discriminator. It can bind
        // exactly its one physical piano part; it does not rank independent parts.
        anyhow::ensure!(document.root_element().children()
            .filter(|node| node.has_tag_name("part")).count() == 1,
            "Joint reading material carrier has no multi-part discriminator");
        let notes = parse_atomic_notes(source);
        // Printed measure labels are diagnostics. Only this same-document
        // structural traversal supplies the model's measure ordinals; it does
        // not manufacture a material source or attack identity.
        let mut measure_ordinals = BTreeMap::new();
        for (index, measure) in document.root_element().children()
            .filter(|node| node.has_tag_name("part"))
            .flat_map(|part| part.children().filter(|node| node.has_tag_name("measure")))
            .enumerate()
        {
            let ordinal = index.checked_add(1)
                .ok_or_else(|| anyhow!("Joint reading measure ordinal overflow"))?;
            for note in measure.children().filter(|node| node.has_tag_name("note")) {
                anyhow::ensure!(measure_ordinals.insert(note.range().start, ordinal).is_none(),
                    "Joint reading note belongs to more than one measure");
            }
        }
        let observed = material_fragments_from_xml(source)?;
        anyhow::ensure!(notes.len() == observed.len(), "Joint reading source topology is incomplete");
        let original = candidate.master_state.original_respelling_input(source)
            .joint_source_occurrences(source)?;
        let mut by_origin = original.into_iter().map(|(span, source)|
            ((span.start, span.end), source)).collect::<BTreeMap<_, _>>();
        let retained = candidate.material_lineage.attacks.iter().filter(|attack| attack.exists)
            .map(|attack| (attack.source_event_id.0, (attack.source_event_id, attack.attack_id))).collect::<BTreeMap<_, _>>();
        anyhow::ensure!(retained.len() == candidate.material_lineage.attacks.iter()
            .filter(|attack| attack.exists).count(), "Joint reading retained source binding is ambiguous");
        anyhow::ensure!(retained.values().map(|(_, attack)| *attack).collect::<BTreeSet<_>>().len() == retained.len(),
            "Joint reading retained attack binding is ambiguous");
        let tonal = derive_tonal_spelling_decision(source, &notes);
        let spelling = tonal.selected_profile
            .filter(|_| tonal.scorewide_attempted || tonal.final_section.is_some())
            .map(|profile| {
            functional_respelling_plans_for_tonal_frame_from_measure(source,
                profile.fifths, profile.tonic, profile.minor,
                tonal.final_section.as_ref().map(|section| section.boundary_measure))
        }).unwrap_or_default();
        let by_spelling = spelling.iter().map(|plan| (plan.note_start, plan)).collect::<BTreeMap<_, _>>();
        anyhow::ensure!(by_spelling.len() == spelling.len(), "Joint reading spelling plan is ambiguous");
        let mut heads = Vec::new();
        let mut by_start = BTreeMap::new();
        for (note, fragment) in notes.iter().zip(&observed) {
            let Some(pitch) = &note.pitch else { continue; };
            anyhow::ensure!(!fragment.is_grace && fragment.duration > 0,
                "Joint reading source has unsupported zero-time pitched fragments");
            let original_source = by_origin.remove(&(note.note_start, note.note_end))
                .ok_or_else(|| anyhow!("Joint reading pitched container has no received source occurrence"))?;
            let (source_event_id, attack_id) = *retained.get(&original_source)
                .ok_or_else(|| anyhow!("Joint reading original source has no retained attack identity"))?;
            let (step, octave) = if let Some(plan) = by_spelling.get(&note.note_start) {
                anyhow::ensure!(plan.note_end == note.note_end,
                    "Joint reading spelling plan belongs to another note container");
                (plan.step, plan.octave)
            } else { (pitch.step.as_str(), pitch.octave) };
            let step = match step { "C" => 0, "D" => 1, "E" => 2, "F" => 3,
                "G" => 4, "A" => 5, "B" => 6, _ => anyhow::bail!("Unknown diatonic spelling") };
            let diatonic = octave.checked_mul(7).and_then(|v| v.checked_add(step))
                .ok_or_else(|| anyhow!("Joint reading diatonic position overflow"))?;
            anyhow::ensure!(by_start.insert(note.note_start, heads.len()).is_none(),
                "Joint reading source note container is ambiguous");
            heads.push(JointReadingOwnerHead { source_event_id, attack_id,
                measure: *measure_ordinals.get(&note.note_start)
                    .ok_or_else(|| anyhow!("Joint reading note has no structural measure"))?,
                onset: fragment.onset, duration: fragment.duration,
                staff: fragment.staff.clone(), visible: fragment.visible, diatonic,
                continuation: fragment.tie_stop, source_span: note.note_start..note.note_end });
        }
        anyhow::ensure!(by_origin.is_empty(), "Joint reading original pitched occurrence was not consumed");
        Ok(Self { source: std::sync::Arc::from(source), heads, groups: Vec::new(), tonal,
            unknown_groups: BTreeMap::new() })
    }

    fn from_candidate(candidate: &SolverCandidate) -> anyhow::Result<Self> {
        let mut input = Self::from_carried_material(candidate)?;
        let capture = candidate.master_state.actual_beam_capture()
            .ok_or_else(|| anyhow!("Joint reading actual beam producer has not executed"))?
            .as_ref().map_err(|reason| anyhow!("Joint reading beam custody unavailable: {reason}"))?;
        input.bind_original_groups(capture)?;
        Ok(input)
    }

    pub(crate) fn bind_original_groups(&mut self, capture: &ActualCanonicalBeamCapture) -> anyhow::Result<()> {
        let source = self.source.as_ref();
        let heads = &self.heads;
        let mut unknown_groups = BTreeMap::new();
        let by_start = heads.iter().enumerate().map(|(index, head)| (head.source_span.start, index)).collect::<BTreeMap<_, _>>();
        let occurrences = capture.occurrences(source)?;
        let locate = |range: &std::ops::Range<usize>| -> anyhow::Result<usize> {
            let index = by_start.get(&range.start).copied()
                .ok_or_else(|| anyhow!("Actual beam occurrence has no bound pitched source"))?;
            anyhow::ensure!(heads[index].source_span == *range,
                "Actual beam occurrence is not the bound complete note container");
            Ok(index)
        };
        let mut primary = BTreeMap::<(usize, usize), Vec<usize>>::new();
        for (range, levels) in occurrences {
            let index = locate(range)?;
            let mut observed_levels = BTreeSet::new();
            for (level, group, _) in levels {
                anyhow::ensure!(observed_levels.insert(*level),
                    "Joint reading actual beam occurrence has duplicate levels");
                if *level == 1 && let Some(group) = group {
                    primary.entry(*group).or_default().push(index);
                }
            }
        }
        let mut groups = primary.into_values().map(|members| JointReadingOwnerGroup {
            kind: JointReadingGroupKind::PrimaryBeam, members,
        }).collect::<Vec<_>>();
        let attachments = capture.chord_attachments(source)?;
        let rest_clock = attachments.iter().any(|row| row.original_rest_host.is_some())
            .then(|| c3_adapter::observe_clock(source)).transpose()
            .map_err(|error| anyhow!("Original chord rest-host clock: {error:?}"))?;
        let mut rest_cohorts = BTreeMap::<(usize, usize), Vec<usize>>::new();
        for attachment in attachments {
            let member = locate(&attachment.member)?;
            let Some(head_range) = attachment.original_head.as_ref() else {
                // Only the producer's explicit absence is local. A lost or
                // ambiguous continuation remains an error for the whole input.
                anyhow::ensure!(attachment.original_head_absent,
                    "Joint reading chord lost its original physical head continuation");
                if let Some(host) = &attachment.original_rest_host {
                    let document = roxmltree::Document::parse(&source[host.clone()])?;
                    let root = document.root_element();
                    let child = |name| root.children().find(|node| node.has_tag_name(name));
                    anyhow::ensure!(child("rest").is_some() && child("pitch").is_none()
                        && child("chord").is_none() && child("grace").is_none()
                        && child("duration").and_then(|node| node.text())
                            .and_then(|text| text.trim().parse::<i64>().ok()).is_some_and(|duration| duration > 0),
                        "Original chord rest host is not a positive cursor carrier");
                    let clock = rest_clock.as_ref().expect("rest-host clock prepared above");
                    let at = clock.times.get(&(host.start, false))
                        .ok_or_else(|| anyhow!("Original rest host has no carried clock"))?;
                    anyhow::ensure!(clock.times.get(&(attachment.member.start, false)) == Some(at),
                        "Original chord rest-host support changed its member onset");
                    let host_staff = child("staff").and_then(|node| node.text()).unwrap_or("1").trim();
                    let member_document = roxmltree::Document::parse(&source[attachment.member.clone()])?;
                    let member_voice = member_document.root_element().children()
                        .find(|node| node.has_tag_name("voice")).and_then(|node| node.text()).map(str::trim);
                    anyhow::ensure!(child("voice").and_then(|node| node.text()).map(str::trim) == member_voice,
                        "Original chord rest-host voice support changed");
                    if host_staff != heads[member].staff {
                        for staff in [host_staff, heads[member].staff.as_str()] {
                            unknown_groups.insert(staff.to_owned(),
                                "original rest-host chord dependency crosses staff domains".to_string());
                        }
                    } else {
                        rest_cohorts.entry((host.start, host.end)).or_default().push(member);
                    }
                    continue;
                }
                unknown_groups.insert(heads[member].staff.clone(),
                    "original chord producer observed no pitched physical head".to_string());
                continue;
            };
            let head = locate(head_range)?;
            if let Some(index) = attachment.head {
                let occurrence = occurrences.get(index)
                    .ok_or_else(|| anyhow!("Joint reading chord attachment lost its producer beam head"))?;
                anyhow::ensure!(occurrence.0 == *head_range && attachment.unknown.is_none(),
                    "Joint reading physical chord head disagrees with its beam occurrence");
            } else {
                // A long/non-beamed head can carry a physical chord relation
                // without acquiring a beam decision. A written primary beam
                // without a captured owner occurrence remains unsupported.
                let head_document = roxmltree::Document::parse(&source[head_range.clone()])?;
                anyhow::ensure!(!head_document.root_element().children().any(|node|
                    node.has_tag_name("beam") && node.attribute("number").unwrap_or("1") == "1"),
                    "Joint reading physical chord head has an unbound primary beam");
            }
            groups.push(JointReadingOwnerGroup {
                kind: JointReadingGroupKind::Chord,
                members: vec![head, member],
            });
        }
        groups.extend(rest_cohorts.into_values().filter(|members| members.len() > 1)
            .map(|members| JointReadingOwnerGroup { kind: JointReadingGroupKind::Chord, members }));
        // A tie is carried by its existing attack identity, not matched by
        // spelling, pitch, voice, adjacency or a final XML traversal ordinal.
        let mut ties = BTreeMap::<SoundingAttackId, Vec<usize>>::new();
        for (index, head) in heads.iter().enumerate() {
            ties.entry(head.attack_id).or_default().push(index);
        }
        groups.extend(ties.into_values().filter(|members| members.len() > 1)
            .map(|members| JointReadingOwnerGroup { kind: JointReadingGroupKind::TiedAttack, members }));
        self.groups = groups;
        self.unknown_groups = unknown_groups;
        Ok(())
    }

    pub(crate) fn bound_source(&self) -> &str { &self.source }
    pub(crate) fn binds(&self, source: &str) -> bool { self.source.as_ref() == source }
    pub(crate) fn heads(&self) -> &[JointReadingOwnerHead] { &self.heads }
    pub(crate) fn groups(&self) -> &[JointReadingOwnerGroup] { &self.groups }
}

impl JointReadingOwnerGroup {
    fn c3_kind(&self) -> c3_adapter::AdapterGroupKind {
        match self.kind {
            JointReadingGroupKind::PrimaryBeam => c3_adapter::AdapterGroupKind::PrimaryBeam,
            JointReadingGroupKind::Chord => c3_adapter::AdapterGroupKind::Chord,
            JointReadingGroupKind::TiedAttack => c3_adapter::AdapterGroupKind::TiedAttack,
        }
    }
}

impl JointReadingOwnerInput {
    /// Read-only C3 proposal over exactly this bound epoch. Locators are the
    /// carried note containers; no identity is derived from the projection.
    pub(crate) fn c3_proposal(
        &self,
        capabilities: &c3_adapter::Capabilities,
        expansion_limit: Option<usize>,
    ) -> Result<c3_adapter::C3Proposal, c3_adapter::AdapterError> {
        let heads = self.heads.iter().map(|head| c3_adapter::AdapterHead {
            span: head.source_span.clone(),
            measure: head.measure,
            onset: head.onset,
            staff: head.staff.clone(),
            visible: head.visible,
            diatonic: head.diatonic,
            continuation: head.continuation,
        }).collect::<Vec<_>>();
        let groups = self.groups.iter().map(|group| c3_adapter::AdapterGroup {
            kind: group.c3_kind(),
            members: group.members.clone(),
        }).collect::<Vec<_>>();
        let mut proposal = c3_adapter::propose(&self.source, &heads, &groups, capabilities, expansion_limit)?;
        for (staff, reason) in &self.unknown_groups {
            proposal.staves.insert(staff.clone(), c3_adapter::StaffVerdict::Unknown { reason: reason.clone() });
        }
        Ok(proposal)
    }
}

/// Negotiate actual existing-owner capabilities before the finite search.
/// A nominal state is never authority for an otherwise unwriteable boundary.
fn observe_joint_capabilities(source: &str) -> anyhow::Result<c3_adapter::Capabilities> {
    let clock = c3_adapter::observe_clock(source).map_err(|error| anyhow!("C3 writer clock: {error:?}"))?;
    let octave = observe_joint_octave_anchors(source)?;
    let clef = crate::structural_write_authority::RustigClefSurfaceDecision::observe_joint_boundaries(source);
    let mut capabilities = c3_adapter::Capabilities::new(source);
    for (staff, (start, incumbent)) in &clock.entries {
        // These are the actual physical domains of the existing owners. E024
        // writes the lower clef; the octave creator supports all five displays.
        let states = match staff.as_str() { "1" => 0..5, "2" => 0..10, _ => continue };
        if staff == "2" && clef.is_none() { continue; }
        let writable = states.map(c3_course::State::new).collect::<Result<Vec<_>, _>>()
            .map_err(|error| anyhow!("C3 materialization states: {error:?}"))?;
        let mut boundaries = BTreeMap::<c3_adapter::Q, c3_adapter::BoundaryCapability>::new();
        for anchor in octave.iter().filter(|anchor| anchor.staff == *staff) {
            let after = anchor.kind == RegisternotationsGrenzart::Stop;
            let time = *clock.times.get(&(anchor.position, after))
                .ok_or_else(|| anyhow!("C3 octave writer anchor has no exact original clock"))?;
            let boundary = boundaries.entry(time).or_default();
            match anchor.kind { RegisternotationsGrenzart::Start => boundary.octave_start = true,
                RegisternotationsGrenzart::Stop => boundary.octave_stop = true,
                RegisternotationsGrenzart::Continue => anyhow::bail!("C3 capability unexpectedly contains a reminder") }
        }
        let mut fixed_clefs = BTreeMap::new();
        if staff == "2" && let Some(clef) = &clef {
            for position in &clef.insertions {
                let time = *clock.times.get(&(*position, false))
                    .ok_or_else(|| anyhow!("C3 clef writer anchor has no exact original clock"))?;
                boundaries.entry(time).or_default().clef = true;
            }
            for (position, treble) in &clef.fixed {
                let time = *clock.times.get(&(*position, false))
                    .ok_or_else(|| anyhow!("C3 fixed clef has no exact original clock"))?;
                if time >= *start { fixed_clefs.insert(time, u8::from(!treble)); }
            }
        }
        let at_start = boundaries.get(start).copied().unwrap_or_default();
        let entry = writable.iter().copied().filter(|state|
            state.index() / 5 == incumbent.index() / 5
                && (state.index() % 5 == 0 || at_start.octave_start)).collect();
        let exit_octave_stop = boundaries.get(&clock.end).is_some_and(|boundary| boundary.octave_stop);
        capabilities.staves.insert(staff.clone(), c3_adapter::StaffCapability {
            evidence: "original E024 normalizer and octave creator's visible begin/end anchors".to_owned(),
            writable, boundaries, entry, fixed_clefs, exit_octave_stop,
            clef_change_inside_primary_beam: true, clef_change_inside_tie: false,
            octave_change_inside_primary_beam: true, octave_change_inside_tie: false,
            boundary_in_silence: true, boundary_at_hidden_head_only: true,
        });
    }
    Ok(capabilities)
}

/// Called once at the actual early Master→Render fork, before its first owner.
fn original_joint_render_material(candidate: &SolverCandidate) -> Result<JointReadingOwnerInput, String> {
    JointReadingOwnerInput::from_carried_material(candidate).map_err(|error| error.to_string())
}
