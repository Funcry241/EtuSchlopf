# r1392: Bootstrap, Launcher und Gate-Build-Script mit opt-level 1 (KodexOnly)

## Anlass

Mit r1390 liefen Bootstrap, Launcher und Gate-Build-Script wieder mit opt-level 0. Die Läufe
seitdem waren absturzfrei, der Korpus ist gleich schnell geblieben und alle XML-Dateien sind
bytegleich. Auf Wunsch des Nutzers geht dieser Take auf den
Mittelweg opt-level 1. Damit bekommt die Gate-Phase eine Grundoptimierung, ohne die hohe
Compiler- und Speicherlast von opt-level 2 (r1294).

## Änderung

- `run.cmd`: `CARGO_PROFILE_DEV_OPT_LEVEL` und `CARGO_PROFILE_DEV_BUILD_OVERRIDE_OPT_LEVEL`
  gehen von 0 auf 1.
- `src/bin/etuschlopf_take_bootstrap.rs`: Die Root-Cargo.lock-Prüfung setzt
  `CARGO_PROFILE_DEV_BUILD_OVERRIDE_OPT_LEVEL` auf 1.
- `src/identity_gate_trust_anchor.rs`: Die Pins `RUN_CMD_FNV1A64` und `TAKE_BOOTSTRAP_FNV1A64`
  sind nachgezogen.

Unverändert bleiben das produktive Strict-Profil, Gates, Writer und Musik. XML und PDF ändern
sich nicht.

## Rückweg

Wenn wieder Build-Abstürze auftreten, wird auf opt-level 0 zurückgegangen (Inhalt von r1390).
