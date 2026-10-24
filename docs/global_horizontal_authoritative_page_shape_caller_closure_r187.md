# GlobalHorizontalAuthoritativePageShapeCallerClosure — v0.10.1839 r187

## Anlass

Der r186-Versuch wurde vor Installation beim Strict-Clippy-Kompilieren fail-closed abgewiesen.
Die Typisierung von `authoritative_page_shape()` selbst war konsistent, aber ein zweiter Aufrufer
in `horizontal_layout_page_count_preference.rs` destrukturierte weiterhin den historischen
7er-Tupel-Rückgabewert. Rust meldete deshalb E0308: erwartet wurde
`GlobalHorizontalAuthoritativePageShape`, gefunden wurde ein Tupelmuster.

r187 basiert deshalb wieder auf dem autoritativ grünen r185-Stand und enthält sowohl den
beabsichtigten Typ-Schnitt als auch die vollständige Aufrufer-Closure.

## Änderung

Der private Domänentyp `GlobalHorizontalAuthoritativePageShape` bündelt weiterhin unverändert:

- autoritative Page-Starts;
- autoritative Systemlängen je Seite;
- minimale/maximale Slots je Seite;
- minimale/maximale Systeme je Seite;
- maximale autoritative Systemlänge.

`evaluate_global_horizontal_page_count_preference()` benötigt davon nur zwei Informationen und
liest diese nun explizit:

- `authoritative_shape.page_starts.len()`;
- `authoritative_shape.max_systems_per_page`.

Es wird bewusst kein Tupeladapter und keine Übergangs-API eingeführt.

## Aufrufer-Closure

Eine vollständige Quellsuche nach `authoritative_page_shape(` ergibt nach der Änderung genau zwei
produktive Aufrufer. Beide arbeiten mit dem neuen Domänentyp. Es existiert kein verbleibendes
Tupelmuster an einem Aufruf dieser Funktion.

Damit ist genau die Compile-Lücke geschlossen, die r186 offengelegt hat.

## Geschützter Kalibrierungsvertrag

Der komplette Legacy-Kalibrierungsrumpf ab

`let allowed_min_page_slots = authoritative_min_page_slots.saturating_sub(1).max(1);`

bis zum Funktionsende ist gegenüber r185 byte-identisch. Unverändert bleiben insbesondere:

- `allowed_min_page_slots` und `allowed_max_page_slots`;
- `min_end..=max_end`;
- `accepted_system_capacity`;
- minimale/maximale Systeme pro Seite;
- maximale autoritative Systemlänge;
- rein beobachtende Legacy-Semantik.

## Erwarteter Clippy-Effekt

Vom grünen r185-Ausgangsstand:

- Normal-Clippy gesamt: `105 -> 103`;
- `architectural_budget`: `7 -> 5`;
- `small_mechanical_budget`: unverändert `98`.

## Unverändert

- PageCount-Preference-Entscheidung;
- Global-Reflow- und Page/System-Auswahl;
- Measure-Demand und Capacity;
- sämtliche Layout-Schwellen und Gewichtungen;
- MusicXML-/Render-/PDF-Semantik;
- Writer-/Structural-Write-Authority;
- Security-Gates;
- Release-/Exit-Semantik.
