//! Read-only diagnostic of a shadow export bound to the successful PDF invocation.

use super::{
    RenderResult, RenderStatus, absolute_path, count_pdf_pages, format_command,
    pdf_output_snapshot_v994, pdf_output_snapshots_content_equal_v1110, pdf_structure_probe_v1110,
    render_command_variants, renderer_executable_identity, run_renderer_command,
    unix_ms_now_string_v1110,
};
use crate::audit_only::engraving_native_export_proof::{stem_length_edits, verify_strokes};
use crate::pipeline::svg_notation_geometry::collision_observation::{report, tag_native};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(test)]
#[path = "tied_stem_render_review.rs"]
mod tied_stem_render_review;

struct Shadow(PathBuf);

impl Shadow {
    fn new() -> Option<Self> {
        static SERIAL: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "etuschlopf-engraving-audit-{}-{}-{}",
            std::process::id(),
            unix_ms_now_string_v1110(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).ok()?;
        Some(Self(path))
    }
}

impl Drop for Shadow {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn pages(dir: &Path, prefix: &str) -> Option<Vec<String>> {
    let mut ordered = BTreeMap::new();
    for item in fs::read_dir(dir).ok()? {
        let path = item.ok()?.path();
        if path.extension().and_then(std::ffi::OsStr::to_str) != Some("svg") {
            continue;
        }
        let stem = path.file_stem()?.to_str()?;
        let number = if stem == prefix {
            1
        } else if let Some(tail) = stem.strip_prefix(prefix).and_then(|v| v.strip_prefix('-')) {
            tail.parse::<usize>().ok()?
        } else {
            continue;
        };
        if ordered
            .insert(number, fs::read_to_string(path).ok()?)
            .is_some()
        {
            return None;
        }
    }
    (!ordered.is_empty() && ordered.keys().copied().eq(1..=ordered.len()))
        .then(|| ordered.into_values().collect())
}

struct NativeInputs {
    score: String,
    style: Option<Vec<u8>>,
}

impl NativeInputs {
    fn capture(path: &Path) -> Option<Self> {
        let style = match fs::read(path.with_file_name("score_style.mss")) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(_) => return None,
        };
        Some(Self {
            score: fs::read_to_string(path).ok()?,
            style,
        })
    }

    fn agrees(&self, path: &Path) -> bool {
        Self::capture(path)
            .is_some_and(|current| current.score == self.score && current.style == self.style)
    }
}

/// The existing preview goes to the collision audit even when the separate
/// native PDF proof cannot be established.
struct NativePreview {
    pages: Vec<String>,
    proof: Result<String, &'static str>,
}

fn native_preview_and_pdf_proof(
    imported: &str,
    native: &Path,
    preview: &Path,
    pdf: &Path,
    convert: &impl Fn(&Path, &Path) -> Option<()>,
) -> Result<NativePreview, &'static str> {
    let input = NativeInputs::capture(native);
    convert(native, preview).ok_or("unmarked_native_svg_unavailable")?;
    let svg = pages(
        preview
            .parent()
            .ok_or("unmarked_native_pages_unavailable")?,
        preview
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or("unmarked_native_pages_unavailable")?,
    )
    .ok_or("unmarked_native_pages_unavailable")?;
    let evidence = (|| {
        let input = input.ok_or("native_input_unavailable")?;
        if !input.agrees(native) {
            return Err("native_input_drift");
        }
        let edits = stem_length_edits(imported, &input.score).ok_or("native_semantic_drift")?;
        if !matches!(pdf.try_exists(), Ok(false)) {
            return Err("native_pdf_target_not_fresh");
        }
        convert(native, pdf).ok_or("native_pdf_unavailable")?;
        if !input.agrees(native) {
            return Err("native_input_drift");
        }
        let structure = pdf_structure_probe_v1110(pdf);
        if !structure.header_valid
            || !structure.eof_marker_present
            || structure.page_count != Some(svg.len())
        {
            return Err("native_pdf_pages_unconfirmed");
        }
        let printed = fs::read(pdf).map_err(|_| "native_pdf_unavailable")?;
        let anchors = verify_strokes(&svg, &printed)?;
        let previews = svg
            .iter()
            .map(|page| crate::sha256::hex(page.as_bytes()))
            .collect::<String>();
        let style = input
            .style
            .as_deref()
            .map_or_else(|| "absent".to_owned(), crate::sha256::hex);
        Ok(format!(
            "EngravingNativeExportProof: result=bound_staff_and_stem_strokes; pages={}; stroke_anchors={anchors}; stem_length_edits={edits}; native_sha256={}; style_sha256={style}; svg_page_digest_chain_sha256={}; pdf_sha256={}; authority=none\n",
            svg.len(),
            crate::sha256::hex(input.score.as_bytes()),
            crate::sha256::hex(previews.as_bytes()),
            crate::sha256::hex(&printed)
        ))
    })();
    Ok(NativePreview {
        pages: svg,
        proof: evidence,
    })
}

