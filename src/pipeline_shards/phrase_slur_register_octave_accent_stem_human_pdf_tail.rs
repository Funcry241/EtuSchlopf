// v0.10.1702 r02 CanonicalMetricBeamObservationBoundary cross-file interface closure.
// PartialBeatMeasureSpan/PartialBeatRawNote/PartialBeatNoteInfo are now owned by
// composer_metric_beam_observation_core; this consumer keeps identical semantics.

// v0.10.716 PhraseSlurRegisterOctaveBehindBarsTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of former phrase_slur_notation_truth_beam_reference_tail.rs
// and register_octave_behind_bars_obstruction_tail.rs in exact include order.
// The former register-octave render-write owner is only moved, not changed.
// No music, render, solver, or report decision logic is changed.

// ---- merged from src/pipeline_shards/phrase_slur_notation_truth_beam_reference_tail.rs ----
// v0.10.714 PhraseSlurNotationTruthBeamReferenceTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of the phrase/slur/WAV-articulation candidate tail and notation-truth/beam-reference tail.
// Former phrase_slur_wav_articulation_tail.rs and notation_truth_beam_reference_tail.rs are included below in the same order; no musical/render/solver/report decision logic is changed.


// r732 CompletedReadOnlyProjectionRetirementWave5.
// The former PauseDelimitedPhrase candidate/slur audit island is retired.
// Automatic phrase-slur write authority was already removed in r1202; its
// remaining read-only candidate transport had no writer, blocker, selector,
// release or final-report authority. Current phrase evidence is owned by the
// typed phrase_boundary_evidence foundation and its later projections.

// ---- merged from src/pipeline_shards/notation_truth_beam_reference_tail.rs ----
// v0.10.704 NotationTruthBeamClosureTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of the notation-truth / beam-evidence closure tail.
// Former part_26..part_27 are included below in the same order.

// ---- merged from src/pipeline_shards/part_26.rs ----
// v0.10.548 SisterFeedbackNotationTruthKernel after T6 rest anchor write.
// Fresh readback of the full sister-feedback package against the current final render XML.

// ---- merged from src/pipeline_shards/part_27.rs ----
// v0.10.505 BeamHierarchyPdfEvidenceClosureNoMutation.
// Closes the next sister-feedback evidence corridor for the beam/rest/beat hierarchy without writing.

// ---- v0.10.710 merged order boundary: former beam_rest_reference_status_tail.rs ----

// v0.10.705 BeamRestReferenceStatusTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of former part_28.rs and part_29.rs.
// Order preserved; report/status/readback only; no music, render, solver, or report-intent change.

// ---- former src/pipeline_shards/part_28.rs ----
// v0.10.513 BeamHierarchyFinalReadbackRepairTinyRenderWrite current release section.
// Keeps the productive sister-feedback beam write visible as the current main action.

// ---- former behind_bars_obstruction_probe_tail.rs ----
// v0.10.700 BehindBarsObstructionProbeTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of former part_31.rs..part_39.rs.
// Order preserved; no music, render, solver, or report-intent change.

// ---- former part_31.rs ----
// v0.10.557 BehindBarsObstructionFocusedProbeNoMutation keeps this block visible.

// ---- former part_32.rs ----
// v0.10.558 AccentNoteheadProximityPdfCoordinateProbeNoMutation keeps this probe visible.
// This shard is report-only. It turns the SchlopfKernel's next operator into a
// focused Behind-Bars obstruction ledger and intentionally opens no write gate.

fn append_behind_bars_rule_class_closure_no_mutation_section(out: &mut String) {
    out.push_static_section_header_v795("\nBEHIND-BARS-RULE-CLASS-CLOSURE-NO-MUTATION", "------------------------------------------------");
    out.push_format_args(format_args!(
        "Status: v{} audit-only. Diese Closure schliesst den Schoenheitsfehler, dass die offenen Behind Bars-Befunde wie Takt-Reparaturen klingen konnten. Die Architektur ist regelbasiert: konkrete Stellen sind Beweisinstanzen fuer Regelklassen, nicht die Form der Entscheidung. Nach der upstream AccentReserve/WavTruth-Materialisierung wird aktuelle positive Akzent-Evidenz getrennt vom erhaltenen TieTail-Regressionsgegenbeispiel gezaehlt.\n",
        env!("CARGO_PKG_VERSION")
    ));
    push_kurz_no_direct_write_release_green_closed(out, "Kurz: BehindBarsRuleClassClosureNoMutation: RuleClasses 3; EvidenceInstancesTotal 25; CurrentPositiveEvidence 24; HistoricalCounterexamples 1; AccentCurrentPositive 17; AccentSuppressedNegative 1; TaktLogic nein; MeasureSpecificArchitecture nein; MachineGreenClasses 3; BehindBarsRuleGreenClasses 0; RuleClosureReady 0; RuleWriteReady 0; ");
}

