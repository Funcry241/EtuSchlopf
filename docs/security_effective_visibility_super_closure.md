# SecurityEffectiveVisibilitySuperClosure

## Ausgangspunkt

Autoritative Quellbasis ist v0.10.1656. Der Deep-Clippy-Bericht weist
`redundant_pub_crate` als groesste verbleibende Familie aus. Diese erste Stufe
bearbeitet ausschliesslich die Security-/Build-Gate-Control-Plane.

## Physischer Scope

153 echte Deklarationen in neun Security-Dateien werden von `pub(crate)` auf
`pub` normalisiert:

- `src/take_change_control.rs`: 60
- `src/identity_gate_trust_anchor.rs`: 19
- `src/build_gate/semantic_capability_diff.rs`: 14
- `src/build_gate/multi_revision_evasion.rs`: 14
- `src/security_control_plane/public_artifact_recovery.rs`: 13
- `src/identity_debt_inventory.rs`: 12
- `src/build_gate/formal_green_contract.rs`: 10
- `src/feedback_structuralization_gate.rs`: 6
- `src/feedback_proof_transport.rs`: 5

Die umschliessenden Module bleiben privat. `pub` beseitigt daher nur die
innerhalb dieser privaten Modulidentitaeten redundante Crate-Einschraenkung;
es oeffnet keine externe Package-API.

## Gate-Koordination

`src/identity_debt_build.rs` prueft die betroffenen Vertrage nun anhand der
effektiven Sichtbarkeit. Uebergangsfaehig akzeptiert es sowohl `pub` als auch
`pub(crate)`, waehrend private Detailtypen weiterhin unter beiden Formen
verboten bleiben. Die generierten Identity- und Productive-Write-Inventare
werden ebenfalls mit `pub` erzeugt.

Alle beruehrten Trust Anchors und FNV-Pins sind auf die exakten neuen Bytes
gebunden. Scannerabdeckung, Fail-Closed-Entscheidungen, Quellwurzeln und
negative Sicherheitspruefungen bleiben erhalten.

## Nicht im Scope

Nicht veraendert werden Musiklogik, Writer, Schwellen, Passreihenfolge,
Reportsemantik, Artefaktvertraege oder Non-Security-Deklarationen. Es wird kein
Lint erlaubt und kein `cargo fix` eingesetzt.

## Beweisstatus

Vor Auslieferung werden Diff-Domaene, Sichtbarkeitsreste, generierte
Rust-Ausgaben, Trust-Anchor-/FNV-Konsistenz, Allow-Bilanz, Zeilenbilanz und
Paketstruktur statisch geprueft. Gruenstatus und exakter Deep-Clippy-Abbau
entstehen erst durch den realen Nutzerlauf.

## Revision r02

Der reale r01-Reexec wurde vom Architektur-Build-Gate zurueckgerollt. Nicht die
private Deklaration war fehlerhaft, sondern die neue Hilfssuche: Sie suchte die
Sichtbarkeit als beliebige Teilzeichenfolge und traf dadurch einen
`pub struct DiagnosticCoordinate`-Pruefstring im eigenen Quelltext.

r02 maskiert zuerst Kommentare und Literale mit der gemeinsamen Rust-Scanner-
Primitive und bindet die Erkennung danach an den Beginn einer tatsaechlichen
Quellzeile nach optionaler Einrueckung. Damit akzeptiert das Gate weiterhin `pub` und
`pub(crate)` fuer effektiv oeffentliche Deklarationen, ignoriert aber
Stringliterale, Kommentare und inline eingebettete Prueftexte. Die
Privacy-Invariante selbst wird weder gelockert noch ausgenommen.

