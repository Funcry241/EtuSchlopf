"""Compare embedded notation geometry in two matched score exports."""
import argparse
from collections import Counter
from dataclasses import asdict, dataclass
from io import BytesIO
from itertools import pairwise
import json
import logging
from pathlib import Path
from xml.etree.ElementTree import ParseError
from zipfile import BadZipFile, ZipFile

import fitz
from fontTools.ttLib import TTFont

from bar_layout import barlines, horiz_staff, spaced_staves, systems_xml
from geometry import Beam, Contact, Head, Stem, attach_beams, attach_heads, classify_contacts, group_stems

logging.getLogger("fontTools.ttLib").setLevel(logging.ERROR)


@dataclass
class PageGeometry:
    stems: list[Stem]
    beams: list[Beam]
    heads: list[Head]
    solid_heads: set[int]
    flags: list[tuple[float, float]]


def font_objects(document, page, cache):
    fonts = {}
    for xref, ext, _, name, *_ in page.get_fonts(full=True):
        if ext != "ttf" or not name.endswith("Leland"):
            continue
        if xref not in cache:
            content = document.extract_font(xref)[3]
            cache[xref] = TTFont(BytesIO(content))
        fonts[name.split("+")[-1]] = cache[xref]
    return fonts


def head_bounds(font, gid, origin, size):
    glyph = font["glyf"][font.getGlyphName(gid)]
    x, y = origin
    factor = size / font["head"].unitsPerEm
    return Head(
        x + factor * glyph.xMin, y - factor * glyph.yMax,
        x + factor * glyph.xMax, y - factor * glyph.yMin,
    )


def read_page(document, page, drawings, font_cache):
    stems = []
    beams = []
    for drawing in drawings:
        items = drawing["items"]
        if drawing["type"] == "s" and len(items) == 1:
            item = items[0]
            if item[0] == "l" and abs(item[1].x - item[2].x) < .025 and .30 <= drawing["width"] <= .56:
                top, bottom = sorted((item[1].y, item[2].y))
                if bottom - top >= 3:
                    stems.append(Stem(item[1].x, top, bottom, drawing["width"]))
        if drawing["type"] != "f" or drawing["fill"] != (0., 0., 0.) or len(items) != 4:
            continue
        if any(item[0] != "l" for item in items):
            continue
        sides = sorted(
            (item[1].x, *sorted((item[1].y, item[2].y)))
            for item in items if abs(item[1].x - item[2].x) < .025
        )
        if len(sides) != 2:
            continue
        (left, top_left, bottom_left), (right, top_right, bottom_right) = sides
        left_thickness = bottom_left - top_left
        right_thickness = bottom_right - top_right
        if 1.45 < right - left < 260 and 1.35 < left_thickness < 3.05 and abs(left_thickness - right_thickness) < .13:
            beams.append(Beam(left, right, top_left, bottom_left, top_right, bottom_right))

    fonts = font_objects(document, page, font_cache)
    heads = []
    solid = set()
    flags = []
    notehead_count = 0
    for span in page.get_texttrace():
        font = fonts.get(span["font"])
        if font is None:
            continue
        for code, gid, origin, _ in span["chars"]:
            if 0xe0a0 <= code <= 0xe0b4:
                notehead_count += 1
            if code in (0xe0a3, 0xe0a4):
                if code == 0xe0a4:
                    solid.add(len(heads))
                heads.append(head_bounds(font, gid, origin, span["size"]))
            if 0xe240 <= code <= 0xe24f:
                flags.append(origin)
    if not fonts or not notehead_count:
        raise ValueError(f"page {page.number + 1} has no supported embedded Leland noteheads")
    return PageGeometry(stems, beams, heads, solid, flags)


