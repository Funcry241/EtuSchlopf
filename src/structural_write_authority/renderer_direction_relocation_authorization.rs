//! Independent capability authorization for the already proven renderer-neutral
//! direction relocation plan.
//!
//! This module may consume exactly one proven, identity-free plan and bind it to
//! the exact pre-relocation writer snapshot. It issues a private, non-clonable
//! capability, resolves an exact byte-bound write instruction against the
//! pre-relocation snapshot, and hands the non-clonable capability exactly once
//! to the renderer-neutral writer in v0.10.1163.

#[cfg(test)]
use super::authority_primitives::yes_no;
use super::locator_resolution::{direction_without_offset, stable_xml_snapshot_fingerprint};
use super::renderer_direction_relocation_plan::{
    ProvenRendererDirectionRelocationPlan, RendererDirectionRelocationAuthorizationParts,
};
use roxmltree::{Document, Node};
use std::{fmt::Write as _, ops::Range};

const VERIFIER_MODULE_SOURCE: &str = include_str!("renderer_direction_relocation_authorization.rs");
#[cfg(test)]
const VERIFIER_MODULE_PATH: &str =
    "src/structural_write_authority/renderer_direction_relocation_authorization.rs";
const WRITER_MODULE_SOURCE: &str = include_str!("renderer_direction_relocation_writer_boundary.rs");
#[cfg(test)]
const WRITER_MODULE_PATH: &str =
    "src/structural_write_authority/renderer_direction_relocation_writer_boundary.rs";
const AUTHORITY_MODULE_SOURCE: &str = include_str!("mod.rs");
const DETECTOR_MODULE_SOURCE: &str = include_str!("detector.rs");
const PLANNER_MODULE_SOURCE: &str = include_str!("renderer_direction_relocation_plan.rs");

const TOKEN_STRUCT_DECLARATION_PATTERN: &str =
    concat!("struct AuthorizedRendererDirection", "Relocation {");
const TOKEN_CONSTRUCTOR_PATTERN: &str =
    concat!("Ok((\n        AuthorizedRendererDirection", "Relocation {");
const PRIVATE_ISSUE_FUNCTION_PATTERN: &str =
    concat!("fn issue_authorized_renderer_direction_", "relocation(");
const TOKEN_CLONE_DERIVE_PATTERN: &str = concat!(
    "#[derive(Debug, Clone)]\npub(super) struct AuthorizedRendererDirection",
    "Relocation"
);
const TOKEN_CLONE_CALL_PATTERN: &str = concat!("token", ".clone()");
const WRITER_ENTRY_DEFINITION_PATTERN: &str = concat!(
    "fn authorized_renderer_direction_",
    "relocation_writer_entry("
);
const WRITER_ENTRY_TOKEN_PARAMETER_PATTERN: &str =
    concat!("token: AuthorizedRendererDirection", "Relocation");
const RAW_PLAN_WRITER_PARAMETER_PATTERN: &str =
    concat!("plan: ProvenRendererDirection", "RelocationPlan");
const WRITER_ENTRY_CALL_PATTERN: &str =
    concat!("authorized_renderer_direction_", "relocation_writer_entry(");
const TOKEN_CONSTRUCTOR_IN_WRITER_PATTERN: &str =
    concat!("AuthorizedRendererDirection", "Relocation {");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectionRelocationAuthorizationBlock {
    PlanBindingIncomplete,
    PreWriterSnapshotInvalid,
}

/// Private capability for the direction-relocation writer. The fields and only
/// constructor stay in this independent verifier. It intentionally does not
/// implement `Clone` or `Copy`.
#[derive(Debug)]
pub(super) struct AuthorizedRendererDirectionRelocation {
    source_snapshot_fingerprint: u64,
    pre_writer_snapshot_fingerprint: u64,
    authorized_beam_group_ordinal: usize,
    authorized_member_pair_ordinal: usize,
    unique_obstacle_element_ordinal: usize,
    group_start_cursor: i64,
    group_end_cursor: i64,
    source_element_cursor: i64,
    source_offset: i64,
    source_offset_explicit: bool,
    source_effective_cursor: i64,
    proposed_host_cursor: i64,
    proposed_offset: i64,
    direction_type_element_count: usize,
    dynamics_mark_count: usize,
    staff_relation_to_right_member_stable: bool,
    payload_without_offset_fingerprint: u64,
    sound_absent: bool,
    timing_spanner_absent: bool,
    words_or_metronome_absent: bool,
    default_x_absent: bool,
    expected_interstitial_elements_after_relocation: usize,
    predicted_renderer_contiguity_green: bool,
    direction_range: Range<usize>,
    host_insert_at: usize,
    insertion_text: String,
    original_direction_xml_fingerprint: u64,
    expected_post_snapshot_fingerprint: u64,
}

#[derive(Debug)]
pub(super) struct RendererDirectionRelocationWriterInstruction {
    pub(super) direction_range: Range<usize>,
    pub(super) host_insert_at: usize,
    pub(super) insertion_text: String,
    pub(super) expected_post_snapshot_fingerprint: u64,
    pub(super) canonical_payload_without_offset_fingerprint: u64,
    pub(super) original_direction_xml_fingerprint: u64,
    pub(super) proposed_host_cursor: i64,
    pub(super) source_effective_cursor: i64,
    pub(super) proposed_offset: i64,
    pub(super) group_end_cursor: i64,
}

