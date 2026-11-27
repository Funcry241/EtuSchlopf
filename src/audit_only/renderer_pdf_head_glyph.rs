//! Closed simple TrueType outlines from an embedded PDF font program.
//!
//! Coordinates are doubled font units so that implied points between two
//! consecutive off-curve points remain exact. Quadratic control points stay
//! distinct from the painted boundary. The parser grants no write authority.

use anyhow::{Context, Result, ensure};
use std::collections::BTreeMap;

const MAX_FONT_BYTES: usize = 4 * 1024 * 1024;
const MAX_TABLES: usize = 64;
const MAX_CONTOURS: usize = 256;
const MAX_POINTS: usize = 8192;

type Point = [i32; 2];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct GlyphCurve {
    pub(super) start: [i32; 2],
    pub(super) control: Option<[i32; 2]>,
    pub(super) end: [i32; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GlyphOutline {
    pub(super) units_per_em: u16,
    pub(super) contours: Vec<Vec<GlyphCurve>>,
}

impl GlyphOutline {
    /// A conservative outer rectangle from decoded curve control points.
    pub(super) fn hull(&self) -> [i32; 4] {
        let mut bounds = [i32::MAX, i32::MAX, i32::MIN, i32::MIN];
        for curve in self.contours.iter().flatten() {
            for point in [Some(curve.start), curve.control, Some(curve.end)]
                .into_iter()
                .flatten()
            {
                bounds[0] = bounds[0].min(point[0]);
                bounds[1] = bounds[1].min(point[1]);
                bounds[2] = bounds[2].max(point[0]);
                bounds[3] = bounds[3].max(point[1]);
            }
        }
        bounds
    }
}

pub(super) struct GlyphForeground {
    pub(super) units_per_em: u16,
    /// Conservative control-point bounds in doubled font units.
    pub(super) hull: [i32; 4],
}

struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    fn take<const N: usize>(&mut self) -> Result<[u8; N]> {
        let (field, rest) = self
            .0
            .split_first_chunk()
            .context("truncated TrueType field")?;
        self.0 = rest;
        Ok(*field)
    }

    fn byte(&mut self) -> Result<u8> {
        Ok(self.take::<1>()?[0])
    }

    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes(self.take()?))
    }

    fn i16(&mut self) -> Result<i16> {
        Ok(i16::from_be_bytes(self.take()?))
    }

    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.take()?))
    }
}

fn font_tables(bytes: &[u8]) -> Result<BTreeMap<[u8; 4], &[u8]>> {
    ensure!(
        bytes.len() <= MAX_FONT_BYTES,
        "TrueType font exceeds profile limit"
    );
    let mut directory = Reader(bytes);
    ensure!(
        directory.u32()? == 0x0001_0000,
        "unsupported TrueType flavor"
    );
    let count = usize::from(directory.u16()?);
    ensure!(
        (1..=MAX_TABLES).contains(&count),
        "unsupported TrueType table count"
    );
    directory.take::<6>()?;
    let directory_end = 12 + count * 16;
    let mut ranges = Vec::with_capacity(count);
    let mut tables = BTreeMap::new();
    for _ in 0..count {
        let tag = directory.take::<4>()?;
        let _checksum = directory.u32()?;
        let start = usize::try_from(directory.u32()?)?;
        let length = usize::try_from(directory.u32()?)?;
        let end = start
            .checked_add(length)
            .context("TrueType table range overflow")?;
        ensure!(
            start >= directory_end && start.is_multiple_of(4),
            "misplaced TrueType table"
        );
        let table = bytes
            .get(start..end)
            .context("invalid TrueType table range")?;
        ensure!(
            tables.insert(tag, table).is_none(),
            "duplicate TrueType table"
        );
        if start < end {
            ranges.push((start, end));
        }
    }
    ranges.sort_unstable();
    ensure!(
        ranges.windows(2).all(|pair| pair[0].1 <= pair[1].0),
        "overlapping TrueType tables"
    );
    for tag in [
        b"fvar", b"gvar", b"VARC", b"CFF ", b"CFF2", b"CBDT", b"CBLC", b"sbix", b"COLR", b"SVG ",
    ] {
        ensure!(
            !tables.contains_key(tag),
            "unsupported alternative glyph program"
        );
    }
    for tag in [b"fpgm", b"prep"] {
        ensure!(
            tables.get(tag).is_none_or(|program| program.is_empty()),
            "unsupported TrueType font hints"
        );
    }
    Ok(tables)
}

