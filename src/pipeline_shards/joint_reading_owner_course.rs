// Finite C3 comparison internal to the existing register/clef owners.
// This owns neither a writer sink nor an adoption or identity-mint capability.
use std::sync::atomic::{AtomicBool, Ordering as JointOrdering};
static JOINT_READING_ACTIVE: AtomicBool = AtomicBool::new(false);
static JOINT_READING_ACCEPTED: std::sync::Mutex<Option<JointAcceptedReading>> = std::sync::Mutex::new(None);
struct JointAcceptedReading {
    _custody: JointReadingRenderCustody,
    master: String,
    render: String,
    master_inputs: JointReadingOwnerInput,
    render_inputs: JointReadingOwnerInput,
    master_proposal: c3_adapter::C3Proposal,
    render_proposal: c3_adapter::C3Proposal,
    selected: BTreeSet<String>,
}
thread_local! {
    static JOINT_READING_PREPARED_LOWER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}
/// Scoped suppression during the original Render checkpoint, before permanent
/// acceptance exists. No receipt, source identity or write permission is issued.
pub(crate) struct JointReadingCheckpointProtection {
    previous: bool,
    _same_thread: std::marker::PhantomData<std::rc::Rc<()>>,
}
pub(crate) fn protect_joint_reading_checkpoint(lower_selected: bool) -> JointReadingCheckpointProtection {
    let previous=JOINT_READING_PREPARED_LOWER.with(|slot| {
        let previous=slot.get();
        slot.set(previous || lower_selected);
        previous
    });
    JointReadingCheckpointProtection {previous,_same_thread:std::marker::PhantomData}
}
impl Drop for JointReadingCheckpointProtection {
    fn drop(&mut self) {
        JOINT_READING_PREPARED_LOWER.with(|slot|slot.set(self.previous));
    }
}
pub(crate) fn joint_reading_course_active() -> bool {
    JOINT_READING_ACTIVE.load(JointOrdering::Relaxed) || JOINT_READING_PREPARED_LOWER.with(|slot|slot.get())
}
pub(crate) fn reset_joint_reading_course() {
    crate::audit_only::c3_diagnostics::reset();
    JOINT_READING_ACTIVE.store(false,JointOrdering::Relaxed);
    *JOINT_READING_ACCEPTED.lock().unwrap_or_else(|error|error.into_inner())=None;
}
fn joint_octave_course(solved: &c3_adapter::SolvedStaff, original: bool, end: c3_adapter::Q) -> Vec<(c3_adapter::Q,u8)> {
    let mut course=vec![(c3_adapter::Q::ZERO,0)];
    for (index,anchor) in solved.anchors.iter().enumerate() {
        let state=if original { solved.domain.layers[index].incumbent } else { solved.outcome.selected[index] };
        let octave=state.index()%5;
        if course.last().is_some_and(|(_,old)| *old==octave) {continue;}
        if course.last().is_some_and(|(at,_)| *at==anchor.at) {course.pop();}
        course.push((anchor.at,octave));
    }
    if course.last().is_some_and(|(_,octave)| *octave!=0) {course.push((end,0));}
    course
}
fn joint_display(index:u8)->crate::registernotation::RegisternotationsDarstellung {
    use crate::registernotation::RegisternotationsDarstellung as D;
    match index {1=>D::Achtva,2=>D::Achtvb,3=>D::Fuenfzehnma,4=>D::Fuenfzehnmb,_=>D::Keine}
}
fn joint_selected_plans(input:&JointReadingOwnerInput, proposal:&c3_adapter::C3Proposal,
    changed:&BTreeSet<String>, old:&crate::registernotation::RegisternotationsSichtmaterialisierungsauftrag)
    ->anyhow::Result<Vec<crate::registernotation::RegisternotationsMaterialisierungsPlan>> {
    let source=input.bound_source();let clock=c3_adapter::observe_clock(source).map_err(|e|anyhow!("{e:?}"))?;
    let anchors=observe_joint_octave_anchors(source)?;
    let doc=roxmltree::Document::parse(source)?;
    let part=doc.root_element().children().find(|node|node.has_tag_name("part"))
        .and_then(|node|node.attribute("id")).ok_or_else(||anyhow!("Missing original part"))?;
    let mut plans=old.plaene.iter().filter(|plan|!changed.contains(&plan.system)).cloned().collect::<Vec<_>>();
    for staff in changed {
        let Some(c3_adapter::StaffVerdict::Solved(solved))=proposal.staves.get(staff) else {anyhow::bail!("Unclosed C3 octave proposal");};
        let course=joint_octave_course(solved,false,clock.end);
        for window in course.windows(2) {
            let [(start,octave),(end,_)]=window else {unreachable!()};
            if *octave==0 {continue;}
            let choose=|at:c3_adapter::Q,kind:RegisternotationsGrenzart| -> anyhow::Result<&JointOctaveWriterAnchor> {
                let mut candidates=anchors.iter().filter(|anchor|anchor.staff==*staff&&anchor.kind==kind)
                    .filter(|anchor|clock.times.get(&(anchor.position,kind==RegisternotationsGrenzart::Stop))==Some(&at)).collect::<Vec<_>>();
                candidates.sort_by_key(|anchor|anchor.position);
                let found=if kind==RegisternotationsGrenzart::Stop {candidates.last()} else {candidates.first()};
                found.copied().ok_or_else(||anyhow!("C3 octave boundary has no original visible writer anchor"))
            };
            let first=choose(*start,RegisternotationsGrenzart::Start)?;
            let last=choose(*end,RegisternotationsGrenzart::Stop)?;
            plans.push(crate::registernotation::RegisternotationsMaterialisierungsPlan {teil:part.to_owned(),system:staff.clone(),
                beginn_takt_ordinal:first.measure,ende_takt_ordinal:last.measure,
                beginn_einsatz:first.onset,ende_einsatz:last.onset,darstellung:joint_display(*octave),sichtwiederholungen:Vec::new()});
        }
    }
    plans.sort_by_key(|plan|(plan.teil.clone(),plan.system.clone(),plan.beginn_takt_ordinal,plan.beginn_einsatz));
    Ok(plans)
}
fn joint_births(directions:&[RegisternotationsRichtung])->Vec<crate::registernotation::reader_policy::Birth> {
    directions.iter().map(|direction|crate::registernotation::reader_policy::Birth {
        part:direction.teil.clone(),staff:direction.system.clone(),slot:direction.takt_ordinal,onset:direction.einsatz,
        kind:match direction.art {RegisternotationsGrenzart::Stop=>"stop",RegisternotationsGrenzart::Continue=>"continue",
            RegisternotationsGrenzart::Start=>match direction.darstellung {crate::registernotation::RegisternotationsDarstellung::Achtvb|crate::registernotation::RegisternotationsDarstellung::Fuenfzehnmb=>"up",_=>"down"}}.to_owned(),
        display:direction.darstellung,line:direction.linie,
    }).collect()
}
fn joint_replay_edits(source:&str,edits:&[(std::ops::Range<usize>,String)])->anyhow::Result<String> {
    let mut parts=Vec::new();let mut cursor=0;
    for (range,text) in edits {
        anyhow::ensure!(range.start>=cursor&&range.start<=range.end&&range.end<=source.len(),"Overlapping original C3 operations");
        parts.push(&source[cursor..range.start]);parts.push(text);cursor=range.end;
    }
    parts.push(&source[cursor..]);Ok(parts.concat())
}
fn prepare_joint_octave_write(source:&str,directions:&[RegisternotationsRichtung])->anyhow::Result<PreparedJointOctaveWrite> {
    let (_,old_ranges)=beobachte_registernotationsrichtungen(source)?;
    let doc=roxmltree::Document::parse(source)?;
    let mut measures=Vec::new();
    for part in doc.root_element().children().filter(|node|node.has_tag_name("part")) {
        let id=part.attribute("id").ok_or_else(||anyhow!("Unbound octave part"))?;
        for (index,measure) in part.children().filter(|node|node.has_tag_name("measure")).enumerate() {
            let span=measure.range();let original=&source[span.clone()];
            let local=directions.iter().filter(|direction|direction.teil==id&&direction.takt_ordinal==index+1).cloned().collect::<Vec<_>>();
            let (_,insertion)=registernotationsrichtungen_in_takt(original,&local)?;
            let mut operations=insertion.original_operations().to_vec();
            operations.extend(old_ranges.iter().filter(|range|span.start<=range.start&&range.end<=span.end)
                .map(|range|(range.start-span.start..range.end-span.start,String::new())));
            operations.sort_by_key(|(range,_)|(range.start,!range.is_empty()));
            if operations.is_empty(){continue;}
            let post=joint_replay_edits(original,&operations)?;
            measures.push((span,post,operations));
        }
    }
    drop(doc);measures.sort_by_key(|(span,_,_)|std::cmp::Reverse(span.start));
    let mut current=source.to_owned();let mut edits=Vec::new();
    for (span,measure,operations) in measures {
        let after=[&current[..span.start],&measure,&current[span.end..]].concat();
        let full=operations.into_iter().map(|(range,text)|(span.start+range.start..span.start+range.end,text)).collect();
        let journal=crate::arrangement::representation_transport::RepresentationStep::recorded(&current,&after,full);
        let journal=crate::registernotation::reader_policy::bind_original_octave_removal_evidence(journal)?;
        edits.push((span.start,span.end,measure,journal));current=after;
    }
    let mut old_ranges=old_ranges;old_ranges.sort_by_key(|range|range.start);
    let removal=old_ranges.into_iter().map(|range|(range,String::new())).collect::<Vec<_>>();
    let stripped=joint_replay_edits(source,&removal)?;
    pruefe_registernotationsmaterialisierung(&stripped,&current,directions)?;
    Ok(PreparedJointOctaveWrite {source:std::sync::Arc::from(source),post:current,edits})
}
impl PreparedJointOctaveWrite {
    fn carry(&self,mut range:std::ops::Range<usize>)->anyhow::Result<std::ops::Range<usize>> {
        for (_,_,_,step) in &self.edits {range=step.carry_original_range(range)?;}Ok(range)
    }
}
fn prepare_joint_clef_write(input:&JointReadingOwnerInput,proposal:&c3_adapter::C3Proposal,
    selected:&BTreeSet<String>,octave:&PreparedJointOctaveWrite)
    ->anyhow::Result<crate::structural_write_authority::RustigClefSurfaceDecision> {
    let Some(c3_adapter::StaffVerdict::Solved(solved))=proposal.staves.get("2").filter(|_|selected.contains("2")) else {
        return crate::structural_write_authority::RustigClefSurfaceDecision::prepare_joint_course(&octave.post,&[]).map_err(|e|anyhow!("{e}"));
    };
    let source=input.bound_source();let clock=c3_adapter::observe_clock(source).map_err(|e|anyhow!("{e:?}"))?;
    let owner=crate::structural_write_authority::RustigClefSurfaceDecision::observe_joint_boundaries(source)
        .ok_or_else(||anyhow!("Missing original E024 capability"))?;
    let doc=roxmltree::Document::parse(source)?;
    let nodes=doc.descendants().filter(|node|node.is_element()).map(|node|(node.range().start,node)).collect::<BTreeMap<_,_>>();
    let mut layers=Vec::new();
    for (index,anchor) in solved.anchors.iter().enumerate() {
        let mut events=Vec::new();
        for position in &anchor.incumbent_events {
            if let Some(node)=nodes.get(position).filter(|node|node.has_tag_name("attributes")) {
                events.push(octave.carry(node.range())?.start);
            }
        }
        let original_anchor=owner.insertions.iter().filter(|position|clock.times.get(&(**position,false))==Some(&anchor.at)).min();
        let mapped=original_anchor.map(|position|nodes.get(position).ok_or_else(||anyhow!("C3 clef anchor lost its complete container"))
            .and_then(|node|octave.carry(node.range()).map(|range|range.start))).transpose()?;
        layers.push((solved.outcome.selected[index].index()<5,events,mapped));
    }
    crate::structural_write_authority::RustigClefSurfaceDecision::prepare_joint_course(&octave.post,&layers).map_err(|e|anyhow!("{e}"))
}
fn record_joint_search_result(label:&str,source:&str,proposal:&c3_adapter::C3Proposal) {
    for (staff,verdict) in &proposal.staves {
                match verdict {
                    c3_adapter::StaffVerdict::Solved(solved)=>{
                        crate::audit_only::c3_diagnostics::record(label,staff,"solved",&format!("IncumbentCost={:?}; SearchSelectedCost={:?}; StrictlyBetter={}; Complete={}",solved.outcome.incumbent_cost,solved.outcome.selected_cost,solved.outcome.improved,solved.outcome.complete));
                        let anchors=solved.anchors.iter().map(|anchor|(anchor.measure,anchor.at.parts())).collect::<Vec<_>>();
                        crate::audit_only::c3_diagnostics::record_course_excursions(label,staff,source,
                            &solved.domain,&solved.outcome,&anchors);
                    },
                    c3_adapter::StaffVerdict::Unknown {reason}=>crate::audit_only::c3_diagnostics::record(label,staff,"unknown",reason),
                }
    }
}
pub(crate) struct JointCourseProof {
    master_input: JointReadingOwnerInput,
    render_input: JointReadingOwnerInput,
    master_proposal: c3_adapter::C3Proposal,
    render_proposal: c3_adapter::C3Proposal,
    visible: crate::registernotation::RegisternotationsSichtmaterialisierungsauftrag,
    selected: BTreeSet<String>,
}
impl JointReadingCandidate for SolverCandidate {
    type Custody = JointReadingRenderCustody;
    type Proof = JointCourseProof;
    fn joint_lower_course_selected(&self, proof: &Self::Proof) -> bool { proof.selected.contains("2") }
    fn joint_master_state(&mut self) -> &mut crate::structural_write_authority::MasterAdoptedState {
        &mut self.master_state
    }
    fn prepare_joint_course(&self, custody:&Self::Custody, render_source:&str)
        -> anyhow::Result<PreparedJointReadingCourse<Self::Proof>> {
        let best=self;
        crate::audit_only::c3_diagnostics::phase("master_input");
        let master_input=JointReadingOwnerInput::from_candidate(best)?;
        crate::audit_only::c3_diagnostics::phase("render_input");
        let render_input=custody.input(render_source)?;
        crate::audit_only::c3_diagnostics::phase("master_capabilities");
        let master_capabilities=observe_joint_capabilities(master_input.bound_source())?;
        crate::audit_only::c3_diagnostics::phase("render_capabilities");
        let render_capabilities=observe_joint_capabilities(render_input.bound_source())?;
        crate::audit_only::c3_diagnostics::phase("master_search");
        let mut master_proposal=master_input.c3_proposal(&master_capabilities,None).map_err(|e|anyhow!("{e:?}"))?;
        record_joint_search_result("Master",master_input.bound_source(),&master_proposal);
        crate::audit_only::c3_diagnostics::phase("render_search");
        let mut render_proposal=render_input.c3_proposal(&render_capabilities,None).map_err(|e|anyhow!("{e:?}"))?;
        record_joint_search_result("Render",render_input.bound_source(),&render_proposal);
        for (label,proposal) in [("Master",&master_proposal),("Render",&render_proposal)] {
            for (staff,verdict) in &proposal.staves {
                if let c3_adapter::StaffVerdict::Unknown {reason}=verdict {
                    println!("C3JointOwner: Artifact={label}; Staff={staff}; UNKNOWN {reason}; incumbent retained");
                }
            }
        }
        crate::audit_only::c3_diagnostics::phase("master_clock");
        let master_clock=c3_adapter::observe_clock(master_input.bound_source()).map_err(|e|anyhow!("{e:?}"))?;
        crate::audit_only::c3_diagnostics::phase("render_clock");
        let render_clock=c3_adapter::observe_clock(render_input.bound_source()).map_err(|e|anyhow!("{e:?}"))?;
        anyhow::ensure!(master_clock.end==render_clock.end,"Master/Render comparison intervals differ");
        crate::audit_only::c3_diagnostics::phase("shared_octave_comparison");
        let mut selected=BTreeSet::new();let mut changed_octaves=BTreeSet::new();let mut improved=false;
        for staff in master_clock.entries.keys() {
            match (master_proposal.staves.get(staff),render_proposal.staves.get(staff)) {
                (Some(c3_adapter::StaffVerdict::Solved(master)),Some(c3_adapter::StaffVerdict::Solved(renderer)))
                    if joint_octave_course(master,false,master_clock.end)==joint_octave_course(renderer,false,render_clock.end)=>{
                    crate::audit_only::c3_diagnostics::record("both",staff,"octave_compatible","equal selected octave courses");
                    selected.insert(staff.clone());improved|=master.outcome.improved||renderer.outcome.improved;
                    if joint_octave_course(master,false,master_clock.end)!=joint_octave_course(master,true,master_clock.end) {changed_octaves.insert(staff.clone());}
                }
                _=>{
                    let reason=match (master_proposal.staves.get(staff),render_proposal.staves.get(staff)) {
                        (Some(c3_adapter::StaffVerdict::Solved(_)),Some(c3_adapter::StaffVerdict::Solved(_)))=>"selected octave courses differ",
                        _=>"unsolved or missing document input",
                    };
                    crate::audit_only::c3_diagnostics::record("both",staff,"octave_not_closed",reason);
                    println!("C3JointOwner: Staff={staff}; UNKNOWN unclosed shared octave/group dependency; incumbent retained");
                }
            }
        }
        crate::audit_only::c3_diagnostics::phase("strict_improvement");
        anyhow::ensure!(improved,"No compatible strictly improved complete C3 course");
        // No greedy reconciliation: incompatible independent optima keep their
        // complete original courses. Compatible independent minima are also a
        // minimum under the proved shared-octave constraint.
        for (staff,verdict) in &mut master_proposal.staves {
            if !selected.contains(staff)&&let c3_adapter::StaffVerdict::Solved(solved)=verdict {
                solved.outcome.selected=solved.domain.layers.iter().map(|layer|layer.incumbent).collect();
                solved.outcome.selected_cost=solved.outcome.incumbent_cost;solved.outcome.improved=false;
            }
        }
        for (staff,verdict) in &mut render_proposal.staves {
            if !selected.contains(staff)&&let c3_adapter::StaffVerdict::Solved(solved)=verdict {
                solved.outcome.selected=solved.domain.layers.iter().map(|layer|layer.incumbent).collect();
                solved.outcome.selected_cost=solved.outcome.incumbent_cost;solved.outcome.improved=false;
            }
        }
        crate::audit_only::c3_diagnostics::phase("existing_octave_plan");
        let old=crate::registernotation::sichtmaterialisierungsauftrag().map_err(|e|anyhow!("{e}"))?;
        crate::audit_only::c3_diagnostics::phase("master_octave_anchors");
        let master_plans=joint_selected_plans(&master_input,&master_proposal,&changed_octaves,&old)?;
        crate::audit_only::c3_diagnostics::phase("render_octave_anchors");
        let render_plans=joint_selected_plans(&render_input,&render_proposal,&changed_octaves,&old)?;
        crate::audit_only::c3_diagnostics::phase("shared_octave_anchors");
        anyhow::ensure!(master_plans==render_plans,"Common octave plan has different original writer anchors");
        let fingerprint=crate::sha256::hex(format!("C3 joint source-carried course {master_plans:?}").as_bytes());
        let visible=crate::registernotation::RegisternotationsSichtmaterialisierungsauftrag {semantische_plaene:master_plans.len(),
            kontaktfusionen:0,stille_bruecken:master_plans.iter().map(|plan|plan.sichtwiederholungen.len()).sum(),
            semantischer_plan_fingerabdruck:fingerprint.clone(),sicht_fingerabdruck:fingerprint,plaene:master_plans};
        crate::audit_only::c3_diagnostics::phase("master_directions");
        let mut master_directions=registernotationsrichtungen_aus_sichtauftrag(master_input.bound_source(),&visible.plaene)?;
        crate::audit_only::c3_diagnostics::phase("render_directions");
        let mut render_directions=registernotationsrichtungen_aus_sichtauftrag(render_input.bound_source(),&visible.plaene)?;
        crate::audit_only::c3_diagnostics::phase("origin_births");
        let births=joint_births(&master_directions);
        anyhow::ensure!(births==joint_births(&render_directions),"Shared origins lack the same actual Master/Render birth anchors");
        crate::audit_only::c3_diagnostics::phase("origin_preparation");
        let origins=crate::registernotation::reader_policy::prepare_joint_origins(&visible,&births,master_input.bound_source(),render_input.bound_source())?;
        for (direction,id) in master_directions.iter_mut().zip(origins.ids()) {direction.origin=Some(id.clone());}
        for (direction,id) in render_directions.iter_mut().zip(origins.ids()) {direction.origin=Some(id.clone());}
        crate::audit_only::c3_diagnostics::phase("master_octave_preparation");
        let master_octaves=prepare_joint_octave_write(master_input.bound_source(),&master_directions)?;
        crate::audit_only::c3_diagnostics::phase("render_octave_preparation");
        let render_octaves=prepare_joint_octave_write(render_input.bound_source(),&render_directions)?;
        crate::audit_only::c3_diagnostics::phase("master_clef_preparation");
        let master_clefs=prepare_joint_clef_write(&master_input,&master_proposal,&selected,&master_octaves)?;
        crate::audit_only::c3_diagnostics::phase("render_clef_preparation");
        let render_clefs=prepare_joint_clef_write(&render_input,&render_proposal,&selected,&render_octaves)?;
        let master_post=master_clefs.joint_course_post().ok_or_else(||anyhow!("Missing Master C3 projection"))?.to_owned();
        let render_post=render_clefs.joint_course_post().ok_or_else(||anyhow!("Missing Render C3 projection"))?.to_owned();
        crate::audit_only::c3_diagnostics::phase("origin_document_validation");
        let origins=origins.validate_documents(&master_post,&render_post)?;
        crate::audit_only::c3_diagnostics::phase("master_course_validation");
        joint_verify_prepared_course(&master_input,&master_proposal,&master_post)?;
        crate::audit_only::c3_diagnostics::phase("render_course_validation");
        joint_verify_prepared_course(&render_input,&render_proposal,&render_post)?;
        Ok(PreparedJointReadingCourse { master_octaves,render_octaves,master_clefs,render_clefs,origins,
            proof:JointCourseProof {master_input,render_input,master_proposal,render_proposal,visible,selected} })
    }
    fn observe_joint_course(&mut self,custody:Self::Custody,proof:Self::Proof,master_post:String,render_post:String)
        -> anyhow::Result<()> {
        let JointCourseProof {master_input,render_input,master_proposal,render_proposal,visible,selected}=proof;
        crate::audit_only::c3_diagnostics::phase("completed");
        crate::audit_only::c3_diagnostics::finish("adopted",&format!("SelectedSystems={selected:?}; LowerClefBindingActive={}",selected.contains("2")));
        for staff in master_proposal.staves.keys() {
            crate::audit_only::c3_diagnostics::record("both",staff,if selected.contains(staff) {"adopted"} else {"retained"},"completed joint transaction");
        }
        JOINT_READING_ACTIVE.store(selected.contains("2"),JointOrdering::Relaxed);
    let (actual_directions,_)=beobachte_registernotationsrichtungen(&master_post)?;
    let carriers=actual_directions.into_iter().filter(|direction|direction.art==RegisternotationsGrenzart::Continue)
        .map(|direction|crate::registernotation::OttavaCarrier {part:direction.teil,staff:direction.system,
            measure:direction.takt_ordinal,onset:direction.einsatz,display:direction.darstellung,line:direction.linie}).collect();
    crate::registernotation::observe_joint_materialized_context(visible,carriers);
    println!("C3JointOwner: adopted complete compatible courses; OriginalHeadsMaster={}; OriginalHeadsRender={}; OriginalGroupsMaster={}; OriginalGroupsRender={}; Identities=carried; SourceEpochs=bound",master_input.heads().len(),render_input.heads().len(),master_input.groups().len(),render_input.groups().len());
    *JOINT_READING_ACCEPTED.lock().unwrap_or_else(|error|error.into_inner())=Some(JointAcceptedReading {
        _custody:custody,master:master_post,render:render_post,master_inputs:master_input,render_inputs:render_input,master_proposal,render_proposal,selected});
    Ok(())
    }
}

