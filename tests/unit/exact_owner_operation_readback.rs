use super::*;
use crate::structural_write_authority as owner;
use std::ops::Range;

fn note(voice: &str, staff: Option<&str>) -> String {
    format!("<note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>{voice}</voice>{}</note>",
        staff.map_or_else(String::new, |s| format!("<staff>{s}</staff>")))
}

fn source(first: &str, second: &str, label: &str) -> String {
    format!("<score-partwise><part id=\"{label}\"><measure number=\"{label}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{first}{second}</measure></part></score-partwise>")
}

fn span(source: &str, text: &str) -> Range<usize> {
    let start = source.find(text).unwrap(); start..start + text.len()
}

fn value(source: &str, range: Range<usize>, tag: &str) -> (usize, usize) {
    let open = format!("<{tag}>"); let close = format!("</{tag}>");
    let start = range.start + source[range.clone()].find(&open).unwrap() + open.len();
    (start, start + source[start..range.end].find(&close).unwrap())
}

#[test]
fn actual_scalar_owner_carries_inside_edits_insertions_and_independent_duplicates() {
    for (label, voice, staff) in [("Renamed", "1", Some("1")), ("Örtlich", "901", None)] {
        let first = note(voice, staff); let second = note("22", Some("2"));
        let pre = source(&first, &second, label); let first_range = span(&pre, &first);
        let second_range = span(&pre, &second);
        let staff_range = staff.map(|_| value(&pre, first_range.clone(), "staff"));
        let (post, receipt) = owner::apply_bound_staff_voice_move_authorized(&pre,
            Some(value(&pre, first_range.clone(), "voice")), staff_range, first_range.end,
            "7001".to_owned(), "2".to_owned());
        let post = post.unwrap(); let receipt = receipt.unwrap();
        let readback = receipt.operation_readback(&pre).unwrap();
        assert_eq!(readback.post_document(), post);
        let carried = readback.carry_range(first_range).unwrap();
        assert!(post[carried].contains("<voice>7001</voice>"));
        assert_eq!(&post[readback.carry_range(second_range).unwrap()], second);
        assert_eq!(receipt.realize_post_document(&pre).as_deref(), Some(post.as_str()));
        assert!(receipt.operation_readback(&format!("{pre} ")).is_err());
    }
    let n = note("1", Some("1")); let pre = source(&n, &n, "Duplicate");
    let a = span(&pre, &n); let b = a.end..a.end + n.len();
    let (post, receipt) = owner::apply_atomic_staff_voice_scalar_batch_authorized(&pre,
        vec![(value(&pre, a.clone(), "staff").0, value(&pre, a.clone(), "staff").1, "2".into())]);
    let receipt = receipt.unwrap(); let readback = receipt.operation_readback(&pre).unwrap();
    assert_ne!(readback.carry_range(a.clone()).unwrap(), readback.carry_range(b.clone()).unwrap());
    let post = post.unwrap();
    assert!(post[readback.carry_range(a).unwrap()].contains("<staff>2</staff>"));
    assert_eq!(&post[readback.carry_range(b).unwrap()], n);
}

#[test]
fn actual_restification_replaces_one_bound_block_and_preserves_the_next() {
    let a = note("1", Some("1")); let b = note("2", Some("2"));
    let pre = source(&a, &b, "Restification"); let range = span(&pre, &a);
    let rest = "<note><rest/><duration>96</duration><voice>1</voice><staff>1</staff></note>";
    let (post, receipt) = owner::apply_atomic_restification_batch_authorized(&pre,
        vec![(range.start, range.end, rest.to_owned())]);
    let receipt = receipt.unwrap(); let readback = receipt.operation_readback(&pre).unwrap();
    let post = post.unwrap();
    assert_eq!(&post[readback.carry_range(range.clone()).unwrap()], rest);
    assert_eq!(&post[readback.carry_range(span(&pre, &b)).unwrap()], b);
    assert!(readback.carry_range(range.start + 1..range.end).is_err());
    assert!(readback.carry_range(0..pre.len() + 1).is_err());
    assert!(readback.carry_range(0..0).is_err());
}

