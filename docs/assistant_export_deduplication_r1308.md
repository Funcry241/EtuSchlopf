# r1308 AssistantExportDeduplication (KodexOnly)

Parent: accepted r1307 StartupTimingCleanup, SHA-256
1f61f4864153fd272472158f84fb9df6a97b25d39ac46af0fcfdbf5362964f19.
Version bleibt 0.10.1839; Musik unverändert.

Drei bislang gleiche ZIP-Veröffentlichungsblöcke werden in einem privaten Helper
zusammengeführt: vorhandene latest.zip entfernen, zip_directory_flat, vorhandene
Root-ZIP entfernen, latest.zip kopieren. Aufrufer behalten Handoff-Staging,
Anfangs-Cleanup, Dateischluss, Folge-Diagnose und Fehlerfortpflanzung an ihren
bisherigen Stellen. Normal-Run löscht alte ZIPs früher als Korpus/Preflight;
dieser Unterschied wird ausdrücklich nicht vereinheitlicht.

Projekt- und Korpus-Manifest teilen den bisherigen Included/Omitted/Upload-Footer
und die identischen Inventory-Felder. Reihenfolge, Leerzeilen, UTF-8, none-Fallbacks
und Upload-Hinweis bleiben gleich. Visual-Preflight behält seine abweichende kompakte
Omitted-Liste, Assistant-Context-Sektion und Upload-Instruktion. Snapshot-Felder
bleiben am Ort: ein zusätzlicher Helper dafür spart netto keine relevante LOC.

Keine Schema-, Export-Scope-, Receipt-, Cache-, Gate-, Release- oder Music-Änderung.
Die exakten Pin-Konstanten der geänderten Module werden synchronisiert.
Im Bootstrap-Test wird zusätzlich der versehentlich in r1307 eingefügte, nicht
ausgewertete prepare_timing entfernt. Der produktive Timer bleibt unverändert;
Clippy wird einschließlich aller Testtargets mit -D warnings ausgeführt.

Vorprüfung: sechs direkt gegen die Parent-Formatter ausgeführte Bytevergleiche
(leere/gefüllte Included- und Omitted-Listen einschließlich UTF-8; Inventory ohne
und mit optionalen Werten). Bestehende Export-/Budget-/Snapshot-Tests bleiben
unverändert und werden mit sämtlichen Cargo-Testtargets ausgeführt. Die LOC-Bilanz
zählt sämtliche geänderten Rust-Zeilen einschließlich Pin-Synchronisation.

Windows: accepted/FinalExit 0, 10/10, alle XMLs bytegleich zum Parent; erwartete
Manifest- und ZIP-Inhalte unverändert abgesehen von aktuellem Source-Snapshot und
Run-Provenienz. Der instrumentierte r1307-Parent liefert für diesen Folgetake die
Startup-Phasen inklusive Dependency-/Incoming-Admission, Backup und Apply;
ReexecChildWait bleibt im Child-Handoff-Capture ausdrücklich offen.
