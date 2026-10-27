#[derive(Debug, Clone)]
struct LowerClefSurfaceMaterializationAudit {
    written: bool,
    pipeline_line: String,
}
fn materialize_lower_clef_surface_decision(
    surface_decision: RustigClefSurfaceDecision,
    source_xml: &str,
    output_path: &std::path::Path,
    adoption: Option<(&mut structural_write_authority::RenderSovereignState, structural_write_authority::RenderTrialRoot)>,
) -> anyhow::Result<LowerClefSurfaceMaterializationAudit> {
    let surface_candidate_count = surface_decision.candidate_count();
    let Some(materialization_plan) = surface_decision.materialization_plan() else {
        return Ok(LowerClefSurfaceMaterializationAudit {
            written: false,
            pipeline_line: format!(
                "TypedLowerClefSurfaceMaterialization v{}: SemanticDecisionAuthorized nein; SemanticRole {}; SemanticRoleMatches {}; MusicalDecisionOwner RustigClefSurfaceDecision; MaterializationOwner not_invoked; ProjectIdentityUsed nein; AbsoluteMeasurePredicate nein; RenderXmlWrite nein; MusicMutation nein; MasterApply nein; Urteil lower_clef_surface_materialization_fail_closed_not_unique.",
                env!("CARGO_PKG_VERSION"),
                LOWER_CLEF_SURFACE_SEMANTIC_ROLE,
                surface_candidate_count
            ),
        });
    };
    let Some(pending) = structural_write_authority::project_clef_surface_plan(
        source_xml,
        materialization_plan,
    ) else {
        return Ok(LowerClefSurfaceMaterializationAudit {
            written: false,
            pipeline_line: format!(
                "TypedLowerClefSurfaceMaterialization v{}: SemanticDecisionAuthorized ja; SemanticRole {}; SemanticRoleMatches {}; MaterializationOwner TypedClefSurfaceMaterialization; source_binding_or_surface_delta_missing; ProjectIdentityUsed nein; AbsoluteMeasurePredicate nein; RenderXmlWrite nein; MusicMutation nein; MasterApply nein; Urteil lower_clef_surface_materialization_stopped_no_target_or_source_mismatch.",
                env!("CARGO_PKG_VERSION"),
                LOWER_CLEF_SURFACE_SEMANTIC_ROLE,
                surface_candidate_count
            ),
        });
    };
    let stats = pending.stats();
    let (before_events, _before_backup_forward_ok) = collect_render_xml_note_events(source_xml);
    let (after_events, _after_backup_forward_ok) =
        collect_render_xml_note_events(pending.candidate_xml());
    let visible_sounding_signature_stable =
        render_visible_sounding_signature(&before_events)
            == render_visible_sounding_signature(&after_events);
    let Some(candidate) = pending.verify_sounding_invariants(
        visible_sounding_signature_stable,
        before_events.len(),
        after_events.len(),
    ) else {
        return Ok(LowerClefSurfaceMaterializationAudit {
            written: false,
            pipeline_line: format!(
                "TypedLowerClefSurfaceMaterialization v{}: SemanticDecisionAuthorized ja; SemanticRole {}; SemanticRoleMatches {}; MaterializationOwner TypedClefSurfaceMaterialization; invariant_guard_failed; hidden_g_clefs {}; lower_f_context {}; note_events {}->{}; sounding_stable {}; ProjectIdentityUsed nein; AbsoluteMeasurePredicate nein; RenderXmlWrite nein; MusicMutation nein; MasterApply nein; Urteil lower_clef_surface_materialization_stopped_invariant_guard.",
                env!("CARGO_PKG_VERSION"),
                LOWER_CLEF_SURFACE_SEMANTIC_ROLE,
                surface_candidate_count,
                stats.hidden_g_clef_blocks,
                stats.lower_f_clef_blocks_in_target,
                before_events.len(),
                after_events.len(),
                yes_no(visible_sounding_signature_stable)
            ),
        });
    };
    if let Some((state, root)) = adoption {
        let ticket = std::cell::RefCell::new(Some(candidate));
        state.adopt_path_bound_transition(root, structural_write_authority::RenderSinkKind::LowerClefSurfaceMaterialization,
            |_pre| {
                let mut binding = ticket.borrow_mut();
                let candidate = binding.as_mut().expect("prepared clef candidate");
                let receipt = candidate.prepare_render_receipt();
                let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
                let planned_post = receipt.map(|receipt| { let (receipt, post) = receipt.split_realized_post_document(); let post = post.expect("clef realized post"); receipts.push_block(Some(receipt), &post); post });
                structural_write_authority::RenderPathBoundOutcome { value: (), planned_post, receipts, failed: false }
            },
            |path, _pre, _post| { let (result, receipt) = structural_write_authority::write_clef_surface_materialization_authorized(path, ticket.borrow_mut().take().expect("consume clef ticket")); assert!(receipt.is_none(), "prepared #61 must not remint at sink"); result })?;
    } else {
        let (result, receipt) = structural_write_authority::write_clef_surface_materialization_authorized(output_path, candidate);
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
        if let Some(receipt) = receipt { let post = std::fs::read_to_string(output_path)?; probe.push_block(Some(receipt), &post); }
        probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        result?;
    }
    Ok(LowerClefSurfaceMaterializationAudit {
        written: true,
        pipeline_line: format!(
            "TypedLowerClefSurfaceMaterialization v{}: SemanticDecisionAuthorized ja; SemanticRole {}; SemanticRoleMatches {}; MusicalDecisionOwner RustigClefSurfaceDecision; MaterializationOwner TypedClefSurfaceMaterialization; hidden_visible_lower_g_clef_blocks {}; lower_f_context {}; note_events {}->{}; sounding_stable {}; output {}; ProjectIdentityUsed nein; AbsoluteMeasurePredicate nein; RenderXmlWrite ja; XmlWrite ja_render_only; MasterMusicXML nein; MusicMutation nein; MasterApply nein; DeltaSurface lower_staff_visible_g_surface_neutralization_keep_f4_context; Invariant pitch/onset/duration/staff/voice/hand/beam/rest/rhythm/articulation preserved_by_note_event_signature_and_exact_source_bound_clef_print_object_delta; Urteil lower_clef_surface_materialization_written.",
            env!("CARGO_PKG_VERSION"),
            LOWER_CLEF_SURFACE_SEMANTIC_ROLE,
            surface_candidate_count,
            stats.hidden_g_clef_blocks,
            stats.lower_f_clef_blocks_in_target,
            before_events.len(),
            after_events.len(),
            yes_no(visible_sounding_signature_stable),
            output_path.display()
        ),
    })
}
#[derive(Debug, Clone)]
struct ClefSurfaceNoteEvent {
    staff: String,
    byte_start: usize,
    is_rest: bool,
    is_hidden: bool,
    midi: Option<i32>,
}



