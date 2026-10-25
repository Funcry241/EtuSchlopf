# Fremdkomponenten und Lizenzen

Der eigene EtuSchlopf-Code einschließlich der eigenen Hilfsprogramme steht unter
der [MIT-Lizenz](LICENSE). Diese Lizenz ersetzt keine Rechte oder Bedingungen
von Fremdkomponenten. Deren Lizenztexte und Urheberrechtshinweise bleiben gültig.

## Rust-Pakete

Die [Paketliste](docs/license_inventory.md) ordnet alle 189 unterschiedlichen
Registry-Paketversionen aus den vier `Cargo.lock`-Dateien ihren Lizenzen zu.
Sie umfasst auch optionale und plattformspezifische Abhängigkeiten. Der Hauptbaum
verwendet davon 44; die übrigen kommen durch die separaten Werkzeuge hinzu.

Die Originalarchive wurden gegen die SHA-256-Prüfsummen der Lockfiles geprüft.
Ihre Lizenz- und NOTICE-Dateien liegen unter [LICENSES/rust](LICENSES/rust).
Die [maschinelle Bestandsaufnahme](docs/license_inventory.json) enthält die
Prüfsummen, Quellarchive, Lizenztexte und die Zuordnung zu den Lockfiles.
`MIT/Apache-2.0` und `Unlicense/MIT` sind unverändert übernommene ältere
Paketangaben für alternative Lizenzen. Bei alternativen Lizenzen kann eine
zulässige Alternative gewählt werden; mit `AND` verbundene Bedingungen gelten
gemeinsam. Insbesondere bleibt bei `unicode-ident` die Unicode-Lizenz erhalten.

`nom-language 0.1.0` deklariert MIT, enthält aber keinen Lizenztext im Crate.
Der nachgereichte Originaltext stammt aus dem Repository-Commit
`2cec1b3e4c9ccac62c902d60c00de6d1549ccbe1`, den das Crate selbst nennt.
Die genaue Quelle steht im Inventar.

### MPL-Komponente im separaten Sensor

`webpki-roots 0.25.4` steht unter MPL-2.0 und gehört zur Abhängigkeit des
separaten WAV-Sensors. Es wird hier nicht unter MIT umgelizenziert. Sein
unveränderter Quellcode einschließlich MPL-Hinweisen ist hier erhältlich:

