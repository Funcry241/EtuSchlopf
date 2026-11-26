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
}
