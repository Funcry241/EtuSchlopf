# AssistantExportFormatPushStringClosure

## Ausgangslage

Der bestaetigte v0.10.1667-Einzellauf endet mit Build 0, Strict Clippy 0 und
FinalExit 0. Die Cargo-native Kurzprojektion reduziert den realen
Deep-Clippy-Bericht von 1.026.914 Bytes/22.044 Zeilen auf 223.037 Bytes/1.738
Zeilen. CargoExit bleibt 0; die Zusammenfassung zaehlt 1.697 Diagnosen bei
1.706 rohen Warnzeilen und neun ausgeschlossenen Cargo-Summary-Zeilen.

Im Launcher-Owner `src/bin/etuschlopf_launcher/assistant_export.rs` verbleiben
32 physische `format_push_string`-Fundstellen. 31 davon schreiben den
Emergence-Auswahlvertrag in einen bereits vorhandenen JSON-String; eine
weitere maskiert Steuerzeichen im lokalen JSON-Escaper.

## Scope

Die 31 zeilenabschliessenden Konstruktionen

```rust
out.push_str(&format!("...\n", value));
```

werden als direkte `writeln!`-Schreibvorgaenge auf demselben `String`
formuliert. Der Steuerzeichenpfad verwendet entsprechend `write!`. Beide
Makros schreiben ueber `std::fmt::Write` direkt in den bestehenden Puffer und
vermeiden die jeweils temporaer allokierte Zwischen-`String`.

Unveraendert bleiben:

- Formatstrings, Feldreihenfolge, Kommata, Newlines und JSON-Escaping,
- Exportdateinamen, ZIP-Inhalt und Manifestvertrag,
- Fehler- und Releaseentscheidungen,
- Musikpipeline, Writer, Reports, Schwellenwerte und Passreihenfolge,
- alle anderen Clippy-Familien und alle anderen Owner.

Die direkte String-Ausgabe ist fuer `String` unfehlbar; der lokale
`expect`-Text dokumentiert diese Standardbibliotheksgarantie. Es wird kein
allgemeiner Formatter, keine neue Exportabstraktion und keine Fehlerkaskade
eingefuehrt.

## Erwartung

Der naechste Deep-Clippy-Lauf soll genau die 32 physischen
`format_push_string`-Diagnosen dieses Owners entfernen. Die aktuelle
Gesamtzahl 1.697 wird damit bei unveraenderten anderen Familien auf 1.665
sinken.

## Sicherheitsbindung

Das Assistant-Export-Modul ist als Launcher-Sicherheitsgrenze FNV-gepinnt.
Deshalb werden sein FNV, der dadurch geaenderte Build-Policy-FNV und der Trust
Anchor koordiniert aktualisiert. Die Change-Class lautet `SecurityOnly`.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
.\run.cmd --clippy-deep
```
