#!/usr/bin/env python3
"""Black, closed, two-cubic MuseScore tie geometry in one synthetic PDF page."""
from dataclasses import dataclass, replace
from decimal import Decimal as D
from pathlib import Path
from xml.etree import ElementTree as ET
import argparse
import io
import json
import re

from pypdf import PdfReader, PdfWriter
from pypdf.generic import ArrayObject, BooleanObject, DecodedStreamObject, FloatObject, NameObject

NS = "{http://www.w3.org/2000/svg}"
MATCH_MARGIN = D("0.015")  # pt, measurement hypothesis for this probe only
NUM = re.compile(r"[-+]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][-+]?[0-9]+)?")
BLACK = (D(0), D(0), D(0))


class Unknown(Exception):
    pass


def require(condition, why):
    if not condition:
        raise Unknown(why)


def number(value):
    value = D(str(value))
    require(value.is_finite(), "nonfinite coordinate")
    return value


def matrix(args):
    a, b, c, d, e, f = map(number, args)
    require(b == c == 0 and a != 0 and abs(a) == abs(d), "unsupported PDF transform")
    return a, d, e, f


def transform(ctm, point, height):
    a, d, e, f = ctm
    x, y = point
    return a * x + e, height - (d * y + f)


def intersect(a, b):
    left, top, right, bottom = max(a[0], b[0]), max(a[1], b[1]), min(a[2], b[2]), min(a[3], b[3])
    require(left < right and top < bottom, "empty PDF clip intersection")
    return left, top, right, bottom


def bbox(points):
    xs, ys = zip(*points)
    return min(xs), min(ys), max(xs), max(ys)


def rectangle(path):
    kinds = tuple(k for k, _ in path)
    require(kinds in [(b"m", b"l", b"l", b"l", b"h"), (b"m", b"l", b"l", b"l", b"l", b"h")], "unsupported clip path")
    points = [points[0] for k, points in path if k != b"h"]
    if len(points) == 5:
        require(points[-1] == points[0], "open rectangular clip")
        points.pop()
    left, top, right, bottom = bbox(points)
    require(left < right and top < bottom and set(points) == {(left, top), (left, bottom), (right, top), (right, bottom)}, "nonrectangular clip corners")
    for (x, y), (nx, ny) in zip(points, points[1:] + points[:1]):
        require((x == nx) != (y == ny), "diagonal or zero-length clip edge")
    return left, top, right, bottom


@dataclass(frozen=True)
class Ink:
    points: tuple
    width: D

    def padded_hull(self):
        left, top, right, bottom = bbox(self.points)
        radius = self.width / 2 + MATCH_MARGIN
        return left - radius, top - radius, right + radius, bottom + radius


@dataclass(frozen=True)
class State:
    clip: tuple
    ctm: tuple = (D(1), D(1), D(0), D(0))
    width: D = D(1)
    cap: int = 0
    join: int = 0
    dash: tuple = ()
    fill: tuple = BLACK
    stroke: tuple = BLACK
    stroke_adjust: bool = False


def only_page(reader):
    require(len(reader.pages) == 1, "only the one-page probe is supported")
    page = reader.pages[0]
    media = tuple(map(number, page.mediabox))
    require(media[:2] == (0, 0) and tuple(map(number, page.cropbox)) == media, "unsupported media/crop box")
    require(int(page.get("/Rotate", 0)) == 0 and number(page.get("/UserUnit", 1)) == 1, "unsupported page rotation/resize")
    require(not (page.get("/Annots") and page["/Annots"]) and not page.get("/Group"), "annotations/transparency group")
    require(media[2] > 0 and media[3] > 0, "nonpositive media size")
    return page, media


