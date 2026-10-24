# CrossStaffHandProjectionReadabilityCommittedStateRecovery r342

## Ausgangspunkt

Autoritative Basis ist r341 `CrossStaffHandProjectionReadabilitySharedBeamOwnerClosure`. Der reale r341-Vollkorpus ist 10/10 GREEN, Build/Clippy/Tests/FinalExit sind GREEN und die geschlossenen Beam-Sicherheitsinvarianten bleiben erhalten: keine Achtelnote erhaelt Beam-Level 2 und kein Restknoten traegt Beam-State.

r341 ist trotzdem kein fachlicher Closure fuer den Kontrollfall Etuede 5 / Takt 5. Das finale `Etüde Nr. 5.render.musicxml` und das PDF sind gegen r339 byte-/pixelidentisch geblieben. Die Zielgeste B5-C#5-D5-G5 auf Zaehleinheit 3 ist im finalen Output weiter in ein Staff-1-Praefix und zwei isolierte Sechzehntel fragmentiert.

Gleichzeitig war r341 nicht wirkungslos: corpusweit blieben 37 reine Beam-Tag-Aenderungen in sechs Etueden erhalten, ohne Pitch-/Onset-/Duration-/Staff-/Voice-Delta. Der gemeinsame BeamHierarchy-DecisionOwner und die strenge Cross-Staff-Kandidatensemantik sind daher nicht widerlegt; falsch war der Materialisierungszeitpunkt fuer den Zielbefund.

## Ursache

r341 projizierte Cross-Staff-BeamHierarchy bereits im `ExternalFeedbackClosureRenderWrite`. Danach laeuft jedoch noch die `FinalRenderNotationAuthorityStage`. Diese darf spaetere Render-Snapshots materialisieren und kann die fruehe Cross-Staff-Projektion deshalb ueberdecken bzw. aus einem spaeteren Staff-/Voice-Zustand neu aufbauen.

Die Ownership-Regel war richtig, der konsumierte Zustand war nicht der finale committed Staff-/Voice-Zustand.

## Recovery-Cut

r342 erzeugt keinen neuen Beam-DecisionOwner und keine neue Cross-Staff-Musikregel.

- `close_structural_beam_gestures` kehrt auf seinen bisherigen stafflokalen Upper-/Lower-RestBridge-Auftrag zurueck; Cross-Staff wird dort nicht mehr materialisiert.
- Die bestehende strenge Cross-Staff-Gruppierung bleibt unveraendert: gleiche Voice und Zaehleinheit, sichtbare klingende beamfaehige Ereignisse, lueckenlos, nicht ueberlappend, mindestens ein Staff-Wechsel, vollstaendig innerhalb einer Zaehleinheit.
- Ein eigener Materialisierungshelfer `close_committed_cross_staff_beam_gestures` konsumiert weiterhin ausschließlich `structural_metric_beam_hierarchy_plan(..., 2)`.
- Dieser Helfer wird erst unmittelbar nach `run_final_render_notation_authority_stage` auf den erneut vom Render-Checkpoint gelesenen committed Staff-/Voice-Snapshot angewandt.
- Der Write benutzt die bereits vorhandene autorisierte `ExternalFeedbackClosure`-Checkpoint-Grenze; es entsteht keine neue Writer-Authority.
- Parse-, Raw-Cursor-/Backup-Forward-, sichtbare Sounding-Signature- und NoteCount-Invarianten muessen vor dem Write GREEN sein, sonst fail-closed.

Damit bleibt die fachliche Architektur:

`MetricBeamHierarchy DecisionOwner -> mehrere Materialisierungsprojektionen`,

wobei CrossStaff nun die final committed Staff-/Voice-Projektion konsumiert statt einen frueheren Render-Zwischenstand.

## PDF-Quellenwahrheit

r341 hatte `ExternalFeedbackClosureRenderWrite` nicht als eigene Evidence-Zeile in der `RenderPdfSourceEvidenceLedger`. r342 nimmt diesen bereits existierenden Writer dort auf. Ein Cross-Staff-only-Write kann damit die Render-MusicXML als tatsaechliche PDF-Quelle waehlen, sofern die bestehenden harten PDF-Source-Gates GREEN bleiben.

Es wird kein Gate gelockert. Insbesondere bleibt die bestehende exakte Kandidatenbindung des `AuthorizedCursorNeutralWriter` unveraendert.

## Erwarteter committed-state Restbefund

Auf dem realen finalen r341-Korpus verbleiben nach der bereits ueberlebenden r341-Projektion noch 7 streng qualifizierte Cross-Staff-Gruppen mit 20 zu normalisierenden Notenprojektionen. Der Zielbefund Etuede 5 / Takt 5 ist eine davon; dort sind noch genau drei Notenprojektionen zu korrigieren, weil die erste Note bereits `begin/begin` traegt.

Diese Zahlen sind Abnahme-Evidence, keine fest codierten Zielwerte.

## Invarianten

Nicht geaendert werden:

- Pitch,
- Onset,
- Duration,
- Staff,
- Voice,
- RestExistence/RestVisibility,
- RestSpanConsolidation,
- Clef-/Register-Entscheidungen,
- BeamHierarchy-DecisionOwner,
- fruehes Staff/Voice/Beat-Scoping,
- Schwellen/Gewichte,
- Security-/Take-Gates,
- AuditOnly-Ownership-Evidence.

Erlaubtes Delta sind ausschließlich L1/L2-Beam-Tags auf bereits vorhandenen sichtbaren klingenden Noten in der streng qualifizierten Cross-Staff-Voice/Beat-Domain.

## Abnahme

Zuerst:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Bei GREEN danach:

```powershell
.\run.cmd --all
```

Ein GREEN-Lauf allein reicht fuer den fachlichen Closure nicht. Zusaetzlich muss der finale Output belegen:

- Etuede 5 / Takt 5 / Zaehleinheit 3 ist als zusammenhaengende Vier-16tel-Cross-Staff-Balkengeste materialisiert,
- `CrossStaffBeamGroups` erscheint in der finalen Reportprojektion,
- corpusweit weiterhin 0 Achtel+Beam2,
- corpusweit weiterhin 0 Restknoten mit Beam,
- Pitch/Onset/Duration/Staff/Voice bleiben invariant.

Erst nach diesem realen Beweis darf die r340-AuditOnly-Evidence `CrossStaffHandProjectionReadability open` in einem separaten kleinen Evidence-Take geschlossen werden.
