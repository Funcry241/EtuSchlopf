# DeadPipelineHistorySinkRetirement

Stand: v0.10.1595 r06
Ausgangspunkt: v0.10.1594 r02 / Export 534

## Ausgangsbeweis

Export 534 bestaetigt v0.10.1594 r02 als real installierten Stand: Strict-Build
und Strict-Clippy erfolgreich, Warnbudget 0, `FinalExit 0`, MusicXML nur mit
Versionswechsel und sechs pixelidentische PDF-Seiten.

## Befund und grosser Schnitt

`pipeline_lines` war ausschliesslich write-only. Der Vektor wurde mit hunderten
History-, Audit- und Statistikzeilen befuellt, aber weder vom typisierten
Finalreport noch von Konsole, Artefaktwahl, Blocker-, Release- oder Exitlogik
gelesen. v0.10.1595 entfernt den Sink vollstaendig; im produktiven Rust-Baum
verbleibt keine `pipeline_lines`-Referenz.

Mit entfernt werden nur Rechenfamilien, deren letzte Senke dieser Vektor war:
rein berichtende Konflikt-, WAV-, PDF-, Sheaf-/Pareto-, Solver-, Near-Miss-,
Hand-, Substance-, Hidden-Rest- und Reportprojektionen sowie ihre XML-/Event-
Zweitscans, Sortierungen, Hashes, Kandidatenklone und Formatierungen.

## Cross-File-Closure r06

Der r05-Take wurde vor dem Kopieren durch die konservative
Cross-File-Interface-Pruefung abgelehnt. Die als unveraendert eingestufte Datei
`render_write_pdf_source_adoption_truth_gate.rs` enthaelt sowohl den Typnamen
`RawCursorInvariantHardGateAudit` als auch einen Zugriff `.pipeline_line`; der
Zugriff gehoerte jedoch zum benachbarten Truth-Gate-Audit. Damit lag keine
fachliche Raw-Cursor-Referenz vor, aber ein formal noch nicht geschlossener
Dateischnitt.

r06 fuegt keinen kuenstlichen Noop-Diff hinzu. Die kollidierende Datei besass
selbst noch eine nach dem History-Sink-Cut write-only gewordene
`RenderWritePdfSourceAdoptionTruthGateAudit.pipeline_line`. Diese Senke, ihre
reine Formatierung sowie das nur dafuer gespeicherte und uebergebene
`project_label` werden vollstaendig entfernt. Die typisierte
Truth-Gate-Reportprojektion verwendet weiterhin alle produktiven Raw-Cursor-,
Renderer-, Bindungs-, PDF-Quellen-, Freigabe- und Blockerwerte.

Die Writer- und Mutationsreihenfolge bleibt unveraendert; es wird weder
MusicXML noch Render-MusicXML oder PDF anders gewaehlt oder geschrieben.

## Compiler-Closure r05

Der echte Strict-Clippy-Lauf von r04 erreichte die vollstaendige Typpruefung
und bestaetigte damit die Closure der sechs vorherigen Compilerfehler. Danach
meldete `#![deny(dead_code)]` 30 nachgelagerte tote Felder, Methoden und
Formatierer; Clippy meldete zusaetzlich 13 kleine mechanische Hinweise.

r05 entfernt diese zweite Welle rekursiv bis zu ihren ausschliesslich
berichtenden Erzeugungswurzeln. Dazu gehoeren die alte FLP-Seed-Statistik,
Reference-/Layout-Historyzeilen, reine Clef-Decomposition-Statistiken und
mehrere nie gelesene Auditfelder. Produktive Daten werden nicht pauschal nach
Namen entfernt: Die Staccato-Acceptance-Zeilen bleiben beispielsweise erhalten,
weil der nachfolgende Suppressionswriter sie als Kandidatenquelle verwendet.

Die 13 Clippy-Hinweise werden ohne Verhaltensaenderung geschlossen: Unit-
Rueckgaben werden ohne `let _ =` aufgerufen und ein identischer `no_pdf`-Zweig
wird vereinfacht. Es gibt keine Lint-Suppression, keinen Dummy-Read und keinen
Ersatzsink.

Statisch verifiziert sind konsistente Struct-Definitionen und -Literale, keine
neu wurzellose private Funktion oder Typebene, keine neu wahrscheinlich
ungenutzte Import-/Parameter-/Let-/Mut-Bindung, ausgeglichene Rust-Struktur,
vorhandene Include-Ziele sowie identische Sequenzen von 174 Authority-Aufrufen,
118 direkten Datei-Writern und 39 produktiven `&mut best`-Aufrufen.

## Compiler-Closure r04

`r03` schloss den Cross-File-Feldvergleich, wurde danach jedoch vom echten
Strict-Clippy-Lauf verworfen. Drei Rückgabebindungen waren durch eine zu enge
statische Let-Consumer-Prüfung entfernt worden, obwohl sie im später per
`include!` eingebundenen typisierten Finalreport weiterhin konsumiert werden.

