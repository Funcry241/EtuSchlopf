# Watch Web Project PDF Open

Version: 0.10.1737 r05

## Ziel

Die lokale Web-Watch verbindet den sichtbaren Projektnamen mit dem bereits erzeugten
Noten-PDF. Ein Klick auf eine Etuede oeffnet deren PDF in einem neuen Browser-Tab,
waehrend die Watch selbst offen und read-only bleibt.

Die Funktion ist reine lokale Artefaktprojektion. Sie erzeugt kein PDF, veraendert
keine Runtime-Wahrheit und fuehrt keinen Command zurueck in EtuSchlopf aus.

## Bedienung

In der Einzelprojektansicht ist der Etuedenname sowohl in der grossen Ueberschrift als
auch in der Projektkarte als dezenter PDF-Link erkennbar. In der kompakten `--all`-
Ansicht ist der Projektname jeder echten Project-Source anklickbar. Die Status-/Detail-
Flaeche der Korpuszeile bleibt davon getrennt und kann weiterhin auf- und zugeklappt
werden.

Sobald das aktuelle PDF im Browser bereitliegt, oeffnet der Projektnamen-Link eine
lokale Browser-Blob-URL in einem neuen Tab. Die Watch-Seite bleibt dadurch als
Laufuebersicht erhalten. Solange das PDF noch erzeugt wird, zeigt der kleine Indikator
`PDF …`; der Klick bleibt in dieser kurzen Phase bewusst ohne Navigation. Danach wird
er zu `PDF ↗`.

## Freshness und Dateigrenze

Die Web-Watch gibt nur ein PDF aus, das:

- exakt `<ProjectLabel>.pdf` im beim Watch-Start aktuellen Projektstamm ist,
- eine regulaere Datei innerhalb dieses kanonischen Projektstamms bleibt,
- seit dem Start der aktuellen Watch erzeugt oder aktualisiert wurde,
- mit dem PDF-Header `%PDF-` beginnt und im Dateiende ein `%%EOF` besitzt.

Ein altes PDF aus einem frueheren Lauf wird deshalb nicht als aktuelles Laufartefakt
angeboten. Pfadseparatoren, Windows-unzulaessige Dateizeichen und Steuerzeichen im
angeforderten Projektnamen werden abgewiesen. Es gibt keine freie Dateipfad- oder
Download-API.

Damit ein PDF auch nach dem planmaessigen Ende des Watch-Prozesses noch per Klick
geoeffnet werden kann, startet der Browser fuer jede sichtbare Project-Source bereits
waehrend des Laufs einen read-only Long-Poll auf `/pdf-data/<ProjectLabel>`. Sobald das
aktuelle PDF vollstaendig vorliegt, wird es als lokaler Browser-Blob gepuffert. Der
Blob-Link lebt mit der offenen Watch-Seite weiter, auch wenn MQTT/SSE/HTTP danach
planmaessig `CLOSED` sind. Es entsteht daraus keine EtuSchlopf-Persistenz; Schliessen
der Browserseite verwirft den Browser-Blob.

## Architektur

```text
Project SourceSnapshotV1
       ↓ ProjectLabel
Web projection
       ↓ read-only /pdf-data/<ProjectLabel>
Loopback HTTP route
       ↓ freshness + root boundary
<ProjectLabel>.pdf
       ↓
Browser Blob cache
       ↓
Browser PDF viewer (neuer Tab)
```

Die Route verwendet `ProjectLabel` nur als lokalen Artefakt-Locator. Sie erweitert
`SourceSnapshotV1` und `SessionSnapshotV1` nicht und fuegt dem MQTT-Vertrag kein
Artefaktfeld hinzu.

## Nicht geaendert

- keine Aenderung an MQTT Topic oder Payload
- keine Aenderung an UDP, Receiver oder SessionAggregator
- keine Aenderung an ObservationSession-/Source-State-Wahrheit
- keine Aenderung an Corpus-Scheduler oder Launcher
- keine Aenderung an PDF-Erzeugung oder Renderer-Auswahl
- kein Command Channel und keine Ruecksteuerung
- keine externe Cargo- oder Web-Abhaengigkeit
- keine Musik-, Writer-, Render- oder Regelmutation

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Danach:

```powershell
.\run.cmd --watch --strict "Etüde Nr. 6"
```

Nach erzeugtem PDF den sichtbaren Projektnamen anklicken. Fuer den Korpus gilt derselbe
Link pro Projektzeile bei:

```powershell
.\run.cmd --watch --strict --all
```
