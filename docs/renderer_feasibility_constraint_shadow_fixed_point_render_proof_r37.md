# RendererFeasibilityConstraintShadowFixedPointRenderProof – r37

## Ausgangsbefund

Der r36-Vollkorpus beweist den Constraint-Ansatz und zugleich die letzte Lücke
des einmaligen Shadow-Proofs:

- Etüde 6: `RENDERER_CONFIRMED`
- Etüden 2, 3, 7 und 8: `RENDERER_REJECTED`
- alle Rejections bestehen ausschließlich aus zusätzlichen Renderer-Systembreaks
- keine kollabierten Systembreaks
- keine Page-Drifts
- produktive Artefakte bleiben unverändert

Die neuen Breaks sind keine Widerlegung des Constraint-Ansatzes. Sie sind neue
Renderer-Evidenz über genau den gerade getesteten Candidate und können deshalb
als zusätzliche exakte No-Good-Systembereiche in denselben bestehenden
Constraint-Solver zurückgeführt werden.

## Aenderung

r37 macht den bereits vorhandenen Shadow-Render-Proof zu einem begrenzten
Fixed-Point-Proof im selben Lauf:

1. initiale produktiv beobachtete Renderer-No-Goods binden,
2. Constraint-Candidate mit unveraendertem Planner-Demand/Capacity-Vertrag
   berechnen,
3. Candidate ausschließlich unter `target/` über die bestehende zentrale
   Shadow-Write-Authority materialisieren,
4. denselben bereits autorisierten MuseScore-Renderer ausführen,
5. PDF-Systemgrenzen zurücklesen,
6. bei rein zusätzlichen Systembreaks die betroffenen Candidate-Systembereiche
   als neue exakte No-Goods aufnehmen,
7. erneut planen und rendern,
8. bei 0 Drift `RENDERER_CONFIRMED`, ansonsten höchstens acht Shadow-Runden.

Es wird ausdrücklich keine Breitenformel, kein Threshold und keine Capacity
gelernt. Das Renderer-Ergebnis autorisiert auch weiterhin keinen produktiven
Write; es liefert nur Unzulässigkeits-Constraints innerhalb des Shadow-Proofs.

## Fail-closed / fail-soft

Der iterative Lernschritt ist nur erlaubt, wenn der Shadow-Renderer ausschließlich
zusätzliche Systembreaks erzeugt. Bei kollabierten Systembreaks oder irgendeiner
Page-Drift stoppt der Proof mit `RENDERER_REJECTED` und
`unsupported_shadow_drift_shape`.

Jede Rejection muss mindestens ein neues, eindeutig an ein Candidate-System
gebundenes No-Good erzeugen. Ohne Fortschritt wird `UNAVAILABLE` gemeldet.
Nach acht erfolglosen Runden endet der Proof mit `RENDERER_REJECTED` und
`shadow_iteration_limit`.

Master-MusicXML, produktives Render-MusicXML und produktives PDF werden nach
jeder Shadow-Runde bytegenau gegen ihren Vorzustand geprüft.

## Neue Evidenz

Der Report enthält zusätzlich:

- `ShadowProofRounds`
- `InitialNoGoods`
- `AccumulatedNoGoods`
- `RejectedRounds`

Damit ist sichtbar, ob MuseScore den ersten Candidate hielt oder welche neuen
No-Goods bis zum bestätigten Fixed Point nötig waren.

## Nicht geaendert

- keine Musikmutation
- keine produktive Layoutauswahl
- keine neue Selection-Authority
- keine produktive Writer-Authority
- keine Demand-Neuberechnung
- keine Capacity-/Threshold-Aenderung
- keine neue Renderer-Heuristik
- keine Productive-Write-Bypass-Baseline-Aenderung
- keine neue externe Write-Surface
- keine Lockerung bestehender Gates

## Regression

Zwei neue reine Unit-Tests beweisen:

1. mehrere zusätzliche Renderer-Breaks werden exakt an ihre materialisierten
   Candidate-Systembereiche gebunden;
2. eine vermeintliche Renderer-Rejection ohne inserted system break erzeugt
   kein No-Good und scheitert fail-closed.

Erwarteter Testbestand: **763/763**.

## Ziel

Der r37-Einzellauf und danach der Vollkorpus sollen zeigen, dass die betroffenen
Stücke innerhalb eines begrenzten Shadow-Fixed-Point-Proofs auf einen von
MuseScore tatsächlich gehaltenen Joint-System-Plan konvergieren können, ohne
produktive Authority zu verändern.