fn joint_verify_prepared_course(input:&JointReadingOwnerInput,proposal:&c3_adapter::C3Proposal,post:&str)->anyhow::Result<()> {
    let courses=c3_adapter::read_register_courses(post).map_err(|error|anyhow!("{error:?}"))?;
    let mut identities=BTreeSet::new();
    for head in &input.heads {
        identities.insert((head.source_event_id.0,head.attack_id.0));
    }
    anyhow::ensure!(!identities.is_empty(),"C3 has no retained original source/attack identities");
    for (staff,verdict) in &proposal.staves {
        let c3_adapter::StaffVerdict::Solved(solved)=verdict else {continue;};
        for anchor in &solved.anchors {
            for &index in &anchor.heads {
                let head=input.heads.get(index).ok_or_else(||anyhow!("C3 layer has an unbound original head"))?;
                anyhow::ensure!(head.staff==*staff && head.measure==anchor.measure,
                    "C3 layer does not bind the original head's staff and measure");
            }
        }
        let course=courses.get(staff).ok_or_else(||anyhow!("C3 actual register state is unsupported"))?;
        let actual=solved.anchors.iter().map(|anchor|c3_adapter::course_state_at(course,anchor.at)
            .ok_or_else(||anyhow!("C3 actual state has no bound entry"))).collect::<anyhow::Result<Vec<_>>>()?;
        anyhow::ensure!(actual==solved.outcome.selected,"Original materializers did not realize the selected complete C3 path");
        let cost=c3_course::evaluate_course(&solved.domain,&actual).map_err(|error|anyhow!("{error:?}"))?;
        anyhow::ensure!(cost==solved.outcome.selected_cost,"Actual prepared C3 path differs from its compared cost");
    }
    Ok(())
}

