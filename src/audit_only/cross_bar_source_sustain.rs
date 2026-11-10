//! Read back the accepted source sustain error class from bound Render occurrences.
use crate::midi::notation_time::NotationTimePlan;
use crate::musicxml::parse_musicxml;
use crate::source_event::CANONICAL_DIVISIONS;
use crate::structural_write_authority::FinalPdfSourceOccurrenceContinuation;
use anyhow::{Context, Result, ensure};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fragment {
    onset: i64,
    end: i64,
    tie_stop: bool,
    visible: bool,
}

fn truncated_at_boundary(
    fragments: &mut [Fragment],
    source_onset: i64,
    boundary: i64,
    release: i64,
) -> bool {
    if source_onset >= boundary || release <= boundary {
        return false;
    }
    fragments.sort_unstable_by_key(|fragment| fragment.onset);
    let mut represented_end = source_onset;
    for (index, fragment) in fragments.iter().enumerate() {
        if !fragment.visible
            || fragment.onset != represented_end
            || fragment.end <= fragment.onset
            || fragment.tie_stop != (index != 0)
        {
            return false;
        }
        represented_end = fragment.end;
    }
    represented_end == boundary
}

fn exact_ticks(value: i64, from: i64, to: i64) -> Option<i64> {
    let scaled = i128::from(value) * i128::from(to);
    let from = i128::from(from);
    (from > 0 && scaled % from == 0)
        .then(|| i64::try_from(scaled / from).ok())
        .flatten()
}

pub(crate) fn report(
    continuation: &FinalPdfSourceOccurrenceContinuation,
    time: &NotationTimePlan,
) -> Result<String> {
    let score = parse_musicxml(continuation.xml())?;
    let [part] = score.parts.as_slice() else {
        anyhow::bail!("Unsupported part domain");
    };
    ensure!(
        part.measures.len() == time.metric.len(),
        "Different metric extents"
    );

    let mut by_measure = BTreeMap::new();
    for (index, measure) in part.measures.iter().enumerate() {
        let bar = time.measure(index)?;
        let rendered = measure.expected;
        ensure!(
            rendered.denominator > 0
                && rendered.numerator % rendered.denominator == 0
                && exact_ticks(
                    rendered.numerator / rendered.denominator,
                    measure.divisions,
                    time.metric.divisions
                ) == Some(bar.end - bar.start),
            "Different measure duration"
        );
        let number = measure.number.parse::<usize>()?;
        let pitched = measure
            .events
            .iter()
            .filter(|event| event.pitch.is_some())
            .collect::<Vec<_>>();
        ensure!(
            by_measure
                .insert(number, (bar, measure.divisions, pitched))
                .is_none(),
            "Nonunique measure locator"
        );
    }

    let rows = continuation.declassified_occurrence_identity_rows();
    let expected_rows = by_measure
        .values()
        .map(|(_, _, notes)| notes.len())
        .sum::<usize>();
    ensure!(rows.len() == expected_rows, "Incomplete occurrence binding");
    let mut seen = std::collections::BTreeSet::new();
    let mut chains = BTreeMap::<(usize, usize), Vec<Fragment>>::new();
    for (source, attack, measure, ordinal, _, _) in rows {
        ensure!(
            seen.insert((measure, ordinal)),
            "Nonunique occurrence binding"
        );
        let (bar, divisions, pitched) =
            by_measure.get(&measure).context("Missing bound measure")?;
        let event = pitched.get(ordinal).context("Missing bound occurrence")?;
        let onset = exact_ticks(event.start, *divisions, time.metric.divisions)
            .and_then(|position| bar.start.checked_add(position))
            .context("Inexact onset")?;
        let end = exact_ticks(event.duration, *divisions, time.metric.divisions)
            .and_then(|duration| onset.checked_add(duration))
            .context("Inexact release")?;
        ensure!(
            onset >= bar.start && end <= bar.end,
            "Occurrence outside metric support"
        );
        chains.entry((source, attack)).or_default().push(Fragment {
            onset,
            end,
            tie_stop: event.tie_stop,
            visible: event.visible,
        });
    }

    let mut findings = Vec::new();
    for ((source, attack), mut fragments) in chains {
        let event = time.source_event(source)?;
        let onset = exact_ticks(event.pos, CANONICAL_DIVISIONS, time.metric.divisions)
            .context("Inexact source onset")?;
        let release = event
            .pos
            .checked_add(event.len)
            .and_then(|end| exact_ticks(end, CANONICAL_DIVISIONS, time.metric.divisions))
            .context("Inexact source release")?;
        let Some(index) = time.metric.ordinal_at(onset) else {
            continue;
        };
        let boundary = time.measure(index)?.end;
        if truncated_at_boundary(&mut fragments, onset, boundary, release) {
            findings.push(format!("source={source} attack={attack} measure={} onset={onset} represented_end={boundary} source_end={release}", index + 1));
        }
    }
    Ok(format!(
        "CrossBarSourceSustainReadback: status=bound findings={} divisions={} authority=none\n{}",
        findings.len(),
        time.metric.divisions,
        findings.join("\n")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn head(onset: i64, end: i64) -> Fragment {
        Fragment {
            onset,
            end,
            tie_stop: false,
            visible: true,
        }
    }

    #[test]
    fn compares_exact_release_after_the_boundary() {
        let mut chain = [
            head(30, 50),
            Fragment {
                tie_stop: true,
                ..head(50, 60)
            },
        ];
        assert!(truncated_at_boundary(&mut chain, 30, 60, 81));
        assert!(!truncated_at_boundary(&mut chain, 30, 60, 60));
        assert!(!truncated_at_boundary(&mut chain, 31, 60, 81));
    }

    #[test]
    fn complete_and_non_contiguous_chains_are_distinct_from_boundary_truncation() {
        let mut chain = [
            head(30, 50),
            Fragment {
                tie_stop: true,
                ..head(50, 81)
            },
        ];
        assert!(!truncated_at_boundary(&mut chain, 30, 60, 81));
        chain[1].onset = 51;
        chain[1].end = 60;
        assert!(!truncated_at_boundary(&mut chain, 30, 60, 81));
        chain[1].onset = 50;
        chain[1].tie_stop = false;
        assert!(!truncated_at_boundary(&mut chain, 30, 60, 81));
    }

    #[test]
    fn independent_relocated_copy_does_not_change_local_classification() {
        let mut first = [head(30, 60)];
        let mut second = [head(270, 300)];
        assert!(truncated_at_boundary(&mut first, 30, 60, 81));
        assert!(truncated_at_boundary(&mut second, 270, 300, 321));
        assert!(truncated_at_boundary(&mut first, 30, 60, 81));
    }

    #[test]
    fn preserves_precision_between_source_and_written_divisions() {
        assert_eq!(exact_ticks(20, 120, 96), Some(16));
        assert_eq!(exact_ticks(21, 120, 96), None);
    }
}
