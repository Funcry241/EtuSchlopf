//! Read-only source facts from existing authenticated creation edges.
//! These describe who emitted support, never musical line or silence authority.
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceOriginKind { SeedCoverageRest, SeedPitched, OriginlessGapRest }

#[derive(Clone)]
struct SourceOriginLeaf {
    creation: RhythmicCreationIdentity,
    kind: SourceOriginKind,
    course: Option<RhythmicCreationIdentity>,
    source: Option<usize>,
    root_support: Range<i64>,
}

#[derive(Clone)]
struct SourceOriginStep {
    creation: RhythmicCreationIdentity,
    birth: Option<PhysicalBirthKind>,
    parents: Vec<RhythmicCreationIdentity>,
}

#[derive(Clone)]
pub(super) struct ReceivedSourceOrigins {
    leaves: Vec<SourceOriginLeaf>,
    steps: Vec<SourceOriginStep>,
}

impl std::fmt::Display for ReceivedSourceOrigins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "leaves=[")?;
        for leaf in &self.leaves {
            write!(f, "{{creation={:?},kind={:?},course={:?},source={:?},root_support={:?}}}",
                leaf.creation, leaf.kind, leaf.course, leaf.source, leaf.root_support)?;
        }
        write!(f, "];edges=[")?;
        for step in &self.steps {
            write!(f, "{{creation={:?},birth={:?},parents={:?}}}", step.creation, step.birth, step.parents)?;
        }
        write!(f, "];musical_line_authority=none;silence_visibility_authority=none")
    }
}

impl OwnedRhythmicContinuation {
    // Called only after the existing exact binding/context check. No state,
    // identity, ancestry or phase is minted or transported by this reader.
    pub(super) fn received_source_origins(&self, creation: RhythmicCreationIdentity)
        -> Result<ReceivedSourceOrigins> {
        fn visit(state: &OwnedRhythmicContinuation, creation: RhythmicCreationIdentity,
            active: &mut BTreeSet<RhythmicCreationIdentity>,
            done: &mut BTreeSet<RhythmicCreationIdentity>, out: &mut ReceivedSourceOrigins) -> Result<()> {
            if done.contains(&creation) { return Ok(()); }
            ensure!(active.insert(creation), "Cyclic received source-origin graph");
            let edge = state.history.get(&creation).ok_or_else(|| anyhow!("Missing received source-origin edge"))?;
            if let Some(parent) = edge.parent {
                out.steps.push(SourceOriginStep { creation, birth: edge.birth, parents: vec![parent] });
                visit(state, parent, active, done, out)?;
            } else if let Some(parents) = state.received_consolidation_parents(creation) {
                ensure!(edge.birth == Some(PhysicalBirthKind::RestConsolidated) && !parents.is_empty(),
                    "Consolidation has no actual received source parents");
                out.steps.push(SourceOriginStep { creation, birth: edge.birth, parents: parents.clone() });
                for parent in parents { visit(state, parent, active, done, out)?; }
            } else {
                let kind = if let Some(root) = state.initial_custody.roots.get(&creation) {
                    ensure!(edge.root == creation && edge.source == root.source && edge.course == Some(root.course)
                        && edge.interval == root.interval && edge.birth.is_none(), "Received seed origin drift");
                    if root.source.is_some() { SourceOriginKind::SeedPitched } else { SourceOriginKind::SeedCoverageRest }
                } else {
                    ensure!(edge.root == creation && edge.birth == Some(PhysicalBirthKind::GapRest)
                        && edge.course.is_none() && edge.source.is_none() && edge.presentation.is_none(),
                        "Missing original creator for received source support");
                    SourceOriginKind::OriginlessGapRest
                };
                out.leaves.push(SourceOriginLeaf { creation, kind, course: edge.course,
                    source: edge.source, root_support: edge.interval.clone() });
            }
            active.remove(&creation); done.insert(creation);
            Ok(())
        }
        let mut out = ReceivedSourceOrigins { leaves: Vec::new(), steps: Vec::new() };
        visit(self, creation, &mut BTreeSet::new(), &mut BTreeSet::new(), &mut out)?;
        ensure!(!out.leaves.is_empty(), "Empty received source-origin evidence");
        Ok(out)
    }
}

#[cfg(test)]
#[path = "../tests/unit/received_source_origins.rs"]
mod tests;
