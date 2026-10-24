#!/usr/bin/env python3
"""Read-only evaluation of real exported artifacts, never a productive selector.

Usage: python compare_beam_take_r1472.py PARENT_EXPORT.zip CANDIDATE_EXPORT.zip
Optional --pdf-pixels compares rendered page pixels with installed PyMuPDF.
Writes JSON to stdout; does not modify the exports or any project.
"""
import argparse
import collections
import json
import re
import xml.etree.ElementTree as ET
import zipfile


def strip_beams(data):
    return re.sub(rb"<beam\b[^>]*>.*?</beam\s*>", b"", data, flags=re.S)


def nonbeam_tree(data):
    def node_value(node):
        text = node.text if node.text and node.text.strip() else None
        tail = node.tail if node.tail and node.tail.strip() else None
        return (node.tag, tuple(sorted(node.attrib.items())), text, tail,
                tuple(node_value(child) for child in node if child.tag != "beam"))
    return node_value(ET.fromstring(data))


def note_rows(data):
    root = ET.fromstring(data)
    rows = []
    for part_index, part in enumerate(root.findall("part")):
        for measure_index, measure in enumerate(part.findall("measure")):
            cursor = last_onset = 0
            ordinal = 0
            for node in measure:
                duration = int(node.findtext("duration", "0"))
                if node.tag == "backup":
                    cursor -= duration
                elif node.tag == "forward":
                    cursor += duration
                elif node.tag == "note":
                    chord = node.find("chord") is not None
                    onset = last_onset if chord else cursor
                    if not chord:
                        last_onset = cursor
                        cursor += duration
                    rows.append({
                        "key": (part_index, measure_index, ordinal),
                        "measure": measure.get("number"), "onset": onset,
                        "staff": node.findtext("staff", "1"),
                        "voice": node.findtext("voice", "1"),
                        "duration": duration, "chord": chord,
                        "beams": tuple((b.get("number", "1"), (b.text or "").strip()) for b in node.findall("beam")),
                    })
                    ordinal += 1
    return rows


def pdf_difference(before, after):
    import fitz
    a = fitz.open(stream=before, filetype="pdf")
    b = fitz.open(stream=after, filetype="pdf")
    if len(a) != len(b):
        return {"before_pages": len(a), "after_pages": len(b), "page_count_changed": True}
    changed = []
    for i, (pa, pb) in enumerate(zip(a, b)):
        xa = pa.get_pixmap(matrix=fitz.Matrix(1.5, 1.5), alpha=False)
        xb = pb.get_pixmap(matrix=fitz.Matrix(1.5, 1.5), alpha=False)
        if (xa.width, xa.height, xa.samples) != (xb.width, xb.height, xb.samples):
            changed.append(i + 1)
    return {"pages": len(a), "changed_pages_at_108dpi": changed}


def compare(parent_path, candidate_path, pixels=False):
    projects = []
    errors = []
    with zipfile.ZipFile(parent_path) as parent, zipfile.ZipFile(candidate_path) as candidate:
        parent_names = set(parent.namelist())
        names = sorted(n for n in candidate.namelist() if n.endswith(".render.musicxml"))
        if len(names) != 10:
            errors.append(f"expected 10 Render XML files, got {len(names)}")
        for name in names:
            if name not in parent_names:
                errors.append(f"parent missing {name}")
                continue
            stem = name.removesuffix(".render.musicxml")
            master_name = stem + ".musicxml"
            old, new = parent.read(name), candidate.read(name)
            master_equal = parent.read(master_name) == candidate.read(master_name)
            nonbeam_bytes_equal = strip_beams(old) == strip_beams(new)
            beam_only = nonbeam_tree(old) == nonbeam_tree(new)
            if not master_equal:
                errors.append(f"Master changed: {stem}")
            if not beam_only:
                errors.append(f"non-beam Render change: {stem}")
            a, b, master = note_rows(old), note_rows(new), note_rows(candidate.read(master_name))
            changes = []
            per_level = collections.Counter()
            master_by_key = {r["key"]: r for r in master}
            if len(a) != len(b):
                errors.append(f"Render note count changed: {stem}")
            for left, right in zip(a, b):
                if {k: v for k, v in left.items() if k != "beams"} != {k: v for k, v in right.items() if k != "beams"}:
                    errors.append(f"Render event mapping changed: {stem} {left['key']}")
                if left["beams"] == right["beams"]:
                    continue
                # Master correspondence is descriptive only. It must match the same
                # ordinal/timing/lane; no nearest-pitch matching is invented.
                m = master_by_key.get(right["key"])
                identity_keys = ("onset", "staff", "voice", "duration", "chord")
                congruent = m is not None and all(m[k] == right[k] for k in identity_keys)
                levels = sorted(set(n for n, _ in left["beams"] + right["beams"]), key=int)
                delta = {}
                for level in levels:
                    x = [v for n, v in left["beams"] if n == level]
                    y = [v for n, v in right["beams"] if n == level]
                    if x != y:
                        per_level[level] += 1
                        delta[level] = {"before": x, "after": y}
                        if congruent:
                            delta[level]["master"] = [v for n, v in m["beams"] if n == level]
                            delta[level]["after_equals_master"] = y == delta[level]["master"]
                changes.append({k: right[k] for k in ("key", "measure", "onset", "staff", "voice")}
                               | {"levels": delta, "master_mapping_congruent": congruent})
            report_name = stem + ".result.txt"
            report = candidate.read(report_name).decode("utf-8-sig")
            observation_count = len(re.findall(r"^MetricProtectionObservation: ", report, re.M))
            physical_trace = "RealeWriterDeltas:" in report
            if observation_count == 0 or not physical_trace:
                errors.append(f"missing optional measurement evidence: {stem}")
            item = {"project": stem, "master_byte_equal": master_equal,
                    "render_byte_equal": old == new, "render_beam_only": beam_only,
                    "nonbeam_bytes_equal": nonbeam_bytes_equal,
                    "changed_notes_per_level": dict(per_level), "changes": changes,
                    "metric_protection_observations": observation_count,
                    "full_writer_trace_present": physical_trace}
            if pixels:
                item["pdf"] = pdf_difference(parent.read(stem + ".pdf"), candidate.read(stem + ".pdf"))
            projects.append(item)
    return {"scope": "read_only_native_export_comparison", "errors": errors, "projects": projects}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("parent")
    parser.add_argument("candidate")
    parser.add_argument("--pdf-pixels", action="store_true")
    args = parser.parse_args()
    result = compare(args.parent, args.candidate, args.pdf_pixels)
    print(json.dumps(result, ensure_ascii=False, indent=2))
    raise SystemExit(bool(result["errors"]))