#[derive(Debug, Clone)]
pub(super) struct RendererDirectionRelocationVerificationContract {
    pub(super) pre_writer_snapshot_fingerprint: u64,
    pub(super) expected_post_snapshot_fingerprint: u64,
    pub(super) canonical_payload_without_offset_fingerprint: u64,
    pub(super) group_start_cursor: i64,
    pub(super) group_end_cursor: i64,
    pub(super) source_effective_cursor: i64,
    pub(super) proposed_host_cursor: i64,
    pub(super) proposed_offset: i64,
    pub(super) expected_interstitial_elements_after_relocation: usize,
}

#[derive(Debug)]
struct ResolvedPreWriterDirectionRelocation {
    direction_range: Range<usize>,
    host_insert_at: usize,
    insertion_text: String,
    original_direction_xml_fingerprint: u64,
    expected_post_snapshot_fingerprint: u64,
    canonical_payload_without_offset_fingerprint: u64,
}

impl AuthorizedRendererDirectionRelocation {
    fn binding_green(&self) -> bool {
        self.source_snapshot_fingerprint != 0
            && self.pre_writer_snapshot_fingerprint != 0
            && self.source_snapshot_fingerprint != self.pre_writer_snapshot_fingerprint
            && self.authorized_beam_group_ordinal < usize::MAX
            && self.authorized_member_pair_ordinal < usize::MAX
            && self.unique_obstacle_element_ordinal < usize::MAX
            && self.group_start_cursor < self.group_end_cursor
            && self.source_element_cursor == self.source_effective_cursor
            && self.source_offset == 0
            && !self.source_offset_explicit
            && self.proposed_host_cursor == self.group_start_cursor
            && self.proposed_offset > 0
            && self
                .proposed_host_cursor
                .saturating_add(self.proposed_offset)
                == self.source_effective_cursor
            && self.source_effective_cursor < self.group_end_cursor
            && self.direction_type_element_count == 1
            && self.dynamics_mark_count == 1
            && self.staff_relation_to_right_member_stable
            && self.payload_without_offset_fingerprint != 0
            && self.sound_absent
            && self.timing_spanner_absent
            && self.words_or_metronome_absent
            && self.default_x_absent
            && self.expected_interstitial_elements_after_relocation == 0
            && self.predicted_renderer_contiguity_green
            && self.direction_range.start < self.direction_range.end
            && self.host_insert_at < self.direction_range.start
            && !self.insertion_text.is_empty()
            && self.original_direction_xml_fingerprint != 0
            && self.expected_post_snapshot_fingerprint != 0
    }

    pub(super) fn into_writer_instruction(
        self,
        source_xml: &str,
    ) -> Option<RendererDirectionRelocationWriterInstruction> {
        if !self.binding_green()
            || stable_xml_snapshot_fingerprint(source_xml) != self.pre_writer_snapshot_fingerprint
            || self.direction_range.end > source_xml.len()
            || self.host_insert_at > source_xml.len()
        {
            return None;
        }
        let direction_xml = source_xml.get(self.direction_range.clone())?;
        if stable_xml_snapshot_fingerprint(direction_xml) != self.original_direction_xml_fingerprint
        {
            return None;
        }
        Some(RendererDirectionRelocationWriterInstruction {
            direction_range: self.direction_range,
            host_insert_at: self.host_insert_at,
            insertion_text: self.insertion_text,
            expected_post_snapshot_fingerprint: self.expected_post_snapshot_fingerprint,
            canonical_payload_without_offset_fingerprint: self.payload_without_offset_fingerprint,
            original_direction_xml_fingerprint: self.original_direction_xml_fingerprint,
            proposed_host_cursor: self.proposed_host_cursor,
            source_effective_cursor: self.source_effective_cursor,
            proposed_offset: self.proposed_offset,
            group_end_cursor: self.group_end_cursor,
        })
    }
}

fn issue_authorized_renderer_direction_relocation(
    parts: RendererDirectionRelocationAuthorizationParts,
    pre_writer_snapshot_fingerprint: u64,
    resolved: ResolvedPreWriterDirectionRelocation,
) -> Result<
    (
        AuthorizedRendererDirectionRelocation,
        RendererDirectionRelocationVerificationContract,
    ),
    DirectionRelocationAuthorizationBlock,
