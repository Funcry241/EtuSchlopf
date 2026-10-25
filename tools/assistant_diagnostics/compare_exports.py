"""Compare EtuSchlopf assistant exports without extracting or modifying them."""
from __future__ import annotations

from contextlib import closing
import argparse
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import re
import zipfile

ROOT_FILES = {"Cargo.toml", "README.md", "ZIP_KIND.txt", "run.cmd", "strict.cmd",
              "rust-toolchain.toml", "rustfmt.toml", "clippy.toml"}
MANAGED_DIRS = {"src", "docs", "tools", "tests"}


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def index(archive: zipfile.ZipFile) -> dict:
    result = {}
    for entry in archive.infolist():
        name = entry.filename.replace("\\", "/")
        path = PurePosixPath(name)
        if path.is_absolute() or ".." in path.parts or ":" in name:
            raise ValueError(f"Unsafe archive path: {entry.filename}")
        if entry.is_dir() or name.endswith("/"):
            continue
        if name in result:
            raise ValueError(f"Duplicate archive entry: {name}")
        result[name] = entry
    return result


def read(archive, entries, name):
    return archive.read(entries[name])


def text(archive, entries, name):
    return read(archive, entries, name).decode("utf-8-sig", errors="replace") if name in entries else ""


def kv(value: str) -> dict:
    return dict(line.split("=", 1) for line in value.splitlines() if "=" in line)


def differences(before: dict, after: dict) -> dict:
    a, b = set(before), set(after)
    return {"identical": sorted(n for n in a & b if before[n] == after[n]),
            "modified": sorted(n for n in a & b if before[n] != after[n]),
            "added": sorted(b - a), "removed": sorted(a - b)}


def is_changed(delta: dict) -> bool:
    return any(delta[key] for key in ("modified", "added", "removed"))


def source_snapshot(archive, entries) -> dict:
    meta = kv(text(archive, entries, "source_snapshot/SOURCE_SNAPSHOT.txt"))
    name = meta.get("SourceArchive", "")
    path = "source_snapshot/" + name
    if not name or path not in entries:
        return {"status": "unavailable", "reason": "bound source snapshot missing"}
    payload = read(archive, entries, path)
    digest = sha(payload)
    claimed = meta.get("SourceZipSha256", "").removeprefix("sha256:")
    if claimed != digest or meta.get("source_complete") != "true":
        return {"status": "unavailable", "reason": "snapshot hash/completeness binding not confirmed",
                "actual_sha256": digest, "claimed_sha256": claimed}
    with zipfile.ZipFile(io.BytesIO(payload)) as inner:
        names = index(inner)
        all_hashes = {n: sha(read(inner, names, n)) for n in names}
    managed = {}
    for n, digest in all_hashes.items():
        parts = PurePosixPath(n).parts
        if n in ROOT_FILES or (parts and parts[0] in MANAGED_DIRS and "target" not in parts):
            managed[n] = digest
    required = {"Cargo.toml", "README.md", "ZIP_KIND.txt", "run.cmd", "strict.cmd"}
    if not required.issubset(managed) or not all(any(n.startswith(d + "/") for n in managed) for d in MANAGED_DIRS):
        return {"status": "unavailable", "reason": "snapshot does not match supported managed-tree layout"}
    return {"status": "available", "archive_sha256": sha(payload), "managed_hashes": managed,
            "all_entry_hashes": all_hashes,
            "scope": "known r1463 managed-tree projection; not a Take validator"}


