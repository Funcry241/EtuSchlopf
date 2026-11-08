# r1674 – Cursor-Schreibrecht beim gebundenen ursprünglichen Owner

ChangeClass: MusicMutation. Parent ist der bereitgestellte r1673 / 0.10.1839;
dessen grüne native Annahme bleibt Voraussetzung der Anwendung. Der letzte hier
vorliegende native Annahmebeleg gehört zu r1671.

## Gefundene Rückkopplung

Die Rest-/Artikulationsstufe rief nach jedem übernommenen
`FragmentedRestChainTransactionalWrite` E057 erneut nur mit dem aktuellen XML auf.
Der XML-only-Eingang rekonstruiert eine lokale Cursor-/Balkenkonstellation und
kann sie als `CursorLaneRepair` sofort in den Render-State schreiben. Der Erfolg
einer Pausenkonsolidierung war damit der Auslöser für einen zweiten, auch in
unabhängigen Takten arbeitenden Cursor-Writer. Er übergab keinen ursprünglichen
Verbindungskurs und schrieb die kanonische Occurrence-Planbindung nicht fort.

Die spätere PDF-Vorbereitung verwendet E057 bereits mit einem anderen Vertrag:
Der bestehende `CanonicalFinalChordStructuralPlan` bindet den aktuellen
Occurrence-Kurs, den eingefrorenen Stimmplan und die gebundenen Auslassungen.
E057 erhält genau diesen unveränderlichen Exchange-Guard. Der Owner transportiert
sein Ergebnis mit der originalen E057-Quittung und unabhängig nachgespielten Edits
in den fortgesetzten Occurrence-Kurs, bevor der vorhandene finale Sink es
übernehmen darf. Unbekannter, leerer oder veralteter Kontext erlaubt dort keinen
Rückfall auf die XML-Heuristik.

Das sind zwei produktive Entscheidungswege für dieselbe Cursor-Darstellung.
Die ursprüngliche Owner-Verbindung gehört zum zweiten, bereits vorhandenen Weg.
Maßgeblich bleiben die unveränderten bestehenden Verträge
`selector_derivation_contract_rest_bridge_consumer_order.md` und
`selector_derivation_contract_rest_bridge_cursor_exchange_repair.md`.

## Änderung

Der frühe Aufruf wird von der XML-only-Anfrage auf den bestehenden typisierten
E057-Eingang umgestellt. Er übergibt den aktuellen kanonischen Plan statt nur
XML. Die Stufe erzeugt dabei keine Cursor-Freigabe und rekonstruiert keine
Verbindungen. Ohne vorbereiteten, exakt aktuellen Exchange-Kontext liefert
E057 die Eingabe unverändert ohne Quittung zurück. Veralteter Kontext erlaubt
keinen Rückfall auf die historische XML-Suche. Der vorhandene bedingte Checkpoint
kann ausschließlich ein vom gebundenen Owner geliefertes Ergebnis übernehmen.

Die bestehende spätere PDF-Vorbereitung bindet den aktuellen Occurrence-Kurs
weiterhin frisch und verwendet denselben gebundenen E057-Owner. Seine originale
Quittung, unabhängige Edit-Prüfung, Occurrence-Fortsetzung und finale Übernahme
bleiben unverändert. Es gibt keinen neuen Writer, keinen Ersatz-Repair und
keine neue Auswahlschleife. Beide produktiven Aufrufe verwenden denselben
Owner-Eingang; kein produktiver Aufruf benutzt mehr den Legacy-XML-Eingang.
Die unveränderten bestehenden Tests des alten Checkpoints bleiben ausführbar.

Die Änderung ist bewusst als MusicMutation deklariert. An bislang nur von der
ungebundenen XML-Reparatur veränderten Stellen kann sich die Render-Serialisierung
ändern. Es wird keine Gleichheit aller späteren Render-/PDF-Ausgaben behauptet.
Die Master-Verarbeitung und klingenden Musikentscheidungen werden nicht geändert.
Der echte Korpus bleibt Evaluation, keine Bedingung für die Schreibentscheidung.

## Synthetische Proben

Ein prozessisolierter Test kombiniert eine tatsächlich angenommene Rest-
Konsolidierung mit einer unabhängigen positiven XML-only-Cursor-Konstellation.
Der unveränderte Legacy-Owner muss an dieser Konstellation wirklich schreiben;
die leere gebundene Owner-Eingabe muss am selben XML verweigern. Anschließend
läuft die echte produktive Rest-/Artikulationsstufe mit einem souveränen
Render-State. Der Rest-Write muss im State ankommen; die unabhängige Cursor-
Reihenfolge muss bis zur späteren Stimmplanbindung unverändert bleiben.
Eine gegebenenfalls fehlende spätere kanonische Bindung darf ausschließlich an
dieser benannten Boundary verweigern. Geprüft werden Relokation und unabhängige
Verdopplung. Die Prozessisolation verhindert Kreuzwirkungen globaler Messzähler.
Dieser Prefix-Test behauptet keinen vollständigen Orchestrator-/Rendererlauf.

Eine zweite Probe erzeugt ausdrücklich synthetische ursprüngliche Verbindungen
und benutzt die echte gebundene E057-API. Sie verlangt einen positiven Write,
Originalquittung und erfolgreiche Occurrence-Fortsetzung. Fehlende Quittung muss
verweigern. Der neu gebundene Folgekurs muss idempotent sein. Umbenannte Stimmen,
versetzte und unabhängig duplizierte Eingänge behalten ihre lokale Entscheidung.
Die bestehende zusätzliche Testfamilie für reale E057-Sink-Adoption, I/O-Fehler,
falschen Readback und stale/fehlenden Kontext bleibt unverändert erhalten.

Die Quellen von E057, Guard-Bindung, Occurrence-Fortsetzung, finalem Sink und
Schreibautorisierung bleiben bytegleich zum Parent. Die lokale Haupttestsuite,
strenges Produktions-Clippy und die statische Take-Prüfung werden vor der
Auslieferung geprüft. Die native Korpusabnahme von r1674 bleibt offen.
