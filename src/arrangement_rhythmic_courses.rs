use super::*;
#[path = "owned_rhythmic_continuation.rs"]
pub(crate) mod continuation;
use crate::midi_seed::{MidiSeedRhythmicCourses, RhythmicCreationIdentity, SeedArrangementCourse, SeedCoursePresentation};
use anyhow::{anyhow, ensure, Result};
use std::ops::Range;

#[derive(Debug, Clone)]
pub(super) struct CourseOccurrence {
    creation: RhythmicCreationIdentity,
    parent: Option<RhythmicCreationIdentity>,
    root: RhythmicCreationIdentity,
    course: RhythmicCreationIdentity,
    interval: Range<i64>,
    source: Option<usize>,
    note: String,
    restified: bool,
    restaffed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CreationEdge {
    parent: Option<RhythmicCreationIdentity>,
    root: RhythmicCreationIdentity,
    course: RhythmicCreationIdentity,
    source: Option<usize>,
    interval: Range<i64>,
}

impl CourseOccurrence {
    fn edge(&self) -> CreationEdge {
        CreationEdge {
            parent: self.parent, root: self.root, course: self.course,
            source: self.source, interval: self.interval.clone(),
        }
    }
}

/// Exact custody at the initial arrangement boundary. It supplies no authority
/// for a different later document and no canonical pitched-source role.
#[derive(Debug, Clone)]
pub(crate) struct ArrangementRhythmicCourses {
    coverage: Vec<SeedArrangementCourse>,
    roots: BTreeMap<RhythmicCreationIdentity, CourseOccurrence>,
    history: BTreeMap<RhythmicCreationIdentity, CreationEdge>,
    occurrences: Vec<CourseOccurrence>,
    snapshot: String,
}

pub(super) struct CourseMeasure {
    coverage: Vec<SeedArrangementCourse>,
    history: BTreeMap<RhythmicCreationIdentity, CreationEdge>,
    incoming: Vec<CourseOccurrence>,
    outgoing: Vec<CourseOccurrence>,
    offset: i64,
    duration: i64,
}

impl ArrangementRhythmicCourses {
    #[cfg(test)]
    pub(crate) fn same_origins_as(&self, other: &Self) -> bool {
        self.history == other.history
            && self.coverage.len() == other.coverage.len()
            && self.coverage.iter().zip(&other.coverage).all(|(a, b)|
                a.creation == b.creation && a.staff == b.staff && a.presentation == b.presentation && a.coverage == b.coverage)
            && self.occurrences.len() == other.occurrences.len()
            && self.occurrences.iter().zip(&other.occurrences).all(|(a, b)| a.creation == b.creation && a.edge() == b.edge())
    }

    pub(crate) fn continue_representation(&self, xml: &str,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport) -> Result<Self> {
        let input = collect_raw_notes(&self.snapshot);
        let ranges = input.iter().map(|note| note.start..note.end).collect();
        let carried = transport.carry(&self.snapshot, xml, ranges)?;
        let output = collect_raw_notes(xml);
        ensure!(output.len() == self.occurrences.len(), "Representation lost or added an occurrence");
        let mut next = self.clone();
        for (((occurrence, original), destination), note) in next.occurrences.iter_mut()
            .zip(input).zip(carried).zip(output) {
            ensure!(destination == (note.start..note.end), "Owner transport does not identify the physical occurrence");
            ensure!(without_beams(&original.text) == without_beams(&note.text),
                "Representation changed a conserved occurrence obligation");
            occurrence.note = note.text;
        }
        next.snapshot = xml.to_owned();
        next.validate_binding(xml)?;
        Ok(next)
    }

