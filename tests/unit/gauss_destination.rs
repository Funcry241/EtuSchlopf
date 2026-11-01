use super::*;

fn fixture(markup: &str, hidden: bool) -> (HiddenChordToneRecoveryCandidate, Vec<VisibleChordToneSwapSlot>, Vec<AtomicNote>) {
    let scope = 1usize;
    let flag = if hidden { " print-object='no'" } else { "" };
    let xml = format!("<score-partwise><part id='synthetic'><measure number='{scope}'>
      <note><pitch><step>E</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>
      <note print-object='no'><chord/><pitch><step>C</step><alter>1</alter><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>
      <note><pitch><step>E</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note>
      <note{flag}><chord/><pitch><step>D</step><alter>-1</alter><octave>4</octave></pitch><duration>96</duration>{markup}<voice>1</voice><staff>1</staff></note>
      <note><pitch><step>G</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>
      </measure></part></score-partwise>");
    let notes = parse_atomic_notes(&xml);
    assert_eq!(notes.len(), 5);
    let source = &notes[1];
    let tone = HiddenChordToneRecoveryCandidate {
        measure: scope, staff: 1, start_tick: source.start, duration: source.duration,
        midi: source.pitch.as_ref().map(|p| p.midi), voice: source.voice.clone(),
        pitch_xml: String::new(), abs_start: source.note_start, abs_end: source.note_end, score: 0,
    };
    let slots = [2usize, 4].into_iter().map(|index| {
        let note = &notes[index];
        VisibleChordToneSwapSlot { measure: scope, staff: 1, start_tick: note.start,
            duration: note.duration, midi: note.pitch.as_ref().map(|p| p.midi),
            voice: note.voice.clone(), note_start: note.note_start, note_end: note.note_end }
    }).collect();
    (tone, slots, notes)
}

#[test]
fn start_stop_and_independent_equal_keys_are_occupied() {
    for markup in ["<tie type='start'/>", "<tie type='stop'/>", ""] {
        let (tone, slots, notes) = fixture(markup, false);
        assert!(!gauss_destination_is_available(&tone, &slots[0], &notes));
    }
}

#[test]
fn hidden_enharmonic_member_blocks_different_pitched_anchor() {
    let (tone, slots, notes) = fixture("<tie type='start'/>", true);
    assert_ne!(tone.midi, slots[0].midi);
    assert_eq!(tone.midi, notes[3].pitch.as_ref().map(|p| p.midi));
    assert!(!notes[3].visible);
    assert!(!gauss_destination_is_available(&tone, &slots[0], &notes));
}

#[test]
fn other_pitch_is_available() {
    let (tone, slots, mut notes) = fixture("", false);
    notes[3].pitch = slots[0].midi.map(|_| Pitch::new("F".into(), 0, 4));
    assert!(gauss_destination_is_available(&tone, &slots[0], &notes));
}

#[test]
fn separate_staff_or_voice_does_not_block_target_channel() {
    let (tone, slots, notes) = fixture("", false);
    let mut other_staff = notes.clone(); other_staff[3].staff = "2".into();
    assert!(gauss_destination_is_available(&tone, &slots[0], &other_staff));
    let mut other_voice = notes; other_voice[3].voice = "2".into();
    assert!(gauss_destination_is_available(&tone, &slots[0], &other_voice));
}

#[test]
fn other_measure_does_not_block_target() {
    let (tone, slots, mut notes) = fixture("", false);
    notes[3].measure = (tone.measure + 1).to_string();
    assert!(gauss_destination_is_available(&tone, &slots[0], &notes));
}

#[test]
fn half_open_intervals_allow_adjacent_release_or_attack() {
    let (tone, slots, notes) = fixture("", false);
    let mut before = notes.clone(); before[3].start = 0; before[3].duration = slots[0].start_tick;
    assert!(gauss_destination_is_available(&tone, &slots[0], &before));
    let mut after = notes; after[3].start = slots[0].start_tick + slots[0].duration;
    assert!(gauss_destination_is_available(&tone, &slots[0], &after));
}

#[test]
fn earlier_start_overlapping_hold_blocks_target() {
    let (tone, slots, mut notes) = fixture("<tie type='start'/>", false);
    notes[3].start = slots[0].start_tick - 1;
    assert!(!gauss_destination_is_available(&tone, &slots[0], &notes));
}

#[test]
fn preceding_planned_insertion_reserves_target() {
    let (tone, slots, mut notes) = fixture("", false);
    notes.remove(3);
    assert!(gauss_destination_is_available(&tone, &slots[0], &notes));
    let mut inserted = notes[1].clone();
    inserted.note_start = usize::MAX - 1; inserted.note_end = usize::MAX;
    inserted.start = slots[0].start_tick; inserted.duration = slots[0].duration;
    notes.push(inserted);
    assert!(!gauss_destination_is_available(&tone, &slots[0], &notes));
}

#[test]
fn real_selection_skips_cheaper_occupied_slot() {
    let (tone, slots, notes) = fixture("<tie type='start'/>", false);
    let used = BTreeSet::new();
    let baseline = consensus_gauss_anchor_for_tone(&tone, &slots, &used, 0, None).unwrap();
    assert_eq!(baseline.note_start, slots[0].note_start);
    let selected = consensus_gauss_anchor_for_tone(&tone, &slots, &used, 0, Some(&notes)).unwrap();
    assert_eq!(selected.note_start, slots[1].note_start);
}

#[test]
fn real_selection_with_only_conflicting_slot_has_no_target() {
    let (tone, slots, notes) = fixture("", false);
    assert!(consensus_gauss_anchor_for_tone(&tone, &slots[..1], &BTreeSet::new(), 0, Some(&notes)).is_none());
}

#[test]
fn own_source_interval_is_excluded() {
    let (tone, slots, mut notes) = fixture("", false);
    notes.remove(3); notes[1].start = slots[0].start_tick;
    assert!(gauss_destination_is_available(&tone, &slots[0], &notes));
}

#[test]
fn absent_source_and_stale_target_binding_are_rejected() {
    let (tone, slots, mut notes) = fixture("", false);
    notes.remove(3); notes[1].note_end += 1;
    assert!(!gauss_destination_is_available(&tone, &slots[0], &notes));
    let (tone, mut slots, mut notes) = fixture("", false);
    notes.remove(3); slots[0].start_tick += 1;
    assert!(!gauss_destination_is_available(&tone, &slots[0], &notes));
}

#[test]
fn missing_pitch_and_invalid_target_extent_are_rejected() {
    let (mut tone, mut slots, notes) = fixture("", false);
    tone.midi = None;
    assert!(!gauss_destination_is_available(&tone, &slots[0], &notes));
    tone.midi = notes[1].pitch.as_ref().map(|p| p.midi);
    slots[0].duration = 0;
    assert!(!gauss_destination_is_available(&tone, &slots[0], &notes));
    slots[0].start_tick = i64::MAX; slots[0].duration = 1;
    assert!(!gauss_destination_is_available(&tone, &slots[0], &notes));
}

#[test]
fn independent_duplication_preserves_local_decision() {
    for occupied in [false, true] {
        let (tone, slots, mut notes) = fixture("", false);
        if !occupied { notes[3].pitch = Some(Pitch::new("F".into(), 0, 4)); }
        let before = gauss_destination_is_available(&tone, &slots[0], &notes);
        let duplicates = notes.iter().cloned().map(|mut note| {
            note.measure = (tone.measure + 1).to_string();
            note.note_start += 10000; note.note_end += 10000; note
        }).collect::<Vec<_>>();
        notes.extend(duplicates);
        assert_eq!(gauss_destination_is_available(&tone, &slots[0], &notes), before);
    }
}

#[test]
fn time_translation_and_transposition_preserve_local_decision() {
    for occupied in [false, true] {
        let (mut tone, mut slots, mut notes) = fixture("", false);
        if !occupied { notes[3].pitch = Some(Pitch::new("F".into(), 0, 4)); }
        let before = gauss_destination_is_available(&tone, &slots[0], &notes);
        tone.start_tick += 384; tone.midi = tone.midi.map(|pitch| pitch + 7);
        for slot in &mut slots { slot.start_tick += 384; slot.midi = slot.midi.map(|pitch| pitch + 7); }
        for note in &mut notes { note.start += 384; if let Some(pitch) = &mut note.pitch { pitch.midi += 7; } }
        assert_eq!(gauss_destination_is_available(&tone, &slots[0], &notes), before);
    }
}
