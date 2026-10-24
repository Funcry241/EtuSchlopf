# r309 – Audit Deep-Clippy Risk Boundary Closure

## Ausgangspunkt

Parent ist der real gruene r308. Strict-Clippy steht bei 141 Warnungen und `architectural_budget=0`. Deep-Clippy ist diagnostisch und meldet 2452 Warnungen. Der Take behandelt nicht die Gesamtzahl als Ziel, sondern priorisiert nach Risiko.

## Geschlossene Risiken

### Endliche Sichtspurwahl

Die bisherige Suche `(1_usize..).find(...)` ist aufgrund der endlichen belegten `BTreeSet`-Menge mathematisch terminierend, fuer statische Analyse jedoch syntaktisch unbeschraenkt. r309 begrenzt die Suche auf `1..=n+1`, wobei `n` die Zahl der belegten Spuren ist. Unter diesen `n+1` positiven Kandidaten kann eine Menge mit hoechstens `n` belegten Werten nicht alle Kandidaten belegen. Die kleinste freie positive Spur bleibt daher dieselbe wie zuvor.

### CMap-Offset ohne Integer-Truncation

Beim PDF-ToUnicode-Readback wurde der Index eines Array-Ziels bisher mit `offset as u16` reduziert. r309 verlangt eine erfolgreiche `u16::try_from`-Konversion. Ein nicht darstellbarer Offset beendet die Arraysequenz. Damit kann ein uebergrosser diagnostischer PDF-Eingang keinen bereits gesehenen 16-Bit-Code durch Integer-Wrap erneut adressieren.

## Mechanische Audit-Bereinigung

Zehn bestehende Strict-Clippy-Stellen werden ohne Verdict-Aenderung in idiomatische, logisch identische Formen ueberfuehrt. Dies betrifft ausschliesslich Audit-Code. Keine Pianistik-Datei wird geaendert.

## Nicht enthalten

Keine Musikmutation, keine MusicXML-Mutation, keine Writer-/Repair-/Recovery-Schicht, keine Security-Policy, keine Pin-Aenderung, keine neue Kalibrierung und kein Etueden-/Taktspezialfall.
