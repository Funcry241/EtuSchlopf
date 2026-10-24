# r1284 – SSC-Vertrag v4, Take 4b: Envelope-Erweiterungen und minimale Witnesses

Parent: accepted r1283 `SscK2GovernanceRevisionKodex`, Candidate-SHA `f020ffef4fd3f342450568cfa092deab160d0e6286ae936db7f7a1d68f92e211`.
Klasse: `GovernedChange`, `music_effect=none`. Shadow only, `PermitEnforcement: none`.

## Grundlage

Codex' A5-Analyse (`docs/ssc_denied_purpose_analysis_r1283.md`): 165 Denied = 137 (a) Zeile zu eng + 28 (b) Zweckklasse zu eng, 0 (c). Die (b)-Klassen hat r1283 korrigiert. Dieser Take trägt die sieben dort vorgeschlagenen Zeilen unverändert ein; jede liegt in der Zweckzuordnung des kompilierten r1283-Validators und ist syntaktisch `envelope_extend`.

| Descriptor | Erweiterung | Mechanismus (versiegelter Writer) |
|---|---|---|
| NOTE_VISIBILITY_EXACT | Feld `note@print-spacing` | `rewrite_note_opening_tag_hidden` setzt `print-object` und `print-spacing` |
| ALLOWED_RANGE_REMOVAL_EXACT | Element `attributes`, Operation `change` | sichere Unter-System-Clef-Hülle löschen; `slur`/`tied` aus einer bleibenden `notations`-Hülle löschen |
| RENDER_ROW_30_EXACT | `rest` als Ganzes (`display-step`, `display-octave`) | Rest-Tag-Form `<rest/>` → `<rest>…</rest>`, `rest@measure` entfällt |
| RENDER_ROW_51_EXACT | Element `forward` | Cursor-Payload: backup, forward, Rest-Note |
| FINAL_LAYOUT_PHRASE_PRESERVING | Elemente `direction-type`, `words` | dokumentierte Ottava-Re-Cue-Hülle beim finalen Layout-Neustart (r605) |
| FUNCTIONAL_RESPELLING_EXACT | `attributes`, `key`, `fifths` | lokale Tonart samt neuer attributes-Hülle |
| RENDER_ROW_66_BLOCK | `attributes` | dieselbe Hülle, gemessen exakt +25 kanonische Bytes |

## Bekannte Obergrenze (ehrlich benannt)

Die Zeilengrammatik kennt Operationen nur je Zeile, nicht je Element. Deshalb lässt die AllowedRange-Zeile jede Änderung innerhalb von `attributes` zu, und E007 jede Änderung innerhalb von `direction-type`, obwohl die Owner nur clef-only-Hüllen löschen bzw. Re-Cue-Hüllen entfernen. Die engere Grenze erzwingt heute der versiegelte Owner selbst (`is_safe_lower_staff_clef_attributes_for_removal`, r605-Projektion). Eine Grammatik mit Operationen je Element wäre eine Gate-Regeländerung (K2) und ist nicht Teil dieses Takes.

## Minimale Witnesses (proof_rule, `src/ssc_evidence_proof.rs`)

- Gemeinsames Präfix und Suffix werden auf Zeichengrenzen abgeschnitten. Reine Einfügungen und Löschungen werden auf den nächsten Tag-Anfang ausgerichtet.
- Exact/Edit: Ein undeklarierter Edit nennt den ersten geänderten Teilbereich in Pre- und Post-Dokumentkoordinaten statt des ganzen Dokuments.
- Kanonisch leere Fälle: Ist die kanonische Form gleich, aber die Whole-Element-Folge verletzt, zeigt der Witness auf Basis `whole_elements` das erste geänderte, eingefügte oder entfernte ganze Element. Das betrifft die vier AllowedRange-Fälle in Etüde 6–9.
- Locality-, Projektions- und Insertions-Gegenbeispiele sind ebenfalls minimal.
- Berichtszeile: `V4DeniedWitness: descriptor=…; reason=…; basis=document|canonical|whole_elements|owner_projection; pre_range=…; post_range=…; pre_bound_bytes=…; post_bound_bytes=…; pre_excerpt="…"; post_excerpt="…"` (Auszug höchstens 120 Bytes).

Kein Verdikt hängt vom Witness ab; die Witness-Änderung verschiebt keine Zahl.

## Tests

- `governed_4b_witness_tests` (Beweis): Für jeden der sieben Mechanismen mit eingefrorenen r1283- und r1284-Zeilen (keine Live-Tabelle): alte Zeile Denied mit benanntem minimalem Witness, neue Zeile Contained, Nachbaränderung (Pitch, Dauer, Staff) weiterhin Escaped. Dazu Zeichengrenzen, Einfüge-Ausrichtung, Auszugskürzung und die exakte Berichtszeile.
- Echte Owner → Receipt → Adoption → Bericht: NoteVisibility jetzt Permit; AllowedRange-Clef-Löschung und Slur-Löschung Permit mit `V4ContentChangingPermitted: 1`.

## Erwartung auf Windows

1438 Owner-Events, `V4OutOfScope: 0`, `V4ContainmentUnproven: 0`. Erwartet sind 0 Denied (1438 Permit), sofern Codex' lokale Ursachenproben alle 165 historischen Events abdecken. Codex hat ausdrücklich keinen Replay der 165 Events behauptet. Verbleibende Denied sind ein echter Befund und tragen jetzt einen minimalen Witness mit Auszug.

20/20 XML bytegleich zu r1283, keine Owner-, Writer-, Mint- oder Capture-Änderung.
