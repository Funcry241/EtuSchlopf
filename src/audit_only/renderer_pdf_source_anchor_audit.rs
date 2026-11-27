//! Source-marker experiment for the flat page tree emitted by MuseScore.
//! A successful observation carries no write permission.

use super::{
    PdfObject, clearance_number_after, clearance_page_size, page_ids_in_order, parse_pdf_objects,
    parse_single_ref_after, parse_to_unicode_cmap, rfind_subslice,
};
use anyhow::{Context, Result, ensure};
use std::collections::{BTreeMap, BTreeSet};

#[path = "renderer_pdf_path_anchor_audit.rs"]
mod path_ink;
#[path = "renderer_pdf_head_glyph.rs"]
mod head_glyph;
#[path = "tied_stem_pdf_transition.rs"]
pub(crate) mod tied_stem_pdf_transition;

const POINT_RESERVE: f64 = 0.015;
const RGB_RESERVE: f64 = 0.000_001;
const BYTE_LIMIT: usize = 256 * 1024;
const FONT_LIMIT: usize = 128 * 1024;

#[derive(PartialEq)]
struct FontResource {
    name: String,
    widths: String,
    descriptor: String,
    cmap_stream: Vec<u8>,
    cmap: BTreeMap<u16, char>,
    font: Vec<u8>,
}

struct PageStream {
    width: f64,
    height: f64,
    content: Vec<u8>,
    fonts: Option<BTreeMap<String, FontResource>>,
}

// Restrict font proof to ordinary Qt dictionaries. PDF name escapes, comments,
// indirect dictionary keys, and ambiguous field occurrences stay unsupported.
fn dictionary_tokens(mut text: &str) -> Result<Vec<&str>> {
    ensure!(text.is_ascii(), "non-ASCII PDF dictionary");
    let delimiters = ['/', '(', ')', '<', '>', '[', ']', '%', '#'];
    let mut tokens = Vec::new();
    loop {
        text = text.trim_ascii_start();
        let Some(first) = text.as_bytes().first() else { break };
        let length = match *first {
            b'<' if text.starts_with("<<") => 2,
            b'>' if text.starts_with(">>") => 2,
            b'[' | b']' => 1,
            b'<' => {
                let end = text.find('>').context("unclosed PDF hex value")?;
                ensure!(text.as_bytes()[1..end].iter().all(u8::is_ascii_hexdigit), "unqualified PDF hex value");
                end + 1
            }
            b'(' => {
                let end = text.find(')').context("unclosed PDF string")?;
                ensure!(matches!(&text[..=end], "(Adobe)" | "(Identity)"), "unqualified PDF string");
                end + 1
            }
            _ => {
                let start = usize::from(*first == b'/');
                let stop = text[start..]
                    .find(|ch: char| ch.is_ascii_whitespace() || delimiters.contains(&ch))
                    .unwrap_or(text.len() - start);
                ensure!(stop != 0, "unqualified PDF dictionary token");
                start + stop
            }
        };
        tokens.push(&text[..length]);
        ensure!(tokens.len() <= 8192, "oversized PDF dictionary");
        text = &text[length..];
    }
    Ok(tokens)
}

fn field_value_len(atoms: &[&str]) -> Result<usize> {
    let first = *atoms.first().context("missing PDF field value")?;
    if matches!(first, "<<" | "[") {
        let mut stack = Vec::new();
        for (i, atom) in atoms.iter().enumerate() {
            match *atom {
                "<<" => stack.push(">>"),
                "[" => stack.push("]"),
                ">>" | "]" => {
                    ensure!(stack.pop() == Some(*atom), "mismatched PDF delimiter");
                    if stack.is_empty() { return Ok(i + 1); }
                }
                _ => {}
            }
            ensure!(stack.len() <= 4, "overly nested PDF dictionary");
        }
        anyhow::bail!("unclosed PDF compound value");
    }
    ensure!(!matches!(first, ">>" | "]" | "R"), "invalid PDF field value");
    if let [id, generation, "R", ..] = atoms
        && id.parse::<u32>().is_ok()
        && generation.parse::<u32>().is_ok()
    {
        return Ok(3);
    }
    Ok(1)
}

