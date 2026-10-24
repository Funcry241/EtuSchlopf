# Non-Security Syntactic Clippy Closure

## Ausgangslage

Der Maschinenlauf von v0.10.1655 bestaetigt 2.417 Deep-Clippy-Diagnosen. Die
Security-Zone aus Schritt 1 ist exakt um 39 Diagnosebloecke gesunken. Von den
drei vereinbarten syntaktischen Familien verbleiben nur noch:

```text
needless_raw_string_hashes   7 Diagnosen /  6 Stellen
single_char_pattern          0 Diagnosen /  0 Stellen
unnested_or_patterns        11 Diagnosen / 11 Stellen
Gesamt                       18 Diagnosen / 17 Stellen
```

## Transformationen

- ueberfluessige Hash-Trenner an Raw-String-Literalen werden entfernt,
- gleichartige `Some(...)`-Alternativen werden als inneres Or-Pattern notiert,
- die gemeinsame zweite Tupelkomponente wird im Pattern nur einmal notiert.

Die Zeichenfolgenwerte, Matchmengen und Kontrollfluesse bleiben identisch. Die
Doppelmeldung fuer eine Pipeline-Stelle entsteht aus zwei vorhandenen
Kompilationsidentitaeten; die physische Stelle wird nur einmal geaendert.

## Grenzen

Der Take verwendet `RefactorOnly`. Security-Pfade bleiben byteidentisch. Es
werden keine Funktionen, Tests, Pfade, Regeln, Writer, Schwellen, Taktadressen,
Etuedenadressen, Passreihenfolgen, Artefaktentscheidungen oder Allows geaendert.

Der erwartete Deep-Clippy-Wert nach Maschinenabnahme ist 2.399. Diese Zahl ist
vor dem realen Lauf nur die statische Differenz 2.417 minus 18.
