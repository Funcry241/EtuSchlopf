# SecurityPubSuperEffectiveVisibilityClosure

## Maschinenbefund v0.10.1658

Der reale All-Targets-Bericht endet mit `CargoExit 0` und `Warnings: 1918`.
`redundant_pub_crate` ist nicht leer: Clippy meldet 28 Diagnosebloecke fuer
`pub(super)`, obwohl alle 515 zuvor vereinbarten `pub(crate)`-Deklarationen
geschlossen sind. Nach Normalisierung doppelter Targetpfade bleiben 27 reale
Item-Deklarationen in vier Dateien.

## Revisionsbefund r01

r01 wurde vor jeder Installation durch den Semantic-Capability-Diff abgelehnt.
`create_assistant_export` und `execute_visual_preflight_run` wurden durch die
Umstellung auf `pub` als neue `ExportedWriterCapability`-Oberflaechen erkannt.
Der Pre-Copy-Beweis ist autoritativ: Diese beiden `pub(super)`-Grenzen sind im
Projekt semantisch enger als unbeschraenktes `pub` und werden nicht fuer eine
Lint-Null aufgeweitet.


## Revisionsbefund r02 und Scope r03

r02 erreichte nach transaktionaler Installation den Strict-Clippy-Compile. Die
Kompilation stoppte fail-closed am konstanten Trust Anchor, weil beide
Assertions fuer `IDENTITY_BUILD_POLICY_FNV1A64` noch den vorherigen Wert
`0x8eae_1360_af0d_5c99` erwarteten. Der tatsaechliche, bereits im
Change-Manifest deklarierte FNV1A64-Wert von `src/identity_debt_build.rs` ist
`0xa25a_8651_797f_7d81`.

r03 aendert ausschliesslich diese zwei Assertions sowie Revisionsmetadaten und
deren Manifest-FNVs. Die 24 Item-Koepfe, alle bewusst engen Writer-Grenzen und
saemtliche fachlichen Invarianten bleiben gegenueber r02 unveraendert.

## Scope r03

Die harte Security-/Non-Security-Grenze wird nicht umgangen. Dieser
`SecurityOnly`-Take bearbeitet genau 24 capability-neutrale Security-
Deklarationen:

- 20 in `src/bin/etuschlopf_launcher/assistant_export.rs`
- 3 in `src/visual_preflight.rs`
- 1 in `src/build_gate/sha256.rs`

Bewusst unveraendert bleiben:

- `assistant_export::create_assistant_export`
- `visual_preflight::execute_visual_preflight_run`
- die Non-Security-Stelle `pipeline::run`

## Invarianten

Nur die 24 capability-neutralen Item-Koepfe wechseln von `pub(super)` auf
`pub`. Umschliessende Module bleiben privat. Felder und Methoden, die Clippy
nicht als redundant diagnostiziert, bleiben unveraendert. Es entsteht keine
neue Writer-Capability und kein neues Schreibrecht.

SHA-256-Primitive und Launcher-Export sind gepinnt; deshalb werden ihre
FNV-Bindungen sowie der Build-Policy-Trust-Anchor koordiniert aktualisiert.
Keine neue Allow-Direktive, kein `cargo fix`, keine Musikmutation und keine
Aenderung von Writer-Senken, Schwellen, Passreihenfolge, Report oder Artefakten.

Nach erfolgreichem Lauf werden drei `redundant_pub_crate`-Diagnosen erwartet.
Dieser Wert ist erst durch den realen All-Targets-Lauf bewiesen.