def tests_observation(value: str) -> dict:
    if "STRICT ALL-TARGET TEST RECEIPT REUSE" in value or "reused_exact_validation_receipt" in value:
        return {"mode": "reported_reused_bound_receipt", "receipt_text_sha256": sha(value.encode()),
                "limit": "receipt claim read, binary and receipt authority not independently validated"}
    rows = re.findall(r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;", value)
    if rows:
        totals = [sum(int(row[i]) for row in rows) for i in range(3)]
        return {"mode": "reported_execution_log", "summary_blocks": len(rows),
                "passed": totals[0], "failed": totals[1], "ignored": totals[2]}
    return {"mode": "unknown", "reason": "no supported execution summary or reuse marker"}


def run_observation(archive, entries) -> dict:
    payload = text(archive, entries, "corpus_run_manifest.json")
    run = json.loads(payload) if payload else {}
    manifest = text(archive, entries, "manifest.txt")
    values = {}
    for line in manifest.splitlines():
        if ": " in line:
            key, value = line.split(": ", 1)
            if key in ("FinalExit", "BuildExit", "ClippyExit", "FullCorpus", "ProjectEvidence"):
                values[key] = value
    return {"run_id": run.get("run_id"), "started_unix_seconds": run.get("started_unix_seconds"),
            "completed_unix_seconds": run.get("completed_unix_seconds"), "reported_outcomes": values,
            "strict_tests": tests_observation(text(archive, entries, "visual_preflight.strict-tests.txt"))}


def compare_pdf(before: bytes, after: bytes, dpi: int) -> dict:
    result = {"bytes_identical": before == after, "before_sha256": sha(before),
              "after_sha256": sha(after), "dpi": dpi}
    try:
        import pypdfium2 as pdfium
    except ImportError:
        return {**result, "visual_status": "unavailable", "reason": "pypdfium2 is not installed"}
    try:
        if dpi <= 0:
            raise ValueError("dpi must be positive")
        with pdfium.PdfDocument(before) as a, pdfium.PdfDocument(after) as b:
            if (pdfium.raw.FPDF_GetSecurityHandlerRevision(a) >= 0
                    or pdfium.raw.FPDF_GetSecurityHandlerRevision(b) >= 0
                    or not len(a) or not len(b)):
                raise ValueError("encrypted or empty PDF")
            changed = []
            for i in range(min(len(a), len(b))):
                with closing(a[i]) as pa, closing(b[i]) as pb:
                    with closing(pa.render(scale=dpi / 72, force_bitmap_format=pdfium.raw.FPDFBitmap_BGR)) as x, \
                            closing(pb.render(scale=dpi / 72, force_bitmap_format=pdfium.raw.FPDFBitmap_BGR)) as y:
                        if (pa.get_size() != pb.get_size()
                                or (x.width, x.height, bytes(x.buffer)) != (y.width, y.height, bytes(y.buffer))):
                            changed.append(i + 1)
            changed.extend(range(min(len(a), len(b)) + 1, max(len(a), len(b)) + 1))
            return {**result, "visual_status": "different" if changed else "pixel_equal",
                    "pages_before": len(a), "pages_after": len(b), "different_pages": changed,
                    "raster_engine": "pypdfium2 " + str(pdfium.PYPDFIUM_INFO)
                                     + " / PDFium " + str(pdfium.PDFIUM_INFO)}
    except Exception as error:
        return {**result, "visual_status": "unavailable", "reason": str(error)}


def compare(before: Path, after: Path, dpi: int = 144) -> dict:
    report = {"status": "INCOMPLETE", "before": str(before.resolve()), "after": str(after.resolve()),
              "before_export_sha256": sha(before.read_bytes()), "after_export_sha256": sha(after.read_bytes()),
              "take_admission": "NOT_EVALUATED", "native_acceptance": "NOT_EVALUATED"}
    with zipfile.ZipFile(before) as a, zipfile.ZipFile(after) as b:
        ai, bi = index(a), index(b)
        report["before_run"], report["after_run"] = run_observation(a, ai), run_observation(b, bi)
        ids = report["before_run"]["run_id"], report["after_run"]["run_id"]
        report["different_run_ids"] = ids[0] != ids[1] if all(ids) else None
        x, y = source_snapshot(a, ai), source_snapshot(b, bi)
        complete = x["status"] == y["status"] == "available"
        changed = False
        if complete:
            managed = differences(x.pop("managed_hashes"), y.pop("managed_hashes"))
            all_entries = differences(x.pop("all_entry_hashes"), y.pop("all_entry_hashes"))
            changed |= is_changed(managed)
            report["source"] = {"before": x, "after": y, "managed_delta": managed,
                                "all_snapshot_entries_delta": all_entries}
        else:
            for item in (x, y):
                item.pop("managed_hashes", None)
                item.pop("all_entry_hashes", None)
            report["source"] = {"before": x, "after": y}
        xml_a = {n: sha(read(a, ai, n)) for n in ai if "/" not in n and n.endswith(".musicxml")}
        xml_b = {n: sha(read(b, bi, n)) for n in bi if "/" not in n and n.endswith(".musicxml")}
        report["xml"] = differences(xml_a, xml_b)
        changed |= is_changed(report["xml"])
        complete &= bool(xml_a and xml_b)
        names_a = {n for n in ai if "/" not in n and n.endswith(".pdf")}
        names_b = {n for n in bi if "/" not in n and n.endswith(".pdf")}
        report["pdf"] = {"added": sorted(names_b - names_a), "removed": sorted(names_a - names_b), "files": {}}
        changed |= names_a != names_b
        complete &= bool(names_a and names_b)
        for name in sorted(names_a & names_b):
            result = compare_pdf(read(a, ai, name), read(b, bi, name), dpi)
            report["pdf"]["files"][name] = result
            changed |= result["visual_status"] == "different"
            complete &= result["visual_status"] != "unavailable"
        report["status"] = "INCOMPLETE" if not complete else ("CHANGED" if changed else "SAME_MANAGED_SOURCE_XML_AND_PDF_RASTER")
    return report


def markdown(report: dict) -> str:
    lines = ["# EtuSchlopf – Exportvergleich", "", f"**{report['status']}**", "",
             "Lesender Vergleich; keine Take-Zulassung oder native Abnahme.", ""]
    if "error" in report:
        return "\n".join(lines + [report["error"], ""])
    source = report.get("source", {}).get("managed_delta")
    if source is not None:
        lines += [f"Verwaltete Quellen: {len(source['identical'])} identisch; "
                  f"{len(source['modified'])} verändert; {len(source['added'])} hinzugefügt; {len(source['removed'])} entfernt.", ""]
        for label in ("modified", "added", "removed"):
            lines += [f"- Quelle {label}: `{n}`" for n in source[label]]
        entire = report["source"]["all_snapshot_entries_delta"]
        if is_changed(entire):
            lines += ["", "Zusätzlich vollständigen Snapshot-Diff im JSON prüfen; er enthält auch lokalen Zustand wie Root-Cargo.lock.", ""]
    else:
        lines += ["Quellvergleich nicht vollständig verfügbar; JSON-Begründung prüfen.", ""]
    delta = report["xml"]
    lines += [f"XML: {len(delta['identical'])} bytegleich; {len(delta['modified'])} verändert; "
              f"{len(delta['added'])} hinzugefügt; {len(delta['removed'])} entfernt.", ""]
    for label in ("modified", "added", "removed"):
        lines += [f"- XML {label}: `{n}`" for n in delta[label]]
    pdf = report["pdf"]
    lines += ["", "| PDF | Seiten vorher/nachher | Rastervergleich | PDF-Bytes gleich |",
              "|---|---|---|---|"]
    for name, value in pdf["files"].items():
        status = value["visual_status"]
        if value.get("different_pages"):
            status += ": " + ", ".join(map(str, value["different_pages"]))
        lines.append(f"| {name} | {value.get('pages_before', '?')}/{value.get('pages_after', '?')} | {status} | {value['bytes_identical']} |")
    for label in ("added", "removed"):
        lines += [f"- PDF {label}: `{n}`" for n in pdf[label]]
    lines += ["", f"Verschiedene Run-IDs: {report['different_run_ids']}.", ""]
    for key, label in (("before_run", "Vorher"), ("after_run", "Nachher")):
        run = report[key]
        lines += [f"{label}: `{run['run_id']}`; Strict-Tests: `{run['strict_tests']['mode']}`.", ""]
        lines += [f"Gemeldete Ergebnisse: `{json.dumps(run['reported_outcomes'], ensure_ascii=False)}`.", ""]
        if run["strict_tests"]["mode"] == "reported_execution_log":
            observed = run["strict_tests"]
            lines += [f"Testlog: {observed['passed']} bestanden, {observed['failed']} fehlgeschlagen, "
                      f"{observed['ignored']} ignoriert; {observed['summary_blocks']} Ergebnisblöcke.", ""]
    lines += ["PDF-Pixelgleichheit gilt nur für die angegebene Auflösung und Raster-Engine.",
              "Ein anderer PDF-Hash allein beweist keine Änderung des Notenbilds.",
              "Gemeldete Lauf-/Testzustände sind keine unabhängige Validierung ihrer Autorität.", ""]
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("before", type=Path)
    parser.add_argument("after", type=Path)
    parser.add_argument("--out", type=Path, required=True, help="New output directory")
    parser.add_argument("--dpi", type=int, default=144)
    args = parser.parse_args()
    if not 36 <= args.dpi <= 600:
        parser.error("--dpi must be between 36 and 600")
    if args.out.exists():
        parser.error("--out must be a new directory")
    args.out.mkdir(parents=True)
    try:
        report = compare(args.before, args.after, args.dpi)
    except (OSError, ValueError, KeyError, zipfile.BadZipFile) as error:
        report = {"status": "INCOMPLETE", "error": str(error), "take_admission": "NOT_EVALUATED"}
    (args.out / "vergleich.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (args.out / "vergleich.md").write_text(markdown(report), encoding="utf-8")
    print(report["status"])
    print(args.out.resolve() / "vergleich.md")
    return {"SAME_MANAGED_SOURCE_XML_AND_PDF_RASTER": 0, "CHANGED": 1}.get(report["status"], 2)


if __name__ == "__main__":
    raise SystemExit(main())