def pdf_paths(reader):
    page, clip = only_page(reader)
    resources = page["/Resources"]
    require(not resources.get("/XObject") and not resources.get("/Pattern"), "external renderer resources")
    color_spaces = resources["/ColorSpace"]
    require(color_spaces.get("/CSp") == "/DeviceRGB", "unsupported RGB binding")
    graphics = resources["/ExtGState"]
    state = State(clip)
    stack = []
    path = []
    ties = []
    text = False
    pending_clip = False
    for args, op in page.get_contents().operations:
        require(not pending_clip or op == b"n", "unsupported clipping sequence")
        if op == b"q":
            stack.append(state)
        elif op == b"Q":
            require(stack, "graphics state stack underflow")
            state = stack.pop()
        elif op == b"cm":
            require(not path, "transform mid-path")
            a, d, e, f = matrix(args)
            oa, od, oe, of = state.ctm
            state = replace(state, ctm=(oa * a, od * d, oa * e + oe, od * f + of))
        elif op in {b"J", b"j", b"w"}:
            x, = args
            name = {b"J": "cap", b"j": "join", b"w": "width"}[op]
            state = replace(state, **{name: number(x)})
        elif op == b"d":
            pattern, phase = args
            state = replace(state, dash=tuple(pattern) + ((phase,) if phase else ()))
        elif op == b"gs":
            key, = args
            g = graphics[key].get_object()
            require(set(g) <= {"/Type", "/SA", "/SM", "/ca", "/CA", "/AIS", "/SMask"}, "unsupported PDF graphics state override")
            require(number(g.get("/ca", 1)) == number(g.get("/CA", 1)) == 1 and not g.get("/AIS", BooleanObject(False)).value and g.get("/SMask", "/None") == "/None", "unsupported alpha or soft mask")
            state = replace(state, stroke_adjust=g.get("/SA", BooleanObject(False)).value)
        elif op in {b"cs", b"CS"}:
            require(args == ["/CSp"], "unsupported color space")
        elif op in {b"scn", b"SCN"}:
            color = tuple(map(number, args))
            require(len(color) == 3 and all(D(0) <= x <= D(1) for x in color), "unsupported RGB tuple")
            state = replace(state, **{"fill" if op == b"scn" else "stroke": color})
        elif op in {b"m", b"l", b"c", b"h"}:
            size = {b"m": 2, b"l": 2, b"c": 6, b"h": 0}[op]
            require(not text and len(args) == size and (op != b"m" or not path), "unsupported path arity or subpaths")
            points = tuple(transform(state.ctm, point, clip[3]) for point in zip(map(number, args[::2]), map(number, args[1::2])))
            path.append((op, points))
        elif op == b"W*":
            state = replace(state, clip=intersect(state.clip, rectangle(path)))
            pending_clip = True
        elif op in {b"S", b"B*", b"f*", b"n"}:
            require(not text, "path paint inside text")
            if op == b"B*":
                require(tuple(k for k, _ in path) == (b"m", b"c", b"c", b"h"), "unsupported filled/stroked curve")
                require(state.width > 0 and state.cap == state.join == 1 and not state.dash and state.fill == state.stroke == BLACK, "unsupported tie stroke/fill style")
                points = tuple(pt for _, pts in path for pt in pts)
                require(points[0] == points[-1] and points[0] != points[3], "tie path does not close at distinct ends")
                shape = Ink(points, abs(state.ctm[0]) * state.width)
                hull = shape.padded_hull()
                require(state.clip[0] <= hull[0] and state.clip[1] <= hull[1] and hull[2] <= state.clip[2] and hull[3] <= state.clip[3], "tie not proven unclipped")
                ties.append(shape)
            else:
                require(all(k != b"c" for k, _ in path), "unclassified cubic path")
            path = []
            pending_clip = False
        elif op == b"BT":
            require(not text and not path, "unsupported text start")
            text = True
        elif op == b"ET":
            require(text, "text end without start")
            text = False
        elif op in {b"Tf", b"Tm", b"Td", b"Tj"}:
            require(text, "text operator outside text scope")
        else:
            raise Unknown(f"unsupported PDF operator {op!r}")
    require(not stack and not path and not pending_clip and not text, "open PDF state at end of page")
    return ties