fn collect_clef_surface_note_events(xml: &str) -> Vec<ClefSurfaceNoteEvent> {
    let mut events = Vec::new();
    let mut scan = 0usize;
    while let Some((measure_start, measure_end, _measure_number)) =
        find_next_measure_bounds(xml, scan)
    {
        let measure = &xml[measure_start..measure_end];
        let mut local_scan = 0usize;
        while let Some(relative_start) = measure[local_scan..].find("<note") {
            let start = local_scan + relative_start;
            let Some(relative_end) = measure[start..].find("</note>") else {
                break;
            };
            let end = start + relative_end + "</note>".len();
            let note = &measure[start..end];
            let staff =
                extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
            events.push(ClefSurfaceNoteEvent {
                staff,
                byte_start: measure_start + start,
                is_rest: note.contains("<rest"),
                is_hidden: is_hidden_note(note),
                midi: midi_from_note_fragment(note),
            });
            local_scan = end;
        }
        scan = measure_end;
    }
    events
}

fn clef_context_readability_penalty(
    sign: &str,
    line: &str,
    min_midi: Option<i32>,
    max_midi: Option<i32>,
) -> i32 {
    let (Some(min_midi), Some(max_midi)) = (min_midi, max_midi) else {
        return 0;
    };
    let (low, high): (i32, i32) = match (sign, line) {
        ("G", "2") => (55, 84),
        ("F", "4") => (34, 69),
        ("C", "3") => (45, 76),
        ("C", "4") => (40, 72),
        _ => (40, 76),
    };
    low.saturating_sub(min_midi)
        .max(max_midi.saturating_sub(high))
}

