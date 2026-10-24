**Historischer Plan (Basis r1368), nicht pauschal abgeschlossen.** Der folgende
Zensus und seine Take-Reihenfolge dokumentieren den damaligen Stand. Custody
ist derzeit pausiert; Wiederaufnahme nur unter den Bedingungen im Kurzstand.

Aktueller Kurzstand: [050_aktueller_stand.md](050_aktueller_stand.md).
Der historische Inhalt bleibt zur Referenz am bisherigen Pfad erhalten.

---

# 040 – Stimmen/Course-Custody: Lückenliste, Take-Reihenfolge, Gate-Checkliste

Advisory only. Keine Autorität, kein Gate-Ersatz. Stand: accepted r1368, Korpuslauf
10/10 grün, 11.538 `CustodyCensus`-Zeilen (r1368, nur Master-Lineage).
Bei neuem Stand: Zensus aus der frischen Konsole neu aggregieren, nicht diese Zahlen übernehmen.
Ab r1372 schreibt der Zensus nur informative Zeilen (reine Text-Übergänge bereits getragener
Receipt-Arten entfallen); Zeilenzahlen sind daher nicht mit r1368 vergleichbar, die Lücken schon.

## Ziel und Stand in einem Satz

Für die Kanalzuteilung (weniger Voice-Labels; Kanalplanung sitzt laut r1345 in
`serialization_plan.rs` am Ende der Pipeline) muss jede Note ihre Course-Custody bis
dorthin tragen. Heute endet die Custody in allen 10 Projekten am ersten
ConsensusGesture-Schritt (Row 26). Nach dem Solver wird Custody gar nicht weitergereicht.

## Regeln, gegen die ein Owner verstößt (owned_rhythmic_continuation.rs)

Transparent ist ein Owner nur, wenn er keine Note löscht oder neu erzeugt, Intervall,
Staff und Disposition (`<pitch>`-Text, rest, chord, grace, Tie-Marken) unverändert lässt
und EXACT-Receipts liefert. Births nur für Rows 3/39/63, Staff-Transition nur für Row 4,
BLOCK nur mit Birth-Capture (Row 52). PRESERVING/ADDITIVE sind in
`OwnerReceipts::continue_owned_rhythm` hart unsupported.

## Lückenliste aus dem Zensus (Master-Adoptionen, Summe über 10 Projekte)

Spalten: Adoptionen | gelöscht/neu | Staffwechsel | Disposition | nur Text.
Bei Adoptionen mit mehreren Receipts zählt die Menge bei jedem beteiligten Owner.

### Bereits getragen (keine Arbeit)
| Row/Owner | n | Befund |
|---|---|---|
| PRESERVING polish_musicxml | 5942 | nur Text; Transport existiert |
| BLOCK 52 normalize_visible_rhythm | 2008 | nur Text; Birth-Capture existiert |
| EXACT 4 staff_voice_move | 789 | 789 Staffwechsel; Transition seit r1361 |
| EXACT 63 restify_patch | 738 | 738 Disposition; Birth seit r1358 |
| EXACT 3 safe_transfer | 481 | 481 neu + Disposition; Birth seit r1358 |
| EXACT 11 remove_note_hidden_attrs | 294 | nur Text; EXACT generisch tragbar |

### Familie A – Receipt-Typ unsupported, Noten unverändert (1 generischer Take)
PRESERVING visual_layout_defaults, local_staff_distance, rustig_clef_course,
final_render_notation_with_phrase_projection, partial_beat_post_rest_beams;
ADDITIVE final_barline, direction_insertion; BLOCK 49 clef_measure, 50 octave_measure,
53 beat_scoped_beaming (nur Text). Alle 0 gelöscht/neu/Staff/Disposition.
Lösung: Byte-Carry für Receipts, deren Noteninventar sich nur im Text ändert
(gleiche Paarung wie `custody_census`), ohne neue Identität.

### Familie B – Löschen + Neu (größter Blocker)
| Row/Owner | n | gelöscht/neu | Disposition | Staff |
|---|---|---|---|---|
| EXACT 26 chord_gesture_distribution | 1133 | 1245/1245 | 131 | 7 |
| EXACT 1 editorial_voice_complete_rewrite | 8 | 69/49 | 1 | 0 |
Row 26 ersetzt Noten 1:1 (gleich viele gelöscht wie neu). Braucht Removal-/Birth-
Accounting wie Restify/Transfer, gebunden an das Original-Edit-Script. Erst Row 26 lösen,
dann ist der Solver-Pfad frei und spätere Lücken werden in OwnedRhythmicContinuation sichtbar.

### Familie C – Disposition (Transitionsregel nötig)
| Row/Owner | n | Disposition | Hinweis |
|---|---|---|---|
| EXACT 8 harmonic_pitch_swap | 42 | 42 | Tonhöhe ändert sich bewusst |
| EXACT 5 hide_note_print_object + EXACT 6 remove_allowed_xml_ranges | 10–12 | 814 | gemeinsame Adoption (MasterNotationPrelude); vermutlich chord-Flag nach Entfernen versteckter Akkordtöne |

