# 050 – Aktueller Stand

Stand 25.10.2026. Advisory only. Massgeblich sind immer die aktuellen hashgebundenen
Lifecycle-/Acceptance-Belege und die Runtime-Belege des Exports, nicht dieser Text.

## Jetzt

- Angenommen bis r1514 / 0.10.1839 (d69b4895...): 10/10 dunkelgruen,
  2485 Tests bestanden, 0 fehlgeschlagen, 1 ignoriert (15 Testprogramme); Build/Clippy Exit 0.
  Alle MuseScore-Aufrufe ohne `-f`,
  erste Variante erfolgreich (`VariantsAttempted 1/7; FailedBeforeSuccess 0`).
- Balkenthema abgeschlossen (seit r1484): Pruefer fuer Primaergruppen ueber Stimmen 0 Takte,
  Pausenbruecken E3/T6, E5/T36, E5/T37, E6/T32, E6/T34 korrekt.
- Writer-Rueckbau abgeschlossen: r1486 (KodexOnly-Zulassung) und r1487 entfernen
  structural_lower_beam (E047), gould_metric_beam (E008/E009), external_feedback_closure M3
  (E048) und lower_bee_queen_beam/FinalCompleteBeat (E006) samt B2/G4/S3. M5 (E049) und
  E050 (Pausenbruecke) bleiben. Master bytegleich, Render nur Leerzeilen, PDFs pixelgleich.
  Details: ../four_beam_writer_retirement_r1487.md.
- r1488 (KodexOnly): Der installierte Rueckbau bindet nur Kodex-Daten, die sechs Loeschungen,
  die Checkpoint58-Nachfolger und das Fehlen der entfernten Befugnis; lebende Dateien sind
  nicht mehr byteweise eingefroren. Details: ../writer_retirement_installed_state_r1488.md.
- r1489/r1490: kleine Bereinigungen ohne Musikwirkung (Kommentar, Testhilfen, Importstatus
  im Bericht konsistent mit der Consumer-Pruefung, dieser Kurzstand).

- r1491 (RefactorOnly): Q02 abgeschlossen; vorhandene Trial-Konflikte werden vor der
  unveraenderten Importprofil-Pruefung wiederverwendet.
- r1492 (KodexOnly / A0): exakte spaetere Windows-Abhaengigkeit zugelassen; noch kein
  Prozessschutz und keine Compiler-Optimierungsumstellung.
- r1493 (RefactorOnly / Q03): Balken-Diagnosen kompakt gezaehlt, Auffaelligkeiten immer
  sichtbar; Einzelheiten ueber `--writer-trace` bzw. dessen Taktfilter.
- r1494 (RefactorOnly / K1): Konsolen-Statusdatensaetze `ETUSCHLOPF-STATUS v1`
  (Owner `src/console_status_record.rs`), nur mit `ETUSCHLOPF_CONSOLE_STATUS=v1` aktiv.
- r1495 (OutcomeProjection / K2): Launcher-Konsole mit Revision, Minuten, Testsumme ueber
  alle Testprogramme, sichtbarem Test-Vorbau, "zuletzt"-Werten und Projekten live.
- r1496 (OutcomeProjection / K3): Laufzeile bricht nicht mehr um; Laufzeile wird vor der
  naechsten regulaeren Zeile geloescht.
- r1497 (OutcomeProjection / K4): Konsole verdichtet, "zuletzt" in Fertig-Zeilen,
  Abschlusszeile ueber alle Projekte.
- r1498 (OutcomeProjection / K5): Aufraeumen (Test-Vorbau "wartet", Gesamtzeit,
  Praefixe, Hygiene) und Messpunkt Jobs des Test-Vorbaus.

