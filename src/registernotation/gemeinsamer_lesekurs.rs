//! The original register owner compares complete same-epoch part courses.
//! Complete existing spans may be omitted; no new span, pitch or writer is made.
use super::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JointLowerCourseCost {
    pub(crate) total: i64,
    pub(crate) pressure: i64,
    pub(crate) transitions: usize,
    pub(crate) octave_changes: usize,
    pub(crate) octave_reminders: usize,
    pub(crate) objects: usize,
    pub(crate) maximum_ledgers: i64,
}



pub(crate) fn vergleiche_gebundene_lesekurse(
    xml: &str,
    evaluate: impl Fn(&RegisternotationsSichtmaterialisierungsauftrag) -> Option<BTreeMap<String, JointLowerCourseCost>>,
) -> Result<(), String> {
    let original = sitzung().lock().unwrap_or_else(|e| e.into_inner()).clone();
    if original.eigener_eingang.as_deref() != Some(xml) || original.materialisierung.is_some() {
        return Err("GemeinsamerLesekurs: input epoch is not the unmaterialized original register input".into());
    }
    let mut proof = original.beweis.clone().ok_or("GemeinsamerLesekurs: original proposal missing")??;
    let old_fingerprint = proof.plan_fingerabdruck.clone();
    let parts = proof.plaene.iter().filter(|p| p.system.trim() == "2")
        .map(|p| p.teil.clone()).collect::<BTreeSet<_>>();
    let mut rows = Vec::new();
    for part in parts {
        let Some(baseline) = evaluate(&sichtauftrag_fuer_beweis(&proof, materialisierungsauftrag_fuer_beweis(&proof)?)?).and_then(|costs| costs.get(&part).cloned()) else {
            rows.push(format!("JointReadingOwner: PartHex {}; Status UNKNOWN; CompleteCourseUnavailable; MusicalDecision none.", hex(part.as_bytes())));
            continue;
        };
        let mut current = baseline.clone();
        let mut omitted = 0;
        loop {
            let positions = proof.plaene.iter().enumerate()
                .filter(|(_, p)| p.teil == part && p.system.trim() == "2")
                .map(|(i, _)| i).collect::<Vec<_>>();
            let mut best: Option<(RegisternotationsBeweis, JointLowerCourseCost, usize)> = None;
            // These are conflicting complete versions of one part, never a
            // ranking between independent parts or a scorewide first winner.
            for start in 0..positions.len() {
                for end in start + 1..=positions.len() {
                    let remove = positions[start..end].iter().copied().collect::<BTreeSet<_>>();
                    let mut candidate = proof.clone();
                    candidate.plaene = candidate.plaene.into_iter().enumerate()
                        .filter(|(i, _)| !remove.contains(i)).map(|(_, p)| p).collect();
                    candidate.plan_fingerabdruck = plan_fingerabdruck(&candidate.plaene);
                    let Some(cost) = evaluate(&sichtauftrag_fuer_beweis(&candidate, materialisierungsauftrag_fuer_beweis(&candidate)?)?).and_then(|costs| costs.get(&part).cloned()) else { continue; };
                    if cost.objects != current.objects || cost.total >= current.total
                        || cost.transitions + cost.octave_changes >= current.transitions + current.octave_changes { continue; }
                    if best.as_ref().is_none_or(|(_, previous, count)|
                        (cost.total, remove.len()) < (previous.total, *count))
                    {
                        best = Some((candidate, cost, remove.len()));
                    }
                }
            }
            match best {
                Some((candidate, cost, count)) => { proof = candidate; current = cost; omitted += count; }
                None => break,
            }
        }
        rows.push(format!("JointReadingOwner: InputSha256 {}; PartHex {}; FullContext same_original_epoch; Baseline {:?}; Selected {:?}; OmittedCompleteSpans {}; MusicalDecision {}.",
            crate::sha256::hex(xml.as_bytes()), hex(part.as_bytes()), baseline, current, omitted,
            if omitted > 0 { "existing_register_proposal_revised_before_materialization" } else { "incumbent_retained" }));
    }
    let topology = pruefe_topologie(&proof.ereignisse, &proof.plaene);
    if topology.leere_betroffene_mengen != 0 || topology.nullspannen != 0
        || topology.ereignisse_ausserhalb_spanne != 0 || topology.systemfremde_betroffene_ereignisse != 0
        || topology.widerspruechliche_ueberlappungen != 0 { return Err("GemeinsamerLesekurs: topology not closed".into()); }
    let mut state = sitzung().lock().unwrap_or_else(|e| e.into_inner());
    if state.eigener_eingang != original.eigener_eingang || state.materialisierung.is_some()
        || state.beweis.as_ref().and_then(|p| p.as_ref().ok()).map(|p| &p.plan_fingerabdruck) != Some(&old_fingerprint)
    { return Err("GemeinsamerLesekurs: original proposal changed during comparison".into()); }
    state.beweis = Some(Ok(proof));
    state.gemeinsame_lesekurse = rows;
    Ok(())
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
