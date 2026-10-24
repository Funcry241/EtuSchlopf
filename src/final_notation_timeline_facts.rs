//! Writer-free final-notation timeline facts shared by layout evidence consumers.
//!
//! This module owns only deterministic observation of already-final MusicXML.
//! It has no demand, capacity, system/page policy, renderer, mutation or writer authority.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinalNotationTimelineMeasureFacts {
    pub(crate) measure: usize,
    pub(crate) columns: usize,
    pub(crate) note_elements: usize,
    pub(crate) pitched_columns: usize,
    pub(crate) max_notes_per_column: usize,
    pub(crate) following_accidental_clearance_edges: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalNotationTimelineSurfaceEvent {
    onset: i64,
    duration: i64,
    staff: String,
    voice: String,
    visible: bool,
    rest_type: Option<String>,
    rest_display_step: Option<String>,
    rest_display_octave: Option<i32>,
    pitch_step: Option<String>,
    pitch_alter: i32,
    pitch_octave: Option<i32>,
    explicit_accidental: bool,
}

pub(crate) fn observe_final_notation_timeline_measure_facts(
    xml: &str,
) -> anyhow::Result<Vec<FinalNotationTimelineMeasureFacts>> {
    let document = roxmltree::Document::parse(xml)?;
    let part = document
        .descendants()
        .find(|node| node.has_tag_name("part"))
        .ok_or_else(|| {
            anyhow::anyhow!("RendererSystemBreakReadbackRhythmicShape: MusicXML part fehlt")
        })?;
    let measures = part
        .children()
        .filter(|node| node.is_element() && node.has_tag_name("measure"))
        .collect::<Vec<_>>();
    if measures.is_empty() {
        anyhow::bail!("RendererSystemBreakReadbackRhythmicShape: MusicXML besitzt keine measures");
    }

    let mut evidence = Vec::with_capacity(measures.len());
    for (index, measure) in measures.iter().enumerate() {
        let slot = index + 1;
        let number = measure
            .attribute("number")
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "RendererSystemBreakReadbackRhythmicShape: nichtnumerische Measure-Nummer an Slot {slot}"
                )
            })?;
        if number != slot {
            anyhow::bail!(
                "RendererSystemBreakReadbackRhythmicShape: Measure-Nummer {} stimmt nicht mit Slot {} ueberein",
                number,
                slot
            );
        }

        let mut cursor = 0_i64;
        let mut previous_note_onset = None;
        let mut columns = std::collections::BTreeMap::<i64, usize>::new();
        let mut pitched_columns = std::collections::BTreeSet::<i64>::new();
        let mut note_elements = 0usize;
        let mut surface_events = Vec::<FinalNotationTimelineSurfaceEvent>::new();

        for child in measure.children().filter(|node| node.is_element()) {
            if child.has_tag_name("note") {
                let is_chord = child
                    .children()
                    .any(|node| node.is_element() && node.has_tag_name("chord"));
                let is_grace = child
                    .children()
                    .any(|node| node.is_element() && node.has_tag_name("grace"));
                let is_rest = child
                    .children()
                    .any(|node| node.is_element() && node.has_tag_name("rest"));
                let onset = if is_chord {
                    previous_note_onset.ok_or_else(|| {
                        anyhow::anyhow!(
                            "RendererSystemBreakReadbackRhythmicShape: chord ohne vorherigen Onset in Takt {number}"
                        )
                    })?
                } else {
                    cursor
                };
                if onset < 0 {
                    anyhow::bail!(
                        "RendererSystemBreakReadbackRhythmicShape: negativer Onset in Takt {number}"
                    );
                }

                let duration = if is_grace {
                    0
                } else {
                    child_duration(&child, "note", number)?
                };
                let visible = child.attribute("print-object") != Some("no");
                let staff = child
                    .children()
                    .find(|node| node.is_element() && node.has_tag_name("staff"))
                    .and_then(|node| node.text())
                    .unwrap_or("1")
                    .trim()
                    .to_string();
                let voice = child
                    .children()
                    .find(|node| node.is_element() && node.has_tag_name("voice"))
                    .and_then(|node| node.text())
                    .unwrap_or("1")
                    .trim()
                    .to_string();
                let rest_type = is_rest.then(|| {
                    child
                        .children()
                        .find(|node| node.is_element() && node.has_tag_name("type"))
                        .and_then(|node| node.text())
                        .unwrap_or("")
                        .trim()
                        .to_string()
                });
                let rest = child
                    .children()
                    .find(|node| node.is_element() && node.has_tag_name("rest"));
                let rest_display_step = rest
                    .and_then(|node| {
                        node.children()
                            .find(|child| {
                                child.is_element() && child.has_tag_name("display-step")
                            })
                    })
                    .and_then(|node| node.text())
                    .map(|value| value.trim().to_string());
                let rest_display_octave = rest
                    .and_then(|node| {
                        node.children()
                            .find(|child| {
                                child.is_element() && child.has_tag_name("display-octave")
                            })
                    })
                    .and_then(|node| node.text())
                    .and_then(|value| value.trim().parse::<i32>().ok());
                let pitch = child
                    .children()
                    .find(|node| node.is_element() && node.has_tag_name("pitch"));
                let pitch_step = pitch
                    .and_then(|node| {
                        node.children()
                            .find(|child| child.is_element() && child.has_tag_name("step"))
                    })
                    .and_then(|node| node.text())
                    .map(|value| value.trim().to_string());
                let pitch_alter = pitch
                    .and_then(|node| {
                        node.children()
                            .find(|child| child.is_element() && child.has_tag_name("alter"))
                    })
                    .and_then(|node| node.text())
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                let pitch_octave = pitch
                    .and_then(|node| {
                        node.children()
                            .find(|child| child.is_element() && child.has_tag_name("octave"))
                    })
                    .and_then(|node| node.text())
                    .and_then(|value| value.trim().parse::<i32>().ok());
                let explicit_accidental = child
                    .children()
                    .any(|node| node.is_element() && node.has_tag_name("accidental"));

                note_elements = note_elements.saturating_add(1);
                *columns.entry(onset).or_default() += 1;
                if !is_rest {
                    pitched_columns.insert(onset);
                }
                surface_events.push(FinalNotationTimelineSurfaceEvent {
                    onset,
                    duration,
                    staff,
                    voice,
                    visible,
                    rest_type,
                    rest_display_step,
                    rest_display_octave,
                    pitch_step,
                    pitch_alter,
                    pitch_octave,
                    explicit_accidental,
                });

                if !is_chord {
                    previous_note_onset = Some(onset);
                    if !is_grace {
                        cursor = cursor.checked_add(duration).ok_or_else(|| {
                            anyhow::anyhow!(
                                "RendererSystemBreakReadbackRhythmicShape: Timeline-Overflow in Takt {number}"
                            )
                        })?;
                    }
                }
            } else if child.has_tag_name("backup") {
                previous_note_onset = None;
                let duration = child_duration(&child, "backup", number)?;
                cursor = cursor.checked_sub(duration).ok_or_else(|| {
                    anyhow::anyhow!(
                        "RendererSystemBreakReadbackRhythmicShape: Timeline-Underflow in Takt {number}"
                    )
                })?;
                if cursor < 0 {
                    anyhow::bail!(
                        "RendererSystemBreakReadbackRhythmicShape: backup vor Measure-Anfang in Takt {number}"
                    );
                }
            } else if child.has_tag_name("forward") {
                let duration = child_duration(&child, "forward", number)?;
                cursor = cursor.checked_add(duration).ok_or_else(|| {
                    anyhow::anyhow!(
                        "RendererSystemBreakReadbackRhythmicShape: Timeline-Overflow in Takt {number}"
                    )
                })?;
            }
        }

        evidence.push(FinalNotationTimelineMeasureFacts {
            measure: number,
            columns: columns.len(),
            note_elements,
            pitched_columns: pitched_columns.len(),
            max_notes_per_column: columns.values().copied().max().unwrap_or(0),
            following_accidental_clearance_edges:
                following_accidental_clearance_edge_count(&surface_events),
        });
    }
    Ok(evidence)
}

