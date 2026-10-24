# ClefReadabilityOctaveShiftBoundaryDecisionScopeClosure

## Version

`v0.10.1839 r506`

## Frische Evidenz

Der bestaetigte r505-Stand ist 10/10 dunkelgruen und besitzt kein automatisch gewaehltes Reparaturziel. Die gezielte Sichtpruefung von Etuede 9, Takt 17/18, unteres System zeigt jedoch einen konkreten Master->Render-Fehler im produktiven `ClefReadabilityPrimaryCourse`.

Im Master endet in Takt 18 die laufende untere 8vb-Spanne unmittelbar vor dem vorhandenen G2-Clef-Surface. Eine Level-1-Balkengruppe beginnt noch innerhalb der 8vb-Spanne und endet erst nach deren Stop. Der aktuelle Clef-DecisionScope behandelt diese Balkengruppe als ungeteilte StructuralAnchor-Einheit. Dadurch wird die nach der 8vb-Grenze sinnvolle G2-Surface entfernt und an den vor der Grenze liegenden `beam 1 begin`-Attack zurueckverankert. Im finalen Render steht G2 deshalb bereits waehrend der noch aktiven 8vb-Spanne; die folgenden tief dargestellten Sechzehntel erhalten unnoetige Hilfslinien.

Der Fehler ist nicht etueden-, takt- oder pitchspezifisch. Er entsteht aus einer fehlenden Closure zwischen bereits vorhandener Tie-/Beam-Ankerlogik und der bereits beobachteten Lower-Staff-Octave-Shift-Semantik.

## Ursache

`scan_lower_pitches()` nutzt den aktiven unteren `octave-shift` bereits zur Ledger-/Registerbewertung, fuehrt die Start-/Stop-Grenze aber nicht als StructuralAnchor-Scope-Grenze. Offene Tie- und Primary-Beam-Urspruenge bleiben deshalb ueber einen Lower-Staff-Octave-Shift-Wechsel hinweg erhalten. Ein nach der Grenze entstehender Registerbedarf kann dadurch an einen vor der Grenze liegenden Tie-/Beam-Ursprung zurueckgebunden werden.

Gleichzeitig klassifiziert der Course-Solver einen direkt an der Octave-Shift-Grenze vorhandenen Clef-Surface bislang als `inside open tie/beam` und darf ihn deshalb nicht retainen. Diese beiden Regeln erzwingen gemeinsam genau das beobachtete Rueckankern.

## Closure

r506 erweitert ausschliesslich den DecisionScope des bestehenden `ClefReadabilityPrimaryCourse`:

1. Jeder beobachtete Lower-Staff-`octave-shift`-Start oder -Stop beginnt fuer Clef-Transition-Anker einen neuen StructuralScope. Offene Tie-/Primary-Beam-Anker werden fuer die Clef-Entscheidung an dieser Grenze verworfen; der musikalische Tie-/Beam-Inhalt selbst bleibt unveraendert.
2. Ein bereits vorhandener sichtbarer Lower-Clef-Surface darf trotz einer roh ueber die Grenze laufenden Tie-/Beam-Struktur genau dann als Boundary-Surface retained werden, wenn seit der letzten Lower-Staff-Octave-Shift-Grenze noch kein Lower-Staff-Note-Event vor diesem Surface liegt. Damit bleibt der direkt an der Anzeigegrenze vorhandene Clef eine kanonische Boundary-Option.
3. Spaetere Clef-Surfaces innerhalb derselben weiterlaufenden Tie-/Beam-Struktur bleiben unveraendert unzulaessig. Die bestehende `normalize_clef_readability_insertions()`-Pruefung wird nicht gelockert; neue Inserts innerhalb offener Ties/Primary-Beams bleiben fail-closed.
4. Kostenfunktion, Clef-Praeferenz, Ledger-Pressure, ShortExcursion-Gewichtung und Materialisierungsboundary bleiben unveraendert.

Es gibt keinen neuen Writer, keinen neuen Checkpoint, keine Repair-/Recovery-Authority und keinen neuen produktiven Caller. Der bestehende Structural-Write-Root materialisiert weiterhin ausschliesslich den vom selben ClefReadability-Owner geplanten Kurs.

## Invarianten

- Pitch unveraendert,
- Onset unveraendert,
- Duration unveraendert,
- Staff/Voice unveraendert,
- Beam-Tags unveraendert,
- Tie-Tags unveraendert,
- Octave-Shift-Tags und deren Semantik unveraendert,
- Clef-Kosten und Schwellen unveraendert,
- keine neue Writer-/Repair-/Recovery-Authority,
- keine Projekt-, Takt- oder Pitch-Identitaet in produktiver Logik.

## Regression

Ein synthetischer Test bildet die generalisierte Struktur ab:

`F4 -> octave-shift down -> beam begin/continue -> octave-shift stop -> vorhandener G2 -> beam end -> hoher Tail`

Er verlangt:

- der vorhandene G2-Surface liegt an einer echten Octave-Shift-Boundary,
- ein Treble-Candidate darf nicht an den vor der Boundary liegenden Beam-Begin zurueckankern,
- nach Materialisierung bleibt G2 hinter dem `octave-shift stop`,
- vor dem Stop entsteht kein neuer `ClefReadabilityPrimaryCourse`-G2-Surface.

## ChangeClass

`MusicMutation`.

Die Klasse ist erforderlich, weil der produktive Clef-DecisionScope und damit die finale Render-Notation geaendert wird. Es gibt keine SoundingEventMutation und keinen MasterApply.

## Erste Abnahme

```powershell
./run.cmd --take "Etüde Nr. 9"
```

Erwartung fuer Takt 17/18 unten: Der G2-Wechsel darf nicht vor das Ende der unteren 8vb-Spanne gezogen werden. Pitch/Onset/Duration/Staff/Voice/Beam/Tie/Octave-Shift muessen stabil bleiben. Danach visuelle Pruefung von Etuede 9; bei gruenem Einzellauf Vollkorpus als Breitenbeweis.