fn glyph_data<'a>(tables: &BTreeMap<[u8; 4], &'a [u8]>, gid: u16) -> Result<(u16, u16, &'a [u8])> {
    let head = *tables.get(b"head").context("missing TrueType head table")?;
    let maxp = *tables.get(b"maxp").context("missing TrueType maxp table")?;
    let loca = *tables.get(b"loca").context("missing TrueType loca table")?;
    let glyf = *tables.get(b"glyf").context("missing TrueType glyf table")?;
    ensure!(
        head.len() == 54 && maxp.len() == 32,
        "unsupported TrueType metrics header"
    );
    ensure!(
        head[..4] == [0, 1, 0, 0] && maxp[..4] == [0, 1, 0, 0],
        "unsupported TrueType header version"
    );
    ensure!(
        head[12..16] == [0x5f, 0x0f, 0x3c, 0xf5] && head[52..54] == [0, 0],
        "unsupported TrueType head data format"
    );
    let units_per_em = Reader(&head[18..]).u16()?;
    ensure!(
        (16..=16384).contains(&units_per_em),
        "invalid TrueType units per em"
    );
    let glyph_count = Reader(&maxp[4..]).u16()?;
    ensure!(gid != 0 && gid < glyph_count, "unbound TrueType glyph ID");
    let stride = match Reader(&head[50..]).i16()? {
        0 => 2,
        1 => 4,
        _ => anyhow::bail!("unsupported TrueType loca format"),
    };
    ensure!(
        loca.len() == (usize::from(glyph_count) + 1) * stride,
        "incomplete TrueType loca table"
    );
    let mut offsets = Reader(&loca[usize::from(gid) * stride..]);
    let mut read_offset = || -> Result<usize> {
        if stride == 2 {
            Ok(2 * usize::from(offsets.u16()?))
        } else {
            Ok(usize::try_from(offsets.u32()?)?)
        }
    };
    let start = read_offset()?;
    let end = read_offset()?;
    let data = glyf
        .get(start..end)
        .context("invalid TrueType glyph data range")?;
    Ok((units_per_em, glyph_count, data))
}

fn glyph_left_bearing(
    tables: &BTreeMap<[u8; 4], &[u8]>,
    glyph_count: u16,
    gid: u16,
) -> Result<i16> {
    let hhea = *tables.get(b"hhea").context("missing TrueType hhea table")?;
    let hmtx = *tables.get(b"hmtx").context("missing TrueType hmtx table")?;
    ensure!(
        hhea.len() == 36 && hhea[..4] == [0, 1, 0, 0] && hhea[32..34] == [0, 0],
        "unsupported TrueType horizontal metrics header"
    );
    let long_count = usize::from(Reader(&hhea[34..]).u16()?);
    let glyph_count = usize::from(glyph_count);
    let gid = usize::from(gid);
    ensure!(
        (1..=glyph_count).contains(&long_count)
            && hmtx.len() == 4 * long_count + 2 * (glyph_count - long_count),
        "incomplete TrueType horizontal metrics table"
    );
    let offset = if gid < long_count {
        4 * gid + 2
    } else {
        4 * long_count + 2 * (gid - long_count)
    };
    Reader(&hmtx[offset..]).i16()
}

#[derive(Clone, Copy)]
struct GlyphPoint {
    xy: Point,
    on_curve: bool,
}

