# r1066 PrimaryBeamVoicePartitionWriterOutcomeProjectionClosure

## Klasse

AuditOnly. Keine MusicMutation, keine Regeländerung, kein neuer Writer und keine Änderung an MusicXML/PDF-Erzeugung.

## Ausgangsevidenz

Der verworfene r1066-MusicMutation-Versuch r03 lief nach den vorangegangenen Runtime-Korrekturen durch alle zehn Projektpipelines: zehn Projektberichte wurden formal ausgewertet, es gab 0 Projektfehler und 0 fehlende Ausgabedateien. Der Lauf blieb ausschließlich am CorpusRunManifest-SelfCheck `manifest_writer_outcome_not_observed_for_all_projects` fail-closed.

Die Ursache ist eine fehlende typisierte Writer-Ursache im bestehenden Manifestvertrag. Der produktive Primary-Beam-Voice-Rebind ist ein Render-MusicXML-Schreibgrund, gehört aber weder zu `routes_written`, `cross_staff_rest_suppressions`, `beam_lost_begin_notes_written`, `source_higher_beam_notes_restored`, `rest_vertical_anchors_written` noch zur Clef-Ursache. Wenn ein späterer MusicMutation-Take deshalb `RenderXmlWrite ja` meldet, ohne diese neue Ursache separat auszuweisen, muss der vorhandene Manifestparser die Writer-Zeile korrekt als inkonsistent verwerfen.

## Closure

Schema 6 ergänzt ausschließlich die additive, typisierte Evidence `primary_beam_voice_elements_written` beziehungsweise pro Projekt `writer_primary_beam_voice_elements_written`.

Der bestehende `SilentVoiceVisibleMeasureCoverageWriterStage` darf in einem späteren produktiven Take optional das Feld `PrimaryBeamVoiceElementsWritten <n>` emittieren. Fehlt dieses Feld, wird es rückwärtskompatibel als 0 gelesen. Ist es vorhanden, muss es eine gültige nichtnegative Ganzzahl sein; ein malformed Wert bleibt fail-closed.

`PrimaryBeamVoiceElementsWritten > 0` zählt als eigener Render-MusicXML-Schreibgrund für die bestehende Konsistenzgleichung `RenderXmlWrite == irgendeine typisierte WriteCause`. Es verändert weder die Bedeutung der vorhandenen Writer-Zähler noch deren historische Werte.

## Selector-Governance

Der optionale fail-closed Parserzweig in `src/corpus_run_manifest.rs` wird vom Take-Change-Control syntaktisch als selector-sensitive eingestuft. Deshalb gehört zu diesem AuditOnly-Take der Implementierungsvertrag `docs/selector_derivation_contract_primary_beam_voice_partition_writer_outcome_projection.md`. Er bindet ausschließlich an die bereits im accepted r1065 eingefrorene Fehlerklassendefinition `docs/selector_derivation_contract_final_primary_beam_serialization_voice_partition.md`.

`SelectorChange: yes` ist hier eine Governance-Klassifikation des Parser-/Projektionszweigs und **keine** neue musikalische Auswahlregel: weder Kandidaten noch Targets werden erzeugt, gerankt, verbreitert oder verengt. Corpus-/Projektidentität und beobachtete Anzahlen besitzen keine Selektionsautorität.

## Nicht geändert

- keine Auswahl oder Erweiterung des Primary-Beam-Reparaturscopes
- keine Voice-, Channel-, Beam-, Pitch-, Onset-, Duration-, Staff-, Chord- oder Tie-Mutation
- keine Änderung an FinalNotation, Renderer oder PDF
- kein neuer Writer-Root und kein Proof-ReentryReader
- keine Lockerung von `writer_outcome_observed` oder der Manifest-SelfChecks
- keine Übernahme eines rejected r1066-Mutationskandidaten als Authority

## Folgeschritt

Erst nach kanonischer Acceptance dieses AuditOnly-Takes darf ein separater MusicMutation-Take das neue optionale Feld mit der real geschriebenen Anzahl bestehender `<voice>`-Scalars befüllen. Dieser Folgetake muss weiterhin gegen den vollständigen bestehenden Writer-/Manifestvertrag fail-closed laufen.
