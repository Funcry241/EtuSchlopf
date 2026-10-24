// v0.10.725 FoundationIoRealModuleSplitNoBehaviorChange.
// Real crate-root module split from the former mechanical IO grouping.
// Preserves original module path `source` and behavior; no wrapper/include shell.

// --- mechanical module boundary: former src/source.rs ---
use anyhow::anyhow;
use std::path::{Path, PathBuf};

const OUTPUT_SUFFIX_MUSICXML: &str = ".musicxml";
const OUTPUT_SUFFIX_PDF: &str = ".pdf";
const OUTPUT_SUFFIX_REPORT: &str = ".result.txt";
const OUTPUT_SUFFIX_RENDER_MUSICXML: &str = ".render.musicxml";

#[derive(Debug, Clone)]
pub struct SourcePlan {
    pub input_label: String,
    pub expected_wav: PathBuf,
    pub expected_midi: PathBuf,
    pub report_out: PathBuf,
    pub musicxml_out: PathBuf,
    pub render_musicxml_out: PathBuf,
    pub pdf_out: PathBuf,
}

pub fn resolve_source_plan(
    raw_input: Option<&Path>,
    report_out: Option<&Path>,
    musicxml_out: Option<&Path>,
    pdf_out: Option<&Path>,
) -> anyhow::Result<SourcePlan> {
    let input = raw_input.ok_or_else(|| anyhow!("Projektname fehlt. Einzelaufruf: run.cmd \"Etüde Nr. 6\". Ordner-Batch: run.cmd --all"))?;

    if is_score_path(input) {
        return Err(anyhow!(
            "MusicXML/MXL ist kein gültiger Input mehr. Nutze den Projektaufruf, z. B. run.cmd \"Etüde Nr. 6\", oder starte run.cmd --all für alle passenden WAV/MIDI-Paare. Gleichnamige .wav- und .mid-Dateien sind die vollständigen Projekteingänge."
        ));
    }

    let (dir, project_name) = split_project_input(input)?;
    let output_stem = project_file_stem(&project_name);
    let expected_wav = dir.join(format!("{project_name}.wav"));
    let expected_midi = dir.join(format!("{project_name}.mid"));

    // v0.1.50: StrictPromptFilenames. Alle Standardausgaben verwenden exakt
    // den Projekt-/Promptnamen als Dateistamm. EtuSchlopf dichtet keine
    // Bezeichnungen wie "Klavierfassung", "EtuSchlopf" oder "Status" mehr
    // in Standarddateinamen hinein. Nur technisch notwendige Endungen bleiben.
    let derived_report_out = output_or_project_default(&dir, report_out, &format!("{output_stem}{OUTPUT_SUFFIX_REPORT}"));
    let derived_musicxml_out = output_or_project_default(&dir, musicxml_out, &format!("{output_stem}{OUTPUT_SUFFIX_MUSICXML}"));
    let derived_pdf_out = output_or_project_default(&dir, pdf_out, &format!("{output_stem}{OUTPUT_SUFFIX_PDF}"));
    let render_musicxml_out = dir.join(format!("{output_stem}{OUTPUT_SUFFIX_RENDER_MUSICXML}"));

    Ok(SourcePlan {
        input_label: project_name,
        expected_wav,
        expected_midi,
        report_out: derived_report_out,
        musicxml_out: derived_musicxml_out,
        render_musicxml_out,
        pdf_out: derived_pdf_out,
    })
}

fn is_score_path(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| matches!(ext.to_ascii_lowercase().as_str(), "musicxml" | "xml" | "mxl"))
}

fn split_project_input(input: &Path) -> anyhow::Result<(PathBuf, String)> {
    let dir = input
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
    let name = input
        .file_name()
        .and_then(|s| s.to_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow!("Projektname fehlt. Einzelaufruf: run.cmd \"Etüde Nr. 6\". Ordner-Batch: run.cmd --all"))?;
    Ok((dir, name.to_string()))
}

fn output_or_project_default(dir: &Path, explicit: Option<&Path>, default_name: &str) -> PathBuf {
    explicit
        .map_or_else(|| dir.join(default_name), Path::to_path_buf)
}

fn project_file_stem(project_name: &str) -> String {
    let mut out = String::new();

    for ch in project_name.trim().chars() {
        if ch.is_control() || matches!(ch, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*') {
            out.push('_');
        } else {
            out.push(ch);
        }
    }

    let out = out.trim_matches(|ch: char| ch.is_whitespace() || ch == '.').to_string();
    if out.is_empty() {
        "EtuSchlopf Projekt".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::project_file_stem;

    #[test]
    fn derives_project_stem_from_etude_name() {
        assert_eq!(project_file_stem("Synthetic Project Nr. 6"), "Synthetic Project Nr. 6");
        assert_eq!(project_file_stem("  Chopin - Étude, Op. 10/1  "), "Chopin - Étude, Op. 10_1");
    }
}
