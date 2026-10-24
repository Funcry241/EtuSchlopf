# DeepClippyMapUnwrapOrMechanicalReduction

## Scope

Dieser RefactorOnly-Take bearbeitet ausschliesslich die ausserhalb der
Security-Pfade von Deep Clippy als `clippy::map_unwrap_or` gemeldeten Ketten aus
`map(...).unwrap_or(...)` und `map(...).unwrap_or_else(...)`.

Der autoritative v0.10.1651-r06-Export meldete insgesamt 280 Diagnosen an 279
realen Quellstellen. Davon liegen 25 reale Fundstellen in geschuetzten
Security-Dateien; eine dieser Stellen wird durch die All-Targets-Kompilierung
zweimal gemeldet. Diese Security-Stellen bleiben unveraendert. Der Take ersetzt:

- 183 eager Fallback-Ketten durch `map_or(default, mapper)`,
- 71 lazy Fallback-Ketten durch `map_or_else(default, mapper)`.

Default-Ausdruck und Mapper werden unveraendert uebernommen; lediglich die von
Clippy vorgeschlagene Standardkombinatorform ersetzt die zweistufige Kette.
Zwischen `map` und `unwrap_or` befanden sich an keiner Zielstelle Kommentare.

## Begrenzung

Nicht Teil dieses Takes sind:

- alle Security-Pfade einschliesslich Launcher, Take-Control und Build-Gates,
- Sichtbarkeiten und `redundant_pub_crate`,
- Casts und numerische Grenzfaelle,
- `suspicious_operation_groupings`,
- Funktionszerlegung, Const-Erweiterungen oder boolesche Datenmodelle,
- Musiklogik, Kandidatenreihenfolge, Schwellenwerte, Writer oder Reportschemata,
- neue `allow`-Attribute.

## Statische Vorpruefung

- 254 Deep-Clippy-Diagnosen eindeutig aufgeloest,
- 254 nicht ueberlappende reale Ausdrucksspannen transformiert,
- 46 betroffene Rust-Dateien,
- alle geaenderten Pfade fuer `RefactorOnly` zulaessig,
- Klammer-/Kommentar-/String-Balance fuer alle geaenderten Dateien gueltig,
- physische Zeilenbilanz der 46 Rust-Dateien: netto -41 Zeilen.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
.\run.cmd --clippy-deep
```

Ein Korpuslauf folgt erst, wenn Build, Strict Clippy, Einzelprojektparitaet und
der erwartete Deep-Clippy-Rueckgang nachgewiesen sind.
