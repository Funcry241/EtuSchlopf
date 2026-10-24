# InitialRenderTransactionStageExtraction

Stand: v0.10.1671 r03  
Ausgangspunkt: v0.10.1670 r01


## Revision r03 – Bootstrap-Dependency-Closure

Der reale r02-Installationsversuch startete weiterhin vom bestaetigten Stand
`v0.10.1670`, weil r01 wegen des Compilefehlers nie installiert worden war.
Das r02-Manifest deklarierte jedoch irrtuemlich einen Same-Version-Uebergang
`0.10.1671 -> 0.10.1671` und wurde deshalb bereits im Dependency-Preflight
korrekt abgelehnt.

r03 veraendert den Rust-Quellstand von r02 nicht. Es deklariert den gesamten
kumulativen Take korrekt als `0.10.1670 -> 0.10.1671`: Stage-Extraktion aus r01
plus die drei Compile-Bindungen aus r02. Take 2 und Take 3 bleiben ausserhalb
des Scopes.


## Revision r02 – Compile-Closure

Der reale r01-Strict-Lauf fand genau drei fehlende Bindungen nach dem
Destructuring von `InitialRenderTransaction`. Alle drei Werte waren bereits
korrekt Bestandteil des Stage-Outcomes; r01 uebernahm sie an der spaeteren
Scope-Grenze jedoch nicht als lokale Bindungen. r02 ergaenzt ausschliesslich:

- `lower_staff_octave_render_suppression_trial`,
- `strict_staff_local_release_completeness_trial`,
- `strict_visible_rest_arithmetic_materializer_trial`.

Die Produktionsanweisungen innerhalb der Stage und alle nachgelagerten
Verwendungen bleiben unveraendert. Gegenueber r01 umfasst der Quellfix exakt
drei zusaetzliche Rust-Zeilen.

## Ziel

Dieser `RefactorOnly`-Take zieht genau die erste geschlossene Render-Transaktion
aus `real_main()` in eine typisierte Stage. Der Scope beginnt mit dem Aufbau des
Render-MusicXML-Pfads und endet nach dem bestehenden
`ExternalNotationFeedback-SafeWrite`-Audit unmittelbar vor der bereits
vorhandenen `RestAndArticulationRenderStage`.

Es werden keine Musikregel, kein Writer, keine Schwelle, kein Reportfeld und
keine Passposition hinzugefügt, entfernt oder umgeordnet.

## Neue Grenze

Die Stage besteht aus:

- `InitialRenderTransactionStageInputs`,
- `run_initial_render_transaction_stage`,
- `InitialRenderTransactionStageOutcome`,
- `InitialRenderTransaction`.

Sie erhält ausschließlich:

- die ausgewählte `SolverCandidate`-Fassung,
- den bestehenden `SourcePlan`,
- den bestehenden `ReferenceReport`.

Sie gibt die mutierte Solverfassung und genau die später tatsächlich benötigten
Renderzustände und Audits zurück. Nur innerhalb der Stage benötigte
Zwischenstände bleiben lokal.

## Verhaltensgleichheit

Der bisherige Produktionsblock wurde ohne inhaltliche Änderung verschoben. Ein
byteweiser Quellvergleich des Blockkörpers bestätigt:

```text
originaler Block: 29.681 Bytes
Stage-Block:      29.681 Bytes
identisch:        ja
```

Damit bleiben insbesondere unverändert:

- Reihenfolge aller bestehenden Builder, Verifier und Writer,
- alle `.written`- und `pdf_source_selected`-Entscheidungen,
- Master- und Render-MusicXML-Transformationen,
- Snapshot- und Fallback-Semantik,
- Rest-, Clef-, Balken-, Register- und Feedbacklogik,
- Fehlertexte und Fail-Closed-Reihenfolge,
- PDF-Quellenauswahl,
- Report- und Releaseprojektion.

`RenderXmlSnapshotReader` wird nach der Stage aus demselben zurückgegebenen
Renderpfad neu gebunden. Der Typ ist schreibfrei und besitzt ausschließlich die
bestehende Read-or-Fallback-Semantik.

## Strukturgewinn

Physische Zeilen:

```text
real_main():                                      1.097 ->   598
pipeline_orchestrator_report_compaction.rs:       3.541 -> 3.632
Rust gesamt unter src:                          117.388 -> 117.482
Netto Rust:                                                 +94
```

Die 499 aus `real_main()` entfernten Zeilen werden durch eine explizite
Input-/Outcome-Grenze und einen besitzenden Transaktionszustand ersetzt. Der
Gesamtzuwachs ist ausschließlich Transport- und Eigentumsstruktur; die
Produktionsanweisungen selbst bleiben bytegleich.

Die Zahl der `#[allow(...)]`-Direktiven bleibt unverändert.

## Stopkriterium

Dieser Take endet nach der Extraktion der frühen Render-Transaktion. Die
metrische Balken-/Finalnotation und die Release-/Report-/Exit-Phase bleiben
vollständig außerhalb des Scopes und werden nicht im selben Take umgebaut.

## Abnahme

Einmalig:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Erwartet werden Build, Strict-Clippy, Program und FinalExit 0 sowie gegenüber
v0.10.1670 versionsnormalisiert identische Master-/Render-MusicXML und sechs
pixelidentische PDF-Seiten. Kein Korpuslauf und kein `--force`.
