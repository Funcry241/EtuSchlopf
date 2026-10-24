# DeepClippyUninlinedFormatArgsMechanicalReduction

## Scope

Dieser Take bearbeitet genau die `uninlined_format_args`-Diagnosen ausserhalb
der durch Take-Change-Control als Security klassifizierten Pfade.

- Deep-Clippy-Diagnosen im RefactorOnly-Scope: 262
- reale eindeutige Formatmakros: 261
- betroffene Rust-Dateien: 45
- verbleibende Security-Diagnosen dieser Familie: 43
- neue `allow`-Attribute: 0
- physische Rust-Zeilenbilanz: -91

## Transformation

Aus bereits vorhandenen Positionsargumenten werden direkte Rust-
Formatstring-Captures. Beispielsweise:

```rust
format!("Projekt: {}", project)
```

wird zu:

```rust
format!("Projekt: {project}")
```

Formatmerkmale bleiben erhalten, etwa `:.2`, `:<12`, `:02`, `:>3` und
`:016x`. Die vorhandene benannte Breitenangabe `width = ...` bleibt bestehen;
nur das zugehoerige Positionsargument wird direkt eingefangen.

## Nicht veraendert

- keine Security-Datei
- keine Musikentscheidung oder Kandidatenreihenfolge
- keine Schwelle, Writer- oder Pipeline-Reihenfolge
- keine Sichtbarkeit und kein API-Vertrag
- keine Casts und keine verdaechtigen Clippy-Logikhinweise
- keine Report-, MusicXML-, PDF- oder Artefaktsemantik

## Beweisfolge

1. `run.cmd --take --strict "Etüde Nr. 4"`
2. Strict-Warnungsbudget 0 und fachliche Paritaet
3. `run.cmd --clippy-deep`
4. erwartete Nettoreduktion um 262 Diagnosen ohne Folgegruppe
5. Korpus erst nach den guenstigen Gates