type Fields<'t, 's> = BTreeMap<&'s str, &'t [&'s str]>;

fn fields<'t, 's>(atoms: &'t [&'s str], allowed: &[&str]) -> Result<Fields<'t, 's>> {
    let ["<<", interior @ .., ">>"] = atoms else {
        anyhow::bail!("unqualified PDF dictionary envelope");
    };
    let mut values = BTreeMap::new();
    let mut todo = interior;
    while let Some((key, tail)) = todo.split_first() {
        ensure!(
            key.starts_with('/') && (allowed.is_empty() || allowed.contains(key)),
            "unqualified PDF dictionary key {key}"
        );
        let length = field_value_len(tail)?;
        let (value, remaining) = tail.split_at(length);
        ensure!(values.insert(*key, value).is_none(), "duplicate PDF dictionary key {key}");
        todo = remaining;
    }
    Ok(values)
}

fn field_atom<'s>(fields: &Fields<'_, 's>, key: &str) -> Result<&'s str> {
    let Some([value]) = fields.get(key) else { anyhow::bail!("missing or compound {key}"); };
    Ok(value)
}

fn field_reference(atoms: &[&str]) -> Result<u32> {
    let [id, "0", "R"] = atoms else { anyhow::bail!("unqualified PDF object reference"); };
    let id = id.parse()?;
    ensure!(id != 0, "invalid PDF object reference");
    Ok(id)
}

fn required_reference(fields: &Fields<'_, '_>, key: &str) -> Result<u32> {
    field_reference(fields.get(key).context("missing PDF object reference")?)
}

// Cross-reference the original bytes, not an opportunistic object-shaped
// substring inside a stream. This covers a flat PDF with one classic table.
fn xref_objects(bytes: &[u8]) -> Result<(u32, BTreeMap<u32, PdfObject>)> {
    let start = rfind_subslice(bytes, b"\nstartxref\n").context("missing classic PDF xref footer")?;
    let footer = std::str::from_utf8(&bytes[start..])?.split_ascii_whitespace().collect::<Vec<_>>();
    let ["startxref", xref, "%%EOF"] = footer.as_slice() else { anyhow::bail!("unqualified PDF footer"); };
    let xref: usize = xref.parse()?;
    let table = std::str::from_utf8(bytes.get(xref..start).context("PDF xref offset out of bounds")?)?;
    let mut lines = table.lines();
    ensure!(lines.next().is_some_and(|line| line.trim_ascii() == "xref"), "unqualified PDF xref");
    let header = lines.next().context("missing PDF xref subsection")?.split_ascii_whitespace().collect::<Vec<_>>();
    let ["0", count] = header.as_slice() else { anyhow::bail!("unsupported PDF xref subsection"); };
    let count: u32 = count.parse()?;
    ensure!((2..=4096).contains(&count), "oversized or empty PDF xref");
    ensure!(lines.next().is_some_and(|line| line.split_ascii_whitespace().eq(["0000000000", "65535", "f"])),
        "unqualified PDF xref object zero");
    let mut offsets = BTreeMap::new();
    for id in 1..count {
        let entry = lines.next().context("truncated PDF xref")?.split_ascii_whitespace().collect::<Vec<_>>();
        let [position, "00000", "n"] = entry.as_slice() else { anyhow::bail!("unqualified PDF xref entry"); };
        let offset = position.parse::<usize>()?;
        ensure!(position.len() == 10 && (9..xref).contains(&offset) && offsets.insert(offset, id).is_none(),
            "misplaced or duplicate PDF object offset");
    }
    ensure!(lines.next().is_some_and(|line| line.trim_ascii() == "trailer"), "missing PDF xref trailer");
    let trailer = lines.collect::<Vec<_>>().join("\n");
    let trailer_tokens = dictionary_tokens(&trailer)?;
    let trailer = fields(&trailer_tokens, &["/Size", "/Info", "/Root", "/ID"])?;
    ensure!(field_atom(&trailer, "/Size")?.parse::<u32>()? == count, "PDF xref trailer size differs");
    let root = required_reference(&trailer, "/Root")?;
    let mut objects = BTreeMap::new();
    let mut entries = offsets.into_iter().peekable();
    while let Some((start, id)) = entries.next() {
        let end = entries.peek().map_or(xref, |(offset, _)| *offset);
        let header = format!("{id} 0 obj\n");
        let body = bytes[start..end]
            .strip_prefix(header.as_bytes())
            .context("PDF xref does not point to the declared object")?
            .trim_ascii_end()
            .strip_suffix(b"\nendobj")
            .context("PDF object lacks its final end marker")?
            .to_vec();
        objects.insert(id, PdfObject { body });
    }
    ensure!(objects.contains_key(&root), "missing PDF trailer root");
    Ok((root, objects))
}

