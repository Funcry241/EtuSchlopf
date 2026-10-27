// v0.10.725 FoundationIoRealModuleSplitNoBehaviorChange.
// Real crate-root module split from the former mechanical IO grouping.
// Preserves original module path `cli` and behavior; no wrapper/include shell.

// --- mechanical module boundary: former src/cli.rs ---
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "etuschlopf")]
#[command(version)]
#[command(about = "MIDI/WAV-Projektpipeline: aus WAV/MIDI-Paaren Klavierfassungen erzeugen und Korpus-Emergenz auswerten", long_about = None)]
pub struct Cli {
    /// Projektname, z. B. "Etüde Nr. 6". Erwartet "Etüde Nr. 6.wav" und "Etüde Nr. 6.mid". Für Ordner-Batch ausdrücklich run.cmd --all verwenden.
    pub input: Option<PathBuf>,

    /// Ordner-Batch: findet alle gleichnamigen WAV/MIDI-Paare, erzeugt pro Projekt eigene Ausgaben und schreibt anschließend corpus_emergence.result.txt.
    #[arg(long)]
    pub all: bool,

    /// Korpusmodus: liest vorhandene *.result.txt-Dateien und schreibt corpus_emergence.result.txt mit PassOrder-/Obstruction-Audit. Keine Musikmutation.
    #[arg(long)]
    pub corpus_emergence: bool,

    /// Ausgabedatei für den Korpus-/Mathematikbericht.
    #[arg(long, default_value = "corpus_emergence.result.txt")]
    pub corpus_out: PathBuf,

    /// Einzellauf darf eine vorhandene corpus_emergence.result.txt als audit-only Cache lesen. Standard: ignorieren.
    #[arg(long)]
    pub use_corpus_cache: bool,

    /// Schreibt die Ergebnisdatei. Ohne Angabe automatisch: <Projekt>.result.txt
    #[arg(long)]
    pub out: Option<PathBuf>,

    /// Interne MusicXML-Zwischendatei für den PDF-Renderer. Nicht als Input/Quelle verwenden. Ohne Angabe automatisch: <Projekt>.musicxml
    #[arg(long)]
    pub musicxml_out: Option<PathBuf>,

    /// Schreibt die PDF-Ausgabe, sofern ein MuseScore-/mscore-Renderer gefunden wird. Ohne Angabe automatisch: <Projekt>.pdf
    #[arg(long)]
    pub pdf_out: Option<PathBuf>,

    /// Explizite WAV-Referenz. Wenn nicht gesetzt, sucht EtuSchlopf eindeutig im Eingabeordner.
    #[arg(long)]
    pub reference_wav: Option<PathBuf>,


    /// PDF-Erzeugung überspringen.
    #[arg(long)]
    pub no_pdf: bool,

    /// Internal distribution startup query; exits before project preparation.
    #[arg(long, hide = true)]
    pub renderer_available: bool,

    /// Optionale feste Ziel-Seitenzahl. Ohne Angabe entscheidet EtuSchlopf automatisch nach Dichte.
    #[arg(long)]
    pub layout_pages: Option<usize>,

    /// Notensatz-Politur überspringen. Dann wird nur geprüft und keine MusicXML-Ausgabe geschrieben.
    #[arg(long)]
    pub no_polish: bool,

    /// Milde Variante: unterfüllte Nebenstimmen nur warnen statt blockieren. Standard ist streng.
    #[arg(long)]
    pub lenient_partial_voices: bool,

    /// Anzahl der wichtigsten Warnungen/Hotspots im Bericht.
    #[arg(long, default_value_t = 12)]
    pub top: usize,

    /// Optionaler, report-only Render-Writer-Trace und ausfuehrliche Balken-Diagnosen. Ohne Wert fuer alle Takte; mit --writer-trace=5,7 nur fuer diese Takte. Writer-Snapshots zeigen kompakte Vorher-/Nachher-Zustaende; Balken-Auffaelligkeiten und Summen bleiben immer sichtbar.
    #[arg(long, num_args = 0..=1, require_equals = true, default_missing_value = "all", value_name = "TAKTE")]
    pub writer_trace: Option<String>,

    /// Gesamtbudget für Rechenparallelität. Im Einzelprojekt steuert es Solver-Threads; bei --all wird es auf höchstens zwei gleichzeitige Projekte aufgeteilt. Die semantische Kandidatenbreite bleibt dabei separat erhalten. Standard: auto = erkannte logische Threads minus 1. Sicherheitsmodus: --jobs 1. Volle Parallelität: --jobs max.
    #[arg(long, default_value = "auto")]
    pub jobs: String,

    /// Entfallener interner Schalter; wird mit einer Fehlermeldung zurueckgewiesen. --jobs steuert nur Parallelitaet; Suchumfang ist fest 192/5.
    #[arg(long, hide = true)]
    pub candidate_budget_jobs: Option<usize>,
}

impl Cli {
    pub fn normalized_from_env() -> Self {
        let mut args: Vec<String> = std::env::args().collect();
        // Rückwärtskompatibilität nur für das alte Präfix: etuschlopf.exe check "Etüde Nr. 6" ...
        if args.get(1).map(String::as_str) == Some("check") {
            args.remove(1);
        }

        // Windows-Nutzer tippen für den Ordnerlauf leicht `-all` statt `--all`.
        // Clap interpretiert `-all` als Kurzoptionen -a -l -l und bricht vor unserem Batchcode ab.
        // Darum normalisieren wir den Alias bewusst vor dem Parser.
        for arg in args.iter_mut().skip(1) {
            if arg.eq_ignore_ascii_case("-all") || arg.eq_ignore_ascii_case("/all") {
                *arg = "--all".to_string();
            }
        }

        Self::parse_from(args)
    }}
