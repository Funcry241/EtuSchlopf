# Launcher Phase Color + Bootstrap Gray Fix — v0.10.1805 r03

## Ausgangspunkt

v0.10.1804 bleibt der autoritative gruene Quellstand. Sowohl r01 als auch r02
der Version 0.10.1805 wurden durch das integrierte Launcher-Testgate vor dem
Programmlauf verworfen und vom Take-Bootstrap vollstaendig zurueckgerollt.

r01: E0433, weil der neue Cache-Test `Launcher` und `Phase` im Child-Testmodul
nicht ueber `super::` aufloeste.

r02: Die Aufloesung war korrigiert, aber `assert_eq!` auf `Option<Phase>`
verlangte fuer die Fehlerdarstellung `Debug` auf `Phase`. Diese rein durch das
Testmakro entstandene Zusatzanforderung soll den produktiven Typ nicht
veraendern.

## Korrektur des Tests

Der Cache-Test verwendet `assert!` und `matches!`. Damit prueft er weiterhin
exakt, dass der sichtbare Phasenwechsel im Cache ankommt, ohne `Debug` fuer
`Phase` zu verlangen.

## Farbrollen

Die Infrastruktur wird in zwei Graustufen getrennt, der eigentliche
Programmlauf bleibt deutlich heller:

- Rust-Take-Bootstrap: dunkelgrau (Windows `08`)
- Rust-Launcher: hellgrau (ANSI `37m`, Windows `07`)
- TAKE: tuerkis (`38;5;45m` / Windows `03`)
- CHECK/CLIPPY: cyan (`96m` / Windows `0B`)
- BUILD: magenta (`95m` / Windows `0D`)
- PROGRAM/PDF: hellweiss (`97m` / Windows `0F`)
- FINAL-Info: blau (`94m` / Windows `09`)
- Ergebnis sauber: dunkelgruen
- Ergebnis Hinweis: hellgruen
- erwarteter Stop: gelb
- Fehler: rot

`run.cmd` setzt vor der Take-Bootstrap-Vorbereitung `color 08`; die gestartete
Bootstrap-EXE erbt damit denselben dunklen Grauton. Vor der normalen
Launcher-Vorbereitung wird auf `color 07` gewechselt. Die Rust-Launcher-Phase
synchronisiert denselben hellgrauen Wert und behaelt den bereits eingefuehrten
Phase-Cache. Ein Fehler der Take-Bootstrap-Kompilation wird vor dem Exit auf
`0C` gesetzt.

Recovery-, Take-, Cache- und Reexec-Reihenfolge bleiben unveraendert. Keine
Musik-, Writer-, Solver-, Passorder-, Kosten-, Schwellen-, Repair-, Recovery-,
Dependency-, Toolchain- oder Korpussemantik wird geaendert.

ChangeClass: `SecurityOnly`.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 1"
```

Der einzelne Strict-Lauf inklusive integriertem Launcher-Testgate ist
ausreichend. Weder Vollkorpus noch erneuter Deep-Clippy werden verlangt.

---

# Launcher Gray Console Tier — v0.10.1803 r01

## Ausgangspunkt

v0.10.1802 r01 `Rust2024LetChainReadOnlyWave2` ist im Vollkorpus real gruen:
10/10 Projekte, FinalExit 0, Build/Strict/Renderer gruen und 129/129
Launcher-Tests. Der diagnostische Deep-Clippy bestaetigt ausserdem den erwarteten
Rueckgang der `collapsible_if`-Befunde von 111 auf 103.

Dieser Take trennt davon bewusst die reine Konsolen-/Launcher-Aenderung ab.

## Ziel

Der Rust-Launcher soll als Infrastruktur-Ebene sofort von produktiven
Programm-, Check-, Build- und Ergebnisphasen unterscheidbar sein, ohne eine
weitere Signalfarbe einzufuehren.

## Aenderung

Die bisherige Launcher-Startphase `Phase::Start` wird in `Phase::Launcher`
umbenannt und ausschliesslich farblich neutralisiert:

- ANSI: `90m` (bright black / grau)
- Windows-Konsole: `08` (grau auf schwarz)

Die fachlichen und statusrelevanten Farben bleiben unveraendert:

- TAKE: tuerkis
- CLIPPY/CHECK: cyan
- BUILD: magenta
- PROGRAM/PDF: helles Weiss
- FINAL SUMMARY: blau
- sauber: dunkelgruen
- externer Renderer-Hinweis: hellgruen
- erwarteter Stop/Warnung: gelb
- Fehler: rot

Damit wird Grau als Rollenfarbe fuer Launcher-Infrastruktur verwendet, nicht als
Statusfarbe.

`run.cmd` setzt die Launcher-Vorbereitungszeile ebenfalls auf Windows-Farbe
`08`. Nur falls bereits der Bootstrap-Build scheitert, wird vor dem Exit auf
`0C` (rot) gewechselt. Recovery-, Take-, Cache- und Reexec-Reihenfolge bleiben
unveraendert.

## Sicherheitsgrenze

`src/bin/etuschlopf_launcher.rs` und `run.cmd` sind gepinnte Security-Flaechen.
Deshalb werden die Fingerprints synchron in `src/identity_debt_build.rs` und
`src/identity_gate_trust_anchor.rs` fortgeschrieben.

Kein Gate wird gelockert. Keine Musik-, Writer-, Solver-, Passorder-, Kosten-,
Schwellen-, Repair-, Recovery-, Dependency- oder Toolchain-Semantik wird
veraendert.

ChangeClass: `SecurityOnly`.

## Abnahme

```powershell
.\run.cmd --take --strict --all
```

Danach diagnostisch:

```powershell
.\run.cmd --clippy-deep
```

Erwartung: identischer technischer und musikalischer Gruenstand wie v0.10.1802,
aber die Launcher-Infrastruktur erscheint neutral grau.
