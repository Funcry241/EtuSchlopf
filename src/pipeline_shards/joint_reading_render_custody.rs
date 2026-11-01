// Original Render emitter maps carry retained material identities. They never
// mint a SourceEvent or Attack ID, infer one from pitch, or grant a write.
#[derive(Debug, Clone)]
pub(crate) struct JointRestForwardNoteCapture {
    execution: crate::master_state_identity::MintedIdentity,
    before: std::sync::Arc<str>,
    after: std::sync::Arc<str>,
    emissions: Vec<(std::ops::Range<usize>, std::ops::Range<usize>)>,
}
impl JointRestForwardNoteCapture {
    fn seal(_owner: &RenderRow55BlockMint, receipt: &BlockTransformationReceipt,
        before: &str, after: &str,
        emissions: Vec<(std::ops::Range<usize>, std::ops::Range<usize>)>) -> Self {
        Self { execution: *receipt.ssc_bound_origin().0, before: std::sync::Arc::from(before),
            after: std::sync::Arc::from(after), emissions }
    }
    pub(crate) fn binds(&self, receipt: &BlockTransformationReceipt) -> bool {
        self.execution == *receipt.ssc_bound_origin().0
    }
}

impl JointReadingOwnerInput {
    fn continue_bound_ranges(&self, after: &str,
        mut carry: impl FnMut(&JointReadingOwnerHead) -> anyhow::Result<Option<std::ops::Range<usize>>>) -> anyhow::Result<Self> {
        let notes = parse_atomic_notes(after);
        let fragments = material_fragments_from_xml(after)?;
        anyhow::ensure!(notes.len() == fragments.len(), "Render material topology is incomplete");
        let mut by_range = notes.iter().zip(&fragments).filter(|(note, _)| note.pitch.is_some())
            .map(|(note, fragment)| ((note.note_start, note.note_end), (note, fragment)))
            .collect::<BTreeMap<_, _>>();
        let old_notes = parse_atomic_notes(&self.source).into_iter()
            .map(|note| ((note.note_start, note.note_end), note)).collect::<BTreeMap<_, _>>();
        let document = roxmltree::Document::parse(after)?;
        let mut ordinals = BTreeMap::new();
        for (index, measure) in document.root_element().children().filter(|node| node.has_tag_name("part"))
            .flat_map(|part| part.children().filter(|node| node.has_tag_name("measure"))).enumerate() {
            for note in measure.children().filter(|node| node.has_tag_name("note")) {
                ordinals.insert(note.range().start, index + 1);
            }
        }
        let tonal = derive_tonal_spelling_decision(after, &notes);
        let spelling = tonal.selected_profile.filter(|_| tonal.scorewide_attempted || tonal.final_section.is_some())
            .map(|profile| functional_respelling_plans_for_tonal_frame_from_measure(after,
                profile.fifths, profile.tonic, profile.minor,
                tonal.final_section.as_ref().map(|section| section.boundary_measure)))
            .unwrap_or_default();
        let spelling = spelling.iter().map(|plan| (plan.note_start, plan)).collect::<BTreeMap<_, _>>();
        let mut heads = Vec::with_capacity(self.heads.len());
        for head in &self.heads {
            let Some(range) = carry(head)? else { continue; };
            let (note, fragment) = by_range.remove(&(range.start, range.end))
                .ok_or_else(|| anyhow!("Original Render head is not exactly one surviving pitched container"))?;
            let old = old_notes.get(&(head.source_span.start, head.source_span.end))
                .and_then(|note| note.pitch.as_ref()).ok_or_else(|| anyhow!("Original Render head lost its source binding"))?;
            let pitch = note.pitch.as_ref().ok_or_else(|| anyhow!("Original Render head was restified"))?;
            anyhow::ensure!(old.midi == pitch.midi, "Original Render head changed sounding pitch");
            let (step, octave) = spelling.get(&note.note_start).map_or((pitch.step.as_str(), pitch.octave), |plan| (plan.step, plan.octave));
            let offset = match step { "C"=>0,"D"=>1,"E"=>2,"F"=>3,"G"=>4,"A"=>5,"B"=>6,_=>anyhow::bail!("Unknown Render spelling") };
            let mut next = head.clone();
            next.source_span = range; next.measure = *ordinals.get(&note.note_start).ok_or_else(|| anyhow!("Render head has no structural measure"))?;
            next.onset = fragment.onset; next.duration = fragment.duration; next.staff = fragment.staff.clone();
            next.visible = fragment.visible; next.continuation = fragment.tie_stop;
            next.diatonic = octave.checked_mul(7).and_then(|value| value.checked_add(offset)).ok_or_else(|| anyhow!("Render spelling overflow"))?;
            heads.push(next);
        }
        anyhow::ensure!(by_range.is_empty(), "A Render pitched emission has no carried source identity");
        Ok(Self { source: std::sync::Arc::from(after), heads, groups: Vec::new(), tonal, unknown_groups: BTreeMap::new() })
    }