- r1499 / K6 (RefactorOnly, accepted): Log je Projekt im Korpuslauf. Die Diagnose (stderr)
  jedes Projekts geht nach `target/etuschlopf_logs/projects/<Projekt>.console.txt` und
  wird nach Projektende als ein Block (`===== PROJEKT-LOG … =====`) ins Konsolen-Log
  uebernommen; parallele Projekte verschraenken keine Zeilen mehr. Statusdatensaetze
  `ETUSCHLOPF-STATUS v1` laufen ueber stdout; der Batch reicht gueltige Datensaetze sofort
  ins Konsolen-Log weiter (Live-Anzeige bleibt). Musik und Entscheidungen unveraendert.
  Annahme durch den gelieferten hashgebundenen Acceptance-Beleg bestaetigt.


- r1500 (KodexOnly, accepted): Stufe-A-Definition der Fehlerklasse
  LowerStaffChordAttackProjection eingefroren
  (../selector_derivation_contract_lower_staff_chord_attack_projection.md) und Kurzstand.

- r1501 / K7 (OutcomeProjection, accepted): Konsole im Vorlauf. Der Take-Bootstrap meldet seine
  langen Schritte (Entpacken, Dead-Code-Integritaet, Abhaengigkeiten, Take-Zulassung, Sicherung,
  Uebernahme, Cargo.lock und Kodex-Gates) mit Beginn und Dauer; sichtbar ab dem ersten Take nach
  K7, weil ein Take vom Bootstrap des Parents angewendet wird. Der Launcher zeigt nach der
  Kopfzeile eine Zeile `Vorlauf` (Bootstrap-Build, Take-Pruefung, Launcher-Build) aus dem
  vorhandenen Startup-Timing. run.cmd benennt die beiden stillen Cargo-Builds. Musik, Gates
  und Entscheidungen unveraendert. Details: ../launcher_preparation_console_k7_r1501.md.
  Annahme durch den frischen Lifecycle-/Acceptance-Beleg bestaetigt.
  Hinweis: Die r1501-reason nennt noch das Mitfuehren der Definition; diese war
  bereits durch r1500 angenommen und wurde in r1501 nicht geaendert.

- r1502 (MusicMutation, Maus, accepted): gemeinsame lokale Akkordbeobachtung vor Projektion;
  Planer und E054 zaehlen nur geklaerte erste untere Beobachtungen; Unknown ankerlos und
  ungezaehlt; E054 schreibt nicht vor kopflosen Mitgliedern. 20/20 XML bytegleich zu r1501,
  PDFs pixelgleich, Reentry 0. Details: ../lower_staff_chord_attack_projection_r1502.md.

- r1503 / K8 (OutcomeProjection, Dachs, accepted): HIMEM-Konsole im Vorlauf. Bootstrap-Schritte als
  eine Zeile mit live hochzaehlenden Sekunden und "zuletzt", am Ende "... fertig · N s";
  strikt nicht blockierend (eigener Thread, kein Join). Zwei bisher stille Abschnitte
  (Revisionsabdruck/Ablehnungsverlauf, Watch-Projekt) haben eigene Zeilen. run.cmd zeigt
  "zuletzt N s" fuer die beiden Cargo-Builds. Musik, Gates und Entscheidungen unveraendert.
  Details: ../launcher_himem_console_k8_r1503.md.
  Annahme durch den hashgebundenen Lifecycle-/Acceptance-Beleg bestaetigt.

## Aktueller Kandidat

- r1504 (RefactorOnly, Maus): Balkenanker-Diagnose im bestehenden finalen
  Master-/Render-Bericht. Gleicher Scanner, rohe Akkord-, Tie-, gespeicherte Balken-
  und effektive Schluesselanker; Takt-/Notenabstand und echte XML-Grenzen dazwischen.
  Alle sichtbaren unteren Tonereignisse mit Primaerbalken werden ohne Kappung gezeigt.
  Endstand-Nachmessung, kein Vorlauf-/Zwischenstandsbeweis und keine Fehlerentscheidung.
  Produktive Planer- und Writerentscheidung unveraendert. Details:
  ../beam_anchor_distance_audit_r1504.md. Native Annahme steht aus.

