# r228 - Exact Rust Owner Relocation Contract Foundation

## Anlass

Nach dem real gruenen r227 ist die logische Modulidentitaet bereits `midi_seed`, waehrend der physische Owner fuer einen Uebergang weiterhin `src/flp.rs` heisst. Ein direkter Delete+Add-Rename des Owners wird von zwei bestehenden Schutzschichten absichtlich misstrauisch behandelt:

1. `Take-CrossFile-Interface-Closure` interpretiert einen geloeschten Struct-Owner zunaechst als moegliche Interface-Entfernung.
2. `NO-NEW-REPAIR-AUTHORITY SECURITY GATE` inventarisiert writer-erreichbare Funktionen pfadgebunden und wuerde dieselbe byteidentische Funktion unter einem neuen Pfad als neue Autoritaet sehen.

Beide Reaktionen sind fuer unbekannte oder semantisch veraenderte Moves richtig. Fuer einen nachweislich byteidentischen Refactor-Owner-Rename braucht es jedoch eine geschlossene Identitaetsregel.

## Neuer enger Vertrag

Die Gates erkennen eine identity-preserving Rust-Owner-Relocation nur dann automatisch, wenn **alle** Bedingungen gleichzeitig erfuellt sind:

- `change_class=RefactorOnly`;
- genau ein geaenderter nicht-sicherheitskritischer `src/**/*.rs`-Pfad existiert nur im Altbaum;
- genau ein solcher Pfad existiert nur im Neubaum;
- beide Pfade liegen im selben Parent-Verzeichnis;
- beide Pfade gehoeren derselben semantischen Domain an;
- die Dateibytes sind exakt identisch;
- der Altbaum besitzt genau eine explizite `#[path = "<altname>"] mod <module>;`-Bindung;
- der Neubaum besitzt genau eine explizite `#[path = "<neuname>"] mod <module>;`-Bindung;
- Binding-Owner und Modulname sind vor und nach dem Rename identisch;
- alte Bindung ist im Neubaum weg und neue Bindung im Altbaum nicht vorhanden;
- der Binding-Owner ist selbst Teil des realen Diffs.

Fehlt eine Bedingung, gibt es keinen Relocation-Bonus: die bisherigen fail-closed Regeln greifen unveraendert.

## Cross-File-Closure

Nur der exakt verifizierte alte Owner darf bei der Struct-Feld-Entfernungspruefung als identity-preserving Rename uebersprungen werden. Andere geloeschte oder veraenderte Owner bleiben voll pruefpflichtig.

## Semantic-Capability-Closure

Das No-New-Repair-Authority-Gate akzeptiert unter dem exakten Relocation-Beweis ausschliesslich einen eingehenden `WriterReachability`-Record als bereits vorhanden, wenn am alten Pfad ein Record mit identischem Kind, Domain und Symbol existiert und sich dessen Detail **ausschliesslich** durch die bereits bewiesene Altpfad->Neupfad-Relocation unterscheidet. Der eingehende Fingerprint muss exakt zum so pfadsubstituierten Detail passen. Andere Detail-, Body- oder Capability-Aenderungen erhalten keinen Relocation-Bonus. Es wird keine neue Writerklasse, kein neuer Repair-Pfad und keine allgemeine Refactor-Ausnahme zugelassen.

## Regressionen

Neue Tests decken ab:

- byteidentischer Refactor-Owner-Rename mit unveraendert genutztem Struct-Feld wird von der Cross-File-Closure akzeptiert;
- nicht-byteidentischer Rename bleibt abgelehnt;
- byteidentische Verlagerung einer writer-erreichbaren `repair`-benannten Funktion wird nicht als neue Autoritaet fehlklassifiziert;
- eine Koerperaenderung beim vermeintlichen Rename bleibt durch das No-New-Repair-Authority-Gate blockiert.

## Nicht enthalten

- kein `src/flp.rs -> src/midi_seed.rs` in diesem Take;
- keine Modul-, Funktions-, Typ- oder Feldumbenennung im Produktcode;
- keine Musikmutation;
- keine Aenderung an Dependencies oder Root-`Cargo.lock`;
- keine Lockerung fuer SecurityOnly-, AuditOnly-, MusicMutation-, CalibrationOnly- oder MechanicalOnly-Takes.

Der eigentliche physische Owner-Rename ist erst der Folgetake auf einem real gruenen r228.
