# r1282: SSC v4 Take 3 – Edit- und Block-Eingrenzung

`GovernedChange` auf accepted r1281 `SscContainmentVerdictsV4`. `music_effect=none`; `PermitEnforcement: none`. Keine Owner-, Mint-, Capture-, Writer- oder Adoptionsfunktion wurde geändert. Kodex-Rustdateien, Trust-Anchor, Launcher und Bootstrap sind unverändert.

Die drei bestehenden Lane-Owner bleiben Lane. Alle übrigen Exact- und alle Block-Descriptoren erhalten eine neue Zeile. Genesis-Owner erhalten keine Zeile. Tabelle und Beweisänderung werden durch den kompilierten r1281-Parent zugelassen.

## Herleitung vor der Korpusmessung

Die folgende Tabelle ist aus den versiegelten Descriptoren und ihren Owner-Implementierungen abgeleitet. Die ausführliche Element-, Feld- und Operationsdeklaration samt Zweck steht in `src/build_gate/ssc_containment_envelopes_v1.tsv`. Keine Zeile wird aus einer Korpusverteilung abgeleitet oder nach einem Denied geweitet.

Neue Zeilen: 54 (38 Edit, 16 Block); insgesamt 76.

| Descriptor | Art | Content | Herleitung am unveränderten Owner |
|---|---|---|---|
| `SAME_LANE_RAW_SPLICE_EXACT` | Edit | yes | `src/render/silent_voice_visible_measure_coverage_render_writer.rs::materialize_silent_voice_visible_measure_coverage_for_pdf_source` (Zeile 1617) |
| `MISSING_REST_RENDER_EXACT` | Edit | yes | `src/structural_write_authority/render_measure_articulation_authority.rs::apply_bound_missing_rest_batch_authorized` (Zeile 493) |
| `DIRECTION_RELOCATION_EXACT` | Edit | no | `src/structural_write_authority/renderer_direction_relocation_writer_boundary.rs::compose_authorized_direction_delta_onto_sink` (Zeile 113) |
| `VISIBLE_RHYTHM_BLOCK` | Block | yes | `src/arrangement.rs::normalize_visible_rhythm_musicxml` (Zeile 375) |
| `BEAT_SCOPED_BEAMING_BLOCK` | Block | yes | `src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::enforce_beat_scoped_beaming_and_rest_completion_in_xml` (Zeile 1492) |
| `CLEF_MEASURE_BLOCK` | Block | no | `src/structural_write_authority/render_measure_articulation_authority.rs::replace_bound_clef_measure_authorized` (Zeile 417) |
| `OCTAVE_MEASURE_BLOCK` | Block | yes | `src/structural_write_authority/render_measure_articulation_authority.rs::replace_bound_octave_measure_authorized` (Zeile 454) |
| `CURSOR_LANE_END_CLOSURE_BLOCK` | Block | yes | `src/pipeline_shards/report_hygiene_cursor_localbest_trace_tail.rs::repair_cursor_lane_end_invariant_xml` (Zeile 293) |
| `SAFE_TRANSFER_EXACT` | Edit | yes | `src/structural_write_authority/remaining_atomic_patch_authority.rs::apply_bound_safe_transfer_authorized` (Zeile 231) |
| `RESTIFY_PATCH_EXACT` | Edit | yes | `src/pipeline_shards/staff_voice_atomic_move_patch_core.rs::apply_restify_patch` (Zeile 892) |
| `VOICE_OVERFULL_CROP_EXACT` | Edit | yes | `src/structural_write_authority/bounded_semantic_rewrite_authority.rs::apply_voice_overfull_crop_edits_authorized` (Zeile 255) |
| `RESTIFICATION_BATCH_EXACT` | Edit | yes | `src/structural_write_authority/staff_voice_atomic_authority.rs::apply_atomic_restification_batch_authorized` (Zeile 139) |
| `STAFF_VOICE_SCALAR_BATCH_EXACT` | Edit | no | `src/structural_write_authority/staff_voice_atomic_authority.rs::apply_atomic_staff_voice_scalar_batch_authorized` (Zeile 179) |
| `EDITORIAL_VOICE_COMPLETE_REWRITE_EXACT` | Edit | yes | `src/structural_write_authority/bounded_semantic_rewrite_authority.rs::apply_editorial_voice_complete_rewrite_authorized` (Zeile 285) |
| `NOTE_VISIBILITY_EXACT` | Edit | no | `src/structural_write_authority/remaining_layout_writer_family.rs::hide_note_print_object_authorized` (Zeile 231) |
| `ALLOWED_RANGE_REMOVAL_EXACT` | Edit | yes | `src/structural_write_authority/remaining_layout_writer_family.rs::remove_allowed_xml_ranges_authorized` (Zeile 372) |
| `HARMONIC_PITCH_SWAP_EXACT` | Edit | yes | `src/structural_write_authority/thirty_surface_productive_authority.rs::apply_harmonic_pitch_swap_authorized` (Zeile 915) |
| `HIDDEN_ATTRIBUTE_REMOVAL_EXACT` | Edit | no | `src/structural_write_authority/thirty_surface_productive_authority.rs::remove_note_hidden_attrs_in_range_authorized` (Zeile 783) |
| `CHORD_GESTURE_DISTRIBUTION_EXACT` | Edit | yes | `src/structural_write_authority/final_buffer_surface_authority.rs::apply_chord_gesture_distribution_batch_authorized` (Zeile 385) |
| `VISIBILITY_STAFF_DISTRIBUTION_EXACT` | Edit | no | `src/structural_write_authority/final_buffer_surface_authority.rs::apply_visibility_staff_distribution_batch_authorized` (Zeile 353) |
| `MASTER_EXPORT_FIFTHS_EXACT` | Edit | no | `src/structural_write_authority/orchestrator_artifact_sink_authority.rs::write_master_musicxml_output_authorized` (Zeile 517) |
| `FUNCTIONAL_RESPELLING_EXACT` | Edit | yes | `src/pipeline_shards/key_signature_enharmonic_report_core.rs::canonical_master_functional_respelling_from_render` (Zeile 927) |
| `FINAL_CHORD_ENCODING_EXACT` | Edit | yes | `src/structural_write_authority/serialization_plan.rs::project_final_chord_encoding_plan` (Zeile 8533) |
| `RENDER_ROW_7_BLOCK` | Block | yes | `src/structural_write_authority/retention_chord_authority.rs::ensure_bound_chord_marker_authorized` (Zeile 39) |
| `RENDER_ROW_17_BLOCK` | Block | yes | `src/structural_write_authority/thirty_surface_productive_authority.rs::set_rest_note_duration_type_authorized` (Zeile 678) |
| `RENDER_ROW_19_BLOCK` | Block | no | `src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::beat_scoped_mark_note_hidden` (Zeile 2384) |
| `RENDER_ROW_20_BLOCK` | Block | no | `src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs::hide_note_fragment_print_object_no` (Zeile 1560) |
| `RENDER_ROW_21_BLOCK` | Block | yes | `src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs::collapse_hidden_timeline_rests_to_forward_in_measure` (Zeile 427) |
| `RENDER_ROW_22_EXACT` | Edit | yes | `src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs::prune_safe_hidden_inner_chord_tones_in_measure` (Zeile 283) |
| `RENDER_ROW_24_BLOCK` | Block | yes | `src/pipeline_shards/report_hygiene_cursor_localbest_trace_tail.rs::repair_cursor_lane_end_invariant_measure` (Zeile 203) |
| `RENDER_ROW_30_EXACT` | Edit | yes | `src/structural_write_authority/rest_voice_family_authority.rs::apply_final_rest_glyph_height_authorized` (Zeile 619) |
| `RENDER_ROW_31_EXACT` | Edit | yes | `src/structural_write_authority/rest_voice_family_authority.rs::apply_final_visible_rest_role_rewrite_authorized` (Zeile 601) |
| `RENDER_ROW_32_EXACT` | Edit | yes | `src/structural_write_authority/rest_voice_family_authority.rs::apply_layer_balanced_rest_consolidation_authorized` (Zeile 504) |
| `RENDER_ROW_33_EXACT` | Edit | yes | `src/structural_write_authority/rest_voice_family_authority.rs::apply_lower_staff_rest_chain_compaction_authorized` (Zeile 486) |
| `RENDER_ROW_34_EXACT` | Edit | yes | `src/structural_write_authority/rest_voice_family_authority.rs::apply_pianistic_rest_consolidation_authorized` (Zeile 529) |
| `RENDER_ROW_35_EXACT` | Edit | yes | `src/structural_write_authority/rest_voice_family_authority.rs::apply_rest_only_staff_readability_authorized` (Zeile 547) |
| `RENDER_ROW_36_EXACT` | Edit | yes | `src/structural_write_authority/rest_voice_family_authority.rs::apply_secondary_reanchored_rest_visibility_authorized` (Zeile 583) |
| `RENDER_ROW_37_EXACT` | Edit | yes | `src/structural_write_authority/rest_voice_family_authority.rs::apply_single_rest_consolidation_authorized` (Zeile 565) |
| `RENDER_ROW_38_EXACT` | Edit | yes | `src/structural_write_authority/rest_voice_family_authority.rs::apply_strict_staff_gap_materialization_authorized` (Zeile 637) |
| `RENDER_ROW_41_EXACT` | Edit | yes | `src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_backup_anchor_rewrite_authorized` (Zeile 331) |
| `RENDER_ROW_42_EXACT` | Edit | yes | `src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_duplicate_same_pitch_suppression_authorized` (Zeile 361) |
| `RENDER_ROW_43_EXACT` | Edit | yes | `src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_empty_voice_rest_suppression_authorized` (Zeile 391) |
| `RENDER_ROW_45_EXACT` | Edit | yes | `src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_lower_visible_rest_suppression_authorized` (Zeile 401) |
| `RENDER_ROW_46_EXACT` | Edit | yes | `src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_rest_beat_materialization_authorized` (Zeile 376) |
| `RENDER_ROW_47_EXACT` | Edit | yes | `src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_upper_duplicate_rest_suppression_authorized` (Zeile 416) |
| `RENDER_ROW_48_EXACT` | Edit | yes | `src/structural_write_authority/strict_staff_visible_lane_authority.rs::apply_upper_initial_rest_coalescing_authorized` (Zeile 426) |
| `RENDER_ROW_51_EXACT` | Edit | yes | `src/structural_write_authority/strict_staff_visible_lane_authority.rs::insert_missing_staff_edge_rest_authorized` (Zeile 439) |
| `RENDER_ROW_54_BLOCK` | Block | yes | `src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs::close_beat_scoped_rest_completion_crossing_rests_in_xml` (Zeile 2212) |
| `RENDER_ROW_55_BLOCK` | Block | yes | `src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs::collapse_hidden_timeline_rests_to_forward` (Zeile 406) |
| `RENDER_ROW_57_BLOCK` | Block | no | `src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs::build_render_xml_print_cleanup_one_safe_visual_trial` (Zeile 1) |
| `RENDER_ROW_58_EXACT` | Edit | no | `src/structural_write_authority/orchestrator_artifact_sink_authority.rs::write_render_checkpoint` (Zeile 405) |
| `RENDER_ROW_61_BLOCK` | Block | no | `src/structural_write_authority/clef_surface_materialization.rs::write_clef_surface_materialization_authorized` (Zeile 168) |
| `RENDER_ROW_64_EXACT` | Edit | yes | `src/pipeline_shards/retention_composer_limit_core.rs::apply_retention_first_p2_assertive_return_write_explicit_gate` (Zeile 173) |
| `RENDER_ROW_66_BLOCK` | Block | yes | `src/pipeline_shards/key_signature_enharmonic_report_core.rs::apply_key_signature_enharmonic_spelling_render_write` (Zeile 142) |