def bar_geometry(pdf_geom, x0, x1, y0, y1, space):
    def within(x, y):
        return x0 <= x < x1 and y0 <= y < y1

    stems = [s for s in pdf_geom.stems if within(s.x, (s.top + s.bottom) / 2)]
    beams = [
        b for b in pdf_geom.beams
        if within((b.left + b.right) / 2, sum(b.span_at((b.left + b.right) / 2)) / 2)
    ]
    heads = []
    solid = []
    for index, head in enumerate(pdf_geom.heads):
        if within(*head.center):
            if index in pdf_geom.solid_heads:
                solid.append(len(heads))
            heads.append(head)

    flags = [flag for flag in pdf_geom.flags if within(*flag)]
    flag_stems: set[int] = set()
    unattached = 0
    for x, y in flags:
        matching = {
            i for i, s in enumerate(stems)
            if abs(s.x-x) < .25 * space and min(abs(s.top-y), abs(s.bottom-y)) < .44 * space
        }
        if not matching:
            unattached += 1
        flag_stems.update(matching)

    head_stems = attach_heads(heads, stems, space)
    beam_stems = attach_beams(beams, stems, space, heads, head_stems, flag_stems)
    contacts = []
    for i, j, verdict in classify_contacts(
        [heads[i] for i in solid], beams, [head_stems[i] for i in solid], beam_stems, space,
    ):
        head_index = solid[i]
        members = beam_stems[j]
        x, y = heads[head_index].center
        # At least two compatible attachments anchor a visible group. Short
        # beamlets, isolated line shapes and unsupported stems stay unresolved.
        if verdict is Contact.FOREIGN_HEAD and len(members) < 2:
            verdict = Contact.UNKNOWN_ATTACHMENT
        contacts.append({
            "contact": verdict.value, "x": round(x, 3), "y": round(y, 3),
            "head": asdict(heads[head_index]), "beam": asdict(beams[j]),
            "head_stems": [asdict(stems[k]) for k in sorted(head_stems[head_index])],
            "beam_stems": [asdict(stems[k]) for k in sorted(members)],
        })
    return {
        "stems": len(stems), "beam_segments": len(beams),
        "connected_beam_groups": sum(len(group) >= 2 for group in group_stems(beam_stems)),
        "flags": len(flags), "flags_without_visible_stem": unattached,
        "longest_stem_spaces": round(max((s.length(space) for s in stems), default=0), 3),
        "contacts": contacts,
    }


def read_pair(pdf_bytes, xml_bytes):
    all_bars = []
    pages = systems_xml(xml_bytes)
    with fitz.open(stream=pdf_bytes, filetype="pdf") as document:
        if len(document) != len(pages):
            raise ValueError(f"PDF has {len(document)} pages, XML declares {len(pages)}")
        font_cache = {}
        for page, systems in zip(document, pages):
            drawings = page.get_drawings()
            staves = spaced_staves(horiz_staff(drawings))
            if len(staves) != len(systems) * 2:
                raise ValueError(f"page {page.number + 1} has {len(staves)} detected staves for {len(systems)} systems")
            geom = read_page(document, page, drawings, font_cache)
            y_bounds = [0, *[(staves[i - 1][1] + staves[i][0]) / 2 for i in range(2, len(staves), 2)], page.rect.height]
            for row, (system, (y0, y1)) in enumerate(zip(systems, pairwise(y_bounds))):
                upper, lower = staves[2 * row:2 * row + 2]
                boundaries = barlines(drawings, upper, lower)
                if len(boundaries) != len(system) + 1:
                    raise ValueError(f"page {page.number + 1} system {row + 1} has {len(boundaries) - 1} measured bars for {len(system)} XML bars")
                for bar, (x0, x1) in zip(system, pairwise(boundaries)):
                    stats = bar_geometry(geom, x0, x1, y0, y1, upper[4])
                    all_bars.append({
                        "ordinal": len(all_bars) + 1, "measure": bar.get("number"),
                        "page": page.number + 1, "system": row + 1, **stats,
                    })
    return all_bars