`r04` bindet die Ergebnisse von Gould-Cutover, ExternalFeedbackClosure und
KeySignatureEnharmonicSpan wieder an ihre bestehenden Reportprojektionen. Die
Writer-Aufrufe und ihre Fehlerfortpflanzung bleiben byte- und reihenfolgegleich;
auch die öffentlichen Reportsektionen bleiben bestehen. Entfernt werden nur
die tatsächlich ungenutzte `accepted_names`-Mini-Senke samt lokaler `accepted`-Formatierung und der verwaiste
Re-Export `FinalBarlineAuthorizationOutcome`. Die autorisierte Final-Barline-
Funktion selbst bleibt erhalten.

Damit werden die sechs gemeldeten Compilerursachen ohne Suppression, Dummywert
oder Reportverlust geschlossen.

## Compiler-Closure r02

r01 wurde vom echten Strict-Clippy-Lauf vor Installation verworfen. Der Log
zeigte offene Refactor-Closure:

- verwaiste `pub`-Fragmente in `central_rulebook.rs`,
- fehlende Sichtbarkeit des weiterhin aktiven Report-Traits
  `StaticNamedReportLineV766`,
- tote Anfangszuweisungen und ungenutzte Fehler-/Auditbindungen.

r02 behebt die Ursachen statt die Meldungen zu unterdruecken. Zusaetzlich
wurden die nach der Closure wirklich wurzellosen Rueckgabestrukturen und Felder
entfernt. Der CrossStaffVoiceLedger-Pfad wurde beim Writerabgleich als
potenziell produktiv erkannt und deshalb einschliesslich seines autorisierten
Writers exakt erhalten; nur seine tote Sinkprojektion entfaellt.

Eine anschliessende statische Let-Consumer-Pruefung fand drei weitere
Audit-Rueckgaben ohne Consumer. Deren potenziell schreibende Aufrufe und
Fehlerfortpflanzung bleiben bestehen; entfernt wurden nur die ungenutzten
Rueckgabebindungen und reinen Skip-Auditobjekte.

## Erhaltene produktive Autoritaet

Statischer Vergleich v0.10.1594 gegen r05:

- autorisierte Writer-Aufrufe in identischer Anzahl und Reihenfolge,
- 39 Orchestrator-Aufrufe mit `&mut best` gegen 39, identische Reihenfolge,
- `std::fs::write`, `fs::write` und `File::create` in identischer Anzahl,
- CrossStaffVoiceLedger-Materialisierung erhalten,
- MaterializedRewritePortfolio und Adoption erhalten,
- Kondensationsklassen, Sortierung, Top-24-Auswahl und Render-Mutation erhalten,
- Rest-, Beam-, Clef-, Artikulations-, Retention- und Raw-Cursor-Gates erhalten,
- Master-/Render-MusicXML- und PDF-Auswahl, typisierter Finalreport sowie
  Blocker-, Release- und Exitentscheidung erhalten,
- keine musikalische Schwelle, Gewichtung oder Passreihenfolge geaendert.

## Bilanz

- v0.10.1594 r02: 120.326 physische Rust-Zeilen
- v0.10.1595 r04: 116.321 physische Rust-Zeilen
- v0.10.1595 r05: 115.587 physische Rust-Zeilen
- v0.10.1595 r06: 115.541 physische Rust-Zeilen
- Netto gegen v0.10.1594: **-4.785**
- Netto r05 -> r06: **-46**
- Orchestrator: 4.864 -> 3.592 Zeilen
- `pipeline_lines`: 0 produktive Rust-Referenzen
- kein Ersatzsink, Adapter, Shadowpfad oder neue Engine

## Statische Abschlusspruefung

- ausgeglichene Rust-Klammer-, String- und Kommentarstruktur,
- keine neu wurzellose private Funktion oder nicht erlaubte private Typebene,
- keine neuen ungenutzten Parameter-, einfachen Let- oder Importbindungen
  gegen den gruenen Ausgangsstand,
- keine neuen unnoetigen `mut`-Bindungen; zwei bekannte Scanner-Falschpositive
  verwenden weiterhin produktiv `Option::replace`,
- entfernte Struct-Felder auf unveraenderte Cross-File-Consumer geprueft,
- alle statischen `include!`-/`#[path]`-Ziele vorhanden,
- autorisierte Writer-, direkte Datei-Writer- und `&mut best`-Sequenzen gegen
  v0.10.1594 abgeglichen,
- Take-Manifest aus dem realen Diff und den eingehenden Dateibytes neu erzeugt.

Ein Rust-Compiler konnte in der Erstellungsumgebung nicht bereitgestellt
werden. Die verbindliche Kompilations- und Outputabnahme bleibt deshalb genau
ein Lauf:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Kein `--force` und kein zweiter Routinelauf.
