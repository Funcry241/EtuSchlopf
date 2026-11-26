#!/usr/bin/env python3
"""Check actual plain/marked PDF noteheads for one bounded synthetic export."""
import argparse
from collections import Counter
from copy import deepcopy
from dataclasses import dataclass, replace
from decimal import Decimal as D
import io
import json
from math import hypot
from pathlib import Path
import re
from xml.etree import ElementTree as ET

from fontTools.pens.basePen import BasePen
from fontTools.ttLib import TTFont
from pypdf import PdfReader, PdfWriter
from pypdf.generic import ByteStringObject, DecodedStreamObject, FloatObject, NameObject, NumberObject

from check import NS, NUM, Unknown, changed_pdf, intersect, matrix, number, only_page
from check import pdf_paths, rectangle, require, transform

COLOR = re.compile(r"#[0-9a-f]{6}")
SVG_TOKEN = re.compile(rf"(?P<operator>[MC])|(?P<number>{NUM.pattern})")
FLATNESS = 0.0005
SAMPLE_STEP = 0.025
CONTOUR_MARGIN = 0.05  # pt, bounded test profile, not a score clearance minimum
ARITHMETIC_MARGIN = 0.000001
IDENTITY = (D(1), D(1), D(0), D(0))


def composed(outer, inner):
    oa, od, oe, of = outer
    ia, id_, ie, if_ = inner
    return oa * ia, od * id_, oa * ie + oe, od * if_ + of


def point_in(matrix_value, point):
    a, d, e, f = matrix_value
    x, y = point
    return a * x + e, d * y + f


def blend(left, right, position):
    return tuple((1 - position) * D(a) + position * D(b) for a, b in zip(left, right))


def xform_segments(contours, function):
    return tuple(tuple(tuple(function(p) for p in s) for s in c) for c in contours)


class CubicPen(BasePen):
    """Trusted outline input is the decoded simple TrueType point sequence."""

    def __init__(self):
        super().__init__(None)
        self.contours = []
        self.segments = []
        self.start = None

    def _moveTo(self, p):
        require(not self.segments and self.start is None, "unclosed TrueType contour")
        self.start = p

    def _lineTo(self, p):
        start = self._getCurrentPoint()
        self._curveToOne(blend(start, p, D(1) / 3), blend(start, p, D(2) / 3), p)

    def _qCurveToOne(self, control, stop):
        start = self._getCurrentPoint()
        ratio = D(2) / 3
        self._curveToOne(blend(start, control, ratio), blend(stop, control, ratio), stop)

    def _curveToOne(self, a, b, stop):
        self.segments.append((self._getCurrentPoint(), a, b, stop))

    def _closePath(self):
        if self._getCurrentPoint() != self.start:
            self._lineTo(self.start)
        require(self.segments, "empty TrueType contour")
        self.contours.append(tuple(self.segments))
        self.segments = []
        self.start = None

    def _endPath(self):
        raise Unknown("open TrueType path")


def cid_font(font):
    require(font["/Type"] == "/Font" and font["/Subtype"] == "/Type0", "not a Type0 font")
    require(font["/Encoding"] == "/Identity-H", "unsupported text code mapping")
    [descendant] = font["/DescendantFonts"]
    cid = descendant.get_object()
    require(cid["/Subtype"] == "/CIDFontType2", "not a TrueType CID font")
    require(cid.get("/CIDToGIDMap") == "/Identity", "unproved CID-to-GID mapping")
    desc = cid["/FontDescriptor"]
    require("/FontFile2" in desc and "/FontFile" not in desc and "/FontFile3" not in desc, "embedded TrueType required")
    return cid


def font_payload(font):
    cid = cid_font(font)
    return (
        str(font["/BaseFont"]),
        str(cid["/BaseFont"]),
        str(cid["/W"]),
        str(cid.get("/DW", 1000)),
        font["/ToUnicode"].get_data(),
        cid["/FontDescriptor"]["/FontFile2"].get_data(),
    )


def ttf_glyph(font, gid):
    glyph_order = font.getGlyphOrder()
    require(0 < gid < len(glyph_order), "unbound TrueType glyph index")
    glyph = font["glyf"][glyph_order[gid]]
    require(glyph.numberOfContours > 0, "empty or composite notehead glyph")
    require(not glyph.program.getBytecode(), "unsupported hinted notehead glyph")
    pen = CubicPen()
    glyph.draw(pen, font["glyf"])
    require(len(pen.contours) == glyph.numberOfContours, "invalid TrueType contours")
    return tuple(pen.contours)


def show_bytes(args):
    [raw] = args
    return raw.original_bytes


