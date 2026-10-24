# r1283 / Take 4a – Analyse der 165 Denied und Bauvorlage für Opus

Parent: accepted r1282 `SscEditBlockContainmentV4`, Candidate-SHA `c6d0dd699872bdd2fa96015f3d0b19296282350f509f3a7150e1de566b1106a2`. Source-Snapshot-SHA `7a2126df49a98e24eb2ff1b2d632f3a895ddfcdc66586332e3132c56ece02e86`.

## Gemessener Stand und Beleggrenze

Export vom 07.10.2026, 20:58:58 UTC: **1438 = 1273 V4Permitted + 165 V4ContainmentDenied; V4OutOfScope 0; V4ContainmentUnproven 0.** Alle zehn Projektberichte haben V4Balance complete. Die 20 finalen XML sind bytegleich zum r1281-Export. r1282 ist angenommen.

Alle 165 Witness-Zeilen wurden einzeln inventarisiert (Projekt, Zeile, Descriptor und vollständige Range-Angaben). Der Export enthält finale Artefakte, aber keine historischen Pre/Post-Dokumente oder ursprünglichen Edit-Skripte jedes Events. Deshalb ist die Ereigniszahl gemessen; die Zuordnung der Änderungsmechanismen ist eine Schlussfolgerung aus versiegeltem Owner-Code, den Witness-Formen und lokalen Durchstichen. Keine Probe wird als Replay aller 165 historischen Events ausgegeben. Exakte damalige Byte-Positionen, Restkoordinaten, Forward-Dauern und die Wahl slur versus tied der vier Notations-Fälle lassen sich aus diesem Export nicht wiederherstellen.

Die nicht produktive lokale Diagnose benutzt den echten r1282-Receipt-Kern und den unveränderten Containment-Checker. NoteVisibility und AllowedRangeRemoval laufen über die echten Owner und ihre Original-Receipt-Skripte. Rest-, Key- und Re-Cue-Helfer werden unverändert aus dem eingefrorenen Code kompiliert; das MissingStaffEdge-Payload folgt dem eingefrorenen Konstruktor. Jede Ursachenprobe ergibt unter der r1282-Zeile Denied und unter der unten vorgeschlagenen Zeile Contained. Bei Exact wird zusätzlich die Eingrenzung des Edit-Skripts geprüft. Die Tabellenänderungen existieren ausschließlich im Diagnose-Harness; die produktive Tabelle bleibt unverändert.

## A5-Tabelle

