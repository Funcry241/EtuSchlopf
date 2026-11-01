//! Read-only comparison of source attacks and their final typed placement.
use super::material_conservation::MaterialPopulation;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    source: usize,
    raw: (i64, i32),
    placed: (i64, i32),
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Coincidences {
    introduced: Vec<(usize, usize)>,
    preserved: Vec<(usize, usize)>,
}

fn classify(points: &[Point]) -> Coincidences {
    let mut groups = BTreeMap::<(i64, i32), Vec<&Point>>::new();
    for point in points {
        groups.entry(point.placed).or_default().push(point);
    }
    let mut result = Coincidences::default();
    for group in groups.values_mut() {
        group.sort_by_key(|point| point.source);
        for (index, a) in group.iter().enumerate() {
            for b in &group[index + 1..] {
                let pair = (a.source, b.source);
                if a.raw == b.raw {
                    result.preserved.push(pair);
                } else {
                    result.introduced.push(pair);
                }
            }
        }
    }
    result.introduced.sort_unstable();
    result.preserved.sort_unstable();
    result
}

fn collect_points(
    input: &MaterialPopulation<'_>,
    absolute_positions: &[Option<i64>],
) -> Result<(Vec<Point>, usize), &'static str> {
    if !input.authority_bound || input.source_divisions <= 0 {
        return Err("source_authority_or_time_domain_unavailable");
    }
    let sources = input
        .sources
        .iter()
        .map(|row| (row.ordinal, row))
        .collect::<BTreeMap<_, _>>();
    let retained = input.retained_ids.iter().copied().collect::<BTreeSet<_>>();
    let planned = input
        .sources
        .iter()
        .filter(|row| row.retained)
        .map(|row| row.ordinal)
        .collect::<BTreeSet<_>>();
    let attack_ids = input
        .attacks
        .iter()
        .map(|row| row.source)
        .collect::<BTreeSet<_>>();
    let active = input.attacks.iter().filter(|row| row.exists).count();
    if sources.len() != input.sources.len()
        || sources.len() != input.canonical
        || retained.len() != input.retained_ids.len()
        || retained.len() != input.lineage
        || retained != planned
        || !retained.iter().all(|id| sources.contains_key(id))
        || attack_ids.len() != input.attacks.len()
        || !attack_ids.is_subset(&retained)
        || active != input.active
        || absolute_positions.len() != input.attacks.len()
    {
        return Err("incomplete_or_ambiguous_source_population");
    }
    let mut points = Vec::new();
    let mut grace = 0;
    for (attack, absolute) in input.attacks.iter().zip(absolute_positions) {
        if !attack.exists {
            continue;
        }
        if attack.grace {
            grace += 1;
            continue;
        }
        let source = sources.get(&attack.source).ok_or("source_missing")?;
        let position = absolute.ok_or("absolute_notation_time_unavailable")?;
        points.push(Point {
            source: attack.source,
            raw: (source.raw_onset, source.pitch),
            placed: (position, attack.pitch),
        });
    }
    Ok((points, grace))
}