def rgb(args):
    colors = tuple(map(number, args))
    require(len(colors) == 3, "not an RGB tuple")
    require(all(0 <= value <= 1 for value in colors), "invalid RGB range")
    quantized = [round(c * 255) for c in colors]
    require(
        all(abs(c - D(value) / 255) < D("0.0000001") for c, value in zip(colors, quantized)),
        "unproved exact source color",
    )
    return "#" + "".join(f"{x:02x}" for x in quantized)


@dataclass(frozen=True)
class Graphics:
    clip: tuple
    ctm: tuple = IDENTITY
    fill: str = "#000000"


@dataclass(frozen=True)
class TextInk:
    key: str
    gid: int
    matrix: tuple
    clip: tuple
    fill: str


def text_ink(operations, media):
    graphics = Graphics(media)
    stack = []
    path = []
    current_font = line = text_matrix = None
    count = 0
    in_text = False
    result = {}
    for index, (args, op) in enumerate(operations):
        if op == b"q":
            stack.append(graphics)
        elif op == b"Q":
            graphics = stack.pop()
        elif op == b"cm":
            graphics = replace(graphics, ctm=composed(graphics.ctm, matrix(args)))
        elif op == b"scn":
            graphics = replace(graphics, fill=rgb(args))
        elif op in {b"m", b"l", b"c", b"h"}:
            points = tuple(transform(graphics.ctm, p, media[3]) for p in zip(map(number, args[::2]), map(number, args[1::2])))
            path.append((op, points))
        elif op == b"W*":
            graphics = replace(graphics, clip=intersect(graphics.clip, rectangle(path)))
        elif op in {b"S", b"B*", b"f*", b"n"}:
            path.clear()
        elif op == b"BT":
            current_font = None
            line = text_matrix = IDENTITY
            count = 0
            in_text = True
        elif op == b"ET":
            in_text = False
        elif op == b"Tf":
            key, size = args
            current_font = str(key), number(size)
        elif op == b"Tm":
            line = text_matrix = matrix(args)
            count = 0
        elif op == b"Td":
            x, y = map(number, args)
            line = text_matrix = composed(line, (D(1), D(1), x, y))
            count = 0
        elif op == b"Tj":
            require(in_text and count == 0 and current_font and text_matrix, "unsupported text advance")
            raw = show_bytes(args)
            require(len(raw) == 2, "only one two-byte CID per show is supported")
            key, size = current_font
            require(size > 0, "nonpositive font size")
            rendering = composed(composed(graphics.ctm, text_matrix), (size, size, D(0), D(0)))
            result[index] = TextInk(key, int.from_bytes(raw, "big"), rendering, graphics.clip, graphics.fill)
            count += 1
    return result


def default_appearance(page):
    require(not page.get("/OC") and not page.get("/AA"), "unsupported page state")
    resources = page["/Resources"]
    require(set(resources) == {"/ColorSpace", "/ExtGState", "/Pattern", "/Font", "/XObject"}, "unsupported resource dictionary")
    require(not resources["/Pattern"] and not resources["/XObject"], "unsupported paint resource")
    require(resources["/ColorSpace"]["/CSp"] == "/DeviceRGB", "unsupported color resource")
    return {
        str(key): dict(value.get_object()) for key, value in resources["/ExtGState"].items()
    }


def svg_head_paths(path):
    root = ET.parse(path).getroot()
    require(root.tag == NS + "svg" and set(root.attrib) <= {"width", "height", "viewBox", "version", "baseProfile"}, "unsupported SVG viewport")
    x, y, w, h = map(number, root.attrib["viewBox"].split())
    require(x == y == 0 and w > 0 and h > 0, "unsupported SVG origin or size")
    require(all(root.attrib[k].endswith("mm") for k in ("width", "height")), "missing SVG physical width")
    sx, sy = (number(root.attrib[k][:-2]) * 72 / D("25.4") / span for k, span in zip(("width", "height"), (w, h)))
    require(abs(sx - sy) < D("0.0000001") and sx > 0, "unsupported SVG anisotropy")
    all_paths = []
    for node in root.iter():
        if node.get("class") != "Note":
            continue
        require(node.tag == NS + "path" and node in list(root) and set(node.attrib) <= {"class", "d", "fill"}, "unsupported SVG note style")
        all_paths.append((node.get("fill", "#000000").lower(), parse_svg_path(node.attrib["d"], sx, sy)))
    return all_paths


