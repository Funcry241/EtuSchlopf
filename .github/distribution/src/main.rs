//! Distribution bootstrap: private runtime extraction and fresh per-run input copies.
//! Musical decoding and writing remain exclusively in the embedded accepted engine.
#![forbid(unsafe_code)]
use std::{env, fs, io::{self, Write}, path::{Path, PathBuf}, process::{Command, ExitCode}, time::{SystemTime, UNIX_EPOCH}};

const BUNDLE: &[u8] = include_bytes!(env!("ETUSCHLOPF_BUNDLE_ZIP"));

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Runtime(PathBuf);
impl Drop for Runtime {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}

fn fresh_directory(parent: &Path, prefix: &str) -> Result<PathBuf> {
    fs::create_dir_all(parent)?;
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    for attempt in 0..1000 {
        let p = parent.join(format!("{prefix}-{stamp}-{}-{attempt}", std::process::id()));
        match fs::create_dir(&p) {
            Ok(()) => return Ok(p),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.into()),
        }
    }
    Err("Ein neuer Arbeitsordner konnte nicht angelegt werden.".into())
}

fn runtime() -> Result<Runtime> {
    let dir = Runtime(fresh_directory(&env::temp_dir(), "EtuSchlopf")?);
    let mut zip = zip::ZipArchive::new(io::Cursor::new(BUNDLE))?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let relative = entry.enclosed_name().ok_or("Ungueltiger Paketpfad")?;
        let path = dir.0.join(relative);
        if entry.is_dir() { fs::create_dir_all(path)?; }
        else {
            fs::create_dir_all(path.parent().ok_or("Paketordner fehlt")?)?;
            let mut output = fs::OpenOptions::new().write(true).create_new(true).open(path)?;
            io::copy(&mut entry, &mut output)?;
        }
    }
    Ok(dir)
}

fn midi_pair(input: &Path) -> Result<(PathBuf, PathBuf, String)> {
    let full = if input.is_absolute() { input.to_path_buf() } else { env::current_dir()?.join(input) };
    let parent = full.parent().ok_or("Eingabeordner fehlt")?;
    let stem = full.file_stem().and_then(|s| s.to_str()).ok_or("Dateiname fehlt")?.to_string();
    let explicit = full.extension().and_then(|s| s.to_str()).is_some_and(|s| s.eq_ignore_ascii_case("mid") || s.eq_ignore_ascii_case("midi"));
    if !explicit { return Err("Bitte eine MIDI-Datei mit der Endung .mid oder .midi waehlen.".into()); }
    if !full.is_file() { return Err(format!("MIDI-Datei fehlt: {}", full.display()).into()); }
    let mut wavs = Vec::new();
    for item in fs::read_dir(parent)? {
        let p = item?.path();
        if p.is_file() && p.file_stem().and_then(|s| s.to_str()).is_some_and(|s| s.eq_ignore_ascii_case(&stem))
            && p.extension().and_then(|s| s.to_str()).is_some_and(|s| s.eq_ignore_ascii_case("wav")) {
            wavs.push(p);
        }
    }
    if wavs.len() != 1 { return Err(format!("Zu '{}' wird genau eine gleichnamige WAV-Datei benoetigt; gefunden: {}.", full.display(), wavs.len()).into()); }
    Ok((full, wavs.remove(0), stem))
}

