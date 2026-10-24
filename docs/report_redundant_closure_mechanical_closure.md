# ReportRedundantClosureRefactorClosure

## Scope

r01 wurde vor jeder Installation abgelehnt, weil `MechanicalOnly` die beiden
semantisch inventarisierten Dateiflaechen nicht gemeinsam aendern darf. r02 ist
der inhaltlich identische `RefactorOnly`-Take und entfernt genau dreizehn
redundante Closures im
gemeinsamen Report-Owner. Fuenf produktive Stellen in `src/report.rs` reichen
`char::is_ascii_digit` direkt an `take_while` weiter. Acht bestehende Tests in
`src/final_report.rs` reichen `ToOwned::to_owned` direkt an `compact` weiter.

Die Transformationen sind mechanisch gleichwertig:

```rust
.take_while(|ch| ch.is_ascii_digit())
```

wird zu:

```rust
.take_while(char::is_ascii_digit)
```

und:

```rust
.compact(|text| text.to_owned())
```

wird zu:

```rust
.compact(ToOwned::to_owned)
```

## Begrenzung

Nicht veraendert werden Reportinhalt, Locator-Maskierung, Abschnittsreihenfolge,
Validierung, MusicXML, Writer, Musiklogik, Schwellenwerte, Passreihenfolge,
Security-Dateien oder `allow`-Direktiven. Keine weitere Deep-Clippy-Familie und
keine weitere `redundant_closure_for_method_calls`-Stelle gehoert zu diesem Take.

## Erwartung

Die dreizehn physischen Stellen koennen durch All-Targets-Kompilierungsrollen
mehr als dreizehn Diagnosen repraesentieren. Ein exakter Abbau wird deshalb erst
durch den realen Deep-Clippy-Lauf behauptet. Der Strict-Einzellauf muss dieselbe
Report-, MusicXML- und PDF-Semantik bestaetigen.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```