    pub(super) fn receive(seed: &MidiSeedRhythmicCourses, xml: &str) -> Result<Self> {
        let inventory = seed.arrangement_inventory(xml)?;
        let coverage = inventory.courses;
        let origins = inventory.occurrences;
        let mut occurrences = Vec::new();
        let raw = collect_raw_notes(xml);
        ensure!(origins.len() == raw.len(), "Incomplete arrangement input inventory");
        for (origin, note) in origins.into_iter().zip(raw) {
            ensure!(xml.get(origin.bytes).map(str::trim) == Some(note.text.as_str()),
                "Seed origin does not identify the received physical note");
            occurrences.push(CourseOccurrence {
                creation: origin.creation, parent: None, root: origin.creation,
                course: origin.course, interval: origin.interval,
                source: origin.source, note: note.text,
                restified: false, restaffed: false,
            });
        }
        let roots = occurrences.iter().map(|o| (o.creation, o.clone())).collect();
        let history = occurrences.iter().map(|o| (o.creation, o.edge())).collect();
        let state = Self { coverage, roots, history, occurrences, snapshot: xml.to_owned() };
        state.validate_binding(xml)?;
        Ok(state)
    }

    pub(super) fn measure(&self, text: &str, offset: i64, duration: i64) -> Result<CourseMeasure> {
        ensure!(duration > 0, "Unsupported course metric");
        let incoming = self.occurrences.iter().filter(|o| {
            o.interval.start >= offset && o.interval.start < offset + duration
        }).cloned().collect::<Vec<_>>();
        let measure = CourseMeasure {
            coverage: self.coverage.clone(), history: BTreeMap::new(),
            incoming, outgoing: Vec::new(), offset, duration,
        };
        measure.validate_notes(text, &measure.incoming)?;
        ensure!(measure.unrepresented(&measure.incoming)?.is_empty(),
            "Initial arrangement requires complete received coverage");
        Ok(measure)
    }

    pub(super) fn finish(&self, occurrences: Vec<CourseOccurrence>,
        mut history: BTreeMap<RhythmicCreationIdentity, CreationEdge>, xml: &str) -> Result<Self> {
        for (id, edge) in &self.history {
            ensure!(history.get(id).is_none_or(|present| present == edge), "Conflicting creation history");
            history.insert(*id, edge.clone());
        }
        let state = Self {
            coverage: self.coverage.clone(), roots: self.roots.clone(),
            history, occurrences, snapshot: xml.to_owned(),
        };
        state.validate_binding(xml)?;
        Ok(state)
    }