> {
    let plan_binding_green = parts.source_snapshot_fingerprint != 0
        && parts.group_start_cursor < parts.group_end_cursor
        && parts.source_element_cursor == parts.source_effective_cursor
        && parts.source_offset == 0
        && !parts.source_offset_explicit
        && parts.proposed_host_cursor == parts.group_start_cursor
        && parts.proposed_offset > 0
        && parts
            .proposed_host_cursor
            .saturating_add(parts.proposed_offset)
            == parts.source_effective_cursor
        && parts.source_effective_cursor < parts.group_end_cursor
        && parts.direction_type_element_count == 1
        && parts.dynamics_mark_count == 1
        && parts.staff_relation_to_right_member_stable
        && parts.payload_without_offset_fingerprint != 0
        && parts.sound_absent
        && parts.timing_spanner_absent
        && parts.words_or_metronome_absent
        && parts.default_x_absent
        && parts.expected_interstitial_elements_after_relocation == 0
        && parts.predicted_renderer_contiguity_green;
    if !plan_binding_green {
        return Err(DirectionRelocationAuthorizationBlock::PlanBindingIncomplete);
    }
    if pre_writer_snapshot_fingerprint == 0
        || resolved.direction_range.start >= resolved.direction_range.end
        || resolved.host_insert_at >= resolved.direction_range.start
        || resolved.insertion_text.is_empty()
        || resolved.original_direction_xml_fingerprint == 0
        || resolved.expected_post_snapshot_fingerprint == 0
        || resolved.canonical_payload_without_offset_fingerprint == 0
    {
        return Err(DirectionRelocationAuthorizationBlock::PreWriterSnapshotInvalid);
    }

    let contract = RendererDirectionRelocationVerificationContract {
        pre_writer_snapshot_fingerprint,
        expected_post_snapshot_fingerprint: resolved.expected_post_snapshot_fingerprint,
        canonical_payload_without_offset_fingerprint: resolved
            .canonical_payload_without_offset_fingerprint,
        group_start_cursor: parts.group_start_cursor,
        group_end_cursor: parts.group_end_cursor,
        source_effective_cursor: parts.source_effective_cursor,
        proposed_host_cursor: parts.proposed_host_cursor,
        proposed_offset: parts.proposed_offset,
        expected_interstitial_elements_after_relocation: parts
            .expected_interstitial_elements_after_relocation,
    };

    Ok((
        AuthorizedRendererDirectionRelocation {
            source_snapshot_fingerprint: parts.source_snapshot_fingerprint,
            pre_writer_snapshot_fingerprint,
            authorized_beam_group_ordinal: parts.authorized_beam_group_ordinal,
            authorized_member_pair_ordinal: parts.authorized_member_pair_ordinal,
            unique_obstacle_element_ordinal: parts.unique_obstacle_element_ordinal,
            group_start_cursor: parts.group_start_cursor,
            group_end_cursor: parts.group_end_cursor,
            source_element_cursor: parts.source_element_cursor,
            source_offset: parts.source_offset,
            source_offset_explicit: parts.source_offset_explicit,
            source_effective_cursor: parts.source_effective_cursor,
            proposed_host_cursor: parts.proposed_host_cursor,
            proposed_offset: parts.proposed_offset,
            direction_type_element_count: parts.direction_type_element_count,
            dynamics_mark_count: parts.dynamics_mark_count,
            staff_relation_to_right_member_stable: parts.staff_relation_to_right_member_stable,
            payload_without_offset_fingerprint:
                resolved.canonical_payload_without_offset_fingerprint,
            sound_absent: parts.sound_absent,
            timing_spanner_absent: parts.timing_spanner_absent,
            words_or_metronome_absent: parts.words_or_metronome_absent,
            default_x_absent: parts.default_x_absent,
            expected_interstitial_elements_after_relocation: parts
                .expected_interstitial_elements_after_relocation,
            predicted_renderer_contiguity_green: parts.predicted_renderer_contiguity_green,
            direction_range: resolved.direction_range,
            host_insert_at: resolved.host_insert_at,
            insertion_text: resolved.insertion_text,
            original_direction_xml_fingerprint: resolved.original_direction_xml_fingerprint,
            expected_post_snapshot_fingerprint: resolved.expected_post_snapshot_fingerprint,
        },
        contract,
    ))
}

