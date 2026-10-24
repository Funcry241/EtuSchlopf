# r535 – FinalNotationLowerStaffClefBeamBoundaryForeignTruthCutover

Parent ist der real installierte und im Einzelprojektlauf dunkelgrüne
`v0.10.1839 r534`
`FinalNotationLowerStaffClefBeamBoundaryForeignTruthAuditPackagingRecovery`.
Der vorangegangene r533-Candidate bleibt rejected und ist kein Parent.

## Anlass

Der r534-Audit beobachtet im finalen Render-MusicXML von Etüde 8 zwei
vollständige fremde Beam-Ketten auf Staff 2, die eine explizite
Staff-2-Clef-Grenze überspannen:

- Takt 25 bei Position 336: G2-Schlüssel, Beam-Level 1;
- Takt 26 bei Position 264: F4-Schlüssel, Beam-Level 1.

Beide Grenzen fallen zugleich mit einer unteren Octave-Shift-Zustandsgrenze
zusammen. Der musikalische Clef-Entscheid wird nicht verändert.

## Root Cause

`polish_measure()` segmentiert seine eigenen positiven BeamPlan-Gruppen bereits
mit `collect_lower_staff_clef_boundaries()` und
`lower_staff_clef_beam_segment()`. Eine echte Staff-2-Clef-Grenze kann deshalb
keine neue kanonische BeamPlan-Gruppe überspannen.

Die spätere Trichotomie für bereits vorhandene fremde Beam-Wahrheit verwendete
dieselbe Segmentgrenze bisher jedoch nicht. Eine vorhandene vollständige
`begin ... end`-Kette konnte dadurch als strukturell gebunden erhalten bleiben,
obwohl sie genau jene Clef-Grenze überspannte, an der der positive BeamPlan
bereits getrennt wird.

## Closure

r535 gibt `structurally_complete_external_beam_levels()` dieselbe bereits
vorhandene Lower-Staff-Clef-Segmentierung wie dem positiven BeamPlan. Die
Foreign-Truth-Preservation gruppiert damit nach

```text
Staff × Voice × Beat × LowerStaffClefSegment
```

statt nur nach

```text
Staff × Voice × Beat
```

Eine fremde Beam-Kette, die eine Staff-2-Clef-Grenze überschreitet, ist damit
innerhalb keines kanonischen Segments mehr vollständig und wird wie jeder andere
ungebundene Beam-Fragmentzustand entfernt. Vollständige fremde Beam-Wahrheit
ohne Clef-Grenze bleibt erhalten.

Es entsteht keine neue Clef-, Octave-, Rhythmus- oder Beam-Entscheidungs-
Authority. Die produktive Änderung schließt nur die bereits vorhandene
Clef-Grenze für den zweiten Materialisierungspfad derselben Beam-Trichotomie.

## Invarianten

Unverändert bleiben:

- Pitch und klingender Eventbestand;
- Onset und Dauer;
- Staff und Voice;
- Clef-Entscheidung und Clef-Materialisierung;
- Octave-Shift-Entscheidung und -Materialisierung;
- Beat-/Metric-Beam-Authority;
- Master-MusicXML.

Bewusst ändern darf sich ausschließlich die sichtbare Beam-Materialisierung im
Render-MusicXML/PDF an bereits existierenden Staff-2-Clef-Grenzen.

Der r534-Audit bleibt read-only Evidenz. r535 importiert oder liest keinen
Audit-Report und erzeugt keine Audit→Writer-Abhängigkeit.

## Regression

Die Notation-Regression bindet zwei positive/negative Nachbarn:

1. Eine vollständige fremde Lower-Staff-Beam-Kette ohne Clef-Grenze bleibt
   byteinhaltlich erhalten.
2. Eine vollständige fremde Lower-Staff-Beam-Kette über einer expliziten
   Staff-2-Clef-Grenze verliert ausschließlich ihre Beam-Tags; Clef und
   Notenpayload bleiben erhalten.

`ChangeClass=MusicMutation`, weil Render-MusicXML und PDF an dieser bestehenden
strukturellen Grenze sichtbar anders werden dürfen.
