//! Exact N:1 origin accounting. Common received course context may be retained;
//! this issues no course, canonical role, silence obligation or writer permit.
use super::*;
use crate::arrangement::physical_births::RestConsolidationCapture;

#[derive(Debug, Clone)]
struct ConsolidatedParent {
    creation: RhythmicCreationIdentity,
    interval: Range<i64>,
    obligation: Range<i64>,
    root: RhythmicCreationIdentity,
}

#[derive(Debug, Clone)]
pub(super) struct ConsolidatedOrigin {
    parents: Vec<ConsolidatedParent>,
    execution: crate::master_state_identity::MintedIdentity,
}

fn exact_interval_union(spans: impl IntoIterator<Item = Range<i64>>, expected: &Range<i64>) -> Result<()> {
    ensure!(expected.end > expected.start, "Consolidation has nonpositive support");
    let mut spans = spans.into_iter().collect::<Vec<_>>();
    ensure!(!spans.is_empty(), "Consolidation has no received support");
    spans.sort_by_key(|r| (r.start, r.end));
    let mut end = expected.start;
    for span in spans {
        ensure!(span.end > span.start && span.start >= expected.start
            && span.start <= end && span.end <= expected.end,
            "Consolidation loses, invents or disconnects received support");
        end = end.max(span.end);
    }
    ensure!(end == expected.end, "Consolidation does not cover the actual output support");
    Ok(())
}

fn common_context<'a>(origins: impl IntoIterator<Item = &'a Origin>)
    -> (Option<RhythmicCreationIdentity>, Option<SeedCoursePresentation>) {
    let mut origins = origins.into_iter();
    let Some(first) = origins.next() else { return (None, None); };
    if first.course.is_some() && first.presentation.is_some()
        && origins.all(|o| o.course == first.course && o.presentation == first.presentation) {
        (first.course, first.presentation)
    } else { (None, None) }
}

impl OwnedRhythmicContinuation {
    pub(super) fn validate_received_origin_path(&self, creation: RhythmicCreationIdentity) -> Result<()> {
        let origin = self.history.get(&creation).ok_or_else(|| anyhow!("Missing received origin"))?;
            let mut id = creation; let mut visited = BTreeSet::new();
            let (mut course, mut presentation) = (origin.course, origin.presentation);
            loop {
                ensure!(visited.insert(id), "Cyclic owner continuation");
                let edge = self.history.get(&id).ok_or_else(|| anyhow!("Missing owner ancestor"))?;
                ensure!(edge.root == origin.root && edge.course == course && edge.source == origin.source
                    && edge.presentation == presentation, "Continuation changed course, source or course role");
                // Course and presentation change only across an explicit relocation edge.
                if let Some(relocation) = &edge.relocation {
                    (course, presentation) = (relocation.from_course, relocation.from_presentation);
                }
                if let Some(parent) = edge.parent {
                    let parent = self.history.get(&parent).ok_or_else(|| anyhow!("Missing captured parent"))?;
                    ensure!(edge.relocation.is_some() || (edge.interval.start >= parent.interval.start
                        && edge.interval.end <= parent.interval.end), "Birth extended received support");
                    id = edge.parent.unwrap();
                } else { ensure!(id == origin.root, "Foreign continuation root"); break; }
            }
            match origin.course {
                Some(course) => {
                    let coverage = self.coverage.iter().find(|c| c.creation == course)
                        .ok_or_else(|| anyhow!("Missing original lifetime decision"))?;
                    ensure!(origin.presentation == Some(coverage.presentation)
                        && origin.interval.start >= coverage.coverage.start && origin.interval.end <= coverage.coverage.end,
                        "Unlicensed course role or support extension");
                },
                None => ensure!(origin.presentation.is_none() && origin.source.is_none(),
                    "Unbound birth received invented musical context"),
            }
        Ok(())
    }

    pub(super) fn continue_consolidations(
        &self, captures: &[RestConsolidationCapture],
        output: &mut BTreeMap<(usize, usize), &PhysicalOccurrence>, next: &mut Self,
    ) -> Result<BTreeSet<(usize, usize)>> {
        let mut claimed = BTreeSet::new();
        for capture in captures {
            let parents = capture.inputs.iter().map(|bytes| {
                ensure!(claimed.insert((bytes.start, bytes.end)), "Consolidation reuses a physical parent");
                self.occurrences.iter().find(|o| o.bytes == *bytes)
                    .ok_or_else(|| anyhow!("Consolidation has no exact received parent"))
            }).collect::<Result<Vec<_>>>()?;
            let p = output.remove(&(capture.output.start, capture.output.end))
                .ok_or_else(|| anyhow!("Consolidation has no unique complete output"))?;
            ensure!(p.text.contains("<rest") && !p.text.contains("<pitch>")
                && !p.text.contains("<chord") && !p.text.contains("<grace"),
                "Consolidation is not an ordinary rest");
            ensure!(parents.iter().all(|o| o.staff == p.staff), "Consolidation changes presentation staff");
            exact_interval_union(parents.iter().map(|o| o.origin.interval.clone()), &p.interval)?;
            let (course, presentation) = common_context(parents.iter().map(|o| &o.origin));
            let parent_edges = parents.iter().map(|old| ConsolidatedParent {
                creation: old.creation, interval: old.origin.interval.clone(),
                obligation: self.obligation_span(old.creation, &old.origin.interval), root: old.origin.root,
            }).collect();
            let origin = Origin { parent: None, root: capture.creation, course, source: None,
                interval: p.interval.clone(), presentation,
                birth: Some(PhysicalBirthKind::RestConsolidated), relocation: None };
            ensure!(next.history.insert(capture.creation, origin.clone()).is_none()
                && next.consolidations.insert(capture.creation, ConsolidatedOrigin { parents: parent_edges, execution: capture.execution }).is_none(),
                "Consolidation reuses a creation identity");
            next.occurrences.push(Occurrence { creation: capture.creation, origin,
                staff: p.staff, bytes: capture.output.clone() });
        }
        Ok(claimed)
    }

