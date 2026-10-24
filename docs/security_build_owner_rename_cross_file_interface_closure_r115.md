# r115 — Security Build Owner Rename Cross-File Interface Closure

## Zweck

Dieser SecurityOnly-Foundation-Take schliesst ausschliesslich die vor dem Kopieren ausgefuehrte Cross-File-Interface-Pruefung fuer den bereits separat autorisierten Cargo-Build-Owner-Rename.

Der abgelehnte r114-Kandidat hat gezeigt, dass ein byteidentischer Dateiumzug von `src/identity_debt_build.rs` nach `src/security_build_orchestrator.rs` vom bestehenden Struct-Feld-Closure noch als vollstaendige Loeschung des alten Interfaces interpretiert wurde. Dadurch wurde `BaselineRow.decision` faelschlich als entfernt gemeldet, obwohl dieselbe Build-Script-Datei byteidentisch unter dem neuen Cargo-Build-Pfad wieder vorhanden war.

## Geschlossener Vertrag

Die Cross-File-Interface-Pruefung behandelt den alten Build-Owner nur dann als identity-preserving Rename und nicht als Interface-Loeschung, wenn gleichzeitig alle Bedingungen gelten:

- `security_build_owner_rename=closed_exact_cargo_build_owner_rename_v1` ist aktiv.
- `Cargo.toml` enthaelt exakt einen bereits separat validierten `package.build`-Pfadwechsel innerhalb desselben `src`-Parent-Verzeichnisses.
- Der reale Diff enthaelt den alten Build-Owner exakt als `Deleted` und den neuen exakt als `Added`.
- Die Bytes des alten und neuen Build-Owners sind exakt identisch.

Fehlt auch nur eine Bedingung, bleibt die bisherige Cross-File-Interface-Pruefung unveraendert aktiv. Normale Struct-Feld-Entfernungen und nicht-byteidentische Renames erhalten keinen Bypass.

## Regressionstest

Ein neuer Unit-Test reproduziert die r114-Situation mit `BaselineRow.decision`: eine unveraenderte Consumer-Datei referenziert das Feld weiter, waehrend der Build-Owner byteidentisch umbenannt wird. Dieser exakte Rename muss akzeptiert werden; die bestehenden Tests fuer echte Feldentfernung bleiben unveraendert bestehen.

## Nicht enthalten

Dieser Take benennt den Build-Owner noch nicht um und aendert keine Musik-, Writer-, Renderer-, Pipeline-, Report-, Console-, Recovery- oder Kalibrierungssemantik.
