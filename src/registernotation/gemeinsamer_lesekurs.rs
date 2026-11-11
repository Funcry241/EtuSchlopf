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
    if let Some((score, treble_parts)) = treble_readback(xml) {
        let count = proof.plaene.len();
        proof.plaene.retain(|plan| !treble_parts.contains(plan.teil.as_str())
            || upper_octave_line_has_severe_ledger_benefit(plan, &score) != Some(false));
        let omitted = count - proof.plaene.len();
        if omitted > 0 {
            proof.plan_fingerabdruck = plan_fingerabdruck(&proof.plaene);
            rows.push(format!("JointReadingOwner: UpperTrebleOmittedCompleteSpans {omitted}; Context same_original_epoch."));
        }
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
// Upper spans use actual pitch spelling in a positively established G2
// course. A variable or missing clef leaves the original decision in place.
fn treble_readback(xml: &str) -> Option<(Score, BTreeSet<String>)> {
    let document = roxmltree::Document::parse(xml).ok()?;
    let mut parts = BTreeSet::new();
    for part in document.root_element().children().filter(|node| node.has_tag_name("part")) {
        let first_measure = part.children().find(|node| node.has_tag_name("measure"))?;
        let first_event = first_measure.children().find(|node|
            matches!(node.tag_name().name(), "note" | "backup" | "forward"));
        let mut clefs = part.descendants().filter(|node| node.has_tag_name("clef")
            && node.attribute("number").unwrap_or("1") == "1");
        let Some(first) = clefs.next() else { continue; };
        if !first.parent().is_some_and(|node| node.has_tag_name("attributes")
            && node.parent() == Some(first_measure))
            || first_event.is_some_and(|node| node.range().start < first.range().end)
        {
            continue;
        }
        let is_treble = |clef: roxmltree::Node<'_, '_>| {
            let child = |name| clef.children().find(|node| node.has_tag_name(name))
                .and_then(|node| node.text()).map(str::trim);
            child("sign") == Some("G") && child("line") == Some("2")
                && matches!(child("clef-octave-change"), None | Some("0"))
        };
        if is_treble(first) && clefs.all(is_treble) {
            parts.insert(part.attribute("id")?.to_owned());
        }
    }
    Some((crate::musicxml::parse_musicxml(xml).ok()?, parts))
}

fn upper_octave_line_has_severe_ledger_benefit(plan: &RegisterNotationPlan, score: &Score) -> Option<bool> {
    if plan.system.trim() != "1" {
        return None;
    }
    let mut plain = [0_usize; 5];
    let mut candidate_ledgers = [0_usize; 5];
    let mut visible = 0;
    for id in &plan.betroffene_ereignisse {
        let part = score.parts.get(id.teil_ordinal.checked_sub(1)?)?;
        let measure = part.measures.get(id.takt_ordinal.checked_sub(1)?)?;
        let event = measure.events.get(id.ereignis_ordinal.checked_sub(1)?)?;
        if part.id != plan.teil || event.staff.trim() != "1" {
            return None;
        }
        if !event.visible {
            continue;
        }
        let pitch = event.pitch.as_ref()?;
        let degree = ["C", "D", "E", "F", "G", "A", "B"].iter()
            .position(|step| *step == pitch.step.trim())?;
        let diatonic = pitch.octave.checked_mul(7)?.checked_add(i32::try_from(degree).ok()?)?;
        record_severe_treble_ledgers(diatonic, &mut plain);
        let shifted = diatonic.checked_add(7 * plan.darstellung.darstellungsoktaven())?;
        record_severe_treble_ledgers(shifted, &mut candidate_ledgers);
        visible += 1;
    }
    (visible > 0).then_some(candidate_ledgers < plain)
}

// As in C3, four or more ledger lines outrank a reading-state change.
// Compare the most severe tiers first, with eight or more as the top tier.
fn record_severe_treble_ledgers(diatonic: i32, histogram: &mut [usize; 5]) {
    let written = i64::from(diatonic);
    let ledgers = (30 - written).max(written - 38).max(0) / 2;
    if ledgers >= 4 {
        let index = 8_usize - usize::try_from(ledgers.min(8)).expect("bounded ledger tier");
        histogram[index] += 1;
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
