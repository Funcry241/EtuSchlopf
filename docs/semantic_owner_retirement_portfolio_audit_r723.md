# r723 SemanticOwnerRetirementPortfolioAudit

## Zweck

Dieser Audit bereitet das vereinbarte Nettoziel von etwa 60.000 weniger physischen Rust-Zeilen vor, ohne in diesem Take produktive Logik, Writer, Tests, Gates oder Musik zu entfernen. Er ist eine Retirement-Landkarte, keine Retirement-Authority.

Ausgangsbasis ist der fuer r722 vorbereitete Baum. r723 darf nur auf einen tatsaechlich angenommenen, dunkelgruenen r722 folgen. Falls r722 nicht angenommen wird, ist dieser Kandidat nicht anzuwenden.

## Harte Zieldefinition

- physische Rust-Baseline `src/ + tests/ + tools/`: **203.261 LOC**
- Nettoziel: **-60.000 LOC**
- rechnerische Zielgroesse: **143.261 LOC**
- Zeilen zaehlen nur als echter Erfolg, wenn semantische Verantwortung, historischer Parallelpfad oder obsolete Beweisduplikation tatsaechlich verschwindet. Reformatierung, Minifizierung, Testtabellen-Kompression oder reine Verschiebung zwischen Dateien zaehlen nicht als Retirement-Erfolg.

## Retirement-Invariante

Ein produktiver Abbau darf erst erfolgen, wenn fuer den betroffenen Bereich alle folgenden Fragen geschlossen sind:

1. Welcher alte Semantic Owner oder Writer wird pensioniert?
2. Welche bereits bewiesene Authority ersetzt ihn?
3. Welche Negative Checks / Regression Detector gehoeren heute noch exklusiv zum alten Pfad?
4. Wohin werden diese Gegenbeweise migriert oder warum sind sie nachweislich ueberfluessig?
5. Gibt es noch Shadow-/Parity-/Fallback-Verbraucher?
6. Ist das produktive Write-Recht des alten Owners bereits null?
7. Bleibt Fail-Closed-Verhalten erhalten?
8. Ist der Vollkorpus nach dem Cut unveraendert gruen?

## Schutzbereiche

Folgende Bereiche sind fuer den 60k-Abbau **nicht blind verfuegbar** und benoetigen einen expliziten Capability-/Owner-Beweis vor jeder Loeschung:

- `src/structural_write_authority`: **25.020 LOC** — neuer zentraler Writer-Kern; nur Uebergangsadapter/Parity nach bewiesenem Cutover.
- `src/build_gate`: **13.548 LOC** — Trust-/Change-Control-/Semantic-Gates; nur echte historische Transition-Capabilities.
- `src/audit_only/pianistic_*`: **10.672 LOC** — noch benoetigte Execution-/Witness-/Capability-Kette auf dem Finalpfad.
Diese Schutzklassifikation bedeutet nicht "nie anfassen", sondern "kein LOC-Ziel darf ihre Entfernung begruenden".

## Groesste Abbaudomaenen

| Domaene | aktuelle LOC | Arbeitskorridor | Bedingung |
|---|---:|---:|---|
| `src/pipeline_shards` | 58.192 | 25–30k | historische Tails/Repair/Recovery nach Authority-Cutover |
| `src/audit_only` | 29.199 | 8–10k | Shadow/Parity/alte Readbacks erst nach Ersatz-Gegenbeweis |
| `src/structural_write_authority` | 25.020 | 4–6k | nur Adapter/Uebergangsgerueste; Kern schuetzen |
| `src/bin` | 18.245 | 6–8k | Launcher/Export/Compatibility nach Pfadkonsolidierung |
| `src/build_gate` | 13.548 | 3–4k | nur bewiesen pensionierte Transition-Capabilities |
| uebriges `src` | — | 8–10k | alte Doppelwege nach Owner-Konsolidierung |

Die Korridore sind Portfoliogrenzen, keine bereits autorisierten Loeschmengen. Ueberschneidungen und notwendige neue gemeinsame Abstraktionen muessen netto bilanziert werden.

Allein `src/pipeline_shards/*tail.rs` umfasst aktuell **28.028 LOC**. Das ist der groesste sichtbare Evolutionssaum. aber nicht automatisch tote Logik.

## Erste Triage — pruefen, nicht loeschen

Die folgenden Dateien erhalten wegen Groesse plus historischen Signalen die hoechste **Pruefprioritaet**. Die Liste erteilt keinerlei Delete-Recht.

