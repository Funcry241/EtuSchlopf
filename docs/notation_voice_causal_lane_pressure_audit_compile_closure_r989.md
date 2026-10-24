# r989 NotationVoiceCausalLanePressureAuditCompileClosure

## Parent und Klasse

r989 baut erneut direkt auf dem real akzeptierten r987 `NativeInventoryKodexWarningDebtClosure` auf. Der verworfene r988 `NotationVoiceCausalLanePressureAudit` bleibt Failure Evidence und ist niemals Parent. Der reale Windows-Strict-Clippy-Lauf stoppte r988 ausschliesslich beim Kompilieren des neuen Audit-Index: Rust konnte fuer `let mut index = HashMap::new()` den Value-Typ hinter `.entry(...).or_default().push(row)` nicht eindeutig inferieren (`E0282`).

r989 reappliziert deshalb dieselbe `AuditOnly`-Fachaenderung direkt auf r987 und schliesst nur diese Compile-Luecke durch den expliziten lokalen Typ `VoiceFreeSourceIndex<'_>`. Auditentscheidung, Join-Schluessel, LanePressureWitness, Reportprojektion und drei Tests bleiben fachlich unveraendert. Erwarteter Strict-Testbestand bleibt 1.283.

## Anlass

r984 zeigte 338 finale Beam-/Sequenzfaelle als moegliche gemeinsame sichtbare Notationsstimme, bewies aber noch nicht kausal, warum die Ereignisse auf unterschiedlichen technischen Sounding-Lanes liegen. r983 hatte zuvor gezeigt, dass ein pauschales Zusammenfuehren von Primary/Secondary bei fehlender Notationsueberdeckung zu breit ist.

Der Audit prueft deshalb einen engeren Beweis: Eine technische Lane-Trennung darf nur dann als Evidenz fuer dieselbe sichtbare Notationsstimme gelten, wenn die finale Render-Notation eine zusammenhaengende primaere Balkenkette bildet und jede benoetigte niedrigere Sounding-Lane am Attack-Beginn ausschliesslich durch Ereignisse derselben Balkenkette blockiert wird. Damit wird der Lane-Wechsel kausal als eigener Nachklang der sichtbaren Linie erklaert.

## Audit-Grenze

Es gibt keine neue Datenkante in gemischter produktiver Source. Die bestehende Audit-Owner-Grenze `src/pipeline_shards/final_report_projection_owner.rs` besitzt am finalen Report-Checkpoint bereits Zugriff auf `CandidateMaterialLineageSpine` und projiziert daraus read-only die benoetigten Source-/Lane-Zeilen in einen Audit-Owner unter `src/audit_only/`.

Geaendert werden nur Audit-/Metadatenpfade. `candidate_material_lineage_spine_core.rs`, `FrozenSoundingTemporalLanePlan`, `FrozenSerializedVoicePlan`, FinalChord, Beam-/Rest-/Clef-/Ottava-Writer und MusicXML/PDF-Materialisierung bleiben unveraendert.

## Voice-freier Join

Der kausale Audit bindet finale sichtbare Beam-Mitglieder an gefrorene Attack-Evidenz ausschliesslich ueber Measure, finalen Attack-Onset, Staff und MIDI-Pitch. Das aktuelle MusicXML-`<voice>`-Label wird nur im Diagnose-Text ausgegeben und besitzt keine Join-, Auswahl- oder Entscheidungs-Authority. Ist dieser Join nicht eindeutig, bleibt der Fall `UNRESOLVED_VOICE_FREE_SOURCE_MAPPING`.

## LanePressureWitness

Fuer jedes eindeutig gebundene Ereignis auf technischer Lane > 1 prueft der Audit jede niedrigere Lane. Eine niedrigere Lane ist am Attack-Start genau dann ein Pressure-Witness, wenn ihr gefrorenes Sounding-Intervall den Attack-Start ueberdeckt.

- gehoeren alle erforderlichen Blocker derselben primaeren Balkenkette an, ist der Lane-Druck `self-induced`;
- fehlt fuer eine benoetigte niedrigere Lane ein Blocker, bleibt der Fall `UNRESOLVED_LANE_PRESSURE_WITNESS`;
- stammt mindestens ein Blocker von ausserhalb der Balkenkette, bleibt der Fall `UNRESOLVED_EXTERNAL_LANE_PRESSURE`.

Nur eine finale, notatorisch lueckenlose primaere Balkenkette mit mindestens zwei technischen Lanes und vollstaendig selbstinduziertem Lane-Druck wird `PROVEN_SAME_NOTATION_VOICE`.

## Keine Schreibrechte

Der Audit erzeugt ausschliesslich die Reportsektion `NOTATION-VOICE-CAUSAL-LANE-PRESSURE-AUDIT`. Keine Voice wird veraendert. Kein Beam, Rest, Clef, Ottava, Staff oder Pitch wird geschrieben. Es entsteht keine Writer-, Repair-, Selection-, Release- oder MusicMutation-Authority. `technical_lane` ist ausschliesslich kausale Evidenz und keine direkte Voice-Authority.

## Tests

Drei generische Tests pinnen:

1. self-induced Sounding-Overlap beweist bei lueckenloser Beam-Linie `PROVEN_SAME_NOTATION_VOICE`;
2. ein externer Lower-Lane-Blocker bleibt fail-closed `UNRESOLVED_EXTERNAL_LANE_PRESSURE`;
3. eine Permutation der aktuellen MusicXML-Voice-Labels aendert die kausale Entscheidung nicht.

Ausgehend von r987 mit 1.280 Strict-Tests werden 1.283 Tests erwartet. Der reale Windows-Lauf entscheidet fail-closed ueber Acceptance.
