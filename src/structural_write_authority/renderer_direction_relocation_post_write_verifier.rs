//! Independent post-write proof for the authorized renderer-neutral direction
//! relocation.
//!
//! This verifier performs no write. It compares the exact pre/post snapshots,
//! re-derives cursor and staff relations from the written XML, proves that all
//! note/backup/forward elements are byte-identical, and requires the targeted
//! beam members to be renderer-contiguous after the relocation.

#[cfg(test)]
use super::authority_primitives::yes_no;
use super::locator_resolution::stable_xml_snapshot_fingerprint;
use super::renderer_direction_relocation_authorization::{
    canonical_direction_without_offset_fingerprint, direct_child_i64, direct_child_text,
    primary_beam_state, RendererDirectionRelocationVerificationContract,
};
use super::renderer_direction_relocation_writer_boundary::RendererDirectionRelocationWriterAudit;
use roxmltree::{Document, Node};

#[cfg(test)]
const VERIFIER_MODULE_PATH: &str =
    "src/structural_write_authority/renderer_direction_relocation_post_write_verifier.rs";

#[derive(Debug, Clone, Default)]
pub struct RendererDirectionRelocationPostWriteAudit {
    #[cfg(test)]
    source_stage: String,
    #[cfg(test)]
    verifier_module_path: String,
    token_handed_to_writer: bool,
    token_consumed: bool,
    writer_token_used: bool,
    writer_exact_edits_applied: usize,
    writer_write_produced: bool,
    pre_writer_parse_ok: bool,
    post_writer_parse_ok: bool,
    pre_writer_snapshot_fingerprint_match: bool,
    expected_post_snapshot_fingerprint_match: bool,
    note_count_before: usize,
    note_count_after: usize,
    note_xml_sequence_stable: bool,
    timeline_xml_sequence_stable: bool,
    direction_count_before: usize,
    direction_count_after: usize,
    direction_payload_multiset_stable: bool,
    unique_relocated_direction: bool,
    offset_explicit: bool,
    offset_exact: bool,
    host_cursor_bound: bool,
    effective_cursor_stable: bool,
    staff_relation_stable: bool,
    beam_group_start_bound: bool,
    beam_group_end_bound: bool,
    renderer_interstitial_elements: usize,
    renderer_contiguous_target_members: bool,
    exact_authorized_delta_only: bool,
    write_validated: bool,
    #[cfg(test)]
    verdict: String,
}

impl RendererDirectionRelocationPostWriteAudit {
    pub fn green(&self) -> bool {
        self.write_validated
    }

    pub fn renderer_contiguous_target_members(&self) -> bool {
        self.renderer_contiguous_target_members
    }

    pub fn renderer_interstitial_elements(&self) -> usize {
        self.renderer_interstitial_elements
    }

    #[cfg(test)]
    pub fn pipeline_line(&self, project_label: &str) -> String {
        format!(
            "RendererDirectionRelocationPostWriteAudit v{}: Project {}; WriteValidated {}; SourceStage {}; VerifierModule {}; TokenHandedToWriter {}; TokenConsumed {}; WriterTokenUsed {}; WriterExactEditsApplied {}; WriterWriteProduced {}; PreWriterParseOk {}; PostWriterParseOk {}; PreWriterSnapshotFingerprintMatch {}; ExpectedPostSnapshotFingerprintMatch {}; NoteCount {}/{}; NoteXmlSequenceStable {}; TimelineXmlSequenceStable {}; DirectionCount {}/{}; DirectionPayloadMultisetStable {}; UniqueRelocatedDirection {}; OffsetExplicit {}; OffsetExact {}; HostCursorBound {}; EffectiveCursorStable {}; StaffRelationStable {}; BeamGroupStartBound {}; BeamGroupEndBound {}; RendererInterstitialElements {}; RendererContiguousTargetMembers {}; ExactAuthorizedDeltaOnly {}; RenderXmlWrite {}; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            project_label,
            yes_no(self.write_validated),
            self.source_stage,
            self.verifier_module_path,
            yes_no(self.token_handed_to_writer),
            yes_no(self.token_consumed),
            yes_no(self.writer_token_used),
            self.writer_exact_edits_applied,
            yes_no(self.writer_write_produced),
            yes_no(self.pre_writer_parse_ok),
            yes_no(self.post_writer_parse_ok),
            yes_no(self.pre_writer_snapshot_fingerprint_match),
            yes_no(self.expected_post_snapshot_fingerprint_match),
            self.note_count_before,
            self.note_count_after,
            yes_no(self.note_xml_sequence_stable),
            yes_no(self.timeline_xml_sequence_stable),
            self.direction_count_before,
            self.direction_count_after,
            yes_no(self.direction_payload_multiset_stable),
            yes_no(self.unique_relocated_direction),
            yes_no(self.offset_explicit),
            yes_no(self.offset_exact),
            yes_no(self.host_cursor_bound),
            yes_no(self.effective_cursor_stable),
            yes_no(self.staff_relation_stable),
            yes_no(self.beam_group_start_bound),
            yes_no(self.beam_group_end_bound),
            self.renderer_interstitial_elements,
            yes_no(self.renderer_contiguous_target_members),
            yes_no(self.exact_authorized_delta_only),
            yes_no(self.writer_write_produced),
            self.verdict,
        )
    }
}

