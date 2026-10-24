# RegisterNotation Silent Bridge Visible Re-Cue Last-Write Closure · r602

## Status

Historischer installierter MusicMutation-Zwischenstand auf r599 `RegisterNotationSilentBridgePersistentScopeRecue`; durch den vollkorpusqualifizierten r605 ueberholt. r602 besitzt weiterhin historische Authority fuer den sichtbaren Re-Cue und dessen Last-Write-Schutz, ist aber nicht mehr aktueller GreenBase.

r600 und r601 besitzen keine Authority:

- r600 wurde vor Installation wegen einer Take-Change-Control-Diffdeklaration abgewiesen.
- r601 bestand Pre-Take, Build, Clippy und alle Rust-Tests, scheiterte aber im realen Etüde-6-Programmlauf am finalen persisted-byte MusicXML-4.0-XSD-Gate.
- Der Take-Mechanismus rollte r601 zurück; GreenBase blieb r599.

## Frische r601-Evidenz

Der r601-Lauf zeigte die erste rote Ursache exakt in:

```text
Etüde Nr. 6
Master MusicXML
Takt 30
direction[2]/direction-type[1]

SchemaSchlopf r021 PersistedByteXsd40Gate:
child sequence does not satisfy bound content model: []
```

Build, Clippy und alle Rust-Testtargets waren davor grün.

Der Fehler war weder eine falsche Ottava-Scope-Entscheidung noch eine
SchemaSchlopf-Fehlinterpretation des gewünschten MusicXML-Modells.

Die Ursache lag in der EtuSchlopf-Last-Write-Reihenfolge:

1. Die r601-RegisterNotation-Materialisierung erzeugte am qualifizierten
   `octave-shift type="continue"` korrekt einen sichtbaren `<words>`-Re-Cue.
2. Der vorhandene Final-Export-Normalizer
   `remove_generated_octave_word_labels` entfernte danach weiterhin jedes
   textlich passende `8va`/`15ma`/`8vb`/`15mb` als historisches Legacy-Label.
3. Weil derselbe `direction`-Block zusätzlich das neue `octave-shift continue`
   enthielt, blieb das umschließende erste `<direction-type>` leer.
4. Das persistierte XML enthielt damit sinngemäß:

```xml
<direction placement="above">
  <direction-type></direction-type>
  <direction-type>
    <octave-shift type="continue" size="8" number="1"/>
  </direction-type>
  <staff>1</staff>
</direction>
```

5. SchemaSchlopf r021 wies dieses leere `direction-type` korrekt fail-closed ab.

Damit ist die tatsächliche Lücke:

```text
autorisierter neuer Re-Cue
    ↓
früher Readback GREEN
    ↓
Legacy Final-Export-Cleanup löscht Cue
    ↓
persistierte Bytes ROT
```

## Produktentscheidung

r602 verändert die musikalische Entscheidung aus r601 nicht.

Weiterhin gilt ausschließlich für bereits von r283 qualifizierte stille
Ottava-Brücken:

```text
Semantischer Scope:
Start -> ... -> Continue -> ... -> Stop

sichtbarer Re-Cue am Continue:
8va / 15ma / 8vb / 15mb
```

Neu ist nur die korrekte Owner-Grenze des bestehenden Legacy-Cleanups.

Ein Oktava-Wort bleibt genau dann vom historischen Cleanup verschont, wenn
derselbe `direction`-Block einen `octave-shift type="continue"` enthält.
Alle sonstigen historischen 8va/15ma/8vb/15mb-Textlabels werden weiterhin
entfernt.

Es entsteht:

- keine neue Pause-Schwelle,
- keine neue stille Brücke,
- kein Takt-/Etüden-Sonderfall,
- kein Offset,
- kein `default-y`,
- keine neue Geometrie,
- keine Stop/Start-Zerlegung,
- keine Änderung an Pitch, Rhythmus, Voice, Staff oder Registerwahl.

## Last-Write-Gate

r602 schließt zusätzlich die von r601 aufgedeckte Prüflücke:

Der finale `OctaveShiftExportGuard` prüft **nach** dem Final-Export-Cleanup
für jedes `octave-shift type="continue"` erneut:

```text
size=8  + placement=above -> exakt "8va"
size=15 + placement=above -> exakt "15ma"
size=8  + placement=below -> exakt "8vb"
size=15 + placement=below -> exakt "15mb"
```

Es muss in derselben `direction` exakt ein nichtleerer `<words>`-Text
vorliegen und dieser muss genau dem erwarteten sichtbaren Re-Cue entsprechen.

Fehlt der Cue nach dem Cleanup oder ist er widersprüchlich:

```text
OCTAVE_CONTINUE_RECUE_MISSING
```

und der Export bricht fail-closed ab.

Damit kann kein später Cleanup den Re-Cue unbemerkt wieder entfernen.

## Bestehender Legacy-Vertrag bleibt erhalten

`remove_generated_octave_word_labels` bleibt weiterhin Owner für alte
textbasierte Oktavierungslabels.

Die neue Ausnahme ist keine allgemeine Wörter-Whitelist. Sie ist an die
bereits vorhandene semantische MusicXML-Continue-Struktur gebunden:

```text
legacy words ohne continue
    -> entfernen

qualifizierter Re-Cue in direction mit continue
    -> erhalten
    -> final erneut validieren
```

Ein falsches oder zusätzliches Wort in einer Continue-Direction wird vom
finalen Re-Cue-Vertrag nicht akzeptiert.

## Tests

Der bestehende ExportGuard-Test wird auf den neuen finalen Produktvertrag
umgestellt:

- Continue mit `8va`-Re-Cue muss den Final-Export-Cleanup unverändert
  überleben.
- Der Cleanup muss dabei `0 alte Oktav-Textlabel entfernt` melden.
- Continue ohne sichtbaren Re-Cue muss final mit
  `OCTAVE_CONTINUE_RECUE_MISSING` scheitern.
- Die bestehenden Placement-/default-y-Gates bleiben unverändert.

## Erwartung für Etüde 6

Bei erfolgreichem Lauf weiterhin:

```text
SemantischePlaene 32
Sichtlinien         27
Starts              27
Continues            2
Stops               27
```

Neu muss zusätzlich gelten:

```text
Persisted Master XSD40: GREEN
Persisted Render XSD40: GREEN
```

und im realen PDF müssen die Re-Cues an den beiden qualifizierten Stellen
T30 und T34 erstmals sichtbar von r599 abweichen.

## Falsifikation

r602 ist verworfen, wenn:

- der XSD40-Gate erneut rot wird,
- der Re-Cue im PDF weiterhin unsichtbar bleibt,
- der Cue mit der Ottava-Linie kollidiert,
- der Cue als neuer Scope missverständlich wirkt,
- Layout-/Geometrie-Sonderregeln nötig würden.

Dann wird nicht mit Takt- oder Etüden-Hacks weitergearbeitet.

## Kern

r601 hat eine echte Last-Write-Owner-Kollision aufgedeckt:

```text
neuer autorisierter Carrier
!=
automatisch vom alten Cleanup als autorisiert erkannt
```

r602 schließt ausschließlich diese Kollision und bindet den sichtbaren
Continue-Re-Cue erstmals bis zu den tatsächlich persistierten finalen Bytes.