pub(crate) fn append_coincidences(
    out: &mut String,
    input: &MaterialPopulation<'_>,
    absolute_positions: &[Option<i64>],
) {
    out.push_str("R6Definition: no_new_same_onset_same_key_coincidence; systems_and_voices_ignored; unit unordered_source_attack_pair; tie_continuations_not_separate_attacks; grace_excluded; raw_time source_divisions; placed_time 96_quarter_ticks; typed_metric_measure_starts; musical_authority none; release_authority none.\n");
    let (points, grace) = match collect_points(input, absolute_positions) {
        Ok(rows) => rows,
        Err(reason) => {
            let _ = writeln!(
                out,
                "R6Coincidence: status UNKNOWN; reason {reason}; introduced_pairs unknown; preserved_source_unison_pairs unknown."
            );
            return;
        }
    };
    let result = classify(&points);
    let _ = writeln!(
        out,
        "R6Coincidence: status COMPLETE; measured_attacks {}; excluded_grace {}; introduced_pairs {}; preserved_source_unison_pairs {}; raw_divisions {}; placed_divisions 96.",
        points.len(),
        grace,
        result.introduced.len(),
        result.preserved.len(),
        input.source_divisions
    );
    let rows = input
        .attacks
        .iter()
        .map(|row| (row.source, row))
        .collect::<BTreeMap<_, _>>();
    let by_id = points
        .iter()
        .map(|point| (point.source, point))
        .collect::<BTreeMap<_, _>>();
    for (kind, pairs) in [
        ("introduced", &result.introduced),
        ("preserved_source_unison", &result.preserved),
    ] {
        for &(a, b) in pairs {
            let pa = by_id[&a];
            let pb = by_id[&b];
            let aa = rows[&a];
            let ab = rows[&b];
            let _ = writeln!(
                out,
                "R6Pair: kind {kind}; sources [{a},{b}]; placed_absolute_tick {}; midi {}; raw_a {:?}; raw_b {:?}; measure_a {}; onset_a {}; staff_a {:?}; voice_a {:?}; measure_b {}; onset_b {}; staff_b {:?}; voice_b {:?}.",
                pa.placed.0,
                pa.placed.1,
                pa.raw,
                pb.raw,
                aa.measure,
                aa.onset,
                aa.staff,
                aa.voice,
                ab.measure,
                ab.onset,
                ab.staff,
                ab.voice
            );
        }
    }
    let involved = result
        .introduced
        .iter()
        .flat_map(|&(a, b)| [a, b])
        .collect::<BTreeSet<_>>();
    for source in involved {
        let memberships = input
            .origins
            .iter()
            .enumerate()
            .filter(|(_, row)| row.sources.contains(&source))
            .map(|(index, row)| (index + 1, row.owner))
            .collect::<Vec<_>>();
        let _ = writeln!(
            out,
            "R6OriginMembership: source {source}; records_and_owners {memberships:?}; attribution membership_only; exact_placer_cause not_proven."
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn point(source: usize, raw: i64, placed: i64) -> Point {
        Point {
            source,
            raw: (raw, 50),
            placed: (placed, 50),
        }
    }
    #[test]
    fn separated_attacks_that_meet_are_reported() {
        assert_eq!(
            classify(&[point(8, 10, 30), point(2, 20, 30)]).introduced,
            vec![(2, 8)]
        );
    }
    #[test]
    fn original_unison_remains_a_separate_category() {
        let result = classify(&[point(3, 10, 30), point(7, 10, 30)]);
        assert!(result.introduced.is_empty());
        assert_eq!(result.preserved, vec![(3, 7)]);
    }
    #[test]
    fn mixed_group_counts_every_unordered_pair() {
        let result = classify(&[point(1, 10, 30), point(2, 10, 30), point(3, 20, 30)]);
        assert_eq!(result.introduced, vec![(1, 3), (2, 3)]);
        assert_eq!(result.preserved, vec![(1, 2)]);
    }
    #[test]
    fn pitch_and_onset_both_belong_to_each_key() {
        let mut other = point(2, 10, 30);
        other.placed.1 = 51;
        assert_eq!(
            classify(&[point(1, 10, 30), other, point(3, 10, 31)]),
            Coincidences::default()
        );
        other.placed.1 = 50;
        other.raw.1 = 51;
        assert_eq!(
            classify(&[point(1, 10, 30), other]).introduced,
            vec![(1, 2)]
        );
    }
    #[test]
    fn report_order_is_independent_of_input_order() {
        let mut rows = vec![
            point(9, 20, 40),
            point(1, 10, 30),
            point(2, 11, 30),
            point(7, 10, 40),
        ];
        let result = classify(&rows);
        rows.reverse();
        assert_eq!(result, classify(&rows));
    }
    #[test]
    fn bound_population_excludes_inactive_and_grace_and_rejects_missing_time() {
        use super::super::material_conservation::{AttackRow, SourceRow};
        let sources = (0..4)
            .map(|ordinal| SourceRow {
                ordinal,
                retained: true,
                raw_onset: ordinal as i64 * 10,
                raw_duration: 10,
                pitch: 50,
                removal_reason: None,
                restified_at_entry: Some(false),
            })
            .collect::<Vec<_>>();
        let attacks = (0..4)
            .map(|source| AttackRow {
                source,
                exists: source != 3,
                grace: source == 2,
                measure: 1,
                onset: 30,
                end: 40,
                pitch: 50,
                staff: if source == 0 { "1" } else { "2" },
                voice: if source == 0 { "1" } else { "3" },
                notated_duration: Some(10),
            })
            .collect::<Vec<_>>();
        let mut input = MaterialPopulation {
            authority_bound: true,
            canonical: 4,
            lineage: 4,
            active: 3,
            source_divisions: 192,
            sources: &sources,
            retained_ids: &[0, 1, 2, 3],
            attacks: &attacks,
            origins: &[],
        };
        let positions = [Some(30), Some(30), None, None];
        let (points, grace) = collect_points(&input, &positions).unwrap();
        assert_eq!(grace, 1);
        assert_eq!(points.len(), 2);
        assert_eq!(classify(&points).introduced, vec![(0, 1)]);
        let mut out = String::new();
        append_coincidences(&mut out, &input, &positions);
        assert!(
            out.contains(
                "status COMPLETE; measured_attacks 2; excluded_grace 1; introduced_pairs 1"
            )
        );
        assert!(out.contains("exact_placer_cause not_proven"));
        assert!(collect_points(&input, &[None, Some(30), None, None]).is_err());
        assert!(collect_points(&input, &positions[..3]).is_err());
        input.active = 4;
        assert!(collect_points(&input, &positions).is_err());
        input.active = 3;
        input.retained_ids = &[0, 1, 2, 2];
        assert!(collect_points(&input, &positions).is_err());
        input.retained_ids = &[0, 1, 2, 3];
        input.attacks = &attacks[..3];
        // An inactive source need not retain an attack row.
        assert!(collect_points(&input, &positions[..3]).is_ok());
        input.attacks = &attacks[..2];
        assert!(collect_points(&input, &positions[..2]).is_err());
        input.attacks = &attacks;
        input.authority_bound = false;
        let mut out = String::new();
        append_coincidences(&mut out, &input, &positions);
        assert!(out.contains("status UNKNOWN"));
        assert!(!out.contains("introduced_pairs 0"));
    }
}
