# SecurityLiteralAndFnvHygiene

## Ausgangslage

Der autoritative Vollkorpusstand v0.10.1659 ist 10/10 gruen, besitzt
inhaltlich identische MusicXML-Ausgaben und 57/57 pixelidentische PDF-Seiten
gegenueber v0.10.1656. Der anschliessende Deep-Clippy-Lauf endet mit
`CargoExit 0`, 1.893 Diagnosen und 95 Treffern der Familie
`unreadable_literal`.

Die betroffenen Quellpfade liegen in Security- und Non-Security-Domaenen. Die
vorhandene Change-Class-Policy erlaubt keinen gemeinsamen Take ueber diese
Grenze. v0.10.1660 r01 bildet deshalb die geschlossene Security-Stufe; eine
spaetere Non-Security-Stufe darf erst vom bestaetigten Zielstand abgeleitet
werden.

## Geschlossener Scope

Diese Stufe normalisiert exakt 60 aktuelle All-Targets-Diagnosen:

- 24 in `src/identity_gate_trust_anchor.rs`,
- 13 in `src/identity_debt_build.rs`,
- 11 in `src/feedback_structuralization_gate.rs`,
- 7 in `src/bin/etuschlopf_launcher.rs`,
- 2 in `src/bin/etuschlopf_take_bootstrap.rs`,
- 3 in den von `identity_debt_build.rs` erzeugten Rust-Konstanten.

57 bestehende Zahlenliterale erhalten ausschliesslich lesbare
Unterstrichgruppen. Die drei generierten FNV-Konstanten werden weiterhin aus
denselben `u64`-Werten erzeugt, nun aber bereits als vier gruppierte
Hexadezimalbloecke geschrieben. Numerischer Wert, Typ, Kontrollfluss,
Hashalgorithmus und Vergleichssemantik bleiben identisch.

Die durch die Byteaenderungen betroffenen FNV-Pins und Trust Anchors werden auf
den vollstaendigen Zielbaum nachgezogen. Keine Musik-, Writer-, Schwellen-,
Passreihenfolge-, Report- oder Artefaktentscheidung wird geaendert. Keine neue
Allow-Direktive.

## Erwartete Diagnosewirkung

Vor dem bestaetigenden Lauf ist dies eine statische Erwartung:

- `unreadable_literal`: 95 -> 35,
- Deep Clippy gesamt: 1.893 -> 1.833,
- alle anderen Lintfamilien unveraendert.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Nach gruenem Installationslauf folgt die Diagnosemessung getrennt mit:

```powershell
.\run.cmd --clippy-deep
```
