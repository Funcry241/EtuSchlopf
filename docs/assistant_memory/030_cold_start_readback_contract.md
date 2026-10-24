# Architecture Essence – Cold-Start Readback Contract

## Zweck

Die repository-residente Essenz ist nur dann als Cold-Start-Hilfe belastbar, wenn ihre Verweise im aktuellen Repository tatsaechlich aufloesbar bleiben.

Dieser Readback ist **kein Architektur-Gate und keine neue normative Quelle**. Er erzeugt keine MusicMutation-, Writer-, Repair-, Release-, Proof- oder sonstige produktive Autoritaet. Er beweist ausschliesslich die technische Integritaet der beratenden Cold-Start-Schicht.

## Read-only Regression

`tests/architecture_essence_cold_start_readback.rs` prueft:

- die im Bootstrap benannten primaeren Quellen existieren im aktuellen Repository,
- Bootstrap und Start-here verweisen auf dieselbe primaere Essenz,
- `authority.class = advisory_only`, `normative = false` und `productive_authority = []` bleiben explizit,
- der Regelindex loest genau E00 bis E32 gegen jeweils genau eine Ueberschrift der einen Essenz-Textquelle auf,
- die Discovery-Hinweise zeigen auf existierende aktuelle Repository-Pfade,
- die Discovery-Hinweise bleiben ausdruecklich nicht autorisierend,
- die Cold-Start-Reihenfolge fuehrt vor historischer Fallmemory zu Repository, Bootstrap, Essenz und den aktuellen kanonischen Contracts/Authorities/Gates,
- E31 und die maschinenlesbaren Vor-Candidate-/Nach-Patch-Reihenfolgen halten `ChangeClass × PathRole × ChangeKind` bereits vor Candidate-/ZIP-Bau fest und pruefen dieselbe Matrix auf dem finalen Paketdiff erneut vor Semantic-/Capability-, Call-Graph- und Build-Pruefungen. Ein Widerspruch muss als Owner- oder kanonische PathRole-Frage geklaert werden; ChangeClass-Hochstufung oder kosmetische Pfad-/Namensaenderung zum Gate-Passieren sind keine zulaessige Aufloesung.
- Der billige fail-closed Take-Tree-Check bleibt vor den teuren semantischen, Call-Graph- und Build-Pruefungen, ohne daraus neue Authority abzuleiten.
- Weiterhin wahre repository-residente Readback-Witnesses werden bei Essenz-Closures additiv bewahrt; eine staerkere Regel darf einen bestehenden Witness nicht unnoetig durch reine Umformulierung brechen.
- Die maschinenlesbare Retirement-Hilfe trennt produktive Reachability, verbleibende Capability-Oberflaeche, Destruktions-/Cleanup-Owner und unabhaengige Contract-Witness-Rollen; ein permanent wahrer Schalter darf nicht allein wegen seiner toten Steuerrolle als kompletter Contract-Rest behandelt werden.
- Die Identity-Hilfe bindet den Merksatz `provenance_carries_known_identity_readback_proves_persisted_state`: Readback validiert Zustand/Repraesentation, mintet aber keine bereits bekannte Identity neu.
- E09/E15/E24 halten Proxy-Identitaet ausdruecklich fest: syntaktische Identity-Freiheit reicht nicht; eine Fehlerklasse wird witness-blind aus kausalen Merkmalen definiert und **ohne neuen selector-sensitiven Code in einem akzeptierten Parent eingefroren**. Erst ein spaeterer Take darf diese unveraenderte Klasse als Klassifikationscode abbilden oder daraus einen Repair-Scope ableiten. Korpusverteilung dient Messung/Falsifikation, Einzigartigkeit ist Ergebnis statt Konstruktionsziel, Repair-Narrowing bleibt auf unabhaengige Authority-/Safety-Grenzen beschraenkt.
- Source-Snapshot und eingehender Take-Baum bleiben getrennte Artefakte; der Managed-Tree-Vertrag entscheidet ueber Take-Kompatibilitaet, und PhysicalStart wird aus aktuellem Repository-/Gate-Befund statt aus Uebergabe-Erinnerung abgeleitet.
- AIC-1 unter `docs/assistant_memory/architecture_inventory_contract_v1.json` bleibt im Bootstrap aufloesbar, traegt `contract_version = AIC-1`, bleibt `advisory_only`/nicht normativ/ohne produktive Authority und definiert die assistant-seitige Rust-AST-Symbolgrundgesamtheit mit genau einer Inventurzeile pro qualifiziertem Symbol. Der Readback prueft nur diese Form und Autoritaetsgrenze, nicht die spaetere A/B/C/N/OFFEN-Interpretation einzelner Funktionen.
- AIF-1 unter `docs/assistant_memory/architecture_inventory_assistant_fast_path_v1.json` bleibt direkt nach AIC-1 aufloesbar, advisory-only/nicht normativ/ohne produktive Authority, bindet mechanische Cache-Wiederverwendung ausschliesslich an `source_snapshot_sha256`, verbietet Interpretation als Cache-Wahrheit und haelt den Ein-Pass-Source-Index plus bevorzugt genau einen finalen Workbook-Export fest.

## Bewusste Grenze

Der Test prueft **nicht**, ob eine Essenz-Regel fachlich oder normativ korrekt ist. Diese Wahrheit gehoert weiterhin ausschliesslich den aktuellen kanonischen Contracts, Authorities und Gates.

Der Test prueft ebenfalls **nicht**, ob ein Discovery-Pfad dauerhaft Owner einer Capability bleibt. Er beweist nur, dass ein als Suchhinweis deklarierter Pfad im gegenwaertigen Repository aufloesbar ist. Die fachliche Ownership muss fuer jede konkrete Aufgabe erneut aus dem aktuellen Source-Baum bestimmt werden.

Der Test prueft ebenso **nicht**, ob eine spaetere KI ein Symbol korrekt als A/B/C/N/OFFEN einordnet, ein Architekturthema richtig deutet oder einen naechsten Schritt richtig waehlt. AIC-1 konserviert Grundgesamtheit, Semantik und Darstellungsvertrag; Bedeutung bleibt KI-Interpretation und erzeugt keine Produktentscheidung.

Damit werden E30, E31 und E32 gegen technischen Verfall abgesichert, ohne E28 zu verletzen.


**r1031 Proxy-Identity-Sperre:** Eine neue kausale Fehlerklasse wird zuerst ohne neuen selector-sensitiven Code als `error_class_definition` eingefroren. Klassifikations- oder Repair-Code darf erst in einem spaeteren Take auf diesen unveraenderten Vertrag aus dem akzeptierten Parent verweisen.
