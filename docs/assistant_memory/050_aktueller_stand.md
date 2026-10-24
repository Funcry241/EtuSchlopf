# 050 – Aktueller Stand

Stand 24.10.2026. Advisory only. Massgeblich sind immer die aktuellen hashgebundenen
Lifecycle-/Acceptance-Belege und die Runtime-Belege des Exports, nicht dieser Text.

## Jetzt

- Angenommen bis r1503 / 0.10.1839 (Kandidat b3f49ac8...): 10/10 dunkelgruen,
  2444 Tests bestanden, 0 fehlgeschlagen, 1 ignoriert (15 Testprogramme); Build/Clippy Exit 0.
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
