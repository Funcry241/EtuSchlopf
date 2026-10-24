**Historischer Bauauftrag; Row 26 seit r1373 umgesetzt.** Die folgende
Vorbereitung und Basis-Evidenz sind Historie, kein naechster Builder-Auftrag.
Dies behauptet keinen Abschluss der gesamten Custody-Kette.

Aktueller Kurzstand: [050_aktueller_stand.md](050_aktueller_stand.md).
Der historische Inhalt bleibt zur Referenz am bisherigen Pfad erhalten.

---

# 041 – Bauauftrag Row 26: Custody durch ConsensusGesture-Relokation

**Status: umgesetzt in r1373** (RefactorOnly). Owner-Capture im Body von Row 26 (Signatur unverändert),
`PhysicalBirthKind::GestureRelocated` mit Anker-Bytes, Relokationskante `Origin::relocation`
(I1/I2/I3 wie unten), Row 26 in der Liste von `continue_owned_rhythm`, Lückenmessung
`relocation_vacancy_count` (Unknown in `require_complete_course_context`, nie geschlossen).
Tests: `tests/unit/gesture_relocation_custody.rs` (8). Prüfung am frischen Export:
OwnedRhythmicContinuation bei ArrangementSolverFinal nicht mehr „PopulationRow=26“ als erster Abbruch.

Advisory only. Vorbereitet für den nächsten Builder-Take. Basis-Evidenz: r1368-Zensus
(Row 26: 1133 Adoptionen, 1245 gelöscht / 1245 neu, 131 Disposition, 7 Staff) und
OwnedRhythmicContinuation „Unknown … OwnerStep=1 PopulationRow=26“ in 10/10 Projekten.

## Was der Owner tatsächlich tut (Code gelesen, nicht vermutet)

