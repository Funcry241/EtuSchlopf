# LegacyClefIslandStaffProjectionRetirement r305

## Ausgangspunkt

Parent ist der real installierte und im Vollkorpus bestaetigte r304
`CrossStaffClefIslandBeamMembershipPreservation`.

Der r304-`--all`-Lauf ist vollstaendig:

- 10/10 Projekte erkannt
- 10/10 `OK`
- 0 Blockerprojekte
- 0 Warnprojekte
- 10/10 `FinalClean`
- 10/10 Ausgaben vollstaendig
- Build/Strict-Clippy/Renderer gruen
- `FinalExit 0`
- 1064 integrierte Tests im r304-Einzelprojektlauf
- aktuelles Clippy-Budget 146 Diagnosen

r304 hat die vorhandene Beam-Wahrheit beim historischen Clef-Island-Stafftransfer
korrekt erhalten. Der Vollkorpus erlaubt nun die vorgelagerte Entscheidung selbst
statt nur ihren Beam-Nebeneffekt zu beurteilen.

## Korpusbefund

Ein identity-freier Master-vs-Render-Vergleich der aktuellen zehn Projekte findet
genau vier gepaarte Pitch/Onset/Dauer/Voice-Ereignisse, deren Staff im Render von
2 nach 1 wechselt und deren Beam-Zugehoerigkeit durch r304 erhalten bleibt:

- Etuede 5: ein Ereignis
- Etuede 6: ein Ereignis
- Etuede 8: zwei Ereignisse

Diese Stellen sind Beweisinstanzen, keine Selektoren. r305 enthaelt weder
Projektname, Taktnummer, Pitch noch Locator als Entscheidungsbedingung.

## Architekturbeweis

Der zentrale `ClefReadability`-Block hat die finale F4/G2-Entscheidung bereits
auf einen writer-freien, scoreweiten Kurs umgestellt. Sein dokumentierter
Cutover-Vertrag lautet:

`ClefReadabilityObservation -> ClefReadability course solver -> complete F4/G2 plan
-> existing Structural Write Authority -> MusicXML`.

Der gleiche Vertrag grenzt den erlaubten Output-Delta auf Clef-Surfaces ein und
fordert Pitch, Onset, Dauer, Voice und Staff als invariant.

Die aeltere Same-Measure-Strecke in
`notation::decompose_lower_staff_clef_islands()` verletzt diese Owner-Grenze
weiterhin: `project_short_lower_staff_clef_island()` entscheidet lokal anhand
einer kurzen G->F-Insel und projiziert vorhandene Noten von Staff 2 nach Staff 1.
Die historische Code-Kommentierung nennt als Motivation, dass eine reine
BeamTagRewrite-Politur im PDF nicht sichtbar genug gewesen sei.

Damit ist diese Staffprojektion eine ueberlebende lokale Render-Mutation unterhalb
des inzwischen autoritativen ClefReadability-Kurses. Sie besitzt kein eigenes
Recht, Staff als Notenwahrheit umzudeuten.

## r305-Aenderung

r305 pensioniert ausschliesslich die Same-Measure-Staffprojektion.

Entfernt werden:

- `decompose_lower_staff_clef_islands_in_measure()`
- `project_short_lower_staff_clef_island()`
- die r304-Regression, die Staff 2 -> Staff 1 noch als erlaubtes Verhalten
  festgeschrieben hatte

Ersetzt wird die Regression durch eine allgemeine Retirement-Pruefung:

- eine beliebige kurze beamed Staff-2-G-Schluesselinsel bleibt Staff 2
- ihre Beam-Level bleiben erhalten
- der lokale Same-Measure-Pfad entfernt nicht mehr eigenmaechtig G/F-Clef-Surfaces

Die bestehenden taktuebergreifenden Clef-Surface-Helfer und die
Registerdruck-Eintrittslogik bleiben bewusst ausserhalb dieses Takes. Sie
veraendern keine Note auf einen anderen Staff und koennen im weiteren Deep-Cleanup
separat gegen den zentralen ClefReadability-Owner geprueft werden.

## ChangeClass

`MusicMutation`

Grund: Der aktuelle Render-Output darf bewusst Staff-2->Staff-1-Deltas verlieren
und dadurch PDF-Layout veraendern. Das ist kein `RefactorOnly`-Versprechen.

Keine neue Writer-Authority, kein neuer Repair-/Recovery-Pass, keine neue
Shadow-Wahrheit und keine Gate-Abschwaechung werden eingefuehrt. Im Gegenteil:
eine bestehende lokale Mutation wird entfernt.

## Erwartete Invarianten

Master-MusicXML:

- unveraendert

Render-MusicXML:

- Pitch unveraendert
- Onset unveraendert
- Dauer unveraendert
- Voice unveraendert
- Beam-Wahrheit unveraendert
- Same-Measure-Clef-Island-Staffprojektionen werden nicht mehr erzeugt

Pianistik:

- kein Eingriff in r292-r300
- keine Hand-/Finger-/Capability-Aenderung
- kein Playability-Claim

Security:

- keine Security-Datei
- keine Gate-Aenderung
- keine Lint-Unterdrueckung

## Akzeptanz

Zuerst:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Entscheidend ist die sichtbare Takt-29-Probe: Das bekannte F#5 soll wieder auf
dem unteren Staff stehen; Pitch/Timing/Voice/Beam bleiben erhalten.

Bei gruenem Einzelprojekt folgt wegen der nachgewiesenen Korpuswirkung ein
erneuter:

```powershell
.\run.cmd --all
```

Dabei muessen insbesondere die drei betroffenen Projekte gemeinsam visuell und
formal bewertet werden.
