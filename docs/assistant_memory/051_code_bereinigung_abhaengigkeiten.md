# Code-Bereinigung: Abhaengigkeiten und bewusst verbleibende Reste

Stand: angenommener r1454-Quellstand, Kandidat r1455. Advisory only.

## Ergebnis

Nur die fest ausgegebenen alten #56-Reparaturwerte im Report werden entfernt.
Der vorhandene Adapter `cursor_lane_temporal_legacy_audit` gibt weiter den
vollstaendigen aktuellen `CursorLaneTemporalCheckAudit` aus; vor dessen
unveraendertem Display steht nun `retired` statt erfundener Reparaturzaehler.
Der bisherige Zeilenpraefix bleibt fuer den Assistant-Export erhalten.
Audit-Struktur, Datenfluesse, Zeitpruefung, Fail-Closed-Entscheidung, Writer,
Material und saemtliche Gates bleiben unveraendert.

## Abhaengigkeitspruefung

| Kandidat | Konkrete Bindung | Konsequenz |
| --- | --- | --- |
| r1441-Borrow-Governance | `ssc_k2_retirement_governance.rs`: `CODE` bindet die Governance und Tests als Parent-Bytes; `validate_transition` verlangt fuer Aenderungen eine benannte Governance-Revision. | Keine Entfernung im Bereinigungs-Take. |
| Alter Borrow-Nachfolger | `R57_CURSOR_COURSE_STAGE_AFTER` wird in `validate_render57_noop_stage_transition` explizit abgewiesen. | Die Sperre gegen die unaktivierte Route bleibt erhalten. |
| Borrow-Helper und Before/After-Texte | `render57_cursor_course_stage_successor` und beide Texte sind cfg(test); drei `original_cursor_evidence_borrow_*`-Tests bauen damit negative Gegenbeispiele. | Nicht verwaist; keine Loeschung von Helper oder Tests. |
| Historische Hashes des Report-Shards | `cursor_temporal_retirement_kodex.rs`: `SUCCESSORS`/`PARENTS` binden die erste Retirement-Transition. `validate_transition` liest die gesamte SUCCESSORS-Liste nur bei `!before && after`; der aktuelle Parent hat den Checker bereits installiert. | Historische Hashes unveraendert erhalten; keine Aufweitung fuer den heutigen Reporttext. |
| Aktive Zeitpruefungs-Pins | `validate_tree` bindet Checker, Replay, Initial-Stage, Lineage und verbietet alte Writer-Symbole; der Report-Shard wird dort nicht vollstaendig gepinnt. | Alle aktiven Pins bytegleich, Reporttext separat aenderbar. |
| Alter Report-Praefix | `assistant_export.rs::write_assistant_project_result_summary` uebernimmt Zeilen beginnend mit `CursorLaneReanchorRestVisibilityContract `. | Praefix erhalten; keine neue Export-/Handoff-Logik. |
| Audit-Typ und Flags | Initial-Stage verwendet `CursorLaneEndInvariantRepairAudit`, Adapter und Merge; Metric-Stage liest `reanchor_contract_active`. | Diese Code-Reste bleiben, damit keine gehashte Stage oder produktive Entscheidung angefasst wird. |
| Alte Nullwerte | `repaired 0`, `crossingRestPostcondition 0` usw. sind ein einzelnes Format-Literal, keine berechneten Auditfelder. Kein anderer produktiver Leser interpretiert diese Teilfelder. | Nur diese veraltete Darstellung entfaellt; aktuelle Messwerte/Hashes bleiben vollstaendig. |

## Klassen- und Abnahmegrenze

Die kanonische Pfadmatrix behandelt den gemischten Report-Shard als OtherSource,
nicht als dedizierten Audit-Owner. AuditOnly erlaubt dort nur Loeschungen oder
strikte use-Verkleinerungen, keinen neuen Formattext. Der geplante kleine
Darstellungsumbau ist deshalb RefactorOnly: unveraenderte fachliche Berechnung,
Materialisierung und musikalische Entscheidungen, allein bereinigter Bericht.
Es wird kein Owner verschoben, kein Pfad umbenannt und keine Pfadrolle angepasst.

Alle 20 finalen XMLs muessen nativ gegen r1454 bytegleich sein, 10/10 PDFs mit
`-o` ohne `-f`, Kanalprofile und NotatedChannelOverlap unveraendert. Im Result
entfallen allein die alten festen Reparaturangaben; der aktuelle Zeitbeleg
einschliesslich Input-SHA und Zaehlern bleibt erhalten. Dies ist kein Abschluss
der verbleibenden Governance-/Carrier-Bereinigung und keine Wiederaufnahme von Custody.
