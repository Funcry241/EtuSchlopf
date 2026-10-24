# r1077 – RejectionLearningEvidenceBoundaryClosure

## Ziel

Die bereits vorhandene Rejection-Provenienz bleibt ein Lern- und Diagnosekanal, ohne neue Entscheidungs- oder Produktiv-Authority zu erzeugen.

## Vorhandene Evidenz

Der Take-Lifecycle persistiert bereits kanonisch gebundene Rejection-Daten und projiziert sie advisory-only:

- Revision und Candidate-SHA,
- `rejection_stage`,
- `attempt_epoch_millis`,
- kanonischen Rejection-Grund,
- im Failure-Handover zusätzlich `time_to_reject_ms`, Transaction-/Child-Kontext und Parent-Acceptance.

r1077 fügt dafür **keine neue Runtime, kein neues Ledger und keine neue Auswahlheuristik** hinzu.

## Architekturgrenze

Rejection-Evidenz darf verwendet werden, um wiederkehrende Failure-Patterns zu erkennen und daraus eine allgemeine, kausal begründete Regel oder Regression abzuleiten.

Sie darf **nicht**:

- einen Rejected Candidate zum Parent machen,
- Kandidaten ranken oder auswählen,
- Selector-/Writer-/Repair-/Release-Authority erzeugen,
- spätere Kandidaten aufgrund historischer Rejection-Häufigkeit bevorzugen oder benachteiligen,
- fehlende aktuelle Repository-/Runtime-Evidenz ersetzen.

Damit bleibt die Rejection-Kette lernwirksam, aber strikt evidence-only.

## Scope

Rein advisory/metadata:

- `E22 – Historie abstrahieren, nicht konservieren` erhält die explizite Rejection-Learning-Grenze.
- Der Architecture-Essence-Bootstrap spiegelt diese Grenze als `take_identity_lifecycle.rejection_learning`.
- Keine Rust-, Musik-, Renderer-, Writer-, Selection-, Acceptance-, Rejection- oder Handover-Runtime wird verändert.
