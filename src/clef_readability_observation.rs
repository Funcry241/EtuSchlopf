//! Writer-free observation primitives for lower-staff clef readability.
//!
//! v0.10.1708 ClefReadabilityObservationOwnership centralizes only facts that
//! were previously reconstructed independently by notation readback and Rustig
//! course planning. It deliberately owns no clef preference, threshold, cost,
//! candidate selection, XML mutation, or write authorization.

/// Current-document attachment only; this does not reconstruct source lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LowerChordAttack {
    First { head: usize },
    Member { head: usize },
    Unknown,
}

impl LowerChordAttack {
    pub(crate) fn head(self) -> Option<usize> {
        match self {
            Self::First { head } | Self::Member { head } => Some(head),
            Self::Unknown => None,
        }
    }

    pub(crate) fn counted(self) -> bool {
        matches!(self, Self::First { .. })
    }
}

#[derive(Default)]
pub(crate) struct LowerChordObservation {
    head: Option<usize>,
    lower_seen: bool,
}

impl LowerChordObservation {
    pub(crate) fn observe(
        &mut self,
        gap: &str,
        start: usize,
        note: &str,
        visible_lower_pitch: bool,
    ) -> LowerChordAttack {
        if chord_attachment_boundary(gap) {
            self.head = None;
            self.lower_seen = false;
        }
        if !note.contains("<chord") {
            self.head = Some(start);
            self.lower_seen = false;
        }
        let relation = match self.head {
            Some(head) if self.lower_seen => LowerChordAttack::Member { head },
            Some(head) => LowerChordAttack::First { head },
            None => LowerChordAttack::Unknown,
        };
        self.lower_seen |= visible_lower_pitch;
        relation
    }
}

fn chord_attachment_boundary(mut gap: &str) -> bool {
    while let Some(start) = gap.find('<') {
        gap = &gap[start..];
        let terminator = if gap.starts_with("<!--") {
            Some("-->")
        } else if gap.starts_with("<![CDATA[") {
            Some("]]>")
        } else {
            None
        };
        if let Some(terminator) = terminator {
            let Some(end) = gap.find(terminator) else {
                return true;
            };
            gap = &gap[end + terminator.len()..];
            continue;
        }
        let Some(end) = gap.find('>') else {
            return true;
        };
        let name = gap[1..end]
            .split(|c: char| c.is_whitespace() || c == '/')
            .find(|s| !s.is_empty());
        if matches!(
            name,
            Some("backup" | "forward" | "attributes" | "measure" | "part")
        ) {
            return true;
        }
        gap = &gap[end + 1..];
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ClefPitchObservation {
    diatonic_position: i64,
}

impl ClefPitchObservation {
    pub(crate) fn shifted(self, diatonic_steps: i64) -> Self {
        Self {
            diatonic_position: self.diatonic_position + diatonic_steps,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PianoClef {
    TrebleG2,
    BassF4,
}

impl PianoClef {
    pub(crate) fn ledger_lines(self, pitch: ClefPitchObservation) -> i64 {
        let (bottom_line, top_line) = match self {
            Self::TrebleG2 => (4 * 7 + 2, 5 * 7 + 3), // E4 .. F5
            Self::BassF4 => (2 * 7 + 4, 3 * 7 + 5),   // G2 .. A3
        };
        let position = pitch.diatonic_position;
        if position < bottom_line {
            (bottom_line - position) / 2
        } else if position > top_line {
            (position - top_line) / 2
        } else {
            0
        }
    }
}

pub(crate) fn observe_pitch(note: &str) -> Option<ClefPitchObservation> {
    let step = first_tag_text(note, "step")?;
    let octave = first_tag_text(note, "octave")?.parse::<i64>().ok()?;
    let step_index = match step.as_str() {
        "C" => 0,
        "D" => 1,
        "E" => 2,
        "F" => 3,
        "G" => 4,
        "A" => 5,
        "B" => 6,
        _ => return None,
    };
    Some(ClefPitchObservation {
        diatonic_position: octave * 7 + step_index,
    })
}

pub(crate) fn observe_lower_staff_octave_display_shift(direction: &str) -> Option<i64> {
    if first_tag_text(direction, "staff").as_deref() != Some("2") {
        return None;
    }
    let start = direction.find("<octave-shift")?;
    let end = direction[start..]
        .find('>')
        .map(|offset| start + offset + 1)?;
    let opening = &direction[start..end];
    if opening.contains("type=\"stop\"") || opening.contains("type='stop'") {
        return Some(0);
    }
    let octaves = if opening.contains("size=\"15\"") || opening.contains("size='15'") {
        2
    } else {
        1
    };
    if opening.contains("type=\"up\"") || opening.contains("type='up'") {
        Some(7 * octaves)
    } else if opening.contains("type=\"down\"") || opening.contains("type='down'") {
        Some(-7 * octaves)
    } else {
        None
    }
}

fn first_tag_text(text: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = text.find(&open)? + open.len();
    let end = text[start..].find(&close)? + start;
    Some(text[start..end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::{PianoClef, observe_lower_staff_octave_display_shift, observe_pitch};

    #[test]
    fn b4_is_four_bass_ledgers_and_zero_treble_ledgers() {
        let pitch = observe_pitch(
            "<note><pitch><step>B</step><octave>4</octave></pitch><staff>2</staff></note>",
        )
        .unwrap();
        assert_eq!(PianoClef::BassF4.ledger_lines(pitch), 4);
        assert_eq!(PianoClef::TrebleG2.ledger_lines(pitch), 0);
    }

    #[test]
    fn lower_staff_octave_display_observation_preserves_existing_semantics() {
        assert_eq!(
            observe_lower_staff_octave_display_shift(
                "<direction><direction-type><octave-shift type=\"up\" size=\"8\"/></direction-type><staff>2</staff></direction>",
            ),
            Some(7)
        );
        assert_eq!(
            observe_lower_staff_octave_display_shift(
                "<direction><direction-type><octave-shift type=\"down\" size=\"15\"/></direction-type><staff>2</staff></direction>",
            ),
            Some(-14)
        );
        assert_eq!(
            observe_lower_staff_octave_display_shift(
                "<direction><direction-type><octave-shift type=\"stop\"/></direction-type><staff>2</staff></direction>",
            ),
            Some(0)
        );
    }
}

#[path = "clef_readability_observation/temporal.rs"]
pub(crate) mod temporal;
