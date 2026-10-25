//! Pure observation over the already loaded current renderer PDF and XML.
//! Origins/decisions remain in RegisterNotation; no files or music are written.
use super::renderer_system_break_readback::{
    PdfStrokeSegment, PdfTextEvent, RendererStaffCell, extract_pdf_text_events,
    extract_pdf_vector_geometry, renderer_staff_cells,
};
use crate::registernotation::{
    RegisternotationsDarstellung,
    reader_policy::{CueRequest, PdfContext, VisibleLine},
};
use std::collections::BTreeMap;

pub(crate) fn observe(xml: &str, bytes: &[u8]) -> PdfContext {
    let mut result = PdfContext {
        xml_sha256: crate::sha256::hex(xml.as_bytes()),
        pdf_sha256: crate::sha256::hex(bytes),
        supported: BTreeMap::new(),
    };
    let requests = crate::registernotation::reader_policy::pdf_requests(xml);
    if requests.is_empty() {
        return result;
    }
    let observation = (|| -> anyhow::Result<_> {
        let document = roxmltree::Document::parse(xml)?;
        let parts = document
            .root_element()
            .children()
            .filter(|n| n.has_tag_name("part"))
            .collect::<Vec<_>>();
        let [part] = parts.as_slice() else {
            anyhow::bail!("Unsupported multi-part ottava PDF binding");
        };
        let count = part
            .children()
            .filter(|n| n.has_tag_name("measure"))
            .count();
        let (segments, _) = extract_pdf_vector_geometry(bytes)?;
        let cells = renderer_staff_cells(&segments, count)?;
        let (texts, _) = extract_pdf_text_events(bytes)?;
        Ok((segments, cells, texts))
    })();
    let Ok((segments, cells, texts)) = observation else {
        return result;
    };
    let mut groups = BTreeMap::new();
    for request in requests {
        groups
            .entry((request.slot, request.staff, request.display))
            .or_insert_with(Vec::new)
            .push(request);
    }
    for ((slot, staff, display), mut requests) in groups {
        let Some(cell) = cells.iter().find(|c| c.measure == slot && c.staff == staff) else {
            continue;
        };
        requests.sort_by_key(|r| r.ordinal);
        // A post-removal check consumes the transported origin and horizontal
        // attachment. It observes vertical engraving afresh in the current cell;
        // it never creates an origin by searching for a nearby replacement.
        if requests.iter().all(|r| {
            crate::registernotation::reader_policy::expected_visible_line(xml, &r.id).is_some()
        }) {
            for request in requests {
                let expected =
                    crate::registernotation::reader_policy::expected_visible_line(xml, &request.id)
                        .unwrap();
                if let Some(current) = current_line_observation(
                    &expected,
                    request.display,
                    cell,
                    &cells,
                    &segments,
                    &texts,
                ) {
                    result.supported.insert(request.id, current);
                }
            }
            continue;
        }
        let mut words = texts
            .iter()
            .filter(|text| {
                text.page_index == cell.page_index
                    && text.text == display.bezeichnung()
                    && text.x >= cell.x_left
                    && text.x <= cell.x_right
                    && on_reading_side(text.page_height - text.y_from_bottom, cell, display, &cells)
            })
            .collect::<Vec<_>>();
        words.sort_by(|a, b| a.x.total_cmp(&b.x));
        // Cardinality is confined to this one physical reading cohort. Other
        // independent measures/parts never decide its eligibility.
        if words.len() != requests.len() || words.windows(2).any(|w| w[0].x == w[1].x) {
            continue;
        }
        for (request, word) in requests.into_iter().zip(words) {
            if let Some(line) = line_for_word(&request, word, cell, &segments, &texts) {
                result.supported.insert(request.id, line);
            }
        }
    }
    result
}
fn on_reading_side(
    y: f64,
    cell: &RendererStaffCell,
    display: RegisternotationsDarstellung,
    cells: &[RendererStaffCell],
) -> bool {
    let above = matches!(
        display,
        RegisternotationsDarstellung::Achtva | RegisternotationsDarstellung::Fuenfzehnma
    );
    let first = cell.line_y_top[0];
    let last = *cell.line_y_top.last().unwrap();
    if above {
        y < first
            && !cells.iter().any(|other| {
                other.page_index == cell.page_index
                    && other.x_left <= cell.x_right
                    && other.x_right >= cell.x_left
                    && other.line_y_top[0] < first
                    && other.line_y_top.last().is_some_and(|bottom| *bottom >= y)
            })
    } else {
        y > last
            && !cells.iter().any(|other| {
                other.page_index == cell.page_index
                    && other.x_left <= cell.x_right
                    && other.x_right >= cell.x_left
                    && other.line_y_top[0] > last
                    && other.line_y_top[0] <= y
            })
    }
}
fn horizontal(segment: &PdfStrokeSegment) -> bool {
    segment.x1 != segment.x2
        && segment.y1_top == segment.y2_top
        && segment.width.is_finite()
        && segment.width > 0.0
}
fn signature(
    segment: &PdfStrokeSegment,
    bounds: [f64; 4],
    cell: &RendererStaffCell,
) -> VisibleLine {
    VisibleLine {
        page: segment.page_index,
        stroke: [
            segment.x1.min(segment.x2).to_bits(),
            segment.y1_top.to_bits(),
            segment.x1.max(segment.x2).to_bits(),
            segment.width.to_bits(),
        ],
        orientation: bounds.map(f64::to_bits),
        cell_x: [cell.x_left.to_bits(), cell.x_right.to_bits()],
    }
}
fn glyph_matches(text: &PdfTextEvent, display: RegisternotationsDarstellung) -> bool {
    let size = display.groesse();
    match text.text.as_str() {
        "\u{e510}" => size == 8,
        "\u{e514}" => size == 15,
        "\u{e511}" => display == RegisternotationsDarstellung::Achtva,
        "\u{e512}" | "\u{e513}" | "\u{e51c}" => display == RegisternotationsDarstellung::Achtvb,
        "\u{e515}" => display == RegisternotationsDarstellung::Fuenfzehnma,
        "\u{e516}" | "\u{e51d}" => display == RegisternotationsDarstellung::Fuenfzehnmb,
        _ => false,
    }
}
fn line_for_word(
    request: &CueRequest,
    word: &PdfTextEvent,
    cell: &RendererStaffCell,
    segments: &[PdfStrokeSegment],
    texts: &[PdfTextEvent],
) -> Option<VisibleLine> {
    let space = cell.line_y_top.windows(2).next().map(|w| w[1] - w[0])?;
    let baseline = word.page_height - word.y_from_bottom;
    let mut matches = Vec::new();
    for segment in segments.iter().filter(|s| {
        horizontal(s)
            && s.page_index == word.page_index
            && s.x1.min(s.x2) <= word.x
            && s.x1.max(s.x2) > word.x
            && s.y1_top >= baseline
            && s.y1_top - baseline <= space
    }) {
        for glyph in texts
            .iter()
            .filter(|g| g.page_index == segment.page_index && glyph_matches(g, request.display))
        {
            let Some(bounds) = glyph.head_bounds else {
                continue;
            };
            let left = segment.x1.min(segment.x2);
            // The actual embedded glyph box must adjoin this same line and
            // overlap its ordinate; no duration, pitch or witness cutoff.
            if bounds[2] <= left
                && left - bounds[2] <= bounds[2] - bounds[0]
                && bounds[1] <= segment.y1_top
                && bounds[3] >= segment.y1_top
            {
                matches.push(signature(segment, bounds, cell));
            }
        }
    }
    matches.sort_by_key(|m| (m.page, m.stroke, m.orientation));
    matches.dedup();
    let [found] = matches.as_slice() else {
        return None;
    };
    Some(found.clone())
}
fn current_line_observation(
    expected: &VisibleLine,
    display: RegisternotationsDarstellung,
    cell: &RendererStaffCell,
    cells: &[RendererStaffCell],
    segments: &[PdfStrokeSegment],
    texts: &[PdfTextEvent],
) -> Option<VisibleLine> {
    if cell.page_index != expected.page
        || [cell.x_left.to_bits(), cell.x_right.to_bits()] != expected.cell_x
    {
        return None;
    }
    let mut found = Vec::new();
    for segment in segments.iter().filter(|s| {
        horizontal(s)
            && s.page_index == expected.page
            && on_reading_side(s.y1_top, cell, display, cells)
    }) {
        for glyph in texts
            .iter()
            .filter(|g| g.page_index == segment.page_index && glyph_matches(g, display))
        {
            let Some(bounds) = glyph.head_bounds else {
                continue;
            };
            let left = segment.x1.min(segment.x2);
            if !bounds.iter().all(|v| v.is_finite())
                || bounds[1] < 0.0
                || bounds[3] > glyph.page_height
            {
                continue;
            }
            if bounds[2] <= left
                && left - bounds[2] <= bounds[2] - bounds[0]
                && bounds[1] <= segment.y1_top
                && bounds[3] >= segment.y1_top
            {
                let current = signature(segment, bounds, cell);
                if current.same_horizontal_course(expected) {
                    found.push(current);
                }
            }
        }
    }
    found.sort_by_key(|line| (line.page, line.stroke, line.orientation, line.cell_x));
    found.dedup();
    let [current] = found.as_slice() else {
        return None;
    };
    Some(current.clone())
}

