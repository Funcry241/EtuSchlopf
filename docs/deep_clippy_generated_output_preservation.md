# DeepClippyGeneratedOutputPreservation

## Ausgangslage

Im akzeptierten v0.10.1719-Launcher laeuft die Root-Hygiene vor der Modusverzweigung. `RunRequest::DeepClippy` besitzt bewusst kein Musikprojekt als `input_arg()`. Dadurch war die Schutzmenge der Root-Hygiene leer und vorhandene Standardoutputs konnten als veraltetes Root-Residuum entfernt werden, obwohl `--clippy-deep` anschliessend keine Musikpipeline startet.

Betroffen waren insbesondere vorhandene `*.musicxml`, `*.render.musicxml`, `*.pdf`, `*.result.txt` und `*.strict-clippy.txt`-Artefakte.

## Ziel

`--clippy-deep` bleibt ein diagnostischer Launcher-Modus. Er darf seinen eigenen Deep-Clippy-Log neu schreiben und normale Root-Hygiene fuer ZIP-, Console- und Temp-Residuen ausfuehren, aber keine bereits erzeugten Standardoutputs entfernen.

## Umsetzung

Die Startup-Root-Hygiene besitzt nun eine kleine typisierte Policy:

- `CurrentTarget` fuer Take- und normale Execute-Pfade,
- `PreserveGeneratedOutputs` fuer `RunRequest::DeepClippy`.

Unter `PreserveGeneratedOutputs` behandelt die bestehende Root-Hygiene jede Datei, die sie sonst als Standardoutput klassifizieren wuerde, als geschuetzt. Die bestehende Zielschutzlogik fuer normale Projekt-, `--all`- und Visual-Preflight-Laeufe bleibt unveraendert.

Es gibt keine Etueden-, Takt- oder Dateinamen-Sonderregel. Der Schutz verwendet ausschliesslich die bereits vorhandene generische Standardoutput-Klassifikation.

## Beweis

Ein fokussierter Launcher-Test erzeugt exemplarisch:

- `notation_test.result.txt`,
- `notation_test.strict-clippy.txt`,
- `notation_test.musicxml`,
- `notation_test.render.musicxml`,
- `notation_test.pdf`,
- sowie ein altes `EtuSchlopf_v...zip`.

Die Deep-Clippy-Startup-Hygiene muss alle Standardoutputs erhalten und das alte ZIP weiterhin entfernen. Damit wird bewiesen, dass nicht die Root-Hygiene insgesamt deaktiviert wurde.

## Unveraendert

- `--strict --all` und dessen projektweiser Freshness-Purge,
- normale Einzelprojektlaeufe,
- Visual Preflight,
- Deep-Clippy-Lints, Targets und Cargo-Aufruf,
- Musikpipeline und Musiklogik,
- Writer-/Materialisierungsgrenzen,
- PDF-Renderer,
- Ergebnis- und Release-Gates.

## Sicherheitsbindung

Der Launcher ist FNV-gepinnt. Deshalb werden der Launcher-FNV im Build-Policy-Owner sowie die zugehoerigen Trust-Anchor-Pins synchronisiert. Die Root-Hygiene selbst bleibt Security-Code; der Take ist `SecurityOnly`.

## Abnahme

```powershell
.\run.cmd --take --strict --all
.\run.cmd --clippy-deep
```

Nach dem zweiten Befehl muessen die zuvor vorhandenen Projekt-PDFs/MusicXML/Result-Artefakte weiterhin vorhanden und inhaltlich unveraendert sein; nur der Deep-Clippy-Diagnoselog darf neu erzeugt werden.