fn semantic_lower_clef_pdf_render_input_force_reason(
    semantic_surface_write_materialized: bool,
    master_musicxml: &Path,
    render_musicxml: &Path,
) -> Option<&'static str> {
    if !semantic_surface_write_materialized
        || !render_musicxml.exists()
        || !master_musicxml.exists()
    {
        return None;
    }
    let master_text = std::fs::read_to_string(master_musicxml).ok()?;
    let render_text = std::fs::read_to_string(render_musicxml).ok()?;
    if master_text == render_text {
        return None;
    }
    let (master_events, master_balance_ok) = collect_render_xml_note_events(&master_text);
    let (render_events, render_balance_ok) = collect_render_xml_note_events(&render_text);
    let sounding_signature_stable = master_balance_ok
        && render_balance_ok
        && master_events.len() == render_events.len()
        && render_visible_sounding_signature(&master_events)
            == render_visible_sounding_signature(&render_events);
    sounding_signature_stable.then_some(
        "materialized_semantic_lower_clef_surface_delta_with_stable_sounding_signature",
    )
}


fn append_silent_voice_coverage_write_outcome_projection(
    out: &mut String,
    projection_message: Option<&str>,
    expected: crate::final_report::RendererOutcomeExpectation,
) {
    if let Some(line) = projection_message {
        out.push_str("Source: existing_writer_audit_pipeline_line; ProjectionOnly ja; WriterGraphChanged nein; SelectionChanged nein; MusicXmlChangedByProjection nein; PdfChangedByProjection nein.\n");
        out.push_str(line);
        out.push('\n');
        out.push_str("RendererOutcome: observed_from_existing_writer_audit.\n");
    } else {
        out.push_str("Source: existing_writer_audit_pipeline_line; ProjectionOnly ja; RendererOutcome not_observed; RenderXmlWrite nein; NotationMutation nein; MusicMutation nein; MasterApply nein.\n");
        out.push_str(match expected {
            crate::final_report::RendererOutcomeExpectation::NotRequested => "RendererOutcome: not_requested.\n",
            crate::final_report::RendererOutcomeExpectation::BlockedBeforeRenderer => "RendererOutcome: blocked_before_renderer.\n",
            crate::final_report::RendererOutcomeExpectation::Observed => "RendererOutcome: missing_writer_audit.\n",
        });
    }
}

const LOWER_CLEF_SURFACE_SEMANTIC_ROLE: &str =
    "same_measure_f4_g2_surface_overlap_marginal_g2_advantage_visible";
