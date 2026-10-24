# Source Release Freshness Contract

Diese Datei ist Teil des externen Workflow-Gedaechtnisses fuer EtuSchlopf.

## Kernregel

Ein Source-Release-ZIP und ein Ergebnisexport haben unterschiedliche Rollen:

- Source-Release-ZIP: wird mit `run.cmd --take` uebernommen.
- `assistant_export_latest.zip`: wird nach einem Lauf erzeugt und dient als Chat-/Thread-Uebergabe.

Daraus folgt: Ein Ergebnisexport darf zeitlich neuer sein als das Source-Release-ZIP, aus dem der lokale Stand entstanden ist. Das allein bedeutet nicht, dass das Source-ZIP veraltet war.

## Was wirklich geprueft werden muss

Ein Source-ZIP ist frisch, wenn seine internen Versions-/Metadaten konsistent sind:

- `Cargo.toml` Version
- README-Haupttitel
- `ZIP_KIND.txt` Version
- Launcher-Version
- Take-Bootstrap-Version
- Abschluss-/Uebergabehinweise
- Dateiname des Source-ZIPs

## Verbotene Schlussfolgerung

Nicht aus einem neueren `visual_preflight.result.txt`-Zeitstempel automatisch schliessen, dass das Source-ZIP falsch war. Der Result-Zeitstempel ist naturgemaess nach dem Take-/Run-Zeitpunkt.

## Richtige Schlussfolgerung

Wenn ein ZIP intern alte Metadaten mitfuehrt, ist das ein Release-Freshness-Fehler. Dieser Fehler muss vor dem Zippen erkannt oder mindestens im Root-Artefakt `SOURCE_RELEASE_FRESHNESS_AUDIT.txt` sichtbar werden.
