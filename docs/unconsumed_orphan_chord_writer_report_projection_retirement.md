# UnconsumedOrphanChordWriterReportProjectionRetirement

Stand: v0.10.1630 r01  
Ausgangspunkt: v0.10.1629 r02

## Befund

`FinalOrphanChordHostRestRenderWriterStage` wurde innerhalb des finalen
PDF-Quellwriters als eigene Textzeile erzeugt und anschließend in
`SilentVoiceVisibleMeasureCoverageRenderWriteAudit.pipeline_line` eingehängt.
Außerhalb dieser Datei existiert jedoch kein Verbraucher für den Präfix:

- kein Finalreport-Producer,
- kein Korpusmanifest-Parser,
- kein Blocker-, Release- oder Exit-Gate,
- kein Writer- oder Mutationsrecht,
- kein Testvertrag auf den Berichtstext.

Die produktiv benötigten Orphan-Chord-Werte werden bereits typisiert in den
konsumierten Sammel-Outcome übernommen: Kandidaten, geplante und ersetzte
Forward-Dauer, ersetzte Forward-Nodes, reanchored Chord Notes,
Residualkandidaten, Write-Zustand und MusicMutation.

## Rückbau

Der Take entfernt ausschließlich die ungenutzte Berichtsprojektion und ihre
Folgeschuld:

- das private Feld `pipeline_line`,
- den Formatter für `FinalOrphanChordHostRestRenderWriterStage`,
- seine Einhängung in drei Sammelpfade,
- zwei Stage-Aufrufe in nicht selektierten beziehungsweise unlesbaren
  Quellzweigen, deren einzige Wirkung die pensionierte Textzeile war,
- zehn private Auditfelder und die zugehörigen Zuweisungen, deren letzter
  Verbraucher der entfernte Formatter war.

Es entsteht kein Ersatzbericht, Adapter, Shadowpfad oder neues Outcome-System.

## Erhaltene produktive Autorität

Unverändert bleiben:

- scoreweite Orphan-Chord-Kandidatenerkennung,
- exakte Bindung an den Master-Rest-Host,
- `apply_strict_staff_gap_materialization_authorized`,
- Raw-Cursor-, Pitched-Payload- und exakte Kandidaten-Readbacks,
- Kandidaten- und Write-Aggregation in der konsumierten Silent-Coverage-Zeile,
- Sink-Anforderung und Sink-Rollback,
- Reihenfolge vor Lost-Beam-, Clef-, Rest- und Higher-Beam-Verarbeitung,
- zentraler FinalNotation-Dateisink,
- MusicXML-, PDF-, Blocker-, Release- und Exitautorität.

## Zeilenbilanz

Physische Rust-Zeilen unter `src`:

- v0.10.1629 r02: 115.907
- v0.10.1630 r01: 115.811
- Netto: **-96**

Die betroffene Datei sinkt von 3.642 auf 3.546 Zeilen.

## Change-Klasse

`RefactorOnly`. Die produktive Writerlogik bleibt unverändert; entfernt werden
eine nicht konsumierte Reportprojektion und ausschließlich durch sie gehaltene
private Auditstruktur.

## Abnahme

Einmalig:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Kein Korpuslauf und kein `--force`. Erwartet sind Strict-Clippy/Build/Program/
FinalExit 0, versionsbereinigt identische Master- und Render-MusicXML sowie
sechs pixelidentische PDF-Seiten. Im Ergebnisbericht entfällt ausschließlich
die Zeile `FinalOrphanChordHostRestRenderWriterStage`; die drei konsumierten
Writerzeilen und alle numerischen Sammelwerte bleiben erhalten.