#52 normalisiert sichtbare Rhythmus-Noten und Cursor, daher `note,forward,backup` mit Content-Deklaration. #57 hat im eingefrorenen Code eine leere Rest-Aktionsliste und schreibt nur den Clef-Kandidaten: seine Zeile erlaubt ausschließlich die Clef-Struktur und ist `content_changing=no`. #58 mints bei der Clef-Plan-Materialisierung im FinalNotation-Checkpoint: ausschließlich Clef-Insertion/Removal, ebenfalls kein Content.

#6 (`ALLOWED_RANGE_REMOVAL_EXACT`) akzeptiert im Owner auch sichere Clef-Attributes-Löschungen. Die eingefrorene `DESCRIPTOR_PURPOSES`-Zeile enthält jedoch nur notation_mark/rest_rhythm, keine clef-Klasse. Deshalb deklariert r1282 nur `notations,slur,tie,tied`; die Clef-Variante bleibt konservativ Denied. Das Kind erweitert keine Parent-Zweckklasse. Dieser Governance-Befund ist unabhängig vom Korpus und gehört in die Prüfung nach A5.

Feldgebundene Änderungen werden als Änderung ihres Host-Elements deklariert. Die r1281-Semantik blendet ausschließlich die benannten Attribute/Kinder aus; reine Visibility-Zeilen erlauben keine Note-Insertion/Removal. Content wird konservativ auch für `rest` und `tie` erklärt, wie es der Parent verlangt.

