# Root Cargo.lock Role Authority Foundation – r368

r368 basiert direkt auf der real installierten und im Vollkorpus bestaetigten GreenAuthority r366 `DeadCodeDetectionIntegrityPolicyOwnerClosure`.

Der verworfene Kandidat r367 wird dabei nicht als Parent benutzt. Sein realer Windows-Pre-Copy-Gate-Befund zeigte, dass die historische Regressionstest-Identitaet `source_snapshot_reuses_take_managed_definition_and_adds_only_root_cargo_lock` selbst als `SecurityNegativeCheck` inventarisiert ist. r368 behaelt deshalb diese bestehende Security-Check-Identitaet bei und aktualisiert nur ihre Assertions auf die gemeinsame Root-Rollen-Authority; kein Retirement und keine Relocation werden verwendet.

## Befund

Root-`Cargo.lock` besitzt im realen r366 mehrere gleichzeitig gueltige Eigenschaften:

- sie ist **kein** eingehender Take-Payload und bleibt deshalb ausserhalb `MANAGED_ROOT_FILES`;
- sie ist Bestandteil des vollstaendigen Source-Snapshots;
- sie ist Bestandteil der Launcher-Build-Signatur;
- sie wird von Cargo selbst gepflegt und darf nicht manuell als Take-Payload gepatcht werden;
- der r366-Project-Catalog bezeichnete sie trotzdem faelschlich als `take_managed=yes`;
- der Assistant-Exporter besass fuer dieselbe Rolle eine zweite hart codierte Sonderliste `SOURCE_SNAPSHOT_EXTRA_REQUIRED_ROOT_FILES`.

Damit existierten zwei widerspruechliche Beschreibungen derselben Root-Dateirolle.

## Schnitt dieses Takes

r368 fuehrt in `managed_source_tree_definition.rs` eine explizite Root-Rollenwahrheit ein:

```text
TakeManaged
LocalTransactional
```

`Cargo.lock` wird exakt einmal als `LocalTransactional` deklariert. Die bestehenden acht Take-managed Root-Dateien bleiben unveraendert `TakeManaged`.

Der Assistant-Exporter konsumiert diese gemeinsame Rollenwahrheit fuer zwei Projektionen:

1. Der vollstaendige Source-Snapshot nimmt `Cargo.lock` weiterhin verpflichtend auf, nun aus `LOCAL_TRANSACTIONAL_ROOT_FILES` statt aus einer Exporter-Sonderliste.
2. Der Project-Catalog meldet Root-`Cargo.lock` nicht mehr als Take-managed, sondern als `root_local_transaction_state / cargo_owned_local_transaction_state` mit `take_managed=no`.

Damit wird der wiederkehrende Packaging-Klassiker nicht durch eine neue Blacklist repariert. Die bereits vorhandene Take-Tree-Integrity bleibt unveraendert fail-closed; lediglich die Rollenbeschreibung wird an einer Authority zusammengefuehrt.

## Bewusst noch nicht Teil von r368

r368 aendert **nicht** die Transaktionssemantik. Insbesondere:

- `Cargo.lock` wird in diesem Take noch nicht in `TakeTransaction::prepare` gesichert;
- Rollback wird noch nicht um lokalen Cargo-Zustand erweitert;
- Root-Cargo-Aufrufe erhalten noch kein neues `--locked`-Verhalten;
- Dependency- oder Versionswechsel werden nicht neu autorisiert;
- `Cargo.lock` wird nicht in Take-ZIPs aufgenommen.

Diese Punkte gehoeren in den nachfolgenden Transactional-Closure-Take, nachdem r368 real GREEN bestaetigt ist.

## Cargo-Semantik

Cargo dokumentiert `Cargo.lock` als von Cargo gepflegte exakte Dependency-Aufloesung; die Datei soll nicht manuell editiert werden. `--locked` verlangt eine vorhandene unveraenderte Lock-Aufloesung und bricht ab, sobald Cargo die Lockdatei aendern muesste. Deshalb ist `--locked` kein Ersatz fuer eine saubere EtuSchlopf-Transaktionsrolle bei legitimen Manifest-/Versionsaenderungen.

## ChangeClass

`SecurityOnly`.

Keine Musik-, MusicXML-, Writer-, Renderer-, Solver-, Layout-, Repair-, Recovery- oder Selection-Authority-Aenderung. Keine Dependency-Aktualisierung. Kein Root-`Cargo.lock` im Take-Payload. Dieser Take vereinheitlicht ausschliesslich die Rollenwahrheit und ihre Export-/Catalog-Projektionen.