fn validate_joint_tonal_closure(input:&JointReadingOwnerInput,source:&str)->anyhow::Result<()> {
    let tonal=&input.tonal;
    if let Some(profile)=tonal.selected_profile.filter(|_|tonal.scorewide_attempted||tonal.final_section.is_some()){
        let notes=parse_atomic_notes(source);
        let by_range=notes.iter().map(|note|((note.note_start,note.note_end),note)).collect::<BTreeMap<_,_>>();
        for plan in functional_respelling_plans_for_tonal_frame_from_measure(source,profile.fifths,profile.tonic,profile.minor,
            tonal.final_section.as_ref().map(|section|section.boundary_measure)) {
            let pitch=by_range.get(&(plan.note_start,plan.note_end)).and_then(|note|note.pitch.as_ref())
                .ok_or_else(||anyhow!("Actual tonal result has no complete original-owner container"))?;
            anyhow::ensure!(pitch.step==plan.step&&pitch.octave==plan.octave&&pitch.alter==plan.alter,
                "Actual tonal writer did not close the C3 prospective spelling context");
        }
    }
    Ok(())
}
fn validate_joint_reading_final_artifacts(master:&Path,render:&Path)->anyhow::Result<()> {
    let accepted=JOINT_READING_ACCEPTED.lock().unwrap_or_else(|error|error.into_inner());
    let Some(accepted)=accepted.as_ref() else{return Ok(());};
    let mut improved=false;
    for (label,path,prepared,input,proposal) in [("Master",master,&accepted.master,&accepted.master_inputs,&accepted.master_proposal),
        ("Render",render,&accepted.render,&accepted.render_inputs,&accepted.render_proposal)]{
        let actual=std::fs::read_to_string(path)?;
        validate_joint_tonal_closure(input,&actual)?;
        let expected=c3_adapter::read_register_courses(prepared).map_err(|error|anyhow!("{error:?}"))?;
        let observed=c3_adapter::read_register_courses(&actual).map_err(|error|anyhow!("{error:?}"))?;
        for (staff,verdict) in &proposal.staves {
            if !accepted.selected.contains(staff) {continue;}
            let c3_adapter::StaffVerdict::Solved(solved)=verdict else{continue;};
            if expected.get(staff)!=observed.get(staff) {
                let difference=crate::audit_only::c3_diagnostics::first_course_difference(
                    expected.get(staff).map(Vec::as_slice).unwrap_or_default(),
                    observed.get(staff).map(Vec::as_slice).unwrap_or_default());
                let detail=format!("FinalArtifact={label}; Staff={staff}; FirstDifference={difference:?}; TimeUnit=quarter_notes");
                crate::audit_only::c3_diagnostics::record(label,staff,"final_course_difference",&detail);
                anyhow::bail!("A later writer changed the selected {label} C3 course; {detail}");
            }
            let (baseline,cost,metrics)=c3_adapter::compare_actual_output(&actual,input.bound_source(),staff)
                .map_err(|error|anyhow!("{label} final C3 readback: {error:?}"))?;
            anyhow::ensure!(cost<=baseline,"Actual {label} C3 cost became worse after late materialization");
            improved|=cost<baseline;
            println!("C3FinalReadback: Artifact={label}; Staff={staff}; Baseline={baseline:?}; Actual={cost:?}; VisibleHeads={}; MaxLedgers={}; LedgerSum={}; Histogram={:?}; PlanningImproved={}; CourseBound=true; HeadGuard=true; MeasurementAuthority=none",metrics.visible_heads,metrics.maximum,metrics.sum,metrics.histogram,solved.outcome.improved);
        }
    }
    anyhow::ensure!(improved,"No strictly improved C3 course remains in the actual final artifacts");
    Ok(())
}

