# RustSourceScanPrimitivesSingleSourceClosure

## Ziel

Der Take zentralisiert ausschliesslich zwei technische Grundoperationen der
Rust-Quelltextscanner:

1. Nicht-Code-Maskierung bei identischen Bytepositionen.
2. FNV-1a-64 als Voll- und inkrementelle Hashoperation.

Die fachlichen Eigentuemer bleiben getrennt. Identity Debt, Productive Write,
Semantic Capability Diff und Take Cross-File Closure behalten ihre Findings,
Klassifikation, Fehlersemantik und Sicherheitsentscheidung.

## Vorher

Es bestanden vier voneinander abweichende Maskierer:

- `identity_debt_scan.rs`
- `productive_write_path_scan.rs`
- `semantic_capability_diff.rs`
- `take_change_control.rs`

Zusaetzlich bestanden lokale FNV-Kerne im Build-Script, Semantic Capability
Diff, Take Change Control, Take-Bootstrap und in der normalisierten
Productive-Write-Baseline.

Die Varianten unterschieden sich unter anderem bei Lifetimes, Byte-Strings,
C-Strings, Raw-Byte-Strings und CRLF-Erhalt. Damit konnte dieselbe Rust-Quelle
je nach Sicherheitsowner unterschiedlich maskiert werden.

## Nachher

`src/build_gate/rust_source_scan_primitives.rs` besitzt den einzigen Vertrag:

- `mask_non_code`
- `fnv1a64`
- `fnv1a64_update`
- `FNV1A64_OFFSET_BASIS`

Der Lexer behandelt:

- `//`-Kommentare,
- verschachtelte `/* ... */`-Kommentare,
- normale, Byte- und C-Strings,
- normale, Byte- und C-Raw-Strings,
- Zeichen- und Bytezeichenliterale,
- Lifetimes und Labels als Code,
- nicht abgeschlossene Literale und Kommentare fail-closed bis Dateiende.

Er erhaelt die Eingabelaenge bytegenau und bewahrt `LF` sowie `CRLF`.

## Modulgraph

Die Datei wird pro Crate genau einmal geladen:

- Build-Script: `identity_debt_build.rs`
- Take-Binaerziel: `etuschlopf_take_bootstrap.rs`
- Hauptprogramm-Testziel: `identity_debt_inventory.rs`
- Integrationstest: `global_identity_debt_inventory.rs`

`take_change_control.rs` und das darunter liegende
`semantic_capability_diff.rs` verwenden dieselbe Bootstrap-/Build-Script-
Instanz. Es gibt keine zweite `mod`-Deklaration innerhalb derselben Crate.

## Testabschluss

Die neue Primitive enthaelt Regressionen fuer:

- bekannte FNV-1a-Vektoren und inkrementelle Gleichheit,
- Laengen-, LF- und CRLF-Erhalt,
- verschachtelte Kommentare,
- normale, Byte-, C- und Raw-Strings,
- Zeichenliterale gegen Lifetimes und Labels,
- fail-closed Verhalten bei nicht abgeschlossenen Nicht-Code-Bereichen.

Die vorhandenen Identity- und Productive-Write-Testfunktionen bleiben bei ihren
fachlichen Eigentuemer-Modulen. Im Hauptprogramm-Testziel stellt
`identity_debt_inventory.rs` die einzige Primitive bereit; beide Scanner
verwenden genau diese Instanz. Die bestehende Integrations-Crate behaelt ihre
unabhaengige End-to-End-Pruefung.

## Ausdruecklich unveraendert

- Musik- und Taktlogik
- MusicXML- und PDF-Erzeugung
- Writer- und Authority-Grenzen
- Pipeline-Reihenfolge
- Schwellenwerte und Gewichtungen
- Recovery- und Ledgerformate
- Take-Manifestformat
- Releaseentscheidung