fn resolve_pre_writer_direction_relocation(
    pre_writer_xml: &str,
    parts: &RendererDirectionRelocationAuthorizationParts,
) -> Option<ResolvedPreWriterDirectionRelocation> {
    let document = Document::parse(pre_writer_xml).ok()?;
    let mut matches = Vec::new();
    for measure in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("measure"))
    {
        let mut cursor = 0_i64;
        let mut records = Vec::new();
        for child in measure.children().filter(Node::is_element) {
            let cursor_before = cursor;
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
            records.push((child, cursor_before, cursor));
        }
        if records.len() < 4 {
            continue;
        }
        for index in 1..records.len().saturating_sub(2) {
            let (direction, direction_cursor, _) = records[index];
            if !direction.has_tag_name("direction") {
                continue;
            }
            let direction_range = direction.range();
            let Some(direction_xml) = pre_writer_xml.get(direction_range.clone()) else {
                continue;
            };
            if stable_xml_snapshot_fingerprint(&direction_without_offset(direction_xml))
                != parts.payload_without_offset_fingerprint
                || direct_child_i64(direction, "offset").unwrap_or(0) != parts.source_offset
                || direction
                    .children()
                    .any(|node| node.is_element() && node.has_tag_name("offset"))
                || direction_cursor != parts.source_element_cursor
            {
                continue;
            }
            let (host, host_before, host_after) = records[index - 1];
            let (right, right_before, right_after) = records[index + 1];
            let (last, last_before, last_after) = records[index + 2];
            if !host.has_tag_name("note")
                || !right.has_tag_name("note")
                || !last.has_tag_name("note")
                || host_before != parts.proposed_host_cursor
                || host_after != parts.source_effective_cursor
                || right_before != parts.source_effective_cursor
                || right_after <= right_before
                || last_before != right_after
                || last_after != parts.group_end_cursor
                || primary_beam_state(host).as_deref() != Some("begin")
                || primary_beam_state(right).as_deref() != Some("continue")
                || primary_beam_state(last).as_deref() != Some("end")
            {
                continue;
            }
            let direction_staff = direct_child_text(direction, "staff");
            let host_staff = direct_child_text(host, "staff");
            let right_staff = direct_child_text(right, "staff");
            if direction_staff.is_none()
                || direction_staff != right_staff
                || direction_staff == host_staff
                || direction.descendants().any(|node| {
                    node.is_element()
                        && matches!(
                            node.tag_name().name(),
                            "sound"
                                | "wedge"
                                | "octave-shift"
                                | "pedal"
                                | "dashes"
                                | "bracket"
                                | "words"
                                | "metronome"
                                | "rehearsal"
                                | "segno"
                                | "coda"
                                | "other-direction"
                        )
                })
                || direction
                    .descendants()
                    .any(|node| node.is_element() && node.attribute("default-x").is_some())
            {
                continue;
            }
            let direction_type_element_count = direction
                .children()
                .filter(|node| node.is_element() && node.has_tag_name("direction-type"))
                .flat_map(|node| node.children().filter(Node::is_element))
                .count();
            let dynamics_mark_count = direction
                .descendants()
                .filter(|node| {
                    node.is_element()
                        && node
                            .parent()
                            .is_some_and(|parent| parent.is_element() && parent.has_tag_name("dynamics"))
                })
                .count();
            if direction_type_element_count != parts.direction_type_element_count
                || dynamics_mark_count != parts.dynamics_mark_count
            {
                continue;
            }
            let relocated_direction = direction_with_offset(direction_xml, parts.proposed_offset)?;
            let host_range = host.range();
            let host_indent_start = pre_writer_xml[..host_range.start]
                .rfind('\n')
                .map_or(0, |index| index + 1);
            let host_indent = &pre_writer_xml[host_indent_start..host_range.start];
            let insertion_text = format!("{relocated_direction}\n{host_indent}");
            let expected_post = apply_bound_text_edits(
                pre_writer_xml,
                direction_range.clone(),
                host_range.start,
                &insertion_text,
            )?;
            matches.push(ResolvedPreWriterDirectionRelocation {
                direction_range,
                host_insert_at: host_range.start,
                insertion_text,
                original_direction_xml_fingerprint: stable_xml_snapshot_fingerprint(direction_xml),
                expected_post_snapshot_fingerprint: stable_xml_snapshot_fingerprint(&expected_post),
                canonical_payload_without_offset_fingerprint:
                    canonical_direction_without_offset_fingerprint(direction_xml)?,
            });
        }
    }
    (matches.len() == 1).then(|| matches.remove(0))
}

fn direction_with_offset(direction_xml: &str, offset: i64) -> Option<String> {
    let document = Document::parse(direction_xml).ok()?;
    let direction = document.root_element();
    let staff = direction
        .children()
        .find(|node| node.is_element() && node.has_tag_name("staff"))?;
    let insert_at = staff.range().start;
    let indent_start = direction_xml[..insert_at]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let indent = &direction_xml[indent_start..insert_at];
    let mut relocated = direction_xml.to_string();
    relocated.insert_str(insert_at, &format!("<offset>{offset}</offset>\n{indent}"));
    Some(relocated)
}

fn apply_bound_text_edits(
    source_xml: &str,
    direction_range: Range<usize>,
    host_insert_at: usize,
    insertion_text: &str,
) -> Option<String> {
    if host_insert_at >= direction_range.start
        || direction_range.end > source_xml.len()
        || host_insert_at > source_xml.len()
    {
        return None;
    }
    let mut out = String::with_capacity(source_xml.len() + insertion_text.len());
    out.push_str(source_xml.get(..host_insert_at)?);
    out.push_str(insertion_text);
    out.push_str(source_xml.get(host_insert_at..direction_range.start)?);
    out.push_str(source_xml.get(direction_range.end..)?);
    Some(out)
}

pub(super) fn canonical_direction_without_offset_fingerprint(direction_xml: &str) -> Option<u64> {
    let document = Document::parse(direction_xml).ok()?;
    let direction = document.root_element();
    if !direction.has_tag_name("direction") {
        return None;
    }
    let mut canonical = String::new();
    append_canonical_element_without_offset(direction, &mut canonical);
    Some(stable_xml_snapshot_fingerprint(&canonical))
}

fn append_canonical_element_without_offset(node: Node<'_, '_>, out: &mut String) {
    if node.has_tag_name("offset") {
        return;
    }
    let name = node.tag_name().name();
    let _ = write!(out, "<{name}:{}", name.len());
    let mut attributes = node
        .attributes()
        .map(|attribute| {
            (
                attribute.namespace().unwrap_or(""),
                attribute.name(),
                attribute.value(),
            )
        })
        .collect::<Vec<_>>();
    attributes.sort_unstable();
    for (namespace, attribute_name, value) in attributes {
        let _ = write!(
            out,
            "|a{}:{}:{}:{}:{}:{}",
            namespace.len(),
            namespace,
            attribute_name.len(),
            attribute_name,
            value.len(),
            value,
        );
    }
    out.push('>');
    for child in node.children() {
        if child.is_element() {
            append_canonical_element_without_offset(child, out);
        } else if child.is_text() {
            let text = child.text().unwrap_or("").trim();
            if !text.is_empty() {
                let _ = write!(out, "|t{}:{text}", text.len());
            }
        }
    }
    let _ = write!(out, "</{name}>");
}