pub(super) fn audit_renderer_direction_relocation_post_write(
    pre_writer_xml: &str,
    post_writer_xml: &str,
    contract: Option<&RendererDirectionRelocationVerificationContract>,
    writer_audit: &RendererDirectionRelocationWriterAudit,
    _source_stage: &str,
) -> RendererDirectionRelocationPostWriteAudit {
    let mut audit = RendererDirectionRelocationPostWriteAudit {
        #[cfg(test)]
        source_stage: _source_stage.to_string(),
        #[cfg(test)]
        verifier_module_path: VERIFIER_MODULE_PATH.to_string(),
        token_handed_to_writer: writer_audit.token_handed_to_writer,
        token_consumed: writer_audit.token_consumed,
        writer_token_used: writer_audit.writer_token_used,
        writer_exact_edits_applied: writer_audit.exact_edits_applied,
        writer_write_produced: writer_audit.write_produced,
        pre_writer_parse_ok: Document::parse(pre_writer_xml).is_ok(),
        post_writer_parse_ok: Document::parse(post_writer_xml).is_ok(),
        note_count_before: pre_writer_xml.matches("<note").count(),
        note_count_after: post_writer_xml.matches("<note").count(),
        direction_count_before: direction_element_count(pre_writer_xml),
        direction_count_after: direction_element_count(post_writer_xml),
        #[cfg(test)]
        verdict: "renderer_direction_relocation_post_write_not_verified".to_string(),
        ..RendererDirectionRelocationPostWriteAudit::default()
    };
    let Some(contract) = contract else {
        #[cfg(test)]
        {
            audit.verdict = "renderer_direction_relocation_verification_contract_missing".to_string();
        }
        return audit;
    };

    audit.pre_writer_snapshot_fingerprint_match =
        stable_xml_snapshot_fingerprint(pre_writer_xml) == contract.pre_writer_snapshot_fingerprint;
    audit.expected_post_snapshot_fingerprint_match =
        stable_xml_snapshot_fingerprint(post_writer_xml)
            == contract.expected_post_snapshot_fingerprint;
    audit.note_xml_sequence_stable = raw_element_sequence(pre_writer_xml, &["note"])
        == raw_element_sequence(post_writer_xml, &["note"]);
    audit.timeline_xml_sequence_stable =
        raw_element_sequence(pre_writer_xml, &["note", "backup", "forward"])
            == raw_element_sequence(post_writer_xml, &["note", "backup", "forward"]);
    audit.direction_payload_multiset_stable = direction_payload_fingerprints(pre_writer_xml)
        == direction_payload_fingerprints(post_writer_xml);

    let relocated = locate_relocated_direction(post_writer_xml, contract);
    if let Some(relocated) = relocated {
        audit.unique_relocated_direction = true;
        audit.offset_explicit = relocated.offset_explicit;
        audit.offset_exact = relocated.offset == contract.proposed_offset;
        audit.host_cursor_bound = relocated.direction_cursor == contract.proposed_host_cursor
            && relocated.host_cursor_before == contract.group_start_cursor;
        audit.effective_cursor_stable = relocated.direction_cursor.saturating_add(relocated.offset)
            == contract.source_effective_cursor;
        audit.staff_relation_stable = relocated.staff_relation_stable;
        audit.beam_group_start_bound = relocated.host_cursor_after
            == contract.source_effective_cursor
            && relocated.host_beam_begin
            && relocated.right_beam_continue;
        audit.beam_group_end_bound =
            relocated.last_cursor_after == contract.group_end_cursor && relocated.last_beam_end;
        audit.renderer_interstitial_elements = relocated.interstitial_elements;
        audit.renderer_contiguous_target_members = relocated.interstitial_elements
            == contract.expected_interstitial_elements_after_relocation;
    }

    audit.exact_authorized_delta_only = audit.expected_post_snapshot_fingerprint_match
        && audit.note_xml_sequence_stable
        && audit.timeline_xml_sequence_stable
        && audit.direction_count_before == audit.direction_count_after
        && audit.direction_payload_multiset_stable;
    audit.write_validated = audit.token_handed_to_writer
        && audit.token_consumed
        && audit.writer_token_used
        && audit.writer_exact_edits_applied == 2
        && audit.writer_write_produced
        && audit.pre_writer_parse_ok
        && audit.post_writer_parse_ok
        && audit.pre_writer_snapshot_fingerprint_match
        && audit.note_count_before == audit.note_count_after
        && audit.unique_relocated_direction
        && audit.offset_explicit
        && audit.offset_exact
        && audit.host_cursor_bound
        && audit.effective_cursor_stable
        && audit.staff_relation_stable
        && audit.beam_group_start_bound
        && audit.beam_group_end_bound
        && audit.renderer_contiguous_target_members
        && audit.exact_authorized_delta_only;
    #[cfg(test)]
    {
        audit.verdict = if audit.write_validated {
            "authorized_renderer_neutral_direction_relocation_post_write_green".to_string()
        } else {
            "renderer_direction_relocation_post_write_quarantined_by_invariant".to_string()
        };
    }
    audit
}

