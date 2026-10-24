# r1503 / K8: HIMEM-Konsole im Vorlauf

Parent: accepted r1502 (Anschlagsprojektion), Kandidat-SHA
`a692ed470af17526c1fc1deb3463b5c356e5fec8bb91db44c7a4ea61923aa392`.
ChangeClass: OutcomeProjection. Reine Konsolenprojektion; keine Musik-, Gate-,
Transaktions-, Recovery-, Build-Reuse- oder Release-Wirkung.

## Ziel

Stille Wartezeiten im Vorlauf eines Takes fuellen, im Stil von HIMEM.SYS und dem
BIOS-Speichertest: eine Zeile je Schritt, die Sekunden zaehlen sichtbar hoch, am Ende
wird dieselbe Zeile zu "... fertig · N s". Ausdrueckliche Vorgabe: nur Lueckenfueller,
niemals blockierend.

## Aenderung

1. `src/bin/etuschlopf_take_bootstrap.rs`
   - `bootstrap_step(key, running)` ersetzt die K7-Zweizeilen-Ausgabe. An einem
     interaktiven Terminal (`ProgressMode::Inline`) schreibt ein eigener Thread die
     Schrittzeile einmal pro Sekunde neu: `Abhaengigkeiten werden geprueft ... 12 s ·
     zuletzt 15 s`. `finish()` setzt unter derselben Ausgabesperre ein Stopp-Flag und
     schreibt `... fertig · 15 s`; es wartet nie auf den Thread (kein Join, der Thread
     wird ueber das Fallenlassen seines Kanals geweckt). Kann der Thread nicht starten,
     entfaellt nur das Hochzaehlen.
   - Ohne Terminal (Umleitung, Log) gibt es keinen Thread und nur die Endzeile.
     Das technische Log erhaelt in beiden Faellen genau die Endzeile.
   - Andere Konsolenzeilen waehrend eines laufenden Schritts raeumen die Schrittzeile
     vorher weg und zeichnen sie danach neu; nichts laeuft ineinander.
   - Ein abgebrochener Schritt beendet nur seine offene Zeile; die vorhandene
     Fehlerzeile bleibt das letzte Wort, eine Fertigmeldung wird nicht behauptet.
   - "zuletzt"-Werte stehen best effort in `target/etuschlopf_logs/bootstrap_step_last.txt`
     (`key=ms`); fehlt oder bricht die Datei, entfaellt nur der Hinweis.
   - Lueckenschluss: zwei bisher ungezaehlte Abschnitte erhalten eigene Zeilen
     (Revisionsabdruck und Ablehnungsverlauf; Watch-Projekt-Pruefung).
   - Wirksam ab dem ersten Take nach K8, weil jeder Take vom Bootstrap des Parents
     angewendet wird.
2. `src/bin/etuschlopf_launcher.rs`: Die vorhandene `Vorlauf`-Auswertung schreibt die
   letzten Dauern der beiden Cargo-Builds aus `run.cmd` als ganze Sekunden best effort nach
   `target/etuschlopf_logs/take_bootstrap_build_last_s.txt` und `launcher_build_last_s.txt`.
3. `run.cmd`: liest diese Dateien mit `set /p` (nur Ziffern, sonst kein Hinweis) und zeigt
   `..., zuletzt 49 s ...` in den beiden Build-Zeilen. Kein Hintergrundprozess, kein Warten;
   Cargo-Aufrufe, Log-Umleitung, Exit-Behandlung, Recovery und Cache-Retention unveraendert.
4. `src/identity_gate_trust_anchor.rs`: Pins fuer Bootstrap, Launcher und run.cmd auf die
   finalen Bytes; keine Pruefung entfernt oder gelockert.

## Abnahme

Tests fuer Live-/Endtext, "zuletzt"-Datei (Lesen, Ersetzen nur des eigenen Schluessels,
kaputte Werte) und die ganzen Sekunden fuer run.cmd. Strict-Clippy ohne Warnung, Korpus
10/10, 20/20 XML bytegleich und PDFs pixelgleich zu r1502. Sichtbar im K8-Lauf: die
"zuletzt"-Angabe im Launcher-Build ab dem zweiten Lauf; die HIMEM-Zeilen beim folgenden Take.
