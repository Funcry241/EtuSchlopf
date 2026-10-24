# StructuralRestBridgeBeamPlanAdoption – v0.10.1601

## Ausgangspunkt

v0.10.1600 ist nach realem Zielsystemlauf dunkelgruen und autoritativ:
Strict Clippy 0, Build 0, FinalExit 0, Renderer bestanden, Master- und
Render-MusicXML gegen v0.10.1599 nur mit geaenderter Softwareversionszeile,
alle sechs PDF-Seiten pixelidentisch.

Nach Gould, scoreweiter metrischer Kontinuitaet, Partial-Beat/Post-Rest und
dem generischen Notation-Polish erzeugten zwei strukturelle Rest-/Closure-
Pfade in `composer_recovery_octave_beam_export_tail.rs` ihre Balkenzustaende
noch lokal als rohe `begin/continue/end`-Strings.

## Aenderung

- `structural_rest_bridge_beam_states` beschreibt Primaer- und
  Sekundaergruppen nun als explizite Eventordinal-Gruppen und laesst
  `BeamPlan::from_level_groups` die typisierten Zustandsedits erzeugen.
- Die bisherige gemischte Hierarchie bleibt erhalten: Der Primaerbalken
  umfasst alle Zielzeilen; der Sekundaerbalken umfasst nur die kurzen
  Zielzeilen oder bei reiner Rest-Bridge weiterhin genau erste und letzte
  Zielzeile.
- Die Structural-Closure-Normalisierung und ihr konfliktpruefender Merge
  verwenden denselben `BeamPlan::from_groups`-Plan. Die lokale zweite
  `begin/continue/end`-Berechnung in `structural_beam_group_needs_normalization`
  und `add_structural_beam_group` entfaellt.
- Bestehende identische Zustandszeilen werden weiterhin nicht dupliziert;
  widerspruechliche Zustandszeilen derselben Note und Ebene blockieren den
  Gruppeneintrag weiterhin fail-closed.

## Bewusst unveraendert

- Auswahl der strukturellen Rest-Bridge- und Closure-Gruppen
- Staff-, Voice-, Beat-, Dauer- und Sichtbarkeitsbedingungen
- XML-Notenbindung und bestehender Beam-Tag-Adapter
- Writer, Checkpoint-Sinks und Pipeline-Reihenfolge
- Pitch, Onset, Dauer, Staff, Voice, Pausen und Notenanzahl
- Artikulationsanteil des kombinierten Closure-Writers

## Statische Aequivalenz

Die alte lokale Rest-Bridge-Zustandsbildung und die neue `BeamPlan`-Abbildung
wurden ueber 200.000 zufaellige Zielgruppen mit variierenden Dauern,
Sortierungen und Hierarchieflags verglichen. Zusaetzlich wurden 200.000
zufaellige Structural-Closure-Mergefaelle mit vorhandenen identischen und
widerspruechlichen Balkenzustaenden verglichen. Zustandsmenge, Reihenfolge,
Annahme und Ablehnung waren identisch.
