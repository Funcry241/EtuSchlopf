//! Evidence of new E050 connections, captured from its exact original execution.
//! This is separate from extension/withdrawal custody: no positive old origin
//! is invented for a connection that E050 has just authored after projection.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AuthoredConnection {
    pub(super) members: Vec<Member>,
}

impl CanonicalFinalChordStructuralPlan {
    pub(super) fn capture_authored_connections(
        &self,
        input: &str,
        output: &str,
        selected: &[Vec<(usize, usize)>],
        offered: &[bool],
        extensions: &BTreeSet<usize>,
    ) -> io::Result<Vec<AuthoredConnection>> {
        let guard = self
            .bridge_voice_guard
            .as_ref()
            .ok_or_else(|| invalid("Authored connection lacks its original guard"))?;
        if !self.bridge_is_post_projection() || !guard.matches_input(input) {
            return Err(invalid(
                "Authored connection is not bound to this projection",
            ));
        }
        let before = notes(input)?;
        let after = notes(output)?;
        let bindings = guard.occurrence_bindings();
        let mut result = Vec::new();
        let mut claimed = BTreeSet::new();
        for (index, heads) in selected.iter().enumerate() {
            // Split-lane offers retain the existing extension/withdrawal path.
            if extensions.contains(&index)
                || offered.get(index).copied().unwrap_or(false)
                || self.bridge_voice_admission(input, heads) != Some(true)
            {
                continue;
            }
            let selected_heads = heads.iter().copied().collect::<BTreeSet<_>>();
            if selected_heads.len() != heads.len() || heads.len() < 2 {
                return Err(invalid(
                    "Authored connection has duplicate or incomplete heads",
                ));
            }
            let Ok(positions) = complete_resource_image(
                &before,
                bindings,
                &self.source_locators,
                &selected_heads,
                &BTreeSet::new(),
            ) else {
                continue;
            };
            let mut members = Vec::new();
            let mut complete = true;
            for position in positions {
                let note = &before[&position];
                let key = (note.range.start, note.range.end);
                let Some(identity) = bindings.get(&key).copied() else {
                    complete = false;
                    break;
                };
                let Some(label) = guard.occurrence_labels().get(&key) else {
                    complete = false;
                    break;
                };
                let Some(post) = after.get(&position) else {
                    complete = false;
                    break;
                };
                let head = selected_heads.contains(&key);
                let prior = beams(input, note)?;
                let written = beams(output, post)?;
                // Only newly written connections qualify. Existing beam origins
                // are never reconstructed from the output or silently replaced.
                if head && (note.chord_member || prior.iter().any(|(level, _)| *level <= 2)) {
                    complete = false;
                    break;
                }
                let anchor = if note.chord_member {
                    before
                        .values()
                        .find(|n| {
                            n.measure == note.measure && Some(n.raw_note_index) == note.anchor_index
                        })
                        .and_then(|n| bindings.get(&(n.range.start, n.range.end)))
                        .copied()
                } else {
                    None
                };
                if note.chord_member && anchor.is_none() {
                    complete = false;
                    break;
                }
                let member = Member {
                    identity,
                    position,
                    original: false,
                    anchor,
                    head,
                    onset: note.onset,
                    duration: note.duration,
                    pitch: note.pitch_midi,
                    staff: note.staff.clone(),
                    voice: note.voice.clone(),
                    sealed_voice: label.clone(),
                    chord: note.chord_member,
                    tie_start: note.tie_start,
                    tie_stop: note.tie_stop,
                    before: prior,
                    written,
                };
                if !member.matches(output, post, false) {
                    complete = false;
                    break;
                }
                members.push(member);
            }
            let written_heads = members.iter().filter(|m| m.head).collect::<Vec<_>>();
            if !complete
                || written_heads.len() != heads.len()
                || written_heads
                    .iter()
                    .any(|m| m.sealed_voice != written_heads[0].sealed_voice)
                || !resource_attachments_match(&members.iter().collect::<Vec<_>>(), &after)
            {
                continue;
            }
            if written_heads.iter().enumerate().any(|(i, m)| {
                m.written
                    .iter()
                    .find(|(level, _)| *level == 1)
                    .map(|(_, s)| s.as_str())
                    != Some(if i == 0 {
                        "begin"
                    } else if i + 1 == written_heads.len() {
                        "end"
                    } else {
                        "continue"
                    })
            }) {
                continue;
            }
            if members.iter().any(|m| !claimed.insert(m.identity)) {
                return Err(invalid("Authored connection operations overlap"));
            }
            result.push(AuthoredConnection { members });
        }
        Ok(result)
    }
}
