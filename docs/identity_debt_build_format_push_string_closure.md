# IdentityDebtBuildFormatPushStringClosure

## Ausgangslage

Der v0.10.1668-Einzellauf endet mit BuildExit 0, ClippyExit 0 und FinalExit 0.
Master- und Render-MusicXML sind gegenueber v0.10.1667 nach
Versionsnormalisierung identisch; alle sechs PDF-Seiten sind pixelidentisch.

Der anschliessende Deep-Clippy-Lauf bestaetigt exakt 1.665 Diagnosen. Damit
wurden die 32 `format_push_string`-Meldungen des Assistant-Export-Owners ohne
Ersatzwarnung entfernt. Der groesste verbleibende homogene Owner derselben
Familie ist das gepinnte Build-Gate `src/identity_debt_build.rs` mit genau 26
physischen Fundstellen.

## Scope

Ausschliesslich diese 26 Stellen werden direkt in den bereits vorhandenen
`String` geschrieben:

- drei nicht zeilenabschliessende FNV-Darstellungen verwenden `write!`,
- 23 zeilenabschliessende Diagnose- und Generatorzeilen verwenden `writeln!`.

Die bisherigen Konstruktionen

```rust
out.push_str(&format!("..."));
```

erzeugen fuer jeden Aufruf eine temporaere Zwischen-`String`. Die direkte
`std::fmt::Write`-Form behaelt Formatargumente, Debug-/Display-Darstellung,
Zeichenreihenfolge und Newlines bei, vermeidet aber diese Zwischenallokation.

Unveraendert bleiben:

- Scannerregeln, Baseline und Fundstellenklassifikation,
- generierte Konstantennamen und generierter Rust-Quelltext,
- Fail-Closed-Meldungen und ihre Reihenfolge,
- Productive-Write-Surface-Inventar und Bypass-Entscheidung,
- Musikpipeline, Writer, Reports, Schwellenwerte und Passreihenfolge,
- alle anderen Clippy-Familien und alle anderen produktiven Owner.

Die Schreiboperation auf `String` ist unfehlbar; der lokale
`expect("writing to String cannot fail")`-Text macht diese
Standardbibliotheksgarantie explizit.

## Sicherheitsbindung

`src/identity_debt_build.rs` ist die compile-time gepinnte Build-Policy.
Deshalb wird ausschliesslich ihr neuer FNV im bestehenden
`src/identity_gate_trust_anchor.rs` synchronisiert. Die Change-Class lautet
`SecurityOnly`.

## Erwartung

Der naechste Deep-Clippy-Lauf soll genau 26 Diagnosen entfernen:

```text
1.665 -> 1.639
```

Ein bestaetigter Wert entsteht erst durch den realen Zielsystemlauf.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
.\run.cmd --clippy-deep
```
