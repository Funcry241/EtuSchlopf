//! One-page source-marker experiment for the PDF emitted by MuseScore.
//! A successful observation carries no write permission.

use super::{
    PdfObject, clearance_number_after, clearance_page_size, page_ids_in_order, parse_pdf_objects,
    parse_single_ref_after,
};
use anyhow::{Context, Result, ensure};
use std::collections::BTreeMap;

#[path = "renderer_pdf_path_anchor_audit.rs"]
mod path_ink;

const POINT_RESERVE: f64 = 0.015;
const RGB_RESERVE: f64 = 0.000_001;
const BYTE_LIMIT: usize = 256 * 1024;

struct PageStream {
    width: f64,
    height: f64,
    content: Vec<u8>,
}

fn only_names(text: &str, allowed: &[&str]) -> bool {
    text.split('/').skip(1).all(|suffix| {
        suffix
            .split(|ch: char| ch.is_ascii_whitespace() || matches!(ch, '<' | '>' | '[' | ']'))
            .next()
            .is_some_and(|name| allowed.contains(&name))
    })
}

fn dictionary_after<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let (_, tail) = text.split_once(key)?;
    if tail.contains(key) {
        return None;
    }
    let inside = tail.trim_start().strip_prefix("<<")?.split_once(">>")?.0;
    (!inside.contains("<<")).then_some(inside)
}

fn object_after<'a>(
    objects: &'a BTreeMap<u32, PdfObject>,
    dictionary: &str,
    key: &str,
) -> Option<&'a PdfObject> {
    let reference = parse_single_ref_after(dictionary, key)?;
    objects.get(&reference)
}

fn page_stream(bytes: &[u8]) -> Result<PageStream> {
    ensure!(
        bytes.len() <= BYTE_LIMIT && bytes.starts_with(b"%PDF-1.4"),
        "outside the synthetic PDF profile"
    );
    let objects = parse_pdf_objects(bytes)?;
    let pages = page_ids_in_order(&objects)?;
    let [page_id] = pages.as_slice() else {
        anyhow::bail!("only one PDF page is qualified")
    };
    let page = objects
        .get(page_id)
        .context("PDF page object")?
        .dictionary_text();
    ensure!(
        only_names(
            page,
            &[
                "Type",
                "Page",
                "Parent",
                "Contents",
                "Resources",
                "Annots",
                "MediaBox",
                "TrimBox"
            ]
        ),
        "unknown page option"
    );
    let parent = object_after(&objects, page, "/Parent")
        .context("PDF page parent")?
        .dictionary_text();
    ensure!(
        only_names(
            parent,
            &[
                "Type", "Pages", "Kids", "Count", "ProcSet", "PDF", "Text", "ImageB", "ImageC"
            ]
        ) && clearance_number_after(parent, "/Count")? == 1.0,
        "inherited page options are unqualified"
    );
    let mut catalogs = objects
        .values()
        .filter(|object| object.dictionary_text().contains("/Type /Catalog"));
    let catalog = catalogs.next().context("PDF catalog")?.dictionary_text();
    ensure!(
        catalogs.next().is_none()
            && only_names(catalog, &["Type", "Catalog", "Pages", "Names", "Metadata"])
            && parse_single_ref_after(catalog, "/Pages") == parse_single_ref_after(page, "/Parent"),
        "catalog and page tree disagree"
    );
    if page.contains("/Annots") {
        let annotations = object_after(&objects, page, "/Annots").context("PDF annotations")?;
        ensure!(
            annotations
                .dictionary_text()
                .split_ascii_whitespace()
                .eq(["[", "]"]),
            "visible PDF annotations are unqualified"
        );
    }
    let resources = object_after(&objects, page, "/Resources")
        .context("PDF resources")?
        .dictionary_text();
    for key in ["/XObject", "/Pattern"] {
        ensure!(
            dictionary_after(resources, key).is_some_and(|value| value.trim().is_empty()),
            "unqualified PDF painting resource {key}"
        );
    }
    let colors = dictionary_after(resources, "/ColorSpace").context("PDF color space")?;
    ensure!(
        colors
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|pair| pair == ["/CSp", "/DeviceRGB"]),
        "unsupported RGB binding"
    );
    let graphics = dictionary_after(resources, "/ExtGState").context("PDF graphics resources")?;
    ensure!(only_names(graphics, &["GSa"]), "unknown graphics state");
    let state = object_after(&objects, graphics, "/GSa")
        .context("PDF graphics state")?
        .dictionary_text();
    let state_tokens = state
        .split(|ch: char| ch.is_ascii_whitespace() || ch == '<' || ch == '>')
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    ensure!(
        only_names(
            state,
            &[
                "Type",
                "ExtGState",
                "SA",
                "SM",
                "ca",
                "CA",
                "AIS",
                "SMask",
                "None"
            ]
        ) && clearance_number_after(state, "/ca")? == 1.0
            && clearance_number_after(state, "/CA")? == 1.0
            && state_tokens
                .windows(2)
                .any(|pair| pair == ["/AIS", "false"])
            && state_tokens
                .windows(2)
                .any(|pair| pair == ["/SMask", "/None"]),
        "unqualified transparency, stroke state, or soft mask"
    );
    let stream = object_after(&objects, page, "/Contents").context("PDF page content")?;
    ensure!(
        only_names(
            stream.dictionary_text(),
            &["Length", "Filter", "FlateDecode"]
        ),
        "unqualified content filter"
    );
    let content = stream
        .stream_bytes(&objects)?
        .context("PDF content stream")?;
    let mut decoded = Vec::new();
    if stream.dictionary_text().contains("/FlateDecode") {
        use std::io::Read as _;
        flate2::read::ZlibDecoder::new(content)
            .take(BYTE_LIMIT as u64 + 1)
            .read_to_end(&mut decoded)?;
    } else {
        decoded.extend_from_slice(content);
    }
    ensure!(decoded.len() <= BYTE_LIMIT, "oversized PDF content stream");
    let (width, height) = clearance_page_size(page)?;
    Ok(PageStream {
        width,
        height,
        content: decoded,
    })
}

