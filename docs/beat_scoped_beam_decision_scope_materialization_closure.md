# BeatScopedBeamDecisionScopeMaterializationClosure

## Gegenbeweis aus v0.10.1778

v0.10.1778 korrigiert die zu grobe Preserve-All-Hypothese von 1777: drei
ungebundene beziehungsweise beatuebergreifende Beam-Fragmente der Etuede-4-
Stressprobe werden wieder entfernt. Gegen die fachlich autoritative Referenz
v0.10.1776 r02 bleibt jedoch genau ein Beam-Delta bestehen:

```text
Takt 17, Staff 1, Voice 1

Onset 192  16tel B5   Beam 1 begin / Beam 2 begin
Onset 216  16tel Rest
Onset 240  16tel F#5  Beam 1 begin / Beam 2 begin
Onset 264  16tel B5   Beam 1 end   / Beam 2 end
```

Der Zustand ist final nicht geschlossen: ein zweites `begin` erscheint, bevor
das erste beendet wurde.

## Temporal Evidence Misbinding

Die read-only 1778-Projektion war auf finalen 1776-Artefakten ausgefuehrt worden.
Damit fehlte gerade die Beam-Kette, die im realen Pre-Polish-Zustand vorhanden
ist. Der Zielsystemlauf beweist den zeitlich korrekten Pfad:

```text
Pre-Polish:
B5 begin -> sichtbare Restluecke -> F#5 continue -> B5 end

NotationPolish Trichotomie:
Kette ist zu diesem Zeitpunkt strukturell vollstaendig
-> fremde Wahrheit bleibt erhalten

BeatScopedBeaming:
gleiche Staff/Voice/Beat-Gruppe
-> zusammenhaengender Run F#5/B5 wird als begin/end neu geplant
-> alter B5-begin vor der Pause blieb bisher unangetastet

Final:
staler Prefix
```

Die Trichotomie ist damit nicht widerlegt. Ihr Ergebnis wurde durch einen
spaeteren partiellen Writer invalidiert.

## Richtiger Owner

`BeatScopedBeaming` gruppiert bereits nach:

```text
Staff
Voice
Beat
```

und analysiert alle balkenfaehigen Kandidaten dieser Gruppe. Sobald mindestens
ein positiver Run akzeptiert wird, ist die ganze Kandidatengruppe der fachliche
DecisionScope des Owners.

Bisher galt physisch nur:

```text
positive Run-Mitglieder -> schreiben
alle uebrigen Kandidaten -> unveraendert
```

Damit war:

```text
WriteFootprint ⊂ DecisionScope
```

und historische Beam-Wahrheit konnte als staler Rest stehenbleiben.

Ab v0.10.1779 gilt:

```text
positive Run-Mitglieder
    -> BeamPlan kanonisch materialisieren

nicht ausgewaehlte Kandidaten derselben entschiedenen Gruppe
    -> explizite negative Beam-Entscheidung
    -> alte Beam-Tags entfernen

Staff/Voice/Beat-Gruppen ohne positiven Run
    -> byteidentisch
```

Formal:

```text
MaterializationScope(BeatScoped) = DecisionScope(BeatScoped)
```

fuer jede tatsaechlich akzeptierte Staff/Voice/Beat-Gruppe.

## Sicherheitsgrenze

Unveraendert bleiben:

- Pitch, Onset und Dauer,
- Staff und Voice,
- Chord-/Rest-Timeline,
- RawCursor,
- Clef- und Layout-Policy,
- Passreihenfolge,
- NotationPolish-Trichotomie,
- alle spaeteren committed-state Gates.

Es wird kein neuer Recovery-Writer eingefuehrt.

## Tests

Ein synthetischer Regressionstest bildet exakt den 1778-Gegenbeweis nach:

```text
16tel begin
16tel Rest
16tel continue
16tel end
```

BeatScoped akzeptiert nur den zusammenhaengenden Zweierlauf nach der Pause. Der
alte Prefix muss dabei seine Beam-Tags verlieren; der Zweierlauf wird begin/end.

Ein zweiter Test beweist Noninterference fuer Staff/Voice/Beat-Gruppen, in denen
BeatScoped ueberhaupt keinen positiven Run akzeptiert.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Vor einem Vollkorpus muessen gegen v0.10.1776 r02 insbesondere gelten:

- der einzelne 1778-Takt-17-Delta ist verschwunden,
- Master und Render sind nach Versionsnormalisierung wieder invariant,
- PDF-Seite 4 ist wieder pixelidentisch,
- FinalBeamLostBegin Candidates/Writes bleiben 0,
- CommittedHigherBeamCandidates bleibt 0,
- RawCursor und sounding event multiset bleiben gruen.