fn diagnostic(
    source: &str,
    input: &Path,
    output: &Path,
    render: &RenderResult,
) -> Result<String, &'static str> {
    let identity = render
        .probe_identity
        .as_ref()
        .ok_or("unbound_pdf_invocation")?;
    let options = render_command_variants(&absolute_path(input), &absolute_path(output));
    let command = options
        .iter()
        .find(|v| format_command(&identity.executable, &v.args) == identity.command)
        .ok_or("unresolved_converter_profile")?;
    let fingerprint = renderer_executable_identity(&identity.executable);
    if fingerprint.starts_with("unavailable") || fingerprint.contains("bytes_unavailable") {
        return Err("unavailable_converter_identity");
    }
    if fs::read(input).ok().as_deref() != Some(source.as_bytes()) {
        return Err("source_drift");
    }
    let before_pdf = pdf_output_snapshot_v994(output);
    let shadow = Shadow::new().ok_or("shadow_directory_unavailable")?;
    let trial = shadow
        .0
        .join("target")
        .join("etuschlopf_renderer_feasibility_shadow")
        .join("audit");
    fs::create_dir_all(&trial).map_err(|_| "shadow_directory_unavailable")?;
    // The approved non-Render basename keeps this disposable input outside
    // the writer trace for real Render-MusicXML changes.
    let xml = trial.join("constraint_candidate.musicxml");
    let plain = trial.join("plain.svg");
    let native = trial.join("shadow.mscx");
    let unmarked = trial.join("unmarked.svg");
    let native_pdf = trial.join("native-proof.pdf");
    let marked_native = trial.join("marked.mscx");
    let color = trial.join("marked.svg");
    let safe = |source: &Path, sink: &Path| {
        super::super::persisted_schema_validation::shadow_renderer_paths_outside_products(
            source, sink,
        )
    };
    if !safe(&xml, &plain) {
        return Err("shadow_path_unavailable");
    }
    crate::structural_write_authority::write_renderer_feasibility_shadow_candidate_authorized(
        &xml, source,
    )
    .map_err(|_| "shadow_source_unavailable")?;
    let convert = |input: &Path, output: &Path| -> Option<()> {
        // Both operands are under the exclusively created Shadow. Only the
        // already bound converter profile is used, with no fallback retries.
        if !safe(input, output) || renderer_executable_identity(&identity.executable) != fingerprint
        {
            return None;
        }
        let variant = render_command_variants(&absolute_path(input), &absolute_path(output))
            .into_iter()
            .find(|v| v.label == command.label)?;
        (run_renderer_command(&identity.executable, &variant.args, None)
            .ok()?
            .code
            == Some(0))
        .then_some(())
    };
    convert(&xml, &plain).ok_or("plain_svg_unavailable")?;
    convert(&xml, &native).ok_or("native_shadow_unavailable")?;
    let native_xml = fs::read_to_string(&native).map_err(|_| "native_shadow_unavailable")?;
    let native_preview =
        native_preview_and_pdf_proof(&native_xml, &native, &unmarked, &native_pdf, &convert)?;
    if fs::read(&native).ok().as_deref() != Some(native_xml.as_bytes()) {
        return Err("native_shadow_drift");
    }
    let marked = tag_native(&native_xml).ok_or("native_shape_binding_unavailable")?;
    // A new disposable MSCX in the same directory can read the import's
    // style sidecars without overwriting the original native shadow.
    fs::File::create_new(&marked_native)
        .and_then(|mut file| file.write_all(marked.xml.as_bytes()))
        .map_err(|_| "marked_source_unavailable")?;
    convert(&marked_native, &color).ok_or("marked_svg_unavailable")?;

    if renderer_executable_identity(&identity.executable) != fingerprint
        || fs::read(&xml).ok().as_deref() != Some(source.as_bytes())
        || fs::read(&native).ok().as_deref() != Some(native_xml.as_bytes())
        || fs::read(&marked_native).ok().as_deref() != Some(marked.xml.as_bytes())
        || fs::read(input).ok().as_deref() != Some(source.as_bytes())
        || !pdf_output_snapshots_content_equal_v1110(&before_pdf, &pdf_output_snapshot_v994(output))
    {
        return Err("bound_artifact_drift");
    }
    let plain = pages(&trial, "plain").ok_or("plain_pages_unavailable")?;
    let tagged = pages(&trial, "marked").ok_or("marked_pages_unavailable")?;
    if count_pdf_pages(output) != Some(plain.len()) {
        return Err("pdf_svg_page_count_unconfirmed");
    }
    let mut findings =
        report(&plain, &native_preview.pages, &tagged, &marked).ok_or("marked_pages_mismatch")?;
    match native_preview.proof {
        Ok(proof) => findings.push_str(&proof),
        Err(reason) => findings.push_str(&format!(
            "EngravingNativeExportProof: unavailable={reason}; authority=none\n"
        )),
    }
    Ok(findings)
}