fn derive_lower_clef_rustig_surface_decision(xml: &str) -> RustigClefSurfaceDecision {
    let note_events = collect_clef_surface_note_events(xml);
    let mut candidates = Vec::new();
    let mut scan = 0usize;
    while let Some((measure_start, measure_end, _measure_number)) =
        find_next_measure_bounds(xml, scan)
    {
        let Some(surface_inspection) = structural_write_authority::inspect_clef_surface_span(
            xml,
            measure_start,
            measure_end,
        ) else {
            scan = measure_end;
            continue;
        };
        let visible_lower_g_clefs = surface_inspection.visible_lower_g_clefs;
        let hidden_lower_g_clefs = surface_inspection.hidden_lower_g_clefs;
        if visible_lower_g_clefs + hidden_lower_g_clefs == 0 {
            scan = measure_end;
            continue;
        }
        let (min_midi, max_midi) = note_events
            .iter()
            .filter(|event| {
                event.byte_start >= measure_start
                    && event.byte_start < measure_end
                    && event.staff == "2"
                    && !event.is_hidden
                    && !event.is_rest
            })
            .filter_map(|event| event.midi)
            .fold((None, None), |(minimum, maximum), midi| {
                (
                    Some(minimum.map_or(midi, |value: i32| value.min(midi))),
                    Some(maximum.map_or(midi, |value: i32| value.max(midi))),
                )
            });
        let f4_readability_penalty =
            clef_context_readability_penalty("F", "4", min_midi, max_midi);
        let g2_readability_penalty =
            clef_context_readability_penalty("G", "2", min_midi, max_midi);
        let same_measure_visible_f4_g2_overlap = surface_inspection.lower_f_clefs > 0
            && visible_lower_g_clefs > 0
            && hidden_lower_g_clefs == 0;
        let materialization_plan =
            surface_inspection.hide_visible_lower_staff_g_preserve_f_context(xml);
        let is_candidate = structural_write_authority::is_rustig_surface_course_candidate(
            materialization_plan,
            same_measure_visible_f4_g2_overlap,
            f4_readability_penalty,
            g2_readability_penalty,
        );
        if let (true, Some(plan)) = (is_candidate, materialization_plan) {
            candidates.push(plan);
        }
        scan = measure_end;
    }
    structural_write_authority::derive_rustig_surface_clef_decision(candidates)
}
#[cfg(test)]
mod rustig_clef_surface_decision_tests {
    use super::{
        derive_lower_clef_rustig_surface_decision,
        materialize_lower_clef_surface_decision,
        RustigClefSurfaceDecision,
    };
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    fn resolve(xml: &str) -> RustigClefSurfaceDecision {
        derive_lower_clef_rustig_surface_decision(xml)
    }
    fn overlap_measure(number: &str) -> String {
        format!(
            r#"<measure number="{number}"><attributes>
<divisions>24</divisions><time><beats>1</beats><beat-type>4</beat-type></time>
<clef number="2"><sign>F</sign><line>4</line></clef>
<clef number="2"><sign>G</sign><line>2</line></clef>
</attributes>
<note><pitch><step>D</step><alter>1</alter><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>
</measure>"#
        )
    }
    fn temporary_render_path(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "etuschlopf_{label}_{}_{}.render.musicxml",
            std::process::id(),
            nonce
        ))
    }
    fn sovereign_fixture(xml: &str, path: &std::path::Path) -> crate::structural_write_authority::RenderSovereignState {
        let master = crate::structural_write_authority::MasterAdoptedState::adopt_genesis_state(
            xml.to_string(), crate::master_state_identity::MasterLineage::lineage_from_genesis(
                crate::master_state_identity::tests::synthetic_genesis_for_tests(xml)));
        let mut state = crate::structural_write_authority::RenderSovereignState::begin_project_render(path, &master);
        let checked = crate::cursor_lane_temporal_check::check_cursor_lane_temporal_precondition("synthetic-clef-master", master.document()).expect("temporally valid clef fixture");
        state.bind_late_master_epoch(crate::master_state_identity::RenderMasterEpoch::after_cursor_lane_before_final_chord(master.lineage(), master.document(), &checked, master.document()));
        state
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/structural_write_authority/clef_sovereign_commit_test.rs"));

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/structural_write_authority/clef_stale_artifact_test.rs"));

    #[test]
    fn production_contract_writes_once_rejects_stale_source_and_is_idempotent() {
        let xml = format!(
            "<score-partwise><part id=\"P1\">{}</part></score-partwise>",
            overlap_measure("1")
        );
        let output_path = temporary_render_path("positive");
        let audit = materialize_lower_clef_surface_decision(
            resolve(&xml),
            &xml,
            &output_path,
            None,
        )
        .expect("authorized production path");
        assert!(audit.written);
        let written = std::fs::read_to_string(&output_path).expect("written render MusicXML");
        assert!(written.contains(
            "<clef number=\"2\" print-object=\"no\"><sign>G</sign><line>2</line></clef>"
        ));
        assert!(written.contains(
            "<clef number=\"2\"><sign>F</sign><line>4</line></clef>"
        ));
        let second_decision = resolve(&written);
        assert_eq!(second_decision.candidate_count(), 0);
        assert!(second_decision.materialization_plan().is_none());

        let stale_source = xml.replacen("<duration>24</duration>", "<duration>48</duration>", 1);
        let stale_path = temporary_render_path("stale");
        let stale_audit = materialize_lower_clef_surface_decision(
            resolve(&xml),
            &stale_source,
            &stale_path,
            None,
        )
        .expect("stale source must fail closed");
        assert!(!stale_audit.written);
        assert!(!stale_path.exists());
        assert!(stale_audit
            .pipeline_line
            .contains("source_binding_or_surface_delta_missing"));

        let _ = std::fs::remove_file(output_path);
    }
    #[test]
    fn ambiguous_or_strong_gain_surface_courses_fail_closed() {
        let ambiguous = format!(
            "<score-partwise><part id=\"P1\">{}{}</part></score-partwise>",
            overlap_measure("1"),
            overlap_measure("2")
        );
        let ambiguous_plan = resolve(&ambiguous);
        assert_eq!(ambiguous_plan.candidate_count(), 2);
        assert!(ambiguous_plan.materialization_plan().is_none());

        let strong_gain = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes>\n<clef number=\"2\"><sign>F</sign><line>4</line></clef>\n<clef number=\"2\"><sign>G</sign><line>2</line></clef>\n</attributes>\n<note><pitch><step>B</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>\n<note><pitch><step>A</step><alter>1</alter><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>\n</measure></part></score-partwise>";
        let strong_gain_plan = resolve(strong_gain);
        assert_eq!(strong_gain_plan.candidate_count(), 0);
        assert!(strong_gain_plan.materialization_plan().is_none());
    }
}
fn append_typed_lower_clef_surface_materialization_result_section(
    out: &mut String,
    audit: &LowerClefSurfaceMaterializationAudit,
) {
    out.push_str("\nTYPED-LOWER-CLEF-SURFACE-MATERIALIZATION\n");
    out.push_str("----------------------------------------------------------------\n");
    out.push_format_args(format_args!(
        "Status: v{} projekt- und taktidentitaetsfreie Materialisierung eines bereits entschiedenen Lower-Clef-Surface-Plans. Rustigs ClefSurfaceDecision darf genau einen scoreweiten Kandidaten der Rolle same_measure_f4_g2_surface_overlap_marginal_g2_advantage_visible auswaehlen; Quellbindung, exakter XML-Delta, Klanginvarianten und Dateisink gehoeren ausschliesslich der typisierten Authority. Kein MasterApply und keine Musikmutation.\n",
        env!("CARGO_PKG_VERSION")
    ));
    out.push_format_args(format_args!(
        "RenderXmlWrite: {}\n",
        if audit.written { "ja" } else { "nein" }
    ));
    out.push_format_args(format_args!("OperatorLine: {}\n", audit.pipeline_line));
    out.push_str("Gate: F4/G2 muessen im selben unteren Taktblock sichtbar ueberlagert sein, G2 darf nach der bestehenden Clef-Lesbarkeit nur marginal besser sein (Vorteil 1..=3), der Kandidat muss scoreweit genau einmal vorkommen, und die typisierte Materialisierung muss Source-Binding, exakten Surface-Delta sowie Klang- und NoteCount-Invarianten bestaetigen. Bei Mehrdeutigkeit oder technischem Beweisbruch gilt fail-closed.\n");
}