## Konstruktiver Beweis und Bindung

- Neue freie Accessoren lesen die versiegelten Descriptoren, das unveränderte Edit-Skript und den gebundenen Block-Span. Sie minten und schreiben nichts. Das gepinnte Block-Impl und #57 bleiben bytegleich.
- Bei Unproved-Capture leitet der Beobachter Edit/Block aus Receipt-Art und Descriptor ab. Das Capture selbst bleibt unverändert. Ein fremder Receipt-Typ oder eine fremde Envelope-Zeile ergibt `ProofFamilyMismatch`.
- Edit: Replay des vollständigen Original-Skripts gegen Pre/Post; alle unveränderten Segmente sind bytegleich. Zusätzlich müssen die tatsächlichen geänderten Byte-Unterbereiche jeder Edit in den deklarierten Elementen/Feldern von Pre und Post liegen. So können Kommentare und Whitespace außerhalb der Envelope nicht im kanonischen Vergleich verschwinden.
- Block mit Span: der unveränderte Pre-Präfix und Pre-Suffix müssen außerhalb des verschobenen Post-Spans bytegleich sein. Ohne Span gilt die Dokument-Containment-Prüfung. Der strukturelle Containment-Checker aus r1281 bleibt bestehen.
- Die bisherigen V2-Coverage- und Necessity-Zeilen bleiben unverändert. Edit/Block schließen die V4-Coverage; ihre konstruktiven Beweise werden dennoch vollständig an Event und Familie gebunden. Notwendigkeit bleibt Messung. Neue Edit/Block-Invarianten geben ohne Necessity-Zertifikat ein V4-Permit. Es entsteht kein Enforcement.
- Berichte ergänzen `V4ContentChangingPermitted` und eine `V4DeniedWitness`-Projektion je Denied mit Descriptor, Basis und gebundenen Pre/Post-Ranges und Byte-Längen. Vollständige konstruktive Witnesses bleiben eventgebunden intern.

