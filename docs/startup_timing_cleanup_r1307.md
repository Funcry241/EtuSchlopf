# r1307 StartupTimingCleanup (KodexOnly)

Parent: accepted r1306, Version bleibt 0.10.1839. Musik unverändert.

run.cmd erfasst vor Recovery nur lokale Uhrwerte; die bisherigen Recovery-Aufrufe
bleiben vor jedem neuen diagnostischen Dateischreiben und Cargo-Build. Danach
initialisiert der äußere Aufruf eine Diagnose-Sitzung unter target/etuschlopf_logs.
Die Kennung wird ausschließlich für diese Projektion an den Child vererbt.
Jeder run.cmd-Aufruf hat eine eigene Invocation. Cargo-Aufrufe, Profile, Argumente,
ERRORLEVEL-Auswertung, Gate-Reihenfolge, Backup, Apply und Rollback bleiben gleich.

Gemessen werden RunCmdBeforeChild (inklusive Recovery/Build/Setup),
TakeBootstrapCargoBuild und LauncherBootstrapCargoBuild: lokale Windows-Uhr,
10-ms-Auflösung, keine monotone OS-Prozesszeit. Datumssprung, rückwärts laufende
Uhr, fehlendes Paar oder veraltete Sitzung führen nicht zu einer behaupteten 0.
Vorwärtssprünge der Systemuhr sind mit dieser Uhr nicht erkennbar. RunCmdBeforeChild
enthält seinen Cargo-Build und darf damit nicht addiert werden.

Der Bootstrap beobachtet monoton TakeBeforeReexec, TakeExtraction,
DependencyAdmission, IncomingTakeAdmission, TransactionBackupPrepare,
ApplyAndVerify, LockAndReexecEvidencePreparation und ReexecChildWait. End bedeutet
Scope-Ende, nicht bestandenes Gate; existierende Exit-/Acceptance-Belege bleiben
allein entscheidend. TakeBeforeReexec enthält die Unterphasen, ReexecChildWait den
ganzen Child-Lauf. Kein gemeinsamer Instant-Zeitnullpunkt verschiedener Prozesse.

Der Launcher kopiert nur das bis dahin vorhandene, sitzungsgebundene Diagnose-Log
(maximal 256 KiB) in seinen Konsolenlog. STARTUP-TIMING-READBACK im Handoff stammt
aus diesem gebundenen, exportierten Konsolenlog. Kein späteres Umschreiben des
hashgebundenen Konsolenlogs. Offene Elternwartezeit sowie Acceptance-, finaler
ZIP- und Exit-Tail nach Handoff-Capture bleiben ausdrücklich offen. Die neue
Messung wird weder für Cache-Reuse noch Gate/Receipt/Release-Entscheidungen gelesen.
Testbinaries lesen/schreiben keine tatsächliche Sitzung des aufrufenden Launchers.

Installationsgrenze: Der laufende accepted r1306-Parent kennt diese Proben noch
nicht. Beim ersten r1307-Take misst der aktualisierte run.cmd den Launcher-Build;
Admission/Apply des alten Parents sind nicht nachträglich messbar. Ab dem nächsten
Take auf accepted r1307 sind beide Seiten instrumentiert. Fehlende Phasen stehen
als not_captured_or_still_open im Handoff, niemals als 0.

Aufräumen: Die zwei Quellsignaturfunktionen teilen ausschließlich ihre bisher
identische Datei-Byte-Kodierung. Dateiscope, Sortierung, Präfixe und Messgrenzen
bleiben getrennt und gleich. Projekt-/Korpus-Inventory teilen den bisherigen
reinen Konsolenformatter. Keine Signaturmemoisierung, neuen Reuse-Entscheidungen
oder erwartete relevante Beschleunigung. Golden-Fixture-Signaturen und exakter
Inventory-Text prüfen die bisherigen Bytes. Timing-Tests prüfen Invocation-
Bindung, Locale-Uhren, fehlende Sitzungen, Datumssprung und offenen Parent-Wait.

Windows-Abnahme: accepted/FinalExit 0, 10/10, vollständige Tests; alle 20 XMLs
bytegleich zum Parent; Startup-Readback im Handoff; bei diesem ersten Take die
fehlenden Parent-Phasen sichtbar, Launcher-Bootstrap-Build mit plausibler Dauer.
Ein Folgetake zeigt zusätzlich Admission/Backup/Apply vor dem Child.