    pub(crate) fn continue_render_exact(&self, receipt: &ExactTransformationReceipt,
        fragment: Option<(usize, usize)>, after: &str) -> anyhow::Result<Self> {
        let (start, end) = fragment.unwrap_or((0, self.source.len()));
        let original = self.source.get(start..end).ok_or_else(|| anyhow!("Render receipt fragment is out of bounds"))?;
        let readback = receipt.operation_readback(original)?;
        let expected = [&self.source[..start], readback.post_document(), &self.source[end..]].concat();
        anyhow::ensure!(expected == after, "Render exact receipt does not bind its complete transition");
        self.continue_bound_ranges(after, |head| {
            let range = head.source_span.clone();
            if range.end <= start { return Ok(Some(range)); }
            if range.start >= end {
                let mapped = range.start - (end-start) + readback.post_document().len()
                    ..range.end - (end-start) + readback.post_document().len();
                return Ok(Some(mapped));
            }
            anyhow::ensure!(range.start >= start && range.end <= end, "Render exact fragment crosses an original head");
            let local = range.start-start..range.end-start;
            // Only the actual hidden-inner-chord pruning owner carries this
            // explicit omission. No visible source disappears by failed lookup.
            if receipt.minted_by::<RenderRow22ExactMint>() && !head.visible
                && crate::master_state_identity::ssc_exact_edit_spans(receipt).iter()
                    .any(|(a,b,text)| *a == local.start && *b == local.end && text.is_empty()) {
                anyhow::ensure!(self.source[range].contains("<chord"), "Render prune omission is not its original chord member");
                return Ok(None);
            }
            let mapped = readback.carry_range(local)?;
            Ok(Some(start+mapped.start..start+mapped.end))
        })
    }

    pub(crate) fn continue_render_fragment_exterior(&self, start: usize, end: usize,
        post: &str, after: &str) -> anyhow::Result<Self> {
        let original = self.source.get(start..end).ok_or_else(|| anyhow!("Render block fragment is outside its original source"))?;
        let expected = [&self.source[..start], post, &self.source[end..]].concat();
        anyhow::ensure!(expected == after, "Render fragment changed its exterior");
        self.continue_bound_ranges(after, |head| {
            let range = head.source_span.clone();
            if range.end <= start { return Ok(Some(range)); }
            if range.start >= end {
                return Ok(Some(range.start - original.len() + post.len()..range.end - original.len() + post.len()));
            }
            anyhow::bail!("Render block fragment intersects a carried pitched head")
        })
    }

    pub(crate) fn continue_render_recorded(&self,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport,
        after: &str) -> anyhow::Result<Self> {
        let ranges = transport.carry(&self.source, after, self.heads.iter().map(|head| head.source_span.clone()).collect())?;
        let mut ranges = ranges.into_iter();
        self.continue_bound_ranges(after, |_| Ok(Some(ranges.next().ok_or_else(|| anyhow!("Incomplete Render journal"))?)))
    }

    pub(crate) fn continue_render_rest_emission(&self, receipt: &BlockTransformationReceipt,
        capture: &JointRestForwardNoteCapture, after: &str) -> anyhow::Result<Self> {
        anyhow::ensure!(capture.binds(receipt) && capture.before.as_ref() == self.source.as_ref()
            && capture.after.as_ref() == after, "Render rest emitter map belongs to another execution");
        let mut mapped = BTreeMap::new();
        for (before, after) in &capture.emissions {
            anyhow::ensure!(mapped.insert((before.start, before.end), after.clone()).is_none(), "Duplicated original Render emission");
        }
        self.continue_bound_ranges(after, |head| Ok(Some(mapped.get(&(head.source_span.start, head.source_span.end))
            .cloned().ok_or_else(|| anyhow!("Original rest emitter did not carry a pitched head"))?)))
    }

