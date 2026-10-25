//! Read-only safety evidence for a fresh lower attack inside a complete beam.
//! No cost, writer, receipt, occurrence identity or beam mutation is owned here.

use super::{
    contains_only_one_lower_clef, first_tag_text, ClefKind,
    observe_lower_staff_octave_display_shift, primary_beam_state, ClefSurface,
    LowerChordObservation, PrimaryBeamState, read_lower_clef,
};
use std::collections::{BTreeMap, BTreeSet};

struct Note<'a> {
    start: usize,
    end: usize,
    head: Option<usize>,
    onset: i64,
    duration: i64,
    raw: &'a str,
    voice: Option<String>,
    lower: bool,
    supported: bool,
    beam: Option<PrimaryBeamState>,
}

fn other_beam_conflict(notes: &[Note<'_>], own_start: usize, own_end: usize, at: i64) -> bool {
    let mut open = BTreeMap::<&str, (i64, bool)>::new();
    for note in notes.iter().filter(|n| n.head == Some(n.start)) {
        if own_start <= note.start && note.start < own_end { continue; }
        let lower = notes.iter().any(|n| n.head == Some(note.start) && n.lower);
        let Some(other) = note.voice.as_deref() else {
            if lower && note.beam.is_some() { return true; }
            continue;
        };
        match note.beam {
            Some(PrimaryBeamState::Begin) => {
                if open.insert(other, (note.onset, lower)).is_some_and(|(start, touches)| touches && start <= at && at <= note.onset) {
                    return true;
                }
            }
            Some(PrimaryBeamState::Continue) => {
                if let Some((_, touches)) = open.get_mut(other) {
                    *touches |= lower;
                } else if lower && note.onset.checked_add(note.duration).is_none_or(|end| end > at) {
                    return true;
                }
            }
            Some(PrimaryBeamState::End) => {
                let end = note.onset.checked_add(note.duration);
                if let Some((start, touches)) = open.remove(other) {
                    if (touches || lower) && end.is_none_or(|end| end <= start) { return true; }
                    if (touches || lower) && start <= at && end.is_none_or(|end| at < end) { return true; }
                } else if lower && end.is_none_or(|end| end > at) {
                    return true;
                }
            }
            Some(PrimaryBeamState::Hook) | None => {
                if open.remove(other).is_some_and(|(start, touches)| touches && start <= at && at <= note.onset) {
                    return true;
                }
            }
        }
    }
    open.values().any(|&(start, lower)| lower && start <= at)
}

pub(super) fn observe(xml: &str) -> BTreeSet<usize> {
    let mut allowed = BTreeSet::new();
    let Ok(doc) = roxmltree::Document::parse(xml) else { return allowed; };
    // The original policy supplies shifted pitch and resolved structural origin.
    // It takes an empty set and cannot recurse into this observation.
    let mut original = BTreeMap::new();
    for part in doc.descendants().filter(|n| n.has_tag_name("part")) {
        let range = part.range();
        for note in super::scan_lower_pitches_with_boundary_policy(&xml[range.clone()], None, &BTreeSet::new()) {
            original.insert(range.start + note.start, (note.pitch, note.clef_transition_anchor.is_some()));
        }
    }
    for measure in doc.descendants().filter(|n| n.has_tag_name("measure")) {
        let mut notes = Vec::new();
        let mut cursor = 0i64;
        let mut head_time = 0i64;
        let mut previous_end = measure.range().start;
        let mut chord = LowerChordObservation::default();
        let mut valid_time = true;
        let mut fences = Vec::new();
        let mut surfaces = Vec::new();
        for node in measure.children().filter(|n| n.is_element()) {
            let range = node.range();
            let raw = &xml[range.clone()];
            match node.tag_name().name() {
                "backup" | "forward" => {
                    fences.push(range.start);
                    let duration = first_tag_text(raw, "duration").and_then(|v| v.parse::<i64>().ok());
                    let sign = if node.has_tag_name("backup") { -1i64 } else { 1 };
                    match duration.filter(|d| *d >= 0).and_then(|d| d.checked_mul(sign)).and_then(|d| cursor.checked_add(d)) {
                        Some(next) if next >= 0 => cursor = next,
                        _ => valid_time = false,
                    }
                }
                "attributes" => {
                    if !notes.is_empty() && first_tag_text(raw, "divisions").is_some() {
                        valid_time = false;
                    }
                    if contains_only_one_lower_clef(raw)
                        && matches!(read_lower_clef(raw), ClefSurface::Known { visible: true, .. })
                    {
                        surfaces.push((range.start, range.end));
                    } else {
                        fences.push(range.start);
                    }
                }
                "direction" => {
                    if observe_lower_staff_octave_display_shift(raw).is_some() {
                        fences.push(range.start);
                    }
                }
                "note" => {
                    let lower = first_tag_text(raw, "staff").as_deref() == Some("2");
                    let visible = !raw.contains("print-object=\"no\"") && !raw.contains("print-object='no'");
                    let pitched = super::observe_pitch(raw).is_some() && !raw.contains("<rest");
                    let relation = chord.observe(&xml[previous_end..range.start], range.start, raw, lower && visible && pitched);
                    previous_end = range.end;
                    let member = raw.contains("<chord");
                    if !member { head_time = cursor; }
                    let duration = first_tag_text(raw, "duration").and_then(|v| v.parse::<i64>().ok()).filter(|d| *d > 0);
                    let duration_value = duration.unwrap_or(0);
                    if duration.is_none() || head_time.checked_add(duration_value).is_none()
                        || first_tag_text(raw, "staff").is_none()
                    {
                        valid_time = false;
                    }
                    notes.push(Note {
                        start: range.start, end: range.end, head: relation.head(),
                        onset: head_time, duration: duration_value, raw,
                        voice: first_tag_text(raw, "voice").filter(|v| !v.is_empty()), lower,
                        supported: lower && visible && pitched && duration.is_some() && relation.head().is_some()
                            && !raw.contains("<grace"),
                        beam: primary_beam_state(raw),
                    });
                    if !member {
                        match cursor.checked_add(duration_value) {
                            Some(next) => cursor = next,
                            None => valid_time = false,
                        }
                    }
                }
                _ => {}
            }
        }
        if !valid_time { continue; }
        let heads = notes.iter().enumerate().filter_map(|(i,n)| (n.head == Some(n.start)).then_some(i)).collect::<Vec<_>>();
        for (first_head, &first_index) in heads.iter().enumerate() {
            let first = &notes[first_index];
            if first.beam != Some(PrimaryBeamState::Begin) || !first.supported || first.voice.is_none() { continue; }
            let mut group_end = None;
            let mut previous_end_time = first.onset.checked_add(first.duration);
            for &index in heads.iter().skip(first_head + 1) {
                let note = &notes[index];
                if !note.supported || note.voice != first.voice || previous_end_time != Some(note.onset) { break; }
                match note.beam {
                    Some(PrimaryBeamState::Continue) => previous_end_time = note.onset.checked_add(note.duration),
                    Some(PrimaryBeamState::End) => { group_end = Some(index); break; }
                    _ => break,
                }
            }
            let Some(end_index) = group_end else { continue; };
            let last = &notes[end_index];
            let full_end = notes.iter().skip(end_index + 1).take_while(|n| n.head == Some(last.start)).last().map_or(last.end, |n| n.end);
            if fences.iter().any(|p| first.start < *p && *p < full_end) { continue; }
            if notes.iter().filter(|n| first.start <= n.start && n.start < full_end).any(|n| {
                !n.supported || n.voice != first.voice
                    || (n.head != Some(n.start) && n.beam.is_some())
                    || n.head.is_some_and(|h| h < first.start)
                    || !original.get(&n.start).is_some_and(|(_, resolved)| *resolved)
            }) { continue; }
            let mut preference = None;
            let mut mixed = false;
            for &index in heads.iter().skip(first_head).take_while(|&&i| i <= end_index) {
                let mut bass = 0;
                let mut treble = 0;
                for member in notes.iter().filter(|n| n.head == Some(notes[index].start)) {
                    let pitch = original[&member.start].0;
                    bass += ClefKind::Bass.ledger_pressure(pitch);
                    treble += ClefKind::Treble.ledger_pressure(pitch);
                }
                let here = if bass < treble { Some(ClefKind::Bass) }
                    else if treble < bass { Some(ClefKind::Treble) } else { None };
                if let Some(here) = here {
                    mixed |= preference.is_some_and(|previous| previous != here);
                    preference = Some(here);
                }
            }
            if !mixed { continue; }
            for &index in heads.iter().skip(first_head + 1).take_while(|&&i| i <= end_index) {
                let note = &notes[index];
                let members = notes.iter().filter(|n| n.head == Some(note.start));
                if members.clone().any(|n| n.raw.contains("<tie")) { continue; }
                // Other voices and overhanging chord tones cannot silently share
                // a changed staff reference. This is time evidence, not identity.
                if notes.iter().any(|n| n.lower && n.head != Some(note.start)
                    && (n.onset == note.onset || (n.onset < note.onset && n.onset.checked_add(n.duration).is_none_or(|end| end > note.onset))))
                { continue; }
                if other_beam_conflict(&notes, first.start, full_end, note.onset) { continue; }
                allowed.insert(note.start);
            }
        }
        for (start,end) in surfaces {
            if let Some(note) = notes.iter().find(|n| n.start >= end)
                    && allowed.contains(&note.start)
                    && super::comments_removed(&xml[end..note.start]).is_some_and(|gap| gap.trim().is_empty())
            {
                allowed.insert(start);
            }
        }
    }
    allowed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pitched(step: &str, octave: usize, beam: &str, extra: &str) -> String {
        let beam = if beam.is_empty() { String::new() } else { format!("<beam number=\"1\">{beam}</beam>") };
        format!("<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>24</duration><voice>lane</voice><staff>2</staff>{beam}{extra}</note>")
    }

    fn clef(sign: &str, line: usize) -> String {
        format!("<attributes><clef number=\"2\"><sign>{sign}</sign><line>{line}</line></clef></attributes>")
    }

    fn score(base: usize, body: &str) -> String {
        format!("<score-partwise><part id=\"P\"><measure number=\"{base}\">{}{body}</measure></part></score-partwise>",clef("G",2))
    }

    fn group() -> String {
        format!("{}{}{}",pitched("B",5,"begin",""),pitched("C",2,"continue",""),pitched("E",3,"end",""))
    }

    fn middle(xml: &str) -> usize {
        let at=xml.find("<beam number=\"1\">continue</beam>").unwrap();
        xml[..at].rfind("<note>").unwrap()
    }

    fn course(base: usize, reverse: bool) -> String {
        let (first,last,pre,tail)=if reverse { (clef("F",4),clef("G",2),pitched("C",2,"",""),pitched("B",5,"","")) }
            else { (clef("G",2),clef("F",4),pitched("B",5,"",""),pitched("C",2,"","")) };
        let (high,low)=if reverse { (("C",2),("B",5)) } else { (("B",5),("C",2)) };
        let opening=pitched(high.0,high.1,"","<tie type=\"start\"/>");
        let begin=pitched(high.0,high.1,"begin","<tie type=\"stop\"/>");
        let inside=pitched(low.0,low.1,"continue","");
        let end=pitched(low.0,low.1,"end","");
        format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{base}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef></attributes>{first}{pre}{pre}{opening}{begin}{inside}{end}</measure><measure number=\"{}\">{last}{tail}{tail}{tail}</measure></part></score-partwise>",base+1)
    }

    fn apply(xml: &str) -> String {
        let mut out=xml.to_owned();
        let (result,receipt)=super::super::apply_rustig_clef_course_to_buffer(&mut out);
        result.unwrap();
        if out!=xml { receipt.as_ref().unwrap().binds_recorded_representation(xml,&out).unwrap(); }
        let mut receipts=crate::master_state_identity::RenderTransitionReceipts::new();
        receipts.push_preserving(receipt,&out);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert_eq!(super::super::raw_note_fragments(xml),super::super::raw_note_fragments(&out));
        out
    }

    #[test]
    fn closed_fresh_attack_and_pure_surface_share_one_boundary() {
        let xml=score(3,&group());let at=middle(&xml);
        assert!(observe(&xml).contains(&at));
        let surface=clef("F",4);let post=format!("{}{surface}{}",&xml[..at],&xml[at..]);
        assert!(observe(&post).contains(&at));
        assert!(observe(&post).contains(&(at+surface.len())));
        let member=pitched("G",3,"","").replace("<note>","<note><chord/>");
        let group=group().replacen(&pitched("C",2,"continue",""),&format!("{}{member}",pitched("C",2,"continue","")),1);
        let xml=score(3,&group);let at=middle(&xml);assert!(observe(&xml).contains(&at));
        let member_at=xml.find("<note><chord/>").unwrap();assert!(!observe(&xml).contains(&member_at));
    }

    #[test]
    fn unresolved_hidden_cross_staff_and_cursor_scopes_grant_no_boundary() {
        let raw=group();
        for body in [raw.replace("<beam number=\"1\">begin</beam>",""),raw.replace("<beam number=\"1\">end</beam>",""),
            raw.replacen("<voice>lane</voice>","<voice>foreign</voice>",1),raw.replacen("<staff>2</staff>","<staff>1</staff>",1),
            raw.replacen("<note>","<note print-object=\"no\">",1),
            raw.replacen("</note>","</note><backup><duration>24</duration></backup>",1),
            raw.replacen("</note>","</note><forward><duration>24</duration></forward>",1),
            raw.replacen("</note>","</note><direction><direction-type><octave-shift type=\"stop\" size=\"8\"/></direction-type><staff>2</staff></direction>",1)] {
            assert!(observe(&score(3,&body)).is_empty());
        }
        assert!(observe("<broken>").is_empty());
    }

    #[test]
    fn incoming_tie_on_any_chord_pitch_and_overlapping_voice_block_boundary() {
        let tied=group().replace(&pitched("C",2,"continue",""),&pitched("C",2,"continue","<tie type=\"stop\"/>"));
        let xml=score(3,&tied);assert!(!observe(&xml).contains(&middle(&xml)));
        let member=pitched("G",3,"","<tie type=\"stop\"/>").replace("<note>","<note><chord/>");
        let raw=group().replace(&pitched("C",2,"continue",""),&format!("{}{member}",pitched("C",2,"continue","")));
        let xml=score(3,&raw);assert!(!observe(&xml).contains(&middle(&xml)));
        let overlap=pitched("A",4,"","").replace("<duration>24</duration>","<duration>96</duration>").replace("<voice>lane</voice>","<voice>other</voice>");
        let xml=score(3,&format!("{overlap}<backup><duration>96</duration></backup>{}",group()));
        assert!(!observe(&xml).contains(&middle(&xml)));
        let outgoing=group().replace(&pitched("C",2,"continue",""),&pitched("C",2,"continue","<tie type=\"start\"/>"));
        let xml=score(3,&outgoing);assert!(!observe(&xml).contains(&middle(&xml)));
    }

    #[test]
    fn foreign_beam_span_blocks_even_when_no_foreign_note_sounds_at_the_boundary() {
        let start=pitched("B",4,"begin","").replace("lane","other");
        let end=pitched("C",4,"end","").replace("lane","other");
        let xml=score(3,&format!("{start}<forward><duration>24</duration></forward>{end}<backup><duration>72</duration></backup>{}",group()));
        assert!(!observe(&xml).contains(&middle(&xml)));
    }

    #[test]
    fn same_preference_and_unmatched_original_tie_grant_no_new_anchor() {
        let ordinary=group().replace("<step>C</step><octave>2</octave>","<step>A</step><octave>5</octave>").replace("<step>E</step><octave>3</octave>","<step>G</step><octave>5</octave>");
        assert!(observe(&score(3,&ordinary)).is_empty());
        let unmatched=group().replacen("</note>","<tie type=\"stop\"/></note>",1);
        assert!(observe(&score(3,&unmatched)).is_empty());
    }

    #[test]
    fn actual_materializer_preserves_notes_receipts_and_reentry_in_both_directions() {
        for reverse in [false,true] {
            for base in [2,91] {
                let xml=course(base,reverse);let post=apply(&xml);
                assert_ne!(post,xml);
                assert_eq!(apply(&post),post);
                assert!(!super::super::lower_clef_surface_inside_open_tie(&post));
                assert!(!super::super::lower_clef_surface_inside_primary_beam_group(&post));
                let old=super::super::scan_lower_pitches_with_boundary_policy(&xml,None,&BTreeSet::new());
                let surfaces=super::super::scan_lower_clef_surfaces(&xml).unwrap();
                let baseline=super::super::solve_readability_course_with_boundaries(&xml,&surfaces,&old,&BTreeSet::new()).unwrap();
                let pitches=super::super::scan_lower_pitches(&xml,None);
                let selected=super::super::solve_readability_course(&xml,&surfaces,&pitches).unwrap();
                assert!(selected.cost.total()<baseline.cost.total());
            }
        }
    }

    #[test]
    fn part_duplication_relocation_and_voice_renaming_preserve_local_materialization() {
        let xml=course(7,false);let post=apply(&xml);
        let part=&xml[super::super::score_part_ranges(&xml)[0].0..super::super::score_part_ranges(&xml)[0].1];
        let expected=&post[super::super::score_part_ranges(&post)[0].0..super::super::score_part_ranges(&post)[0].1];
        let renamed=part.replace("id=\"P\"","id=\"Q\"").replace("lane","renamed");
        let combined=format!("<score-partwise>{part}{renamed}</score-partwise>");let out=apply(&combined);let ranges=super::super::score_part_ranges(&out);
        assert_eq!(&out[ranges[0].0..ranges[0].1],expected);
        assert_eq!(&out[ranges[1].0..ranges[1].1],expected.replace("id=\"P\"","id=\"Q\"").replace("lane","renamed"));
    }

    #[test]
    fn actual_checkpoint_receipt_adoption_and_committed_reentry() {
        const CHILD: &str = "ETUS_TEST_FRESH_CLEF_BOUNDARY_CHECKPOINT";
        if std::env::var_os(CHILD).is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "structural_write_authority::rustig_clef_planning::beam_boundary::tests::actual_checkpoint_receipt_adoption_and_committed_reentry", "--nocapture", "--test-threads=1"])
                .env(CHILD,"1").output().unwrap();
            assert!(output.status.success(),"{}{}",String::from_utf8_lossy(&output.stdout),String::from_utf8_lossy(&output.stderr));
            assert!(String::from_utf8_lossy(&output.stdout).contains("test result: ok. 1 passed;"));
            return;
        }
        use crate::master_state_identity::{MasterLineage,RenderTransitionReceipts};
        use crate::structural_write_authority::{MasterAdoptedState,RenderSovereignState,RenderSinkKind};
        for reverse in [false,true] {
            for base in [2,91] {
                crate::master_state_identity::reset_render_lineage_measurement();
                let src=course(base,reverse);let expected=apply(&src);
                let master=MasterAdoptedState::adopt_genesis_state(src.clone(),MasterLineage::lineage_from_genesis(
                    crate::master_state_identity::tests::synthetic_genesis_for_tests(&src)));
                let path=std::env::temp_dir().join(format!("etuschlopf-fresh-clef-{}-{base}-{reverse}.render.musicxml",std::process::id()));
                let mut state=RenderSovereignState::begin_project_render(&path,&master);
                let root=state.initial_trial_root(true);
                state.adopt_transition(root,&src,RenderTransitionReceipts::new(),RenderSinkKind::PrunedPdfTrial,
                    |p,_,post|(std::fs::write(p,post),None)).unwrap();
                let root=state.require_state_root("fresh boundary checkpoint");
                state.adopt_final_notation_checkpoint(root,&src,RenderTransitionReceipts::new()).unwrap();
                let post=std::fs::read_to_string(&path).unwrap();
                assert_eq!(post,expected);
                assert_eq!(state.document(),Some(post.as_str()));
                assert_eq!(super::super::raw_note_fragments(&src),super::super::raw_note_fragments(&post));
                let root=state.require_state_root("fresh boundary reentry");
                state.adopt_final_notation_checkpoint(root,&post,RenderTransitionReceipts::new()).unwrap();
                assert_eq!(std::fs::read_to_string(&path).unwrap(),post);
                assert_eq!(state.document(),Some(post.as_str()));
                std::fs::remove_file(path).unwrap();
            }
        }
        crate::master_state_identity::reset_render_lineage_measurement();
    }
}
