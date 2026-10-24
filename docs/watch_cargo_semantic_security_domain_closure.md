# WatchCargoSemanticSecurityDomainClosure

Stand: v0.10.1814 r02

## Anlass

Der erste v0.10.1814-r01-Kandidat `WatchRust2024Resolver3RustVersionContract` wurde
auf dem realen Windows-Zielsystem vor jeder Installation korrekt fail-closed
abgewiesen.

Der v0.10.1813-Take hatte die separate Watch-Cargo-Grenze im
`take_change_control` bereits als Security-Pfad geschlossen und einen exakt begrenzten
spaeteren 2021->2024/MSRV-1.88/Resolver-3-Vertrag vorbereitet. Der unabhaengige
`semantic_capability_diff` besass jedoch weiterhin seine allgemeine Regel, nach der
sonstige verwaltete `tools/`-Dateien der Musik-Domaene zugeordnet werden. Deshalb
wurde eine Aenderung von `tools/etuschlopf_watch/Cargo.toml` gleichzeitig als
`MusicLogic` klassifiziert und unter `SecurityOnly` blockiert.

Diese Ablehnung ist ein wertvoller Gegenbeweis: Zwei Security-Komponenten hatten fuer
dieselbe konkrete Datei noch nicht dieselbe Domaenenwahrheit.

## Ziel dieses Takes

v0.10.1814 r02 migriert die Watch **noch nicht** auf Edition 2024.

Der Take schliesst ausschliesslich die semantische Domaenengrenze:

```text
tools/etuschlopf_watch/Cargo.toml -> Domain::Security
tools/etuschlopf_watch/Cargo.lock -> Domain::Security
```

im `semantic_capability_diff`.

Damit stimmen danach Take-Change-Control und Semantic-Capability-Diff fuer die beiden
separaten Watch-Cargo-Steuerdateien ueberein. Andere `tools/.../Cargo.toml`-Dateien
werden dadurch nicht pauschal zu Security-Pfaden; die Freigabe bleibt exakt auf die
beiden bereits benannten Watch-Pfade begrenzt.

## Kein Selbstautorisieren

Der r01-Fehler wird absichtlich **nicht** dadurch behoben, dass in einem Take zugleich

1. die semantische Klassifikation geaendert und
2. das Watch-Manifest auf Edition 2024 umgestellt

werden.

Das waere eine Selbstautorisierung derselben Revision. Deshalb bleibt
`tools/etuschlopf_watch/Cargo.toml` in v0.10.1814 r02 bytegleich auf Edition 2021.
Auch `tools/etuschlopf_watch/Cargo.lock` und der gesamte Watch-Rustcode bleiben
bytegleich.

Erst nach real gruener Windows-Abnahme dieses Domain-Closure-Takes darf ein
nachfolgender separater Take den bereits in v0.10.1813 etablierten geschlossenen
Marker verwenden und den Watch-Crate wirklich auf

```toml
edition = "2024"
rust-version = "1.88"
resolver = "3"
```

umstellen.

## Invarianten

- ChangeClass: `SecurityOnly`
- keine Watch-Runtime-Aenderung
- keine Watch-Manifest-Aenderung
- keine Lockfile-Aenderung
- keine Dependency-/Feature-Aenderung
- keine Musikmutation
- keine Writer-/Solver-/Repair-/Recovery-Aenderung
- keine Passorder-/Render-/Artifact-Aenderung
- bestehende negative Security-Pruefungen bleiben erhalten
- neue Testinvariante beweist die semantische Security-Domain fuer genau die beiden
  Watch-Cargo-Pfade

## Abnahme

```powershell
.\\run.cmd --take "Etüde Nr. 6"
```

Der normale Strict-Pfad bleibt die Freigabeautoritaet.
