// Read-only physical component envelopes for an optional fixed-page trial.
// Unsupported PDF syntax or ambiguous ownership never supplies adoption evidence.
#[derive(Clone, Debug)]
pub(crate) struct PageClearanceSystem {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) page: usize,
    pub(crate) ink_top: f64,
    pub(crate) ink_bottom: f64,
}
#[derive(Clone, Debug)]
pub(crate) struct PageClearanceObservation {
    pub(crate) systems: Vec<PageClearanceSystem>,
    pub(crate) gaps: Vec<Vec<f64>>,
    pub(crate) page_bottom: Vec<f64>,
    pub(crate) page_top: Vec<f64>,
    pub(crate) glyphs: Vec<BTreeMap<char, usize>>,
}
#[derive(Clone, Copy, Debug)]
struct ClearanceBox([f64; 4]);
impl ClearanceBox {
    fn intersects(self, other: Self) -> bool {
        self.0[0].max(other.0[0]) <= self.0[2].min(other.0[2])
            && self.0[1].max(other.0[1]) <= self.0[3].min(other.0[3])
    }
    fn inside(self, outer: Self) -> bool {
        self.0[0] >= outer.0[0]
            && self.0[1] >= outer.0[1]
            && self.0[2] <= outer.0[2]
            && self.0[3] <= outer.0[3]
    }
    fn checked(values: [f64; 4]) -> anyhow::Result<Self> {
        anyhow::ensure!(
            values.iter().all(|v| v.is_finite())
                && values[0] <= values[2]
                && values[1] <= values[3],
            "PageClearance: invalid component bounds"
        );
        Ok(Self(values))
    }
}
#[derive(Clone)]
struct ClearanceComponent {
    bounds: ClearanceBox,
    glyph: Option<char>,
    text: Option<String>,
    bold: bool,
}
#[derive(Clone)]
struct ClearanceFontGlyph {
    bounds: Option<[f64; 4]>,
    advance: f64,
    character: char,
}

fn clearance_font(
    font: &PdfObject,
    objects: &BTreeMap<u32, PdfObject>,
) -> anyhow::Result<BTreeMap<u16, ClearanceFontGlyph>> {
    let dict = font.dictionary_text();
    anyhow::ensure!(
        dict.contains("/Encoding /Identity-H"),
        "PageClearance: unsupported font encoding"
    );
    let descendants = parse_ref_array_after(dict, "/DescendantFonts");
    let [id] = descendants.as_slice() else {
        anyhow::bail!("PageClearance: missing font descendant");
    };
    let descendant = objects
        .get(id)
        .ok_or_else(|| anyhow::anyhow!("PageClearance: missing CID font"))?
        .dictionary_text();
    anyhow::ensure!(
        descendant.contains("/CIDToGIDMap /Identity"),
        "PageClearance: unsupported glyph map"
    );
    let descriptor = objects
        .get(
            &parse_single_ref_after(descendant, "/FontDescriptor")
                .ok_or_else(|| anyhow::anyhow!("PageClearance: descriptor"))?,
        )
        .ok_or_else(|| anyhow::anyhow!("PageClearance: descriptor object"))?
        .dictionary_text();
    let font_bytes = objects
        .get(
            &parse_single_ref_after(descriptor, "/FontFile2")
                .ok_or_else(|| anyhow::anyhow!("PageClearance: embedded TrueType required"))?,
        )
        .ok_or_else(|| anyhow::anyhow!("PageClearance: font object"))?
        .decoded_stream()?
        .ok_or_else(|| anyhow::anyhow!("PageClearance: font stream"))?;
    let cmap_id = parse_single_ref_after(dict, "/ToUnicode")
        .ok_or_else(|| anyhow::anyhow!("PageClearance: unicode map"))?;
    let cmap_bytes = objects
        .get(&cmap_id)
        .ok_or_else(|| anyhow::anyhow!("PageClearance: unicode object"))?
        .decoded_stream()?
        .ok_or_else(|| anyhow::anyhow!("PageClearance: unicode stream"))?;
    clearance_true_type(&font_bytes, &parse_to_unicode_cmap(&cmap_bytes))
}