pub(in crate::pipeline) fn observe(
    source: &str,
    input: &Path,
    output: &Path,
    render: &RenderResult,
    custom_slur_paint: bool,
) -> String {
    let result = match (render.status, custom_slur_paint) {
        (RenderStatus::Written, false) => diagnostic(source, input, output, render),
        (RenderStatus::Written, true) => Err("custom_slur_geometry_has_separate_source"),
        _ => Err("pdf_not_confirmed"),
    };
    result.unwrap_or_else(|reason| {
        format!("EngravingCollisionReadback: unavailable={reason}; authority=none\n")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SYNTHETIC_POLYPHONY: &str = r#"<score-partwise version="4.0">
      <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
      <part id="P1"><measure number="@measure@">
        <attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves>
          <clef number="1"><sign>G</sign><line>2</line></clef>
          <clef number="2"><sign>F</sign><line>4</line></clef>
        </attributes>
        <note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><tie type="start"/>
          <voice>1</voice><type>quarter</type><stem>down</stem><staff>1</staff><notations><tied type="start"/></notations></note>
        <note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><tie type="stop"/>
          <voice>1</voice><type>quarter</type><stem>down</stem><staff>1</staff><notations><tied type="stop"/></notations></note>
        <note><rest/><duration>8</duration><voice>1</voice><type>half</type><staff>1</staff></note>
        <backup><duration>16</duration></backup>
        <note><pitch><step>F</step><octave>3</octave></pitch><duration>8</duration><voice>2</voice><type>half</type><staff>2</staff></note>
        <note><pitch><step>G</step><octave>3</octave></pitch><duration>8</duration><voice>2</voice><type>half</type><staff>2</staff></note>
      </measure></part>
    </score-partwise>"#;

    #[test]
    #[ignore = "requires ETUSCHLOPF_PDF_RENDERER pointing to a MuseScore converter"]
    fn changed_native_stem_is_bound_to_the_pdf_actually_written() {
        let executable = std::env::var("ETUSCHLOPF_PDF_RENDERER").expect("MuseScore executable");
        let folder = Shadow::new().expect("disposable audit directory");
        let source = folder.0.join("synthetic.musicxml");
        let native = folder.0.join("original.mscx");
        let variant = folder.0.join("variant.mscx");
        let baseline_pdf = folder.0.join("baseline.pdf");
        let candidate_pdf = folder.0.join("candidate.pdf");
        let input_score = SYNTHETIC_POLYPHONY.replace("@measure@", &37.to_string());
        let convert = |input: &Path, output: &Path| {
            let variant = render_command_variants(&absolute_path(input), &absolute_path(output))
                .into_iter()
                .next()?;
            (run_renderer_command(&executable, &variant.args, None)
                .ok()?
                .code
                == Some(0))
            .then_some(())
        };

        fs::write(&source, &input_score).unwrap();
        convert(&source, &native).expect("native import");
        let original = fs::read_to_string(&native).unwrap();
        let document = roxmltree::Document::parse(&original).unwrap();
        let chord = document
            .descendants()
            .find(|node| node.has_tag_name("Chord"))
            .unwrap();
        let mut changed = original.clone();
        let stem_change = "<Stem><userLen>1.5</userLen></Stem>";
        if let Some(stem) = chord.children().find(|node| node.has_tag_name("Stem")) {
            changed.replace_range(stem.range(), stem_change);
        } else {
            let body_start = chord.range().start + "<Chord>".len();
            assert_eq!(&original[chord.range().start..body_start], "<Chord>");
            changed.insert_str(body_start, stem_change);
        }
        assert_eq!(stem_length_edits(&original, &changed), Some(1));
        fs::write(&variant, changed).unwrap();

        let baseline = native_preview_and_pdf_proof(
            &original,
            &native,
            &folder.0.join("baseline.svg"),
            &baseline_pdf,
            &convert,
        )
        .unwrap();
        let baseline_receipt = baseline.proof.expect("baseline preview and PDF agree");
        assert!(baseline_receipt.contains("stem_length_edits=0"));

        let corrected = native_preview_and_pdf_proof(
            &original,
            &variant,
            &folder.0.join("candidate.svg"),
            &candidate_pdf,
            &convert,
        )
        .unwrap();
        let candidate_receipt = corrected.proof.expect("changed preview and PDF agree");
        assert!(candidate_receipt.contains("stem_length_edits=1"));
        let stale_pdf = fs::read(&baseline_pdf).unwrap();
        let final_pdf = fs::read(&candidate_pdf).unwrap();
        assert_ne!(final_pdf, stale_pdf);
        assert_eq!(
            verify_strokes(&corrected.pages, &stale_pdf),
            Err("svg_pdf_stroke_mismatch")
        );
        assert_eq!(
            verify_strokes(&baseline.pages, &final_pdf),
            Err("svg_pdf_stroke_mismatch")
        );
        assert_eq!(fs::read_to_string(&source).unwrap(), input_score);
    }

    #[test]
    #[ignore = "requires ETUSCHLOPF_PDF_RENDERER pointing to a MuseScore converter"]
    fn native_stem_pair_audit_across_two_svgs() {
        use crate::pipeline::svg_notation_geometry::collision_observation::stem_pair_audit;

        let executable = std::env::var("ETUSCHLOPF_PDF_RENDERER").expect("MuseScore executable");
        let folder = Shadow::new().expect("disposable audit directory");
        let source = folder.0.join("pair.musicxml");
        let original_score = folder.0.join("pair.mscx");
        let input_score = SYNTHETIC_POLYPHONY.replace("@measure@", "1");
        let convert = |input: &Path, output: &Path| {
            let variant =
                render_command_variants(&absolute_path(input), &absolute_path(output)).remove(0);
            let run = run_renderer_command(&executable, &variant.args, None).unwrap();
            assert_eq!(run.code, Some(0), "conversion for {}", output.display());
        };
        fs::write(&source, &input_score).unwrap();
        convert(&source, &original_score);
        let original = fs::read_to_string(&original_score).unwrap();
        let document = roxmltree::Document::parse(&original).unwrap();
        let chord = document
            .descendants()
            .filter(|node| node.has_tag_name("Chord"))
            .nth(2)
            .unwrap();
        let stem_change = "<Stem><userLen>1.5</userLen></Stem>";
        let span = chord
            .children()
            .find(|node| node.has_tag_name("Stem"))
            .map_or_else(
                || {
                    let at = chord.range().start + "<Chord>".len();
                    assert_eq!(&original[chord.range().start..at], "<Chord>");
                    at..at
                },
                |stem| stem.range(),
            );
        let changed = format!("{}{stem_change}{}", &original[..span.start], &original[span.end..]);
        assert_eq!(stem_length_edits(&original, &changed), Some(1));
        let before = tag_native(&original).unwrap();
        let after = tag_native(&changed).unwrap();
        let render = |prefix: &str, native: &str, painted: &str| {
            std::array::from_fn::<_, 2, _>(|index| {
                let name = format!("{prefix}-{index}");
                let input = folder.0.join(format!("{name}.mscx"));
                let output = folder.0.join(format!("{name}.svg"));
                fs::write(&input, [native, painted][index]).unwrap();
                convert(&input, &output);
                let [page] = pages(&folder.0, &name).unwrap().try_into().unwrap();
                page
            })
        };
        let old = render("before", &original, &before.xml);
        let new = render("after", &changed, &after.xml);
        stem_pair_audit::assert_native_pairs(
            &before,
            &after,
            old.each_ref().map(String::as_str),
            new.each_ref().map(String::as_str),
        );
        assert_eq!(fs::read_to_string(&source).unwrap(), input_score);
    }

    #[test]
    #[ignore = "requires ETUSCHLOPF_PDF_RENDERER pointing to a MuseScore converter"]
    fn musicxml_stem_source_anchor_across_two_imports() {
        use crate::audit_only::renderer_system_break_readback::pdf_source_anchor_audit;
        use crate::pipeline::svg_notation_geometry::collision_observation::source_anchor_audit;

        let executable = std::env::var("ETUSCHLOPF_PDF_RENDERER").expect("MuseScore executable");
        let folder = Shadow::new().expect("disposable audit directory");
        let convert = |input: &Path, output: &Path| {
            let option =
                render_command_variants(&absolute_path(input), &absolute_path(output)).remove(0);
            let run = run_renderer_command(&executable, &option.args, None).unwrap();
            assert_eq!(run.code, Some(0), "conversion for {}", output.display());
        };
        let draw = |name: &str, direction: &str, first_measure: u16| {
            let dir = folder.0.join(name);
            fs::create_dir(&dir).unwrap();
            let plain = dir.join("plain.musicxml");
            let painted = dir.join("painted.musicxml");
            let native = dir.join("painted.mscx");
            fs::write(&plain, source_anchor_audit::score(direction, first_measure, false))
                .unwrap();
            let marked_source = source_anchor_audit::score(direction, first_measure, true);
            fs::write(&painted, &marked_source).unwrap();
            convert(&plain, &dir.join("plain.svg"));
            convert(&painted, &dir.join("painted.svg"));
            convert(&painted, &native);
            convert(&native, &dir.join("from-native.svg"));
            assert_eq!(fs::read_to_string(&painted).unwrap(), marked_source);
            let [uncolored] = pages(&dir, "plain").unwrap().try_into().unwrap();
            let [colored] = pages(&dir, "painted").unwrap().try_into().unwrap();
            let [native_ink] = pages(&dir, "from-native").unwrap().try_into().unwrap();
            let plain_pdf = dir.join("plain.pdf");
            let painted_pdf = dir.join("painted.pdf");
            convert(&plain, &plain_pdf);
            convert(&painted, &painted_pdf);
            let plain_pdf = fs::read(plain_pdf).unwrap();
            let painted_pdf = fs::read(painted_pdf).unwrap();
            pdf_source_anchor_audit::probe(&uncolored, &colored, &plain_pdf, &painted_pdf)
                .expect("regular PDF retains the unmarked and source-anchored stem and tie contours");
            assert!(pdf_source_anchor_audit::probe(&uncolored, &colored, &plain_pdf, &plain_pdf).is_err());
            [fs::read_to_string(native).unwrap(), uncolored, colored, native_ink]
        };
        let before = draw("down", "down", 11);
        let after = draw("up", "up", 34);
        source_anchor_audit::assert_source_pair(
            before.each_ref().map(String::as_str),
            after.each_ref().map(String::as_str),
        );
    }

    #[test]
    #[ignore = "requires ETUSCHLOPF_PDF_FOREGROUND_INPUT pointing to an exported PDF"]
    fn exported_pdf_foreground_text_is_observed() {
        use crate::audit_only::renderer_system_break_readback::pdf_source_anchor_audit::probe_pdf_foreground;

        let input = std::env::var_os("ETUSCHLOPF_PDF_FOREGROUND_INPUT").expect("exported PDF path");
        let input = fs::read(input).expect("read exported PDF");
        let pages = probe_pdf_foreground(&input).expect("qualified PDF text paint");
        assert!(pages.iter().any(|page| !page.is_empty()), "PDF has no proven text ink");
        for (index, page) in pages.iter().enumerate() {
            println!("PDF page {}: {} nonempty glyph shows", index + 1, page.len());
            for glyph in page.iter().filter(|glyph| glyph.marked_actual_text) {
                println!("  ActualText glyph {} CID {} in page points: {:?}", glyph.font, glyph.cid, glyph.bounds);
            }
        }
    }

    #[test]
    #[ignore = "requires ETUSCHLOPF_PDF_RENDERER pointing to a MuseScore converter"]
    fn synthetic_two_measure_tied_stem_previews_are_bound_to_pdf() -> anyhow::Result<()> {
        use super::tied_stem_render_review::{PdfReviewFrame, review_tied_stem_previews};

        struct SyntheticDrawnFrame {
            source: String,
            marked_source: String,
            pdf: Vec<u8>,
            marked_pdf: Vec<u8>,
            svg: Vec<String>,
        }

        impl SyntheticDrawnFrame {
            fn preview_frame(&self) -> PdfReviewFrame<'_> {
                PdfReviewFrame { source: &self.source, marked_source: &self.marked_source,
                    pdf: &self.pdf, marked_pdf: &self.marked_pdf, svg: &self.svg }
            }
        }

        let executable = std::env::var("ETUSCHLOPF_PDF_RENDERER")?;
        let shadow = Shadow::new().expect("disposable synthetic preview directory");
        let paint_note = |side: &str, first: bool, painted: bool| {
            let attributes = |code| if painted { format!(r##" color="#{code}""##) } else { String::new() };
            let (tie, head_rgb, stem_rgb) = if first { ("start", "310001", "320001") }
                else { ("stop", "310002", "320002") };
            let (head, stem) = (attributes(head_rgb), attributes(stem_rgb));
            let bow = if first { attributes("330001") } else { String::new() };
            format!(r#"<note{head}><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration>
                       <tie type="{tie}"/><voice>1</voice><type>quarter</type><stem{stem}>{side}</stem>
                       <notations><tied type="{tie}"{bow}/></notations></note>"#)
        };
        let score = |side: &str, painted| {
            let first = paint_note(side, true, painted);
            let second = paint_note(side, false, painted);
            let (a, b) = (1, 2);
            format!(r#"<score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
            <part id="P1"><measure number="{a}"><attributes><divisions>1</divisions>
            <time><beats>1</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef>
            </attributes>{first}</measure><measure number="{b}">{second}</measure></part></score-partwise>"#)
        };
        let draw_frame = |side: &str| -> anyhow::Result<SyntheticDrawnFrame> {
            let folder = shadow.0.join(side);
            fs::create_dir(&folder)?;
            let source = score(side, false);
            let marked_source = score(side, true);
            let plain_path = folder.join("plain.musicxml");
            let marked_path = folder.join("marked.musicxml");
            fs::write(&plain_path, &source)?;
            fs::write(&marked_path, &marked_source)?;
            for (input, name) in [(&plain_path, "plain.pdf"), (&plain_path, "plain.svg"), (&marked_path, "marked.pdf")] {
                let output = folder.join(name);
                let option = render_command_variants(&absolute_path(input), &absolute_path(&output)).remove(0);
                let run = run_renderer_command(&executable, &option.args, None)?;
                anyhow::ensure!(run.code == Some(0), "synthetic preview render of {name} failed");
            }
            Ok(SyntheticDrawnFrame { source, marked_source, pdf: fs::read(folder.join("plain.pdf"))?,
                marked_pdf: fs::read(folder.join("marked.pdf"))?, svg: pages(&folder, "plain").expect("synthetic preview pages") })
        };
        let baseline = draw_frame("up")?;
        let candidate = draw_frame("down")?;
        let marks = "group\tfrom_measure\tfrom_note\tto_measure\tto_note\tfrom_head\tfrom_stem\ttie\tto_head\tto_stem\n\
                     1\t1\t0\t2\t0\t#310001\t#320001\t#330001\t#310002\t#320002";
        let report = review_tied_stem_previews(&baseline.preview_frame(), &[(1, candidate.preview_frame())],
            &candidate.preview_frame(), marks)?;
        assert!(report.contains("owned=Proven"), "{report}");
        assert!(report.contains("stem_changes=2 scoped=true"), "{report}");
        Ok(())
    }

    #[test]
    #[ignore = "requires ETUSCHLOPF_PDF_RENDERER pointing to a MuseScore converter"]
    fn tied_stem_correction_renders_across_page_breaks() {
        use crate::pipeline::notation::project_tied_stem_chain_probe;
        use crate::pipeline::svg_notation_geometry::{
            Side,
            collision_observation::{
                pdf_head_binding::{Attachment, prove_bound_heads},
                source_anchor_audit::split_tie::{SplitLayout, SvgFrame, assert_split_transition},
            },
        };

        let executable = std::env::var("ETUSCHLOPF_PDF_RENDERER").expect("MuseScore executable");
        let folder = Shadow::new().expect("disposable audit directory");
        let convert = |input: &Path, output: &Path| {
            let option = render_command_variants(&absolute_path(input), &absolute_path(output)).remove(0);
            let run = run_renderer_command(&executable, &option.args, None).unwrap();
            assert_eq!(run.code, Some(0), "conversion for {}", output.display());
        };
        let attachments = |side| [
            Attachment { head: [0x31, 0x10, 1], stem: [0x31, 0x10, 2], side },
            Attachment { head: [0x31, 0x10, 3], stem: [0x31, 0x10, 4], side },
            Attachment { head: [0x31, 0x10, 5], stem: [0x31, 0x10, 6], side: Side::Below },
        ];
        let draw = |name: &str, score: &str, painted: &str, side| {
            let dir = folder.0.join(name);
            fs::create_dir(&dir).unwrap();
            let plain_xml = dir.join("plain.musicxml");
            let marked_xml = dir.join("painted.musicxml");
            let native = dir.join("painted.mscx");
            fs::write(&plain_xml, score).unwrap();
            fs::write(&marked_xml, painted).unwrap();
            for (input, target) in [
                (&plain_xml, "plain.svg"),
                (&marked_xml, "painted.svg"),
                (&marked_xml, "painted.mscx"),
                (&native, "reread.svg"),
                (&plain_xml, "plain.pdf"),
                (&marked_xml, "painted.pdf"),
            ] {
                convert(input, &dir.join(target));
            }
            assert_eq!(fs::read_to_string(&plain_xml).unwrap(), score);
            assert_eq!(fs::read_to_string(&marked_xml).unwrap(), painted);
            let plain = pages(&dir, "plain").unwrap();
            let painted = pages(&dir, "painted").unwrap();
            prove_bound_heads(
                &plain.iter().map(String::as_str).collect::<Vec<_>>(),
                &painted.iter().map(String::as_str).collect::<Vec<_>>(),
                &fs::read(dir.join("plain.pdf")).unwrap(),
                &fs::read(dir.join("painted.pdf")).unwrap(),
                &attachments(side),
            )
            .expect("PDF retains embedded head contours, source-bound stem attachments and split ties");
            SvgFrame {
                native: fs::read_to_string(native).unwrap(),
                plain,
                painted,
                reread: pages(&dir, "reread").unwrap(),
            }
        };

        let system = SplitLayout::System;
        let page = SplitLayout::Page;
        let system = draw("system", &system.score("up", 17, false),
            &system.score("up", 17, true), Side::Above);
        let baseline = page.score("up", 37, false);
        let authored_ranges = |score: &str| {
            let xml = roxmltree::Document::parse(score).unwrap();
            let tied: Vec<_> = xml
                .descendants()
                .filter(|note| note.has_tag_name("note") && note.children().any(|n| n.has_tag_name("tie")))
                .map(|note| note.range())
                .collect();
            assert_eq!(tied.len(), 2);
            tied
        };
        let candidate = project_tied_stem_chain_probe(&baseline, &[authored_ranges(&baseline)])
            .unwrap().expect("the final stem planner corrects the authored chain");
        assert_eq!(candidate, page.score("down", 37, false));
        assert!(project_tied_stem_chain_probe(&candidate, &[authored_ranges(&candidate)]).unwrap().is_none());
        let before = draw("before", &baseline, &page.score("up", 37, true), Side::Above);
        let after = draw("after", &candidate, &page.score("down", 37, true), Side::Below);
        assert_split_transition(&system, &before, &after);
        let stale_pdf = fs::read(folder.0.join("before/plain.pdf")).unwrap();
        let marked_candidate_pdf = fs::read(folder.0.join("after/painted.pdf")).unwrap();
        let regular = after.plain.iter().map(String::as_str).collect::<Vec<_>>();
        let marked = after.painted.iter().map(String::as_str).collect::<Vec<_>>();
        assert!(prove_bound_heads(&regular, &marked, &stale_pdf, &marked_candidate_pdf,
            &attachments(Side::Below)).is_err());
        let final_pdf = fs::read(folder.0.join("after/plain.pdf")).unwrap();
        let wrong_direction = prove_bound_heads(&regular, &marked, &final_pdf, &marked_candidate_pdf,
            &attachments(Side::Above)).unwrap_err();
        assert!(wrong_direction.to_string().contains("intended stem attachment"));
    }
}
