# LegacyMusicMutationReportSemanticsClosure

## Problem

Der oeffentliche Projektbericht verwendete `MusicMutation` in historischen
Operatorsektionen mit verschiedenen Bedeutungen. Notations-only-Schreiber
meldeten teils `nein`, Beam-/Rest-Abschluesse teils `ja`, obwohl klingende
Ereignisse und Master-MusicXML unveraendert blieben. Das Feld konnte daher nicht
als gemeinsame Mutationsautoritaet gelesen werden.

## Abschluss

Werttragende historische Felder werden an der finalen Berichtsgrenze als
`LegacyMusicMutation` gekennzeichnet. Namen der Take-Klasse `MusicMutation`,
`DeclaredMusicMutation`, `NoMusicMutation` und `no_music_mutation` bleiben
unveraendert.

Eine eigene Sektion erklaert die autoritativen getrennten Dimensionen:

- `RenderXmlWrite`
- `RenderNotationMutation`
- `SoundingEventMutation`
- `MasterMusicXmlMutation`
- `DeclaredTakeChangeClass`

`LegacyMusicMutation` bleibt nur diagnostische historische Information und darf
keine dieser Dimensionen ersetzen.

## Grenzen

Keine Operatorvariable, kein Writer, keine Mutation, kein Kandidat und keine
Freigabeentscheidung wird geaendert. Der Take aendert ausschliesslich die finale
Berichtssemantik und bindet sie mit Regressionstests.
