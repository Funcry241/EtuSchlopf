#[path = "src/sha256.rs"]
mod sha256;
#[path = "src/source_event.rs"]
mod source_event;
#[path = "src/source_time.rs"]
mod source_time;
#[path = "src/midi.rs"]
mod midi;

fn main() -> anyhow::Result<()> {
    let path = std::env::args_os().nth(1).expect("MIDI path");
    let decoded = midi::read_standard_midi(std::path::Path::new(&path))?;
    let proof = source_event::project_source_event_causality(
        &decoded.source_event_causal_ledger, &decoded.notes, decoded.source_ppq)?;
    let historical = source_event::project_source_events(&decoded.notes);
    for entry in &decoded.source_event_causal_ledger.entries {
        assert_eq!(i128::from(entry.canonical.pos) * i128::from(decoded.source_ppq),
            i128::from(entry.note_on.tick) * i128::from(decoded.time_domain.divisions));
        assert_eq!(i128::from(entry.canonical.len) * i128::from(decoded.source_ppq),
            i128::from(entry.note_off.tick - entry.note_on.tick) * i128::from(decoded.time_domain.divisions));
        assert!(decoded.metric_timeline.ordinal_at(entry.canonical.pos).is_some());
    }
    println!("format={} tracks={} ppq={} exact_divisions={} notes={} meter_declarations={} tempo_declarations={} measures={} first_meter={}/{} pairs={} raw_events={}",
        decoded.format, decoded.tracks, decoded.source_ppq, decoded.time_domain.divisions,
        decoded.notes.len(), decoded.time_signatures.len(), decoded.tempo_microseconds_per_quarter.len(),
        decoded.metric_timeline.len(), decoded.metric_timeline.measure(0).unwrap().beats,
        decoded.metric_timeline.measure(0).unwrap().beat_type, proof.entry_count, proof.raw_note_event_count);
    println!("historical_28_byte_event_fingerprint={} event_count={} duplicate_onsets={} terminal_extent_in_bound_units={} cross_track_pairs={} raw_pair_fingerprint={} causal_fingerprint={}",
        historical.event_fingerprint_sha256, historical.event_count,
        historical.duplicate_pitch_onset_events, historical.terminal_extent_ticks,
        proof.cross_track_pair_count, proof.raw_pair_fingerprint_sha256, proof.causal_fingerprint_sha256);
    Ok(())
}