pub(super) fn direct_child_i64(node: Node<'_, '_>, tag: &str) -> Option<i64> {
    direct_child_text(node, tag)?.parse().ok()
}

pub(super) fn direct_child_text(node: Node<'_, '_>, tag: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.has_tag_name(tag))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

pub(super) fn primary_beam_state(note: Node<'_, '_>) -> Option<String> {
    note.children()
        .find(|child| {
            child.is_element()
                && child.has_tag_name("beam")
                && child.attribute("number").unwrap_or("1") == "1"
        })
        .and_then(|beam| beam.text())
        .map(str::trim)
        .map(str::to_string)
}

#[derive(Debug, Clone, Default)]
pub struct RendererDirectionRelocationAuthorizationBoundaryAudit {
    #[cfg(test)]
    source_stage: String,
    #[cfg(test)]
    verifier_module_path: String,
    #[cfg(test)]
    writer_module_path: String,
    #[cfg(test)]
    proven_plan_received: bool,
    #[cfg(test)]
    main_candidate_written: bool,
    #[cfg(test)]
    source_snapshot_parse_ok: bool,
    #[cfg(test)]
    pre_writer_snapshot_parse_ok: bool,
    source_snapshot_fingerprint_bound: bool,
    pre_writer_snapshot_fingerprint_bound: bool,
    snapshot_transition_bound: bool,
    authorized_beam_group_bound: bool,
    unique_direction_obstacle_bound: bool,
    payload_without_offset_fingerprint_bound: bool,
    source_cursor_bound: bool,
    host_cursor_bound: bool,
    positive_offset_bound: bool,
    effective_cursor_stable: bool,
    staff_relation_bound: bool,
    exclusion_set_bound: bool,
    expected_interstitial_zero_bound: bool,
    authorization_attempted: bool,
    authorization_granted: usize,
    tokens_issued: usize,
    #[cfg(test)]
    token_constructor_sites: usize,
    #[cfg(test)]
    token_fields_private: bool,
    #[cfg(test)]
    private_issue_function: bool,
    #[cfg(test)]
    token_clone_paths: usize,
    #[cfg(test)]
    token_forgery_paths: usize,
    #[cfg(test)]
    token_origin: String,
    #[cfg(test)]
    writer_entry_definitions: usize,
    #[cfg(test)]
    writer_entry_token_parameter_sites: usize,
    #[cfg(test)]
    raw_plan_writer_entry_points: usize,
    #[cfg(test)]
    token_constructor_sites_in_writer: usize,
    #[cfg(test)]
    authority_writer_call_sites: usize,
    #[cfg(test)]
    detector_writer_call_sites: usize,
    #[cfg(test)]
    planner_writer_call_sites: usize,
    #[cfg(test)]
    writer_input_authorized_token_only: bool,
    token_handed_to_writer: bool,
    token_consumed: bool,
    tokens_dropped_without_writer: usize,
    writer_token_used: bool,
    write_produced: bool,
    render_xml_write: bool,
    #[cfg(test)]
    verdict: String,
}

impl RendererDirectionRelocationAuthorizationBoundaryAudit {
    #[cfg(test)]
    pub fn green(&self) -> bool {
        self.proven_plan_received
            && self.main_candidate_written
            && self.source_snapshot_parse_ok
            && self.pre_writer_snapshot_parse_ok
            && self.source_snapshot_fingerprint_bound
            && self.pre_writer_snapshot_fingerprint_bound
            && self.snapshot_transition_bound
            && self.authorized_beam_group_bound
            && self.unique_direction_obstacle_bound
            && self.payload_without_offset_fingerprint_bound
            && self.source_cursor_bound
            && self.host_cursor_bound
            && self.positive_offset_bound
            && self.effective_cursor_stable
            && self.staff_relation_bound
            && self.exclusion_set_bound
            && self.expected_interstitial_zero_bound
            && self.authorization_attempted
            && self.authorization_granted == 1
            && self.tokens_issued == 1
            && self.token_constructor_sites == 1
            && self.token_fields_private
            && self.private_issue_function
            && self.token_clone_paths == 0
            && self.token_forgery_paths == 0
            && self.token_origin == "independent_verifier_only"
            && self.writer_entry_definitions == 1
            && self.writer_entry_token_parameter_sites == 1
            && self.raw_plan_writer_entry_points == 0
            && self.token_constructor_sites_in_writer == 0
            && self.authority_writer_call_sites == 1
            && self.detector_writer_call_sites == 0
            && self.planner_writer_call_sites == 0
            && self.writer_input_authorized_token_only
            && self.token_handed_to_writer
            && self.token_consumed
            && self.tokens_dropped_without_writer == 0
            && self.writer_token_used
            && self.write_produced
            && self.render_xml_write
    }

