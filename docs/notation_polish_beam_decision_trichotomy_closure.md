# NotationPolishBeamDecisionTrichotomyClosure

## Basis und Gegenbeweis

Die fachlich autoritative dunkelgruene Referenz bleibt bis zur erfolgreichen Abnahme `v0.10.1776 r02`. Der aktuell installierte `v0.10.1777 r01 NotationPolishBeamWriteFootprintClosure` hat den Etuede-4-Stresstest mit
`FinalExit 0` beendet, aber nur `light_green`: der externe MuseScore-Prozess lieferte ein
gueltiges PDF und beendete sich danach mit `-1073741819`. Gleichzeitig war der Stressfall
nicht output-invariant zu `v0.10.1776 r02`: Master und Render enthielten vier zusaetzliche
Notenknoten mit insgesamt fuenf erhaltenen Beam-Tags; eine von sechs PDF-Seiten war
pixelverschieden.

Die vier erhaltenen Zustandsstellen liegen ausschliesslich in der von r01 neu freigegebenen
Nicht-Plan-Domain. Strukturell sind sie keine vollstaendig gebundene fremde Beam-Wahrheit:
es handelt sich um `begin`-Fragmente ohne Abschluss innerhalb derselben Staff/Voice/Beat-
Domain beziehungsweise um ein `begin/end`-Paar ueber eine Beat-Grenze. r01 hat damit
bewiesen, dass die Formel `kein positiver BeamPlan -> immer byteidentisch bewahren` zu grob
ist.

## Dreiteilige Beam-Entscheidung

Ab v0.10.1778 unterscheidet NotationPolish drei Zustandsklassen:

1. **Positive eigene Plan-Wahrheit**  
   Die Note liegt im bestehenden metrischen `BeamPlan`; vorhandene Beam-Tags werden wie
   bisher durch den kanonischen Plan ersetzt.

2. **Strukturell vollstaendig gebundene fremde Beam-Wahrheit**  
   Ausserhalb des positiven Plans wird eine Beam-Ebene nur dann als fremd und erhaltenswert
   behandelt, wenn `begin -> continue* -> end` innerhalb derselben Staff/Voice/Beat-Domain
   vollstaendig gebunden ist. Direkte zeitliche Nachbarschaft oder eine lueckenlos sichtbare
   Restabdeckung darf die Mitglieder verbinden. Diese Beam-Tags bleiben byteidentisch.

3. **Explizit negative, strukturell ungebundene Beam-Wahrheit**  
   Erkennbare `begin`/`continue`/`end`-Tags ohne vollstaendige Bindung derselben Ebene im
   selben Beat werden entfernt. Unbekannte Beam-Zustaende oder nicht interpretierbare Tags
   werden nicht destruktiv angefasst.

Damit gilt nicht mehr `kein positiver Plan -> negativ`, aber ebenso wenig `kein positiver
Plan -> immer bewahren`.

## Read-only Korpusprojektion

Auf den zehn autoritativen 1776-Master-Artefakten liegen ausserhalb des positiven lokalen
Polish-Plans 235 beamtragende Noten mit 445 Beam-Tags.

Die strukturelle Dreiteilung klassifiziert davon:

- 207 Noten / 391 Tags als vollstaendig gebundene fremde Beam-Wahrheit,
- 28 Noten / 54 Tags als ungebundene Beam-Fragmente.

Die bereits fuer r01 verwendete unmittelbare Projektion
`PartialBeatPostRest + BeatScopedBeaming` reproduziert den bekannten r01-Gegenbefund:
die reine Preserve-All-Variante laesst gegen das alte Verhalten sechs residuale
Master-Beam-Deltas im 10er-Korpus stehen. Mit der neuen Dreiteilung bleiben in derselben
Projektion **0 residuale Master-Beam-Deltas in 10/10 Projekten**.

Das ist kein Ersatz fuer den Zielsystemlauf. Es ist der statische Grund, warum r02/r1778
nicht zur global destruktiven 1776-Politur zurueckkehrt, sondern nur die tatsaechlich
ungebundene negative Domain zuruecknimmt.

## Invarianten

Unveraendert bleiben:

- Pitch, Onset, Dauer, Staff, Voice und Chord-/Rest-Timeline,
- RawCursor und Clef-Policy,
- positive metrische BeamPlan-Erzeugung,
- PartialBeatPostRest- und BeatScoped-Beam-Owner,
- alle spaeteren FinalRender-/Committed-State-Gates.

Der Wurf fuegt keinen Recovery-Writer hinzu.

## Abnahme

Erster Gegenbeweis bleibt der bereits verwendete Stressfall:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Erwartung gegen v0.10.1776:

- `FinalExit 0`,
- idealerweise wieder `dark_green` / Rendererprozess erfolgreich,
- die vier r01-zusaetzlichen Beam-Zustaende verschwinden,
- Master/Render sind nach Versionsnormalisierung wieder 1776-invariant,
- 6/6 PDF-Seiten pixelidentisch,
- `FinalBeamLostBegin Candidates/Writes = 0`,
- `CommittedHigherBeamCandidates = 0`,
- RawCursor und pitched Event-Multiset stabil.

Erst danach folgt der 10er-Korpus.