## Offen

- E6/T22 (Fis2/Cis3 im Violinschluessel): Regelentscheidung Wolfgang.
- Beam-Fernbindung offener unterer Primaerbalken: r1504 liefert die Endstand-Messung;
  Bewertung und gegebenenfalls Messung der produktiven Zwischenstaende bleiben offen.
- Laufzeit E6: ArrangementSolver bewertet rund 1830 Varianten (E1: rund 130); Ansatz
  inkrementelle Bewertung bzw. Parallelitaet erst nach Messung und Entscheidung.

## Offen und abgestimmt (Maus/Dachs, 23.10.2026)

- Laufzeit-Kandidaten (Q16/Q17/Q23/Q24, Q25/Q26, Q27) nur nach tatsaechlicher Messung.
- Vorlaufzeit eines Takes: die Bibliothek wird vor dem Launcher dreimal uebersetzt bzw. geprueft
  (Bootstrap-Build, `cargo check --lib` der Cargo.lock-Pruefung, Launcher-Build). Erst mit
  `cargo build --timings` messen, dann entscheiden; keine Optimierungsstufe aendern.

## Zurueckgestellt (Entscheidung Wolfgang)

Spielbarkeit nach objektiven Kriterien statt Dichtegrenzen; WAV- und Boegen-Themen;
Source-Selection-Rueckbau; Taktverschiebungstest mit echten Etueden.

## Testpraxis ab r1477

Taktnummern in Test-XML nur ueber Variablen oder tests/support/synthetic_score.rs.
Writer-Tests mit Einzeltakt-Fixture zusaetzlich durch assert_shift_invariant fuehren.
Siehe 010 Muster 31 und ../measure_shift_test_bench_r1477.md.

## Historie (nur zur Einordnung, kein Auftrag)

- r1469: lokale validate_binding-Memoisierung. r1470: abgeschlossene Report-Abschnitte
  entfernt (../closed_audit_report_sections_retirement_r1470.md).
- r1472: Schutzregel in structural_lower_beam gegen Faehnchen. r1473: optionale
  Writer-Spur. r1474/r1475: Vertraege und E050-Schnittstellenzulassung. r1476: E050 prueft
  Gruppen gegen den eingefrorenen Stimmenplan.
- r1477: Verschiebungs-Pruefstand (../measure_shift_test_bench_r1477.md).
- r1478: spaeter RestBridge-Teil in E048 stillgelegt (../late_rest_bridge_retirement_r1478.md).
- r1479–r1483: Erweiterungsbeleg, Checkpoint58-Transportzulassungen und spaete Ruecknahme
  der Pausenbruecke; r1483 behebt E6/T32.
- r1484: MuseScore ohne `-f`; vorlaeufige Bruecken mit Entscheidung bei der Ruecknahme
  (../provisional_rest_bridge_and_unforced_renderer_r1484.md).
- r1485: Diagnosezaehler bereinigt (../diagnostic_counter_cleanup_r1485.md).


## r1505 – BeamAnchorPlannerInputTrace (candidate, parent r1504 accepted)

r1504 native acceptance confirmed with exact candidate hash 9c4c34bb5cea949d07e709c2f99915ecacdbc62f53a227c34757642206091e7c; Windows 2450 tests passed, 0 failed, 1 ignored, 10/10 corpus; 20 XMLs byte-identical and 50 pages pixel-identical at 200 dpi to r1503.

r1505 adds input-bound stderr diagnostics at actual eligible clef planner calls. It observes existing scanner anchors, actual part solution and candidates; no continuity rule, counterfactual solver, error classification or musical change. It does not prove committed writer effects. Final-document r1504 audit remains unchanged. See docs/beam_anchor_planner_input_trace_r1505.md. Native acceptance and full Windows corpus comparison pending.


## r1506 - BeamClefOctaveScopeDefinition (Stage A candidate)

