# LegacyCandidateScoreRetentionAuthorityCalibrationRetirementRecovery r699

## Ausgangslage

r698 wurde im PRE-TAKE Semantic-Capability-Diff abgelehnt und niemals installiert.
Der Parent dieses Recovery-Takes bleibt deshalb der letzte akzeptierte Stand r697.

Der r698-Befund war eindeutig: Die produktive U-foermige
ArrangementVisibleRetention-Score-Policy wird vom bestehenden Gate als Music-domain
`CalibrationThreshold` inventarisiert. `MusicMutation` darf diese Capability nicht aendern.

## Exakter r699-Cutover

r699 ist `CalibrationOnly` und aendert ausschliesslich die zwei produktiven Gewichte der
Retention-Strafe:

- unter 68 Prozent: Gewicht `500.0` -> `0.0`,
- ueber 82 Prozent: Gewicht `350.0` -> `0.0`.

Damit ist der produktive Retention-Beitrag fuer jeden Retention-Wert exakt null.
`ArrangementVisibleRetention` besitzt nach diesem Cutover keine additive CandidateScore-
Autoritaet mehr. Hohe oder niedrige sichtbare Retention kann den zentralen Score nicht
mehr verschlechtern oder verbessern.

Die 68/82-Branch-Struktur, die Retention-Zuleitung, die 15 Aufrufstellen und das read-only
`retention_penalty`-Feld bleiben fuer **genau diesen einen Cutover-Take** stehen. Das ist
keine neue fachliche Policy, sondern die minimale Form, die das vorhandene fail-closed
Calibration-Gate als isolierte Threshold-/Weight-Aenderung pruefen kann.

## Warum nicht `retention_penalty = 0` in diesem Take?

Das Semantic-Capability-Gate normalisiert bei `CalibrationOnly` numerische Threshold-Werte,
aber nicht strukturelle Operator-/Branch-Aenderungen. Das direkte Entfernen der Formel oder
das Ersetzen des gesamten Ausdrucks durch `0` waere deshalb bereits ein zusaetzlicher
MusicLogic-Diff und gehoert nicht in denselben CalibrationOnly-Cutover.

## Unmittelbarer Folgetake

Nach erfolgreichem r699 folgt r700 als `RefactorOnly`. Dort wird die nun wirkungslose
Retention-Policy **vollstaendig entfernt**:

- 68/82-Branch-Struktur raus,
- Retention-Argument aus `candidate_score_breakdown()` und `score_candidate()` raus,
- alle 15 produktiven Aufrufstellen auf die retention-freie Signatur umstellen,
- `retention_penalty`-Kompatibilitaetsfeld und historische Counterfactual-Audits pensionieren.

Damit bleiben weder ein gewuenschter Retention-Korridor noch eine `retention_penalty = 0`-
Legacy-Leiche als dauerhafter Vertrag zurueck.

## Unveraenderte Grenzen

`ArrangementVisibleRetention` bleibt Mess-/Reportgroesse. `CanonicalSourceRetention` bleibt
getrennte Provenienzmetrik. `HumanPlayability` erhaelt keine Authority. Writer-, MusicXML-,
Renderer-, PDF-, FinalChord-, Consensus-, Freeze- und VisibleRhythm-Logik bleiben
unveraendert. Ebenso bleiben alle anderen additiven Score-Komponenten und externen
Score-Aufschlaege unangetastet.
