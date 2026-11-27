# r2015 · Vollständigkeit der Paarermittlung

Der AuditOnly-Take baut auf dem unter Windows angenommenen r2014 auf.
Er prüft die Erfassung von Notenköpfen, Hälsen und Haltebogenkonturen in
einem kontrollierten einseitigen MuseScore-Profil. Die synthetische
MusicXML-Vorlage legt den erwarteten Zeichenbestand vor dem Rendern fest.
[Vollständigkeit der Paarermittlung](https://github.com/Funcry241/EtuSchlopf/blob/995426b074f7139b74f71c171637eef6a1828662/docs/MATHEMATIK.md#vollstaendigkeit-paarermittlung)

Für beide Fassungen gleicht der Audit den Sollbestand mit dem nativen
Import und mit sämtlichen sichtbaren Kopf-, Hals- und Haltebogenformen
in den drei SVG-Ansichten ab. Jede Quellmarke benötigt genau eine Form
je Ansicht. Fehlende, doppelte, zusätzliche und ungebundene Formen
verweigern den Inventarnachweis. Die unveränderte r2014-PDF-Probe
bestätigt die Hals- und Bogenkonturen in den zugehörigen PDFs.
[PDF-Nachweis aus r2014](final_stem_pdf_evidence_r2014.md)

Aus den belegten Inventaren ermittelt der Audit die Vereinigung der
Hals–Kopf- und Hals–Haltebogen-Paare beider Fassungen. Er erfasst jedes
dieser Paare, sobald sich mindestens ein Partner in der SVG-Geometrie
ändert, hinzukommt oder entfällt. Er behält auch Paare mit eigenem Kopf
für die Prüfung bei. Die Prüfmenge enthält damit mögliche Kontakte und
Abstandsfälle. Die Rollenprüfung verwendet keine musikalischen
Mindestabstände.

| Gegenprobe | Erwartung |
| --- | --- |
| Vollständige Inventare ohne geometrisch betroffenes Paar | Die ermittelte Paarmenge ist leer. |
| Inventar enthält einen Partner, die Vergleichsmenge lässt sein Paar aus | Die unabhängige erschöpfende Suche deckt die Abweichung auf. |
| Ein erwarteter Nachbar fehlt in beiden Fassungen | Der Inventarnachweis scheitert vor der Paarsuche. |
| Ein Rollenpaar tritt nur in einer Fassung auf | Die Vereinigung enthält das Paar. |

Die Rust-Tests führen diese Gegenproben aus. Der zusätzliche
MuseScore-Test liest zwei getrennt importierte Fassungen sowie gezielt
unvollständige SVG-Ansichten der gerenderten Fassungen zurück. Er folgt
den drei geprüften Rollen im qualifizierten SVG-Profil. Die allgemeine
PDF-Kopfprovenienz, geteilte Haltebögen, Seitenwechsel und der Beleg der
produktiven Teilfassung nach dem endgültigen Layout bleiben eigene
Prüfschritte.

```powershell
cargo +1.98.0 test --locked --bin etuschlopf musicxml_stem_source_anchor_across_two_imports -- --ignored --nocapture
```

`ETUSCHLOPF_PDF_RENDERER` bezeichnet für diese Zusatzprobe die lokale
MuseScore-Programmdatei.
