> Historisches Arbeitsdokument, Befundstand r1305. Kein aktueller Bauauftrag.
> Die damaligen neun Kollisionen und erzwungenen PDF-Aufrufe beschreiben nur
> diesen alten Stand. Aktuelle Referenz und Abnahme: siehe
> [050 - Aktueller Stand](assistant_memory/050_aktueller_stand.md) und frische Laufbelege.
> Der historische Text bleibt darunter unveraendert erhalten.

# Opus: Stimmen-Kollisionen – belegter Einstieg nach accepted r1305

Advisory only. Keine neue Musik-, Writer-, Selector-, Proof- oder Release-Authority.
Belegstand: accepted r1305, SHA-256
`39672b8a5ba189c133d07c53d87a15304fbc750409833c79f25fb11dcce0f8a7`.
Dieser historische Befund ersetzt nicht die frischen Artefakte des aktuellen Exports.

## Was bewiesen ist

Windows: FinalExit 0, 10/10, Korpus 5:02. Alle 20 Master-/Render-XMLs sind
bytegleich zu r1304. Neun Kollisionen bleiben; MuseScore braucht bei den fünf
betroffenen Etüden weiterhin `-f`. Das ist kein erfolgreicher Musik-Fix.

Alle neun Einführer sind jetzt direkt belegt:
`src/structural_write_authority/serialization_plan.rs::project_final_chord_encoding_plan`.
Vorher-/Nachher-Taktprotokolle und separate erfolgreiche Render-/Master-Übernahme
stehen in `CollisionIntroducer` und `FinalChordProjectionAdoption` der Resultate.
Acht betroffene Noten wechseln 1→2, die neunte 2→1; Einsatz/Tonhöhe/Dauer dieser
Noten bleiben an diesem Übergang gleich. Manche Berichtsprojektionen maskieren
Taktnummern als `scope-`; die XML-Takte bleiben die physischen Locator-Belege.

| Evaluation, kein Selektor | Takte und Noten | Stimmenwechsel |
|---|---|---|
| Etüde 1 | 12 D4; 17 G4 | 1→2 |
| Etüde 2 | 13 F#4 | 1→2 |
| Etüde 3 | 15 C#6 | 1→2 |
| Etüde 6 | 16, 22, 26 F#4; 42 F#3 | 1→2; beim F#3 2→1 |
| Etüde 7 | 25 Eb4 | 1→2 |

r1302 hatte EditorialVoiceDecision-Trials beobachtet, aber deren Übernahme nicht
bewiesen. Der r1303-Angebotsguard verhindert solche Trials; er änderte keine der
20 finalen XMLs. Den fachlichen Fix deshalb nicht erneut dort ansetzen.
Worker-Threads waren nicht die belegte Ursache dieser Messlücke; die diskutierten
`thread::spawn`-Stellen in `master_adoption/adopted_state.rs` waren Testcode.

## Kleine Lesereihenfolge

1. Im frischen Export: `take_lifecycle_handover.txt`, dessen Acceptance-Beleg und
   `assistant_context.json`; dann gebundenen Source-Snapshot auflösen.
2. Fehlerklasse: `docs/selector_derivation_contract_staff_relocation_notation_integrity_error_class.md`.
3. `serialization_plan.rs`: `FrozenSerializedVoicePlan`,
   `build_frozen_serialization_voice_channels_for_canonical`, `bind_musicxml_voice_labels`,
   `label_for_source`, dann `bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound`.
   Die physische Umbezeichnung steht bei `final_chord_set_child_text(source, "voice", label)`.
4. Renderpfad: `pipeline_orchestrator_final_notation_support.rs`,
   `run_final_render_notation_authority_stage` → erfolgreicher FinalNotation-Receipt-Commit.
   Masterpfad: `pipeline_orchestrator_metric_final_notation_stage.rs` →
   `CommittedMasterExport` → `pipeline_orchestrator_release_support.rs` →
   `ReleasedMaster/FinalExport`. Beide Übernahmen haben eigene Audit-Nachweise.
5. Rohmessung: `audit_only/voice_run_collision_origin.rs`. Proben sind vorläufig;
   erst exakte Eingangs-/Ausgangsbytebindung nach erfolgreicher Übernahme bestätigt sie.

## Nächster fachlicher Schnitt

Ein eigener MusicMutation-Take auf dem dann letzten accepted Parent, Version
0.10.1839. Die fachliche Stimmen-/Kanalbindung so ableiten, dass die belegte
Fehlerklasse nicht entsteht. Nicht einfach die versiegelte Stimmenvorgabe im
Writer überspringen: FrozenPlan, Output-Alpha, Occurrence-Continuation und Receipts
müssen weiterhin dieselbe zulässige Darstellung bezeugen.

Die produktive Fehlerklasse ist corpusunabhängig: klingende Nicht-Akkord-Note in
einer fremden backup-begrenzten Folge; Zielstimme besitzt in anderer Folge eine
überlappende klingende Nicht-Akkord-Note mit anderem Einsatz oder anderer Dauer.
Staff allein genügt nicht; die Zielbelegung muss systemübergreifend geprüft werden.
Kein singleton/ranking-Trick und kein Etüden-/Taktnummern-Selektor.
`AtomicNote.run` ist seit r1303 verfügbar. Keine Production→audit_only-Abhängigkeit
für die fachliche Prüfung. `add_safe_atomic_variants` bleibt unangetastet.

Selector-Änderung: genau ein Implementierungsvertrag mit allen Pflichtfeldern;
ParentErrorClassContract wie oben, SelectorChange yes, SelectorRole repair_scope.
Keine neue Funktion außerhalb structural_write_authority, die einen Musik-Writer
reicht; auch Testfunktionen beachten. MusicMutation verändert die Audit-Dateien
nicht. Synthetische Tests: kollidierende, deckungsgleiche und nicht überlappende
Intervalle, beide Cross-Staff-Richtungen und Duplikationsinvarianz; zusätzlich
realer Writer-Durchstich in der vorhandenen kanonischen Authority.

Abnahme: FinalExit 0, 10/10; neun Kollisionen weg; `-o` ohne `-f`; sämtliche
klingenden Einsätze/Tonhöhen/Dauern taktweise erhalten, nur Voice-Wechsel erlaubt.
Etüden 4/5/8/9/10 bytegleich. Großflächige Schreib-/Sichtbarkeitsänderung: Take
zurücknehmen. Früh Parent-Admission; danach Clippy, vollständige Tests, ZIP
entpacken/bytevergleichen; kein Root-Cargo.lock, kein target, kein --force.

## Zeiten richtig lesen

Der aktuelle Handoff enthält LAUNCHER-TIMING-READBACK: Mikrosekunden-Walltime,
benannte Start/End-Offsets, inklusive Zeit und exklusive beobachtete Zeit aus der
Intervallunion. Parallel laufenden Test-Vorbau nicht zu Clippy/Build addieren.
Exit-Unter-/Obergrenzen zeigen die Poll-/Scheduler-Unschärfe. Reuse spart den
Child-Prozess, nicht automatisch die Zeit für Receipt-/Quellsignaturprüfungen.
run.cmd-/Take-Bootstrap und der Export-Tail nach dem Capture sind separat außerhalb
dieser Messdomäne; sie werden nicht als 0 ms ausgegeben. Pipeline-PhaseTiming
bleibt im Konsolenlog; inklusiven Eltern-/Unterphasen keine additive Summe geben.