fn rgb_agrees(left: [f64; 3], right: [f64; 3]) -> bool {
    left.iter()
        .zip(right)
        .all(|(a, b)| (a - b).abs() <= RGB_RESERVE)
}

fn rgb_follows_svg(
    expected: &[path_ink::Contour],
    actual: &[path_ink::Contour],
    bindings: &[usize],
) -> bool {
    expected.iter().zip(bindings).all(|(svg, &index)| {
        let pdf = &actual[index];
        rgb_agrees(svg.stroke, pdf.stroke)
            && match (svg.fill, pdf.fill) {
                (Some(a), Some(b)) => rgb_agrees(a, b),
                (None, None) => true,
                _ => false,
            }
    })
}

pub(crate) fn probe(plain_svg: &str, marked_svg: &str, plain: &[u8], marked: &[u8]) -> Result<()> {
    let printed = page_stream(plain)?;
    let shadow = page_stream(marked)?;
    ensure!(
        (printed.width, printed.height) == (shadow.width, shadow.height),
        "source paint changed the PDF page dimensions"
    );
    let read_page_paths = |page: &PageStream| {
        path_ink::read_ink(
            &page.content,
            page.width,
            page.height,
            "/GSa",
            POINT_RESERVE,
        )
    };
    let original_ink = read_page_paths(&printed)?;
    let painted_ink = read_page_paths(&shadow)?;
    path_ink::same_except_rgb(&printed.content, &shadow.content)?;

    for (svg, ink) in [(plain_svg, original_ink), (marked_svg, painted_ink)] {
        let svg_ties = path_ink::svg_ties(svg)?;
        ensure!(!svg_ties.is_empty(), "synthetic tie disappeared");
        let ties = path_ink::bind_contours(&svg_ties, &ink.ties, POINT_RESERVE)?;
        ensure!(
            rgb_follows_svg(&svg_ties, &ink.ties, &ties),
            "tie mark does not bind"
        );

        let svg_stems = path_ink::svg_stems(svg)?;
        ensure!(!svg_stems.is_empty(), "synthetic stems disappeared");
        let mut stems = Vec::new();
        for expected in &svg_stems {
            let mut matches = ink.strokes.iter().filter(|stroke| {
                path_ink::bind_contours(
                    std::slice::from_ref(expected),
                    std::slice::from_ref(*stroke),
                    POINT_RESERVE,
                )
                .is_ok()
            });
            let stem = matches
                .next()
                .context("missing PDF stroke for synthetic stem")?;
            ensure!(
                matches.next().is_none(),
                "ambiguous PDF stroke for synthetic stem"
            );
            stems.push(stem);
        }
        for (index, (svg, pdf)) in svg_stems.iter().zip(&stems).enumerate() {
            ensure!(
                rgb_agrees(svg.stroke, pdf.stroke) && pdf.fill.is_none(),
                "stem mark does not bind"
            );
            ensure!(
                !stems[..index]
                    .iter()
                    .any(|other| std::ptr::eq(*other, *pdf)),
                "one PDF stroke cannot bind several stems"
            );
        }
    }
    Ok(())
}
