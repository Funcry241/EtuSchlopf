# RendererNonzeroMissingOutputSettleBound

**Version:** v0.10.1750 r01  
**Change-Klasse:** RefactorOnly  
**Ausgangsstand:** v0.10.1749 r02

## Gemessene Ursache

Etüde Nr. 9 benötigt im reproduzierten v0.10.1749-Einzellauf rund 343 Sekunden
Programmlaufzeit. Die sichtbaren Solver-/Candidate-Pässe erklären davon nur ungefähr
12 Sekunden. Der finale Rendererabschluss verwendet `-f -o <pdf> <input>`, also Variante
8 von 11. Die sieben vorherigen Kommandovarianten können nach bereits beendetem
MuseScore-Prozess jeweils bis zu 45 Sekunden auf eine noch nicht vorhandene PDF warten.
Sieben Diagnosefenster ergeben 315 Sekunden und erklären damit nahezu vollständig den
projektspezifischen Laufzeitunterschied.

Etüde Nr. 1 verwendet dagegen bereits Variante 1 `-o <pdf> <input>` und besitzt diesen
Fallback-Stau nicht. Die Pausen-/Solver-Änderungen v0.10.1747 bis v0.10.1749 sind daher
nicht die Ursache.

## Änderung

Die Kommando-Reihenfolge bleibt unverändert. Insbesondere werden `-f`-Varianten nicht
nach vorne gezogen. Der 45-Sekunden-Settle-Vertrag bleibt ebenfalls unverändert, wenn

- der Renderer mit ExitCode 0 beendet wurde oder
- bereits beim Prozessende eine nichtleere PDF vorhanden ist.

Nur für den engeren Fall

```text
blocking Command::output beendet
+ ExitCode != 0 bzw. kein normaler ExitCode
+ am Prozessende kein PDF
```

wird das Warten vor der nächsten vorhandenen Fallback-Variante auf eine 3-Sekunden-Grace
begrenzt. Diese Grace bleibt bewusst erhalten, damit verzögerte Dateisichtbarkeit oder ein
renderer-eigener Helper-Prozess nicht sofort verworfen werden.

Zusätzlich projiziert der bestehende Rendererabschluss eine kompakte
`RendererFallbackTiming`-Zeile mit Anzahl der Versuche, vorangegangenen Fehlschlägen,
kurzen Nonzero-Waits, weiterhin langen Zero-Waits sowie Prozess- und Settle-Zeit vor dem
erfolgreichen Versuch. Diese Telemetrie besitzt kein Musik-, Writer- oder Freigaberecht.

## Unverändert

- Reihenfolge aller 11 MuseScore-Kommandovarianten,
- `-f` ausschließlich in den vorhandenen späten Kompatibilitäts-Fallbacks,
- Renderer-Erkennung und Executable-Auswahl,
- MusicXML-PDF-Quelle und PDF-Ausgabepfad,
- PDF-Vorab-Löschung, Struktur- und Freshness-Prüfung,
- Akzeptanz eines tatsächlich erzeugten PDFs auch bei Nonzero-Exit,
- Arrangement, Solver, Writer, MusicXML, MasterApply und musikalische Regeln,
- Takt-, Etüden-, Projekt-, Pitch- und Locatorauswahl.

## Abnahme

Zuerst ausschließlich Etüde Nr. 9:

```powershell
.\run.cmd --take --strict "Etüde Nr. 9" --display
```

Erwartung: Wenn die sieben vorgeschalteten Varianten wie vermutet Nonzero ohne PDF enden,
fällt deren kumulierte Settle-Zeit von maximal 315 Sekunden auf ungefähr 21 Sekunden.
`RendererFallbackTiming` muss den tatsächlichen Exit-/Wait-Pfad belegen. PDF, Master- und
Render-MusicXML werden anschließend gegen v0.10.1749/r02 verglichen. Ein Korpuslauf ist
für diesen Beweis nicht erforderlich.