## Erwartete Windows-Messung

Auf denselben musikalischen Owner-Ausführungen: 1438 released Events, 0 Integritätsfehler, `ReleasedBalance: complete`, `V4Balance: complete`, `V4OutOfScope: 0`. Die bisherigen 248 V4-Permits und 6 E007-Denials bleiben unverändert; die bisher 1184 OutOfScope-Events werden Permit, Denied oder Unproven. Exakte neue Permit-/Denied-/Unproven-Zahlen müssen im frischen Windows-Lauf gemessen werden. Sie werden nicht aus dem Ziel rückwärts festgelegt.

Alle 20 XML, alle gerenderten PDF-Seiten und released Lineage müssen zum r1281-Export neutral sein. Ein neues Denied wird unverändert berichtet und nicht im selben Take durch eine weitere Envelope-Öffnung beseitigt.

## Tests

Positiv, Escape und Invariantenverletzung für beide Arten; Skript-Replay, Byteänderung außerhalb der Edits, Byteänderung außerhalb des Spans, Kommentar/Whitespace außerhalb einer Edit-Envelope und Fremd-Bindung. Durchstich über echte Owner, Receipt, Master-Adoption und Abschlussbericht für StaffVoiceScalarBatch, HarmonicPitchSwap (Content), ClefMeasure (Span) und VisibleRhythm (dokumentweit). NoteVisibility prüft zusätzlich einen echten Denied samt Witness. Vorhandene V2-/V4-Tests bleiben erhalten.

Der VisibleRhythm-Durchstich liegt in `tests/support/ssc_edit_block_document_owner.rs`, eingebunden durch die bestehenden SSC-Bundle-Tests. Damit ist der Testaufruf eines externen Dokument-Owners keine produktive Delegation aus StructuralWriteAuthority. Der Standalone-Raw-Splice-Test verwendet weiterhin den echten Identity-/Proof-Code und dieselbe Envelope-Tabelle; es entsteht keine zusätzliche Laufzeitabhängigkeit.

## Erster lokaler Denied bleibt sichtbar

Der erste lokale Durchstich von `NOTE_VISIBILITY_EXACT` ergibt `V4ContainmentDenied: 1`: `rewrite_note_opening_tag_hidden` in `remaining_layout_writer_family.rs` (Zeilen 301–330) schreibt neben `print-object` auch `print-spacing`. Meine zuerst deklarierte Zeile enthält nur `note@print-object` und ist damit zu eng. Gemäß Auftrag wird sie nach dem gemessenen Denied nicht in diesem Take geweitet. Der Denied samt Witness bleibt als Befund für die A5-Prüfung erhalten. Die zwei positiven Exact-Durchstiche verwenden den reinen Voice/Staff-Scalar-Owner und den PitchSwap-Owner. Es wurde kein Owner geändert und keine Musikentscheidung aus dem Test abgeleitet.