Last confirmed accepted parent: r1505, exact take SHA-256 78b2a231f0b7d5775512dd1e7f9caf787adcc64b393720c8259c99cc33c3edd5.
Native r1505: 2453 tests passed, 0 failed, 1 ignored; build/clippy green; 10/10 corpus; 20 XMLs byte-identical and 50 pages pixel-identical at 200 dpi to r1504. All 1641 managed candidate files match the accepted snapshot.

Analysis found inconsistent lower octave-display scope handling between the anchor scanner and the second primary-beam interior guard. Isolated end-state experiments changed E6/E7/E9 Master plans; all ten Render end states stayed equal. These are not productive transient-input replays or musical approvals. E6/T22 remains a separate musical decision.

The existing KODEX requires a frozen parent error-class definition before selector-sensitive implementation. The chord/Unknown parent contract explicitly excludes general beam policy, and the older clef-relocation envelope contract describes another mechanism. r1506 therefore freezes only docs/selector_derivation_contract_primary_beam_lower_octave_scope.md, with necessity review, same-scope/pre-boundary protection and synthetic/multiplicity obligations. No runtime selector, scope reset or musical repair is installed by this take. No gate/pin/lock change. Native acceptance pending. Only after acceptance may a later implementation reference this contract unchanged; no future revision is reserved or accepted here.


## r1507 - BeamClefOctaveScopeClosure (MusicMutation candidate)

Parent r1506 accepted: a4bc2f0bc8bf65ca822bddc066936ecf8171e13652cd27b5516f8d420a477fcf. Native 2453 passed, 0 failed, 1 ignored, build/clippy green, 10/10; 20 XMLs byte-identical and 50 pages pixel-identical at 200 dpi to r1505. The scope class definition is frozen at SHA-256 80ba736a0aa531e90d588a0cdf8c0e1dd7884d9f9850992dabc4ef4c9ce3c2e4.

r1507 implements only the second primary-beam guard's part-local lower octave decision scope. Query-relative scoping prevents stale protection after a boundary. Proven interiors before future boundaries remain protected; unresolved prefixes block new insertions without forcing existing-clef removal. An earlier boolean-prefix prototype was rejected after an isolated E4 pressure regression; it is not the delivered code. Scanner, costs, tie/chord rules, writers and gates unchanged. No generic temporal-gap rule. Musical effect is declared through the existing pure-clef course; actual Windows artifacts and native acceptance pending. E6/T22 musical approval is not implied. See the separate repair contract. No further revision is assumed accepted.


## r1508 - LicenseNoticeAndPdfiumDiagnostics (RefactorOnly candidate)

Parent r1507 accepted: `2a2ce714d783101956ace0753c6ff2f9b3a9a8603c143081dc23a5288a286a66`.
The supplied native export reports 2465 passed, 0 failed, 1 ignored, build/clippy green, 10/10 corpus.
r1508 installs license notices and exact third-party license texts under docs/, and replaces PyMuPDF with pypdfium2 in the separate Python diagnostics.
The two additional Cargo resolutions are audit snapshots only, not installed component/runtime locks.
All Rust source files except the required change-manifest text, qualified component bytes, root Cargo.toml/Cargo.lock, gates and music logic remain unchanged.
Native acceptance and the fresh Windows full-corpus comparison remain pending. See ../license_take_r1508.md.


## r1509 - RegisterGestureContextDefinition (Stage A candidate)

Parent r1508 accepted: `2b5b26a70bf1e4807a56ad25bea036fab31b2854d063de297712637df0450144`.
Native 2465 passed, 0 failed, 1 ignored; build/clippy green; 10/10 corpus.
All 20 XMLs byte-identical and all 50 PDF pages pixel-identical at 144 dpi to r1507.
r1509 freezes only the causal definition of an octave-display choice omitting independently bound rhythmic-gesture context, and the work/acceptance plan.
The actual pre-K00 group evidence and its phase binding must be established before productive repair; final PDF beams are not retroactive authority.
All Rust sources, musical decisions, gates, tests and Cargo files remained unchanged. Native r1509 accepted; see the r1510 parent record below.
See ../register_gesture_work_plan_r1509.md and ../selector_derivation_contract_register_gesture_context.md.


