# r23 – LocalLayoutIdempotenceAndUnambiguousTestDriftClosure

Parent: committed v0.10.1839 r17.

Die All-Targets-Diagnoseläufe r19/r20/r22 wurden absichtlich zurückgerollt und sind keine produktive Basis. Sie lieferten jedoch frische Evidenz für mehrere bislang nicht ausgeführte Main-Tests. r23 übernimmt ausschließlich Befunde, deren Ursache im r17-Source ohne Interpretation eindeutig ist; Security-/Launcher-Testgate-Änderungen aus r22 werden ausdrücklich nicht mit einer MusicMutation vermischt.

Produktive Korrekturen:

1. `local_staff_distance_transaction`: Die Nested-Measure-Abwehr suchte bisher nach dem bloßen Prefix `<measure` und klassifizierte dadurch das gültige MusicXML-Element `<measure-layout>` als verschachtelten Takt. Die Abwehr bleibt fail-closed, erkennt als nested measure jetzt aber nur einen exakten `<measure...>`-Tagstart anhand des folgenden XML-Tagzeichens. Echte verschachtelte `<measure>` bleiben verboten.
2. `horizontal_layout_plan_materialization`: Beim Entfernen eines reinen horizontalen Break-`<print>` blieb dessen ausschließlich führende Einrückung erhalten. Beim anschließenden Wiedereinfügen desselben Breaks konnte sich dadurch bei Rematerialisierung Whitespace akkumulieren. Führender Whitespace, der ausschließlich zu einem entfernten Break-Print gehört, wird nun mit dem Print kanonisch entfernt; semantischer Prefix-Inhalt bleibt bytegetreu erhalten.

Eindeutige Testdrift-Korrekturen:

- Productive-write-path-Test erwartet die aktuelle eingefrorene Baseline mit genau einem retired external surface statt des historischen Werts 122.
- Move-only-Capability-Source-Test betrachtet nur den Production-Prefix vor `#[cfg(test)]`, damit sein eigener verbotener Suchstring nicht den Test selbst rot macht.
- Zwei Visual-Layout-Defaults-Fixtures verwenden einen kanonischen nicht-self-closing `<part-list>`-Boundary statt `<part-list/>`, den der produktive Boundary-Vertrag absichtlich nicht akzeptiert.

Nicht angefasst: Beam-/Notation-Idempotenz, Silent-Rest-Placement, Remaining-Layout-Range-Removal und sämtliche Clef-Planning-Fälle. Diese bleiben für den nächsten separaten All-Targets-Diagnoselauf sichtbar.

ChangeClass: `MusicMutation`.
