

fn decode_wav_mono_samples(path: &Path) -> Option<(Vec<f32>, u32)> {
    let data = std::fs::read(path).ok()?;
    if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return None;
    }
    let mut audio_format = 1_u16;
    let mut channels = 0_u16;
    let mut sample_rate = 0_u32;
    let mut bits_per_sample = 0_u16;
    let mut block_align = 0_u16;
    let mut data_range: Option<(usize, usize)> = None;
    let mut pos = 12_usize;
    while pos + 8 <= data.len() {
        let id = &data[pos..pos + 4];
        let size = read_u32_le_local(&data[pos + 4..pos + 8])? as usize;
        let body = pos + 8;
        if body + size > data.len() {
            break;
        }
        if id == b"fmt " && size >= 16 {
            audio_format = read_u16_le_local(&data[body..body + 2])?;
            channels = read_u16_le_local(&data[body + 2..body + 4])?;
            sample_rate = read_u32_le_local(&data[body + 4..body + 8])?;
            block_align = read_u16_le_local(&data[body + 12..body + 14])?;
            bits_per_sample = read_u16_le_local(&data[body + 14..body + 16])?;
        } else if id == b"data" {
            data_range = Some((body, body + size));
        }
        pos = body + size + (size % 2);
    }
    let (start, end) = data_range?;
    if channels == 0 || sample_rate == 0 || bits_per_sample == 0 || block_align == 0 {
        return None;
    }
    let bytes_per_sample = (bits_per_sample as usize).div_ceil(8);
    if bytes_per_sample == 0 || (block_align as usize) < bytes_per_sample * channels as usize {
        return None;
    }
    let mut out = Vec::with_capacity((end - start) / block_align as usize);
    for frame in data[start..end].chunks_exact(block_align as usize) {
        let mut sum = 0.0_f32;
        let mut used = 0_usize;
        for ch in 0..channels as usize {
            let off = ch * bytes_per_sample;
            if off + bytes_per_sample <= frame.len()
                && let Some(value) = decode_wav_sample(
                    &frame[off..off + bytes_per_sample],
                    bits_per_sample,
                    audio_format,
                )
            {
                sum += value;
                used += 1;
            }
        }
        if used > 0 {
            out.push(sum / used as f32);
        }
    }
    if out.is_empty() { None } else { Some((out, sample_rate)) }
}

fn decode_wav_sample(bytes: &[u8], bits: u16, format: u16) -> Option<f32> {
    if format == 3 && bits == 32 && bytes.len() >= 4 {
        return Some(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]).clamp(-1.0, 1.0));
    }
    match bits {
        8 if !bytes.is_empty() => Some(((bytes[0] as f32) - 128.0) / 128.0),
        16 if bytes.len() >= 2 => {
            let value = i16::from_le_bytes([bytes[0], bytes[1]]) as f32;
            Some((value / 32768.0).clamp(-1.0, 1.0))
        }
        24 if bytes.len() >= 3 => {
            let mut value = (bytes[0] as i32) | ((bytes[1] as i32) << 8) | ((bytes[2] as i32) << 16);
            if value & 0x0080_0000 != 0 {
                value |= !0x00ff_ffff;
            }
            Some(((value as f32) / 8_388_608.0).clamp(-1.0, 1.0))
        }
        32 if bytes.len() >= 4 => {
            let value = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f32;
            Some((value / 2_147_483_648.0).clamp(-1.0, 1.0))
        }
        _ => None,
    }
}

fn read_u16_le_local(bytes: &[u8]) -> Option<u16> {
    if bytes.len() < 2 { None } else { Some(u16::from_le_bytes([bytes[0], bytes[1]])) }
}

fn read_u32_le_local(bytes: &[u8]) -> Option<u32> {
    if bytes.len() < 4 { None } else { Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])) }
}

fn audio_salience_features(samples: &[f32]) -> (i64, i64, i64) {
    if samples.is_empty() {
        return (0, 0, 0);
    }
    let mut sq = 0.0_f64;
    let mut diff = 0.0_f64;
    let mut prev = samples[0] as f64;
    for sample in samples {
        let v = *sample as f64;
        sq += v * v;
        diff += (v - prev).abs();
        prev = v;
    }
    let rms = (sq / samples.len() as f64).sqrt();
    let mean_diff = if samples.len() > 1 { diff / (samples.len() - 1) as f64 } else { 0.0 };
    let energy = (rms * 1000.0).round() as i64;
    let attack = (mean_diff * 4000.0).round() as i64;
    let flux = ((mean_diff / (rms + 0.000_1)) * 500.0).round() as i64;
    (energy.clamp(0, 4000), attack.clamp(0, 4000), flux.clamp(0, 4000))
}