fn closed_contour(points: &[GlyphPoint]) -> Result<Vec<GlyphCurve>> {
    let mut ring = Vec::with_capacity(2 * points.len() + 1);
    for (&point, &next) in points.iter().zip(points.iter().cycle().skip(1)) {
        ring.push(point);
        if !point.on_curve && !next.on_curve {
            ring.push(GlyphPoint {
                xy: [
                    (point.xy[0] + next.xy[0]) / 2,
                    (point.xy[1] + next.xy[1]) / 2,
                ],
                on_curve: true,
            });
        }
    }
    let first = ring
        .iter()
        .position(|point| point.on_curve)
        .context("empty TrueType contour")?;
    ring.rotate_left(first);
    ring.push(ring[0]);
    let mut remaining = &ring[1..];
    let mut start = ring[0].xy;
    let mut curves = Vec::with_capacity(points.len());
    while let Some((&next, rest)) = remaining.split_first() {
        let control = (!next.on_curve).then_some(next.xy);
        let end = if control.is_some() {
            let (&termination, tail) = rest.split_first().context("unclosed TrueType quadratic")?;
            ensure!(termination.on_curve, "ambiguous TrueType quadratic");
            remaining = tail;
            termination.xy
        } else {
            remaining = rest;
            next.xy
        };
        curves.push(GlyphCurve {
            start,
            control,
            end,
        });
        start = end;
    }
    Ok(curves)
}

fn coordinates(
    input: &mut Reader<'_>,
    flags: &[u8],
    short: u8,
    same_or_positive: u8,
) -> Result<Vec<i32>> {
    let mut current = 0_i32;
    flags
        .iter()
        .map(|flag| {
            let same_or_positive = flag & same_or_positive != 0;
            let delta = if flag & short != 0 {
                let short = i32::from(input.byte()?);
                if same_or_positive { short } else { -short }
            } else if same_or_positive {
                0
            } else {
                i32::from(input.i16()?)
            };
            current += delta;
            ensure!(
                i16::try_from(current).is_ok(),
                "arithmetic overflow in TrueType coordinates"
            );
            Ok(current * 2)
        })
        .collect()
}

fn expanded_flags(input: &mut Reader<'_>, count: usize) -> Result<Vec<u8>> {
    let mut flags = Vec::with_capacity(count);
    while flags.len() < count {
        let flag = input.byte()?;
        ensure!(
            flag & 0xc0 == 0,
            "unsupported overlapping or reserved TrueType flag"
        );
        let n = if flag & 0x08 != 0 {
            usize::from(input.byte()?) + 1
        } else {
            1
        };
        ensure!(n <= count - flags.len(), "overflowing TrueType flag repeat");
        flags.resize(flags.len() + n, flag);
    }
    Ok(flags)
}

pub(super) fn glyph_outline(bytes: &[u8], gid: u16) -> Result<GlyphOutline> {
    let tables = font_tables(bytes)?;
    let (units_per_em, _, bytes) = glyph_data(&tables, gid)?;
    decode_outline(units_per_em, bytes)?.context("missing TrueType glyph outline")
}

/// Encloses both the original control points and their horizontal bearing
/// alignment. Alternate placement uses the decoded horizontal metric, so no
/// font-specific margin is needed.
pub(super) fn foreground_hull_or_empty(bytes: &[u8], gid: u16) -> Result<Option<GlyphForeground>> {
    let tables = font_tables(bytes)?;
    let (units_per_em, glyph_count, bytes) = glyph_data(&tables, gid)?;
    let bearing = 2 * i32::from(glyph_left_bearing(&tables, glyph_count, gid)?);
    Ok(decode_outline(units_per_em, bytes)?.map(|outline| {
        let [left, bottom, right, top] = outline.hull();
        let shift = bearing - left;
        GlyphForeground {
            units_per_em,
            hull: [left + shift.min(0), bottom, right + shift.max(0), top],
        }
    }))
}