def parse_svg_path(data, sx, sy):
    require(not SVG_TOKEN.sub("", data).replace(",", "").strip(), "unsupported SVG path grammar")
    tokens = [m.group() for m in SVG_TOKEN.finditer(data)]
    require(tokens and tokens[0] == "M", "SVG contour must start with move")
    result = []
    segments = []
    index = 0
    current = start = None
    while index < len(tokens):
        op = tokens[index]
        index += 1
        size = 2 if op == "M" else 6
        args = [number(x) for x in tokens[index:index + size]]
        require(len(args) == size, "unsupported SVG path arity")
        index += size
        points = tuple((x * sx, y * sy) for x, y in zip(args[::2], args[1::2]))
        if op == "M":
            if segments:
                require(current == start, "open SVG contour")
                result.append(tuple(segments))
            segments = []
            start = current = points[0]
        else:
            require(op == "C" and current is not None, "unsupported SVG note segment")
            segments.append((current, *points))
            current = points[-1]
    require(segments and current == start, "open or empty SVG final contour")
    result.append(tuple(segments))
    return tuple(result)


def lerp(a, b, t):
    return a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])


def flatten(a, b, c, d, points, depth=0):
    chord_a = lerp(a, d, 1 / 3)
    chord_b = lerp(a, d, 2 / 3)
    if max(hypot(b[0] - chord_a[0], b[1] - chord_a[1]), hypot(c[0] - chord_b[0], c[1] - chord_b[1])) < FLATNESS:
        points.append(d)
        return
    require(depth < 15, "curve flatness unresolved")
    ab, bc, cd = lerp(a, b, 0.5), lerp(b, c, 0.5), lerp(c, d, 0.5)
    abc, bcd = lerp(ab, bc, 0.5), lerp(bc, cd, 0.5)
    middle = lerp(abc, bcd, 0.5)
    flatten(a, ab, abc, middle, points, depth + 1)
    flatten(middle, bcd, cd, d, points, depth + 1)


def polygon(contour):
    floats = tuple(tuple(tuple(map(float, p)) for p in segment) for segment in contour)
    require(all(abs(v) < 2000 for segment in floats for p in segment for v in p), "contour beyond numeric profile")
    points = [floats[0][0]]
    for segment in floats:
        require(points[-1] == segment[0], "disconnected segment")
        flatten(*segment, points)
    require(points[0] == points[-1], "unclosed polygon")
    return points


def orientation(points):
    origin = points[0]
    pairs = ((a[0] - origin[0], a[1] - origin[1], b[0] - origin[0], b[1] - origin[1]) for a, b in zip(points, points[1:]))
    area_twice = sum(ax * by - ay * bx for ax, ay, bx, by in pairs)
    require(abs(area_twice) > 0.1, "contour winding unresolved")
    return area_twice > 0


def sample_points(points):
    for a, b in zip(points, points[1:]):
        dist = hypot(a[0] - b[0], a[1] - b[1])
        if dist == 0:
            continue
        steps = int(dist / SAMPLE_STEP) + 1
        for m in range(steps):
            yield lerp(a, b, m / steps)
    yield points[-1]


def edges(points):
    return [
        (a[0], a[1], b[0] - a[0], b[1] - a[1], (b[0] - a[0]) ** 2 + (b[1] - a[1]) ** 2)
        for a, b in zip(points, points[1:]) if a != b
    ]


def distance_sq(p, edge):
    x, y, dx, dy, length_sq = edge
    px, py = p[0] - x, p[1] - y
    fraction = (px * dx + py * dy) / length_sq
    t = max(0, min(1, fraction))
    return (px - t * dx) ** 2 + (py - t * dy) ** 2


def contour_error(first, second):
    if orientation(first) != orientation(second):
        return None
    first_edges, second_edges = edges(first), edges(second)
    require(first_edges and second_edges, "no contour edges")
    measures = (
        max(min(distance_sq(p, edge) for edge in second_edges) for p in sample_points(first)),
        max(min(distance_sq(p, edge) for edge in first_edges) for p in sample_points(second)),
    )
    # The cubic-to-chord error is bounded by the Bézier control-point hull of
    # their difference. The point-to-polyline metric is 1-Lipschitz, so an
    # arbitrary point is at most half a sample step further from the other path.
    return max(measures) ** 0.5 + SAMPLE_STEP / 2 + 2 * FLATNESS + ARITHMETIC_MARGIN


