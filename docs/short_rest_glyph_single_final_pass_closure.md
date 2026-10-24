# Short Rest Glyph Single Final Pass Closure

## Scope

`v0.10.1688 r03` ist die vollstaendige Single-Final-Pass- plus Compile-Closure desselben Take 2 des auf maximal drei Takes begrenzten
`RepairLoopOwnershipClosure`-Superwurfs. Take 1 (`v0.10.1687`) wurde im
Vollkorpus mit 10/10 Projekten, BuildExit 0, ClippyExit 0, FinalExit 0,
0 Blockerprojekten und 0 Warnprojekten bestaetigt. Gegen den korpusautoritativen
Stand `v0.10.1686` sind alle 20 Master-/Render-MusicXML-Dateien nach Entfernung
des reinen Software-Versionsstrings identisch; alle 57 PDF-Seiten sind
pixelidentisch.

## Korpusbeweis aus Take 1

Die neue `REPAIR-LOOP-INVOCATION-EVIDENCE` zeigt fuer den Short-Rest-Glyph-Pfad:

- Pre-Aufruf: 10/10 invoked, 0/10 written.
- Pre-Kandidaten existieren nur in Etuede 5 (79) und Etuede 8 (55), werden aber
  von den unveraenderten Invarianten verworfen und nie materialisiert.
- FragmentedRest schreibt in 6/10 Projekten.
- Der bisher bedingte Post-Aufruf laeuft deshalb in 6/10 Projekten und schreibt
  ebenfalls 0/10.
- Kein einziges Korpusartefakt benoetigt damit zwei produktive
  Short-Rest-Glyph-Positionen.

## Eigentumswechsel

Die bisherige Reihenfolge

`ShortRestGlyph (pre)`
-> `FragmentedRestChainTransactionalWrite`
-> bedingt `ShortRestGlyph (post)`

wird zu

`FragmentedRestChainTransactionalWrite`
-> `ShortRestGlyphSpecificPlacementCompileFix`
-> bestehender `PianoFinalRestGlyphCanonicalHeightRenderWrite`.

Der Short-Rest-Glyph-Algorithmus selbst bleibt byteidentisch. Candidate-Erkennung,
Rollenklassifikation, Glyph-Zielposition, Invarianten, Sounding-Signatur und
Write-Zulassung werden nicht geaendert. Es wird nur seine produktive Position
vereinheitlicht: genau ein Aufruf nach der strukturellen Fragmented-Rest-Mutation.

Der eine verbleibende Write benutzt die bereits vorhandene
`write_post_fragment_rest_glyph_checkpoint_authorized`-Authority. r03 ist direkt gegen den installierten Ausgangsstand `v0.10.1687` deklariert und enthaelt sowohl den Single-Final-Pass-Rueckbau als auch die daraus zwingend folgende Entfernung der toten fruehen Checkpoint-Authority. Der fruehe
`write_short_rest_glyph_checkpoint_authorized`-Aufruf ist damit nicht nur aus der
produktiven Stage entfernt: r03 entfernt auch die dadurch unreferenziert gewordene
Checkpoint-Authority, ihre Enum-Variante und ihren Re-Export. Diese Compile-Closure
aendert keinen produktiven Write-Pfad; sie schliesst ausschliesslich den vom
`deny(dead_code)`-/`deny(unused_imports)`-Gate sichtbar gemachten toten Rest.

## Evidence waehrend des Superwurfs

Die Take-1-Evidence bleibt absichtlich erhalten. Fuer Take 2 gilt nun:

- `ShortRestGlyphPre.invoked = 0`,
- `ShortRestGlyphPost.invoked = 1`, wenn eine finale Render-Quelle existiert.

`Post` bezeichnet waehrend des laufenden Superwurfs damit die einzig verbliebene
finale Position nach FragmentedRest. So ist im 10/10-Beweislauf direkt sichtbar,
dass der doppelte Invocation-Pfad physisch geschlossen wurde.

## Explizit unveraendert

Nicht geaendert werden:

- `build_short_rest_glyph_specific_placement_write`,
- Fragmented-Rest-Candidate- oder Mutationsemantik,
- PianoFinalRestGlyphCanonicalHeight,
- Staff/Voice/Pitch/Onset/Duration,
- Schwellenwerte oder Gewichte,
- musikalische Regeln,
- Releaseentscheidung,
- Render-/PDF-Auswahl,
- Shadow-Rest- oder Staccato-Schleifen.

## Zielsystembeweis

Abnahme mit genau einem Befehl:

```powershell
.\run.cmd --take --strict --all
```

Erwartet werden 10/10 OK, Build/Clippy/Final 0, 0 Blocker, 0 Warnungen,
versionsbereinigt identische Master-/Render-MusicXML und pixelidentische PDFs
zu `v0.10.1687`. Die Evidence muss pro Projekt genau einen finalen
ShortRestGlyph-Aufruf und keinen Pre-Aufruf zeigen.
