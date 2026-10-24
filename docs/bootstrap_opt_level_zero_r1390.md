# r1390 – Bootstrap, Launcher und Gate-Build-Script wieder mit opt-level 0 (KodexOnly)

## Anlass

Seit dem 9. Oktober stürzen auf der Zielmaschine `rustc`, der Take-Bootstrap und das Build-Script
mit Zugriffsverletzungen ab (0xC0000005, wechselnde Offsets). Vor dem 9. Oktober gab es keine
Build-Abstürze. Am 9. Oktober wurde r1294 angenommen: Es setzte für Bootstrap, Launcher und
Build-Script opt-level 2. Für den Gewinn („Gate-Scans etwa zehnmal schneller“) gibt es keine
Messung. Er betrifft nur die Phase vor dem Korpuslauf.

Der Code ist `forbid(unsafe_code)`. Die Optimierung ist deshalb kein Programmfehler. Sie erhöht
aber die Compiler- und Speicherlast. Dieser Take nimmt sie als Stabilitätsversuch zurück, auf
Wunsch des Nutzers.

## Änderung

- `run.cmd`: `CARGO_PROFILE_DEV_OPT_LEVEL` und `CARGO_PROFILE_DEV_BUILD_OVERRIDE_OPT_LEVEL` von 2
  auf 0. Das Zurücksetzen vor dem EXE-Start bleibt unverändert.
- `src/bin/etuschlopf_take_bootstrap.rs`: Die Root-Cargo.lock-Prüfung setzt
  `CARGO_PROFILE_DEV_BUILD_OVERRIDE_OPT_LEVEL` auf 0 statt 2.
- `src/identity_gate_trust_anchor.rs`: Die Pins `RUN_CMD_FNV1A64` und `TAKE_BOOTSTRAP_FNV1A64`
  werden nachgezogen.

Nicht geändert:
- das produktive Strict-Profil (release + overflow-checks)
- der Strict-Build-Override des Launchers (opt-level 1)
- Gates, Writer, Musik

XML und PDF bleiben unverändert.

## Erwartung

- Die Gate- und Admission-Phase wird langsamer.
- Kompilieren und Korpus bleiben praktisch gleich.
- Bleiben die Abstürze danach aus, spricht das für Hardware am Lastlimit und nicht für einen
  Softwarefehler.