| Descriptor | Anzahl | Tatsächlicher bzw. aus dem eingefrorenen Writer belegter Änderungsmechanismus | Code-Stelle im r1282-Parent | A5 | Vorschlag |
|---|---:|---|---|:---:|---|
| NOTE_VISIBILITY_EXACT | 114 | Auf `<note>` `print-object="no"` und fehlendes `print-spacing="no"` einsetzen; alle gemessenen Raw-Deltas +37 Bytes. Keine Note-Insertion. | `remaining_layout_writer_family.rs:231–259`, Helper `rewrite_note_opening_tag_hidden:301–330` | a | bestehende visibility-Klasse genügt; Zeile um `note@print-spacing` erweitern |
| ALLOWED_RANGE_REMOVAL_EXACT – Clef-Zweig | 20 | komplette sichere `<attributes><clef number="2"><sign>F/G</sign>…</clef></attributes>` löschen; kein divisions/key/time/staves/transpose. Kanonische Form schrumpft. | `remaining_layout_writer_family.rs:372–475`, Safe-Guard `488–500`; Caller `notation_endpoint_layout_core.rs:348–414,416–495` | b | Descriptor zusätzlich der bestehenden Klasse `clef` zuordnen; Envelope in 4b um `attributes` erweitern |
| ALLOWED_RANGE_REMOVAL_EXACT – Notations-Zweig | 4 | `slur` oder notation-level `tied` aus einer `notations`-Hülle löschen; die Hülle ist danach geändert statt als Ganzes entfernt. Nicht sounding `<tie>` aus dem Endpoint-Caller. Kanonische Form vor/nach identisch, Whole-Element-Liste verschieden. | `key_signature_enharmonic_report_core.rs:2426–2500`; `notation_endpoint_layout_core.rs:30–58`; erlaubter Lösch-Owner wie oben | a | `change,remove` für die unverändert erlaubten Notations-Elemente. Native slur-Probe reproduziert den kanonisch leeren Witness. Welcher der beiden Tags pro historischem Event betroffen war, bleibt offen. |
| RENDER_ROW_30_EXACT | 10 | Display-Kinder hinzufügen/ersetzen; `<rest/>` in `<rest>…</rest>` überführen; gegebenenfalls `rest@measure="yes"` entfernen. Die Felderzeile deckt die Rest-Tag-Geometrie nicht ab. Gesamtwachstum ist viele Pausen im Batch, kein Nachweis neuer Noten. | `rest_voice_family_authority.rs:619–638`; Caller `staff_voice_rest_glyph_topology_tail.rs:1192–1269`; Helper `thirty_surface_productive_authority.rs:968–1015` | a | Rest als Ganzes mit change; vorhandene rest_rhythm/visibility/layout-Klassen reichen aus |
| RENDER_ROW_51_EXACT | 9 | Cursor-erhaltende Insertion: backup, gegebenenfalls leading forward, neuer Rest-Note-Block, gegebenenfalls tail forward. Die beiden Forward-Zweige fehlen in der Zeile. | `strict_staff_visible_lane_authority.rs:439–469`; Payload `strict_staff_visible_lane_rest_rewrite_tail.rs:665–691` | a | insert-only beibehalten; `forward` ergänzen, rest_rhythm erlaubt es bereits |
| FINAL_LAYOUT_PHRASE_PRESERVING (E007) | 6 | bei finalem Layout-Neustart generierten 8va/15ma/8vb/15mb-Words-Re-Cue samt seinem `<direction-type>`-Wrapper entfernen; octave-shift-Continue, Linien-ID und Staff bleiben. Gemessene kanonische Deltas −73/−219 entsprechen einer/drei Cue-Hüllen. | `horizontal_layout_plan_materialization.rs:253–312`, E007-Transport `final_notation_phrase_projection_transaction.rs:92–241`; ausdrücklicher Zweck `docs/registernotation_silent_bridge_final_layout_recue_projection_closure_r605.md:70–113` | b | neue enge Klasse `ottava_recue`: direction-type/words, remove, content=false; nur E007 zuordnen |
| FUNCTIONAL_RESPELLING_EXACT | 1 | Etüde 3: lokale Tonart aus gebundenem Render spiegeln: key/fifths und neue attributes-Hülle an der gebundenen Grenze, dazu pitch/accidental-Respelling. Der Owner verlangt genau den Render-Key-Vektor und unveränderte MIDI/Onset/Duration/Staff/Voice. | `key_signature_enharmonic_report_core.rs:927–1075`, lokaler Zweig `1002–1032`, Key-Helfer `2195–2242` | b | pitch_spelling deckt key, aber nicht die neue Hülle; zusätzliche `tonal_key_container`-Klasse nur diesem Owner und #66 zuordnen |
| RENDER_ROW_66_BLOCK | 1 | Etüde 3: lokales key/fifths samt neuer attributes-Hülle, pitch/accidental-Respelling; der alte Checker blendet key aus, lässt aber den leeren neuen attributes-Wrapper stehen. Gemessen +25 kanonische Bytes; native Probe reproduziert exakt +25. | `key_signature_enharmonic_report_core.rs:142–223`, Helper `1111–1163` | b | zusätzliche `tonal_key_container`-Klasse; Envelope um attributes erweitern |

**Summe: 137 (a), 28 (b), 0 belegte (c).** Die Einordnungen sind jeweils aus dem Owner-Zweck begründet, nicht daraus, dass ein Korpusfall vorkommt. Die vier Notations-Fälle werden ausdrücklich nicht als Clef-Fälle mitgezählt.

## Korrekturen in 4a

Nur vier Descriptor-Zuordnungen werden erweitert:

- ALLOWED_RANGE_REMOVAL_EXACT: notation_mark/rest_rhythm **plus clef**.
- FINAL_LAYOUT_PHRASE_PRESERVING: notation_mark/layout **plus ottava_recue**.
- FUNCTIONAL_RESPELLING_EXACT und RENDER_ROW_66_BLOCK: pitch_spelling **plus tonal_key_container**.

Neue Klassen sind eng auf die fehlenden Container beschränkt:

```text
ottava_recue: elements=direction-type,words; hosts=-; attrs=-; operations=remove; content=false
tonal_key_container: elements=attributes; hosts=-; attrs=-; operations=insert; content=false
```

