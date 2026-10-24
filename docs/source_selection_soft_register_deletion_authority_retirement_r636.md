# r636 SourceSelectionSoftRegisterDeletionAuthorityRetirement

## Zweck

r636 ist die erste semantische SourceSelection-Korrektur nach dem r634-Kausalitaetsbeweis und dem r635-SelectionPlan-Authority-Cutover.

Sie aendert genau eine belegte Policyklasse:

> Eine Abweichung von einer **weichen** Hand-Registergrenze ist fuer sich allein kein hinreichender Grund, ein CanonicalSourceEvent zu loeschen.

Daher besitzt der fruehe `HotspotRegisterOutlier`-Schritt ab r636 keine SourceEvent-Loeschautoritaet mehr. Die bereits vorhandenen spaeteren, source-erhaltenden Trajectory-/Residual-Register-Move-Paesse bleiben unveraendert und erhalten dadurch erstmals die Chance, solche Registerkonflikte nach der Hotspot-Phase zu bearbeiten.

## Empirische Grundlage

Der unveraenderte dunkelgruene r634-Vollkorpus umfasst 10/10 Etueden und 10.166 CanonicalSourceEvents:

- retained: 10.089
- removed: 77
- CanonicalSourceRetention: 99,243 %
- 8/10 Etueden: 100 % CanonicalSourceRetention
- Etuede 6: 54 Removals
- Etuede 8: 23 Removals

Von den 77 Removals stammen:

- 16 aus `hotspot_span_reduction`
- 22 aus `hotspot_jump_reduction`
- 39 aus `hotspot_register_outlier_reduction`
- 0 aus `trajectory_zigzag_reduction`
- 0 aus `residual_critical_span_reduction`
- 0 aus `residual_critical_jump_reduction`

Die 39 Register-Outlier-Removals sind kausal einheitlich:

- Etuede 6: 22
- Etuede 8: 17
- alle 39 CanonicalSourceEvents wurden initial Staff 1 zugeordnet,
- alle 39 wurden vor der Entfernung durch vorhandene Hand-/Jump-/Simultaneity-Logik nach Staff 2 verschoben,
- alle 39 wurden danach als zu hoch fuer die linke Hand relativ zum `left_high_soft_limit + 7` geloescht,
- kein beobachteter Register-Outlier-Removal ist ein urspruenglich linker hoher Ton oder ein rechter tiefer Register-Outlier.

Damit liegt fuer diese Korpusklasse die geschlossene Kausalkette vor:

    CanonicalSourceEvent
      -> initial Staff 1
      -> bestehendes Staff-Reassignment nach Staff 2
      -> soft-register Konflikt in Staff 2
      -> SourceEvent-Loeschung

Die Register-Loeschregel heilt damit im beobachteten Korpus ausschliesslich Konflikte, die eine vorherige Staff-Zuordnung selbst erzeugt hat.

## Policy

r636 fuehrt **keine** globale Retention-Gewichtung ein.

Insbesondere bleibt `CanonicalSourceRetention`:

- kein Candidate-Score,
- kein Guard,
- kein Ranking-Key,
- kein automatischer Restoration-Trigger.

Stattdessen wird nur die nicht hinreichend begruendete Delete-Authority pensioniert:

    soft register evidence != SourceEvent delete authority

Aktiv bleiben die bestehenden Hotspot-Removals fuer Span und Jump. Diese werden in r636 weder gelockert noch verschaerft.

## Pass-Reihenfolge

Vor r636:

    Staff-Reassignments
      -> Hotspot Span Remove
      -> Hotspot Jump Remove
      -> Hotspot Register-Outlier Remove
      -> Trajectory Move/Remove
      -> Residual Register Move/critical Remove

Ab r636:

    Staff-Reassignments
      -> Hotspot Span Remove
      -> Hotspot Jump Remove
      -> Hotspot Register-Outlier Observe (keine Removal-Schreibautoritaet)
      -> Trajectory Move/Remove
      -> Residual Register Move/critical Remove

Der entscheidende Unterschied ist, dass ein nur registerbezogener Konflikt weiterhin gegen dieselben weichen Register-Thresholds beobachtet wird, dort aber nicht mehr vernichtet wird. Die bereits vorhandenen source-erhaltenden Move-Stufen erhalten dadurch den unverkuerzten SourceEvent-Bestand.

## Literaturgrenze

Nakamura & Yoshii, *Statistical piano reduction controlling performance difficulty* (APSIPA Transactions, 2018, DOI 10.1017/ATSIP.2018.18), formulieren Piano Reduction als Optimierung musikalischer Fidelity unter Difficulty-Constraints und zeigen, dass sequenzielle Pitch-Abhaengigkeit und Fingering-/Handbewegung fuer die Qualitaet der Spielbarkeitsmodellierung relevant sind.

Diese Literatur wird hier **nicht** als konkrete Schwellen- oder Handzuweisungsregel uebernommen. Sie stuetzt nur den Policy-Grundsatz, dass eine einzelne lokale Registergrenze keine vollstaendige Spielbarkeitsentscheidung darstellt.

## Unveraendert

r636 aendert nicht:

- MIDI als ProductiveSourceAuthority,
- `source_ordinal` als Event-Identitaet,
- SelectionPlan als produktive Materialisierungsautoritaet,
- Hotspot-Erzeugung, Gewichtung oder Truncation,
- Span-/Jump-Thresholds,
- CandidateProfile-Werte,
- Candidate-Ranking oder Score-Gewichte,
- `CanonicalSourceRetention` als Score/Guard,
- Writer-, MusicXML-, Render- oder PDF-Logik,
- WAV-Authority.

Die historische Enum-/Ledger-Vokabel `HotspotRegisterOutlier` bleibt lesbar, damit alte r633-r635 Evidence weiterhin decodierbar ist. Die bestehende Helper-Funktion `reduce_hotspot_register_outliers` samt ihren unveraenderten Threshold-Ausdruecken bleibt weiterhin produktiv aufgerufen, schreibt ab r636 jedoch weder in die Removal-Menge noch in Removal-Reason-/Decision-Maps. Damit pensioniert r636 die Delete-/Writer-Authority der Stufe, ohne im selben Take ihre CalibrationThreshold-Capability zu entfernen oder einen toten Legacy-Writer per Lint-Ausnahme zu konservieren.

## Qualification

Der lokale fail-closed Lauf muss mindestens beweisen:

1. Take-Change-Control `MusicMutation` akzeptiert die Capability-Verengung ohne gleichzeitige CalibrationThreshold-Aenderung.
2. Build, Rustfmt, Strict-Clippy und alle Tests sind gruen.
3. r635 SelectionPlan-Authority bleibt gebunden.
4. `ReasonCount hotspot_register_outlier_reduction` ist fuer den neuen Lauf 0.
5. `CanonicalSourceRetention` wird weiterhin nicht in Score/Guard/Ranking aufgenommen.
6. Etuede 6 bleibt ohne Blocker und ohne neue unbegruendete Release-Regressions.
7. Anschliessend muss `--all` den 10/10-Korpus qualifizieren; erst dieser Lauf entscheidet, wie viele der 39 Events tatsaechlich dauerhaft erhalten bleiben und ob andere Removal-Familien neu aktiviert werden.

Es wird **nicht** vorab behauptet, dass CanonicalSourceRetention um exakt 39 Events steigt: Nach dem Wegfall der fruehen Register-Loeschung duerfen bestehende spaetere Solver-/Trajectory-/Residual-Entscheidungen einen anderen Kandidatenpfad erzeugen. Genau das wird im Vollkorpus gemessen.