fn font_stream(
    objects: &BTreeMap<u32, PdfObject>,
    id: u32,
    true_type: bool,
) -> Result<Vec<u8>> {
    let object = objects.get(&id).context("missing PDF font stream")?;
    let atoms = dictionary_tokens(object.dictionary_text())?;
    let form = fields(&atoms, &["/Length", "/Length1", "/Filter"])?;
    let raw = object.stream_bytes(objects)?.context("missing PDF font payload")?;
    let data = if form.contains_key("/Filter") {
        ensure!(field_atom(&form, "/Filter")? == "/FlateDecode", "unqualified PDF font filter");
        use std::io::Read as _;
        let mut decoded = Vec::new();
        flate2::read::ZlibDecoder::new(raw)
            .take(FONT_LIMIT as u64 + 1)
            .read_to_end(&mut decoded)?;
        decoded
    } else {
        raw.to_vec()
    };
    ensure!(!data.is_empty() && data.len() <= FONT_LIMIT, "unsupported PDF font stream size");
    if true_type {
        ensure!(field_atom(&form, "/Length1")?.parse::<usize>()? == data.len(), "PDF font length differs");
    } else {
        ensure!(!form.contains_key("/Length1"), "unsupported PDF CMap stream");
    }
    Ok(data)
}

fn font_resource(objects: &BTreeMap<u32, PdfObject>, id: u32) -> Result<FontResource> {
    let dictionary = |id| -> Result<&str> {
        Ok(objects.get(&id).context("missing PDF font dictionary")?.dictionary_text())
    };
    let tokens = dictionary_tokens(dictionary(id)?)?;
    let outer = fields(&tokens, &[
        "/Type", "/Subtype", "/BaseFont", "/Encoding", "/DescendantFonts", "/ToUnicode",
    ])?;
    let name = field_atom(&outer, "/BaseFont")?;
    ensure!(name.starts_with('/') && field_atom(&outer, "/Type")? == "/Font"
        && field_atom(&outer, "/Subtype")? == "/Type0"
        && field_atom(&outer, "/Encoding")? == "/Identity-H", "unqualified PDF Type0 font");
    let Some(["[", entry @ .., "]"]) = outer.get("/DescendantFonts") else {
        anyhow::bail!("unqualified PDF descendant font array");
    };
    let tokens = dictionary_tokens(dictionary(field_reference(entry)?)?)?;
    let cid = fields(&tokens, &[
        "/Type", "/Subtype", "/BaseFont", "/CIDSystemInfo", "/FontDescriptor", "/CIDToGIDMap", "/W", "/DW",
    ])?;
    ensure!(field_atom(&cid, "/Type")? == "/Font"
        && field_atom(&cid, "/Subtype")? == "/CIDFontType2"
        && field_atom(&cid, "/BaseFont")? == name
        && field_atom(&cid, "/CIDToGIDMap")? == "/Identity", "unqualified PDF CIDFontType2 mapping");
    let identity = fields(cid.get("/CIDSystemInfo").context("missing PDF CID identity")?,
        &["/Registry", "/Ordering", "/Supplement"])?;
    ensure!(field_atom(&identity, "/Registry")? == "(Adobe)"
        && field_atom(&identity, "/Ordering")? == "(Identity)"
        && field_atom(&identity, "/Supplement")? == "0", "unqualified PDF CID identity");
    let widths = cid.get("/W").context("missing PDF CID widths")?.join(" ");
    let default_width = cid.get("/DW").map_or("1000".to_owned(), |width| width.join(" "));

    let tokens = dictionary_tokens(dictionary(required_reference(&cid, "/FontDescriptor")?)?)?;
    let descriptor = fields(&tokens, &[
        "/Type", "/FontName", "/Flags", "/FontBBox", "/ItalicAngle", "/Ascent", "/Descent", "/CapHeight",
        "/StemV", "/FontFile2", "/CIDSet",
    ])?;
    ensure!(field_atom(&descriptor, "/Type")? == "/FontDescriptor", "unqualified PDF FontDescriptor");
    let declared = field_atom(&descriptor, "/FontName")?.strip_prefix('/').context("unqualified PDF font name")?;
    let Some((subset_prefix, declared_base)) = declared.split_once('+') else {
        anyhow::bail!("missing PDF font subset prefix");
    };
    ensure!(subset_prefix.len() == 6 && subset_prefix.bytes().all(|byte| byte.is_ascii_uppercase())
        && name.strip_prefix('/') == Some(declared_base), "unqualified PDF font name");
    let font = font_stream(objects, required_reference(&descriptor, "/FontFile2")?, true)?;
    // Subset prefixes and indirect object numbers legitimately vary with Qt's
    // emission order. All actual metric fields and the payload stay identical.
    let metrics = descriptor.iter()
        .filter(|(key, _)| !matches!(**key, "/FontName" | "/FontFile2" | "/CIDSet"))
        .map(|(key, atoms)| format!("{key} {}", atoms.join(" ")))
        .collect::<Vec<_>>()
        .join(" ");
    let cmap_stream = font_stream(objects, required_reference(&outer, "/ToUnicode")?, false)?;
    let cmap = parse_to_unicode_cmap(&cmap_stream);
    ensure!(!cmap.is_empty(), "missing PDF font CMap");
    Ok(FontResource {
        name: name.to_owned(),
        widths: format!("{default_width} {widths}"),
        descriptor: metrics,
        cmap_stream,
        cmap,
        font,
    })
}

