# r323 StructuralUpperBeamHierarchyDecisionConvergenceRecovery

## Ausgangspunkt und Recovery

r321 `UnifiedDomainOwnershipEvidenceGraphAudit` bleibt die autoritative 10/10-Vollkorpus-GreenAuthority. Der Kandidat r322 `ExternalFeedbackUpperBeamHierarchyDecisionConvergence` wurde zwar vom PRE-TAKE-DIFF-GATE akzeptiert und kurz installiert, scheiterte jedoch beim anschliessenden autoritativen Build-Gate an `PRODUCTIVE_FEEDBACK_ORIGIN_CHANNEL` und wurde automatisch auf r321 zurueckgerollt. r322 ist damit kein Green-Parent und wird nicht wiederverwendet.

Der Gate-Befund war architektonisch korrekt: Die neue level-aware Planfunktion trug `external_feedback` im produktiven Symbolnamen. Dadurch wurde ein Writer-Aufruf an eine Herkunftsidentitaet gekoppelt, obwohl Auswahl und Materialisierung ausschliesslich aus struktureller Beat-/Staff-/Voice-/Notenwert-Evidenz abgeleitet werden muessen.

## r323

r323 behaelt die fachliche Musiksemantik des verworfenen r322, korrigiert aber die Ownership-Grenze. Der neue notenwertsensitive Upper-Plan lebt im bestehenden `composer_metric_beam_planning_core` als herkunftsneutraler struktureller Planer. `close_structural_beam_gestures_in_segment` konsumiert nur diesen Plan; kein neuer produktiver Selektor oder Writer enthaelt eine Feedback-Ursprungsidentitaet.

Der Plan ist weiterhin eng begrenzt:

- Level 1 umfasst die vollstaendige bereits strukturell ausgewaehlte Staff/Voice/Beat-Gruppe.
- Level 2 umfasst nur zusammenhaengende Runs aus Ereignissen mit `partial_beat_level >= 2`.
- Ein Level-2-Run wird nur mit mindestens zwei Ereignissen materialisiert.
- Level 3 und Level 4 liegen ausserhalb dieser Authority und werden bei einem notwendigen L1/L2-Rewrite unveraendert aus dem beobachteten MusicXML durchgereicht.
- Detection und Materialisierung verwenden exakt denselben `BeamPlan`.

## Scope-Grenze

Unveraendert bleiben:

- `structural_lower_rest_bridge_groups` und sein bisheriger kompletter Level-1/2-Pfad,
- `FinalCompleteBeatPrimaryBeamContinuity`,
- BeatScoped-Gruppenselektion,
- Pitch, Onset, Duration, Staff, Voice und Noten-/Restbestand,
- Artikulation, Clef/Register/Octave, Layout, Tonalitaet und Dynamik,
- Security-/Take-Control und StructuralWriteAuthority.

Der Take ist `ChangeClass=MusicMutation`, aber die beabsichtigte produktive Mutation ist auf Beam-Tags im generischen Upper-Scope begrenzt. `sounding_event_mutation = false`.

## Regressionen

Die vorhandenen r322-Regressionsideen werden unter strukturellen Symbolen fortgefuehrt:

- `4 x 16th`: Level 1 und Level 2 bleiben vollstaendig,
- `16th + 16th + eighth`: die Achtel besitzt nur Level 1,
- `eighth + 16th + 16th`: die Achtel besitzt nur Level 1,
- 32stel/64stel: keine neue Level-3/4-Authority und vorhandene hoehere Beam-Zustaende bleiben erhalten,
- isolierte kurze Note in gemischter Gruppe: kein erfundener Level-2-Run,
- Detection eines bereits korrekt level-aware materialisierten gemischten Plans ist idempotent.

## Erwartete Korpus-Abnahme

Auf Basis des r321-Vollkorpus bleibt die harte Zielsignatur unveraendert:

- generischer Upper-Scope: Achtel + Beam2 `467 -> 0`,
- Gesamt: Achtel + Beam2 `479 -> 12`,
- Etuede 6: `68 -> 5`.

Die verbleibenden 12 Korpusfaelle bzw. 5 Faelle in Etuede 6 gehoeren zum bewusst unangetasteten Lower-RestBridge-Scope. `ExternalFeedbackClosureRenderWrite UpperBeamGroups` bleibt nach dem realen Lauf der Ownership-Indikator; ein Retirement des Upper-Owners ist nicht Bestandteil von r323.