def prove_contours(pdf, svg):
    require(len(pdf) == len(svg), "embedded glyph contour count differs from painted SVG")
    pdf, svg = [polygon(c) for c in pdf], [polygon(c) for c in svg]
    unmatched = set(range(len(svg)))
    maximum = 0.0
    for contour in pdf:
        possible = [(i, contour_error(contour, svg[i])) for i in unmatched]
        candidates = [(i, error) for i, error in possible if error is not None and error <= CONTOUR_MARGIN]
        require(len(candidates) == 1, "embedded glyph contour coordinates differ or are ambiguous")
        i, error = candidates[0]
        unmatched.remove(i)
        maximum = max(maximum, error)
    return maximum


def source_signature(node):
    allowed = {"color", "id"} if node.tag in {"note", "stem", "tied"} else set()
    attributes = {key: value for key, value in node.attrib.items() if key not in allowed}
    return node.tag, attributes, (node.text or "").strip(), [source_signature(c) for c in node]


def source_marks(plain_xml, marked_xml):
    plain = ET.parse(plain_xml).getroot()
    marked = ET.parse(marked_xml).getroot()
    require(source_signature(plain) == source_signature(marked), "marked source has changed notation")
    marks = [n.get("color", "").lower() for n in marked.iter("note") if n.find("pitch") is not None]
    require(marks and all(COLOR.fullmatch(m) and m != "#000000" for m in marks), "missing source note mark")
    require(len(set(marks)) == len(marks), "duplicate source note mark")
    return set(marks)


def frozen(args, op):
    if op == b"Tj":
        return show_bytes(args)
    if op in {b"scn", b"SCN"}:
        rgb(args)
        return None
    if op == b"d":
        return tuple(map(number, args[0])), number(args[1])
    return tuple(str(value) for value in args)


def qualify(plain_reader, marked_reader, svg, expected):
    require(not any(reader.root_object.get("/OCProperties") for reader in (plain_reader, marked_reader)), "optional PDF layers unsupported")
    plain, media = only_page(plain_reader)
    marked, marked_media = only_page(marked_reader)
    require(media == marked_media and default_appearance(plain) == default_appearance(marked), "page appearance profile differs")
    pdf_paths(plain_reader)
    fonts_plain = plain["/Resources"]["/Font"]
    fonts_marked = marked["/Resources"]["/Font"]
    require(set(fonts_plain) == set(fonts_marked), "font resources differ")
    font_data = {str(k): font_payload(fonts_plain[k]) for k in fonts_plain}
    require(font_data == {str(k): font_payload(fonts_marked[k]) for k in fonts_marked}, "font resources or embedded font bytes differ")
    plain_ops = plain.get_contents().operations
    marked_ops = marked.get_contents().operations
    require(len(plain_ops) == len(marked_ops), "PDF operation counts differ")
    for (old, op), (new, marked_op) in zip(plain_ops, marked_ops):
        require(op == marked_op and frozen(old, op) == frozen(new, op), "PDF non-color operation changed")
    require(Counter(c for c, _ in svg) == Counter(expected), "missing or duplicate SVG notehead anchor")
    source_svg = dict(svg)
    plain_ink = text_ink(plain_ops, media)
    marked_ink = text_ink(marked_ops, media)
    source_pdf = [(i, ink) for i, ink in marked_ink.items() if ink.fill in expected]
    require(Counter(ink.fill for _, ink in source_pdf) == Counter(expected), "missing or duplicate PDF notehead anchor")
    pdf_matches = {}
    opened = {}
    for index, ink in source_pdf:
        require(plain_ink[index] == replace(ink, fill="#000000"), "marked PDF note changed actual ink")
        basefont, cidfont, _, _, _, font_bytes = font_data[ink.key]
        require(basefont == cidfont == "/Leland", "unqualified note font")
        font = opened.get(ink.key)
        if font is None:
            font = opened[ink.key] = TTFont(io.BytesIO(font_bytes))
            require(set(font.keys()) == {"GlyphOrder", "head", "hhea", "maxp", "OS/2", "hmtx", "loca", "glyf", "name"}, "unsupported embedded font profile")
        units = D(font["head"].unitsPerEm)
        require(units > 0, "invalid units per em")
        outline = xform_segments(
            ttf_glyph(font, ink.gid),
            lambda p: transform(ink.matrix, (D(p[0]) / units, D(p[1]) / units), media[3]),
        )
        l, t, r, b = ink.clip
        require(
            all(l + D(str(CONTOUR_MARGIN)) <= x <= r - D(str(CONTOUR_MARGIN)) and t + D(str(CONTOUR_MARGIN)) <= y <= b - D(str(CONTOUR_MARGIN)) for contour in outline for segment in contour for x, y in segment),
            "notehead clipping unresolved",
        )
        error = prove_contours(outline, source_svg[ink.fill])
        coordinates = [p for contour in outline for segment in contour for p in segment]
        xs, ys = zip(*coordinates)
        box = [min(xs), min(ys), max(xs), max(ys)]
        pdf_matches[ink.fill] = {"cid_gid": ink.gid, "contours": len(outline), "error_pt": str(error), "outer_control_hull_pt": list(map(str, box))}
    return pdf_matches