fn clearance_true_type(
    bytes: &[u8],
    cmap: &BTreeMap<u16, char>,
) -> anyhow::Result<BTreeMap<u16, ClearanceFontGlyph>> {
    let parse = || -> Option<BTreeMap<u16, ClearanceFontGlyph>> {
        let u16_at = |p: usize| {
            Some(u16::from_be_bytes(
                bytes.get(p..p.checked_add(2)?)?.try_into().ok()?,
            ))
        };
        let u32_at = |p: usize| {
            usize::try_from(u32::from_be_bytes(
                bytes.get(p..p.checked_add(4)?)?.try_into().ok()?,
            ))
            .ok()
        };
        if bytes.get(..4)? != [0, 1, 0, 0] {
            return None;
        }
        let mut tables = BTreeMap::new();
        for i in 0..usize::from(u16_at(4)?) {
            let p = 12 + 16 * i;
            let start = u32_at(p + 8)?;
            let len = u32_at(p + 12)?;
            bytes.get(start..start.checked_add(len)?)?;
            tables.insert(bytes.get(p..p + 4)?, (start, len));
        }
        let &(head, head_len) = tables.get(b"head".as_slice())?;
        let &(loca, loca_len) = tables.get(b"loca".as_slice())?;
        let &(glyf, glyf_len) = tables.get(b"glyf".as_slice())?;
        let &(maxp, maxp_len) = tables.get(b"maxp".as_slice())?;
        let &(hhea, hhea_len) = tables.get(b"hhea".as_slice())?;
        let &(hmtx, hmtx_len) = tables.get(b"hmtx".as_slice())?;
        if head_len < 54 || maxp_len < 6 || hhea_len < 36 {
            return None;
        }
        let units = f64::from(u16_at(head + 18)?);
        if units <= 0.0 {
            return None;
        }
        let count = u16_at(maxp + 4)?;
        let metrics = usize::from(u16_at(hhea + 34)?);
        if metrics == 0 || metrics > usize::from(count) || metrics.checked_mul(4)? > hmtx_len {
            return None;
        }
        let format = u16_at(head + 50)?;
        let stride = match format {
            0 => 2,
            1 => 4,
            _ => return None,
        };
        let position = |p| {
            if format == 0 {
                Some(usize::from(u16_at(loca + p)?) * 2)
            } else {
                u32_at(loca + p)
            }
        };
        let mut result = BTreeMap::new();
        for (&code, &character) in cmap {
            if code >= count {
                return None;
            }
            let offset = usize::from(code) * stride;
            if offset + 2 * stride > loca_len {
                return None;
            }
            let start = position(offset)?;
            let end = position(offset + stride)?;
            if start > end || end > glyf_len {
                return None;
            }
            let bounds = if start == end {
                None
            } else {
                if end < start + 10 {
                    return None;
                }
                let signed = |p| {
                    Some(
                        f64::from(i16::from_be_bytes(bytes.get(p..p + 2)?.try_into().ok()?))
                            / units,
                    )
                };
                let b = [
                    signed(glyf + start + 2)?,
                    signed(glyf + start + 4)?,
                    signed(glyf + start + 6)?,
                    signed(glyf + start + 8)?,
                ];
                if b[0] > b[2] || b[1] > b[3] {
                    return None;
                }
                Some(b)
            };
            let advance = f64::from(u16_at(hmtx + usize::from(code).min(metrics - 1) * 4)?) / units;

            result.insert(
                code,
                ClearanceFontGlyph {
                    bounds,
                    advance,
                    character,
                },
            );
        }
        Some(result)
    };
    parse().ok_or_else(|| anyhow::anyhow!("PageClearance: incomplete TrueType geometry"))
}