fn wav_low_register_proxy(samples: &[f32], energy_milli: i64) -> i64 {
    if samples.len() < 2 || energy_milli <= 0 {
        return 0;
    }
    let mut crossings = 0_usize;
    let mut previous = samples[0];
    for sample in samples.iter().skip(1) {
        if (previous >= 0.0 && *sample < 0.0) || (previous < 0.0 && *sample >= 0.0) {
            crossings += 1;
        }
        previous = *sample;
    }
    let zcr_milli = ((crossings as f64 / (samples.len() - 1) as f64) * 1000.0).round() as i64;
    let lowness = (1000_i64 - zcr_milli.saturating_mul(12)).clamp(80, 1000);
    (energy_milli.saturating_mul(lowness) / 1000).clamp(0, 4000)
}

fn collect_wav_function_gap_staff_stats(render_xml: &str) -> BTreeMap<usize, WavFunctionGapStaffMeasureStats> {
    let mut map: BTreeMap<usize, WavFunctionGapStaffMeasureStats> = BTreeMap::new();
    for note in parse_atomic_notes(render_xml) {
        let Some(scope) = note.measure.parse::<usize>().ok() else { continue; };
        let entry = map.entry(scope).or_default();
        if note.staff.trim() == "1" && note.pitch.is_some() && note.visible {
            entry.staff1_visible_pitched += 1;
        } else if note.staff.trim() == "2" {
            if note.pitch.is_some() {
                if note.visible {
                    entry.staff2_visible_pitched += 1;
                } else {
                    entry.staff2_hidden_pitched += 1;
                }
            } else if note.visible {
                entry.staff2_visible_rests += 1;
            }
        }
    }
    map
}

struct WavFunctionGapRowClassInput {
    staff1_visible_pitched: usize,
    staff2_visible_pitched: usize,
    energy_milli: i64,
    attack_milli: i64,
    bass_proxy_milli: i64,
    avg_energy_milli: i64,
    avg_attack_milli: i64,
    avg_bass_proxy_milli: i64,}