def fixture(directory):
    return (
        directory / "plain.pdf",
        directory / "marked.pdf",
        svg_head_paths(directory / "marked-1.svg"),
        source_marks(directory / "plain.musicxml", directory / "marked.musicxml"),
    )


def verify(directory, negative_probes):
    plain, marked, svg, expected = fixture(directory)
    match = qualify(PdfReader(plain, strict=True), PdfReader(marked, strict=True), svg, expected)
    result = {
        "verified_noteheads": len(match),
        "max_contour_distance_upper_bound_pt": str(max(D(v["error_pt"]) for v in match.values())),
        "noteheads": match,
    }
    if negative_probes:
        result["negative_probes"] = negatives(plain, marked, svg, expected)
    return result


def font_change(path):
    writer = PdfWriter(clone_from=PdfReader(path, strict=True))
    for resource in writer.pages[0]["/Resources"]["/Font"].values():
        font = resource.get_object()
        if font["/BaseFont"] != "/Leland":
            continue
        stream = cid_font(font)["/FontDescriptor"]["/FontFile2"]
        ttf = TTFont(io.BytesIO(stream.get_data()), recalcTimestamp=False)
        hollow, filled = ttf.getGlyphOrder()[1:3]
        require(ttf["hmtx"][hollow] == ttf["hmtx"][filled], "fixture glyph metrics are unequal")
        ttf["glyf"][hollow] = deepcopy(ttf["glyf"][filled])
        output = io.BytesIO()
        ttf.save(output)
        replaced = DecodedStreamObject()
        replaced.set_data(output.getvalue())
        cid_font(font)["/FontDescriptor"][NameObject("/FontFile2")] = writer._add_object(replaced)
        break
    else:
        raise Unknown("mutation needs renderer's Leland font")
    file = io.BytesIO()
    writer.write(file)
    return PdfReader(file, strict=True)


def negatives(plain, marked, svg, expected):
    result = {}

    def refuse(name, first, second, required_reason):
        try:
            qualify(first, second, svg, expected)
        except Unknown as e:
            require(str(e) == required_reason, f"unexpected {name} rejection: {e}")
            result[name] = str(e)
        else:
            raise AssertionError(f"{name} was accepted")

    refuse("embedded_wrong_glyph_equal_width_same_unicode", font_change(plain), font_change(marked), "embedded glyph contour count differs from painted SVG")

    def first_hollow(ops):
        return next(i for i, (args, op) in enumerate(ops) if i > 100 and op == b"Tj" and show_bytes(args) == b"\x00\x01")

    def wrong_code(ops):
        index = first_hollow(ops)
        ops[index] = ([ByteStringObject(b"\x00\x02")], b"Tj")

    refuse("equal_width_wrong_CID", changed_pdf(plain, wrong_code), changed_pdf(marked, wrong_code), "embedded glyph contour count differs from painted SVG")

    def relocated(ops):
        index = first_hollow(ops)
        args, op = ops[index - 1]
        require(op == b"Td", "relocated fixture needs an explicit text origin")
        args[0] = FloatObject("10")

    refuse("changed_notehead_transform", changed_pdf(plain, relocated), changed_pdf(marked, relocated), "embedded glyph contour coordinates differ or are ambiguous")

    def clipped(ops):
        index = first_hollow(ops)
        require(ops[index - 4][1] == b"BT" and ops[index + 1][1] == b"ET", "unknown synthetic text scope")
        add = lambda op, *coords: ([NumberObject(n) for n in coords], op)
        ops.insert(index + 2, add(b"Q"))
        ops[index - 4:index - 4] = [
            add(b"q"), add(b"m", -100, -100), add(b"l", 52, -100),
            add(b"l", 52, 100), add(b"l", -100, 100), add(b"h"), add(b"W*"), add(b"n"),
        ]

    refuse("clipped_notehead", changed_pdf(plain, clipped), changed_pdf(marked, clipped), "notehead clipping unresolved")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("--negative-probes", action="store_true")
    args = parser.parse_args()
    try:
        print(json.dumps(verify(args.fixture, args.negative_probes), indent=2))
    except (Unknown, ValueError, KeyError, TypeError, IndexError) as error:
        parser.exit(2, f"unknown: {error}\n")


if __name__ == "__main__":
    main()