### Familie D – Release-Stufe (inhaltliche Entscheidung nötig)
EXACT 65 final_chord_encoding, 59 master_musicxml_output, 60 functional_respelling
(ReleasedMaster, gemeinsame Adoptionen): je 2/2 gelöscht/neu, 886 Disposition.
Enharmonische Umschreibung ändert den `<pitch>`-Text bei gleicher Tonhöhe.
Entschieden (Wolfgang, 2026-10-12): Enharmonische Umschreibung ist dieselbe Occurrence bei
gleicher MIDI-Tonhöhe, gleichem Einsatz, gleicher Dauer, gleichem Staff; Tie-Ketten gemeinsam.
Normativ: `docs/enharmonic_occurrence_identity_contract_v1.md`.

### Nicht erfasst
Render-Lineage (`render_lineage.rs::commit`, gepinntes `render_adopted_state.rs`):
Initial-Render- und Final-Notation-Writer (Rest-Familie, Beams, Sichtbarkeit) fehlen im
Zensus. Erfassung braucht eine Governance-konforme Erweiterung (R57-Pin beachten).

## Empfohlene Take-Reihenfolge (gebündelt)

1. **Row 26 Gesture-Relokation**: umgesetzt in r1373 (siehe 041); am frischen Export prüfen, welche Lücke danach als erste sichtbar wird.
2. **Familie A generisch + Custody-Durchreichung in MasterNotationPrelude und ReleasedMaster**
   (Messung, kein Fail): danach sind Folge-Lücken in einem Lauf sichtbar.
3. **Familie C** (Row 8, Rows 5/6) mit expliziter Transitionsregel.
4. **Familie D** nach Enharmonik-Entscheidung.
5. **Render-Zensus**, dann Render-Custody familienweise.
6. Danach Kanalzuteilung (3–5 Takes) und Umstellung der Rest-/Beam-Leser (2–4 Takes).

## Gate-Checkliste vor jeder Übergabe (aus realen Ablehnungen r1365–r1368)

- Take-ZIP enthält **kein `Cargo.lock`** (LocalTransactional). Erlaubt im Root nur:
  Cargo.toml, README.md, ZIP_KIND.txt, run.cmd, strict.cmd, rust-toolchain.toml,
  rustfmt.toml, clippy.toml sowie src/ docs/ tools/ tests/.
- Neue Test-/Source-Zeilen: **keine Ziffer nach** `<measure number=`, `measure_`, `takt_`,
  `measure ==` usw.; Taktnummern nur über Variablen (`format!("<measure number=\"{ordinal}\">")`)
  oder den Prüfstand `tests/support/synthetic_score.rs`, **nie als Wort** (`"probe"` bricht
  numerische Binder); keine Etüden-Namen mit Ziffer;
  kein `fn …regression`; keine Korpus-Fixtures.
- Neue `.rs`-Zeilen: Kontrollfluss (`if`, `match`, `&&`, `.filter(` …) **und** Wörter wie
  candidate, target, scope, verdict, decision, repair, selector, blocker im selben Datei-Diff
  verlangen genau einen Selector-Derivation-Vertrag. Neutral benennen oder Vertrag beilegen.
  Auch Testhilfen zählen: eine Taktnummer-Variable `scope` plus `if`/`.filter(` im selben
  Datei-Diff löst den Vertrag aus; im Prüfstand heißt sie `ordinal`.
- `commit_owner_receipts` und `render_lineage.rs::commit` sind SSC-Wiring, `render_adopted_state.rs`
  ist R57-gepinnt: nicht anfassen.
- Der Raw-Splice-Harness kompiliert `master_state_identity.rs` nur **bis `mod render_lineage;`**;
  neue Methoden mit neuen Crate-Abhängigkeiten dahinter in eigenem `impl` ablegen.
- Manifest `src/build_gate/take_change_class.txt`: jede geänderte Datei außer dem Manifest
  selbst, Hash = FNV-1a-64 der exakten Bytes.
- Vor Übergabe die Bootstrap-Prüfungen lokal laufen lassen: `validate_dead_code_detection_integrity_transition`,
  `validate_incoming_change_dependencies`, `validate_incoming_take` aus `src/take_change_control.rs`
  (eigene Probe-Crate mit `#[path]`-Modulen wie in `src/bin/etuschlopf_take_bootstrap.rs`).

## Bekannter intermittierender Fehler

`Physical course continuation moved or changed support … exact_note=false`
(arrangement_rhythmic_courses.rs `validate_notes`, Panic über `.expect` in arrangement.rs).
Tritt nicht in jedem Lauf auf (r1365 ja, r1367/r1368 nein). Intervall gleich, Notenbytes
verschieden. Bei Auftreten: Lauf wiederholen; Ursache noch offen.