fn classify_wav_function_gap_row(input: WavFunctionGapRowClassInput) -> (String, String) {
    let WavFunctionGapRowClassInput {
        staff1_visible_pitched,
        staff2_visible_pitched,
        energy_milli,
        attack_milli,
        bass_proxy_milli,
        avg_energy_milli,
        avg_attack_milli,
        avg_bass_proxy_milli,
    } = input;
    let energy_high = energy_milli >= (avg_energy_milli.saturating_mul(13) / 10).max(1);
    let attack_high = attack_milli >= (avg_attack_milli.saturating_mul(13) / 10).max(1);
    let bass_high = bass_proxy_milli >= (avg_bass_proxy_milli.saturating_mul(13) / 10).max(1);
    let energy_near_or_above = energy_milli >= (avg_energy_milli.saturating_mul(9) / 10).max(1);
    let attack_near_or_above = attack_milli >= (avg_attack_milli.saturating_mul(9) / 10).max(1);
    let bass_near_or_above = bass_proxy_milli >= (avg_bass_proxy_milli.saturating_mul(9) / 10).max(1);
    let energy_clearly_low = energy_milli <= (avg_energy_milli.saturating_mul(3) / 4).max(1);
    let attack_clearly_low = attack_milli <= (avg_attack_milli.saturating_mul(4) / 5).max(1);
    let bass_clearly_low = bass_proxy_milli <= (avg_bass_proxy_milli.saturating_mul(4) / 5).max(1);
    let rh_dense = staff1_visible_pitched >= 14;

    if staff2_visible_pitched == 0 && bass_high && attack_high {
        (
            "echte_fehlende_linke_hand_funktion_pruefen".to_string(),
            "starker WAV-Funktionsbefund: keine blinde Notenrueckholung, aber spielbare linke-Hand-Kondensation aus Bassanker plus rhythmischem Kern vorbereiten".to_string(),
        )
    } else if staff2_visible_pitched == 0 && bass_high && energy_high {
        (
            "echte_fehlende_linke_hand_funktion_pruefen".to_string(),
            "starker Fundamentbefund: pruefe spaeter eine reduzierte linke-Hand-Rolle; nur mit Invariantenbeweis und Human-Gate".to_string(),
        )
    } else if staff2_visible_pitched == 0 && rh_dense && energy_near_or_above && (attack_near_or_above || bass_near_or_above) {
        (
            "komponisten_override_noetig".to_string(),
            "kein automatischer Fix: obere Dichte plus mittlere WAV-Funktion verlangt Komponisten-Gegenhoeren; nur bei ausdruecklichem Wunsch spaeter Kondensation suchen".to_string(),
        )
    } else if staff2_visible_pitched == 0 && (energy_near_or_above || attack_near_or_above || bass_near_or_above) {
        (
            "schwacher_grenzfall_gegenhoeren".to_string(),
            "schwacher Grenzfall: nicht automatisch fuellen; gegen WAV hoeren und nur bei klarer musikalischer Funktion weiterverfolgen".to_string(),
        )
    } else if staff2_visible_pitched == 0 && energy_clearly_low && bass_clearly_low {
        (
            "echte_kompositorische_leere_wahrscheinlich".to_string(),
            "untere Hand wahrscheinlich bewusst/zulässig leer; kein Operator, hoechstens Human-WAV-Gegenhoeren".to_string(),
        )
    } else if staff2_visible_pitched == 0 && attack_clearly_low && bass_clearly_low {
        (
            "nur_resonanz_oder_nachhall_wahrscheinlich".to_string(),
            "WAV-Anteil wirkt eher wie Nachhall/Resonanz; nicht notieren, solange der Komponist keine Funktion fordert".to_string(),
        )
    } else if staff2_visible_pitched == 0 {
        (
            "schwacher_grenzfall_gegenhoeren".to_string(),
            "kein starker Funktionsbeweis, aber auch keine harte Leere; gegen WAV und kompositorische Absicht entscheiden".to_string(),
        )
    } else if staff2_visible_pitched <= 1 && (bass_high || energy_high) {
        (
            "duenne_untere_hand_mit_wav_druck_pruefen".to_string(),
            "untere Hand ist vorhanden, aber eventuell als Funktion zu schwach kondensiert; P2-Write eng begrenzt".to_string(),
        )
    } else {
        (
            "unauffaellig".to_string(),
            "kein prioritaerer WAV-Funktionsluecken-Befund".to_string(),
        )
    }
}

fn wav_function_gap_priority(class_label: &str) -> i32 {
    match class_label {
        "echte_fehlende_linke_hand_funktion_pruefen" => 5,
        "duenne_untere_hand_mit_wav_druck_pruefen" => 4,
        "komponisten_override_noetig" => 3,
        "schwacher_grenzfall_gegenhoeren" => 2,
        "echte_kompositorische_leere_wahrscheinlich" => 0,
        "nur_resonanz_oder_nachhall_wahrscheinlich" => 0,
        _ => -1,
    }
}

#[derive(Debug, Clone)]
struct CondensationGapCandidate {
    measure: usize,
    energy_milli: i64,
    bass_proxy_milli: i64,
    class_label: String,
}