    pub(super) fn bind_writer_outcome(
        &mut self,
        token_handed_to_writer: bool,
        token_consumed: bool,
        writer_token_used: bool,
        write_produced: bool,
    ) {
        self.token_handed_to_writer = token_handed_to_writer;
        self.token_consumed = token_consumed;
        self.tokens_dropped_without_writer = usize::from(!token_handed_to_writer);
        self.writer_token_used = writer_token_used;
        self.write_produced = write_produced;
        self.render_xml_write = write_produced;
        #[cfg(test)]
        {
            self.verdict = if self.green() {
                "renderer_direction_relocation_token_handed_to_writer_and_consumed".to_string()
            } else {
                "renderer_direction_relocation_writer_handoff_or_write_blocked".to_string()
            };
        }
    }

    #[cfg(test)]
    pub fn pipeline_line(&self, project_label: &str) -> String {
        format!(
            "RendererDirectionRelocationAuthorizationBoundaryAudit v{}: Project {}; AuthorizationBoundaryGreen {}; SourceStage {}; VerifierModule {}; WriterModule {}; ProvenPlanReceived {}; MainCandidateWritten {}; SourceSnapshotParseOk {}; PreWriterSnapshotParseOk {}; SourceSnapshotFingerprintBound {}; PreWriterSnapshotFingerprintBound {}; SnapshotTransitionBound {}; AuthorizedBeamGroupBound {}; UniqueDirectionObstacleBound {}; PayloadWithoutOffsetFingerprintBound {}; SourceCursorBound {}; HostCursorBound {}; PositiveOffsetBound {}; EffectiveCursorStable {}; StaffRelationBound {}; ExclusionSetBound {}; ExpectedInterstitialZeroBound {}; AuthorizationAttempted {}; AuthorizationGranted {}; TokensIssued {}; TokenConstructorSites {}; TokenFieldsPrivate {}; PrivateIssueFunction {}; TokenClonePaths {}; TokenForgeryPaths {}; TokenOrigin {}; WriterEntryDefinitions {}; WriterEntryTokenParameterSites {}; RawPlanWriterEntryPoints {}; TokenConstructorSitesInWriter {}; AuthorityWriterCallSites {}; DetectorWriterCallSites {}; PlannerWriterCallSites {}; WriterInputAuthorizedTokenOnly {}; TokenHandedToWriter {}; TokenConsumed {}; TokensDroppedWithoutWriter {}; WriterTokenUsed {}; WriteProduced {}; RenderXmlWrite {}; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            project_label,
            yes_no(self.green()),
            self.source_stage,
            self.verifier_module_path,
            self.writer_module_path,
            yes_no(self.proven_plan_received),
            yes_no(self.main_candidate_written),
            yes_no(self.source_snapshot_parse_ok),
            yes_no(self.pre_writer_snapshot_parse_ok),
            yes_no(self.source_snapshot_fingerprint_bound),
            yes_no(self.pre_writer_snapshot_fingerprint_bound),
            yes_no(self.snapshot_transition_bound),
            yes_no(self.authorized_beam_group_bound),
            yes_no(self.unique_direction_obstacle_bound),
            yes_no(self.payload_without_offset_fingerprint_bound),
            yes_no(self.source_cursor_bound),
            yes_no(self.host_cursor_bound),
            yes_no(self.positive_offset_bound),
            yes_no(self.effective_cursor_stable),
            yes_no(self.staff_relation_bound),
            yes_no(self.exclusion_set_bound),
            yes_no(self.expected_interstitial_zero_bound),
            yes_no(self.authorization_attempted),
            self.authorization_granted,
            self.tokens_issued,
            self.token_constructor_sites,
            yes_no(self.token_fields_private),
            yes_no(self.private_issue_function),
            self.token_clone_paths,
            self.token_forgery_paths,
            self.token_origin,
            self.writer_entry_definitions,
            self.writer_entry_token_parameter_sites,
            self.raw_plan_writer_entry_points,
            self.token_constructor_sites_in_writer,
            self.authority_writer_call_sites,
            self.detector_writer_call_sites,
            self.planner_writer_call_sites,
            yes_no(self.writer_input_authorized_token_only),
            yes_no(self.token_handed_to_writer),
            yes_no(self.token_consumed),
            self.tokens_dropped_without_writer,
            yes_no(self.writer_token_used),
            yes_no(self.write_produced),
            yes_no(self.render_xml_write),
            self.verdict,
        )
    }
}

fn token_struct_fields_are_private() -> bool {
    let Some(start) = VERIFIER_MODULE_SOURCE.find(TOKEN_STRUCT_DECLARATION_PATTERN) else {
        return false;
    };
    let tail = &VERIFIER_MODULE_SOURCE[start..];
    let Some(end) = tail.find('}') else {
        return false;
    };
    tail[..end]
        .lines()
        .skip(1)
        .filter(|line| line.contains(':'))
        .all(|line| !line.trim_start().starts_with("pub"))
}

