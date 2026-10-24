# CanonicalRenderTimelineObservationBoundary

## Zweck

v0.10.1696 ist Take 1 des auf drei Takes begrenzten `LowerDualEdgeTimelineOwnershipArchitecture`-Superwurfs.

`LocalVoiceRewriteBatch` und die Distinct-Lane/Lane-Fold-Planung lesen bisher denselben MusicXML-Notenkern separat: Node-Bindung und Fingerprint, Dauer, Staff, Voice, Typ, Stem, Rest/Chord/Hidden, Pitch und Beam-Zustaende. Take 1 fuehrt dafuer genau eine typisierte `RenderTimelineNoteObservation` ein. Beide bestehenden Parser behalten ihre eigene Cursor-, Beat-, Scope- und Lane-Epoch-Semantik, beziehen den gemeinsamen Notenkern aber aus derselben Observation.

Der Take fuehrt keinen neuen Writer, keinen neuen Scheduler und keine neue musikalische Regel ein. Der bestehende autorisierte LowerDualEdge-Writer-Root bleibt unveraendert. Candidate-Erkennung, Cursorfortschritt, Lane-Epoch, Materialisierung, Readback und Reports bleiben semantisch unveraendert.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```

Erwartet: 10/10 artefaktidentische Projekte gegen v0.10.1695.