## r1510 - RegisterPlannerInputReadback (RefactorOnly candidate)

Parent r1509 accepted: `e16ca7354500e6076ea9b701ec2d6f51ef7cb7db297756857b9bc745e92bf783`.
Export: native 2465 passed, 0 failed, 1 ignored; build/clippy 0; corpus 10/10; final exit 0.
r1510 captures the exact existing pre-K00 planner input into the current result report,
with byte count, SHA-256 and recoverable hex. No musical selector or writer changes.
An endpoint replay exposed segmentation and the four-object eligibility rule as a
serious alternative explanation; it does not prove the real pre-K00 cause.
Essence E01 gains two advisory sentences on alternatives and falsifying experiments.
Native r1510 accepted; see the r1511 parent record below. See ../register_planner_input_readback_r1510.md.


## r1511 - RegisterCommonSpanExclusionDefinition (Stage A candidate)

Parent r1510 accepted: `a2612a9712bdc01d6345c203242acfc12d16e706253e8266bcca4f241eb34187`.
Export: 2469 passed, 0 failed, 1 ignored; build/clippy passed; corpus 10/10; final exit 0.
All 20 XMLs were byte-identical to r1509; all 50 PDF pages matched at 144 dpi.
All 10 exact captured inputs replayed their native plan hash through r1510 source.
The blanket removal of pitch-only segment boundaries failed an independent synthetic
counterexample: two existing octave spans became one, leaving a high island unserved.
A read-only guarded comparison passed 13 synthetic expectations, including coverage,
strict whole-context cost, pause, incoming-tie, overlap, scaling and independent duplication.
This defines a different cause from the accepted r1509 gesture-context class.
r1511 freezes that class only. Rust sources and all musical decisions remain unchanged;
no classifier or repair is installed. Native r1511 accepted; see the r1512 parent record below.
See ../selector_derivation_contract_register_common_span_exclusion.md.


## r1512 - RegisterCommonSpanClosedComparison (MusicMutation candidate)

Parent r1511 accepted: `6aaf1fd728c6000a41f51b085628c1127793b7584fc3bd36213fb8dafe81965b`.
Export: 2469 passed, 0 failed, 1 ignored; build/clippy passed; corpus 10/10; final exit 0.
All 20 XMLs remain byte-identical to r1510. r1511 definition bytes remain unchanged.
r1512 compares a common octave window with the entire frozen baseline inside closed
pitch-only components. Baseline octave coverage is mandatory within window search;
strict whole-context improvement, anchor/tie safety and topology remain required.
Actual input replay: all 10 baseline hashes match; 16 logical replacements in 7 pieces.
Lower-clef comparison authority remains explicitly unresolved; its baseline is preserved.
Register tests: 73 local real-module cases passed. A separate real-writer/receipt/adoption
binary integration probe uses synthetic test genesis. Windows/native corpus acceptance,
actual downstream output comparison and new PDF readback remain pending.
See ../register_common_span_closed_comparison_r1512.md.


## r1513 - LowerClefBeamInteriorContextDefinition (Stage A candidate)