fn page_fonts(resources: &str, objects: &BTreeMap<u32, PdfObject>) -> Result<BTreeMap<String, FontResource>> {
    let tokens = dictionary_tokens(resources)?;
    let resources = fields(&tokens, &["/ColorSpace", "/ExtGState", "/Pattern", "/Font", "/XObject"])?;
    let colors = fields(resources.get("/ColorSpace").context("missing PDF page color space")?, &[])?;
    ensure!(field_atom(&colors, "/CSp")? == "/DeviceRGB", "unqualified PDF RGB resource");
    let font_refs = fields(resources.get("/Font").context("missing PDF page font resources")?, &[])?;
    ensure!(!font_refs.is_empty() && font_refs.len() <= 32, "unqualified PDF font resource count");
    font_refs.into_iter().map(|(slot, spec)| Ok((slot.to_owned(), font_resource(objects, field_reference(spec)?)?))).collect()
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

fn page_streams(bytes: &[u8]) -> Result<Vec<PageStream>> {
    page_streams_for(bytes, false)
}

fn page_streams_for(bytes: &[u8], with_fonts: bool) -> Result<Vec<PageStream>> {
    ensure!(
        bytes.len() <= BYTE_LIMIT && bytes.starts_with(b"%PDF-1.4"),
        "outside the synthetic PDF profile"
    );
    let (xref_root, objects) = if with_fonts {
        let (root, objects) = xref_objects(bytes)?;
        (Some(root), objects)
    } else {
        (None, parse_pdf_objects(bytes)?)
    };
    let pages = page_ids_in_order(&objects)?;
    let mut catalogs = objects
        .iter()
        .filter(|(_, object)| object.dictionary_text().contains("/Type /Catalog"));
    let (&catalog_id, catalog) = catalogs.next().context("PDF catalog")?;
    ensure!(xref_root.is_none_or(|root| root == catalog_id), "PDF xref root differs from catalog");
    let catalog = catalog.dictionary_text();
    ensure!(
        catalogs.next().is_none()
            && only_names(catalog, &["Type", "Catalog", "Pages", "Names", "Metadata"]),
        "unqualified PDF catalog"
    );
    let root = parse_single_ref_after(catalog, "/Pages").context("PDF root page tree")?;
    if with_fonts {
        let tokens = dictionary_tokens(catalog)?;
        let catalog = fields(&tokens, &["/Type", "/Pages", "/Names", "/Metadata"])?;
        ensure!(field_atom(&catalog, "/Type")? == "/Catalog" && required_reference(&catalog, "/Pages")? == root,
            "unqualified PDF page root binding");
    }
    let parent = objects
        .get(&root)
        .context("PDF page parent")?
        .dictionary_text();
    if with_fonts {
        let tokens = dictionary_tokens(parent)?;
        let tree = fields(&tokens, &["/Type", "/Kids", "/Count", "/ProcSet"])?;
        ensure!(field_atom(&tree, "/Type")? == "/Pages", "unqualified PDF page parent");
    }
    let kids = parent
        .split_once("/Kids")
        .context("PDF page children")?
        .1
        .trim_start()
        .strip_prefix('[')
        .context("PDF page child array")?
        .split_once(']')
        .context("unclosed PDF page child array")?
        .0
        .split_ascii_whitespace()
        .collect::<Vec<_>>();
    let (references, extra) = kids.as_chunks::<3>();
    ensure!(
        only_names(
            parent,
            &[
                "Type", "Pages", "Kids", "Count", "ProcSet", "PDF", "Text", "ImageB", "ImageC"
            ]
        ) && clearance_number_after(parent, "/Count")? == pages.len() as f64
            && extra.is_empty()
            && references.len() == pages.len()
            && references
                .iter()
                .zip(&pages)
                .all(|([id, generation, marker], page)| {
                    *generation == "0" && *marker == "R" && id.parse::<u32>() == Ok(*page)
                }),
        "inherited page options or page order are unqualified"
    );
    let mut seen_streams = BTreeSet::new();
    pages
        .iter()
        .map(|page_id| {
            let page = objects
                .get(page_id)
                .context("PDF page object")?
                .dictionary_text();
            ensure!(
                parse_single_ref_after(page, "/Parent") == Some(root),
                "unqualified nested PDF page"
            );
            let contents =
                parse_single_ref_after(page, "/Contents").context("PDF page content reference")?;
            ensure!(seen_streams.insert(contents), "reused PDF page stream");
            page_stream(&objects, page, contents, with_fonts)
        })
        .collect()
}

fn page_stream(
    objects: &BTreeMap<u32, PdfObject>,
    page: &str,
    contents: u32,
    with_fonts: bool,
) -> Result<PageStream> {
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
    if page.contains("/Annots") {
        let annotations = object_after(objects, page, "/Annots").context("PDF annotations")?;
        ensure!(
            annotations
                .dictionary_text()
                .split_ascii_whitespace()
                .eq(["[", "]"]),
            "visible PDF annotations are unqualified"
        );
    }
    let resources = object_after(objects, page, "/Resources")
        .context("PDF resources")?
        .dictionary_text();
    let fonts = if with_fonts {
        let tokens = dictionary_tokens(page)?;
        let page = fields(&tokens, &["/Type", "/Parent", "/Contents", "/Resources", "/Annots", "/MediaBox", "/TrimBox"])?;
        ensure!(field_atom(&page, "/Type")? == "/Page" && required_reference(&page, "/Contents")? == contents,
            "unqualified PDF page contents binding");
        ensure!(page.get("/TrimBox").is_none_or(|trim| Some(trim) == page.get("/MediaBox")),
            "unqualified PDF trim box");
        Some(page_fonts(resources, objects)?)
    } else {
        None
    };
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
    let state = object_after(objects, graphics, "/GSa")
        .context("PDF graphics state")?
        .dictionary_text();
    if with_fonts {
        let tokens = dictionary_tokens(state)?;
        let profile = fields(&tokens, &["/Type", "/SA", "/SM", "/ca", "/CA", "/AIS", "/SMask"])?;
        ensure!(field_atom(&profile, "/Type")? == "/ExtGState", "unqualified PDF graphics profile");
    }
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
    let stream = objects.get(&contents).context("PDF page content")?;
    if with_fonts {
        let tokens = dictionary_tokens(stream.dictionary_text())?;
        let profile = fields(&tokens, &["/Length", "/Filter"])?;
        if profile.contains_key("/Filter") {
            ensure!(field_atom(&profile, "/Filter")? == "/FlateDecode", "unqualified PDF content filter");
        }
    }
    ensure!(
        only_names(
            stream.dictionary_text(),
            &["Length", "Filter", "FlateDecode"]
        ),
        "unqualified content filter"
    );
    let content = stream
        .stream_bytes(objects)?
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
        fonts,
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

fn stem_bindings(svg: &str, strokes: &[path_ink::Contour]) -> Result<BTreeSet<usize>> {
    let (expected, stems) = path_ink::svg_vertical_strokes(svg)?;
    let mut vertical = Vec::new();
    let mut page_indices = Vec::new();
    for (index, contour) in strokes.iter().enumerate() {
        let [(x1, y1), (x2, y2)] = contour.points.as_slice() else {
            anyhow::bail!("unexpected PDF stroke shape");
        };
        if x1 == x2 {
            vertical.push(contour.clone());
            page_indices.push(index);
        } else {
            ensure!(y1 == y2, "unsupported PDF stroke orientation");
        }
    }
    if expected.is_empty() && vertical.is_empty() {
        return Ok(BTreeSet::new());
    }
    let bindings = path_ink::bind_contours(&expected, &vertical, POINT_RESERVE)?;
    ensure!(
        rgb_follows_svg(&expected, &vertical, &bindings),
        "stroke mark does not bind"
    );
    Ok(stems
        .into_iter()
        .map(|stem| page_indices[bindings[stem]])
        .collect())
}

fn tie_bindings(expected: &[path_ink::Contour], ties: &[path_ink::Contour]) -> Result<Vec<usize>> {
    if expected.is_empty() && ties.is_empty() {
        return Ok(Vec::new());
    }
    let bindings = path_ink::bind_contours(expected, ties, POINT_RESERVE)?;
    ensure!(
        rgb_follows_svg(expected, ties, &bindings),
        "tie mark does not bind"
    );
    Ok(bindings)
}

struct MarkedPagePaths {
    plain: path_ink::InkPage,
    marked: path_ink::InkPage,
    stems: BTreeSet<usize>,
}

fn marked_page_paths(
    plain_svg: &str,
    marked_svg: &str,
    printed: &PageStream,
    shadow: &PageStream,
) -> Result<MarkedPagePaths> {
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
    path_ink::same_except_rgb(&printed.content, &shadow.content)?;
    let original_ink = read_page_paths(printed)?;
    let painted_ink = read_page_paths(shadow)?;
    let original_ties = tie_bindings(&path_ink::svg_ties(plain_svg)?, &original_ink.ties)?;
    let painted_ties = tie_bindings(&path_ink::svg_ties(marked_svg)?, &painted_ink.ties)?;
    ensure!(
        original_ties == painted_ties
            && original_ink
                .ties
                .iter()
                .zip(&painted_ink.ties)
                .all(|(original, painted)| !rgb_agrees(original.stroke, painted.stroke)),
        "PDF tie colors or order do not follow the source marks"
    );
    let original_stems = stem_bindings(plain_svg, &original_ink.strokes)?;
    let painted_stems = stem_bindings(marked_svg, &painted_ink.strokes)?;
    // Bar lines, staff lines and stems share the same PDF paint operator. A
    // color change binds exactly the source-marked subset of those strokes.
    ensure!(
        original_stems == painted_stems
            && original_ink.strokes.len() == painted_ink.strokes.len()
            && original_ink
                .strokes
                .iter()
                .zip(&painted_ink.strokes)
                .enumerate()
                .all(|(i, (original, painted))| {
                    original_stems.contains(&i) != rgb_agrees(original.stroke, painted.stroke)
                }),
        "PDF stroke colors do not follow the source-marked stems"
    );
    Ok(MarkedPagePaths { plain: original_ink, marked: painted_ink, stems: original_stems })
}

pub(crate) fn probe_pages(
    plain_svg: &[&str],
    marked_svg: &[&str],
    plain: &[u8],
    marked: &[u8],
) -> Result<()> {
    let printed = page_streams(plain)?;
    let shadow = page_streams(marked)?;
    ensure!(
        printed.len() == shadow.len()
            && printed.len() == plain_svg.len()
            && printed.len() == marked_svg.len(),
        "PDF and SVG page counts disagree"
    );
    let (mut ties, mut stems) = (0, 0);
    for (index, ((a, b), (plain_svg, marked_svg))) in printed
        .iter()
        .zip(&shadow)
        .zip(plain_svg.iter().zip(marked_svg))
        .enumerate()
    {
        let paths = marked_page_paths(plain_svg, marked_svg, a, b)
            .with_context(|| format!("PDF source-anchor page {}", index + 1))?;
        ties += paths.plain.ties.len();
        stems += paths.stems.len();
    }
    ensure!(ties > 0 && stems > 0, "synthetic ties or stems disappeared");
    Ok(())
}

pub(crate) fn probe(plain_svg: &str, marked_svg: &str, plain: &[u8], marked: &[u8]) -> Result<()> {
    probe_pages(&[plain_svg], &[marked_svg], plain, marked)
}

pub(crate) struct PdfHeadInk {
    pub(crate) mark: [u8; 3],
    /// Closed quadratic contour segments in physical PDF page points (y down).
    pub(crate) outline: Vec<Vec<[(f64, f64); 3]>>,
}

pub(crate) struct PdfStemInk {
    pub(crate) mark: [u8; 3],
    pub(crate) start: (f64, f64),
    pub(crate) end: (f64, f64),
    pub(crate) width: f64,
}

pub(crate) struct PdfHeadPage {
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) heads: Vec<PdfHeadInk>,
    pub(crate) stems: Vec<PdfStemInk>,
    /// Conservative outer bounds of the other text paint in physical page points.
    pub(crate) foreground: Vec<[[f64; 2]; 2]>,
}

fn source_color(ink: [f64; 3]) -> Result<[u8; 3]> {
    let mut result = [0; 3];
    for (component, byte) in ink.into_iter().zip(&mut result) {
        ensure!((0.0..=1.0).contains(&component), "invalid PDF source color");
        let quantized = (component * 255.0).round();
        ensure!((component - quantized / 255.0).abs() <= RGB_RESERVE, "ambiguous PDF source color");
        *byte = quantized as u8;
    }
    Ok(result)
}

fn read_text(page: &PageStream) -> Result<Vec<path_ink::TextInk>> {
    path_ink::read_head_text(&page.content, page.width, page.height, "/GSa", POINT_RESERVE)
}

fn glyph_font<'a>(
    fonts: &'a BTreeMap<String, FontResource>,
    show: &path_ink::TextInk,
) -> Result<&'a FontResource> {
    fonts.get(&show.font).context("unbound PDF glyph font resource")
}

