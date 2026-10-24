# Structural-Authority-Primitivkern v0.10.1522

## Problem

Die Structural Write Authority besaß dieselbe elementare Sicherheitslogik in vielen Geschwistermodulen mehrfach: FNV-Quellbindung, XML-Fragmentprüfung, exakte Note/Pause, XML-Kindzugriff und boolesche Reportprojektion. Diese Kopien hatten keine eigene fachliche Auswahlkompetenz, konnten aber unabhängig auseinanderlaufen.

## Lösung

`authority_primitives.rs` ist ein privates Kind der Structural Write Authority. Seine Elemente sind ausschließlich `pub(super)` und damit nur innerhalb dieses Eigentümers sichtbar.

Die Writer und Verifier importieren exakt die benötigten Primitiven. Es gibt keinen glob-Reexport und keine neue Crate-weite Hilfsfläche.

## Semantik

Die früheren Varianten bleiben getrennt, wenn ihre Verträge verschieden waren:

- `parsed_exact_note` verlangt weiterhin einen gültig parsebaren XML-Fragmentbaum.
- `syntactic_exact_note` bewahrt die bisherige rein syntaktische Prüfung.
- `optional_xml_fragment_parses` akzeptiert weiterhin das leere Fragment, während `xml_fragment_parses` den normalen Parsevertrag besitzt.

Damit wird keine schwächere Universalprüfung eingeführt.

## Sicherheitswirkung

- eine Implementierung pro elementarer Zusicherung
- ein privater Eigentümer
- keine neue Schreibberechtigung
- kein Transport über Strings oder Reporttext
- keine Musikmutation
- keine Änderung der Writer-Reihenfolge
- lokale Tests für die semantisch getrennten Varianten