// ---- former part_33.rs ----
// v0.10.558 AccentNoteheadProximityPdfCoordinateProbeNoMutation.
// Report-only probe for the first focused Behind Bars obstruction.

// ---- former part_34.rs ----
// v0.10.559 LowerStaffBeamContinuityBehindBarsProbeNoMutation.
// Report-only probe for the lower-staff beam readability obstruction.

// ---- former part_35.rs ----
// v0.10.560 HumanRestVerticalPdfSpotcheckProbeNoMutation.
// Report-only probe for the rest vertical placement Behind-Bars obstruction.

// ---- former part_36.rs ----
// v0.10.561 BehindBarsObstructionClosureLedgerNoMutation.
// Report-only closure ledger for the three focused Behind-Bars obstruction classes.

// --- mechanical consolidation boundary: former src/pipeline_shards/part_40.rs ---

// r725 RetiredObservationProjectionHarvestClippyClosure:
// The retired ConfusingTiedTailAccentSuppress writer report-wrapper is removed.
// The historical-domain detector below remains live and fail-closed; productive
// accent ownership stays upstream with WavTruth/AccentReserve.

fn note_fragment_has_tie_role(fragment: &str, role: &str) -> bool {
    fragment.contains(&format!("<tie type=\"{role}\""))
        || fragment.contains(&format!("<tied type=\"{role}\""))
}

fn same_relational_pitch(left: &AtomicNote, right: &AtomicNote) -> bool {
    left.pitch.as_ref().map(|pitch| pitch.midi)
        == right.pitch.as_ref().map(|pitch| pitch.midi)
}

fn note_has_planned_accent(
    note: &AtomicNote,
    planned: &[AssertiveVisibleArticulationRow],
) -> bool {
    planned.iter().any(|row| {
        row.mark == "accent"
            && AccentAttackOwnershipKey {
                scope: row.scope,
                staff: row.staff.clone(),
                voice: row.voice.clone(),
                onset: row.onset,
                midi: row.midi,
                duration: row.duration,
            }
            .matches_note(note)
    })
}

fn confusing_tied_tail_accent_keys(
    xml: &str,
    planned: &[AssertiveVisibleArticulationRow],
) -> Vec<AccentAttackOwnershipKey> {
    let notes = parse_atomic_notes(xml);
    let mut candidates = Vec::<AccentAttackOwnershipKey>::new();
    for tail in notes.iter().filter(|note| {
        note.visible && !note.is_chord && note.pitch.is_some() && note.duration > 0
    }) {
        let Some(tail_fragment) = xml.get(tail.note_start..tail.note_end) else {
            continue;
        };
        let existing_tail_accent = tail_fragment.contains("<accent");
        let planned_tail_accent = note_has_planned_accent(tail, planned);
        if (!existing_tail_accent && !planned_tail_accent)
            || !note_fragment_has_tie_role(tail_fragment, "stop")
            || (existing_tail_accent
                && remove_first_accent_and_cleanup_articulations(tail_fragment).is_none())
        {
            continue;
        }
        let mut attack_owners = notes.iter().filter(|attack| {
            attack.visible
                && !attack.is_chord
                && attack.pitch.is_some()
                && attack.measure == tail.measure
                && attack.staff == tail.staff
                && attack.voice == tail.voice
                && attack.start < tail.start
                && attack.start.saturating_add(attack.duration) == tail.start
                && same_relational_pitch(attack, tail)
                && xml
                    .get(attack.note_start..attack.note_end)
                    .is_some_and(|fragment| note_fragment_has_tie_role(fragment, "start"))
        });
        let Some(attack) = attack_owners.next() else {
            continue;
        };
        if attack_owners.next().is_some() {
            continue;
        }
        let Some(attack_fragment) = xml.get(attack.note_start..attack.note_end) else {
            continue;
        };
        if attack_fragment.contains("<accent") || note_has_planned_accent(attack, planned) {
            continue;
        }
        let Some(scope) = tail.measure.trim().parse::<usize>().ok() else {
            continue;
        };
        let Some(pitch) = tail.pitch.as_ref() else {
            continue;
        };
        candidates.push(AccentAttackOwnershipKey {
            scope,
            staff: tail.staff.clone(),
            voice: tail.voice.clone(),
            onset: tail.start,
            midi: pitch.midi,
            duration: tail.duration,
        });
    }
    candidates
}

// Legacy callable name retained so the closure does not introduce a new pipeline callable surface.
// Semantics are no longer scorewide-unique: this returns the full local tied-tail candidate count.
pub(crate) fn unique_confusing_tied_tail_accent_regression_candidates(
    xml: &str,
) -> Option<usize> {
    parse_musicxml(xml).ok()?;
    Some(confusing_tied_tail_accent_keys(xml, &[]).len())
}

fn remove_first_accent_and_cleanup_articulations(fragment: &str) -> Option<String> {
    crate::structural_write_authority::remove_first_bound_accent_authorized(fragment)
}

// r725: retired TieTail report-wrapper removed; live regression detector remains above.

