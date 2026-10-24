# DeepClippyCargoShortProjection

## Ausgangslage

Der bestaetigte v0.10.1666-Einzellauf endet mit Build 0, Strict Clippy 0 und
FinalExit 0. Der anschliessende reale Deep-Clippy-Lauf meldet 1.705 Diagnosen
und erzeugt 1.026.914 Bytes auf 22.044 Zeilen. Gegenueber v0.10.1665 sinkt die
Diagnosezahl netto um elf: vier `option_if_let_else` und elf
`redundant_closure_for_method_calls` entfallen, waehrend vier neue
`unnecessary_map_or`-Hinweise entstehen.

## Scope

Der Deep-Clippy-Cargo-Aufruf verwendet kuenftig Cargo-eigenes
`--message-format short`. Dadurch bleibt jede Warnung mit Pfad, Zeile, Spalte,
Meldung und Lintkennung erhalten, waehrend wiederholte Quellenausschnitte,
Unterstreichungen und ausfuehrliche Hilfebloecke nicht mehr in die Root-Datei
geschrieben werden.

Unveraendert bleiben:

- `--all-targets`,
- `clippy::pedantic`, `clippy::nursery` und `clippy::cargo`,
- CargoExit und Fehlerweitergabe,
- Shared BuildPlan und anschliessende Buildfrische,
- die Warnungszaehlung und Cargo-Summary-Bereinigung,
- Musikpipeline, Writer, Reports, Schwellenwerte und `allow`-Direktiven.

Die Warnungszaehlung erkennt sowohl die bisherige humane Cargo-Darstellung als
auch die kurze Form `path:line:column: warning: ...`. Ein fokussierter
Launcher-Test haelt beide Formen fest. Es wird kein eigener Diagnoseparser,
keine zweite Clippy-Ausfuehrung und keine neue Rohlog-Schattenstruktur
eingefuehrt.

## Erwartung

Die bisherige Datei von rund 1,03 MB/22.044 Zeilen sollte durch die
Cargo-native Kurzprojektion auf grob 200 bis 300 KB und etwa 2.000 Zeilen
sinken. Der verbindliche Wert entsteht erst durch den realen
v0.10.1667-Deep-Clippy-Lauf. Die echte Warnungszahl darf durch die
Darstellungsoption nicht sinken; etwaige Quellcode-Diagnosen des Launchers
werden weiterhin normal mitgezaehlt.

## Sicherheitsbindung

Der Launcher ist FNV-gepinnt. Deshalb ist dies ein koordinierter
`SecurityOnly`-Take: Launcher-FNV, Build-Policy-FNV und Trust Anchor werden
synchronisiert.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
.\run.cmd --clippy-deep
```
