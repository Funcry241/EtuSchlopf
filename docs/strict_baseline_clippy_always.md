# StrictBaselineClippyAlways


## Revision r04

r04 ersetzt den vom Semantic-Capability-Diff bereits vor der Installation
abgelehnten r03-Kandidaten. Die produktive Strict-Baseline und der
Clippy-Always-Pfad bleiben unveraendert. Der korrigierte Legacy-Test behaelt
seine bestehende negative Security-Capability-Identitaet
`legacy_visual_preflight_arg_does_not_create_a_non_strict_test_plan`; nur die
Assertions pruefen nun den aktuellen Vertrag: Visual-Preflight-Wunsch sichtbar,
Gesamtplan Strict, `--all` als Projekteingabe und kein eigenstaendiger
nicht-strikter Launcher-Modus.

## Ziel

Ab EtuSchlopf v0.10.1808 r04 ist Strict die oeffentliche Launcher-Baseline.
Der Benutzer muss `--strict` nicht mehr angeben, um den normalen
Qualitaetsvertrag zu erhalten.

## Oeffentlicher Vertrag

- Projekt-, Korpus- und Visual-Preflight-Ausfuehrungen starten standardmaessig
  mit `ValidationPlan::Strict`.
- `--strict` bleibt als idempotenter Kompatibilitaetsmarker akzeptiert.
- Das produktive Buildprofil ist dadurch standardmaessig `strict`.
- Das integrierte Launcher-Testgate bleibt Bestandteil jedes normalen
  Strict-Laufs.
- `run.cmd` erhaelt keine neue Policy und bleibt ein duenner Starter.

## Clippy-Vertrag

Clippy Strict ist nicht nur eine Eigenschaft eines irgendwann erzeugten
Build-Artefakts. Der aktuelle Source-Stand wird bei jedem normalen Lauf erneut
mit dem blockierenden Strict-Gate geprueft:

```text
cargo clippy --profile strict --jobs <auto> -- \
  -D clippy::correctness \
  -D clippy::suspicious \
  -D clippy::perf
```

Das gilt auch dann, wenn ein bereits verifiziertes Strict-Binary anhand von
Version, Source-Signatur und Executable-Hash wiederverwendet werden darf.
Build-Reuse spart den Build, niemals den aktuellen Clippy-Gate.

## Deep Clippy

`--clippy-deep` bleibt bewusst ausserhalb des verpflichtenden Normalvertrags.
Es ist eine diagnostische, nicht blockierende Projektion fuer pedantic, nursery,
cargo und Rust-2024-Kompatibilitaetswarnungen. Die verpflichtende Basis braucht
keinen separaten Clippy-Kommandozeilenmodus.

## Rueckwaertskompatibilitaet

Bestehende Aufrufe mit `--strict` funktionieren unveraendert. Neue normale
Aufrufe koennen kuerzer sein:

```powershell
.\run.cmd "Etüde Nr. 6"
.\run.cmd --all
.\run.cmd --watch "Etüde Nr. 6"
```

Alle drei besitzen denselben Strict-Basisvertrag.

## Revisionsdisziplin

Die zuvor erzeugte v0.10.1808 r01 `Rust2024LetChainReadOnlyWave4` war zum
Zeitpunkt dieser Aenderung nicht abgenommen. r03 basiert deshalb direkt auf dem
real gruenen v0.10.1807 r01 und enthaelt die r01-Refactor-Welle nicht. Erst nach
gruener r03-Abnahme darf die Let-Chain-Welle auf dieser neuen Basis neu
aufgelegt werden.
