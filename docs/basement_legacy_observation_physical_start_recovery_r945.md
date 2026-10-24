# r945 BasementLegacyObservationPhysicalStartRecovery

## Authority und PhysicalStart

Autoritativer Parent bleibt der akzeptierte r942 `GlobalSystemReflowReleaseShadowRefactorClosure`.
r943 und r944 sind keine Authority-Parents.

Der reale r944-Take-Change-Control-Befund praezisiert jedoch den PhysicalStart:

- `src/midi_seed.rs` war gegen den eingehenden r944-Kandidaten bereits unveraendert; der physische Arbeitsbaum besitzt damit bereits die in r943 vorgesehenen MIDI-Wrapper-Retirements.
- `docs/midi_seed_optional_api_retirement_r943.md` wurde vom Gate als reale, aber in r944 nicht deklarierte Differenz erkannt; r945 bewahrt diese physische Datei statt sie durch den Kellerbatch unbeabsichtigt zu entfernen.
- Die r942-Dokumentation und `pipeline_orchestrator_release_support.rs` waren bereits im PhysicalStart vorhanden und werden nicht erneut als Aenderung deklariert.

Diese Trennung folgt dem Repository-Vertrag: AuthorityParent, Snapshot und PhysicalStart sind nicht automatisch identisch; der Change-Control-Diff wird gegen den aktuellen PhysicalStart deklariert.

## ChangeClass

`RefactorOnly`.

## Ziel

Den noch offenen Teil des Kellerbatches ausfuehren, ohne den bereits physisch vorhandenen MIDI-Rueckbau erneut zu deklarieren oder zurueckzudrehen.

### Bereits physisch vorhanden und nur bewahrt

- `midi_to_musicxml_seed` entfernt
- `midi_to_musicxml_seed_with_profile` entfernt
- `_midi_seed_optional_api_keepalive` entfernt
- produktiver `midi_to_musicxml_seed_with_profile_and_hotspots` bleibt

### In r945 noch real zu aendern

- retired `observe_legacy_page_slot_calibration`-Korridor entfernen,
- optionalen Runtime-Beobachtungspfad entfernen,
- `legacy_page_slot_calibration_candidate_available` aus dem Audit entfernen,
- `LegacyPageSlotCalibrationCandidate` aus Console-/Readback-Projektion entfernen,
- bestehende Readback-Fixture nur an die kleinere lebende Planner-Projektion anpassen.

## Nicht veraendert

- produktiver Global Page/System Planner
- Demand/Capacity Evidence
- PageCountPreference
- RendererFeasibility
- MusicXML-/PDF-Writer
- ReleaseDecision
- Gates oder Recovery

## r944 Rejection als Evidence

r944 wurde vor Semantic-Capability-Diff durch Take-Change-Control abgelehnt, weil sein Manifest noch die r942-Deklarationen enthielt. Die Source-Idee wurde dadurch nicht akzeptiert oder verworfen; r945 korrigiert ausschliesslich die Change-Control-Bindung gegen den durch das Gate nachgewiesenen PhysicalStart.