Die Tonart-/Pitch-Inhalte bleiben in der bereits content-fähigen Klasse pitch_spelling. Die neue Container-Klasse bekommt keine zusätzliche Sounding-, Timing-, Clef- oder Staff-Authority. Der Parent-Tabellengrammatik zufolge ist eine Envelope-Zeile eine gemeinsame Obergrenze; sie kann keine bedingte XML-Payload-Grammatik ausdrücken. Belegt ist hier ausschließlich die eingefrorene key-only Container-Insertion der beiden versiegelten Writer. Keine neue Regelart wird eingeführt.

Keine vorhandene Klasse oder Zuordnung wird verengt. Die produktiven Envelopes, Runtime-Typen, Beweise, Mint-, Owner- und Musikbytes werden nicht geändert. 4a allein ändert daher keine der gemessenen SSC-Zahlen.

## Konkrete Zeilen als Bauvorlage für 4b – nicht Bestandteil von 4a

```text
row=NOTE_VISIBILITY_EXACT|Exact|Edit|note|note@print-object,note@print-spacing|change|edit_script_locality|no|sealed visibility owner hides note opening tags without changing musical content
row=ALLOWED_RANGE_REMOVAL_EXACT|Exact|Edit|notations,slur,tie,tied,attributes|-|change,remove|edit_script_locality|yes|content: frozen structural removal deletes bound notation endpoints and safe lower staff clef attributes
row=RENDER_ROW_30_EXACT|Exact|Edit|rest,note,display-step,display-octave|note@default-y|change|edit_script_locality|yes|content: frozen rest display placement materializes rest children and rest tag form while preserving the note ledger
row=RENDER_ROW_51_EXACT|Exact|Edit|note,backup,forward|-|insert|edit_script_locality|yes|content: frozen missing staff edge rest payload inserts cursor backup forward and rest notes
row=FINAL_LAYOUT_PHRASE_PRESERVING|Preserving|Projection|notations,slur,print,system-layout,measure,note,direction-type,words|measure@width,note@default-x|change,insert,remove|owner_conserved_projection|no|E007 phrase horizontal layout and documented generated ottava recue carrier removal at final layout restarts
row=FUNCTIONAL_RESPELLING_EXACT|Exact|Edit|pitch,accidental,attributes,key,fifths|-|change,insert,remove|edit_script_locality|yes|content: frozen master tonal frame mirrors bound local key attributes and functional note spelling
row=RENDER_ROW_66_BLOCK|Block|Block|pitch,accidental,key,fifths,mode,cancel,attributes|-|change,insert,remove|bound_span_locality|yes|content: frozen render tonal key signature inserts key container and preserves sounding invariants during spelling
```

Admission-Hinweis für #30: `narrows` in `ssc_governed_change.rs:540–548` vergleicht Feldmengen syntaktisch. Bloßes Entfernen der beiden Rest-Felder würde trotz semantisch weiterer Rest-Hülle als `envelope_narrow` eingestuft. Die Vorlage führt deshalb die tatsächlich geschriebenen `display-step`/`display-octave`-Elemente ausdrücklich auf: neue Elemente außerhalb der alten Elementmenge, innerhalb derselben bereits vorhandenen rest_rhythm-Klasse. Dadurch ist die Deklaration `envelope_extend` eindeutig und nativ prüfbar. Keine Gate-Regel wird verändert.

Diese Zeilen reproduzieren die belegten Mechanismen in der lokalen Diagnose, sind aber kein behaupteter vollständiger 165-Event-Replay. Opus muss außerdem minimalere Witnesses herstellen und im neuen Windows-Lauf überprüfen, ob weitere bisher verdeckte Escape-Details auftauchen. Bestehende Lane-Zeilen bleiben unverändert. Keine Änderung wird in diesem Take rückwärts auf einen Zielzähler zugeschnitten.

## Getrennte (c)-Liste zur Nutzerentscheidung

Keine belegte Zwecküberschreitung in den sieben untersuchten Familien. Insbesondere ist die E007-Re-Cue-Projektion seit r605 ausdrücklich dokumentiert; sie ist keine neu erfundene Rechtfertigung aus der Korpusmessung. Es wird keine Owner-Korrektur und keine Musikänderung gebaut. Die oben genannten historischen Detailgrenzen bleiben sichtbar.

## Einzelinventar aller 165 Ereignisse

Das Prüflog-ZIP enthält `1283_denied_export_events.json` mit jeder originalen Witness-Zeile und `1283_denied_classification.json` mit genau einer A5-Kategorie je Ereignis. Die vier kanonisch leeren AllowedRange-Fälle liegen in Etüde 6, 7, 8 und 9. Projektbezeichnungen und gemessene Ranges sind ausschließlich Analyse-Daten, keine Rust-Selektoren oder Purpose-Begründungen.