`structural_write_authority/final_buffer_surface_authority.rs::apply_chord_gesture_distribution_batch_authorized`
(EXACT #26) wendet Edit-Paare an, die in
`pipeline_shards/hidden_chord_reduction_texture_rebuild_tail.rs` gebaut werden
(`consensus_probe_apply_gauss_gesture_distribution_inner`, `consensus_probe_apply_broken_gesture_distribution`):

- `(tone.abs_start, tone.abs_end, "")` löscht einen versteckten Akkordton,
- `(anchor.note_end, anchor.note_end, clone)` fügt ihn hinter einer sichtbaren Anker-Note ein.
- `clone = consensus_note_block_as_visible_chord(source_block, anchor)`: print-object/print-spacing
  entfernt, `<chord/>` gesetzt, **duration, voice, staff vom Anker übernommen**.
- Die Paare stehen bereits in `moves: Vec<(abs_start, abs_end, anchor.note_end)>` und werden zu
  `CandidateMaterialExactNoteMoveCarrier` verarbeitet.

Folge: Onset/Dauer (Δzeit) und ggf. Staff ändern sich. Darum scheitern alle bestehenden
Birth-Arten (Transferred verlangt gleiches Intervall) und `carry_range` meldet
„removed or invalid occurrence requires separate accounting“.

Dritte Aufrufstelle: `render/silent_voice_visible_measure_coverage_render_writer.rs` (Render-Domain,
anchor_promotion_plan) – nicht Teil dieses Takes.

## Vorgeschlagener Schnitt (ein Take)

1. Neue Birth-Art `PhysicalBirthKind::GestureRelocated` und
   `PhysicalRhythmicBirths::gesture_relocation(owner: &ChordGestureDistributionExactMint, before, after, moves)`,
   analog `transfer`: Input = Notenbereich des gelöschten Tons im Pre-Dokument, Output = eingefügter
   Clone im Post-Dokument (Offsets aus dem gebundenen Edit-Script, wie `restification`).
   Identität über einen neuen `RhythmicCreationIdentity::gesture_relocation_birth(&ChordGestureDistributionExactMint)`.
2. Capture im Owner-Body, exakt nach dem Muster von Row 3
   (`remaining_atomic_patch_authority.rs::apply_bound_safe_transfer_authorized`):
   nach `mint_exact_receipt` → `if let Ok(births) = PhysicalRhythmicBirths::gesture_relocation(...)`
   → `receipt.attach_physical_births(births).expect(...)`. Kein neuer Rückgabewert, keine neuen
   Parameter: die Owner-Signatur ist in `src/build_gate/ssc_population_prerequisites_r1250_v1.txt`
   (Zeile Row 26) und `receipt_surface_test_parent_identity_v1.txt` gebunden und muss
   byte-gleich bleiben. Der Body ist nicht gepinnt.
   Paarung aus den Original-`edits` (Eingabereihenfolge, vor `issue`): Die ConsensusGesture-Aufrufer
   pushen strikt abwechselnd `(start,end,"")` / `(pos,pos,clone)`. Nur wenn die Liste exakt aus
   solchen Paaren besteht, wird eine Capture gebaut; sonst keine (→ Custody bleibt Unknown, wie heute).
   Wichtig: Die dritte Aufrufstelle (Render, `anchor_promotion_plan`, 2×Promotions + Beam-Completions)
   hat eine andere Edit-Struktur und muss dadurch ohne Capture bleiben; sie liegt in der Render-Lineage
   und betrifft `continue_owned_rhythm` nicht.
   Output-Offsets wie in `restification` aus der aufsteigend sortierten Edit-Arithmetik berechnen,
   nie im Output suchen.
3. `owned_rhythmic_continuation.rs::continue_operations`: Familie `[GestureRelocated]` zulassen mit
   Prüfungen: gleiche MIDI-Tonhöhe (siehe enharmonic_occurrence_identity_v1), Quelle (`origin.source`)
   vorhanden, Output ist `<chord/>`-Mitglied, Staff/Intervall = Anker-Note.
4. `master_state_identity.rs` Zeile ~2017 (`continue_owned_rhythm`, „Missing actual creator context
   capture“, `3 | 39 | 63`) um 26 erweitern. Die gleichlautende Liste in `physical_birth_readback`
   (~2055) NICHT erweitern: Ihr einziger Produktivaufrufer ist der SafeAtomicCandidateProducer
   (`corpus_fastjump_human_potential_repair_tail.rs:1095`), der Row 26 nicht erzeugt; eine Erweiterung
   brächte nur Risiko ohne Nutzen.

Anker-Bindung: Die Projekt-Verträge verbieten, Course-Zugehörigkeit aus XML-Geometrie zu rekonstruieren.
Deshalb bei der Capture zusätzlich den Pre-Bereich der Anker-Note festhalten (die Insert-Position ist
`anchor.note_end`, also Ende der Anker-Note; Anfang über `captured_note_range` auf genau diese Note).
Die Fortsetzung findet den Anker dann über seine getragenen Bytes (`old.bytes`) im aktuellen
Custody-Zustand, nicht über Staff/Zeit-Suche. Dafür braucht `PhysicalBirth` ein optionales Feld
`anchor: Option<Range<usize>>` (nur für GestureRelocated gesetzt).

## Entscheidungen (Wolfgang, 2026-10-12) – vor dem Bau geklärt

1. **Course:** Ist der verschobene Ton am Ziel tatsächlich Akkordton des Ankers, gehört er dem
   Course des Zielankers an (gleicher Einsatz, gleiche Dauer, gleiche Stimme). Seine ursprüngliche
   Quellenidentität (`origin.source`, Parent-Kante) bleibt erhalten. Der Übergang vom bisherigen
   zum Ziel-Course wird ausdrücklich transportiert und am empfangenden Owner geprüft. Gleiche
   Rhythmuswerte allein begründen keine Zugehörigkeit.
2. **Verpflichtung:** Mit dem ausdrücklich belegten Wechsel zum Anker-Course wandert auch die
   Verpflichtung des Tons. Die alte Stelle beansprucht ihn danach nicht mehr; dort wird keine Pause
   erfunden. `print-object="no"` erklärt nur die Unsichtbarkeit; den Wechsel begründet die autorisierte
   Gesten-Verteilung (Row 26). Bleibt im alten Course eine echte zeitliche Lücke, wird sie gemessen und
   als Unknown ausgewiesen – ohne automatische Schließung oder Freigabe.

## Drei Invarianten, an denen ein naiver Bau scheitert (owned_rhythmic_continuation.rs)

`OwnedRhythmicContinuation::validate_binding` würde eine Relokation mit Course-Wechsel ablehnen:

| # | Prüfung | Fehlermeldung |
|---|---|---|
| I1 | Jede Kante der Ahnenkette hat dieselbe root/course/source/presentation wie die Occurrence | `Continuation changed course, source or course role` |
| I2 | Kind-Intervall liegt im Eltern-Intervall | `Birth extended received support` |
| I3 | Vereinigung der Nachkommen-Intervalle deckt jede Root-Verpflichtung | `Owner continuation lost a received obligation` |

Ohne Modelländerung wird der Take entweder abgelehnt oder die Custody reißt eine Stufe später.

## Modell: ausdrückliche Relokationskante

- `Origin` erhält ein Feld `relocation: Option<Relocation>` mit `{ from_course, to_course,
  to_presentation, anchor: RhythmicCreationIdentity, vacated: Range<i64> }`, nur gesetzt bei
  `PhysicalBirthKind::GestureRelocated`, nur erzeugt in `continue_operations` aus einer Row-26-Capture
  mit Anker-Bytes. Der Anker ist eine zu diesem Zeitpunkt getragene Occurrence (Suche über `old.bytes`,
  nie über Staff/Zeit); `to_course`/`to_presentation` werden aus dessen Origin übernommen.
- I1: Beim Aufstieg durch die Kette ist ab einer Relokationskante der Vergleichswert für
  course/presentation der `from_course` dieser Kante; root und source bleiben überall gleich.
  Course/presentation dürfen sich also **genau an** Relokationskanten ändern, sonst nirgends.
- I2: An einer Relokationskante gilt statt „Kind ⊆ Eltern“: Kind-Intervall == Intervall des Ankers
  (zum Capture-Zeitpunkt) und das Kind ist `<chord/>`-Mitglied des Ankers. Für alle anderen Kanten bleibt I2.
- I3: Die Root-Verpflichtung gilt als übergeben für das Intervall `vacated` des relozierten Teils.
  Die Abdeckungsprüfung zieht `vacated` von der Root-Verpflichtung ab, statt es als verloren zu werten.
  Keine Pause, keine Freigabe.
- Lücke im alten Course: Eine zusätzliche Mess-Ausgabe (nicht in `validate_binding`) zählt, ob der alte
  Course nach der Relokation unrepräsentierte Zeit im Intervall `vacated` hat; diese erscheint in
  `owned_rhythm_diagnostic_line` als Unknown-Anteil. Kein Fehler, keine Schließung.
- Course-Lebensdauer: `to_course` muss das neue Intervall nach `coverage` lizenzieren (bestehende
  Prüfung „Unlicensed course role or support extension“ bleibt unverändert).

## Pflicht-Tests (synthetisch, `<measure number="probe">`)

- Relokation mit Capture und Anker-Bytes → fortgesetzt; Occurrence hat Anker-Course, alte Quelle, Parent-Kante.
- Gleiche Situation ohne Capture → Unknown (Owner wird nicht abgewiesen).
- Capture mit Anker, dessen Bytes nicht getragen werden → Fehler.
- Ziel-Intervall ≠ Anker-Intervall, oder kein `<chord/>` → Fehler (I2 bleibt streng).
- Course-Wechsel ohne Relokationskante (z. B. manipulierte Origin) → I1-Fehler.
- Relokation ändert MIDI-Tonhöhe → Fehler; enharmonische Umschreibung bei gleicher MIDI-Tonhöhe → erlaubt
  (enharmonic_occurrence_identity_v1).
- Alter Course behält Lücke → Mess-Ausgabe zählt sie, `validate_binding` bleibt grün.
- Zwei identische Ausführungen → verschiedene Identitäten.

## Test-Hinweise (Gate E32)

Synthetische Ein-Takt-Dokumente, `<measure number="probe">` (keine Ziffer), kein Etüden-Bezug.
Fälle: gleiche Tonhöhe → fortgesetzt; andere Tonhöhe → Fehler; fehlende Birth-Capture → Unknown;
zwei gleiche Ausführungen → verschiedene Identitäten.

## Erfolgskriterium

OwnedRhythmicContinuation meldet bei ArrangementSolverFinal nicht mehr Row 26 als ersten Abbruch;
der nächste Abbruch (laut Zensus vermutlich Row 8 oder Row 1) wird sichtbar.
