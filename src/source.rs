// v0.10.725 FoundationIoRealModuleSplitNoBehaviorChange.
// Real crate-root module split from the former mechanical IO grouping.
// Preserves original module path `source` and behavior; no wrapper/include shell.

// --- mechanical module boundary: former src/source.rs ---
use anyhow::{anyhow, Context};
use std::fs;
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
    // Keep the public project identity and output names identical to the caller's
    // argument. The launcher uses that same identity for its existing backup set.
    let input_label = input.file_name().and_then(|name| name.to_str()).unwrap_or(&project_name).trim().to_string();
    let output_stem = project_file_stem(&input_label);
    let expected_wav = if input.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("wav")) {
        input.to_path_buf()
    } else {
        resolve_member(&dir, &project_name, &["wav"], "WAV")?
    };
    let explicit_midi = input.extension().and_then(|ext| ext.to_str())
        .is_some_and(|ext| matches!(ext.to_ascii_lowercase().as_str(), "mid" | "midi"));
    let expected_midi = if explicit_midi {
        input.to_path_buf()
    } else {
        resolve_member(&dir, &project_name, &["mid", "midi"], "MIDI")?
    };

    // v0.1.50: StrictPromptFilenames. Alle Standardausgaben verwenden exakt
    // den Projekt-/Promptnamen als Dateistamm. EtuSchlopf dichtet keine
    // Bezeichnungen wie "Klavierfassung", "EtuSchlopf" oder "Status" mehr
    // in Standarddateinamen hinein. Nur technisch notwendige Endungen bleiben.
    let derived_report_out = output_or_project_default(&dir, report_out, &format!("{output_stem}{OUTPUT_SUFFIX_REPORT}"));
    let derived_musicxml_out = output_or_project_default(&dir, musicxml_out, &format!("{output_stem}{OUTPUT_SUFFIX_MUSICXML}"));
    let derived_pdf_out = output_or_project_default(&dir, pdf_out, &format!("{output_stem}{OUTPUT_SUFFIX_PDF}"));
    let render_musicxml_out = dir.join(format!("{output_stem}{OUTPUT_SUFFIX_RENDER_MUSICXML}"));

    Ok(SourcePlan {
        input_label,
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
    let name = if input.extension().and_then(|ext| ext.to_str())
        .is_some_and(|ext| matches!(ext.to_ascii_lowercase().as_str(), "mid" | "midi" | "wav"))
    {
        input.file_stem().and_then(|stem| stem.to_str()).unwrap_or(name).trim()
    } else {
        name
    };
    if name.is_empty() {
        return Err(anyhow!("Projektname vor der Dateiendung fehlt: {}", input.display()));
    }
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

fn resolve_member(dir: &Path, stem: &str, extensions: &[&str], kind: &str) -> anyhow::Result<PathBuf> {
    let mut found = Vec::new();
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                let path = entry?.path();
                if !path.file_stem().and_then(|value| value.to_str()).is_some_and(|name| if cfg!(windows) { name.eq_ignore_ascii_case(stem) } else { name == stem }) {
                    continue;
                }
                if path.extension().and_then(|ext| ext.to_str())
                    .is_some_and(|ext| extensions.iter().any(|candidate| ext.eq_ignore_ascii_case(candidate)))
                {
                    found.push(path);
                }
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error).with_context(|| format!("Eingabeordner nicht lesbar: {}", dir.display())),
    }
    found.sort();
    if found.len() > 1 {
        return Err(anyhow!("Mehrere {kind}-Dateien fuer '{stem}' gefunden: {}. Waehle die MIDI-Datei ausdruecklich oder entferne die Mehrdeutigkeit.", found.iter().map(|path| path.display().to_string()).collect::<Vec<_>>().join(", ")));
    }
    Ok(found.pop().unwrap_or_else(|| dir.join(format!("{stem}.{}", extensions[0]))))
}

impl SourcePlan {
    pub fn output_paths(&self) -> [PathBuf; 4] {
        [self.musicxml_out.clone(), self.render_musicxml_out.clone(), self.pdf_out.clone(), self.report_out.clone()]
    }
}

