# NonSecurityLiteralAndFnvHygiene

## Ausgangslage

Der bestaetigte v0.10.1660-Einzelprojektlauf endet mit Build 0, Strict Clippy
0 und FinalExit 0. Master- und Render-MusicXML sind nach Normalisierung des
Versionsstrings byteidentisch zu v0.10.1659; alle sechs PDF-Seiten sind bei
200 dpi pixelidentisch.

Der vorausgehende v0.10.1659-Deep-Clippy-Bericht enthaelt 95 echte
`unreadable_literal`-Warnbloecke. Die Security-Stufe v0.10.1660 schliesst 60
davon. Diese zweite Stufe bearbeitet die verbleibenden 35 All-Targets-Treffer
ausserhalb der Security-Domaene.

## Geschlossener Scope

Die 35 Diagnosen entsprechen 33 realen Quellstellen in 12 Dateien. Zwei
zusaetzliche Diagnosebloecke entstehen, weil `src/corpus_run_manifest.rs` in
zwei Targets kompiliert wird.

Die Aenderungen sind ausschliesslich wertneutrale Schreibweisen:

- FNV-Offset-Basis und FNV-Primzahl erhalten lesbare Unterstrichgruppen,
- zwei bekannte FNV-Vergleichswerte erhalten dieselbe Gruppierung,
- der konstante Iterations-Seed `780000usize` wird als `780_000_usize`
  geschrieben.

Numerischer Wert, Rust-Typ, Hashalgorithmus, Kontrollfluss, Schwellen,
Writer-Senken, Musikregeln, Passreihenfolge, Reports und Artefakte bleiben
unveraendert. Security-Dateien, Trust Anchors und Build-Pins bleiben
byteidentisch. Keine neue Allow-Direktive.

## Erwartete Diagnosewirkung

Vor der separaten Laufbestaetigung ist dies eine statische Erwartung:

- `unreadable_literal`: 35 -> 0 gegenueber dem Stand nach v0.10.1660,
- Deep Clippy gesamt: 1.833 -> 1.798, falls alle anderen Familien
  unveraendert bleiben.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Danach wird die aktuelle Diagnosezahl getrennt ermittelt:

```powershell
.\run.cmd --clippy-deep
```