#[test]
fn actual_transfer_carries_old_ranges_without_inventing_an_origin_for_the_inserted_note() {
    for source_first in [true, false] {
        let a = note("1", Some("1")); let b = note("2", Some("2"));
        let pre = if source_first { source(&a, &b, "After") } else { source(&b, &a, "Before") };
        let range = span(&pre, &a); let anchor = span(&pre, &b);
        let rest = "<note><rest/><duration>96</duration><voice>1</voice><staff>1</staff></note>";
        let inserted = "<note><chord/><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>2</voice><staff>2</staff></note>";
        let (post, receipt) = owner::apply_bound_safe_transfer_authorized(&pre, range.start,
            range.end, anchor.end, rest.into(), inserted.into());
        let receipt = receipt.unwrap(); let post = post.unwrap();
        let readback = receipt.operation_readback(&pre).unwrap();
        assert_eq!(post.matches("<note>").count(), 3);
        assert_eq!(&post[readback.carry_range(range).unwrap()], rest);
        assert_eq!(&post[readback.carry_range(anchor).unwrap()], b);
        // Only the actual creator may later bind a new occurrence. The reader
        // exposes no source/fragment identity, role, course or birth capability.
        assert_eq!(readback.post_document(), post);
    }
}

#[test]
fn actual_receipt_readback_does_not_replace_adoption_authority() {
    let a = note("1", Some("1")); let b = note("2", Some("2"));
    let pre = source(&a, &b, "MissingReceipt"); let range = span(&pre, &a);
    let (post, receipt) = owner::apply_bound_staff_voice_move_authorized(&pre,
        Some(value(&pre, range.clone(), "voice")), Some(value(&pre, range.clone(), "staff")),
        range.end, "2".into(), "2".into());
    let receipt = receipt.unwrap(); let post = post.unwrap();
    receipt.operation_readback(&pre).unwrap().carry_range(range).unwrap();
    let genesis = super::tests::synthetic_genesis_for_tests(&pre);
    let mut state = owner::MasterAdoptedState::adopt_genesis_state(pre.clone(),
        MasterLineage::lineage_from_genesis(genesis));
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        state.adopt_materialized_document(post, OwnerReceipts::default(), "SyntheticMissingExactReceipt");
    }));
    assert!(refusal.is_err()); assert_eq!(state.document(), pre);
}

#[test]
fn altered_actual_owner_script_and_its_foreign_post_state_are_refused() {
    let a = note("1", Some("1")); let pre = source(&a, "", "Altered");
    let range = span(&pre, &a); let staff = value(&pre, range, "staff");
    let (_, receipt) = owner::apply_atomic_staff_voice_scalar_batch_authorized(&pre,
        vec![(staff.0, staff.1, "2".into())]);
    let mut receipt = receipt.unwrap();
    assert!(receipt.operation_readback(&pre).is_ok());
    receipt.edits[0].replacement = "1".into();
    assert!(receipt.operation_readback(&pre).is_err());
    assert!(receipt.realize_post_document(&pre).is_none());
}

#[test]
fn actual_structural_deletion_requires_separate_accounting_for_its_removed_range() {
    let n = note("1", Some("1")); let print = "<notations></notations>";
    let pre = source(print, &n, "Örtlich");
    let removed = span(&pre, print); let mut post = pre.clone();
    let (result, receipt) = owner::remove_allowed_xml_ranges_authorized(&mut post,
        &[(removed.start, removed.end)]);
    assert_eq!(result.unwrap(), 1);
    let receipt = receipt.unwrap(); let readback = receipt.operation_readback(&pre).unwrap();
    assert_eq!(readback.post_document(), post);
    assert!(readback.carry_range(removed).is_err());
    assert_eq!(&post[readback.carry_range(span(&pre, &n)).unwrap()], n);
    let utf8 = pre.find('Ö').unwrap();
    assert!(readback.carry_range(utf8 + 1..utf8 + 2).is_err());
}
