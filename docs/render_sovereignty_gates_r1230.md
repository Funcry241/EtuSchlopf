# r1230 Gate-Ergebnisse – korrigierter Ersatz-Take

Prüfstand: 2026-10-03, Rust 1.98.0, Linux. Compiler und bestehende Abhängigkeiten wurden aus der verfügbaren Toolchain verwendet. Der Cargo-Offline-Lauf kann `anyhow` mangels Registry-Quellcache nicht auflösen; die folgenden Compiler-/Testprüfungen wurden daher direkt mit rustc ausgeführt. Das ist kein ausgeführter Cargo-all-targets- oder Windows-Lauf.

| Prüfung | Ergebnis | Umfang |
|---|---|---|
| Unverändertes Parent-Take-Admission | PASS | tatsächlicher korrigierter Payload gegen accepted r1229 |
| No-new-structural-authority | PASS | zuvor 115, jetzt null Verstöße; unveränderte Parent-Prüfung |
| Vollständiger Build-Gate-Orchestrator | PASS | unveränderte Governance-/Freeze-/Normalization-/RSP-/Gould-/Selector-/Corpus-Admission-Gates |
| rustc normal und Test-Kompilation | PASS | aktueller Main-Source und tatsächlich generierte Inventories |
| Main-Testbinary, parallel und seriell | PASS | 1.133 bestanden, null Fehler; fünf Läufe mit 19 Threads und ein serieller Lauf |
| Clippy correctness/suspicious/perf mit deny | PASS mit Warnungen | normal 17 andere, nicht blockierende Warnungen; zusätzlicher Test-Clippy geprüft; keine Zero-Warning-Aussage |
| Trust-Quellen | PASS | alle 61 gebundenen Quellen bytegleich zum Parent; Cargo.toml unverändert |
| Parent-Patch, Manifest, ZIP | PASS | Source-Reproduktion, FNV-Abdeckung, CRC und sichere Take-Pfade |
| cargo fmt / Cargo all-targets | NOT_EXECUTED | keine vollständige Cargo-Registry verfügbar |
| Windows run.cmd/strict.cmd, Korpus, Renderer/PDF-Parität | NOT_EXECUTED | lokale Nutzerumgebung erforderlich |

## Reparatur der Authority-Ablehnung

Neue productive Writerimplementierungen liegen in der kanonischen Structural-Write-Authority. Testdelegationen befinden sich in zulässigen Test-Supportmodulen. Nicht schreibende Lineage-Replays öffnen keine zusätzliche Writerautorität. Die echten Owner, Receiptbindung, Sink-/Readback-Assertions und unveränderten Trust-Gates bleiben maßgeblich; Gates wurden nicht gelockert.

Die Compilerprüfung deckte zusätzlich Integrationsfehler und ungenutzte neue Adapter auf. Die Testkorrekturen binden parametrische numerische Ordinalwerte an reale Parser, erhalten historische Checkpointdaten unabhängig von deren späterer Write-Berechtigung und führen Raw-Splice-Prüfungen mit dem tatsächlichen EXACT-Core aus. Der Wrong-Note-Sink-Test übergibt eine konkrete inkompatible historische Senke und prüft weiterhin Tokenverbrauch und Write-Ablehnung.

## Installation

Das ZIP enthält den vollständigen korrigierten Take. Keine beigefügten Referenzpatches nachträglich anwenden. Bestehendes Take-Verfahren mit `run.cmd --take --all` verwenden; keine Force-/Bypass-Option. Root-Cargo.lock bleibt lokal/transaktional und ist nicht Teil des Pakets. Die lokale Windows-/Korpusprüfung bleibt notwendig, bevor der Take als accepted gilt.

## Korrektur nach Windows-Paralleltest-Rollback

Das Nutzerlog vom 2026-10-03 zeigt sechs Closure-Testfehler bei paralleler Ausführung, obwohl Build und Clippy bestanden. Ursache: neue Tests teilen processweite Render-Messdaten; ein Reset eines Tests löscht Stage-Records oder setzt einen noch lebenden Pending-Zähler eines anderen Tests auf null. Dessen späterer Decrement erzeugt den beobachteten usize-Unterlauf. Der vorherige serielle Prüflauf hat diesen Fehler nicht erfasst.

Nur cfg(test) verwendet nun threadlokale Messspeicher für die Render-Zähler und die zwei Closure-Records. Produktive processweite Atomics/Mutexe, Receipt-Lebenszyklen, Stage-Assertions und fail-closed Closure bleiben unverändert. Ein synchronisierter Zwei-Thread-Regressionstest beweist die Isolation eines lebenden Pending-Zählers und Stage-Records gegenüber fremdem Reset. Fünf vollständige Main-Läufe mit 19 Threads und der serielle Kontrolllauf bestehen mit jeweils 1.133 Tests. Windows-all-targets und Korpus müssen erneut über das bestehende Take-Verfahren laufen.

Die MultiRevision-Semantic-Capability-Evasion-Warnung bleibt separat offen. Sie betrifft den Vergleich mit früheren abgelehnten Kandidaten, insbesondere Quellverlegungen und nicht mutierende Replay-/Guardpfade der vorangegangenen Authority-Reparatur. Die neue Änderung hebt keine Admission-Regel auf und beansprucht keine Beseitigung dieser Warnung.

Zusätzlicher Test-Clippy: die unnötige Clone-Slice-Erzeugung in einer Gould-Fixture wurde durch `std::slice::from_ref` ersetzt. Kein produktiver Gould-Code geändert.

## Korrektur des Korpus-Manifestabschlusses

Der Windows-Lauf des Kandidaten `2347e1c81fcbdea15c55803b5d6f7d4c74b2d40d0ec1b28475af1dfbcbaa962c` bestand Clippy, Build und sämtliche Cargo-Testtargets. Alle zehn Projekte meldeten OK, Exit 0 und vollständige Ausgaben; der Korpus-SelfCheck blieb mit `manifest_writer_outcome_not_observed_for_all_projects` rot. Die zehn nachgereichten Result-Dateien stimmen in Bytezahl und FNV64 jeweils exakt mit dem Manifest überein. In jeder fehlen SilentVoice-, FinalBeamLostBegin- und FinalRestVerticalPlacement-Writerabschlusszeilen.

Ursache: Der bereits geschlossene FinalPdfSourceCommit trägt die wirklichen Writer-Audits. Die neue Release-E007-Fortschreibung erneuert den Commit durch die bestehende HorizontalLayout-Preservation. Deren Rückgabe ersetzte den gesamten Audittext durch einen einzelnen Layout-Hinweis. Der Rendererbericht wurde erst nach dieser Fortschreibung erzeugt; damit gingen die nötigen Writerzeilen auf dem Weg ins Manifest verloren.

Die Preservation übernimmt nun den bisherigen Bericht unverändert und hängt ihren Layout-Hinweis an. Kein Writer wird erneut ausgeführt, keine Zähler werden erfunden und keine Corpus-Admission-Regel geändert. XML-Witness, Continuation, Receipts und Closed-Status bleiben unverändert. Zwei Report-zu-Manifest-Regressionstests prüfen wiederholte Fortschreibung für schreibende und nicht schreibende Resultate sowie die weiterhin abgelehnte fehlende Writer-Evidenz. Aktueller Linux-Prüflauf: 1.135 Main-Tests mit 20 Threads bestanden; Test-Clippy und unveränderte Admission-/Build-Gates bestanden. Der erneute Windows-/Korpuslauf dieses neuen Pakets steht aus.