Parent r1512 accepted: `ea977b4a07e176a72f046b256cb0a4258df678241f32ecd491822ff05c5ed3e8`.
The 20261025-112259 export supplies the parent source and native lifecycle record.
A fresh PDF/XML review selected lower clef readability as the next musical topic.
Source inspection shows whole-course optimization already exists: primary-beam
and transitive tie anchors coalesce internal preferences before candidate
formation. Final Render readback shows that pattern in two low tails; it does
not establish their transient productive input or a newly measured distribution.
r1513 freezes only the causal missing-boundary-context definition, with a
necessity review, UNKNOWN, positive neighbors and independent proof obligations.
No musical source, gate, test, dependency or existing class contract changes.
Native acceptance is pending. A later repair must reference the accepted
definition unchanged and reconcile all existing clef owner guards, not bypass
one of them. No corrected PDF or musical result is claimed by this take.
See ../lower_clef_beam_interior_context_r1513.md and
../selector_derivation_contract_lower_clef_beam_interior_context.md.


## r1514 - ClefPlannerExactInputReadback (RefactorOnly candidate)

Parent r1513 accepted: `fdb4270fb2d193b719bc188f7ffd8176cdd14d466ef42adcfad89454c1205ee8`.
Native: 2482 passed, 0 failed, 1 ignored; build/clippy 0; corpus 10/10 dark-green.
The source snapshot matches all delivered r1513 source bytes.
r1514 adds one unconditional read-only call at the actual clef-planner entry.
Complete input bytes are SHA-256/length/FNV-bound and emitted as zlib hex frames
through existing stderr. Repeated calls are preserved; no classifier, selector,
cost, musical writer, receipt or adoption changes. r1513 definition unchanged.
The strict extractor retains all observations and rejects incomplete/corrupt data.
Actual input is observed; productive phase and writer commit remain unproven
without separate caller/receipt evidence. Native acceptance is pending.
See ../clef_planner_exact_input_readback_r1514.md.


## r1515 - LowerClefFreshBeamBoundary (MusicMutation candidate)

Parent r1514 accepted: `d69b4895a41aed42c9a5d40b7eb1b9eb67affd9703adfbb56ff28e60edc5c762`.
Native 2485 passed, 0 failed, 1 ignored; build/clippy passed; ten-piece corpus dark-green.
All 20 XMLs byte-equal and all 50 pages pixel-equal at 144 dpi to r1513.
Exact input capture: 90 observations, 55 distinct byte states; unchanged planner
replay matches all 32 native detailed solution and candidate-list occurrences.
r1515 adds a shared, conservative fresh-attack boundary proof inside a complete
lower primary beam. Original whole-part baseline and candidate choices remain
available; only strict whole-part cost improvement can replace that baseline.
The same proof is used in scanner, solver, normalization and postwrite safety.
No note/beam/tie or writer/receipt/adoption ownership mutation is introduced by
the clef operation. Unknown grants no new boundary; lower-clef costs stay fixed.
Correction: E-sharp3 has three treble ledger lines, not four as earlier reported.
The r1513 class bytes remain unchanged. Native r1515 acceptance is pending.
See ../lower_clef_fresh_beam_boundary_r1515.md.

## r1516 - OttavaRecueContextDefinition (Stage A candidate)

Parent r1515 accepted: `15cb8e29f849c4f1d27f431f9d940ec6faf2e5bd323b87f8b59ac10fdf9e848d`.
Native 2493 passed, 0 failed, 1 ignored; 15 test programs; build/clippy passed;
ten-piece corpus dark-green. Delivered source and acceptance hashes verified.
Five PDF pages change; the other 45 are pixel-identical at 144 dpi. All pitched
note bytes remain unchanged; three rest display rebindings follow clef changes.
The two lower-clef examples are improved; committed clef reentry changes are zero.
The proposed repeated-ottava topic is not a duplicate-writer bug: existing r605
policy intentionally requires visible reminders at in-system silent-bridge
continues. The final upper-domain gaps contain lower-domain notes, not scorewide
silence. A continuous final PDF line alone does not authorize reminder removal.
r1516 freezes only the omitted visible-line cue-context comparison definition.
No runtime selector, cue removal, octave-scope change, new writer or guard change.
All prior contracts and runtime/test source bytes stay unchanged. Native acceptance
is pending. See ../ottava_recue_context_r1516.md and
../selector_derivation_contract_ottava_recue_context.md.

