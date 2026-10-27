//! Distribution bootstrap: private runtime extraction and fresh per-run input copies.
//! Musical decoding and writing remain exclusively in the embedded accepted engine.
#![forbid(unsafe_code)]
use std::{env, fs, io::{self, IsTerminal, Write}, path::{Path, PathBuf}, process::{Command, ExitCode, Stdio}, thread, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};

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

fn execute_engine(command: &mut Command, log_path: &Path) -> Result<i32> {
    // File-backed output cannot fill a pipe while the parent waits. Keep the
    // engine's complete diagnostics, including its original colour bytes, off
    // the user console and preserve its actual exit status.
    let log = fs::OpenOptions::new().write(true).create_new(true).open(log_path)?;
    command.stdout(Stdio::from(log.try_clone()?)).stderr(Stdio::from(log));
    let mut child = command.spawn()?;
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            println!("\rVerarbeitung beendet nach {} Sekunden.                  ", started.elapsed().as_secs());
            return Ok(status.code().unwrap_or(1));
        }
        print!("\rDein Stück wird verarbeitet ... {} Sekunden", started.elapsed().as_secs());
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(250));
    }
}

fn open_result_folder(output: &Path) {
    #[cfg(windows)]
    if io::stdin().is_terminal() {
        // Starting Explorer is a convenience; a shell failure must not change
        // the musical engine's result or hide the printed folder path.
        if let Err(error) = Command::new("explorer.exe").arg(output).spawn() {
            eprintln!("Der Ergebnisordner konnte nicht automatisch geöffnet werden: {error}");
        }
    }
    #[cfg(not(windows))]
    let _ = output;
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
    println!("Ich bereite das Programm vor. Der Ergebnisordner öffnet sich nach dem Lauf.");
    let rt = runtime()?;
    let mut child = Command::new(rt.0.join("etuschlopf_engine.exe"));
    child.current_dir(&rt.0).arg(&project);
    if args.get(1).is_some_and(|s| s == "--no-pdf") { child.arg("--no-pdf"); }
    let log_path = output.join("Laufprotokoll.txt");
    let result = execute_engine(&mut child, &log_path);
    println!("\nErgebnisordner: {}", output.display());
    println!("Technische Einzelheiten stehen in Laufprotokoll.txt.");
    open_result_folder(&output);
    let status = result?;
    if status == 0 {
        if args.get(1).is_some_and(|s| s == "--no-pdf") {
            println!("Fertig. Öffne {stem}.render.musicxml in MuseScore. Eine PDF war nicht angefordert.");
        } else {
            println!("Fertig. Öffne {stem}.pdf im Ergebnisordner.");
            println!("Zum Bearbeiten in MuseScore nimm {stem}.render.musicxml.");
        }
    } else {
        eprintln!("Der Lauf ist fehlgeschlagen. Im Ergebnisordner findest du Laufprotokoll.txt mit der genauen Meldung.");
        eprintln!("Deine Originaldateien und frühere Ergebnisse bleiben erhalten.");
    }
    Ok(status)
}

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    // Drag-and-drop also supplies an argument. It still needs a persistent
    // result message, whereas help/version and noninteractive jobs must exit.
    let query = args.len() == 1 && matches!(args[0].as_str(), "--help" | "-h" | "--version");
    let interactive = !query && io::stdin().is_terminal();
    let code = match run(&args) { Ok(code) => code, Err(e) => { eprintln!("EtuSchlopf: {e}");1 } };
    if interactive { println!("Zum Schließen dieses Fensters drücke die Eingabetaste."); let mut s=String::new();let _=io::stdin().read_line(&mut s); }
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

    #[test]
    fn engine_failure_keeps_both_diagnostic_streams_and_exit_code() {
        let root = Runtime(fresh_directory(&env::temp_dir(), "distribution-log-test").unwrap());
        let path = root.0.join("Laufprotokoll.txt");
        #[cfg(windows)]
        let mut command = {
            let mut c = Command::new("cmd.exe");
            c.args(["/D", "/C", "echo engine-stdout & echo engine-stderr 1>&2 & exit /b 7"]);
            c
        };
        #[cfg(not(windows))]
        let mut command = {
            let mut c = Command::new("sh");
            c.args(["-c", "printf '\\033[92mengine-stdout\\033[0m\\n'; printf 'engine-stderr\\n' >&2; exit 7"]);
            c
        };
        assert_eq!(execute_engine(&mut command, &path).unwrap(), 7);
        let diagnostic = fs::read_to_string(path).unwrap();
        assert!(diagnostic.contains("engine-stdout"));
        assert!(diagnostic.contains("engine-stderr"));
    }
}
