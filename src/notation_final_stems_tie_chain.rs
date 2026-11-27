use super::{
    Direction, Group, PrintContext, StemEdit, TieSides, beam_groups, collect_attacks,
    compare_groups, isolated_direction,
};
use roxmltree::Document;
use std::collections::BTreeMap;
use std::ops::Range;

struct Unit {
    direction: Option<Direction>,
    edits: Vec<StemEdit>,
    neighbors: Vec<usize>,
}

fn proposals_for_unit(
    group: &Group<'_>,
    isolated: bool,
    memberships: &BTreeMap<(usize, usize), Vec<usize>>,
) -> Unit {
    let covers_ties = group.attacks.iter().flat_map(|attack| &attack.heads).all(|head| {
        let occurrence_count = memberships.get(&head.source).map_or(0, Vec::len);
        head.ordinary_stem && head.ties.is_some_and(|ties| {
            usize::from(ties.sides != TieSides::default()) == occurrence_count
        })
    });
    let direction = if isolated && covers_ties && group.writable() {
        isolated_direction(group)
    } else {
        None
    };
    let mut edits = Vec::new();
    if let Some(direction) = direction {
        for (start, end, existing) in group.attacks.iter().flat_map(|attack| &attack.heads)
            .filter_map(|head| head.stem)
        {
            if existing != direction.xml() {
                edits.push((start, end, direction.xml()));
            }
        }
    }
    Unit { direction, edits, neighbors: Vec::new() }
}