#[derive(Clone, Copy)]
struct ClearanceState {
    ctm: Matrix,
    width: f64,
    fill_white: bool,
    stroke_white: bool,
    join: i32,
    clip: ClearanceBox,
}
fn clearance_transform(matrix: Matrix, p: (f64, f64), height: f64) -> (f64, f64) {
    let (x, y) = matrix.point(p.0, p.1);
    (x, height - y)
}
fn clearance_envelope(points: &[(f64, f64)], pad: f64) -> anyhow::Result<ClearanceBox> {
    anyhow::ensure!(
        !points.is_empty() && pad.is_finite() && pad >= 0.0,
        "PageClearance: invalid path"
    );
    ClearanceBox::checked([
        points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min) - pad,
        points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min) - pad,
        points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max) + pad,
        points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max) + pad,
    ])
}
fn clearance_contents(
    bytes: &[u8],
    height: f64,
    width: f64,
    fonts: &BTreeMap<String, BTreeMap<u16, ClearanceFontGlyph>>,
    bold_fonts: &BTreeSet<String>,
) -> anyhow::Result<Vec<ClearanceComponent>> {
    let text = std::str::from_utf8(bytes)?;
    let mut state = ClearanceState {
        ctm: Matrix::IDENTITY,
        width: 1.0,
        fill_white: false,
        stroke_white: false,
        join: 0,
        clip: ClearanceBox::checked([0.0, 0.0, width, height])?,
    };
    let mut stack = Vec::new();
    let mut points = Vec::new();
    let mut subpath = None;
    let mut line = Matrix::IDENTITY;
    let mut tm = Matrix::IDENTITY;
    let mut font = String::new();
    let mut size = 0.0;
    let mut in_text = false;
    let mut out = Vec::new();
    let mut pending = Vec::<ClearanceComponent>::new();
    let mut group = String::new();
    for raw in text.lines() {
        let normalized = raw.replace('[', " [ ").replace(']', " ] ");
        let tokens = normalized.split_whitespace().collect::<Vec<_>>();
        for (i, &op) in tokens.iter().enumerate() {
            let number = |back: usize| -> anyhow::Result<f64> {
                let v: f64 = tokens
                    .get(
                        i.checked_sub(back)
                            .ok_or_else(|| anyhow::anyhow!("PageClearance: missing operand"))?,
                    )
                    .ok_or_else(|| anyhow::anyhow!("PageClearance: missing operand"))?
                    .parse()?;
                anyhow::ensure!(v.is_finite(), "PageClearance: nonfinite operand");
                Ok(v)
            };
            match op {
                "q" => stack.push(state),
                "Q" => {
                    state = stack
                        .pop()
                        .ok_or_else(|| anyhow::anyhow!("PageClearance: graphics stack"))?
                }
                "cm" => {
                    anyhow::ensure!(i >= 6, "PageClearance: matrix");
                    let m = matrix_from_tokens(&tokens[i - 6..i])
                        .ok_or_else(|| anyhow::anyhow!("PageClearance: matrix"))?;
                    anyhow::ensure!(
                        m.b == 0.0 && m.c == 0.0 && m.a != 0.0 && m.d != 0.0,
                        "PageClearance: rotated/sheared transform"
                    );
                    state.ctm = state.ctm.multiply(m);
                }
                "w" => {
                    state.width = number(1)?;
                    anyhow::ensure!(state.width >= 0.0, "PageClearance: line width");
                }
                "j" => state.join = number(1)? as i32,
                "J" => {
                    anyhow::ensure!(
                        [0.0, 1.0, 2.0].contains(&number(1)?),
                        "PageClearance: line cap"
                    );
                }
                "scn" | "SCN" | "rg" | "RG" => {
                    let rgb = [number(3)?, number(2)?, number(1)?];
                    anyhow::ensure!(
                        rgb == [0.0; 3] || rgb == [1.0; 3],
                        "PageClearance: colored/pattern ink"
                    );
                    if op == "scn" || op == "rg" {
                        state.fill_white = rgb == [1.0; 3];
                    } else {
                        state.stroke_white = rgb == [1.0; 3];
                    }
                }
                "cs" | "CS" => anyhow::ensure!(
                    i > 0 && tokens[i - 1] == "/CSp",
                    "PageClearance: color space"
                ),
                "m" => {
                    let p = clearance_transform(state.ctm, (number(2)?, number(1)?), height);
                    subpath = Some(p);
                    points.push(p);
                }
                "l" => points.push(clearance_transform(
                    state.ctm,
                    (number(2)?, number(1)?),
                    height,
                )),
                "c" => {
                    for back in [6, 4, 2] {
                        points.push(clearance_transform(
                            state.ctm,
                            (number(back)?, number(back - 1)?),
                            height,
                        ));
                    }
                }
                "h" => {
                    let start =
                        subpath.ok_or_else(|| anyhow::anyhow!("PageClearance: close path"))?;
                    if points.last().copied() != Some(start) {
                        points.push(start);
                    }
                }
                "W" | "W*" => {
                    anyhow::ensure!(
                        points.len() == 5
                            && points[0] == points[4]
                            && points
                                .windows(2)
                                .all(|p| p[0].0 == p[1].0 || p[0].1 == p[1].1),
                        "PageClearance: nonrectangular clipping"
                    );
                    let b = clearance_envelope(&points, 0.0)?;
                    state.clip = ClearanceBox::checked([
                        state.clip.0[0].max(b.0[0]),
                        state.clip.0[1].max(b.0[1]),
                        state.clip.0[2].min(b.0[2]),
                        state.clip.0[3].min(b.0[3]),
                    ])?;
                }
                "S" | "s" | "f" | "f*" | "F" | "B" | "B*" | "b" | "b*" => {
                    let stroke =
                        matches!(op, "S" | "s" | "B" | "B*" | "b" | "b*") && !state.stroke_white;
                    let fill = !matches!(op, "S" | "s") && !state.fill_white;
                    if stroke || fill {
                        anyhow::ensure!(
                            !stroke || state.join != 0 || points.len() <= 2,
                            "PageClearance: unsupported miter path"
                        );
                        let pad = if stroke {
                            state.width * state.ctm.a.abs().max(state.ctm.d.abs()) / 2.0
                        } else {
                            0.0
                        };
                        let bounds = clearance_envelope(&points, pad)?;
                        anyhow::ensure!(bounds.inside(state.clip), "PageClearance: clipped path");
                        out.push(ClearanceComponent {
                            bounds,
                            glyph: None,
                            text: None,
                            bold: false,
                        });
                    }
                    points.clear();
                    subpath = None;
                }
                "n" => {
                    points.clear();
                    subpath = None;
                }
                "BT" => {
                    anyhow::ensure!(!in_text, "PageClearance: nested text");
                    in_text = true;
                    line = Matrix::IDENTITY;
                    tm = line;
                    pending.clear();
                    group.clear();
                }
                "ET" => {
                    anyhow::ensure!(in_text, "PageClearance: unmatched text");
                    in_text = false;
                    for mut component in pending.drain(..) {
                        component.text = Some(group.clone());
                        out.push(component);
                    }
                }
                "Tf" => {
                    anyhow::ensure!(i >= 2 && in_text, "PageClearance: font selection");
                    font = tokens[i - 2].trim_start_matches('/').to_owned();
                    size = number(1)?;
                    anyhow::ensure!(size > 0.0, "PageClearance: font size");
                }
                "Tm" => {
                    anyhow::ensure!(i >= 6 && in_text, "PageClearance: text matrix");
                    line = matrix_from_tokens(&tokens[i - 6..i])
                        .ok_or_else(|| anyhow::anyhow!("PageClearance: text matrix"))?;
                    anyhow::ensure!(
                        line.b == 0.0 && line.c == 0.0,
                        "PageClearance: text rotation"
                    );
                    tm = line;
                }
                "Td" => {
                    anyhow::ensure!(in_text, "PageClearance: text move");
                    line = line.multiply(Matrix {
                        a: 1.0,
                        b: 0.0,
                        c: 0.0,
                        d: 1.0,
                        e: number(2)?,
                        f: number(1)?,
                    });
                    tm = line;
                }
                "Tj" => {
                    anyhow::ensure!(in_text && i > 0, "PageClearance: text show");
                    let hex = tokens[i - 1]
                        .strip_prefix('<')
                        .and_then(|s| s.strip_suffix('>'))
                        .ok_or_else(|| anyhow::anyhow!("PageClearance: text encoding"))?;
                    anyhow::ensure!(hex.len() % 4 == 0, "PageClearance: truncated glyph code");
                    let map = fonts
                        .get(&font)
                        .ok_or_else(|| anyhow::anyhow!("PageClearance: unknown font"))?;
                    for chunk in hex.as_bytes().chunks(4) {
                        let code = u16::from_str_radix(std::str::from_utf8(chunk)?, 16)?;
                        let glyph = map
                            .get(&code)
                            .ok_or_else(|| anyhow::anyhow!("PageClearance: missing glyph"))?;
                        anyhow::ensure!(
                            glyph.bounds.is_some() || glyph.character.is_whitespace(),
                            "PageClearance: empty visible glyph"
                        );
                        if let Some(b) = glyph.bounds {
                            let matrix = state.ctm.multiply(tm);
                            let pts = [
                                clearance_transform(matrix, (b[0] * size, b[1] * size), height),
                                clearance_transform(matrix, (b[2] * size, b[3] * size), height),
                            ];
                            let bounds = clearance_envelope(&pts, 0.0)?;
                            anyhow::ensure!(
                                !state.fill_white && bounds.inside(state.clip),
                                "PageClearance: invisible/clipped glyph"
                            );
                            pending.push(ClearanceComponent {
                                bounds,
                                glyph: Some(glyph.character),
                                text: None,
                                bold: bold_fonts.contains(&font),
                            });
                        }
                        group.push(glyph.character);
                        tm = tm.multiply(Matrix {
                            a: 1.0,
                            b: 0.0,
                            c: 0.0,
                            d: 1.0,
                            e: glyph.advance * size,
                            f: 0.0,
                        });
                    }
                }
                "gs" => anyhow::ensure!(
                    i > 0 && tokens[i - 1] == "/GSa",
                    "PageClearance: graphics profile"
                ),
                "M" => anyhow::ensure!(number(1)? >= 1.0, "PageClearance: miter limit"),
                "d" | "BDC" | "EMC" => {}
                _ if op.starts_with('/')
                    || op.starts_with('<')
                    || op.starts_with('[')
                    || op.ends_with(']')
                    || op == ">>"
                    || op.parse::<f64>().is_ok() => {}
                _ => anyhow::bail!("PageClearance: unsupported operator {op}"),
            }
        }
    }
    anyhow::ensure!(
        !in_text && stack.is_empty() && points.is_empty(),
        "PageClearance: incomplete content stream"
    );
    Ok(out)
}

