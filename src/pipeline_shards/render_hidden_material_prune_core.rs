






fn build_render_xml_hidden_material_prune_audit(xml: &str) -> RenderXmlHiddenMaterialPruneAudit {
    let (events, backup_forward_balance_ok) = collect_render_xml_note_events(xml);
    let mut audit = RenderXmlHiddenMaterialPruneAudit {
        backup_forward_balance_ok,
        visible_signature_before: render_visible_signature(&events, false),
        ..RenderXmlHiddenMaterialPruneAudit::default()
    };

    let mut scope_stats: BTreeMap<(usize, String, String), RenderXmlScopeAggregate> = BTreeMap::new();
    for event in &events {
        let entry = scope_stats
            .entry((event.scope, event.staff.clone(), event.voice.clone()))
            .or_default();
        if event.is_hidden {
            audit.hidden_events_total += 1;
            if event.midi.is_some() {
                audit.hidden_pitched_notes += 1;
                entry.hidden_pitched += 1;
                if event.is_chord {
                    audit.hidden_inner_chord_tones += 1;
                    match event.anchor {
                        RenderXmlAnchorKind::VisiblePitched => audit.hidden_chord_on_visible_pitched_anchor += 1,
                        RenderXmlAnchorKind::VisibleRest => audit.hidden_chord_on_rest_anchor += 1,
                        _ => audit.hidden_chord_without_visible_anchor += 1,
                    }
                } else {
                    audit.hidden_primary_notes += 1;
                }
            }
            if event.is_rest {
                audit.hidden_rest_events += 1;
                entry.hidden_rests += 1;
            }
        } else if event.midi.is_some() {
            audit.visible_pitched_notes += 1;
            entry.visible_pitched += 1;
        } else if event.is_rest {
            entry.visible_rests += 1;
        }
    }

    for event in &events {
        if !event.is_hidden {
            continue;
        }
        let key = (event.scope, event.staff.clone(), event.voice.clone());
        let scope = scope_stats.get(&key).cloned().unwrap_or_default();
        let rest_only_aux_voice = event.is_rest
            && scope.visible_pitched == 0
            && scope.visible_rests == 0
            && scope.hidden_pitched == 0
            && scope.hidden_rests > 0;

        let class = if event.has_visible_dependency {
            "blocked"
        } else if rest_only_aux_voice {
            "safe_rest_only"
        } else if event.is_rest {
            "risky_rest"
        } else if event.midi.is_some() && !event.is_chord {
            "risky_primary"
        } else if event.midi.is_some() && event.is_chord && event.anchor == RenderXmlAnchorKind::VisiblePitched {
            "safe_inner"
        } else {
            "unknown"
        };

        let entry = scope_stats.entry(key).or_default();
        match class {
            "blocked" => {
                audit.blocked_visible_dependency += 1;
                entry.blocked += 1;
            }
            "safe_rest_only" => {
                audit.safe_rest_only_aux_voice_events += 1;
            }
            "risky_rest" => {
                audit.risky_timeline_rests += 1;
                entry.risky_rests += 1;
            }
            "risky_primary" => {
                audit.risky_hidden_primary_notes += 1;
                entry.unknown += 1;
            }
            "safe_inner" => {
                audit.safe_hidden_inner_chord_tones += 1;
                entry.safe_inner += 1;
            }
            _ => {
                audit.unknown_hidden_events += 1;
                entry.unknown += 1;
            }
        }
    }

    audit.visible_signature_after_virtual_prune = render_visible_signature(&events, true);
    audit.visible_signature_stable = audit.visible_signature_before == audit.visible_signature_after_virtual_prune;

    let mut rows = scope_stats
        .into_iter()
        .filter_map(|((scope, staff, voice), stats)| {
            if stats.hidden_pitched == 0 && stats.hidden_rests == 0 && stats.safe_inner == 0 && stats.risky_rests == 0 && stats.unknown == 0 && stats.blocked == 0 {
                return None;
            }
            let score = (stats.blocked as i64) * 400
                + (stats.risky_rests as i64) * 250
                + (stats.unknown as i64) * 180
                + (stats.hidden_pitched as i64) * 20
                + (stats.hidden_rests as i64) * 30
                + (stats.safe_inner as i64) * 5;
            
            Some(RenderXmlHiddenScopeAudit {
                scope,
                staff,
                voice,
                hidden_pitched: stats.hidden_pitched,
                hidden_rests: stats.hidden_rests,
                score,
            })
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        b.score.cmp(&a.score)
            .then_with(|| b.hidden_rests.cmp(&a.hidden_rests))
            .then_with(|| b.hidden_pitched.cmp(&a.hidden_pitched))
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
    });
    audit.top_scopes = rows.into_iter().take(12).collect();

    if audit.hidden_chord_on_rest_anchor > 0 {
        audit.examples.push(format!(
            "{} versteckte klingende Chord-Events hängen an sichtbarem Restanker; diese Events bleiben prune_unknown/prune_risky_hidden_sound_on_rest_anchor.",
            audit.hidden_chord_on_rest_anchor
        ));
    }
    if audit.hidden_rest_events > 0 {
        audit.examples.push(format!(
            "{} versteckte Pausen tragen Timeline-/Voice-Kontext; keine Pausenlöschung in dieser Version.",
            audit.hidden_rest_events
        ));
    }
    if audit.safe_hidden_inner_chord_tones > 0 {
        audit.examples.push(format!(
            "{} versteckte Innenton-Events haben sichtbaren klingenden Akkordanker und keine erkannte sichtbare Bindung.",
            audit.safe_hidden_inner_chord_tones
        ));
    }
    if audit.blocked_visible_dependency > 0 {
        audit.examples.push(format!(
            "{} versteckte Events haben Tie/Slur/Beam/Ottava/Articulation-/Notation-Abhängigkeit und sind blockiert.",
            audit.blocked_visible_dependency
        ));
    }

    audit.verdict = if audit.visible_signature_stable && audit.backup_forward_balance_ok {
        "render_hidden_material_prune_audit_only_ok".to_string()
    } else {
        "render_hidden_material_prune_audit_needs_manual_review".to_string()
    };
    audit.done_fragment = format!(
        "RenderXmlHiddenMaterialPruneAudit audit-only: sichtbar {}, versteckt klingend {}, versteckte Pausen {}, safe Innentöne {}, riskante/unklare Events {}",
        audit.visible_pitched_notes,
        audit.hidden_pitched_notes,
        audit.hidden_rest_events,
        audit.safe_hidden_inner_chord_tones,
        audit.risky_timeline_rests + audit.unknown_hidden_events + audit.blocked_visible_dependency + audit.risky_hidden_primary_notes
    );
    audit.pipeline_line = format!(
        "RenderXmlHiddenMaterialPruneAudit v{}: audit-only; sichtbare klingende Noten {}, versteckte klingende Noten {}, versteckte Pausen {}, safe_hidden_inner_chord_tone {}, hidden_sound_on_rest_anchor {}, unknown/blockiert/riskant {}; keine Render-XML-Ausgabe, keine Musikmutation, keine Pausenmutation, keine PDF-Verbesserungsbehauptung.",
        env!("CARGO_PKG_VERSION"),
        audit.visible_pitched_notes,
        audit.hidden_pitched_notes,
        audit.hidden_rest_events,
        audit.safe_hidden_inner_chord_tones,
        audit.hidden_chord_on_rest_anchor,
        audit.unknown_hidden_events + audit.blocked_visible_dependency + audit.risky_timeline_rests + audit.risky_hidden_primary_notes
    );
    audit
}