    pub(super) fn validate_consolidated_origins(&self) -> Result<()> {
        // Received creator edges remain authenticated even after every physical
        // descendant has joined a consolidated representation.
        for (&id, edge) in &self.initial_custody.history {
            let original_course = self.initial_custody.coverage.iter().find(|c| c.creation == edge.course)
                .ok_or_else(|| anyhow!("Missing received creator course"))?;
            let origin = self.history.get(&id).ok_or_else(|| anyhow!("Lost received creator edge"))?;
            ensure!(origin.parent == edge.parent && origin.root == edge.root && origin.source == edge.source
                && origin.interval == edge.interval && origin.course == Some(edge.course)
                && origin.presentation == Some(original_course.presentation)
                && origin.birth.is_none() && origin.relocation.is_none(),
                "Received creator ancestry was changed");
        }
        for (&id, origin) in &self.history {
            if origin.parent.is_none() && origin.birth == Some(PhysicalBirthKind::RestConsolidated) {
                ensure!(self.consolidations.contains_key(&id), "Missing actual consolidation ancestry");
            }
        }
        let mut executions = BTreeSet::new();
        for (&id, merge) in &self.consolidations {
            ensure!(executions.insert(merge.execution), "One editorial emission has duplicate rest creations");
            let origin = self.history.get(&id).ok_or_else(|| anyhow!("Consolidation has no created origin"))?;
            ensure!(origin.root == id && origin.parent.is_none() && origin.source.is_none()
                && origin.relocation.is_none() && origin.birth == Some(PhysicalBirthKind::RestConsolidated),
                "Consolidation origin is not its actual independent rest birth");
            let mut unique = BTreeSet::new();
            let parents = merge.parents.iter().map(|edge| {
                ensure!(unique.insert(edge.creation) && edge.creation != id, "Duplicate or cyclic consolidation parent");
                let parent = self.history.get(&edge.creation).ok_or_else(|| anyhow!("Missing consolidation parent origin"))?;
                self.validate_received_origin_path(edge.creation)?;
                ensure!(parent.root == edge.root && parent.interval == edge.interval
                    && edge.obligation == self.obligation_span(edge.creation, &edge.interval)
                    && edge.obligation.end - edge.obligation.start == edge.interval.end - edge.interval.start,
                    "Consolidation parent origin or received support drift");
                Ok(parent)
            }).collect::<Result<Vec<_>>>()?;
            exact_interval_union(merge.parents.iter().map(|p| p.interval.clone()), &origin.interval)?;
            ensure!((origin.course, origin.presentation) == common_context(parents),
                "Consolidation invented or changed common received context");
        }
        Ok(())
    }

    /// Project each current representation to every retained root, including
    /// intermediate consolidated roots. Ancestor identities never collapse.
    pub(super) fn support_by_root(&self) -> Result<BTreeMap<RhythmicCreationIdentity, Vec<Range<i64>>>> {
        let mut support = BTreeMap::<_, Vec<_>>::new();
        for occurrence in self.occurrences.iter().chain(self.encoded.iter().map(|encoded| &encoded.received)) {
            let mut pending = vec![(occurrence.origin.root,
                self.obligation_span(occurrence.creation, &occurrence.origin.interval), BTreeSet::new())];
            while let Some((root, span, mut path)) = pending.pop() {
                ensure!(path.insert(root), "Cyclic consolidation ancestry");
                support.entry(root).or_default().push(span.clone());
                if let Some(merge) = self.consolidations.get(&root) {
                    for parent in &merge.parents {
                        let overlap_start = span.start.max(parent.interval.start);
                        let overlap_end = span.end.min(parent.interval.end);
                        if overlap_end <= overlap_start { continue; }
                        let mapped_start = parent.obligation.start.checked_add(overlap_start - parent.interval.start)
                            .ok_or_else(|| anyhow!("Consolidation support overflow"))?;
                        let mapped_end = mapped_start.checked_add(overlap_end - overlap_start)
                            .ok_or_else(|| anyhow!("Consolidation support overflow"))?;
                        pending.push((parent.root, mapped_start..mapped_end, path.clone()));
                    }
                }
            }
        }
        Ok(support)
    }
}

#[cfg(test)]
#[path = "../tests/unit/editorial_rest_consolidation_custody.rs"]
mod tests;