def svg_paths(svg_file):
    svg = ET.parse(svg_file).getroot()
    require(svg.tag == NS + "svg" and set(svg.attrib) <= {"width", "height", "viewBox", "version", "baseProfile"}, "unsupported SVG root")
    x0, y0, w, h = map(number, svg.attrib["viewBox"].split())
    require(x0 == y0 == 0 and w > 0 and h > 0, "unsupported SVG viewport")
    widths = [svg.attrib[k] for k in ("width", "height")]
    require(all(v.endswith("mm") for v in widths), "SVG physical scale unavailable")
    sx, sy = [(number(value[:-2]) * 72 / D("25.4")) / span for value, span in zip(widths, (w, h))]
    require(abs(sx - sy) <= D("0.0000001") and sx > 0, "unsupported SVG anisotropic scale")
    ties = []
    for node in svg.iter():
        if node.attrib.get("class") != "TieSegment":
            continue
        require(node in list(svg) and node.tag == NS + "path" and set(node.attrib) <= {"class", "stroke", "stroke-width", "stroke-linecap", "stroke-linejoin", "fill-rule", "fill", "d"}, "unsupported SVG tie hierarchy/style")
        require(node.attrib["stroke"].lower() == "#000000" and node.get("fill", "#000000").lower() == "#000000" and node.get("stroke-linecap") == node.get("stroke-linejoin") == "round" and node.get("fill-rule") == "evenodd", "unsupported SVG tie stroke/fill")
        data = node.attrib["d"]
        residual = NUM.sub("", data).replace(",", "").replace(" ", "")
        require(residual == "MCC", "unsupported SVG tie path")
        numbers = list(map(number, NUM.findall(data)))
        require(len(numbers) == 14, "SVG tie path arity")
        points = tuple((sx * x, sy * y) for x, y in zip(numbers[::2], numbers[1::2]))
        width = number(node.attrib["stroke-width"]) * sx
        require(width > 0 and points[0] == points[-1] and points[0] != points[3], "bad SVG tie width or endpoints")
        ties.append(Ink(points, width))
    return ties


def horizontal_fixture_ink_bounds(ink):
    levels = []
    for start, first, second, stop in (ink.points[:4], ink.points[3:]):
        xs = tuple(p[0] for p in (start, first, second, stop))
        require(xs == tuple(sorted(xs)) or xs == tuple(sorted(xs, reverse=True)), "nonmonotone fixture x")
        require(start[1] == stop[1] and first[1] == second[1], "asymmetric fixture y")
        # At t=1/2 the equal endpoint/control pairs attain the only interior extremum.
        levels.extend([start[1], (start[1] + 3 * first[1]) / 4])
    radius = ink.width / 2
    return (ink.points[0][0] - radius, min(levels) - radius,
            ink.points[3][0] + radius, max(levels) + radius)


def distance(a, b):
    require(len(a.points) == len(b.points) == 7, "unsupported contour endpoint count")
    distance = max(((x - xx) ** 2 + (y - yy) ** 2).sqrt() for (x, y), (xx, yy) in zip(a.points, b.points))
    return distance, abs(a.width - b.width)


def bind(svg, pdf):
    require(len(svg) == len(pdf) > 0, "missing or excess PDF cubic contours")
    used = set()
    matches = []
    for expected in svg:
        possible = [(index, distance(expected, observed)) for index, observed in enumerate(pdf)]
        possible = [(index, delta) for index, delta in possible if max(delta) <= MATCH_MARGIN]
        require(len(possible) == 1, f"PDF contour match count {len(possible)}")
        index, gap = possible[0]
        require(index not in used, "same PDF contour assigned more than once")
        used.add(index)
        matches.append(gap)
    return matches


