# BeatScopedRestCompletionVoiceGapDecisionScopeClosure

## Anlass

Frische Evidenz aus Etuede 3, Takt 19 zeigt einen realen Producerfehler vor dem spaeten `CursorLaneEndInvariantRepair`. Der vorhandene Writer-Trace beweist die Reihenfolge:

- bis `register_octave_suppression`: RawCursor 384/384, keine ueberlappenden backup-getrennten Staff/Voice-Lanes;
- `final_notation`: RawCursor waechst auf 648 divisions;
- erst `cursor_lane_repair`: der 648er Ueberhang wird auf 384 reankert und die dadurch verschobene zweite Lane ueberlagert die erste sichtbar.

Der eigentliche Producer ist damit die in `final_notation` ausgefuehrte BeatScoped-Rest-Completion, nicht der spaete Cursor-Repair.

## Ursache

`plan_visible_voice_rest_completion_gaps()` bewertet sichtbare Luecken je `(Staff, Voice)`. Bei einer Voice, die innerhalb derselben MusicXML-Lane zwischen den beiden Klavierstaffs wechselt, kann ein staff-lokaler sichtbarer Gap jedoch klingendes Material derselben Voice auf dem anderen Staff enthalten. Der bisherige Planner materialisierte dort trotzdem sichtbare Pausen. Diese Pausen ruecken den globalen MusicXML-Cursor weiter und koennen einen zuvor metrisch gruenen Takt ueber seine Soll-Dauer hinausschieben.

## Closure

Die bestehende Rest-Completion bleibt einziger Owner. Keine neue Repair-/Recovery-Schicht entsteht.

1. Ein staff-lokaler Gap gilt nur noch als Voice-Stille, wenn innerhalb des Spans **kein** gepitchtes Material derselben Voice liegt - unabhaengig von Staff, Hidden-Status oder Chord-Rolle.
2. Die bestehende Materialisierung besitzt zusaetzlich eine read-only Postcondition: gepitchte Timeline muss identisch bleiben; ein vor der Rest-Completion metrisch gruener Cursor darf durch sie nicht rot werden.
3. Verletzt ein Kandidat diese Bedingungen, bleibt der pre-owner Takt unveraendert. Es wird kein spaeter Reparaturwriter gerufen oder erweitert.

## 1793-No-op wird entfernt

Der in v0.10.1793 r03 eingefuehrte Source-Lane-Voice-Split wird vollstaendig zurueckgenommen. Der 1793-Einzellauf zeigte `CandidateRows 0 / MasterApply nein`; der Trace beweist nun, dass die sichtbare Kollision erst nach dieser Grenze durch FinalNotation-Rest-Completion entsteht. Die nicht wirksame Voice-Mutation bleibt daher nicht als tote Parallelhypothese im Produktcode.

## Invarianten

Unveraendert bleiben als Entscheidung:

- Pitch;
- klingender Onset und Dauer;
- Staff und Voice;
- Chord-/Hidden-Rolle;
- Beam-Owner;
- Clef-Owner;
- Security-/Capability-Policy;
- Passorder und Thresholds.

Der Take aendert nur, ob eine sichtbare Rest-Completion ueberhaupt zulaessig ist.

## Regression

Ein generischer Cross-Staff-Same-Voice-Test bindet die Fehlerklasse ohne Etueden-/Takt-/Pitch-Selektion: alternierendes klingendes Material derselben Voice auf Staff 1/2 darf nicht als staff-lokale Stille interpretiert werden; der Cursor bleibt 384/384.

## Erste Abnahme

```powershell
.\run.cmd --take --strict --writer-trace=19 "Etüde Nr. 3"
```

Erwartung fuer Takt 19:

- `final_notation` erzeugt keinen 648-divisions-Ueberhang mehr;
- `cursor_lane_repair` muss fuer diese Fehlerklasse nicht mehr eingreifen;
- keine gleichzeitig klingenden backup-getrennten Noten mit gleicher Staff-/Voice-Identitaet entstehen durch Reanchor;
- PDF zeigt keine uebereinander geschobenen 16tel-Linien am Taktende.
