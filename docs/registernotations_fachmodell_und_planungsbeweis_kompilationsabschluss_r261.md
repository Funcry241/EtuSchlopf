# r261 – RegisternotationsFachmodellUndPlanungsbeweisKompilationsabschluss

## Elternstand

Akzeptierter Elternstand ist EtuSchlopf v0.10.1839 r258 `OktavKausalkantenLokalisierung`. r259 und r260 sind verworfene Kandidaten und keine Elternstaende.

## Warum r261 existiert

r259 wurde vor jeder Installation durch Take-Tree-Integrity verworfen, weil das Paket versehentlich den nicht verwalteten Root-Pfad `Cargo.lock` enthielt. r260 beseitigte diesen Paketfehler und erreichte den realen Windows-Rust-Baugate. Dort trat genau ein harter Compilerfehler E0505 in `src/registernotation/mod.rs` auf.

Die Ursache war eine falsche Reihenfolge beim Abschluss des schreibfreien Beweiswerts:

```text
gruppen -> enthaelt Referenzen auf ereignisse
ereignisse -> sollte bereits in RegisternotationsBeweis verschoben werden
gruppen.len() -> wurde erst danach ausgewertet
```

Damit blieb die Leihe bis hinter den vorgesehenen Verschiebezeitpunkt lebendig.

## Enge Korrektur

r261 materialisiert `gruppen.len()` vor dem Verschieben von `ereignisse` in den Rueckgabewert. Es wird bewusst kein `clone()` der Ereignisse oder Gruppen eingefuehrt.

Die Korrektur veraendert nicht:

- die Score-basierte Ereignisbeobachtung vor K00,
- Sichtbarkeit oder Ereignisidentitaet,
- kanonische Zeit,
- Grenzenevidenz,
- bestehende Oktav-Bewertungsformel,
- Segmentierung oder Planbildung,
- betroffene Ereignismengen,
- Spannentopologie,
- typisierte Abweichungsgruende,
- Plan-Fingerabdruck-Verfahren,
- produktive MusicXML- oder StructuralWriteAuthority-Befugnisse.

Die 49 im r260-Clippy-Lauf gemeldeten Hinweise sind laut bestehendem Warnungsbudget diagnostisch. r261 enthaelt deshalb kein allgemeines Clippy-Aufraeumen.

## Fachlicher Schnitt bleibt unveraendert

r261 fuehrt weiterhin die schreibfreie Fachdomaene `src/registernotation/` direkt auf dem r258-Elternstand ein. Sie liest vor K00 ausschliesslich `model::Score` und erzeugt typisierte Ereignisse, Grenzenevidenz und `RegisterNotationPlan`. Der alte produktive Oktavierungsbesitzer bleibt unveraendert aktiv. Nach K00 wird nur das bereits etablierte r257-Altmaterialisierungszeugnis als unabhaengige Zaehler-Evidenz gelesen.

Die bestehenden Konstanten bleiben unveraendert:

- mindestens 4 Tonobjekte,
- oberes System 60..84,
- unteres System 36..60,
- Kosten ausserhalb des Bereichs `delta^2 + delta`,
- Verbesserungsschwelle 8,
- Zuschlag 24 fuer 15ma/15mb,
- MIDI 69 als bestehender Vorrang der spaeteren unteren Schluesselbehandlung.

## Harte Topologie

Vorab festgelegt bleiben:

- leere betroffene Ereignismengen: 0,
- Nullspannen: 0,
- betroffene Ereignisse ausserhalb ihrer Spanne: 0,
- systemfremde betroffene Ereignisse: 0,
- widerspruechliche ueberlappende Registerplaene: 0,
- nicht klassifizierte Abweichungen: 0.

Die bekannten r257/r258-Korpuswerte bleiben unveraendert Vergleichsbasis: 729 System-Takt-Gruppen, 10.562 Tonobjekte, 7.855 gedruckt, 2.707 verborgen/nicht gedruckt, 53 Sichtbarkeitsgegenfaelle, 179 vorhandene Hauptdatei-Oktavspannen, 20 verspaetete Beginne, 1 alte Nullspanne und 20/20 K00-Lokalisierungen.

## Pruefzahl und naechster Besitzschnitt

r258 besitzt real 920/920 Pruefungen. r261 fuegt gegenueber r258 weiterhin exakt 28 kuenstliche Rust-Pruefungen hinzu. Die vorab festgelegte Erwartung bleibt **948/948**.

Nach real gruenem r261 darf der spaetere produktive Besitzwechsel als r262 ausschliesslich denselben in r261 bewiesenen Planer materialisieren. Zwischen r261 und r262 darf keine neue musikalische Entscheidungsregel eingefuehrt werden.