    pub(crate) fn validate_binding(&self, xml: &str) -> Result<()> {
        ensure!(self.snapshot == xml, "Course custody belongs to another document state");
        let mut offset = 0;
        let mut context = RhythmContext::default();
        let mut count = 0;
        let mut identities = self.coverage.iter().map(|c| c.creation).collect::<BTreeSet<_>>();
        ensure!(identities.len() == self.coverage.len(), "Duplicate course creation");
        for occurrence in &self.occurrences {
            ensure!(identities.insert(occurrence.creation), "Duplicate physical creation");
            ensure!(self.history.get(&occurrence.creation) == Some(&occurrence.edge()), "Missing actual creation edge");
            let root = self.roots.get(&occurrence.root).ok_or_else(|| anyhow!("Foreign root origin"))?;
            let mut visited = BTreeSet::new();
            let mut id = occurrence.creation;
            loop {
                ensure!(visited.insert(id), "Cyclic creation ancestry");
                let edge = self.history.get(&id).ok_or_else(|| anyhow!("Missing creation ancestor"))?;
                ensure!(edge.root == root.creation && edge.course == root.course && edge.source == root.source,
                    "Creation ancestry changed course or source");
                if let Some(parent) = edge.parent {
                    let ancestor = self.history.get(&parent).ok_or_else(|| anyhow!("Missing parent creation"))?;
                    ensure!(edge.interval.start >= ancestor.interval.start && edge.interval.end <= ancestor.interval.end,
                        "Creation ancestry extended licensed support");
                    id = parent;
                } else {
                    ensure!(id == root.creation && edge.interval == root.interval, "Foreign creation root");
                    break;
                }
            }
            let root_staff = extract_first_i64(&root.note, "staff");
            let final_staff = extract_first_i64(&occurrence.note, "staff");
            let course = self.coverage.iter().find(|c| c.creation == root.course)
                .ok_or_else(|| anyhow!("Missing original course"))?;
            ensure!(course.presentation == SeedCoursePresentation::Primary && root_staff == Some(i64::from(course.staff)),
                "Original presentation is not the issued seed decision");
            ensure!(if occurrence.restaffed { root.source.is_some() && root_staff == Some(1) && final_staff == Some(2) }
                else { final_staff == root_staff }, "Unbound staff transition");
            ensure!(if occurrence.restified { root.source.is_some() && root.note.contains("<pitch>") && occurrence.note.contains("<rest") }
                else { extract_first_text(&occurrence.note, "pitch") == extract_first_text(&root.note, "pitch") },
                "Unbound sounding disposition change");
        }
        for root in self.roots.values() {
            let mut fragments = self.occurrences.iter().filter(|o| o.root == root.creation).collect::<Vec<_>>();
            fragments.sort_by_key(|o| (o.interval.start, o.interval.end));
            let mut end = root.interval.start;
            for fragment in fragments {
                ensure!(fragment.interval.start == end && fragment.interval.end > end,
                    "Source-origin split lost or duplicated represented support");
                end = fragment.interval.end;
            }
            ensure!(end == root.interval.end, "Source-origin split does not cover its original interval");
        }
        for span in find_measure_spans(xml) {
            let text = &xml[span.start..span.end];
            context = context.update_from_measure(text);
            let measure = self.measure(text, offset, context.expected_duration())?;
            count += measure.incoming.len();
            offset += context.expected_duration();
        }
        ensure!(count == self.occurrences.len(), "Unbound occurrence outside the metric domain");
        ensure!(self.coverage.iter().all(|c| c.coverage.start == 0 && c.coverage.end == offset),
            "Arrangement implicitly changed the declared lifetime");
        Ok(())
    }
}

fn without_beams(note: &str) -> String {
    let mut out = String::new();
    let mut cursor = 0;
    while let Some(relative) = note[cursor..].find("<beam") {
        let start = cursor + relative;
        let Some(relative_end) = note[start..].find("</beam>") else { break; };
        out.push_str(&note[cursor..start]);
        cursor = start + relative_end + "</beam>".len();
    }
    out.push_str(&note[cursor..]);
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

impl CourseMeasure {
    fn validate_notes(&self, text: &str, occurrences: &[CourseOccurrence]) -> Result<()> {
        let raw = collect_raw_notes(text);
        let metas = collect_note_metas(text, &raw);
        ensure!(raw.len() == occurrences.len() && metas.len() == raw.len(),
            "Incomplete physical course continuation");
        for ((note, meta), occurrence) in raw.iter().zip(metas).zip(occurrences) {
            ensure!(note.text == occurrence.note
                && occurrence.interval.start == self.offset + meta.start
                && occurrence.interval.end == self.offset + meta.start + meta.duration,
                "Physical course continuation moved or changed support: expected {:?}, observed {:?}, exact_note={}; {}",
                occurrence.interval, self.offset + meta.start..self.offset + meta.start + meta.duration,
                note.text == occurrence.note,
                crate::audit_only::panic_origin_observation::difference_evidence(&occurrence.note, &note.text));
        }
        Ok(())
    }

    /// Licensed domain minus the complete represented union. Hidden sounding
    /// members, physical rest hosts and source-bound restifications all count.
    /// XML voice labels and visibility never select course membership.
    fn unrepresented(&self, occurrences: &[CourseOccurrence]) -> Result<Vec<Range<i64>>> {
        let mut gaps = Vec::new();
        for course in &self.coverage {
            let lifetime = &course.coverage;
            let begin = lifetime.start.max(self.offset);
            let end = lifetime.end.min(self.offset + self.duration);
            if begin >= end { continue; }
            let mut support = occurrences.iter().filter(|o| o.course == course.creation)
                .map(|o| o.interval.clone()).collect::<Vec<_>>();
            support.sort_by_key(|r| (r.start, r.end));
            let mut cursor = begin;
            for interval in support {
                ensure!(interval.start >= begin && interval.end <= end && interval.start < interval.end,
                    "Occurrence extends beyond its declared course support");
                if interval.start > cursor { gaps.push(cursor..interval.start); }
                cursor = cursor.max(interval.end);
            }
            if cursor < end { gaps.push(cursor..end); }
        }
        ensure!(occurrences.iter().all(|o| self.coverage.iter().any(|c| c.creation == o.course)),
            "Foreign course membership");
        Ok(gaps)
    }

    pub(super) fn disposition(
        &mut self, index: usize, note: &str, action: ArrangementMaterialDispositionKind,
        projected_staff: u8, moved: bool,
    ) -> Result<()> {
        let occurrence = self.incoming.get_mut(index).ok_or_else(|| anyhow!("Missing owner origin"))?;
        let old_staff = extract_first_i64(&occurrence.note, "staff").unwrap_or(0);
        ensure!(note.contains("<chord") == occurrence.note.contains("<chord"),
            "Disposition changed physical host membership");
        ensure!(extract_first_i64(note, "duration") == Some(occurrence.interval.end - occurrence.interval.start),
            "Disposition changed licensed support");
        ensure!(extract_first_i64(note, "staff") == Some(i64::from(projected_staff)),
            "Staff transition does not match the physical action");
        if moved {
            ensure!(occurrence.source.is_some() && old_staff == 1 && projected_staff == 2,
                "Staff transition lacks source-bound owner action");
            occurrence.restaffed = true;
        } else {
            ensure!(old_staff == i64::from(projected_staff), "Implicit course staff transition");
        }
        if action == ArrangementMaterialDispositionKind::RestifiedPitched {
            ensure!(occurrence.source.is_some() && occurrence.note.contains("<pitch>")
                && !occurrence.note.contains("<chord") && note.contains("<rest"),
                "Restification lacks source-bound pitched origin");
            occurrence.parent = Some(occurrence.creation);
            occurrence.creation = RhythmicCreationIdentity::arrangement_fragment(&ArrangementRhythmicMint { _sealed: () })?;
            occurrence.restified = true;
        } else {
            ensure!(extract_first_text(&occurrence.note, "pitch") == extract_first_text(note, "pitch"),
                "Unlicensed disposition change");
        }
        occurrence.note = note.to_owned();
        if occurrence.parent.is_some() { self.history.insert(occurrence.creation, occurrence.edge()); }
        Ok(())
    }

    pub(super) fn normalized(&mut self, index: usize, replacement: &str) -> Result<Vec<CourseOccurrence>> {
        let origin = self.incoming.get(index).ok_or_else(|| anyhow!("Missing normalization origin"))?;
        let parts = collect_raw_notes(replacement);
        ensure!(!parts.is_empty(), "Normalization removed an obligation");
        ensure!(parts.len() == 1 || (!origin.note.contains("<tie") && !origin.note.contains("<grace")
            && !origin.note.contains("<notations")), "Existing ties/notations require their own continuation owner");
        ensure!(parts.iter().map(|n| extract_first_i64(&n.text, "duration").unwrap_or(0)).sum::<i64>()
            == origin.interval.end - origin.interval.start, "Normalization partition changed support");
        let mut cursor = origin.interval.start;
        let mut fragments = Vec::new();
        for (ordinal, note) in parts.iter().enumerate() {
            let duration = extract_first_i64(&note.text, "duration").unwrap_or(0);
            ensure!(duration > 0 && cursor + duration <= origin.interval.end,
                "Split extends source-bound support");
            ensure!(extract_first_i64(&note.text, "staff") == extract_first_i64(&origin.note, "staff")
                && extract_first_text(&note.text, "pitch") == extract_first_text(&origin.note, "pitch")
                && extract_first_text(&note.text, "voice") == extract_first_text(&origin.note, "voice")
                && note.text.contains("<chord") == origin.note.contains("<chord")
                && is_note_block_visible(&note.text) == is_note_block_visible(&origin.note),
                "Normalization changed disposition or staff without an owner action");
            let mut fragment = origin.clone();
            if parts.len() > 1 {
                fragment.parent = Some(origin.creation);
                fragment.creation = RhythmicCreationIdentity::arrangement_fragment(&ArrangementRhythmicMint { _sealed: () })?;
            }
            fragment.interval = cursor..cursor + duration;
            fragment.note = note.text.clone();
            if parts.len() > 1 && fragment.note.contains("<pitch>") {
                ensure!(fragment.note.contains("<tie type=\"start\"/>") == (ordinal + 1 < parts.len())
                    && fragment.note.contains("<tie type=\"stop\"/>") == (ordinal > 0)
                    && fragment.note.contains("<tied type=\"start\"/>") == (ordinal + 1 < parts.len())
                    && fragment.note.contains("<tied type=\"stop\"/>") == (ordinal > 0),
                    "Split changed source attack or tie continuity");
            }
            cursor += duration;
            self.history.insert(fragment.creation, fragment.edge());
            fragments.push(fragment);
        }
        ensure!(cursor == origin.interval.end, "Normalization lost course support");
        Ok(fragments)
    }

    pub(super) fn push_normalized(&mut self, index: usize, replacement: &str) -> Result<()> {
        let fragments = self.normalized(index, replacement)?;
        self.outgoing.extend(fragments);
        Ok(())
    }

    pub(super) fn cohort_support(&self, first: usize, end: usize, parts: &[i64]) -> Result<()> {
        let host = self.incoming.get(first).ok_or_else(|| anyhow!("Missing chord host origin"))?;
        ensure!(end > first && end <= self.incoming.len() && !parts.is_empty()
            && parts.iter().all(|d| *d > 0)
            && parts.iter().sum::<i64>() == host.interval.end - host.interval.start,
            "Unsupported chord host partition");
        for (ordinal, occurrence) in self.incoming[first..end].iter().enumerate() {
            ensure!(occurrence.interval == host.interval && !occurrence.note.contains("<grace")
                && !occurrence.note.contains("<tie")
                && !occurrence.note.contains("<notations")
                && occurrence.note.contains("<chord") == (ordinal > 0),
                "Cohort requires independently proven support and existing-tie continuation");
        }
        Ok(())
    }

    pub(super) fn cohort_normalized(&mut self, first: usize, replacements: &[String]) -> Result<()> {
        let fragments = replacements.iter().enumerate().map(|(member, text)| {
            self.normalized(first + member, text)
        }).collect::<Result<Vec<_>>>()?;
        let count = fragments.first().ok_or_else(|| anyhow!("Empty cohort"))?.len();
        ensure!(fragments.iter().all(|v| v.len() == count), "Incomplete cohort partition");
        for ordinal in 0..count {
            for member in &fragments { self.outgoing.push(member[ordinal].clone()); }
        }
        Ok(())
    }

    pub(super) fn complete(&mut self, text: &str) -> Result<()> {
        self.validate_notes(text, &self.outgoing)?;
        ensure!(self.unrepresented(&self.outgoing)?.is_empty(), "Ledger lost licensed coverage");
        Ok(())
    }

    pub(super) fn take(self) -> (Vec<CourseOccurrence>, BTreeMap<RhythmicCreationIdentity, CreationEdge>) {
        (self.outgoing, self.history)
    }
}

#[cfg(test)]
#[path = "../tests/unit/arrangement_rhythmic_course_continuation.rs"]
mod tests;
