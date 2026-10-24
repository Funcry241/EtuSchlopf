use super::{RenderPathBoundOutcome, RenderSinkKind, RenderSovereignState};
use crate::audit_only::phrase_boundary_evidence::PhraseNotationIntentProjection;
use crate::master_state_identity::RenderTransitionReceipts;
use crate::structural_write_authority::HorizontalLayoutMaterializationDirective;
use crate::structural_write_authority::final_notation_phrase_projection_transaction::{
    PreparedFinalLayoutPhrase, prepare_final_render_notation_with_phrase_projection,
};
use anyhow::{ensure, Context, Result};
use std::cell::RefCell;
use std::path::Path;

pub(crate) fn mutate_final_layout_phrase_insertions(
    source: &str,
    insertions: &[(usize, String)],
) -> Result<String> {
    // Transportgeometrie prÃƒÂ¼fen, kein neuer Musikguard/Selector.
    for (position, _) in insertions {
        ensure!(*position <= source.len() && source.is_char_boundary(*position),
            "phrase insertion escaped source bounds");
    }
    ensure!(insertions.windows(2).all(|pair| pair[0].0 <= pair[1].0),
        "phrase insertion order changed");
    let mut candidate = source.to_owned();
    for (position, text) in insertions.iter().rev() {
        candidate.insert_str(*position, text);
    }
    Ok(candidate)
}

pub(super) fn adopt_prepared_final_layout_phrase_materialization(
    path: &Path, pre: &str, post: &str, prepared: PreparedFinalLayoutPhrase,
) -> Result<()> {
    let (bound_pre, layout_post, final_post, _changed_slots, phrase_write) = prepared.into_parts();
    ensure!(pre == bound_pre && post == final_post, "E007 ticket/sovereign bytes drift");
    ensure!(path.file_name().and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".musicxml")),
        "horizontal-layout final artifact path is not a MusicXML file");

    // Bestehende E011-Persistenz unverÃƒÂ¤ndert als typed Konsolidierung:
    // AÃ¢â€ â€™B physisch, exakter B-Readback; kein Dateiread berechnet B.
    let layout_trace = crate::audit_only::writer_trace::capture_before(path);
    #[cfg(test)]
    transport_fault(1, path)?;
    std::fs::write(path, layout_post.as_bytes())
        .with_context(|| format!("horizontal-layout final artifact could not be written: {}", path.display()))?;
    #[cfg(test)]
    transport_fault(2, path)?;
    let layout_readback = std::fs::read_to_string(path)
        .with_context(|| format!("horizontal-layout final artifact readback failed: {}", path.display()))?;
    ensure!(layout_readback == layout_post, "horizontal-layout final artifact exact readback mismatch");
    if layout_trace.is_some() {
        crate::audit_only::writer_trace::record_after(
            "horizontal_layout_persistence", path, layout_trace.as_deref(), &layout_readback,
        );
    }
    if phrase_write {
        let phrase_trace = crate::audit_only::writer_trace::capture_before(path);
        #[cfg(test)]
        transport_fault(3, path)?;
        std::fs::write(path, final_post.as_bytes())
            .with_context(|| format!("phrase-slur verified render candidate could not be persisted: {}", path.display()))?;
        #[cfg(test)]
        transport_fault(4, path)?;
        let phrase_readback = std::fs::read_to_string(path)?;
        if phrase_readback != final_post {
            // Heute restauriert E007 seinen vor-Phrase Layoutpost B, nicht State.
            let _ = std::fs::write(path, layout_post.as_bytes());
            anyhow::bail!("phrase-slur persisted candidate verification failed; source bytes restored");
        }
        crate::audit_only::writer_trace::record_after(
            "phrase_slur_policy_persistence", path, phrase_trace.as_deref(), &phrase_readback,
        );
    }
    // Receiptgebundenes C ist bereits vor beiden Writes souverÃƒÂ¤n verifiziert.
    // ZusÃƒÂ¤tzlicher State-Witness und einziger E007-Commit folgen in der API.
    Ok(())
}

pub(crate) fn adopt_final_layout_phrase_projection(
    state: &mut RenderSovereignState,
    directives: &[HorizontalLayoutMaterializationDirective],
    intents: &[PhraseNotationIntentProjection],
) -> Result<usize> {
    adopt_final_layout_phrase_projection_with_course(state, directives, intents, None)
}

pub(crate) fn adopt_final_layout_phrase_projection_with_course(
    state: &mut RenderSovereignState,
    directives: &[HorizontalLayoutMaterializationDirective],
    intents: &[PhraseNotationIntentProjection],
    course: Option<&crate::structural_write_authority::final_notation_phrase_projection_transaction::BoundPhraseOccurrenceCourse>,
) -> Result<usize> {
    let root = state.require_state_root("Release/E007");
    let ticket = RefCell::<Option<PreparedFinalLayoutPhrase>>::new(None);
    state.adopt_path_bound_transition(
        root,
        RenderSinkKind::FinalLayoutPhraseProjection,
        |pre| {
            let (prepared, receipt) = if course.is_some() {
                crate::structural_write_authority::final_notation_phrase_projection_transaction::prepare_final_render_notation_with_phrase_course(pre, directives, intents, course)
            } else { prepare_final_render_notation_with_phrase_projection(pre, directives, intents) };
            match prepared {
                Err(error) => RenderPathBoundOutcome {
                    value: Err(error), planned_post: None,
                    receipts: RenderTransitionReceipts::new(), failed: true,
                },
                Ok(prepared) => {
                    let changed_slots = prepared.changed_slots();
                    let (receipt, post) = match receipt {
                        Some(receipt) => {
                            let (receipt, post) = receipt.split_realized_post_document();
                            (Some(receipt), post.expect("E007 file-bound PR must carry post"))
                        }
                        None => {
                            assert_eq!(pre, prepared.final_post());
                            (None, prepared.final_post().to_owned())
                        }
                    };
                    assert_eq!(post, prepared.final_post());
                    let mut receipts = RenderTransitionReceipts::new();
                    receipts.push_preserving(receipt, &post);
                    *ticket.borrow_mut() = Some(prepared);
                    RenderPathBoundOutcome {
                        value: Ok(changed_slots), planned_post: Some(post), receipts, failed: false,
                    }
                }
            }
        },
        |path, pre, post| {
            let prepared = ticket.borrow_mut().take().expect("accepted E007 prepared ticket");
            adopt_prepared_final_layout_phrase_materialization(path, pre, post, prepared)
        },
    )?
}

#[cfg(test)]
thread_local! {
    static TRANSPORT_FAULT: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}
#[cfg(test)]
pub(crate) fn set_final_layout_transport_fault(phase: u8) {
    TRANSPORT_FAULT.with(|fault| fault.set(phase));
}
#[cfg(test)]
fn transport_fault(phase: u8, path: &Path) -> Result<()> {
    if TRANSPORT_FAULT.with(|fault| fault.get()) != phase { return Ok(()); }
    if phase == 1 || phase == 3 { anyhow::bail!("injected typed E007 transport failure"); }
    std::fs::write(path, "wrong E007 readback")?;
    Ok(())
}