fn clearance_number_after(dict: &str, key: &str) -> anyhow::Result<f64> {
    let tokens = dict.split_whitespace().collect::<Vec<_>>();
    let i = tokens
        .iter()
        .position(|s| *s == key)
        .ok_or_else(|| anyhow::anyhow!("PageClearance: missing {key}"))?;
    let value: f64 = tokens
        .get(i + 1)
        .ok_or_else(|| anyhow::anyhow!("PageClearance: missing number"))?
        .parse()?;
    anyhow::ensure!(value.is_finite(), "PageClearance: nonfinite number");
    Ok(value)
}
fn clearance_page_size(dict: &str) -> anyhow::Result<(f64, f64)> {
    let at = dict
        .find("/MediaBox")
        .ok_or_else(|| anyhow::anyhow!("PageClearance: page size"))?;
    let tail = &dict[at..];
    let start = tail
        .find('[')
        .ok_or_else(|| anyhow::anyhow!("PageClearance: page box"))?;
    let end = tail
        .find(']')
        .ok_or_else(|| anyhow::anyhow!("PageClearance: page box"))?;
    let values = tail[start + 1..end]
        .split_whitespace()
        .map(str::parse::<f64>)
        .collect::<Result<Vec<_>, _>>()?;
    anyhow::ensure!(
        values.len() == 4
            && values[0] == 0.0
            && values[1] == 0.0
            && values[2] > 0.0
            && values[3] > 0.0
            && values.iter().all(|x| x.is_finite()),
        "PageClearance: unsupported page box"
    );
    Ok((values[2], values[3]))
}
fn clearance_components(bytes: &[u8]) -> anyhow::Result<Vec<Vec<ClearanceComponent>>> {
    let objects = parse_pdf_objects(bytes)?;
    let pages = page_ids_in_order(&objects)?;
    let mut output = Vec::new();
    for id in pages {
        let page = objects
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("PageClearance: page object"))?
            .dictionary_text();
        let (width, height) = clearance_page_size(page)?;
        let resource = objects
            .get(
                &parse_single_ref_after(page, "/Resources")
                    .ok_or_else(|| anyhow::anyhow!("PageClearance: resources"))?,
            )
            .ok_or_else(|| anyhow::anyhow!("PageClearance: resource object"))?
            .dictionary_text();
        let words = resource.split_whitespace().collect::<Vec<_>>();
        anyhow::ensure!(
            words.windows(2).any(|w| w == ["/CSp", "/DeviceRGB"]),
            "PageClearance: RGB profile"
        );
        let gs = objects
            .get(
                &parse_single_ref_after(resource, "/GSa")
                    .ok_or_else(|| anyhow::anyhow!("PageClearance: graphics state"))?,
            )
            .ok_or_else(|| anyhow::anyhow!("PageClearance: graphics object"))?
            .dictionary_text();
        anyhow::ensure!(
            clearance_number_after(gs, "/ca")? == 1.0
                && clearance_number_after(gs, "/CA")? == 1.0
                && (!gs.contains("/SMask")
                    || gs
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .windows(2)
                        .any(|w| w[0] == "/SMask" && w[1].trim_end_matches('>') == "/None"))
                && !gs.contains("/BM"),
            "PageClearance: transparency/blending"
        );
        let mut fonts = BTreeMap::new();
        let mut bold = BTreeSet::new();
        for (name, font_id) in parse_font_refs(resource) {
            let font = objects
                .get(&font_id)
                .ok_or_else(|| anyhow::anyhow!("PageClearance: font object"))?;
            if font.dictionary_text().contains("Bold") {
                bold.insert(name.clone());
            }
            let metrics = clearance_font(font, &objects)
                .map_err(|e| anyhow::anyhow!("PageClearance: font {name}: {e}"))?;
            fonts.insert(name, metrics);
        }
        let content = objects
            .get(
                &parse_single_ref_after(page, "/Contents")
                    .ok_or_else(|| anyhow::anyhow!("PageClearance: content"))?,
            )
            .ok_or_else(|| anyhow::anyhow!("PageClearance: content object"))?
            .decoded_stream()?
            .ok_or_else(|| anyhow::anyhow!("PageClearance: content stream"))?;
        output.push(clearance_contents(&content, height, width, &fonts, &bold)?);
    }
    Ok(output)
}
fn clearance_find(parents: &mut [usize], mut i: usize) -> usize {
    while parents[i] != i {
        parents[i] = parents[parents[i]];
        i = parents[i];
    }
    i
}
fn clearance_row_assignment(
    components: &[ClearanceComponent],
    staff: &[ClearanceBox],
) -> anyhow::Result<Vec<usize>> {
    anyhow::ensure!(!staff.is_empty(), "PageClearance: no systems");
    let nearest = |b: ClearanceBox| -> anyhow::Result<usize> {
        let center = (b.0[1] + b.0[3]) / 2.0;
        let mut distances = staff
            .iter()
            .enumerate()
            .map(|(i, s)| (i, (center - (s.0[1] + s.0[3]) / 2.0).abs()))
            .collect::<Vec<_>>();
        distances.sort_by(|a, b| a.1.total_cmp(&b.1));
        anyhow::ensure!(
            distances.len() == 1 || distances[0].1 != distances[1].1,
            "PageClearance: ambiguous isolated component"
        );
        Ok(distances[0].0)
    };
    let mut parents = (0..components.len()).collect::<Vec<_>>();
    let mut order = (0..components.len()).collect::<Vec<_>>();
    order.sort_by(|&a, &b| components[a].bounds.0[1].total_cmp(&components[b].bounds.0[1]));
    let mut active = Vec::<usize>::new();
    for i in order {
        active.retain(|&j| components[j].bounds.0[3] >= components[i].bounds.0[1]);
        for &j in &active {
            if components[i].bounds.intersects(components[j].bounds) {
                let a = clearance_find(&mut parents, i);
                let b = clearance_find(&mut parents, j);
                parents[b] = a;
            }
        }
        active.push(i);
    }
    let mut anchors = BTreeMap::<usize, BTreeSet<usize>>::new();
    for (i, component) in components.iter().enumerate() {
        let key = clearance_find(&mut parents, i);
        let set = anchors.entry(key).or_default();
        if component.glyph.is_some() {
            set.insert(nearest(component.bounds)?);
        }
        // Staff strokes are independent physical anchors, not inferred from a
        // low beam's midpoint. Intersections propagate the whole connected beam.
        for (row, s) in staff.iter().enumerate() {
            let b = component.bounds.0;
            if component.glyph.is_none()
                && b[0] <= s.0[0]
                && b[2] >= s.0[2]
                && b[1] <= s.0[3]
                && b[3] >= s.0[1]
                && b[3] - b[1] <= s.0[3] - s.0[1]
            {
                set.insert(row);
            }
        }
    }
    components
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let key = clearance_find(&mut parents, i);
            let owners = &anchors[&key];
            anyhow::ensure!(
                owners.len() <= 1,
                "PageClearance: connected component crosses systems"
            );
            if let Some(&row) = owners.iter().next() {
                Ok(row)
            } else {
                nearest(c.bounds)
            }
        })
        .collect()
}