fn following_accidental_clearance_edge_count(events: &[FinalNotationTimelineSurfaceEvent]) -> usize {
    events
        .iter()
        .filter(|rest| {
            rest.visible
                && rest.pitch_step.is_none()
                && rest.duration > 0
                && rest.rest_type.as_deref() == Some("16th")
        })
        .filter(|rest| {
            let rest_end = rest.onset.saturating_add(rest.duration);
            events.iter().any(|next| {
                if !next.visible
                    || next.pitch_step.is_none()
                    || next.staff != rest.staff
                    || next.voice != rest.voice
                    || next.onset != rest_end
                {
                    return false;
                }
                let Some(next_step) = next.pitch_step.as_deref() else {
                    return false;
                };
                let Some(next_octave) = next.pitch_octave else {
                    return false;
                };

                let immediate_previous = events.iter().find(|previous| {
                    previous.visible
                        && previous.pitch_step.as_deref() == Some(next_step)
                        && previous.pitch_octave == Some(next_octave)
                        && previous.staff == rest.staff
                        && previous.voice == rest.voice
                        && previous.onset.saturating_add(previous.duration) == rest.onset
                });
                if immediate_previous.is_some_and(|previous| {
                    next.explicit_accidental || previous.pitch_alter != next.pitch_alter
                }) {
                    return true;
                }

                let carried_previous = events
                    .iter()
                    .filter(|previous| {
                        previous.visible
                            && previous.pitch_step.as_deref() == Some(next_step)
                            && previous.pitch_octave == Some(next_octave)
                            && previous.staff == rest.staff
                            && previous.voice == rest.voice
                            && previous.onset.saturating_add(previous.duration) < rest.onset
                    })
                    .max_by_key(|previous| {
                        (
                            previous.onset.saturating_add(previous.duration),
                            previous.onset,
                        )
                    });
                let Some(previous) = carried_previous else {
                    return false;
                };
                if !next.explicit_accidental && previous.pitch_alter == next.pitch_alter {
                    return false;
                }

                carried_flat_fifth_clearance_geometry(rest, next)
            })
        })
        .count()
}