fn foreground_bounds(show: &path_ink::TextInk, font: &FontResource) -> Result<Option<[[f64; 2]; 2]>> {
    // /Identity-H and /CIDToGIDMap /Identity bind the painted glyph. The
    // ToUnicode and ActualText extraction data cannot remove its geometry.
    let Some(glyph) = head_glyph::foreground_hull_or_empty(&font.font, show.cid)
        .with_context(|| format!("PDF foreground glyph {} CID {}", font.name, show.cid))?
    else {
        return Ok(None);
    };
    let [left, bottom, right, top] = glyph.hull;
    let scale = 2.0 * f64::from(glyph.units_per_em);
    let corners = [[left, bottom], [left, top], [right, bottom], [right, top]].map(|[x, y]| {
        let (x, y) = show.transform(f64::from(x) / scale, f64::from(y) / scale);
        [x, y]
    });
    ensure!(
        corners.iter().flatten().all(|point| point.is_finite()),
        "unrepresentable PDF foreground geometry"
    );
    // A clip can only remove ink. The entire control-point hull gives a
    // conservative test even when some foreground ink is clipped.
    let mut minimum = corners[0];
    let mut maximum = corners[0];
    for [x, y] in corners.into_iter().skip(1) {
        minimum = [minimum[0].min(x), minimum[1].min(y)];
        maximum = [maximum[0].max(x), maximum[1].max(y)];
    }
    Ok(Some([minimum, maximum]))
}