pub(crate) fn page_clearance_observation(
    xml: &str,
    pdf: &Path,
) -> anyhow::Result<PageClearanceObservation> {
    let document = Document::parse(xml)?;
    let parts = document
        .descendants()
        .filter(|n| n.has_tag_name("part"))
        .collect::<Vec<_>>();
    anyhow::ensure!(
        parts.len() == 1,
        "PageClearance: only bound single-part scores supported"
    );
    let measures = parts[0]
        .children()
        .filter(|n| n.has_tag_name("measure"))
        .collect::<Vec<_>>();
    let (measure_count, starts, page_starts) = planned_breaks_from_musicxml(xml)?;
    anyhow::ensure!(
        measure_count == measures.len(),
        "PageClearance: measure domain"
    );
    let bytes = std::fs::read(pdf)?;
    let observation = audit_paths_text(xml, &bytes)?;
    anyhow::ensure!(
        !observation.drifted(),
        "PageClearance: unconfirmed partition"
    );
    let pages = clearance_components(&bytes)?;
    anyhow::ensure!(
        pages.len() == page_starts.len(),
        "PageClearance: page domain"
    );
    let (segments, _) = extract_pdf_vector_geometry(&bytes)?;
    let cells = renderer_staff_cells(&segments, measure_count)?;
    let normalize = |s: &str| s.split_whitespace().collect::<String>();
    let titles = document
        .descendants()
        .filter(|n| n.has_tag_name("work-title") || n.has_tag_name("movement-title"))
        .filter_map(|n| n.text())
        .map(normalize)
        .collect::<BTreeSet<_>>();
    let names = document
        .descendants()
        .filter(|n| n.has_tag_name("part-name") || n.has_tag_name("part-abbreviation"))
        .filter_map(|n| n.text())
        .map(normalize)
        .collect::<BTreeSet<_>>();
    let mut result = PageClearanceObservation {
        systems: Vec::new(),
        gaps: Vec::new(),
        page_bottom: Vec::new(),
        page_top: Vec::new(),
        glyphs: Vec::new(),
    };
    for (pi, mut components) in pages.into_iter().enumerate() {
        let indices = starts
            .iter()
            .enumerate()
            .filter(|&(_, &start)| {
                start >= page_starts[pi]
                    && (pi + 1 == page_starts.len() || start < page_starts[pi + 1])
            })
            .collect::<Vec<_>>();
        let mut staff = Vec::new();
        for &(_, &start) in &indices {
            let pair = cells
                .iter()
                .filter(|c| c.measure == start)
                .collect::<Vec<_>>();
            anyhow::ensure!(
                pair.len() == 2 && pair.iter().all(|c| c.page_index == pi + 1),
                "PageClearance: staff/source binding"
            );
            staff.push(ClearanceBox::checked([
                pair[0].x_left,
                pair[0].line_y_top[0],
                pair[0].x_right,
                *pair[1]
                    .line_y_top
                    .last()
                    .ok_or_else(|| anyhow::anyhow!("PageClearance: staff lines"))?,
            ])?);
        }
        anyhow::ensure!(!staff.is_empty(), "PageClearance: empty page");
        components.retain(|c| {
            let Some(text) = c.text.as_deref() else {
                return true;
            };
            let text = normalize(text);
            let b = c.bounds.0;
            !(b[3] < staff[0].0[1]
                && (titles.contains(&text) || (c.bold && text == (pi + 1).to_string())))
                && !(names.contains(&text) && b[2] < staff[0].0[0])
        });
        let assignment = clearance_row_assignment(&components, &staff)?;
        let mut groups = vec![Vec::<ClearanceBox>::new(); staff.len()];
        let mut glyphs = vec![BTreeMap::<char, usize>::new(); staff.len()];
        for (component, &row) in components.iter().zip(&assignment) {
            groups[row].push(component.bounds);
            if let Some(ch) = component.glyph {
                *glyphs[row].entry(ch).or_default() += 1;
            }
        }
        let mut gaps = Vec::new();
        for row in 0..staff.len() {
            let (index, &start) = indices[row];
            let end = starts.get(index + 1).copied().unwrap_or(measure_count + 1) - 1;
            let expected = measures[start - 1..end]
                .iter()
                .flat_map(|m| m.children().filter(|n| n.has_tag_name("note")))
                .filter(|n| {
                    n.attribute("print-object") != Some("no")
                        && n.children().any(|c| c.has_tag_name("pitch"))
                })
                .count();
            let heads = glyphs[row]
                .iter()
                .filter(|(ch, _)| ('\u{e0a0}'..='\u{e0af}').contains(ch))
                .map(|(_, n)| n)
                .sum::<usize>();
            anyhow::ensure!(
                expected == heads,
                "PageClearance: notehead coverage {start}..{end}: {heads}/{expected}"
            );
            let top = groups[row]
                .iter()
                .map(|b| b.0[1])
                .fold(f64::INFINITY, f64::min);
            let bottom = groups[row]
                .iter()
                .map(|b| b.0[3])
                .fold(f64::NEG_INFINITY, f64::max);
            anyhow::ensure!(
                top.is_finite() && bottom.is_finite(),
                "PageClearance: missing system components"
            );
            result.systems.push(PageClearanceSystem {
                start,
                end,
                page: pi,
                ink_top: top,
                ink_bottom: bottom,
            });
            result.glyphs.push(glyphs[row].clone());
            if row + 1 < staff.len() {
                let gap = groups[row]
                    .iter()
                    .flat_map(|a| {
                        groups[row + 1]
                            .iter()
                            .filter(move |b| a.0[0].max(b.0[0]) < a.0[2].min(b.0[2]))
                            .map(move |b| b.0[1] - a.0[3])
                    })
                    .fold(f64::INFINITY, f64::min);
                anyhow::ensure!(gap.is_finite(), "PageClearance: missing pair coverage");
                gaps.push(gap);
            }
        }
        let scale = document
            .descendants()
            .find(|n| n.has_tag_name("scaling"))
            .ok_or_else(|| anyhow::anyhow!("PageClearance: scaling"))?;
        let value = |node: roxmltree::Node<'_, '_>, tag: &str| -> anyhow::Result<f64> {
            let v = node
                .children()
                .find(|n| n.has_tag_name(tag))
                .and_then(|n| n.text())
                .ok_or_else(|| anyhow::anyhow!("PageClearance: geometry {tag}"))?
                .parse::<f64>()?;
            anyhow::ensure!(v.is_finite() && v > 0.0, "PageClearance: geometry value");
            Ok(v)
        };
        let points_per_tenth = value(scale, "millimeters")? / value(scale, "tenths")? * 72.0 / 25.4;
        let layout = document
            .descendants()
            .find(|n| n.has_tag_name("page-layout"))
            .ok_or_else(|| anyhow::anyhow!("PageClearance: layout"))?;
        let margin = layout
            .children()
            .find(|n| n.has_tag_name("page-margins") && n.attribute("type") == Some("both"))
            .ok_or_else(|| anyhow::anyhow!("PageClearance: both margins"))?;
        result.page_bottom.push(
            (value(layout, "page-height")? - value(margin, "bottom-margin")?) * points_per_tenth,
        );
        result.page_top.push(
            result
                .systems
                .iter()
                .find(|s| s.page == pi)
                .ok_or_else(|| anyhow::anyhow!("PageClearance: first system"))?
                .ink_top,
        );
        result.gaps.push(gaps);
    }
    Ok(result)
}

#[cfg(test)]
#[path = "../../tests/unit/renderer_page_clearance.rs"]
mod renderer_page_clearance_tests;
