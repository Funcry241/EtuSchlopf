# r273 – Take-Rueckroll-Wiederanlauf-Invariantenbeweis

r273 basiert direkt auf dem real installierten und vollstaendig gruenen r272.

## Anlass

Der Vorfall r267/r270 zeigte eine Fehlerklasse, die nicht nur dokumentiert, sondern dauerhaft als ausfuehrbare Regressionsevidenz erhalten werden muss: Ein Kind-Take kann nach Installation scheitern; der Eltern-Bootstrap stellt den Source-Vollbaum wieder her, darf aber keine Paketartefakte des verworfenen Kindstands im dedizierten Bootstrap-Cargo-Ziel zuruecklassen, die den naechsten normalen Bootstrap vor der Paketwahl blockieren.

r272 hat die produktive Ursache geschlossen. r273 veraendert diese Logik nicht erneut, sondern beweist ihren tatsaechlichen Wiederanlaufvertrag.

## Ausfuehrbarer Beweis

Der neue integrierte Bootstrap-Test:

1. erzeugt getrennte temporaere Eltern- und Kind-Source-Baeume als minimale Cargo-Pakete `etuschlopf`,
2. bereitet eine echte `TakeTransaction` vor,
3. installiert den Kindbaum und setzt den Zustand `reexec`,
4. baut den Kindstand in `target/etuschlopf_bootstrap_cache`,
5. legt dort zusaetzlich einen paketfremden Sentinel an,
6. ruft die vorhandene `recover_pending_transaction` auf,
7. beweist die Rueckkehr des Eltern-Sources und die Entfernung des Pending-Markers,
8. beweist, dass keine `etuschlopf-*`-Paketfingerprints des Kindstands im Bootstrap-Cache verbleiben,
9. beweist zugleich, dass der paketfremde Sentinel erhalten bleibt,
10. baut den wiederhergestellten Elternstand im **selben** Bootstrap-Cargo-Ziel erneut erfolgreich.

Damit wird nicht nur die Quelltextreihenfolge aus r272 geprueft, sondern die paketbezogene Cargo-Bereinigung und der anschliessende Eltern-Wiederaufbau wirklich ausgefuehrt.

## Harte Invariante

> Ein Take-Rueckfall ist erst abgeschlossen, wenn der wiederhergestellte Elternstand ohne manuelle Cachepflege wieder aus demselben dedizierten Bootstrap-Cargo-Ziel gebaut werden kann.

## Nicht enthalten

- keine neue Rueckroll- oder Cache-Produktivlogik,
- keine Aenderung an `run.cmd`,
- keine Writer-Root-Pensionierung,
- keine Aenderung am Semantic-Capability-Diff,
- keine Registernotations- oder MusicXML-Mutation,
- keine Musik-, Renderer-, Solver-, Layout- oder Auswahlregel.

Der Test bezieht sich auf den Rust-/Cargo-Kern der Take-Transaktion. Er simuliert nicht die Windows-Shell selbst.
