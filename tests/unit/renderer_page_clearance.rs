use super::*;

fn component(b: [f64; 4], glyph: Option<char>) -> ClearanceComponent {
    ClearanceComponent {
        bounds: ClearanceBox::checked(b).unwrap(),
        glyph,
        text: None,
        bold: false,
    }
}
#[test]
fn connected_low_beam_keeps_its_anchored_stems_in_one_system() {
    let staff = vec![
        ClearanceBox([0.0, 10.0, 100.0, 30.0]),
        ClearanceBox([0.0, 70.0, 100.0, 90.0]),
    ];
    let nodes = vec![
        component([30.0, 28.0, 34.0, 33.0], Some('\u{e0a4}')),
        component([32.0, 30.0, 33.0, 54.0], None),
        component([32.0, 53.0, 58.0, 55.0], None),
        component([30.0, 75.0, 34.0, 80.0], Some('\u{e0a4}')),
    ];
    assert_eq!(
        clearance_row_assignment(&nodes, &staff).unwrap(),
        vec![0, 0, 0, 1]
    );
}
#[test]
fn one_connected_component_with_two_staff_origins_is_unavailable() {
    let staff = vec![
        ClearanceBox([0.0, 10.0, 100.0, 30.0]),
        ClearanceBox([0.0, 70.0, 100.0, 90.0]),
    ];
    let nodes = vec![
        component([30.0, 20.0, 35.0, 25.0], Some('a')),
        component([30.0, 75.0, 35.0, 80.0], Some('b')),
        component([30.0, 22.0, 35.0, 78.0], None),
    ];
    assert!(clearance_row_assignment(&nodes, &staff).is_err());
}
#[test]
fn independent_page_duplication_and_translation_preserve_local_ownership() {
    let staff = vec![
        ClearanceBox([0.0, 10.0, 100.0, 30.0]),
        ClearanceBox([0.0, 70.0, 100.0, 90.0]),
    ];
    let nodes = vec![
        component([30.0, 20.0, 35.0, 25.0], Some('a')),
        component([30.0, 75.0, 35.0, 80.0], Some('b')),
    ];
    let before = clearance_row_assignment(&nodes, &staff).unwrap();
    let shift = |b: ClearanceBox| {
        ClearanceBox([
            b.0[0] + 400.0,
            b.0[1] + 500.0,
            b.0[2] + 400.0,
            b.0[3] + 500.0,
        ])
    };
    let shifted_staff = staff.iter().copied().map(shift).collect::<Vec<_>>();
    let shifted = nodes
        .iter()
        .map(|n| component(shift(n.bounds).0, n.glyph))
        .collect::<Vec<_>>();
    assert_eq!(
        clearance_row_assignment(&shifted, &shifted_staff).unwrap(),
        before
    );
    assert_eq!(clearance_row_assignment(&nodes, &staff).unwrap(), before);
}
#[test]
fn clipping_unknown_paint_and_incomplete_stacks_do_not_supply_geometry() {
    let fonts = BTreeMap::new();
    let bold = BTreeSet::new();
    for content in [
        "q",
        "Do",
        "1 2 3 4 re S",
        "20 20 m 200 20 l S",
        "0 0 m 20 20 l W n",
    ] {
        assert!(
            clearance_contents(content.as_bytes(), 100.0, 100.0, &fonts, &bold).is_err(),
            "{content}"
        );
    }
    assert!(clearance_contents(b"q 1 0 1 1 0 0 cm Q", 100.0, 100.0, &fonts, &bold).is_err());
}
#[test]
fn curves_and_strokes_use_conservative_painted_bounds_only() {
    let fonts = BTreeMap::new();
    let bold = BTreeSet::new();
    let boxes = clearance_contents(
        b"q 2 j 2 w 20 20 m 40 20 l S 20 40 m 10 45 50 50 40 40 c 20 40 l f Q",
        100.0,
        100.0,
        &fonts,
        &bold,
    )
    .unwrap();
    assert_eq!(boxes.len(), 2);
    assert_eq!(boxes[0].bounds.0, [19.0, 79.0, 41.0, 81.0]);
    assert_eq!(boxes[1].bounds.0, [10.0, 50.0, 50.0, 60.0]);
}
#[test]
fn glyph_bounds_advance_and_line_offsets_are_separate() {
    let mut glyphs = BTreeMap::new();
    glyphs.insert(
        1,
        ClearanceFontGlyph {
            bounds: Some([0.0, 0.0, 1.0, 1.0]),
            advance: 2.0,
            character: 'x',
        },
    );
    let fonts = BTreeMap::from([("F1".to_owned(), glyphs)]);
    let boxes = clearance_contents(
        b"BT /F1 10 Tf 1 0 0 1 0 0 Tm 20 20 Td <00010001> Tj 5 0 Td <0001> Tj ET",
        100.0,
        100.0,
        &fonts,
        &BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(
        boxes.iter().map(|b| b.bounds.0[0]).collect::<Vec<_>>(),
        vec![20.0, 40.0, 25.0]
    );
    assert!(
        clearance_contents(
            b"BT /F1 10 Tf <0002> Tj ET",
            100.0,
            100.0,
            &fonts,
            &BTreeSet::new()
        )
        .is_err()
    );
}
#[test]
fn nonfinite_bounds_and_missing_embedded_font_cannot_authorize() {
    assert!(ClearanceBox::checked([0.0, 0.0, f64::NAN, 1.0]).is_err());
    assert!(clearance_true_type(&[0; 54], &BTreeMap::from([(1, 'x')])).is_err());
}
#[test]
fn complete_synthetic_true_type_supplies_bounds_and_advance_without_defaults() {
    let mut bytes = vec![0u8; 228];
    bytes[..4].copy_from_slice(&[0, 1, 0, 0]);
    bytes[4..6].copy_from_slice(&6u16.to_be_bytes());
    for (i, (name, start, len)) in [
        (b"head", 108u32, 54u32),
        (b"loca", 162, 6),
        (b"glyf", 168, 10),
        (b"maxp", 178, 6),
        (b"hhea", 184, 36),
        (b"hmtx", 220, 8),
    ]
    .iter()
    .enumerate()
    {
        let at = 12 + 16 * i;
        bytes[at..at + 4].copy_from_slice(*name);
        bytes[at + 8..at + 12].copy_from_slice(&start.to_be_bytes());
        bytes[at + 12..at + 16].copy_from_slice(&len.to_be_bytes());
    }
    for (at, value) in [
        (126, 1000u16),
        (166, 5),
        (168, 1),
        (172, (-100i16) as u16),
        (174, 600),
        (176, 700),
        (182, 2),
        (218, 2),
        (220, 500),
        (224, 650),
    ] {
        bytes[at..at + 2].copy_from_slice(&value.to_be_bytes());
    }
    let map = BTreeMap::from([(1, 'x')]);
    let parsed = clearance_true_type(&bytes, &map).unwrap();
    assert_eq!(parsed[&1].bounds, Some([0.0, -0.1, 0.6, 0.7]));
    assert_eq!(parsed[&1].advance, 0.65);
    bytes.truncate(227);
    assert!(clearance_true_type(&bytes, &map).is_err());
}
