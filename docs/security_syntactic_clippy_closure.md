# Security Syntactic Clippy Closure

## Scope

Dieser `SecurityOnly`-Take schliesst die drei rein syntaktischen Lintfamilien
innerhalb der geschuetzten Security-Pfadmenge:

```text
needless_raw_string_hashes  31 Diagnosen / 21 Stellen
single_char_pattern          5 Diagnosen /  3 Stellen
unnested_or_patterns         3 Diagnosen /  2 Stellen
Gesamt                       39 Diagnosen / 26 Stellen
```

Mehrfachdiagnosen entstehen durch die vorhandenen Build-Script-, Bootstrap-,
Hauptprogramm- und Integrationstest-Kompilationsidentitaeten. Jede physische
Stelle wird genau einmal geaendert.

## Transformationen

- `r#"..."#` wird nur dort zu `r"..."`, wo der Inhalt kein Hash-Trennzeichen
  benoetigt.
- einzeichenlange `str`-Suchmuster werden zu `char`-Suchmustern.
- gleichartige `Some(...)`-Alternativen werden in das innere Or-Pattern
  zusammengezogen.

## Invarianten

Keine Funktion, kein Test, kein Symbol, kein Pfad und keine Capability wird
entfernt oder verschoben. Scannerergebnis, Vergleichslogik, Rueckgabewert,
Auswertungsreihenfolge und erzeugte Zeichenfolgen bleiben unveraendert. Alle
geaenderten gepinnten Security-Dateien erhalten koordinierte FNV- und
Trust-Anchor-Werte. Keine Allow-Direktive wird hinzugefuegt.

Die nicht zur Security-Domaene gehoerenden Fundstellen sind ausdruecklich nicht
Teil dieses Takes und werden erst in der separaten Non-Security-Closure
bearbeitet.