def pairs_in(root, load):
    by_name = {}
    xml_names = {name[:-len(".render.musicxml")]: name for name in root if name.endswith(".render.musicxml")}
    for pdf in sorted(name for name in root if name.endswith(".pdf")):
        name = pdf[:-len(".pdf")]
        if name not in xml_names:
            continue
        title = Path(name).name
        if title in by_name:
            raise ValueError(f"duplicate score name {title}")
        try:
            by_name[title] = read_pair(load(pdf), load(xml_names[name]))
        except (ValueError, ParseError) as error:
            raise ValueError(f"{name}: {error}") from error
    if not by_name:
        raise ValueError("no matching PDF / .render.musicxml pair")
    unmatched = sorted(set(xml_names) - {name[:-4] for name in root if name.endswith(".pdf")})
    if unmatched:
        raise ValueError("missing PDF for " + ", ".join(unmatched))
    return by_name


def audit(source):
    if source.is_dir():
        files = [str(path.relative_to(source)) for path in source.rglob("*") if path.is_file()]
        return pairs_in(files, lambda name: (source / name).read_bytes())
    with ZipFile(source) as archive:
        return pairs_in(archive.namelist(), archive.read)


def score_summary(scores):
    bars = [bar for parts in scores.values() for bar in parts]
    return {
        "scores": len(scores), "bars": len(bars),
        "pages": sum(len({bar["page"] for bar in parts}) for parts in scores.values()),
        "contacts": dict(Counter(hit["contact"] for bar in bars for hit in bar["contacts"])),
        "longest_stem_spaces": max(bar["longest_stem_spaces"] for bar in bars),
        "bars_over_ten_stem_spaces": sum(bar["longest_stem_spaces"] > 10 for bar in bars),
        **{key: sum(bar[key] for bar in bars) for key in (
            "stems", "beam_segments", "connected_beam_groups", "flags", "flags_without_visible_stem"
        )},
    }


def differences(base, candidate):
    common = sorted(base.keys() & candidate.keys())
    if not common:
        raise ValueError("baseline and candidate have no common score names")
    missing = sorted(base.keys() - candidate.keys())
    new = sorted(candidate.keys() - base.keys())
    changed = []
    for score in common:
        old_bars = base[score]
        new_bars = candidate[score]
        if [bar["measure"] for bar in old_bars] != [bar["measure"] for bar in new_bars]:
            raise ValueError(f"{score}: printed measure sequence differs")
        for old, new_bar in zip(old_bars, new_bars):
            summary = {}
            for key in ("flags", "connected_beam_groups", "longest_stem_spaces"):
                if old[key] != new_bar[key]:
                    summary[key] = [old[key], new_bar[key]]
            old_contacts = dict(Counter(hit["contact"] for hit in old["contacts"]))
            new_contacts = dict(Counter(hit["contact"] for hit in new_bar["contacts"]))
            if old_contacts != new_contacts:
                summary["contacts"] = [old_contacts, new_contacts]
            if summary:
                changed.append({
                    "score": score, "measure": old["measure"], "ordinal": old["ordinal"], **summary,
                })
    return {"compared_scores": common, "missing_scores": missing, "new_scores": new, "changed_bars": changed}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("baseline", type=Path)
    parser.add_argument("candidate", type=Path, nargs="?")
    parser.add_argument("--json-out", type=Path)
    args = parser.parse_args()
    try:
        baseline = audit(args.baseline)
        summary = {"baseline": score_summary(baseline)}
        details = {"baseline": baseline}
        if args.candidate is not None:
            candidate = audit(args.candidate)
            diff = differences(baseline, candidate)
            summary["candidate"] = score_summary(candidate)
            summary["comparison"] = {k: v for k, v in diff.items() if k != "changed_bars"}
            summary["comparison"]["changed_bars"] = len(diff["changed_bars"])
            details["candidate"] = candidate
            details["comparison"] = diff
    except (ValueError, OSError, BadZipFile) as error:
        parser.exit(2, f"{error}\n")
    print(json.dumps(summary, indent=2, ensure_ascii=False))
    if args.json_out is not None:
        args.json_out.write_text(json.dumps({"summary": summary, **details}, indent=2, ensure_ascii=False) + "\n")


if __name__ == "__main__":
    main()