def changed_pdf(pdf_file, mutate):
    writer = PdfWriter(clone_from=str(pdf_file))
    stream = writer.pages[0].get_contents()
    operations = stream.operations
    mutate(operations)
    payload = DecodedStreamObject()
    payload.set_data(stream.get_data())
    writer.pages[0].replace_contents(payload)
    out = io.BytesIO()
    writer.write(out)
    out.seek(0)
    return PdfReader(out, strict=True)


def probe(pdf_file, svg_file, tests):
    svg = svg_paths(svg_file)
    original = PdfReader(str(pdf_file), strict=True)
    shapes = pdf_paths(original)
    pairs = bind(svg, shapes)
    result = {
        "pdf": pdf_file.name,
        "paired_svg_pdf_ties": len(pairs),
        "control_point_max_error_pt": str(max(p[0] for p in pairs)),
        "stroke_width_max_error_pt": str(max(p[1] for p in pairs)),
    }
    if tests:
        def shifted(ops):
            curves = [args for args, op in ops if op == b"c"]
            require(len(curves) == 2, "expected one complete two-curve proof fixture")
            endpoint_y = number(curves[0][5])
            inner, outer = sorted(curves, key=lambda args: abs(number(args[1]) - endpoint_y))
            outer_y = number(outer[1])
            inner_y = number(inner[1]); changed = (endpoint_y + inner_y) / 2
            require(min(outer_y, endpoint_y) < changed < max(outer_y, endpoint_y), "mutant would expand the old control hull")
            inner[1] = inner[3] = FloatObject(str(changed))
        changed = pdf_paths(changed_pdf(pdf_file, shifted))
        require(len(shapes) == len(changed) == 1, "expected one synthetic tie")
        a, b = shapes[0], changed[0]
        require((a.points[0], a.points[3], a.points[-1], bbox(a.points), a.width) == (b.points[0], b.points[3], b.points[-1], bbox(b.points), b.width), "mutant changed endpoint, width, or control hull")
        require(horizontal_fixture_ink_bounds(a) == horizontal_fixture_ink_bounds(b), "mutant changed its exact ink bounds")
        try:
            bind(svg, changed)
        except Unknown as e:
            result["inner_curve_mutation"] = {"outcome": str(e), "endpoints_same": True, "control_hull_same": True, "ink_aabb_same": True, "control_point_max_error_pt": str(distance(svg[0], changed[0])[0])}
        else:
            raise AssertionError("moved inner curve was accepted")
        # Two exact copies share the same geometry. Neither source order nor
        # paint order is allowed to resolve the ambiguity.
        try:
            bind([svg[0], svg[0]], [shapes[0], shapes[0]])
        except Unknown as e:
            result["coincident_contours"] = str(e)
        else:
            raise AssertionError("duplicate contour binding was accepted")

        def clipped(ops):
            index = next(i for i, (_, op) in enumerate(ops) if op == b"W*")
            for args, op in ops[index - 6:index]:
                if op == b"l" and number(args[0]) > 0:
                    args[0] = FloatObject("2000")
        try:
            pdf_paths(changed_pdf(pdf_file, clipped))
        except Unknown as e:
            require(str(e) == "tie not proven unclipped", f"unexpected clipping probe rejection: {e}")
            result["clipped_tie"] = str(e)
        else:
            raise AssertionError("clipped tie was accepted")

        def unsupported(ops):
            ops.append((ArrayObject([NameObject("/Untested")]), b"Do"))
        try:
            pdf_paths(changed_pdf(pdf_file, unsupported))
        except Unknown as e:
            result["unhandled_operator"] = str(e)
        else:
            raise AssertionError("unsupported external drawing was ignored")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("prefix", type=Path)
    parser.add_argument("--negative-probes", action="store_true")
    args = parser.parse_args()
    try:
        print(json.dumps(probe(args.prefix.with_suffix(".pdf"), args.prefix.with_name(args.prefix.name + "-1.svg"), args.negative_probes), indent=2))
    except (Unknown, ValueError, KeyError, TypeError) as error:
        parser.exit(2, f"unknown: {error}\n")