#[cfg(test)]
pub(crate) fn synthetic_pdf(
    display: RegisternotationsDarstellung,
    staff: u8,
    word: bool,
    line: bool,
    orientation: bool,
    font_name: &str,
    shift: f64,
) -> Vec<u8> {
    let above = matches!(
        display,
        RegisternotationsDarstellung::Achtva | RegisternotationsDarstellung::Fuenfzehnma
    );
    let y = match (staff, above) {
        (1, true) => 50.0,
        (1, false) => 95.0,
        (2, true) => 100.0,
        _ => 150.0,
    } + shift;
    let mut font = vec![0u8; 152];
    font[..4].copy_from_slice(&[0, 1, 0, 0]);
    font[4..6].copy_from_slice(&4u16.to_be_bytes());
    for (i, (name, start, length)) in [
        (b"head", 76u32, 54u32),
        (b"loca", 130, 6),
        (b"glyf", 136, 10),
        (b"maxp", 146, 6),
    ]
    .iter()
    .enumerate()
    {
        let at = 12 + 16 * i;
        font[at..at + 4].copy_from_slice(*name);
        font[at + 8..at + 12].copy_from_slice(&start.to_be_bytes());
        font[at + 12..at + 16].copy_from_slice(&length.to_be_bytes());
    }
    font[94..96].copy_from_slice(&1000u16.to_be_bytes());
    font[134..136].copy_from_slice(&5u16.to_be_bytes());
    font[136..138].copy_from_slice(&1u16.to_be_bytes());
    for (at, value) in [(138, 0i16), (140, -250), (142, 500), (144, 250)] {
        font[at..at + 2].copy_from_slice(&value.to_be_bytes());
    }
    font[150..152].copy_from_slice(&2u16.to_be_bytes());
    let mut content = String::new();
    for top in [70.0, 110.0] {
        for i in 0..5 {
            let at = 200.0 - top - f64::from(i) * 5.0 - shift;
            content.push_str(&format!("0.5 w 10 {at} m 120 {at} l S\n"));
        }
    }
    if line {
        content.push_str(&format!("0.5 w 22 {} m 110 {} l S\n", 200.0 - y, 200.0 - y));
    }
    if orientation {
        content.push_str(&format!(
            "BT /F1 10 Tf 1 0 0 1 15 {} Tm <0001> Tj ET\n",
            200.0 - y
        ));
    }
    if word {
        let code = display
            .bezeichnung()
            .chars()
            .map(|ch| match ch {
                '8' => "0001",
                'v' => "0002",
                'a' => "0003",
                '1' => "0004",
                '5' => "0005",
                'm' => "0006",
                _ => "0007",
            })
            .collect::<String>();
        content.push_str(&format!(
            "BT /F2 10 Tf 1 0 0 1 65 {} Tm <{code}> Tj ET\n",
            200.0 - y + 3.0
        ));
    }
    let stream = |bytes: &[u8]| {
        let mut out = format!("<< /Length {} >>\nstream\n", bytes.len()).into_bytes();
        out.extend_from_slice(bytes);
        out.extend_from_slice(b"\nendstream");
        out
    };
    let glyph = if display.groesse() == 8 {
        "E510"
    } else {
        "E514"
    };
    let cmap = format!("1 beginbfchar\n<0001> <{glyph}>\nendbfchar\n");
    let text_cmap = "7 beginbfchar\n<0001> <0038>\n<0002> <0076>\n<0003> <0061>\n<0004> <0031>\n<0005> <0035>\n<0006> <006D>\n<0007> <0062>\nendbfchar\n";
    let objects=vec![b"<< /Type /Pages /Kids [2 0 R] /Count 1 >>".to_vec(),b"<< /Type /Page /Contents 3 0 R /Resources 4 0 R /MediaBox [0 0 140 200] >>".to_vec(),stream(content.as_bytes()),b"<< /Font << /F1 5 0 R /F2 10 0 R >> >>".to_vec(),format!("<< /Type /Font /Subtype /Type0 /Encoding /Identity-H /BaseFont /{font_name} /DescendantFonts [6 0 R] /ToUnicode 7 0 R >>").into_bytes(),b"<< /Subtype /CIDFontType2 /CIDToGIDMap /Identity /FontDescriptor 8 0 R >>".to_vec(),stream(cmap.as_bytes()),b"<< /FontFile2 9 0 R >>".to_vec(),stream(&font),b"<< /Type /Font /Subtype /Type0 /Encoding /Identity-H /BaseFont /EdwinItalic /ToUnicode 11 0 R >>".to_vec(),stream(text_cmap.as_bytes())];
    let mut out = b"%PDF-1.4\n".to_vec();
    for (i, body) in objects.iter().enumerate() {
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::registernotation::reader_policy::{self as policy, synthetic};
    #[test]
    fn real_pdf_parser_requires_orientation_glyph_and_continuous_line_for_every_display() {
        let _guard = crate::registernotation::sperre_fuer_tests();
        for display in [
            RegisternotationsDarstellung::Achtva,
            RegisternotationsDarstellung::Achtvb,
            RegisternotationsDarstellung::Fuenfzehnma,
            RegisternotationsDarstellung::Fuenfzehnmb,
        ] {
            for staff in [1u8, 2] {
                policy::reset();
                let xml = synthetic::course(display, &staff.to_string(), "renamed", "free-label");
                assert_eq!(
                    observe(
                        &xml,
                        &synthetic_pdf(display, staff, true, true, true, "Leland", 0.0)
                    )
                    .supported
                    .len(),
                    1
                );
                for (line, glyph, font) in [
                    (false, true, "Leland"),
                    (true, false, "Leland"),
                    (true, true, "Unsupported"),
                ] {
                    assert!(
                        observe(
                            &xml,
                            &synthetic_pdf(display, staff, true, line, glyph, font, 0.0)
                        )
                        .supported
                        .is_empty()
                    );
                }
            }
        }
    }
    #[test]
    fn missing_pdf_and_absent_word_do_not_create_a_baseline_attachment() {
        let _guard = crate::registernotation::sperre_fuer_tests();
        policy::reset();
        let d = RegisternotationsDarstellung::Achtva;
        let xml = synthetic::course(d, "1", "p", "x");
        assert!(observe(&xml, b"not a PDF").supported.is_empty());
        assert!(
            observe(&xml, &synthetic_pdf(d, 1, false, true, true, "Leland", 0.0))
                .supported
                .is_empty()
        );
    }
}

#[cfg(test)]
#[path = "../../tests/unit/ottava_reader_geometry.rs"]
mod geometry_tests;
