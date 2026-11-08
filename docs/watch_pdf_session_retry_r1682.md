# r1682 – Sitzungsgebundener PDF-Zugriff und gezieltes Wiederholen

ChangeClass: RefactorOnly. Parent ist der bereitgestellte r1681 / 0.10.1839;
seine native Annahme bleibt Voraussetzung. Die letzte hier geprüfte native
Annahme gehört zu r1680, dessen Beleg im Export
`assistant_export_latest(20261108-035922).zip` enthalten ist.

Die Reihenfolge wurde wegen der unveränderten Paketprüfung getauscht: r1681
enthält die Verbindungsanzeige und die eingefrorene PDF-Fehlerklasse; dieser
Take implementiert den PDF-Zugriff auf genau dieser unveränderten Definition.

## Eigenständig gewählter Take

Ein fehlgeschlagener PDF-Abruf blieb bisher als „PDF wird vorbereitet“ sichtbar.
Jeder weitere Snapshot startete einen erneuten Versuch. Der Browsercache war
nur an den Projektnamen gebunden. Bei einem späteren Lauf mit demselben Namen
konnte ein früheres PDF wieder erscheinen; sogar verspätete alte Antworten
erreichten die neue Ansicht. Diese Fälle wurden vor dem Patch synthetisch im
Browser nachgestellt.

PDF-Zustand und laufender Abruf sind jetzt an Sitzung und Projektname gebunden.
Beim Sitzungswechsel werden alte laufende Abrufe abgebrochen und die alten
Cachezuordnungen verworfen. Auch eine trotz Abbruch eintreffende Antwort muss
noch zum aktuellen Eintrag und zur aktuellen Sitzung passen. Erst dann darf
sie einen Blob-Link erzeugen oder einen sichtbaren Zustand ändern.

Die HTTP-Route lautet jetzt `/pdf-data/<kodierte Sitzung>/<kodierter Projektname>`.
Der bestehende Server prüft seine aktuelle Sitzungskennung vor dem Dateizugriff
und während des Wartens. Eine fremde oder unbekannte Sitzung erhält 409,
eine ungültige beziehungsweise alte ungebundene Route 400. Der Sitzungswert
ist ausschließlich Vergleichsdaten und wird nie als Dateipfad benutzt.
Die bisherigen Grenzen für Projektpfade, Symlinks, Frische, PDF-Kopf und
Abschlussmarker bleiben erhalten. Es gibt keine ungebundene Ersatzroute.
Bereits geöffnete Seiten älterer Versionen benötigen die aktuelle Watch-Seite;
der Launcher öffnet sie beim neuen Watch-Start.

## Bedienung

Das frühe Vorladen bleibt erhalten. Pro Sitzung/Projekt ist höchstens ein
Abruf aktiv. Ein Fehler erscheint als „PDF erneut laden“. Weitere Snapshots
starten keinen neuen Versuch; Klick, Eingabetaste oder Leertaste lösen einen
gezielten Versuch aus. Währenddessen ist die Aktion als beschäftigt markiert.
Leere Antworten, falscher Medientyp, HTTP- und Netzwerkfehler bleiben sichtbar
wiederholbar, ohne den musikalischen Lauf zu bewerten oder neu zu starten.

Der erfolgreiche Zustand ist ein normaler Link mit Blob-URL, neuem Tab und
`noopener`. Das Bedienelement wird bei asynchronen Zustandswechseln nicht
ersetzt; Fokus und aufgeklappte Details bleiben erhalten. Antwortende Abrufe
holen den Fokus nicht zurück. Erneutes Laden öffnet kein späteres Popup.
Vorher geladene Blob-URLs bleiben für bereits geöffnete PDFs bis zum Schließen
der Browserseite erhalten, werden aber keinem neuen Lauf zugeordnet.

## Belege und Grenzen

Zwei neue Rust-Proben benutzen den echten Loopback-HTTP-Handler: Ein frisches
synthetisches PDF ist mit passender Sitzung lesbar; falsche/unbekannte Sitzung,
alte Route und Pfadtraversal liefern keine PDF-Bytes. Kodierte Sitzungen bleiben
Daten, Unicode-Projektnamen bleiben zulässig.

`tools/etuschlopf_watch/tests/web_pdf.cjs` prüft neun Browser-Szenarien am
eingebetteten Produktions-HTML: frühes Vorladen und normale Strg-Klick-Links;
sichtbare Fehler ohne Snapshot-Retry; einfache Tastaturwiederholung mit
erhaltenem Fokus; kein Fokusdiebstahl; neue Sitzung mit gleichem Namen;
ignorierte alte Antwort trotz Abbruch; gleiche Labels bei getrenntem
Quellenfokus und ohne Aufklappen der Projektzeile; Netzwerk-/Typ-/Leerfehler.
Eine weitere Probe verdoppelt unabhängige Ressourcen, vertauscht ihre
Reihenfolge und prüft getrennte Fehler-/Wiederholungszustände.
PDF-Antworten sind synthetisch. Blob-Bytes, DOM, Tastatur und native Tab-Erzeugung
werden im echten Chromium geprüft. Der PDF-Viewer selbst und musikalische
Notenseiten werden damit nicht qualifiziert.

Vor Auslieferung werden die 40 Watch-Rust-Tests, die neun PDF-Proben,
neun Fokus-, zehn Titel- und sieben Netzwerkproben, strenges Watch-Clippy für alle Targets,
Strict-Build und die statische Take-Prüfung geprüft. Playwright ist eine
separate manuelle Testvoraussetzung, keine neue Produktabhängigkeit.
Die native Abnahme von r1682 bleibt offen. Musik-/Launcherquellen und deren
Schreib-, Freigabe- und Ergebnisentscheidungen bleiben unverändert.