/// Decodes the used glyph, treating a zero-length `loca` interval as no ink.
/// A nonempty but unsupported glyph program is an error even for text whose
/// extraction metadata is empty.
fn decode_outline(units_per_em: u16, bytes: &[u8]) -> Result<Option<GlyphOutline>> {
    if bytes.is_empty() {
        return Ok(None);
    }
    let mut input = Reader(bytes);
    let contour_count =
        usize::try_from(input.i16()?).context("unsupported TrueType composite glyph")?;
    ensure!(
        (1..=MAX_CONTOURS).contains(&contour_count),
        "unsupported TrueType contour count"
    );
    let declared_bounds =
        [input.i16()?, input.i16()?, input.i16()?, input.i16()?].map(|coord| i32::from(coord) * 2);
    let mut ends = Vec::with_capacity(contour_count);
    let mut start = 0;
    for _ in 0..contour_count {
        let end = usize::from(input.u16()?) + 1;
        ensure!(
            end >= start + 2 && end <= MAX_POINTS,
            "invalid TrueType contour endpoint"
        );
        ends.push(end);
        start = end;
    }
    ensure!(input.u16()? == 0, "unsupported TrueType glyph hints");
    let flags = expanded_flags(&mut input, start)?;
    let x = coordinates(&mut input, &flags, 0x02, 0x10)?;
    let y = coordinates(&mut input, &flags, 0x04, 0x20)?;
    ensure!(
        input.0.len() <= 3 && input.0.iter().all(|&byte| byte == 0),
        "unknown TrueType glyph trailer"
    );
    let points: Vec<_> = x
        .into_iter()
        .zip(y)
        .zip(flags)
        .map(|((x, y), flag)| GlyphPoint {
            xy: [x, y],
            on_curve: flag & 1 != 0,
        })
        .collect();
    let mut contours = Vec::with_capacity(contour_count);
    start = 0;
    for end in ends {
        contours.push(closed_contour(&points[start..end])?);
        start = end;
    }
    let outline = GlyphOutline {
        units_per_em,
        contours,
    };
    let bounds = outline.hull();
    let [left, bottom, right, top] = bounds;
    ensure!(
        left < right && bottom < top,
        "degenerate TrueType glyph outline"
    );
    ensure!(
        bounds == declared_bounds,
        "TrueType glyph metadata does not match its decoded control points"
    );
    Ok(Some(outline))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn font(glyph: Vec<u8>, short_loca: bool) -> Vec<u8> {
        font_with_bearing(glyph, short_loca, 0, 2)
    }

    fn font_with_bearing(
        glyph: Vec<u8>,
        short_loca: bool,
        bearing: i16,
        long_count: u16,
    ) -> Vec<u8> {
        let mut head = vec![0; 54];
        head[..4].copy_from_slice(&[0, 1, 0, 0]);
        head[12..16].copy_from_slice(&[0x5f, 0x0f, 0x3c, 0xf5]);
        head[18..20].copy_from_slice(&2048_u16.to_be_bytes());
        head[51] = u8::from(!short_loca);
        let mut maxp = vec![0; 32];
        maxp[..6].copy_from_slice(&[0, 1, 0, 0, 0, 2]);
        let mut hhea = vec![0; 36];
        hhea[..4].copy_from_slice(&[0, 1, 0, 0]);
        hhea[34..36].copy_from_slice(&long_count.to_be_bytes());
        let mut hmtx = vec![0; 2 * (2 + usize::from(long_count))];
        let bearing_offset = hmtx.len() - 2;
        hmtx[bearing_offset..].copy_from_slice(&bearing.to_be_bytes());
        let loca = if short_loca {
            [0_u16, 0, (glyph.len() / 2).try_into().unwrap()]
                .into_iter()
                .flat_map(u16::to_be_bytes)
                .collect()
        } else {
            [0_u32, 0, glyph.len().try_into().unwrap()]
                .into_iter()
                .flat_map(u32::to_be_bytes)
                .collect()
        };
        let tables = [
            (b"glyf", glyph),
            (b"head", head),
            (b"hhea", hhea),
            (b"hmtx", hmtx),
            (b"loca", loca),
            (b"maxp", maxp),
        ];
        let mut bytes = vec![0; 12 + 16 * tables.len()];
        bytes[..4].copy_from_slice(&[0, 1, 0, 0]);
        bytes[4..6].copy_from_slice(&u16::try_from(tables.len()).unwrap().to_be_bytes());
        for (index, (name, data)) in tables.into_iter().enumerate() {
            bytes.resize(bytes.len().next_multiple_of(4), 0);
            let record = 12 + 16 * index;
            let start = u32::try_from(bytes.len()).unwrap();
            bytes[record..record + 4].copy_from_slice(name);
            bytes[record + 8..record + 12].copy_from_slice(&start.to_be_bytes());
            bytes[record + 12..record + 16]
                .copy_from_slice(&u32::try_from(data.len()).unwrap().to_be_bytes());
            bytes.extend_from_slice(&data);
        }
        bytes
    }

    fn fixture(first: [i16; 2]) -> Vec<u8> {
        let [x, y] = first;
        let mut result = [1_i16, 0, 0, 100, 100, 3, 0]
            .into_iter()
            .flat_map(i16::to_be_bytes)
            .collect::<Vec<_>>();
        result.extend([0x08, 3]);
        result.extend(
            [x, 100 - x, 0, -100, y, -y, 100, 0]
                .into_iter()
                .flat_map(i16::to_be_bytes),
        );
        result
    }

    #[test]
    fn uses_original_repeat_coordinates_and_closes_implied_quadratics() {
        for short_loca in [false, true] {
            let outline = glyph_outline(&font(fixture([0, 0]), short_loca), 1).unwrap();
            assert_eq!(outline.units_per_em, 2048);
            assert_eq!(outline.hull(), [0, 0, 200, 200]);
            let [curves] = outline.contours.as_slice() else {
                panic!("one contour expected")
            };
            assert_eq!(curves.len(), 4);
            assert_eq!(
                curves[0],
                GlyphCurve {
                    start: [100, 0],
                    control: Some([200, 0]),
                    end: [200, 100],
                }
            );
            assert_eq!(curves.last().unwrap().end, curves[0].start);
        }
    }

    #[test]
    fn equal_header_bounds_do_not_hide_different_control_points() {
        let a = glyph_outline(&font(fixture([0, 0]), true), 1).unwrap();
        let b = glyph_outline(&font(fixture([10, 10]), true), 1).unwrap();
        assert_eq!(a.hull(), b.hull());
        assert_ne!(a.contours, b.contours);
    }

    #[test]
    fn rejects_hints_composite_wrong_repeat_and_misleading_header() {
        for (offset, value) in [(13, 1), (0, 0xff), (15, 4), (7, 101)] {
            let mut bytes = fixture([0, 0]);
            bytes[offset] = value;
            assert!(glyph_outline(&font(bytes, true), 1).is_err());
        }
    }

    #[test]
    fn only_a_zero_length_glyph_range_certifies_no_ink() {
        for short_loca in [false, true] {
            let empty = font(Vec::new(), short_loca);
            assert!(foreground_hull_or_empty(&empty, 1).unwrap().is_none());
            assert!(glyph_outline(&empty, 1).is_err());
            assert!(foreground_hull_or_empty(&empty, 0).is_err());
            let incomplete = font(vec![0; 12], short_loca);
            assert!(foreground_hull_or_empty(&incomplete, 1).is_err());
        }
    }

    #[test]
    fn foreground_encloses_raw_and_bearing_aligned_outline_in_both_directions() {
        for (bearing, long_count, expected) in
            [(-35, 1, [-70, 0, 200, 200]), (35, 2, [0, 0, 270, 200])]
        {
            let bytes = font_with_bearing(fixture([0, 0]), true, bearing, long_count);
            let foreground = foreground_hull_or_empty(&bytes, 1).unwrap().unwrap();
            assert_eq!(foreground.units_per_em, 2048);
            assert_eq!(foreground.hull, expected);
            assert_eq!(glyph_outline(&bytes, 1).unwrap().hull(), [0, 0, 200, 200]);
        }
        let bad = font_with_bearing(fixture([0, 0]), true, 35, 3);
        assert!(foreground_hull_or_empty(&bad, 1).is_err());
    }
}