#[derive(Debug)]
struct RelocatedDirectionEvidence {
    direction_cursor: i64,
    offset: i64,
    offset_explicit: bool,
    host_cursor_before: i64,
    host_cursor_after: i64,
    last_cursor_after: i64,
    staff_relation_stable: bool,
    host_beam_begin: bool,
    right_beam_continue: bool,
    last_beam_end: bool,
    interstitial_elements: usize,
}

fn locate_relocated_direction(
    xml: &str,
    contract: &RendererDirectionRelocationVerificationContract,
) -> Option<RelocatedDirectionEvidence> {
    let document = Document::parse(xml).ok()?;
    let mut matches = Vec::new();
    for measure in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("measure"))
    {
        let mut cursor = 0_i64;
        let mut records = Vec::new();
        for child in measure.children().filter(Node::is_element) {
            let before = cursor;
            if child.has_tag_name("backup") {
                cursor -= direct_child_i64(child, "duration").unwrap_or(0);
            } else if child.has_tag_name("forward") {
                cursor += direct_child_i64(child, "duration").unwrap_or(0);
            } else if child.has_tag_name("note") {
                let duration = direct_child_i64(child, "duration").unwrap_or(0).max(0);
                let chord = child
                    .children()
                    .any(|node| node.is_element() && node.has_tag_name("chord"));
                if !chord {
                    cursor += duration;
                }
            }
            records.push((child, before, cursor));
        }
        if records.len() < 4 {
            continue;
        }
        for index in 0..records.len().saturating_sub(3) {
            let (direction, direction_cursor, _) = records[index];
            if !direction.has_tag_name("direction") {
                continue;
            }
            let Some(raw) = xml.get(direction.range()) else {
                continue;
            };
            if canonical_direction_without_offset_fingerprint(raw)
                != Some(contract.canonical_payload_without_offset_fingerprint)
            {
                continue;
            }
            let offset = direct_child_i64(direction, "offset").unwrap_or(0);
            let offset_explicit = direction
                .children()
                .any(|node| node.is_element() && node.has_tag_name("offset"));
            let (host, host_before, host_after) = records[index + 1];
            let (right, right_before, right_after) = records[index + 2];
            let (last, last_before, last_after) = records[index + 3];
            if !host.has_tag_name("note")
                || !right.has_tag_name("note")
                || !last.has_tag_name("note")
                || right_before != host_after
                || last_before != right_after
            {
                continue;
            }
            let direction_staff = direct_child_text(direction, "staff");
            let host_staff = direct_child_text(host, "staff");
            let right_staff = direct_child_text(right, "staff");
            matches.push(RelocatedDirectionEvidence {
                direction_cursor,
                offset,
                offset_explicit,
                host_cursor_before: host_before,
                host_cursor_after: host_after,
                last_cursor_after: last_after,
                staff_relation_stable: direction_staff.is_some()
                    && direction_staff == right_staff
                    && direction_staff != host_staff,
                host_beam_begin: primary_beam_state(host).as_deref() == Some("begin"),
                right_beam_continue: primary_beam_state(right).as_deref() == Some("continue"),
                last_beam_end: primary_beam_state(last).as_deref() == Some("end"),
                interstitial_elements: 0,
            });
        }
    }
    (matches.len() == 1).then(|| matches.remove(0))
}

fn raw_element_sequence(xml: &str, tags: &[&str]) -> Option<Vec<String>> {
    let document = Document::parse(xml).ok()?;
    Some(
        document
            .descendants()
            .filter(|node| node.is_element() && tags.iter().any(|tag| node.has_tag_name(*tag)))
            .filter_map(|node| xml.get(node.range()).map(str::to_string))
            .collect(),
    )
}

fn direction_payload_fingerprints(xml: &str) -> Option<Vec<u64>> {
    let document = Document::parse(xml).ok()?;
    let mut fingerprints = document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("direction"))
        .filter_map(|node| xml.get(node.range()))
        .filter_map(canonical_direction_without_offset_fingerprint)
        .collect::<Vec<_>>();
    fingerprints.sort_unstable();
    Some(fingerprints)
}

fn direction_element_count(xml: &str) -> usize {
    Document::parse(xml)
        .map(|document| {
            document
                .descendants()
                .filter(|node| node.is_element() && node.has_tag_name("direction"))
                .count()
        })
        .unwrap_or(0)
}


