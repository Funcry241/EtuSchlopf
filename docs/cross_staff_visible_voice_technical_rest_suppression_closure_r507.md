# CrossStaffVisibleVoiceTechnicalRestSuppressionClosure

## Version

`v0.10.1839 r507`

## Frische Evidenz

Der bestaetigte r506-Stand ist 10/10 dunkelgruen. Eine anschliessende menschliche Sichtpruefung zeigt jedoch in Etuede 3, Takt 19, dass die im finalen Render-MusicXML vorhandenen Cross-Staff-Beam-Ketten im PDF als einzelne Faehnchen erscheinen.

Die Master->Render->PDF-Rekonstruktion trennt den Befund von einem reinen Beam-Planungsfehler:

- `CommittedCrossStaffBeamProjection` materialisiert fuer Takt 19 drei explizite Cross-Staff-Beam-Gruppen.
- Im selben finalen Takt liegt fuer Voice 1 am Taktanfang gleichzeitig eine sichtbare 16tel-Pause auf Staff 1 (0-24) und eine sichtbare Achtelpause auf Staff 2 (0-48).
- Voice 1 ist auf Staff 1 im gesamten Intervall 0-48 bereits sichtbar repraesentiert: Pause 0-24, danach klingendes Event 24-48.
- Die laengere Staff-2-Pause ist damit keine zusaetzliche Voice-Silence, sondern eine technische Staff-Projektionsspur, die in dieselbe logische Voice hineinragt.
- Im r506-Korpus tritt diese Konstellation genau zweimal auf: Etuede 3 Takt 19 und Etuede 8 Takt 45. Beide PDF-Stellen zeigen denselben fehlenden Cross-Staff-Beam-Eindruck. Die uebrigen untersuchten Cross-Staff-Beam-Masse ohne diese ueberlappende Same-Voice-Pause rendern ihre Balken sichtbar.

Die vorhandene `structural_cross_staff_covered_rest_lane_suppression` besitzt bereits die fachliche Ownership fuer technische Cross-Staff-Restspuren, erkennt bislang aber nur einen Rest, dessen gesamtes Intervall durch **klingende** Same-Voice-Events auf dem anderen Staff abgedeckt ist. Der reale Start-Silence-Fall `sichtbare Pause -> klingendes Event` wird deshalb nicht geschlossen.

## Ursache

Der bisherige Selector berechnet fuer einen sichtbaren Rest ausschliesslich die ueberlappende Dauer sichtbarer gepitchter Same-Voice-Events auf dem jeweils anderen Staff.

Damit gilt fuer den realen 0-48-Fall:

- Staff-2-Rest: Dauer 48,
- gegenueberliegende Staff-1-Pitch-Coverage: nur 24-48 = 24,
- vorhandene sichtbare Staff-1-Voice-Coverage: Pause 0-24 + Pitch 24-48 = 48.

Die bestehende Bedingung `pitched_coverage >= rest_duration` bleibt false und laesst die technische Staff-2-Pause bestehen, obwohl die logische Voice bereits vollstaendig sichtbar ist.

## Closure

r507 erweitert ausschliesslich den bestehenden Pure-Candidate-Owner `SilentVoiceCoveragePlanningCore`:

1. Ueberlappungsdauer wird als echte Intervall-Union berechnet, damit sich ueberlappende Fragmente nicht durch Summierung selbst vervielfachen koennen.
2. Die bisherige vollstaendige Pitch-Coverage bleibt unveraendert zulaessig.
3. Zusaetzlich darf ein technischer Cross-Staff-Rest unterdrueckt werden, wenn sein komplettes Intervall durch die **sichtbare Same-Voice-Timeline des anderen Staffs** abgedeckt ist und innerhalb dieses Intervalls mindestens ein sichtbares klingendes Same-Voice-Event liegt.
4. Der Pitch-Anteil ist ein fail-closed Guard: reine Rest-gegen-Rest-Ueberlappung darf nicht dazu fuehren, dass beide Darstellungen echter Stille verschwinden.
5. Die bestehende Structural Write Authority ersetzt weiterhin nur den selektierten technischen Rest byte-lokal durch einen daueridentischen `<forward>`-Carrier. Es gibt keinen neuen Writer und keinen neuen produktiven Caller.

Damit wird weder eine Beam-Kette neu erfunden noch ein Renderer-Sonderfall kodiert. Der vorhandene Cross-Staff-Beam-Plan bleibt unangetastet; lediglich die konkurrierende technische Same-Voice-Restprojektion wird im bereits dafuer vorgesehenen Owner geschlossen.

## Invarianten

- Pitch unveraendert,
- Onset unveraendert,
- klingende Duration unveraendert,
- Staff/Voice der klingenden Events unveraendert,
- Beam-Tags unveraendert,
- Tie-Tags unveraendert,
- Clef-/Octave-Shift-Semantik unveraendert,
- Raw-Cursor-Dauer des ersetzten Restes bleibt durch daueridentischen `<forward>` erhalten,
- keine neue Writer-/Repair-/Recovery-Authority,
- keine Projekt-, Takt-, Pitch- oder Etuedenidentitaet in produktiver Logik.

## Regression

Ein synthetischer Test bildet den generalisierten Start-Silence-Fall ab:

- Voice 1 Staff 1: sichtbare 16tel-Pause 0-24,
- Voice 1 Staff 1: klingendes 16tel 24-48,
- Voice 1 wechselt spaeter innerhalb des Taktes cross-staff,
- Voice 1 Staff 2: technische Achtelpause 0-48.

Er verlangt:

- keine Silent-Coverage-Materialisierung,
- genau eine Cross-Staff-Rest-Suppression,
- die laengere technische 48er-Pause wird zu einem daueridentischen `<forward>`,
- die echte 24er-Startpause bleibt erhalten,
- reine Rest-Ueberlappung reicht niemals als Loeschgrund; im zu schliessenden Intervall muss gepitchte Gegenstaff-Evidenz vorhanden sein.

## ChangeClass

`MusicMutation`.

Die Klasse ist erforderlich, weil die finale sichtbare Render-Notation durch Rest->Forward fuer einen bereits als technisch klassifizierten Cross-Staff-Carrier geaendert werden kann. Es gibt keine SoundingEventMutation und keinen MasterApply.

## Erste Abnahme

```powershell
./run.cmd --take "Etüde Nr. 3"
```

Danach visuell Takt 19 pruefen. Erwartung: Die bereits im Render-MusicXML geplanten Cross-Staff-Beams werden im PDF sichtbar zusammenhaengend gerendert; die echte Startpause der logischen Voice bleibt erhalten. Zusaetzlich Etuede 8 Takt 45 als unabhaengigen Comparator pruefen. Erst bei gruenem Zielbefund Vollkorpus als Breitenbeweis.