    pub(crate) fn continue_render_beam_emission(&self, receipt: &BlockTransformationReceipt,
        capture: &ActualCanonicalBeamCapture, after: &str) -> anyhow::Result<Self> {
        let births = capture.course_carry_births(receipt)?;
        let ranges = births.original_carry_ranges(&self.source, after,
            self.heads.iter().map(|head| head.source_span.clone()).collect())?;
        let mut ranges = ranges.into_iter();
        self.continue_bound_ranges(after, |_| Ok(Some(ranges.next().ok_or_else(|| anyhow!("Incomplete original beam emission"))?)))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct JointRenderCustodyContext {
    pub(crate) material: Result<std::sync::Arc<JointReadingOwnerInput>, String>,
    pub(crate) beams: Option<Result<std::sync::Arc<ActualCanonicalBeamCapture>, String>>,
    recorder: std::sync::Arc<std::sync::Mutex<JointRenderCustodyState>>,
}
#[derive(Debug)]
struct JointRenderCustodyState {
    source: String,
    material: Result<std::sync::Arc<JointReadingOwnerInput>, String>,
    beams: Option<Result<std::sync::Arc<ActualCanonicalBeamCapture>, String>>,
}
#[derive(Debug, Clone)]
pub(crate) struct JointReadingRenderCustody(std::sync::Arc<std::sync::Mutex<JointRenderCustodyState>>);
thread_local! {
    static JOINT_RENDER_CUSTODY: std::cell::RefCell<std::sync::Weak<std::sync::Mutex<JointRenderCustodyState>>> = const { std::cell::RefCell::new(std::sync::Weak::new()) };
}
fn begin_joint_reading_render_custody(candidate: &SolverCandidate) -> JointReadingRenderCustody {
    let material = original_joint_render_material(candidate).map(std::sync::Arc::new);
    let state = std::sync::Arc::new(std::sync::Mutex::new(JointRenderCustodyState {
        source: candidate.layout_xml().to_owned(), material, beams: None }));
    JOINT_RENDER_CUSTODY.with(|slot| *slot.borrow_mut() = std::sync::Arc::downgrade(&state));
    JointReadingRenderCustody(state)
}
pub(crate) fn joint_render_custody_context() -> Option<JointRenderCustodyContext> {
    JOINT_RENDER_CUSTODY.with(|slot| {
        let recorder = slot.borrow().upgrade()?;
        let state = recorder.lock().unwrap_or_else(|error| error.into_inner());
        Some(JointRenderCustodyContext { material: state.material.clone(), beams: state.beams.clone(), recorder: recorder.clone() })
    })
}
pub(crate) fn observe_committed_joint_render_custody(context: JointRenderCustodyContext, source: &str,
    material: Option<Result<std::sync::Arc<JointReadingOwnerInput>, String>>,
    beams: Option<Result<std::sync::Arc<ActualCanonicalBeamCapture>, String>>) {
    let mut state = context.recorder.lock().unwrap_or_else(|error| error.into_inner());
    state.source = source.to_owned();
    state.material = material.unwrap_or_else(|| Err("Missing original Render material context".into()))
        .and_then(|input| if input.binds(source) { Ok(input) } else { Err("Committed Render material does not bind actual sovereign bytes".into()) });
    state.beams = beams;
}
impl JointReadingRenderCustody {
    fn input(&self, sovereign_source: &str) -> anyhow::Result<JointReadingOwnerInput> {
        let state = self.0.lock().unwrap_or_else(|error| error.into_inner());
        anyhow::ensure!(state.source == sovereign_source, "Render custody does not bind the actual sovereign root");
        let mut input = state.material.as_ref().map_err(|error| anyhow!("{error}"))?.as_ref().clone();
        anyhow::ensure!(input.binds(sovereign_source), "Render material belongs to another epoch");
        let beams = state.beams.as_ref().ok_or_else(|| anyhow!("Render E053 producer has not executed"))?
            .as_ref().map_err(|error| anyhow!("Render group custody: {error}"))?;
        input.bind_original_groups(beams)?;
        Ok(input)
    }
}
