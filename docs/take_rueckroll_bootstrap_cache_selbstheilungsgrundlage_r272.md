# r272 – Take-Rueckroll-Bootstrap-Cache-Selbstheilungsgrundlage

r272 basiert direkt auf dem weiterhin autoritativen und real gruenen r265.

## Anlass

r270 bewies den eigentlichen Infrastrukturfehler erneut: Der Take wurde korrekt installiert, der aktualisierte Stand lief durch Clippy und Build, scheiterte danach aber in einem integrierten Rust-Test. Der noch laufende Eltern-Bootstrap stellte daraufhin den r265-Quellbaum wieder her. Das separate Cargo-Ziel `target/etuschlopf_bootstrap_cache` enthielt danach jedoch weiterhin EtuSchlopf-Paketartefakte des zwischenzeitlich installierten Standes. Beim naechsten `--take` lief deshalb ein alter Security-Buildskript-Pin gegen den wiederhergestellten r265-Scanner und blockierte bereits vor der Paketwahl.

Die entscheidende Transaktionsgrenze lautet: Der Rollback eines Takes wird vom bereits laufenden Bootstrap des Elternstands ausgefuehrt. Eine Aenderung in einem noch nicht freigegebenen Kind-Take kann daher ihren eigenen fehlgeschlagenen Rollback nicht absichern. Die Absicherung muss zuerst als eigener gruener Elternstand installiert werden.

## Aenderung

`TakeTransaction::rollback` bleibt in derselben Reihenfolge fail-closed:

1. offene Public-Artefakte wiederherstellen,
2. Transaktionszustand `rolling_back`,
3. gesicherten Source-Vollbaum wiederherstellen,
4. wiederhergestellten Vollbaum bytegenau verifizieren,
5. **erst danach** ausschliesslich das Paket `etuschlopf` im dedizierten Cargo-Ziel `target/etuschlopf_bootstrap_cache` mit `cargo clean --package` ungueltig machen,
6. erst bei erfolgreicher Bereinigung Zustand `rolled_back`, Marker und Transaktionsordner abschliessen.

Der laufende Bootstrap selbst liegt in einem separaten Laufordner und wird nicht aus dem bereinigten Cargo-Ziel ausgefuehrt. Abhaengigkeitsartefakte, das normale `target`, Recovery-EXE, Transaktionsjournal und Public-Artefakte werden nicht geloescht.

Wenn die paketbezogene Cargo-Bereinigung fehlschlaegt, gilt auch der Rollback als fehlgeschlagen; der Abschlussmarker wird nicht vorzeitig entfernt.

## Beweisgrenze

Ein neuer integrierter Bootstrap-Test prueft quelltextgebunden, dass Source-Restore und Byteverifikation vor der Cache-Bereinigung liegen und dass `rolled_back` erst danach geschrieben wird. Ausserdem ist die Bereinigung auf `cargo clean --package <CARGO_PKG_NAME> --target-dir target/etuschlopf_bootstrap_cache` gebunden; ein pauschales `remove_dir_all` des Bootstrap-Caches ist ausgeschlossen.

## Nicht enthalten

- keine Writer-Root-Pensionierung,
- keine Aenderung am Semantic-Capability-Diff,
- keine Registernotations- oder MusicXML-Mutation,
- keine Aenderung an `run.cmd`,
- keine Musik-, Renderer-, Solver-, Layout- oder Auswahlregel.

r272 ist absichtlich kleiner als r269–r271. Erst wenn diese Infrastrukturgrundlage real gruen installiert ist, duerfen die getrennten Sicherheits- und Musikschritte wieder aufgenommen werden.