// The occurrence owner binds the ranges. Musical location and pitch cannot
// substitute for that witness. Every group traversed by one of those ties
// contributes its complete rhythmic unit and independent direction decision.
pub(super) fn plan(xml: &str, authenticated_chains: &[Vec<Range<usize>>]) -> Vec<Vec<StemEdit>> {
    if authenticated_chains.is_empty() {
        return Vec::new();
    }
    let Ok(document) = Document::parse(xml) else {
        return Vec::new();
    };
    let mut memberships = BTreeMap::<_, Vec<usize>>::new();
    for (chain_index, chain) in authenticated_chains.iter().enumerate() {
        for range in chain {
            memberships.entry((range.start, range.end)).or_default().push(chain_index);
        }
    }

    let mut units = Vec::new();
    let mut chain_units = vec![Vec::new(); authenticated_chains.len()];
    for part in document.root_element().children().filter(|node| node.has_tag_name("part")) {
        let mut context = PrintContext::default();
        for measure in part.children().filter(|node| node.has_tag_name("measure")) {
            let print = context.for_measure(measure);
            let Some(printed) = collect_attacks(xml, measure, print) else {
                continue;
            };
            let groups = beam_groups(printed.attacks);
            let relations = compare_groups(&groups);
            for (group, relation) in groups.iter().zip(relations) {
                let chains = group.attacks.iter().flat_map(|attack| &attack.heads)
                    .filter_map(|head| memberships.get(&head.source)).flatten().copied();
                if chains.clone().next().is_none() {
                    continue;
                }
                for chain in chains {
                    chain_units[chain].push(units.len());
                }
                units.push(proposals_for_unit(group, relation.neighbors.is_empty(), &memberships));
            }
        }
    }

    for (chain, found) in authenticated_chains.iter().zip(chain_units) {
        let Some((&first, rest)) = found.split_first() else {
            continue;
        };
        let complete = chain.len() >= 2 && found.len() == chain.len()
            && chain.iter().all(|range| !range.is_empty())
            && chain.windows(2).all(|pair| pair[0].end <= pair[1].start);
        for &index in &found {
            if !complete {
                units[index].direction = None;
            }
        }
        for &index in rest {
            if first != index {
                units[first].neighbors.push(index);
                units[index].neighbors.push(first);
            }
        }
    }

    let mut visited = vec![false; units.len()];
    let mut result = Vec::new();
    for root in 0..units.len() {
        let mut pending = vec![root];
        let mut connected = Vec::new();
        while let Some(index) = pending.pop() {
            if !std::mem::replace(&mut visited[index], true) {
                connected.push(index);
                pending.extend(&units[index].neighbors);
            }
        }
        let expected = units[root].direction;
        if expected.is_some() && connected.iter().all(|&index| units[index].direction == expected) {
            let mut edits = connected.iter().flat_map(|&index| &units[index].edits)
                .copied().collect::<Vec<_>>();
            if !edits.is_empty() {
                edits.sort_unstable_by_key(|edit| edit.0);
                result.push(edits);
            }
        }
    }
    result.sort_unstable_by_key(|edits| edits[0].0);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TREBLE: &str = "<attributes><divisions>1</divisions><clef><sign>G</sign><line>2</line></clef></attributes>";
    const BASS: &str = "<attributes><clef><sign>F</sign><line>4</line></clef></attributes>";

    fn head(step: &str, octave: u8, stem: &str, tie: &str) -> String {
        let (sound, print) = if tie.is_empty() {
            (String::new(), String::new())
        } else {
            (format!("<tie type=\"{tie}\"/>"), format!("<notations><tied type=\"{tie}\"/></notations>"))
        };
        format!("<note><pitch><step>{step}</step><octave>{octave}</octave></pitch>\
            <duration>1</duration>{sound}<voice>1</voice><type>quarter</type>\
            <stem>{stem}</stem>{print}</note>")
    }

    fn score(body: &str) -> String {
        format!("<score-partwise><part id=\"p\"><measure number=\"first\">{TREBLE}{body}</measure></part></score-partwise>")
    }

    fn occurrence_ranges(xml: &str) -> Vec<Range<usize>> {
        Document::parse(xml).unwrap().descendants().filter(|note| note.has_tag_name("note"))
            .map(|note| note.range()).collect()
    }

    #[test]
    fn authenticated_chain_decides_both_units_and_is_a_fixpoint_across_barline() {
        for boundary in ["", "</measure><measure number=\"last\">"] {
            for (step, octave, before, after) in [("C", 5, "up", "down"), ("G", 4, "down", "up")] {
                let left = head(step, octave, before, "start");
                let right = head(step, octave, before, "stop");
                let xml = score(&format!("{left}{boundary}{right}"));
                let expected = score(&format!("{}{boundary}{}", head(step, octave, after, "start"), head(step, octave, after, "stop")));
                let [edits] = plan(&xml, &[occurrence_ranges(&xml)]).try_into().unwrap();
                assert_eq!(edits.len(), 2);
                assert!(super::super::matches_planned_stem_edit(&xml, &expected, &edits));
                assert!(plan(&expected, &[occurrence_ranges(&expected)]).is_empty());
                assert!(plan(&xml, &[]).is_empty());
            }
        }
    }

    #[test]
    fn different_clefs_can_disagree_and_preserve_the_entire_chain() {
        let left = head("G", 4, "down", "start");
        let right = head("G", 4, "up", "stop");
        let xml = score(&format!("{left}</measure><measure number=\"last\">{BASS}{right}"));
        assert!(plan(&xml, &[occurrence_ranges(&xml)]).is_empty());
    }

    #[test]
    fn a_missing_occurrence_does_not_suppress_an_independent_chain() {
        let left = head("C", 5, "up", "start");
        let incomplete = head("C", 5, "up", "stop").replace("<note>", "<note print-object=\"no\">");
        let independent_left = head("G", 4, "down", "start");
        let independent_right = head("G", 4, "down", "stop");
        let xml = score(&format!("{left}{incomplete}{independent_left}{independent_right}"));
        let ranges = occurrence_ranges(&xml);
        let [edits] = plan(&xml, &[ranges[..2].to_vec(), ranges[2..].to_vec()]).try_into().unwrap();
        assert_eq!(edits.len(), 2);
        assert!(edits[0].0 >= ranges[2].start);
        assert!(edits.iter().all(|edit| edit.2 == "<stem>up</stem>"));
    }

    #[test]
    fn shared_rhythmic_unit_requires_all_tied_heads_to_be_bound() {
        let a = head("C", 5, "up", "start");
        let b = head("E", 5, "up", "start").replace("<pitch>", "<chord/><pitch>");
        let c = head("C", 5, "up", "stop");
        let d = head("E", 5, "up", "stop").replace("<pitch>", "<chord/><pitch>");
        let xml = score(&format!("{a}{b}</measure><measure number=\"last\">{c}{d}"));
        let rows = occurrence_ranges(&xml);
        let first = vec![rows[0].clone(), rows[2].clone()];
        assert!(plan(&xml, std::slice::from_ref(&first)).is_empty());
        let second = vec![rows[1].clone(), rows[3].clone()];
        let [edits] = plan(&xml, &[first, second]).try_into().unwrap();
        assert_eq!(edits.len(), 4);
    }
}