pub(crate) struct PdfForegroundInk {
    pub(crate) bounds: [[f64; 2]; 2],
    pub(crate) marked_actual_text: bool,
    pub(crate) font: String,
    pub(crate) cid: u16,
}

/// Standalone text-paint observation without a source-marking authorization.
pub(crate) fn probe_pdf_foreground(bytes: &[u8]) -> Result<Vec<Vec<PdfForegroundInk>>> {
    page_streams_for(bytes, true)?
        .iter()
        .enumerate()
        .map(|(index, page)| {
            let fonts = page.fonts.as_ref().context("missing qualified PDF page fonts")?;
            let glyphs = read_text(page)
                .with_context(|| format!("PDF foreground text on page {}", index + 1))?;
            let mut foreground = Vec::new();
            for show in glyphs {
                let font = glyph_font(fonts, &show)?;
                if let Some(bounds) = foreground_bounds(&show, font)? {
                    foreground.push(PdfForegroundInk {
                        bounds,
                        marked_actual_text: show.marked_actual_text,
                        font: font.name.clone(),
                        cid: show.cid,
                    });
                }
            }
            Ok(foreground)
        })
        .collect()
}

struct MarkedPageText {
    heads: Vec<PdfHeadInk>,
    foreground: Vec<[[f64; 2]; 2]>,
}