- [Original-Quellarchiv](https://static.crates.io/crates/webpki-roots/webpki-roots-0.25.4.crate)
- SHA-256: `5f20c57d8d7db6d3b86154206ae5d8fba62dd39573114de97c2cb0578251f8e1`
- [MPL-2.0-Lizenztext](LICENSES/rust/texts/6a70a4bf6b010016d59a.txt)

Wer eine daraus gebaute Sensor-Binärdatei weitergibt, muss die Empfänger auf die
Verfügbarkeit dieses MPL-Quellcodes hinweisen. Änderungen an MPL-Dateien müssen
ebenfalls unter MPL zugänglich sein. Die getrennten eigenen MIT-Dateien behalten
ihre Lizenz. Dieser Hinweis und die zugehörigen Lizenztexte gehören deshalb auch
in ein späteres Binärpaket.

## MusicXML und W3C

Die Dateien `musicxml.xsd` und `xlink.xsd` unter
`docs/normative_sources/musicxml/3.1/schema` und `4.0/schema` stammen aus
**MusicXML 3.1** beziehungsweise **MusicXML 4.0**. Copyright © 2004–2017
beziehungsweise © 2004–2021, Contributors to the MusicXML Specification.
Veröffentlicht durch die W3C Music Notation Community Group unter dem
[W3C Community Final Specification Agreement](LICENSES/W3C/w3c-fsa.txt).

Die jeweiligen `xml.xsd` sind die mit den Upstream-Versionen gelieferten
W3C-Schemas für den XML-Namensraum. Dazu werden die
[W3C Software License 2002](LICENSES/W3C/w3c-software.txt) und deren
[Software and Document License 2015](LICENSES/W3C/w3c-software-2015.txt)
mitgeführt. W3C hat die 2015-Fassung auch auf zuvor unter der 2002-Fassung
veröffentlichte Dokumente angewandt. Die Rechte an diesen W3C-Dateien bleiben
bei ihren jeweiligen Rechteinhabern.

Die sechs XSD-Dateien wurden für diese Lizenzkorrektur **nicht verändert**.
Ihre eingebetteten Herkunftshinweise bleiben erhalten. Die eigene
Schemaschlopf-Implementierung ist davon getrennt unter MIT lizenziert.

Quellen:

- [MusicXML 3.1](https://www.w3.org/2017/12/musicxml31/)
- [MusicXML 4.0](https://www.w3.org/2021/06/musicxml40/)
- [W3C FSA](https://www.w3.org/community/about/agreements/final/)
- [W3C Software and Document License 2015](https://www.w3.org/copyright/software-license-2015/)

## PDF-Diagnose: pypdfium2 und PDFium

Die separaten Python-Diagnosewerkzeuge verwenden `pypdfium2 5.14.0` anstelle
von PyMuPDF. pypdfium2: Copyright 2026 geisserml <geisserml@gmail.com>,
Apache-2.0 **oder** BSD-3-Clause. Seine Dokumentation und Beispiele stehen unter
CC-BY-4.0. PDFium und die im jeweiligen Binärbuild enthaltenen Fremdkomponenten
besitzen zusätzliche Lizenzen.

Diese Bibliotheken werden separat installiert; das Repository liefert keine
PDFium-Binärdatei oder Python-Wheels aus. Unter
[LICENSES/pypdfium2-5.14.0](LICENSES/pypdfium2-5.14.0) liegen die pypdfium2-Texte
und die Hinweise des hier geprüften Linux-x64-Wheels. Diese Linux-Hinweise sind
keine pauschale Freigabe anderer Plattformbuilds. Wer Wheels oder PDFium später
mitliefert, muss die vollständigen Lizenzhinweise **genau dieser Binärbuilds**
mitliefern. Der normale separate Installationsweg behält die im Wheel
enthaltenen Hinweise bei.

- [Upstream-Lizenzinformationen](https://pypdfium2-team.github.io/pypdfium2/readme.html#licensing)

Die früheren Diagnoseversionen mit PyMuPDF bleiben im Git-Verlauf sichtbar.
Für diese Kombination waren zusätzlich die AGPL- oder kommerziellen Bedingungen
von PyMuPDF maßgeblich. Die MIT-Lizenz eigener Dateien allein macht sie nicht
unzulässig, ersetzt aber auch nicht diese zusätzlichen Bedingungen. Die aktuelle
Diagnoseimplementierung importiert PyMuPDF nicht mehr.

## Basic Pitch

Das optionale Sensormodell stammt aus Spotify Basic Pitch, Copyright 2022
Spotify AB, Apache-2.0. Das Modell liegt nicht im Repository. Der bereits
vorgesehene Bootstrap lädt es separat und prüft seine festgelegte Prüfsumme.
Die Primärquelle wurde beim Audit gegen diese Prüfsumme geprüft.

- [Herkunft und Modellidentität](tools/wav_pitch_external_sensor_audit/BASIC_PITCH_NOTICE.md)
- [Apache-2.0-Lizenztext](LICENSES/Basic-Pitch-Apache-2.0.txt)
- [Upstream](https://github.com/spotify/basic-pitch)

Wer das Modell separat ausliefert, muss dessen Apache-Lizenz und anwendbare
Upstream-Hinweise erhalten. Eine eventuelle Spiegelquelle ändert diese Lizenz
nicht und ersetzt nicht den Identitätsnachweis.

## MuseScore, Eingaben und Ausgaben

MuseScore wird als separat installiertes Programm aufgerufen. Weder dessen
Programmdateien noch Quellcode werden hier mitgeliefert. Eine eigene
Weiterverteilung von MuseScore wäre gesondert nach dessen Lizenz zu behandeln.

Das Repository enthält den Programmquellbaum, keine WAV-/MIDI-Eingaben oder
PDF-/MusicXML-Ausgaben des Etüdenkorpus. Rechte an Musik, Aufnahmen und sonstigen
später hinzugefügten Inhalten müssen jeweils geklärt sein; die Softwarelizenz
erteilt dafür keine Rechte.

## Umfang und Wiederholung der Prüfung

Stand: 25.10.2026, Ausgangscommit
`5fce4153f10b8fac90b171e003eaf98a89ac1db3` plus die hier dokumentierte Änderung.
Die beiden neuen Lockfiles fixieren die zuvor separat aufgelösten
Sensor- und Schemaschlopf-Abhängigkeiten. Root-`Cargo.lock` bleibt unverändert.

Geprüft wurden Paketmetadaten, Archividentitäten, vorhandene Lizenztexte,
mitgelieferte Fremdschemas und die genannten externen Komponenten. Dies ist
keine Prüfung jeder Quellzeile auf fremde Herkunft und kein Nachweis der
Urheberschaft sämtlicher eigener Beiträge. Neue Abhängigkeiten, veränderte
Lockfiles und Binärpakete benötigen eine erneute Prüfung ihres konkreten Inhalts.
Ein separat verpackter nativer Take oder Assistant-Source-Snapshot muss seine
Lizenz- und Fremdhinweise ebenfalls vollständig mitführen; die Prüfung dieses
GitHub-Quellbaums bescheinigt nicht automatisch den Inhalt solcher ZIP-Pakete.