fn carried_flat_fifth_clearance_geometry(
    rest: &FinalNotationTimelineSurfaceEvent,
    next: &FinalNotationTimelineSurfaceEvent,
) -> bool {
    if next.pitch_alter != -1 {
        return false;
    }
    let (Some(rest_step), Some(rest_octave), Some(next_step), Some(next_octave)) = (
        rest.rest_display_step.as_deref(),
        rest.rest_display_octave,
        next.pitch_step.as_deref(),
        next.pitch_octave,
    ) else {
        return false;
    };
    let (Some(rest_index), Some(next_index)) = (
        diatonic_index(rest_step, rest_octave),
        diatonic_index(next_step, next_octave),
    ) else {
        return false;
    };
    next_index.saturating_sub(rest_index) == 5
}

fn diatonic_index(step: &str, octave: i32) -> Option<i32> {
    let local = match step.trim() {
        "C" => 0,
        "D" => 1,
        "E" => 2,
        "F" => 3,
        "G" => 4,
        "A" => 5,
        "B" => 6,
        _ => return None,
    };
    Some(octave.saturating_mul(7).saturating_add(local))
}

fn child_duration(
    node: &roxmltree::Node<'_, '_>,
    kind: &str,
    measure: usize,
) -> anyhow::Result<i64> {
    let value = node
        .children()
        .find(|child| child.is_element() && child.has_tag_name("duration"))
        .and_then(|duration| duration.text())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "RendererSystemBreakReadbackRhythmicShape: {kind} ohne duration in Takt {measure}"
            )
        })?;
    let duration = value.trim().parse::<i64>().map_err(|_| {
        anyhow::anyhow!(
            "RendererSystemBreakReadbackRhythmicShape: ungueltige duration {value:?} in Takt {measure}"
        )
    })?;
    if duration < 0 {
        anyhow::bail!(
            "RendererSystemBreakReadbackRhythmicShape: negative duration in Takt {measure}"
        );
    }
    Ok(duration)
}

#[cfg(test)]
mod carried_accidental_state_clearance_geometry_tests {
    use super::observe_final_notation_timeline_measure_facts;

    fn score(notes: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="1"><attributes><divisions>24</divisions></attributes>{notes}</measure></part></score-partwise>"#
        )
    }

    fn note(step: &str, alter: i32, octave: i32, duration: i32) -> String {
        format!(
            "<note><pitch><step>{step}</step><alter>{alter}</alter><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>2</voice><type>16th</type><staff>2</staff></note>"
        )
    }

    fn rest(step: &str, octave: i32, duration: i32) -> String {
        format!(
            "<note><rest><display-step>{step}</display-step><display-octave>{octave}</display-octave></rest><duration>{duration}</duration><voice>2</voice><type>16th</type><staff>2</staff></note>"
        )
    }

    fn clearance_edges(notes: &str) -> usize {
        observe_final_notation_timeline_measure_facts(&score(notes))
            .expect("timeline facts")
            .into_iter()
            .next()
            .expect("measure facts")
            .following_accidental_clearance_edges
    }

    #[test]
    fn immediate_state_change_remains_a_clearance_edge() {
        let notes = format!(
            "{}{}{}",
            note("B", 0, 3, 24),
            rest("D", 3, 24),
            note("B", -1, 3, 24)
        );
        assert_eq!(clearance_edges(&notes), 1);
    }

    #[test]
    fn carried_flat_fifth_geometry_promotes_the_observed_clearance_class() {
        let notes = format!(
            "{}{}{}{}",
            note("B", 0, 3, 24),
            note("G", 0, 3, 24),
            rest("D", 3, 24),
            note("B", -1, 3, 24)
        );
        assert_eq!(clearance_edges(&notes), 1);
    }

    #[test]
    fn carried_flat_below_rest_is_not_promoted() {
        let notes = format!(
            "{}{}{}{}",
            note("B", 0, 2, 24),
            note("G", 0, 3, 24),
            rest("D", 3, 24),
            note("B", -1, 2, 24)
        );
        assert_eq!(clearance_edges(&notes), 0);
    }

    #[test]
    fn carried_flat_far_above_rest_is_not_promoted() {
        let notes = format!(
            "{}{}{}{}",
            note("G", 0, 4, 24),
            note("A", 0, 3, 24),
            rest("D", 3, 24),
            note("G", -1, 4, 24)
        );
        assert_eq!(clearance_edges(&notes), 0);
    }

    #[test]
    fn carried_natural_fifth_is_not_promoted_without_matching_flat_glyph_geometry() {
        let notes = format!(
            "{}{}{}{}",
            note("B", -1, 3, 24),
            note("G", 0, 3, 24),
            rest("D", 3, 24),
            note("B", 0, 3, 24)
        );
        assert_eq!(clearance_edges(&notes), 0);
    }
}
