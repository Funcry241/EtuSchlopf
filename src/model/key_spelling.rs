const NATURAL_PITCHES: [(&str, i32); 7] = [
    ("C", 0),
    ("D", 2),
    ("E", 4),
    ("F", 5),
    ("G", 7),
    ("A", 9),
    ("B", 11),
];

pub(crate) fn natural_pitch_class(step: &str) -> Option<i32> {
    NATURAL_PITCHES
        .iter()
        .find_map(|&(letter, pitch_class)| (letter == step).then_some(pitch_class))
}

pub(crate) fn signature_alter(fifths: i32, step: &str) -> i32 {
    const SHARPS: [&str; 7] = ["F", "C", "G", "D", "A", "E", "B"];
    const FLATS: [&str; 7] = ["B", "E", "A", "D", "G", "C", "F"];

    let count = fifths.unsigned_abs().min(7) as usize;
    let order = if fifths < 0 { &FLATS } else { &SHARPS };
    if order[..count].contains(&step) {
        fifths.signum()
    } else {
        0
    }
}

pub(crate) fn diatonic_spelling(fifths: i32, pitch: i32) -> Option<(&'static str, i32)> {
    NATURAL_PITCHES.iter().find_map(|&(step, natural)| {
        let alter = signature_alter(fifths, step);
        ((natural + alter).rem_euclid(12) == pitch.rem_euclid(12)).then_some((step, alter))
    })
}

pub(crate) fn diatonic_pitch(fifths: i32, midi: i32) -> Option<(&'static str, i32, i32)> {
    let (step, alter) = diatonic_spelling(fifths, midi)?;
    let octave = (midi - natural_pitch_class(step)? - alter).div_euclid(12) - 1;
    Some((step, alter, octave))
}
