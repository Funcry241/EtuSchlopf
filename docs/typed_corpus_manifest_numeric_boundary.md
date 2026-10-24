# TypedCorpusManifestNumericBoundary

Stand: v0.10.1662 r01

## Ausgangslage

Der bestaetigte v0.10.1661-Deep-Clippy-Lauf enthaelt 1.798 Diagnosen. Die
Familie `cast_possible_truncation` umfasst 112 All-Targets-Treffer. Davon
entstehen 52 Treffer aus 26 realen Konvertierungen in
`src/corpus_run_manifest.rs`, weil dieselbe Quelle in zwei
Kompilationsidentitaeten verwendet wird.

Alle betroffenen Werte werden als JSON-`u64` gelesen und anschliessend mit
`as u32` oder `as usize` in die typisierten Manifeststrukturen uebernommen.
Auf 64-Bit-Systemen bleiben die `usize`-Werte meistens erhalten; auf
32-Bit-Systemen koennen sie jedoch abgeschnitten werden. Ein zu grosser
`schema_version`-Wert konnte auf allen Plattformen in einen anderen `u32`-Wert
ueberlaufen.

## Aenderung

Die Manifest-Eingangsgrenze besitzt nun zwei zentrale Konvertierungshelfer:

- `json_u32_field`
- `json_usize_field`

Beide lesen zuerst weiterhin ueber `json_u64_field`. Erst danach erfolgt eine
gepruefte `TryFrom`-Konvertierung. Ein Wert ausserhalb des Zielbereichs wird
nicht mehr still abgeschnitten, sondern mit einem feldgebundenen Fehler
abgelehnt:

```text
integer_field_out_of_range:<field>:target=<type>:value=<value>
```

Alle 26 betroffenen Felder in Projektzeilen und Korpusprojektion verwenden
diese Grenze. Zeitstempel, Dateigroessen und andere bewusst als `u64`
modellierte Werte bleiben unveraendert.

## Testvertrag

Die vorhandene Roundtrip- und Manipulationserkennung bleibt bestehen.
Zusaetzlich pruefen fokussierte Tests:

- `u32`-Ueberlauf wird mit Feld, Zieltyp und Wert abgelehnt.
- Der maximale plattformgueltige `usize`-Wert wird akzeptiert.
- Auf 32-Bit-Zielen wird ein nur auf 64 Bit darstellbarer Wert abgelehnt.

## Nicht geaendert

Keine Writer-Entscheidung, Musikregel, Passreihenfolge, Reportprojektion,
Hashbildung, Manifeststruktur oder JSON-Feldbezeichnung wird geaendert. Es
wird keine neue Allow-Direktive eingefuegt.

## Erwartete Diagnosewirkung

Da die 26 realen Stellen in zwei All-Targets-Kompilationsidentitaeten
erscheinen, lautet die statische Erwartung:

- `cast_possible_truncation`: 112 -> 60
- Deep Clippy gesamt: 1.798 -> 1.746

Die Werte gelten erst nach einem separaten realen Deep-Clippy-Lauf als
bestaetigt.
