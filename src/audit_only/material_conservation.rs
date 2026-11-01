//! Cardinality and existing provenance records; no musical transformation.
use crate::audit_only::notation_invariants::{PitchedFragment, material_fragments};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub(crate) const SECTION: &str = "MATERIAL-CONSERVATION";
pub(crate) const SECTION_RULE: &str = "---------------------";

pub(crate) struct OriginRecord<'a> {
    pub(crate) owner: &'a str,
    pub(crate) sources: &'a [usize],
    pub(crate) existence_changes: usize,
    pub(crate) other_changes: usize,
}
pub(crate) struct SourceRow {
    pub(crate) ordinal: usize,
    pub(crate) retained: bool,
    pub(crate) raw_onset: i64,
    pub(crate) raw_duration: i64,
    pub(crate) pitch: i32,
    pub(crate) removal_reason: Option<String>,
    pub(crate) restified_at_entry: Option<bool>,
}
pub(crate) struct AttackRow<'a> {
    pub(crate) source: usize,
    pub(crate) exists: bool,
    pub(crate) grace: bool,
    pub(crate) measure: usize,
    pub(crate) onset: i64,
    pub(crate) end: i64,
    pub(crate) pitch: i32,
    pub(crate) staff: &'a str,
    pub(crate) voice: &'a str,
    pub(crate) notated_duration: Option<i64>,
}
pub(crate) struct MaterialPopulation<'a> {
    pub(crate) authority_bound: bool,
    pub(crate) canonical: usize,
    pub(crate) lineage: usize,
    pub(crate) active: usize,
    pub(crate) source_divisions: i64,
    pub(crate) sources: &'a [SourceRow],
    pub(crate) retained_ids: &'a [usize],
    pub(crate) attacks: &'a [AttackRow<'a>],
    pub(crate) origins: &'a [OriginRecord<'a>],
}
#[derive(Default, Debug, PartialEq, Eq)]
struct HeadCounts {
    visible_attacks: usize,
    hidden_attacks: usize,
    hidden_continuations: usize,
    hidden_fragments: usize,
}
fn head_counts(notes: &[PitchedFragment]) -> HeadCounts {
    let mut c = HeadCounts::default();
    for n in notes {
        if !n.visible {
            c.hidden_fragments += 1;
        }
        match (n.visible, n.tie_stop) {
            (true, false) => c.visible_attacks += 1,
            (false, false) => c.hidden_attacks += 1,
            (false, true) => c.hidden_continuations += 1,
            _ => {}
        }
    }
    c
}
#[derive(Default, Debug, PartialEq, Eq)]
struct Populations {
    removed_before: usize,
    inactive_within: usize,
    missing_origin: usize,
    restified_at_entry: usize,
    duration_changed: usize,
    duration_unknown: usize,
    valid: bool,
}
fn populations(input: &MaterialPopulation<'_>) -> Populations {
    let sources: BTreeSet<_> = input.sources.iter().map(|r| r.ordinal).collect();
    let retained: BTreeSet<_> = input.retained_ids.iter().copied().collect();
    let planned_retained: BTreeSet<_> = input
        .sources
        .iter()
        .filter(|r| r.retained)
        .map(|r| r.ordinal)
        .collect();
    let active: BTreeSet<_> = input
        .attacks
        .iter()
        .filter(|r| r.exists)
        .map(|r| r.source)
        .collect();
    let attack_ids: BTreeSet<_> = input.attacks.iter().map(|r| r.source).collect();
    let inactive: BTreeSet<_> = retained.difference(&active).copied().collect();
    let mut c = Populations {
        removed_before: sources.difference(&retained).count(),
        inactive_within: inactive.len(),
        valid: input.authority_bound
            && sources.len() == input.sources.len()
            && sources.len() == input.canonical
            && retained.len() == input.retained_ids.len()
            && retained.len() == input.lineage
            && planned_retained == retained
            && retained.is_subset(&sources)
            && attack_ids.len() == input.attacks.len()
            && attack_ids.is_subset(&retained)
            && active.len() == input.active,
        ..Populations::default()
    };
    for source in &inactive {
        c.restified_at_entry += usize::from(
            input
                .sources
                .iter()
                .any(|r| r.ordinal == *source && r.restified_at_entry == Some(true)),
        );
        if !input
            .origins
            .iter()
            .any(|r| exact_existence_membership(r, *source))
        {
            c.missing_origin += 1;
        }
    }
    for attack in input.attacks.iter().filter(|a| a.exists && !a.grace) {
        if let Some(original) = attack.notated_duration {
            match attack.end.checked_sub(attack.onset) {
                Some(length) => c.duration_changed += usize::from(length != original),
                None => c.duration_unknown += 1,
            }
        } else {
            c.duration_unknown += 1;
        }
    }
    c
}
fn exact_existence_membership(record: &OriginRecord<'_>, source: usize) -> bool {
    record.existence_changes > 0
        && record.existence_changes == record.sources.len()
        && record.sources.contains(&source)
        && record
            .sources
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            == record.sources.len()
}
fn rational(value: (i128, i128)) -> String {
    format!("{}/{}", value.0, value.1)
}
fn append_heads(out: &mut String, level: &str, notes: &[PitchedFragment]) {
    let c = head_counts(notes);
    let _ = writeln!(
        out,
        "MaterialHeads: level {level}; visible_attacks {}; hidden_attacks {}; hidden_fragments {}; hidden_tied_continuations {}; fragment_bridge {} = {} + {}; grace excluded.",
        c.visible_attacks,
        c.hidden_attacks,
        c.hidden_fragments,
        c.hidden_continuations,
        c.hidden_fragments,
        c.hidden_attacks,
        c.hidden_continuations
    );
    let _ = writeln!(
        out,
        "HiddenOrigin: level {level}; hidden_attacks {}; proven_owner 0; unreceipted {}; reason lineage_records_have_no_visibility_dimension_and_no_bound_hide_record_is_exposed_here.",
        c.hidden_attacks, c.hidden_attacks
    );
    for note in notes.iter().filter(|n| !n.visible) {
        let _ = writeln!(
            out,
            "HiddenMaterial: level {level}; part {}; measure_ordinal {}; onset_quarters {}; duration_quarters {}; midi {}; staff {:?}; voice {:?}; tied_continuation {}; source_identity not_claimed; hidden_owner unreceipted.",
            note.part,
            note.measure,
            rational(note.onset),
            rational(note.duration),
            rational(note.pitch),
            note.staff,
            note.voice,
            note.tie_stop
        );
    }
}
type UnisonoKey = (usize, usize, (i128, i128), (i128, i128), (i128, i128));
fn visible_multiplicities(notes: &[PitchedFragment]) -> BTreeMap<UnisonoKey, usize> {
    let mut counts = BTreeMap::new();
    for n in notes.iter().filter(|n| n.visible && !n.tie_stop) {
        *counts
            .entry((n.part, n.measure, n.onset, n.duration, n.pitch))
            .or_default() += 1;
    }
    counts
}
fn append_render_difference(
    out: &mut String,
    master: &[PitchedFragment],
    render: &[PitchedFragment],
) {
    let before = visible_multiplicities(master);
    let after = visible_multiplicities(render);
    let (mut compatible, mut absent, mut added) = (0, 0, 0);
    let continued: BTreeSet<_> = render
        .iter()
        .filter(|n| n.visible && n.tie_stop)
        .map(|n| (n.part, n.measure, n.onset, n.duration, n.pitch))
        .collect();
    let mut continuation_partners = 0;
    for (key, count) in &before {
        let now = after.get(key).copied().unwrap_or(0);
        if now < *count {
            if now > 0 {
                compatible += count - now;
            } else {
                absent += count;
                if continued.contains(key) {
                    continuation_partners += count;
                }
            }
            let _ = writeln!(
                out,
                "RenderHeadDifference: part {}; measure_ordinal {}; onset_quarters {}; first_fragment_duration_quarters {}; midi {}; master {}; render {}; duplicate_class_remains {}.",
                key.0,
                key.1,
                rational(key.2),
                rational(key.3),
                rational(key.4),
                count,
                now,
                now > 0
            );
        }
    }
    for (key, count) in after {
        added += count.saturating_sub(before.get(&key).copied().unwrap_or(0));
    }
    let _ = writeln!(
        out,
        "RenderVisibleDifference: duplicate_class_compatible_reduction {compatible}; absent_classes {absent}; added_heads {added}; with_visible_continuation_partner {continuation_partners}; absent_classes_mean_no_visible_attack_not_no_visible_fragment; equality_key part_measure_onset_pitch_first_fragment_duration; full_sounding_duration_equivalence not_claimed; no_new_unisono_permission."
    );
}
pub(crate) fn append_material_conservation(
    out: &mut String,
    input: &MaterialPopulation<'_>,
    master: &str,
    render: Option<&str>,
) {
    out.push_str("Documents: final in-memory Master and Render. MusicalAuthority none; ReleaseAuthority none. Cardinalities, source identity rows, printed fragments and existing origin records are separate dimensions.\n");
    let p = populations(input);
    let notes = match material_fragments(master) {
        Ok(notes) => notes,
        Err(reason) => {
            let _ = writeln!(
                out,
                "MaterialConservation: status MODEL_INPUT_UNAVAILABLE; reason {reason}; counts unknown."
            );
            return;
        }
    };
    let counts = head_counts(&notes);
    let xml_attacks = counts.visible_attacks + counts.hidden_attacks;
    let grace = input.attacks.iter().filter(|a| a.exists && a.grace).count();
    let lineage_identity = p.valid
        && xml_attacks.checked_add(grace) == Some(input.active)
        && input.active.checked_add(p.inactive_within) == Some(input.lineage);
    let canonical_identity =
        lineage_identity && input.lineage.checked_add(p.removed_before) == Some(input.canonical);
    let _ = writeln!(
        out,
        "MaterialConservation: canonical {}; lineage {}; active {}; visible_master {}; hidden_master {}; inactive_within_lineage {}; removed_before_lineage {}; canonical_inactive {}; active_grace {}; lineage_identity {}; canonical_identity {}; identity_kind cardinality_only_not_a_late_source_binding.",
        input.canonical,
        input.lineage,
        input.active,
        counts.visible_attacks,
        counts.hidden_attacks,
        p.inactive_within,
        p.removed_before,
        p.inactive_within + p.removed_before,
        grace,
        lineage_identity,
        canonical_identity
    );
    let _ = writeln!(
        out,
        "InactiveOrigins: restified_at_lineage_entry {}; without_exact_existence_record {}; duration_changed_vs_notated_entry {}; duration_unknown {}; duration_reference notation_96_quarter_ticks; raw_source_divisions {}.",
        p.restified_at_entry,
        p.missing_origin,
        p.duration_changed,
        p.duration_unknown,
        input.source_divisions
    );
    append_heads(out, "Master", &notes);
    let active_ids: BTreeSet<_> = input
        .attacks
        .iter()
        .filter(|r| r.exists)
        .map(|r| r.source)
        .collect();
    let mut sources: Vec<_> = input.sources.iter().collect();
    sources.sort_by_key(|r| (r.raw_onset, r.pitch, r.ordinal));
    for row in sources {
        let state = if !row.retained {
            "removed_before_lineage"
        } else if active_ids.contains(&row.ordinal) {
            "active"
        } else {
            "inactive_within_lineage"
        };
        let _ = writeln!(
            out,
            "MaterialSource: source_ordinal {}; raw_source_onset {}; raw_source_duration {}; raw_source_midi {}; state {state}; removal_reason {:?}; restified_at_entry {:?}; printed_source_binding not_claimed.",
            row.ordinal,
            row.raw_onset,
            row.raw_duration,
            row.pitch,
            row.removal_reason,
            row.restified_at_entry
        );
    }
    for attack in input.attacks {
        let _ = writeln!(
            out,
            "MaterialAttack: source_ordinal {}; exists {}; grace {}; measure_ordinal {}; onset_notation_ticks {}; sounding_end_notation_ticks {}; midi {}; staff {:?}; voice {:?}; entry_notated_duration {:?}.",
            attack.source,
            attack.exists,
            attack.grace,
            attack.measure,
            attack.onset,
            attack.end,
            attack.pitch,
            attack.staff,
            attack.voice,
            attack.notated_duration
        );
    }
    for (index, record) in input.origins.iter().enumerate() {
        let _ = writeln!(
            out,
            "MaterialOrigin: record_ordinal {}; owner {:?}; existence_changes {}; other_changes {}; source_ordinals {:?}; attribution existing_membership_only_unless_all_listed_sources_changed_existence.",
            index + 1,
            record.owner,
            record.existence_changes,
            record.other_changes,
            record.sources
        );
    }
    if let Some(xml) = render {
        match material_fragments(xml) {
            Ok(render_notes) => {
                append_heads(out, "Render", &render_notes);
                append_render_difference(out, &notes, &render_notes);
            }
            Err(reason) => {
                let _ = writeln!(
                    out,
                    "MaterialConservationRender: status MODEL_INPUT_UNAVAILABLE; reason {reason}; counts unknown."
                );
            }
        }
    } else {
        out.push_str("MaterialConservationRender: status DOCUMENT_UNAVAILABLE; counts unknown.\n");
    }
}
#[cfg(test)]
#[path = "material_conservation_tests.rs"]
mod tests;