fn collect_render_xml_note_events(xml: &str) -> (Vec<RenderXmlNoteEvent>, bool) {
    let mut events = Vec::new();
    let mut balance_ok = true;
    let mut scan = 0_usize;
    while let Some((start, end, scope_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[start..end];
        let (mut measure_events, measure_balance_ok) = collect_render_measure_note_events(measure, scope_number);
        if !measure_balance_ok {
            balance_ok = false;
        }
        events.append(&mut measure_events);
        scan = end;
    }
    (events, balance_ok)
}

fn collect_render_measure_note_events(measure: &str, scope_number: usize) -> (Vec<RenderXmlNoteEvent>, bool) {
    let mut out = Vec::new();
    let mut cursor = 0_i32;
    let mut scan = 0_usize;
    let mut balance_ok = true;
    let mut last_anchor: Option<(i32, RenderXmlAnchorKind)> = None;

    loop {
        let next_note = measure[scan..].find("<note").map(|p| scan + p);
        let next_backup = measure[scan..].find("<backup").map(|p| scan + p);
        let next_forward = measure[scan..].find("<forward").map(|p| scan + p);
        let next = [
            next_note.map(|pos| (pos, "note")),
            next_backup.map(|pos| (pos, "backup")),
            next_forward.map(|pos| (pos, "forward")),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|(pos, _)| *pos);

        let Some((pos, kind)) = next else { break; };
        match kind {
            "note" => {
                let Some(close_rel) = measure[pos..].find("</note>") else { break; };
                let end = pos + close_rel + "</note>".len();
                let note = &measure[pos..end];
                let duration = extract_first_text_from_fragment(note, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "?".to_string());
                let voice = extract_first_text_from_fragment(note, "voice").unwrap_or_else(|| "?".to_string());
                let is_chord = note.contains("<chord") || note.contains("<chord/>");
                let is_rest = note.contains("<rest");
                let midi = midi_from_note_fragment(note);
                let is_hidden = is_hidden_note(note);
                let onset = if is_chord {
                    last_anchor.map_or(cursor, |(onset, _)| onset)
                } else {
                    cursor
                };
                let anchor = if is_chord {
                    last_anchor.map_or(RenderXmlAnchorKind::None, |(_, anchor)| anchor)
                } else {
                    RenderXmlAnchorKind::None
                };
                let current_anchor = render_anchor_kind(is_hidden, is_rest, midi.is_some());
                if !is_chord {
                    last_anchor = Some((onset, current_anchor));
                    cursor = cursor.saturating_add(duration);
                }
                out.push(RenderXmlNoteEvent {
                    scope: scope_number,
                    staff,
                    voice,
                    onset,
                    duration,
                    is_hidden,
                    is_rest,
                    is_chord,
                    midi,
                    anchor,
                    has_visible_dependency: render_note_has_visible_dependency(note),
                });
                scan = end;
            }
            "backup" => {
                let Some(close_rel) = measure[pos..].find("</backup>") else { break; };
                let end = pos + close_rel + "</backup>".len();
                let fragment = &measure[pos..end];
                let duration = extract_first_text_from_fragment(fragment, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                cursor = cursor.saturating_sub(duration);
                if cursor < 0 {
                    balance_ok = false;
                    cursor = 0;
                }
                last_anchor = None;
                scan = end;
            }
            "forward" => {
                let Some(close_rel) = measure[pos..].find("</forward>") else { break; };
                let end = pos + close_rel + "</forward>".len();
                let fragment = &measure[pos..end];
                let duration = extract_first_text_from_fragment(fragment, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                cursor = cursor.saturating_add(duration);
                last_anchor = None;
                scan = end;
            }
            _ => break,
        }
    }
    (out, balance_ok)
}

fn render_anchor_kind(hidden: bool, is_rest: bool, is_pitched: bool) -> RenderXmlAnchorKind {
    match (hidden, is_rest, is_pitched) {
        (false, false, true) => RenderXmlAnchorKind::VisiblePitched,
        (false, true, _) => RenderXmlAnchorKind::VisibleRest,
        (true, false, true) => RenderXmlAnchorKind::HiddenPitched,
        (true, true, _) => RenderXmlAnchorKind::HiddenRest,
        _ => RenderXmlAnchorKind::None,
    }
}

fn render_note_has_visible_dependency(note: &str) -> bool {
    [
        "<tie",
        "<tied",
        "<slur",
        "<beam",
        "<notations",
        "<articulations",
        "<technical",
        "<ornaments",
        "<octave-shift",
        "<tuplet",
    ]
    .iter()
    .any(|needle| note.contains(needle))
}

fn render_visible_signature(events: &[RenderXmlNoteEvent], virtual_prune_safe_hidden: bool) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for event in events {
        let pruned_safe_hidden = virtual_prune_safe_hidden
            && event.is_hidden
            && event.midi.is_some()
            && event.is_chord
            && event.anchor == RenderXmlAnchorKind::VisiblePitched
            && !event.has_visible_dependency;
        if event.is_hidden || pruned_safe_hidden {
            continue;
        }
        event.scope.hash(&mut hasher);
        event.staff.hash(&mut hasher);
        event.voice.hash(&mut hasher);
        event.onset.hash(&mut hasher);
        event.duration.hash(&mut hasher);
        event.is_rest.hash(&mut hasher);
        event.is_chord.hash(&mut hasher);
        event.midi.hash(&mut hasher);
    }
    hasher.finish()
}



