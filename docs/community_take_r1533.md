# r1533 – Community-Einstieg

Version 0.10.1839, ChangeClass RefactorOnly. Einziger Parent ist der im
hochgeladenen Export hashgebunden angenommene r1532:
`f5ce7c1517cbfa8bca7afda10defaadfb089d310dddd4082268224f9c0f5ed00`.
Der eingebettete Source-Snapshot ist durch SHA-256
`b0642919be4b789f032e6558e63f82fb7559a59cb14da22f08ae1110b893333c` gebunden.

## Inhalt

Die README erhält vor dem unveränderten bisherigen Einstieg eine sachliche
Kurzbeschreibung, die Kennzeichnung als experimentelles Privatprojekt und
Dokumentationslinks. Neue deutsche Dokumente regeln Beiträge, Sicherheit,
Verhalten, Barrierefreiheit und Orientierung. Fehler- und Wunschvorlagen sowie
eine PR-Vorlage und ein Beschreibungstext sind enthalten.

Fehlerberichte sind jederzeit willkommen. Größere Änderungen werden vorab über
GitHub besprochen. Antwortzeiten, Fehlerbehebungen und die Betreuung älterer
Versionen werden nicht garantiert. Barrierefreiheit beschreibt vorhandene
Quellmerkmale und ausdrücklich offene praktische Prüfungen.

Die privaten GitHub-Sicherheitsmeldungen konnten über die verfügbare Integration
nicht geprüft oder aktiviert werden. Es wird keine Aktivierung behauptet. Ein
öffentlicher neutraler Kontaktwunsch darf keine Sicherheitsdetails enthalten.
Für Verhaltensmeldungen wird kein privater Sicherheitskanal zweckentfremdet.

## Paketgrenze und erwartete Wirkung

Alle neuen Dateien liegen unter `docs/`. Änderungen außerhalb davon betreffen nur
README, Take-Identität und die vorgeschriebene Änderungsdeklaration. Kein Rust-Code,
Test, Gate, Pin, Starter, Cargo-Manifest, qualifizierter Komponentenbaum oder
bestehender Vertrag wird verändert. Die Musikverarbeitung bleibt bytegleich.
Ein Root-Cargo.lock ist nicht Teil der eingehenden Take-Nutzlast.

Die Issue-Vorlagen werden zusätzlich als getrennte GitHub-Projektion geliefert.
Der `.github`-Ordner bleibt außerhalb der Take-Nutzlast. Eine vorhandene Root-Lizenz
im öffentlichen Git-Checkout bleibt dort erhalten. Einzelheiten stehen im
[Veröffentlichungsablauf](community/GITHUB_PUBLISHING.md).

## Validierung und Annahme

Der separate Übergabebericht nennt die tatsächlich ausgeführten Struktur-,
Deklarations-, Link- und Quellidentitätsprüfungen sowie die Ausführung der
unveränderten Parent-Zulassungslogik. Er ist kein nativer Acceptance-Beleg.
Ein neuer vollständiger Windows-/Renderer-Korpuslauf und die native Annahme
bleiben dem normalen Benutzerlauf vorbehalten. Erwartung: unveränderte
musikalische Ausgaben; tatsächliche XML-/PDF-Gleichheit ist frisch zu prüfen.

## Anwendung

Nur auf dem angenommenen r1532 und vor einem anderen Folgetake verwenden.
Bei einem neueren angenommenen Stand diesen Take zuerst neu aufsetzen.
Das eigentliche Take-ZIP bereitstellen und den gewohnten Aufruf verwenden:

```cmd
.\run.cmd --take --all
```

Das separate GitHub-Paket und der Assistant-Export sind keine Takes.
