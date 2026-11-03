# r1591: eng begrenzte kanonische Eingabe für E057

Parent: nativ angenommener r1590. Klasse: KodexOnly.
Der technische Take-Name lautet nach dem bestehenden Governance-Vertrag
`SscK2GovernanceRevisionKodex`.

## Zweck und genaue Grenze

Die angenommene Fehlerklasse in
`selector_derivation_contract_rest_bridge_consumer_order.md` bleibt unverändert.
Für die spätere Reparatur muss der bereits vorhandene Note–Forward-Owner E057
einen tatsächlich kanonisch gebundenen Austauschplan lesen können. Seine bisherige
Schnittstelle erhält ausschließlich XML. Die allgemeine Receipt-Migration
erlaubt keine zusätzlichen Eingaben. Die vorhandene E050-Ausnahme gehört einem
anderen, ausschließlich für Balken zuständigen Owner.

Diese ausdrücklich begrenzte Erweiterung der Schnittstellenzulassung gilt nur
für E057, dessen genauen bisherigen Owner-Pfad und Basissignatur sowie die
unveränderte Klassifikation PRESERVING. Genau eine weitere Signatur wird erlaubt:

```rust
fn human_bound_cursor_lane_owner_transfer_xml(
    xml: &str,
    exchange_guard: Option<&structural_write_authority::CanonicalFinalChordStructuralPlan>,
) -> (String, PartialBeatLaneSerializationOwnerTransferStats)
```

XML-Eingabe und Rückgabe bleiben unverändert. Der vorhandene Receipt-Wrapper
behält die ursprüngliche einzelne E057-PRESERVING-Quittung; diese Regel führt
keinen weiteren Issuer, Companion oder Receipt-Typ ein. Der vorhandene exportierte
kanonische Plan dient als unveränderlicher Träger eines später privat gebundenen
Plans. Die Schnittstellenzulassung konstruiert diesen Plan nicht.

Nur eine tatsächliche Definition darf die neue Signatur verwenden. Auch eine
Mischung aus alter und neuer Definition ist abgelehnt. Andere Owner, IDs,
Klassifikationen oder Basissignaturen, veränderliche/eigene/erased/zusätzliche
Kontexte, Callbacks, geänderte XML-Eingaben, Generics, andere Rückgaben und weitere
Quittungen erhalten keine Zulassung. Die neue Signatur wird auf dasselbe bestehende
physische Surface-Budget abgebildet; zusätzliche Instanzen, XML-Mutationen oder
Dateischreibvorgänge bleiben verboten.

Die allgemeine Receipt-Migration sowie beide bestehenden E050-Formen bleiben
unverändert. Population, Ausschlüsse, Zweckgrenzen, Quittungssemantik und
ursprüngliche Mint-Zuordnung bleiben bytegleich. Der bestehende Trust-Anchor
prüft die endgültigen Gate-Bytes weiter mit einem exakten aktualisierten Digest;
kein Pin entfällt und keine generische Freigabe entsteht.

## Was dieser Take tatsächlich verändert

Nur die Gate-Signaturprüfung, deren Zuordnung auf das bisherige Surface-Budget,
zugehörige Gate-Tests, der exakte Trust-Digest und Take-Metadaten ändern sich.
Sämtlicher produktiver Musik-/Rendercode bleibt bytegleich mit r1590. Die
zusätzliche Signatur wird noch nicht produktiv benutzt. Notenbild und musikalische
Ausgaben sollen deshalb vollständig unverändert bleiben.

Sieben neue synthetische Gate-Tests prüfen die genaue positive Form, fremde
Owner/IDs/Klassen/Baselines, unerlaubte Kontexte, geänderte Ausgaben, tatsächliche
Owner-Zulassung einschließlich gemischter/mehrfacher Definitionen, unveränderte
physische Budgets und Fortbestand der alten Receipt-/E050-Regeln. Sie sind keine
Tests der späteren musikalischen Reparatur.

## Verbindliche Grenze der späteren Umsetzung

`None` soll ausschließlich den bisherigen frühen E057-Pfad unverändert ausführen.
`Some(guard)` soll nur den vorab gebundenen neuen Austauschplan verwenden. Ein
fehlgeschlagener oder veralteter Guard darf keinen Rückfall auf den anderen
Selektor auslösen. Diese Verhaltensanforderungen sind hier dokumentiert, noch
nicht implementiert oder durch die Signaturprüfung allein bewiesen.

Die spätere kanonische Bindung muss die tatsächlich übernommenen Verbindungen,
ihre echten Source-/Attack-/Fragment-Occurrences, den endgültigen Stimmenplan,
vollständige Akkordressourcen, bestehende Pausenabstammung und die vorhandenen
Cursor-Platzhalter prüfen. Labels und XML-Balkentags allein sind keine Autorität.
Unknown bleibt Unknown; keine neue Pausenrolle wird aus der Serialisierung erfunden.

Der echte E057-Owner muss die tatsächliche Umsortierung ausführen. Ein unabhängiger
Prüfer muss die ursprüngliche E057-Quittung, exakte Vor-/Nachbytes, unveränderte
musikalische Zeitlinie und den vollständigen Positionswechsel aus der gebundenen
Operation prüfen. Die IDs stammen aus dem alten Beleg; gleiche Tonhöhen oder
Einsatzzeiten dürfen keine Identität rekonstruieren. Bytebereiche, alle Notenindizes
und reine Tonindizes sind verschieden. Akkordanbindungen, Balkenkurse und
Phrasenenden müssen vollständig fortgesetzt werden. Der gewöhnliche
reihenfolgeerhaltende Transport bleibt streng.

Nur erfolgreiche Übernahme am bestehenden finalen Render-Sink aktiviert die
fortgesetzten Belege. Shadow-, verworfene oder abgelehnte Kandidaten dürfen Main
nicht speisen. Unabhängige Konfliktgruppen benötigen eigene Zulassungsentscheidungen;
globale Einzigkeit oder Rangfolge sind keine Schreibautorität. Kein neuer Writer,
Dateifilter, globaler Modus oder XML-Marker ersetzt diese Bindung.

## Native Annahme

```text
run.cmd --take --all
```

Erwartet: exakte Annahme dieses r1591-Pakets, grüne strikte Prüfungen und alle
20 XML-Dateien sowie alle 39 PDF-Seiten unverändert gegenüber r1590. Erst der
angenommene Parent kann die neue Eingabeform eines späteren Reparatur-Takes zulassen.
