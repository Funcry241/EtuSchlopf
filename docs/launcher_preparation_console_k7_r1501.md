# r1501 / K7: Konsole im Vorlauf

Parent: accepted r1500 (Stufe-A-Definition), Kandidat-SHA `b302a5cda9b82efd118b4f9024cdef1c59ab2ef6900a74e90a275038219a561c`.
Zeitmessungen unten stammen aus dem Take-Lauf von r1499.
ChangeClass: OutcomeProjection. Reine Konsolenprojektion; keine Musik-, Gate-, Transaktions-,
Recovery-, Build-Reuse- oder Release-Wirkung.

## Anlass

Ein Take-Lauf auf r1499 zeigte rund drei Minuten fast ohne Konsolenausgabe:

| Abschnitt | Dauer (r1499-Lauf) | Sichtbar |
| --- | --- | --- |
| run.cmd: Cargo-Build Take-Bootstrap | 27 s | eine Zeile "wird vorbereitet" |
| Bootstrap vor Reexec, davon Entpacken 1 s, Abhaengigkeiten 15 s, Take-Zulassung 17 s, Sicherung 11 s, Uebernahme 1 s, Cargo.lock/Kodex-Gates 38 s | 94 s | "Paket: ..." und "Installiert · Reexec" |
| run.cmd: Cargo-Build Launcher | 50 s | eine Zeile "wird vorbereitet" |

## Aenderung

1. `src/bin/etuschlopf_take_bootstrap.rs`: `bootstrap_step(running, done)` schreibt beim
   Beginn eines langen Schritts eine Zeile ("... wird geprueft ...") und `finish()` nach Erfolg
   eine Zeile mit Dauer ("... geprueft · 15 s"). Betroffen sind Entpacken, Dead-Code-Integritaet,
   Abhaengigkeiten, Take-Zulassung, Sicherung, Uebernahme/Pruefung und Cargo.lock/Kodex-Gates.
   Ein abgebrochener Schritt schreibt keine Fertig-Zeile; die vorhandene Fehlerzeile bleibt das
   letzte Wort. `startup_timing::Phase` bleibt unveraendert die advisory Messung.
   Wirksam ab dem ersten Take nach K7, weil jeder Take vom Bootstrap des Parents angewendet wird.
2. `src/bin/etuschlopf_launcher.rs`: nach der Kopfzeile eine Zeile `Vorlauf` aus dem bereits
   vorhandenen `startup_timing::readback()` der laufenden Session, z. B.
   `Vorlauf · Bootstrap-Build 27 s · Take-Pruefung 1 min 34 s · Launcher-Build 50 s`.
   Fehlende oder nicht erfasste Messwerte werden weggelassen, nie als 0 gezeigt.
3. `run.cmd`: die beiden stillen Cargo-Builds heissen jetzt "wird gebaut - Bibliothek +
   Kodex-Gates, Cargo ohne Zwischenausgabe". Nur die beiden `echo`-Texte; Cargo-Aufrufe, Log-
   Umleitung, Exit-Behandlung, Recovery-Reihenfolge und Cache-Retention sind bytegleich.
4. `src/identity_gate_trust_anchor.rs`: `TAKE_BOOTSTRAP_FNV1A64`, `LAUNCHER_FNV1A64` und
   `RUN_CMD_FNV1A64` auf die finalen Bytes nachgezogen; keine Pruefung entfernt oder gelockert.

Keine Musik- oder Selektor-Aenderung; die geaenderten Rust-Dateien sind ausschliesslich Kodex-Pfade.

## Abnahme

Strict-Tests (neue Tests: Dauerformat Bootstrap, Vorlauf-Zusammenfassung mit und ohne Take,
fehlende Messwerte), Strict-Clippy ohne Warnung, Korpus 10/10 dunkelgruen, 20/20 XML und PDFs
gegenueber r1500 unveraendert (Metadaten ausgenommen). In der Konsole des K7-Laufs muss die Zeile
`Vorlauf` erscheinen; die Schrittzeilen des Bootstraps erst beim folgenden Take.

## Nicht Teil von K7

Die Vorlaufzeit selbst. Die Bibliothek wird vor dem Launcher dreimal uebersetzt bzw. geprueft
(Bootstrap-Build, `cargo check --lib` der Cargo.lock-Pruefung, Launcher-Build). Erst mit
`cargo build --timings` messen; Optimierungsstufen bleiben unveraendert.