| LOC | Pfad | Review-Klasse | Signale |
|---:|---|---|---|
| 4908 | `src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; name:recovery; content:legacy=3; content:noop=1 |
| 4468 | `src/audit_only/phrase_boundary_evidence.rs` | PRIORITY_AUDIT_CONSOLIDATION_REVIEW | content:legacy=35; content:audit-only=1 |
| 3395 | `src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:legacy=1; content:audit-only=2 |
| 2948 | `src/pipeline_shards/corpus_fastjump_human_potential_repair_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; name:repair |
| 2790 | `src/pipeline_shards/hidden_chord_reduction_texture_rebuild_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail |
| 2629 | `src/pipeline_shards/strict_staff_visible_lane_rest_rewrite_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:retired=1; content:legacy=1; content:shadow=43 |
| 2160 | `src/audit_only/source_selection_full_causal_audit_shadow_plan.rs` | PRIORITY_AUDIT_CONSOLIDATION_REVIEW | name:shadow; content:legacy=27; content:shadow=97; content:audit-only=1; content:noop=2 |
| 1935 | `src/pipeline_shards/noether_register_articulation_readback_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:legacy=31 |
| 1817 | `src/audit_only/wav_attack_pulse_metric.rs` | PRIORITY_AUDIT_CONSOLIDATION_REVIEW | content:legacy=32; content:audit-only=1 |
| 1450 | `src/pipeline_shards/phrase_slur_register_octave_accent_stem_human_pdf_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:retired=8; content:retirement=2; content:audit-only=2 |
| 1232 | `src/pipeline_shards/rest_glyph_retention_batch_corpus_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:retired=1; content:retirement=1; content:shadow=16 |
| 1177 | `src/pipeline_shards/audio_salience_discrete_action_conflict_gate_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:legacy=7; content:audit-only=3; content:noop=2; content:no-op=1 |
| 1123 | `src/pipeline_shards/cross_staff_editorial_voice_math_gate_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail |
| 1010 | `src/pipeline_shards/lower_dual_edge_beat_beam_continuity_repair.rs` | PRIORITY_RETIREMENT_REVIEW | name:repair; content:retired=16; content:retirement=1; content:legacy=8; content:no-op=1 |
| 910 | `src/pipeline_shards/staff_voice_move_restify_atomic_patch_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail |
| 902 | `src/pipeline_shards/staff_voice_rest_glyph_topology_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:retirement=1; content:legacy=1; content:shadow=37; content:audit-only=1 |
| 725 | `src/pipeline_shards/partial_beat_post_rest_triple_beam_final_repair.rs` | PRIORITY_RETIREMENT_REVIEW | name:repair; content:retired=3; content:noop=1 |
| 717 | `src/audit_only/wav_pitch_harmonic_evidence.rs` | PRIORITY_AUDIT_CONSOLIDATION_REVIEW | content:legacy=11; content:audit-only=1 |
| 685 | `src/pipeline_shards/report_hygiene_cursor_localbest_trace_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:audit-only=1 |
| 678 | `src/pipeline_shards/sheaf_noether_harmonic_micro_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:shadow=5; content:noop=10 |
| 578 | `src/pipeline_shards/notation_endpoint_clef_layout_playability_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail |
| 537 | `src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs` | PRIORITY_RETIREMENT_REVIEW | name:tail; content:noop=3 |
| 421 | `src/audit_only/wav_observation_legacy_oracle.rs` | PRIORITY_AUDIT_CONSOLIDATION_REVIEW | name:legacy; content:legacy=23; content:audit-only=1 |
| 297 | `src/pipeline_shards/composer_recovery_hand_cause_core.rs` | PRIORITY_RETIREMENT_REVIEW | name:recovery |

## Bekannte Gegenbeispiele gegen blindes Loeschen

- `lower_dual_edge_beat_beam_continuity_repair.rs`: produktive historische Recovery ist pensioniert, aber committed-state Regressionsevidenz und lebende FinalCompleteBeat-Planung sind noch vorhanden. Erst auseinanderziehen, dann alten Anteil entfernen.
- `partial_beat_post_rest_triple_beam_final_repair.rs`: der Kommentar bezeichnet den Recovery/Audit-Pfad als retired, zugleich bleibt ein Readback gegen Wiederauftreten der historischen Lane-Split-Domaene. Negative Check zuerst migrieren.
- `source_selection_full_causal_audit_shadow_plan.rs`: explizit audit-only und ohne Writer-Recht, enthaelt aber noch unabhhaengige Shadow-/Causality-Evidenz. Erst nach abgeschlossenem Source/Lane/Voice-Cutover konsolidieren.
- `rustig_clef_planning.rs`: mischt produktiven Readability-Solver, postwrite Audit und historische Legacy-Aussagen; kein Datei-Retirement, bevor diese Rollen getrennt und Owner einzeln bewertet sind.

## Arbeitsstrategie

Das 60k-Ziel wird als **Harvest** an den Finalpfad gekoppelt, nicht als separates Cleanup-Projekt:

1. ohnehin notwendiger fachlicher/technischer Cut beweist eine neue Authority;
2. Gegenbeweise und Readbacks werden an die neue Authority gebunden;
3. alter Writer/Shadow/Fallback verliert sein letztes exklusives Recht;
4. unmittelbar danach wird der gesamte ersetzte Pfad physisch pensioniert;
5. RustLinesTotal, RetiredSemanticLOC, ActiveSemanticOwners und ActiveProductiveWriters werden neu bilanziert;
6. Vollkorpus muss dunkelgruen bleiben.

Damit soll der Code waehrend der verbleibenden Finaltakes schrumpfen, ohne die IR-Migration oder ein separates Grossrefactoring auf den kritischen Pfad zu ziehen.

## Zielkorridor ueber die Reststrecke

- nach vollstaendiger Execution-/Witness-Kette: kumuliert etwa **-10 bis -20k**
- nach notwendigen Notations-/Authority-Cutovers: kumuliert etwa **-35 bis -45k**
- vor finalem NOOP nach Shadow-/Legacy-Konsolidierung: **-50 bis -60k**

60k ist Ziel, kein Release-Gate. Wenn die letzten Zeilen einzigartige Witness-, Recovery- oder Negative-Check-Capability tragen, bleiben sie bestehen.

## Maschinenlesbares Inventar

`docs/semantic_owner_retirement_inventory_r723.tsv` listet jede Rust-Datei aus `src/`, `tests/` und `tools/` mit physischer LOC, Review-Klasse und rein heuristischen historischen Signalen. Diese Klassifikation besitzt keine Writer-, Delete- oder Release-Authority.