/// Observation after successful Render commit only; no veto, edits or permits.
pub(crate) fn observe_joint_render_course_checkpoint(transition:&str,source:&str) {
    let accepted=JOINT_READING_ACCEPTED.lock().unwrap_or_else(|error|error.into_inner());
    let Some(accepted)=accepted.as_ref() else {return;};
    if accepted.selected.contains("2") {crate::audit_only::c3_diagnostics::guard_checkpoint(transition,source);}
    let courses=c3_adapter::read_register_courses(&accepted.render)
        .and_then(|expected|c3_adapter::read_register_courses(source).map(|actual|(expected,actual)));
    match courses {
        Ok((expected,actual))=>for staff in &accepted.selected {
            let before=expected.get(staff).map(Vec::as_slice).unwrap_or_default();
            let after=actual.get(staff).map(Vec::as_slice).unwrap_or_default();
            let difference=crate::audit_only::c3_diagnostics::first_course_difference(before,after);
            crate::audit_only::c3_diagnostics::render_checkpoint(staff,difference.is_some(),
                &format!("Transition={transition:?}; FirstDifference={difference:?}; TimeUnit=quarter_notes; State=clef_and_octave; MusicalAuthority=none"));
        },
        Err(error)=>crate::audit_only::c3_diagnostics::record("Render","selected","checkpoint_unknown",
            &format!("Transition={transition:?}; ReadbackError={error:?}; no_difference_inferred")),
    }
}
