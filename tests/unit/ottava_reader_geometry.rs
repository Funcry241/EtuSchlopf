use super::*;
use crate::registernotation::reader_policy::{self as policy, synthetic};

fn parsed(
    display: RegisternotationsDarstellung,
    staff: u8,
) -> (
    VisibleLine,
    Vec<PdfStrokeSegment>,
    Vec<RendererStaffCell>,
    Vec<PdfTextEvent>,
) {
    policy::reset();
    let xml = synthetic::course(display, &staff.to_string(), "independent", "arbitrary");
    let pdf = synthetic_pdf(display, staff, true, true, true, "Leland", 0.0);
    let expected = observe(&xml, &pdf).supported.into_values().next().unwrap();
    let (segments, _) = extract_pdf_vector_geometry(&pdf).unwrap();
    let cells = renderer_staff_cells(&segments, 1).unwrap();
    let (texts, _) = extract_pdf_text_events(&pdf).unwrap();
    (expected, segments, cells, texts)
}
fn inspect(
    expected: &VisibleLine,
    display: RegisternotationsDarstellung,
    staff: u8,
    segments: &[PdfStrokeSegment],
    cells: &[RendererStaffCell],
    texts: &[PdfTextEvent],
) -> Option<VisibleLine> {
    current_line_observation(
        expected,
        display,
        cells.iter().find(|c| c.staff == staff).unwrap(),
        cells,
        segments,
        texts,
    )
}
fn move_pair(segments: &mut [PdfStrokeSegment], texts: &mut [PdfTextEvent], dy: f64) {
    for line in segments
        .iter_mut()
        .filter(|s| s.x1 == 22.0 && s.x2 == 110.0)
    {
        line.y1_top += dy;
        line.y2_top += dy;
    }
    for glyph in texts
        .iter_mut()
        .filter(|g| g.text.chars().any(|c| matches!(c, '\u{e510}' | '\u{e514}')))
    {
        glyph.y_from_bottom -= dy;
        if let Some(bounds) = glyph.head_bounds.as_mut() {
            bounds[1] += dy;
            bounds[3] += dy;
        }
    }
}
#[test]
fn current_geometry_rejects_separation_truncation_split_and_horizontal_drift() {
    let _guard = crate::registernotation::sperre_fuer_tests();
    let d = RegisternotationsDarstellung::Achtva;
    let (expected, segments, cells, texts) = parsed(d, 1);
    assert!(inspect(&expected, d, 1, &segments, &cells, &texts).is_some());
    let at = segments
        .iter()
        .position(|s| s.x1 == 22.0 && s.x2 == 110.0)
        .unwrap();
    let mut broken = segments.clone();
    broken[at].x2 -= 1.0;
    assert!(inspect(&expected, d, 1, &broken, &cells, &texts).is_none());
    let mut split = segments.clone();
    let mut second = split[at];
    split[at].x2 = 60.0;
    second.x1 = 61.0;
    split.push(second);
    assert!(inspect(&expected, d, 1, &split, &cells, &texts).is_none());
    let mut detached = segments.clone();
    detached[at].y1_top += 10.0;
    detached[at].y2_top += 10.0;
    assert!(inspect(&expected, d, 1, &detached, &cells, &texts).is_none());
    let mut other_cell = cells.clone();
    other_cell.iter_mut().find(|c| c.staff == 1).unwrap().x_left += 1.0;
    assert!(inspect(&expected, d, 1, &segments, &other_cell, &texts).is_none());
    let mut wrong = texts.clone();
    for g in &mut wrong {
        if g.text == "\u{e510}" {
            g.text = "\u{e514}".into();
        }
    }
    assert!(inspect(&expected, d, 1, &segments, &cells, &wrong).is_none());
}
#[test]
fn current_reading_domain_and_local_ambiguity_are_not_resolved_by_nearest_position() {
    let _guard = crate::registernotation::sperre_fuer_tests();
    let d = RegisternotationsDarstellung::Achtva;
    let (expected, segments, cells, texts) = parsed(d, 1);
    let mut far_lines = segments.clone();
    let mut far_glyphs = texts.clone();
    move_pair(&mut far_lines, &mut far_glyphs, 60.0);
    assert!(inspect(&expected, d, 1, &far_lines, &cells, &far_glyphs).is_none());
    let mut doubled_lines = segments.clone();
    let mut doubled_glyphs = texts.clone();
    let mut extra_line = *segments
        .iter()
        .find(|s| s.x1 == 22.0 && s.x2 == 110.0)
        .unwrap();
    extra_line.y1_top += 1.0;
    extra_line.y2_top += 1.0;
    doubled_lines.push(extra_line);
    let mut extra_glyph = texts.iter().find(|g| g.text == "\u{e510}").unwrap().clone();
    extra_glyph.y_from_bottom -= 1.0;
    let b = extra_glyph.head_bounds.as_mut().unwrap();
    b[1] += 1.0;
    b[3] += 1.0;
    doubled_glyphs.push(extra_glyph);
    assert!(inspect(&expected, d, 1, &doubled_lines, &cells, &doubled_glyphs).is_none());
    // A line in the other staff's domain neither wins nor vetoes this course.
    let mut independent_lines = segments.clone();
    let mut independent_texts = texts.clone();
    independent_lines.extend(
        far_lines
            .into_iter()
            .filter(|s| s.x1 == 22.0 && s.x2 == 110.0),
    );
    independent_texts.extend(far_glyphs.into_iter().filter(|g| g.text == "\u{e510}"));
    assert!(
        inspect(
            &expected,
            d,
            1,
            &independent_lines,
            &cells,
            &independent_texts
        )
        .is_some()
    );
}