fn select_single_condensation_preview_measure(
    reference: &ReferenceReport,
    render_xml: &str,
    measure_count_hint: usize,
) -> Option<usize> {
    let wav = reference.wav.as_ref()?;
    let (samples, _sample_rate) = decode_wav_mono_samples(&wav.path)?;
    let staff_stats = collect_wav_function_gap_staff_stats(render_xml);
    let max_scope_from_xml = staff_stats.keys().copied().max().unwrap_or(0);
    let measure_count = measure_count_hint.max(max_scope_from_xml).max(1);
    let mut feature_rows: Vec<(usize, i64, i64, i64)> = Vec::new();
    let mut energy_sum = 0_i64;
    let mut attack_sum = 0_i64;
    let mut bass_sum = 0_i64;
    for scope in 1..=measure_count {
        let start = (scope - 1) * samples.len() / measure_count;
        let end = (scope * samples.len() / measure_count).min(samples.len());
        let slice = &samples[start..end];
        let (energy, attack, _flux) = audio_salience_features(slice);
        let bass_proxy = wav_low_register_proxy(slice, energy);
        energy_sum += energy;
        attack_sum += attack;
        bass_sum += bass_proxy;
        feature_rows.push((scope, energy, attack, bass_proxy));
    }
    let avg_energy = energy_sum / measure_count as i64;
    let avg_attack = attack_sum / measure_count as i64;
    let avg_bass = bass_sum / measure_count as i64;

    let mut candidates = Vec::new();
    for (scope, energy, attack, bass_proxy) in feature_rows {
        let stats = staff_stats.get(&scope).cloned().unwrap_or_default();
        if stats.staff1_visible_pitched == 0 || stats.staff2_visible_pitched > 1 {
            continue;
        }
        let (class_label, _recommendation) =
            classify_wav_function_gap_row(WavFunctionGapRowClassInput {
                staff1_visible_pitched: stats.staff1_visible_pitched,
                staff2_visible_pitched: stats.staff2_visible_pitched,
                energy_milli: energy,
                attack_milli: attack,
                bass_proxy_milli: bass_proxy,
                avg_energy_milli: avg_energy,
                avg_attack_milli: avg_attack,
                avg_bass_proxy_milli: avg_bass,
            });
        if class_label == "unauffaellig" {
            continue;
        }
        candidates.push(CondensationGapCandidate {
            measure: scope,
            energy_milli: energy,
            bass_proxy_milli: bass_proxy,
            class_label,
        });
    }
    candidates.sort_by(|a, b| {
        wav_function_gap_priority(&b.class_label)
            .cmp(&wav_function_gap_priority(&a.class_label))
            .then_with(|| b.bass_proxy_milli.cmp(&a.bass_proxy_milli))
            .then_with(|| b.energy_milli.cmp(&a.energy_milli))
            .then_with(|| a.measure.cmp(&b.measure))
    });
    candidates
        .into_iter()
        .take(24)
        .find(|row| {
            matches!(
                row.class_label.as_str(),
                "echte_fehlende_linke_hand_funktion_pruefen"
                    | "duenne_untere_hand_mit_wav_druck_pruefen"
                    | "komponisten_override_noetig"
            )
        })
        .map(|row| row.measure)
}

fn materialize_single_condensation_render_preview(
    render_xml: &mut String,
    measure: usize,
) -> bool {
    let before_signature = simple_xml_event_signature(render_xml);
    let (written, _anchor, _anchor_duration, _follow_rest_duration) =
        materialize_single_condensation_anchor_in_render_xml(render_xml, measure);
    written && before_signature != simple_xml_event_signature(render_xml)
}










fn simple_xml_event_signature(xml: &str) -> (usize, usize, usize) {
    (xml.matches("<note").count(), xml.matches("<rest").count(), xml.len())
}

fn materialize_single_condensation_anchor_in_render_xml(
    xml: &mut String,
    measure_number: usize,
) -> (bool, Option<CondensationPitchAnchor>, i32, i32) {
    let Some(anchor) = select_condensation_anchor_from_neighboring_lower_staff(xml, measure_number) else {
        return (false, None, 0, 0);
    };
    let Some((measure_start, measure_end, _)) = find_measure_bounds_by_number(xml, measure_number) else {
        return (false, Some(anchor), 0, 0);
    };
    let measure = &xml[measure_start..measure_end];
    if !collect_visible_staff2_note_infos(measure).is_empty() {
        return (false, Some(anchor), 0, 0);
    }
    let Some((local_start, local_end, full_duration)) = find_visible_staff2_full_measure_rest_note(measure) else {
        return (false, Some(anchor), 0, 0);
    };
    let anchor_duration = (full_duration / 4).max(1);
    let follow_rest_duration = full_duration.saturating_sub(anchor_duration).max(1);
    let replacement = render_preview_anchor_plus_rest_note(&anchor, anchor_duration, follow_rest_duration);
    let global_start = measure_start + local_start;
    let global_end = measure_start + local_end;
    let written = crate::structural_write_authority::replace_bound_condensation_rest_authorized(
        xml,
        global_start,
        global_end,
        replacement,
    );
    (written, Some(anchor), anchor_duration, follow_rest_duration)
}

fn find_visible_staff2_full_measure_rest_note(measure: &str) -> Option<(usize, usize, i32)> {
    let mut scan = 0usize;
    while let Some(rel) = measure[scan..].find("<note") {
        let start = scan + rel;
        let close_rel = measure[start..].find("</note>")?;
        let end = start + close_rel + "</note>".len();
        let note = &measure[start..end];
        if !is_hidden_note(note)
            && note.contains("<rest")
            && extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string()).trim() == "2"
        {
            let duration = extract_first_text_from_fragment(note, "duration")
                .and_then(|v| v.trim().parse::<i32>().ok())
                .unwrap_or(0);
            let measure_rest = note.contains("<rest measure=\"yes\"") || note.contains("<type>whole</type>");
            if duration > 0 && measure_rest {
                return Some((start, end, duration));
            }
        }
        scan = end;
    }
    None
}