fn run(args: &[String]) -> Result<i32> {
    if args.len() == 1 && matches!(args[0].as_str(), "--help" | "-h") {
        println!("EtuSchlopf fuer Windows\n\nZiehe deine .mid- oder .midi-Datei auf diese EXE.\nEine gleichnamige WAV-Datei muss daneben liegen.\nOhne Argument fragt das Programm nach dem Dateipfad.\n\nEtuSchlopf.exe \"Mein Stueck.mid\" [--no-pdf]\n--no-pdf erzeugt die Notendateien ohne PDF.\n--version zeigt die Programmversion.\nJeder Lauf legt einen neuen Ordner unter EtuSchlopf-Ergebnisse an.");
        return Ok(0);
    }
    if args.len() == 1 && args[0] == "--version" {
        let rt = runtime()?;
        return Ok(Command::new(rt.0.join("etuschlopf_engine.exe")).current_dir(&rt.0).arg(&args[0]).status()?.code().unwrap_or(1));
    }
    if args.len() > 2 || (args.len() == 2 && args[1] != "--no-pdf") {
        return Err("Aufruf: EtuSchlopf.exe \"Mein Stueck.mid\" [--no-pdf]".into());
    }
    let rt = runtime()?;
    let renderer_probe = Command::new(rt.0.join("etuschlopf_engine.exe"))
        .current_dir(&rt.0).arg("--renderer-available").output();
    let renderer_missing = match renderer_probe {
        Ok(probe) if probe.status.code() == Some(0) => false,
        Ok(probe) if probe.status.code() == Some(3) => true,
        _ => {
            eprintln!("Warnung: MuseScore-Abfrage fehlgeschlagen. Der normale Lauf wird fortgesetzt.");
            false
        }
    };
    if renderer_missing {
        println!("MuseScore ist nicht installiert. Die Notendateien werden trotzdem erstellt.\nFür eine PDF installiere MuseScore kostenlos: https://musescore.org/de/download\n");
    }
    let input = if args.is_empty() {
        println!("EtuSchlopf - aus MIDI und WAV zu einer Klavierfassung");
        println!("Du kannst deine MIDI-Datei auch direkt auf diese EXE ziehen.");
        print!("Pfad zur MIDI-Datei: "); io::stdout().flush()?;
        let mut s = String::new();io::stdin().read_line(&mut s)?;s.trim().trim_matches('"').to_string()
    } else { args[0].clone() };
    let (midi, wav, stem) = midi_pair(Path::new(&input))?;
    let output_root = midi.parent().ok_or("Eingabeordner fehlt")?.join("EtuSchlopf-Ergebnisse");
    let output = fresh_directory(&output_root, "Lauf")?;
    let project = output.join(&stem);
    fs::copy(&midi, output.join(format!("{stem}.mid")))?;
    fs::copy(&wav, output.join(format!("{stem}.wav")))?;
    println!("Ergebnisse dieses Laufs: {}", output.display());
    println!("Deine Originaldateien und bisherige Ergebnisordner bleiben erhalten.");
    let mut child = Command::new(rt.0.join("etuschlopf_engine.exe"));
    child.current_dir(&rt.0).arg(&project);
    let internal_work = rt.0.join("work");
    fs::create_dir(&internal_work)?;
    child.arg("--internal-work-dir").arg(&internal_work);
    if renderer_missing || args.get(1).is_some_and(|s| s == "--no-pdf") { child.arg("--no-pdf"); }
    let status = child.status()?.code().unwrap_or(1);
    println!("\nErgebnisordner: {}", output.display());
    if status == 0 {
        if output.join(format!("{stem}.pdf")).is_file() {
            println!("Fertig. Öffne die PDF oder die Datei mit der Endung .render.musicxml in MuseScore.");
        } else if renderer_missing {
            println!("Fertig. Deine Notendateien liegen im Ergebnisordner.\nZum Ansehen installiere MuseScore kostenlos: https://musescore.org/de/download\nÖffne danach die Datei mit der Endung .render.musicxml in MuseScore.");
        } else {
            println!("Fertig. Öffne die Datei mit der Endung .render.musicxml in MuseScore.");
        }
    }
    else { eprintln!("Der Lauf ist fehlgeschlagen. Die Meldung oben beschreibt die Ursache; fruehere Ergebnisse bleiben erhalten."); }
    Ok(status)
}

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let interactive = args.is_empty();
    let code = match run(&args) { Ok(code) => code, Err(e) => { eprintln!("EtuSchlopf: {e}");1 } };
    if interactive { println!("Zum Schliessen Eingabetaste druecken."); let mut s=String::new();let _=io::stdin().read_line(&mut s); }
    if code == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn workspaces_are_new_and_existing_results_are_preserved() {
        let root = Runtime(fresh_directory(&env::temp_dir(), "distribution-test").unwrap());
        let a = fresh_directory(&root.0, "Lauf").unwrap();fs::write(a.join("old.pdf"), b"old").unwrap();
        let b = fresh_directory(&root.0, "Lauf").unwrap();assert_ne!(a,b);assert_eq!(fs::read(a.join("old.pdf")).unwrap(),b"old");
    }
    #[test]
    fn long_midi_extension_resolves_real_wav_without_modifying_inputs() {
        let root = Runtime(fresh_directory(&env::temp_dir(), "distribution-test").unwrap());
        let midi=root.0.join("Example Ü.midi");let wav=root.0.join("Example Ü.WAV");
        fs::write(&midi,b"midi bytes").unwrap();fs::write(&wav,b"wav bytes").unwrap();
        let (m,w,s)=midi_pair(&midi).unwrap();assert_eq!(m,midi);assert_eq!(w,wav);assert_eq!(s,"Example Ü");
        assert_eq!(fs::read(midi).unwrap(),b"midi bytes");assert_eq!(fs::read(wav).unwrap(),b"wav bytes");
    }
    #[test]
    fn missing_wav_is_reported_without_creating_output_workspaces() {
        let root = Runtime(fresh_directory(&env::temp_dir(), "distribution-test").unwrap());let midi=root.0.join("Example.mid");fs::write(&midi,b"source").unwrap();
        assert!(midi_pair(&midi).is_err());assert!(!root.0.join("EtuSchlopf-Ergebnisse").exists());assert_eq!(fs::read(midi).unwrap(),b"source");
    }
}