pub(super) fn authorize_renderer_direction_relocation_for_writer(
    proven_plan: Option<ProvenRendererDirectionRelocationPlan>,
    source_xml: &str,
    pre_writer_xml: &str,
    main_candidate_written: bool,
    _source_stage: &str,
) -> (
    RendererDirectionRelocationAuthorizationBoundaryAudit,
    Option<AuthorizedRendererDirectionRelocation>,
    Option<RendererDirectionRelocationVerificationContract>,
) {
    let token_constructor_sites = VERIFIER_MODULE_SOURCE
        .matches(TOKEN_CONSTRUCTOR_PATTERN)
        .count();
    let token_fields_private = token_struct_fields_are_private();
    let private_issue_function = VERIFIER_MODULE_SOURCE.lines().any(|line| {
        line.trim_start()
            .starts_with(PRIVATE_ISSUE_FUNCTION_PATTERN)
            && !line.trim_start().starts_with("pub")
    });
    let token_clone_paths = VERIFIER_MODULE_SOURCE
        .matches(TOKEN_CLONE_DERIVE_PATTERN)
        .count()
        + VERIFIER_MODULE_SOURCE
            .matches(TOKEN_CLONE_CALL_PATTERN)
            .count()
        + WRITER_MODULE_SOURCE
            .matches(TOKEN_CLONE_CALL_PATTERN)
            .count();
    let token_forgery_paths = if token_constructor_sites != 1
        || !token_fields_private
        || !private_issue_function
        || token_clone_paths != 0
    {
        1
    } else {
        0
    };
    let writer_entry_definitions = WRITER_MODULE_SOURCE
        .matches(WRITER_ENTRY_DEFINITION_PATTERN)
        .count();
    let writer_entry_token_parameter_sites = WRITER_MODULE_SOURCE
        .matches(WRITER_ENTRY_TOKEN_PARAMETER_PATTERN)
        .count();
    let raw_plan_writer_entry_points = WRITER_MODULE_SOURCE
        .matches(RAW_PLAN_WRITER_PARAMETER_PATTERN)
        .count();
    let token_constructor_sites_in_writer = WRITER_MODULE_SOURCE
        .matches(TOKEN_CONSTRUCTOR_IN_WRITER_PATTERN)
        .count();
    let authority_writer_call_sites = AUTHORITY_MODULE_SOURCE
        .matches(WRITER_ENTRY_CALL_PATTERN)
        .count();
    let detector_writer_call_sites = DETECTOR_MODULE_SOURCE
        .matches(WRITER_ENTRY_CALL_PATTERN)
        .count();
    let planner_writer_call_sites = PLANNER_MODULE_SOURCE
        .matches(WRITER_ENTRY_CALL_PATTERN)
        .count();
    let writer_input_authorized_token_only = writer_entry_definitions == 1
        && writer_entry_token_parameter_sites == 1
        && raw_plan_writer_entry_points == 0
        && token_constructor_sites_in_writer == 0
        && token_clone_paths == 0;
    let source_snapshot_parse_ok = Document::parse(source_xml).is_ok();
    let source_snapshot_fingerprint = stable_xml_snapshot_fingerprint(source_xml);
    let pre_writer_snapshot_parse_ok = Document::parse(pre_writer_xml).is_ok();
    let pre_writer_snapshot_fingerprint = stable_xml_snapshot_fingerprint(pre_writer_xml);

    let mut audit = RendererDirectionRelocationAuthorizationBoundaryAudit {
        #[cfg(test)]
        source_stage: _source_stage.to_string(),
        #[cfg(test)]
        verifier_module_path: VERIFIER_MODULE_PATH.to_string(),
        #[cfg(test)]
        writer_module_path: WRITER_MODULE_PATH.to_string(),
        #[cfg(test)]
        proven_plan_received: proven_plan.is_some(),
        #[cfg(test)]
        main_candidate_written,
        #[cfg(test)]
        source_snapshot_parse_ok,
        #[cfg(test)]
        pre_writer_snapshot_parse_ok,
        pre_writer_snapshot_fingerprint_bound: pre_writer_snapshot_fingerprint != 0,
        snapshot_transition_bound: source_snapshot_fingerprint != 0
            && pre_writer_snapshot_fingerprint != 0
            && source_snapshot_fingerprint != pre_writer_snapshot_fingerprint,
        #[cfg(test)]
        token_constructor_sites,
        #[cfg(test)]
        token_fields_private,
        #[cfg(test)]
        private_issue_function,
        #[cfg(test)]
        token_clone_paths,
        #[cfg(test)]
        token_forgery_paths,
        #[cfg(test)]
        token_origin: "independent_verifier_only".to_string(),
        #[cfg(test)]
        writer_entry_definitions,
        #[cfg(test)]
        writer_entry_token_parameter_sites,
        #[cfg(test)]
        raw_plan_writer_entry_points,
        #[cfg(test)]
        token_constructor_sites_in_writer,
        #[cfg(test)]
        authority_writer_call_sites,
        #[cfg(test)]
        detector_writer_call_sites,
        #[cfg(test)]
        planner_writer_call_sites,
        #[cfg(test)]
        writer_input_authorized_token_only,
        token_handed_to_writer: false,
        token_consumed: false,
        tokens_dropped_without_writer: 0,
        writer_token_used: false,
        write_produced: false,
        render_xml_write: false,
        #[cfg(test)]
        verdict: "renderer_direction_relocation_authorization_not_attempted".to_string(),
        ..RendererDirectionRelocationAuthorizationBoundaryAudit::default()
    };

    let Some(proven_plan) = proven_plan else {
        #[cfg(test)]
        {
            audit.verdict = "renderer_direction_relocation_proven_plan_missing".to_string();
        }
        return (audit, None, None);
    };
    let parts = proven_plan.into_authorization_parts();
    audit.source_snapshot_fingerprint_bound = source_snapshot_parse_ok
        && parts.source_snapshot_fingerprint == source_snapshot_fingerprint;
    audit.authorized_beam_group_bound = parts.authorized_beam_group_ordinal < usize::MAX
        && parts.authorized_member_pair_ordinal < usize::MAX
        && parts.group_start_cursor < parts.group_end_cursor;
    audit.unique_direction_obstacle_bound = parts.unique_obstacle_element_ordinal < usize::MAX
        && parts.direction_type_element_count == 1
        && parts.dynamics_mark_count == 1;
    audit.payload_without_offset_fingerprint_bound = parts.payload_without_offset_fingerprint != 0;
    audit.source_cursor_bound = parts.source_element_cursor == parts.source_effective_cursor
        && parts.source_offset == 0
        && !parts.source_offset_explicit;
    audit.host_cursor_bound = parts.proposed_host_cursor == parts.group_start_cursor;
    audit.positive_offset_bound = parts.proposed_offset > 0;
    audit.effective_cursor_stable = parts
        .proposed_host_cursor
        .saturating_add(parts.proposed_offset)
        == parts.source_effective_cursor;
    audit.staff_relation_bound = parts.staff_relation_to_right_member_stable;
    audit.exclusion_set_bound = parts.sound_absent
        && parts.timing_spanner_absent
        && parts.words_or_metronome_absent
        && parts.default_x_absent;
    audit.expected_interstitial_zero_bound = parts.expected_interstitial_elements_after_relocation
        == 0
        && parts.predicted_renderer_contiguity_green;

    let prerequisites_green = main_candidate_written
        && source_snapshot_parse_ok
        && pre_writer_snapshot_parse_ok
        && audit.source_snapshot_fingerprint_bound
        && audit.pre_writer_snapshot_fingerprint_bound
        && audit.snapshot_transition_bound
        && token_forgery_paths == 0
        && writer_input_authorized_token_only
        && authority_writer_call_sites == 1
        && detector_writer_call_sites == 0
        && planner_writer_call_sites == 0;
    if !prerequisites_green {
        #[cfg(test)]
        {
            audit.verdict =
                "renderer_direction_relocation_authorization_boundary_prerequisite_blocked".to_string();
        }
        return (audit, None, None);
    }

    let Some(resolved) = resolve_pre_writer_direction_relocation(pre_writer_xml, &parts) else {
        #[cfg(test)]
        {
            audit.verdict = "renderer_direction_relocation_pre_writer_target_not_unique".to_string();
        }
        return (audit, None, None);
    };

    audit.authorization_attempted = true;
    match issue_authorized_renderer_direction_relocation(
        parts,
        pre_writer_snapshot_fingerprint,
        resolved,
    ) {
        Ok((token, contract)) => {
            if !token.binding_green() {
                #[cfg(test)]
                {
                    audit.verdict =
                        "renderer_direction_relocation_token_binding_self_check_failed".to_string();
                }
                return (audit, None, None);
            }
            audit.authorization_granted = 1;
            audit.tokens_issued = 1;
            #[cfg(test)]
            {
                audit.verdict =
                    "renderer_direction_relocation_token_issued_pending_exact_writer_handoff"
                        .to_string();
            }
            (audit, Some(token), Some(contract))
        }
        Err(_) => {
            #[cfg(test)]
            {
                audit.verdict =
                    "renderer_direction_relocation_authorization_blocked_by_plan_binding".to_string();
            }
            (audit, None, None)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_boundary_accepts_only_private_capability_and_has_exactly_one_call_site() {
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(WRITER_ENTRY_DEFINITION_PATTERN)
                .count(),
            1
        );
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(WRITER_ENTRY_TOKEN_PARAMETER_PATTERN)
                .count(),
            1
        );
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(RAW_PLAN_WRITER_PARAMETER_PATTERN)
                .count(),
            0
        );
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(TOKEN_CONSTRUCTOR_IN_WRITER_PATTERN)
                .count(),
            0
        );
        assert_eq!(
            AUTHORITY_MODULE_SOURCE
                .matches(WRITER_ENTRY_CALL_PATTERN)
                .count(),
            1
        );
        assert_eq!(
            VERIFIER_MODULE_SOURCE
                .matches(TOKEN_CONSTRUCTOR_PATTERN)
                .count(),
            1
        );
        assert!(token_struct_fields_are_private());
        assert_eq!(
            VERIFIER_MODULE_SOURCE
                .matches(TOKEN_CLONE_DERIVE_PATTERN)
                .count(),
            0
        );
        assert_eq!(
            VERIFIER_MODULE_SOURCE
                .matches(TOKEN_CLONE_CALL_PATTERN)
                .count(),
            0
        );
    }
}