fn select_condensation_anchor_from_neighboring_lower_staff(xml: &str, measure_number: usize) -> Option<CondensationPitchAnchor> {
    // Erst den unmittelbar vorherigen unteren Handanker nehmen: das ist spielerisch stabiler
    // als ein isolierter aus dem Nichts konstruierter Pitch. Danach vorwärts suchen.
    for distance in 1..=2usize {
        if let Some(prev) = measure_number.checked_sub(distance)
            && let Some(anchor) = last_visible_staff2_pitch_anchor_in_measure(xml, prev)
        {
            return Some(anchor);
        }
    }
    for distance in 1..=2usize {
        let next = measure_number + distance;
        if let Some(anchor) = first_visible_staff2_pitch_anchor_in_measure(xml, next) {
            return Some(anchor);
        }
    }
    None
}

fn first_visible_staff2_pitch_anchor_in_measure(xml: &str, measure_number: usize) -> Option<CondensationPitchAnchor> {
    let (start, end, _) = find_measure_bounds_by_number(xml, measure_number)?;
    let measure = &xml[start..end];
    collect_visible_staff2_pitch_anchors(measure).into_iter().next()
}

fn last_visible_staff2_pitch_anchor_in_measure(xml: &str, measure_number: usize) -> Option<CondensationPitchAnchor> {
    let (start, end, _) = find_measure_bounds_by_number(xml, measure_number)?;
    let measure = &xml[start..end];
    collect_visible_staff2_pitch_anchors(measure).into_iter().last()
}

fn collect_visible_staff2_pitch_anchors(measure: &str) -> Vec<CondensationPitchAnchor> {
    let mut anchors = Vec::new();
    let mut scan = 0usize;
    while let Some(rel) = measure[scan..].find("<note") {
        let start = scan + rel;
        let Some(close_rel) = measure[start..].find("</note>") else { break; };
        let end = start + close_rel + "</note>".len();
        let note = &measure[start..end];
        let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
        if !is_hidden_note(note)
            && staff.trim() == "2"
            && !note.contains("<rest")
            && let Some(anchor) = pitch_anchor_from_note_fragment(note)
            && (36..=60).contains(&anchor.midi)
        {
            anchors.push(anchor);
        }
        scan = end;
    }
    anchors
}

fn pitch_anchor_from_note_fragment(note: &str) -> Option<CondensationPitchAnchor> {
    let step = extract_first_text_from_fragment(note, "step")?;
    let alter = extract_first_text_from_fragment(note, "alter")
        .and_then(|value| value.trim().parse::<i32>().ok())
        .unwrap_or(0);
    let octave = extract_first_text_from_fragment(note, "octave")?.trim().parse::<i32>().ok()?;
    let midi = midi_for_pitch(&step, alter, octave)?;
    Some(CondensationPitchAnchor {
        midi,
        step,
        alter,
        octave,
    })
}

fn render_preview_anchor_plus_rest_note(anchor: &CondensationPitchAnchor, anchor_duration: i32, follow_rest_duration: i32) -> String {
    let alter_xml = if anchor.alter == 0 {
        String::new()
    } else {
        format!("      <alter>{}</alter>\n", anchor.alter)
    };
    let accidental_xml = match anchor.alter {
        -2 => "      <accidental>flat-flat</accidental>\n".to_string(),
        -1 => "      <accidental>flat</accidental>\n".to_string(),
        1 => "      <accidental>sharp</accidental>\n".to_string(),
        2 => "      <accidental>double-sharp</accidental>\n".to_string(),
        _ => String::new(),
    };
    format!(
        "    <note>\n      <pitch>\n        <step>{}</step>\n{}        <octave>{}</octave>\n      </pitch>\n      <duration>{}</duration>\n      <voice>2</voice>\n      <type>quarter</type>\n{}      <stem>down</stem>\n      <staff>2</staff>\n    </note>\n    <note>\n      <rest/>\n      <duration>{}</duration>\n      <voice>2</voice>\n      <type>half</type>\n      <dot/>\n      <staff>2</staff>\n    </note>",
        anchor.step.trim(),
        alter_xml,
        anchor.octave,
        anchor_duration,
        accidental_xml,
        follow_rest_duration
    )
}