fn marked_page_text(
    printed: &PageStream,
    shadow: &PageStream,
    expected: &[[u8; 3]],
) -> Result<MarkedPageText> {
    let fonts = printed.fonts.as_ref().context("missing qualified PDF page fonts")?;
    ensure!(shadow.fonts.as_ref() == Some(fonts), "PDF font resources or embedded payload differ");
    let original = read_text(printed)?;
    let painted = read_text(shadow)?;
    ensure!(original.len() == painted.len(), "PDF glyph inventory differs");
    let mut remaining = expected.iter().copied().collect::<BTreeSet<_>>();
    ensure!(remaining.len() == expected.len() && !remaining.contains(&[0; 3]), "duplicate or black source head mark");
    let mut heads = Vec::new();
    let mut foreground = Vec::new();
    for (ordinary, marked) in original.iter().zip(&painted) {
        ensure!(
            ordinary.font == marked.font && ordinary.cid == marked.cid
                && ordinary.marked_actual_text == marked.marked_actual_text,
            "PDF glyph show differs"
        );
        let font = glyph_font(fonts, marked)?;
        let code = font.cmap.get(&marked.cid).copied();
        let marked_color = source_color(marked.fill)?;
        let known_head = font.name == "/Leland"
            && code.is_some_and(|code| ('\u{e0a0}'..='\u{e0ff}').contains(&code));
        if !remaining.remove(&marked_color) {
            ensure!(!known_head && rgb_agrees(ordinary.fill, marked.fill), "missing or duplicate PDF notehead source mark");
            if let Some(bounds) = foreground_bounds(ordinary, font)? {
                foreground.push(bounds);
            }
            continue;
        }
        ensure!(
            known_head && matches!(code, Some('\u{e0a3}' | '\u{e0a4}')) && ordinary.cid != 0
                && source_color(ordinary.fill)? == [0; 3],
            "unqualified marked PDF notehead"
        );
        let glyph = head_glyph::glyph_outline(&font.font, marked.cid)?;
        let scale = 2.0 * f64::from(glyph.units_per_em);
        let project = |[x, y]: [i32; 2]| ordinary.transform(f64::from(x) / scale, f64::from(y) / scale);
        let outline = glyph.contours.iter().map(|contour| {
            contour.iter().map(|curve| {
                let a = project(curve.start);
                let b = project(curve.end);
                let middle = curve.control.map_or_else(|| (a.0.midpoint(b.0), a.1.midpoint(b.1)), project);
                [a, middle, b]
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        let points = outline.iter().flatten().flatten().copied().collect::<Vec<_>>();
        ordinary.proves_unclipped(&points, POINT_RESERVE)?;
        marked.proves_unclipped(&points, POINT_RESERVE)?;
        heads.push(PdfHeadInk { mark: marked_color, outline });
    }
    ensure!(remaining.is_empty(), "missing PDF notehead source marks");
    Ok(MarkedPageText { heads, foreground })
}

/// Binds source-marked heads and stems to actual, paired regular/shadow PDF ink.
/// The caller supplies the independent source/SVG inventory and compares the
/// actual contour geometry. This native PDF observation issues no authority.
pub(crate) fn probe_head_pages(
    plain_svg: &[&str],
    marked_svg: &[&str],
    plain: &[u8],
    marked: &[u8],
    required_marks: &[Vec<[u8; 3]>],
) -> Result<Vec<PdfHeadPage>> {
    let printed = page_streams_for(plain, true)?;
    let shadow = page_streams_for(marked, true)?;
    ensure!(printed.len() == shadow.len() && printed.len() == plain_svg.len()
        && printed.len() == marked_svg.len() && printed.len() == required_marks.len(), "PDF source-anchor page counts differ");
    ensure!(required_marks.iter().any(|page| !page.is_empty()), "missing source head inventory");
    printed.iter().zip(&shadow).enumerate().map(|(index, (a, b))| {
        let paths = marked_page_paths(plain_svg[index], marked_svg[index], a, b)
            .with_context(|| format!("PDF source-anchor path page {}", index + 1))?;
        let text = marked_page_text(a, b, &required_marks[index])
            .with_context(|| format!("PDF source-anchor head page {}", index + 1))?;
        let stems = paths.stems.into_iter().map(|position| {
            let ordinary = &paths.plain.strokes[position];
            let [start, end] = ordinary.points.as_slice() else { anyhow::bail!("unqualified PDF stem geometry"); };
            let mark = source_color(paths.marked.strokes[position].stroke)?;
            Ok(PdfStemInk { mark, start: *start, end: *end, width: ordinary.width })
        }).collect::<Result<Vec<_>>>()?;
        Ok(PdfHeadPage { width: a.width, height: a.height, heads: text.heads, stems, foreground: text.foreground })
    }).collect()
}