## r1517 - Ottava-Kontextbeobachtung (Kandidat)

- Frischer Parent-Beleg: r1516 accepted; 2493 Tests, 10/10 dunkelgruen,
  Build/Clippy erfolgreich. r1516 friert nur die fehlende Kontextfrage ein.
- r1517 (RefactorOnly): der bestehende Registereingangs-Bericht zeigt fruehe
  semantische Plaene, validierten Sichtplan und tatsaechlich materialisierte
  Continue-Liniennummern sowie getrennte finale XML-Beobachtungen.
- Keine Zeichenentfernung, neue Leserregel oder Musikwirkung. PDF-Linienkontinuitaet
  und Leserbedarf bleiben UNKNOWN; finales XML rekonstruiert keine fruehe Identitaet.
- Details: ../ottava_context_observation_r1517.md. Native Annahme steht aus.

## r1518 - Leserregel und Reparaturumfang (Kandidat)

- Frischer r1517-Beleg: accepted, 2500 Tests bestanden, 0 fehlgeschlagen,
  1 ignoriert; Build/Clippy erfolgreich, 10/10 dunkelgruen. Gegen r1516 sind
  20/20 XMLs bytegleich und 50/50 PDF-Seiten bei 144 dpi pixelgleich.
- Zensus: 12 Fortsetzungen, davon 3 finale Restart-Projektionen ohne zusaetzliches
  Wort und 9 inner-systemische Erinnerungswoerter. Die neun Stellen wurden im
  PDF visuell geprueft; das ersetzt keine transportierte Identitaet oder PDF-Proof.
- r1518 (KodexOnly) friert die begruendete Leser-Praeferenz und den separat
  anzunehmenden Reparaturumfang ein. Keine Runtime-, Guard- oder Musikaenderung.
- Details: ../ottava_reader_reminder_scope_r1518.md und
  ../ottava_reader_reminder_repair_scope_v1.md. Native Annahme steht aus.

## r1519 - Belegte Leserentscheidung (Kandidat)

- Parent r1518 accepted, 2500 Tests / 10/10 dunkelgruen, 20/20 XML bytegleich
  und 50/50 PDF-Seiten bei 144 dpi pixelgleich zu r1517.
- r1519 implementiert den angenommenen Reparaturumfang: originale Herkunftsmarker als nichtdruckende XML-Kommentare,
  aktuelle PDF-Linien-/Orientierungsbeobachtung, quellengebundene Leserentscheidung
  im vorhandenen E012-Owner und exakter Receipt-/Adoptions-/Export-Nachweis.
- UNKNOWN behaelt die Baseline; finale PDF-Geometrie muss den Nachweis erhalten.
  Keine Aenderung eingefrorener E007-/Checkpoint58-/Render-Adoptionsdateien.
- Details: ../ottava_reader_decision_r1519.md. Native Annahme steht aus.

## r1520 - Vertikale Ottava-Renderbewegung (Kandidat)

- Parent bleibt r1518, hashgebunden angenommen. r1519 (4329c5be...) wurde
  nativ verworfen und auf r1518 zurueckgerollt: 2509 Tests, Build/Clippy gruen;
  finale PDF-Nachweise in drei Projekten fehlgeschlagen.
- Das getrennte native Gegenexperiment belegt vertikale Layoutbewegung bei
  erhaltenen Linien/Orientierungszeichen. Keine Korpusadressen oder gemessenen
  Abstaende werden zu produktiven Cutoffs.
- r1520 behaelt die quellgetragene Identitaet und Original-Receipt-/Adoptionskette.
  Der finale Beobachter prueft aktuelle Orientierung/Kontinuitaet im aktuellen
  Lesebereich bei unveraendertem horizontalen Verlauf und Seiten-/Zellenbezug.
- Native Annahme und vollstaendige XML-/PDF-Korpusbewertung stehen aus.