/// Read-only admission of filesystem targets. No output is opened for writing.
/// Equal contents are a conservative input-protection refusal, not proof of file identity.
pub fn validate_file_targets(inputs: &[PathBuf], outputs: &[PathBuf]) -> anyhow::Result<()> {
    let mut source_paths = Vec::new();
    for input in inputs {
        if !input.is_file() {
            return Err(anyhow!("Eingabedatei fehlt oder ist keine regulaere Datei: {}. Bitte eine lesbare MIDI-/WAV-Datei bereitstellen.", input.display()));
        }
        source_paths.push(fs::canonicalize(input).with_context(|| format!("Eingabepfad nicht aufloesbar: {}", input.display()))?);
    }
    let mut destinations = Vec::<PathBuf>::new();
    for output in outputs {
        if output.extension().and_then(|ext| ext.to_str())
            .is_some_and(|ext| matches!(ext.to_ascii_lowercase().as_str(), "mid" | "midi" | "wav"))
        {
            return Err(anyhow!("Ausgabe darf keine MIDI-/WAV-Eingabe ersetzen: {}. Bitte einen eigenen Ausgabepfad verwenden.", output.display()));
        }
        let parent = output.parent().filter(|path| !path.as_os_str().is_empty()).unwrap_or(Path::new("."));
        let directory = fs::canonicalize(parent).with_context(|| format!("Ausgabeordner fehlt oder ist nicht zugaenglich: {}. Bitte einen vorhandenen Ausgabeordner waehlen.", parent.display()))?;
        if !directory.is_dir() {
            return Err(anyhow!("Ausgabeordner ist kein Verzeichnis: {}", parent.display()));
        }
        let name = output.file_name().ok_or_else(|| anyhow!("Ausgabedateiname fehlt: {}", output.display()))?;
        let destination = directory.join(name);
        match fs::symlink_metadata(output) {
            Ok(metadata) => {
                if !metadata.is_file() || metadata.file_type().is_symlink() {
                    return Err(anyhow!("Ausgabeziel ist keine regulaere unverknuepfte Datei: {}. Bitte einen eigenen Dateipfad waehlen.", output.display()));
                }
                if metadata.permissions().readonly() {
                    return Err(anyhow!("Ausgabedatei ist schreibgeschuetzt: {}. Bitte Schreibschutz entfernen oder einen anderen Ausgabepfad waehlen.", output.display()));
                }
                let canonical = fs::canonicalize(output)?;
                for input in &source_paths {
                    if same_path(&canonical, input) || (metadata.len() == fs::metadata(input)?.len()
                        && fs::read(output)? == fs::read(input)?)
                    {
                        return Err(anyhow!("Ausgabe kollidiert mit einer Eingabe oder hat identische Eingabebytes: {} / {}. Zur Sicherheit wird nichts geloescht; bitte einen anderen Ausgabepfad waehlen.", output.display(), input.display()));
                    }
                }
                let mut file = fs::File::open(output)?;
                let mut prefix = [0_u8; 12];
                use std::io::Read;
                let count = file.read(&mut prefix)?;
                if (count >= 4 && &prefix[..4] == b"MThd") || (count == 12 && &prefix[..4] == b"RIFF" && &prefix[8..] == b"WAVE") {
                    return Err(anyhow!("Ausgabeziel enthaelt MIDI-/WAV-Daten: {}. Die Datei bleibt erhalten; bitte einen eigenen Ausgabepfad waehlen.", output.display()));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error).with_context(|| format!("Ausgabeziel nicht pruefbar: {}", output.display())),
        }
        if source_paths.iter().any(|input| same_path(input, &destination)) {
            return Err(anyhow!("Ausgabe zeigt auf eine Eingabedatei: {}", output.display()));
        }
        if destinations.iter().any(|previous| same_path(previous, &destination)) {
            return Err(anyhow!("Zwei Ausgaben zeigen auf dieselbe Datei: {}. Bitte getrennte Ausgabepfade waehlen.", output.display()));
        }
        destinations.push(destination);
    }
    Ok(())
}

fn same_path(left: &Path, right: &Path) -> bool {
    if cfg!(windows) {
        left.to_string_lossy().to_lowercase() == right.to_string_lossy().to_lowercase()
    } else {
        left == right
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

#[cfg(test)]
mod input_admission_tests {
    use super::*;

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!("etuschlopf-input-admission-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
        fn file(&self, name: &str, bytes: &[u8]) -> PathBuf {
            let path = self.0.join(name);
            fs::write(&path, bytes).unwrap();
            path
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
    }

    #[test]
    fn file_arguments_resolve_sources_while_preserving_the_callers_output_identity() {
        let fixture = Fixture::new();
        fixture.file("Synthetic Ü.midi", b"input");
        fixture.file("Synthetic Ü.WAV", b"audio");
        for name in ["Synthetic Ü", "Synthetic Ü.midi", "Synthetic Ü.WAV"] {
            let plan = resolve_source_plan(Some(&fixture.0.join(name)), None, None, None).unwrap();
            assert_eq!(plan.input_label, name);
            assert_eq!(plan.expected_midi, fixture.0.join("Synthetic Ü.midi"));
            assert_eq!(plan.expected_wav, fixture.0.join("Synthetic Ü.WAV"));
            assert_eq!(plan.pdf_out, fixture.0.join(format!("{name}.pdf")));
        }
    }

    #[test]
    fn ambiguous_midi_pair_requires_explicit_file_choice() {
        let fixture = Fixture::new();
        fixture.file("Synthetic.mid", b"first");
        fixture.file("Synthetic.midi", b"second");
        assert!(resolve_source_plan(Some(&fixture.0.join("Synthetic")), None, None, None).is_err());
        for name in ["Synthetic.mid", "Synthetic.midi"] {
            let plan = resolve_source_plan(Some(&fixture.0.join(name)), None, None, None).unwrap();
            assert_eq!(plan.expected_midi, fixture.0.join(name));
        }
    }

    #[test]
    fn admission_refuses_input_aliases_and_duplicate_destinations_without_mutation() {
        let fixture = Fixture::new();
        let input = fixture.file("Synthetic.mid", b"input bytes");
        let existing = fixture.file("previous.pdf", b"previous PDF");
        fs::create_dir(fixture.0.join("nested")).unwrap();
        let cases = [
            vec![input.clone()],
            vec![existing.clone(), fixture.0.join("nested/../previous.pdf")],
            vec![fixture.0.join("Synthetic.WAV")],
        ];
        for outputs in cases {
            assert!(validate_file_targets(std::slice::from_ref(&input), &outputs).is_err());
            assert_eq!(fs::read(&input).unwrap(), b"input bytes");
            assert_eq!(fs::read(&existing).unwrap(), b"previous PDF");
        }
    }

    #[test]
    fn admission_refuses_a_hard_link_to_input_and_accepts_independent_equal_outputs() {
        let fixture = Fixture::new();
        let input = fixture.file("Synthetic.mid", b"source bytes");
        let alias = fixture.0.join("alias.pdf");
        fs::hard_link(&input, &alias).unwrap();
        assert!(validate_file_targets(std::slice::from_ref(&input), &[alias]).is_err());
        assert_eq!(fs::read(&input).unwrap(), b"source bytes");
        let first = fixture.file("first.musicxml", b"equal old output");
        let second = fixture.file("second.musicxml", b"equal old output");
        validate_file_targets(&[input], &[first, second]).unwrap();
    }

    #[test]
    fn independent_admission_survives_renaming_relocation_and_duplication() {
        let fixture = Fixture::new();
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for name in ["First", "Relocated Ü"] {
            let dir = fixture.0.join(name);
            fs::create_dir(&dir).unwrap();
            let input = dir.join(format!("{name}.mid"));
            let output = dir.join(format!("{name}.pdf"));
            fs::write(&input, name.as_bytes()).unwrap();
            fs::write(&output, b"previous valid output").unwrap();
            validate_file_targets(std::slice::from_ref(&input), std::slice::from_ref(&output)).unwrap();
            inputs.push(input);
            outputs.push(output);
        }
        validate_file_targets(&inputs, &outputs).unwrap();
        for output in &outputs {
            assert_eq!(fs::read(output).unwrap(), b"previous valid output");
        }
        let cross_project_alias = inputs[1].clone();
        assert!(validate_file_targets(&inputs, &[outputs[0].clone(), cross_project_alias]).is_err());
        for (input, expected) in inputs.iter().zip(["First", "Relocated Ü"]) {
            assert_eq!(fs::read(input).unwrap(), expected.as_bytes());
        }
    }

    #[test]
    fn admission_preserves_missing_inputs_directories_and_disguised_source_data() {
        let fixture = Fixture::new();
        let input = fixture.file("Synthetic.mid", b"source bytes");
        let existing = fixture.file("old.pdf", b"old output");
        assert!(validate_file_targets(&[fixture.0.join("missing.mid")], std::slice::from_ref(&existing)).is_err());
        assert!(validate_file_targets(std::slice::from_ref(&input), std::slice::from_ref(&fixture.0)).is_err());
        assert!(validate_file_targets(std::slice::from_ref(&input), &[fixture.0.join("missing/output.pdf")]).is_err());
        let disguised = fixture.file("foreign.pdf", b"MThd unrelated MIDI data");
        assert!(validate_file_targets(&[input], std::slice::from_ref(&disguised)).is_err());
        assert_eq!(fs::read(disguised).unwrap(), b"MThd unrelated MIDI data");
        assert_eq!(fs::read(existing).unwrap(), b"old output");
    }

    #[cfg(unix)]
    #[test]
    fn admission_refuses_symbolic_output_links() {
        let fixture = Fixture::new();
        let input = fixture.file("Synthetic.mid", b"source bytes");
        let other = fixture.file("other.pdf", b"old output");
        let alias = fixture.0.join("alias.pdf");
        std::os::unix::fs::symlink(&other, &alias).unwrap();
        assert!(validate_file_targets(&[input], &[alias]).is_err());
        assert_eq!(fs::read(other).unwrap(), b"old output");
    }
}
