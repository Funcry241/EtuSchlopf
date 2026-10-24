# NonProjectedPostRenderReadabilityRetentionAuditRetirement

## Ausgangslage

Der reale Lauf von v0.10.1593 r02 / Export 533 ist grün bestätigt:
Strict-Build und Strict-Clippy liefen ohne Warnung, `FinalExit` ist 0,
Master- und Render-MusicXML unterscheiden sich vom vorherigen Stand nur im
Versionsstring und alle sechs PDF-Seiten sind pixelidentisch. Der externe
MuseScore-Prozessabgang trat erst nach gültiger PDF-Erzeugung auf und bleibt
deshalb die bekannte hellgrüne Fremdursache.

Der typisierte Finalreport konsumiert den historischen `pipeline_lines`-Vektor
nicht mehr. Die in diesem Take entfernte Familie berechnete dennoch weiterhin
mehrere vollständige XML-, Event-, Pausen- und Retention-Projektionen und
schrieb ihre Ergebnisse ausschließlich in diesen verworfenen Sink.

## Entfernte Auditfamilie

Vollständig entfernt werden:

- `PianisticNotationQualityAudit` einschließlich Clef-/Octave-/Rest-Toplisten,
- `RestGlyphVisualEnvelopeTruthAudit`,
- `ExternalNotationFeedbackReadabilityAudit`,
- die zweite, rein berichtende PianistHelpful-Articulation-Projektion,
- `FragmentedRestChainLocalInvariantRepairCandidateAudit`,
- `RetentionShadowHarnessEventGroupAudit`,
- die dazugehörigen Row-, Event-, Builder-, Formatierungs- und Transporttypen,
- historische Sink-Zeilen ohne Finalreport-, Writer- oder Releaseconsumer.

Damit entfallen pro Projekt zusätzliche MusicXML- und Atomic-Note-Scans,
Clef-/Octave-/Rest-Auswertungen, Pausenglyph-Klassifikation, Retention-Gruppierung,
Sortierung, Hash-/String-Key-Aufbau und umfangreiche reine Berichtformatierung.

## Produktive Artikulationsgrenze

Der produktive Single-Clear-Staccato-Writer verwendet weiterhin dieselbe
Kandidatenklassifikation. Sein bisheriger Builder berechnete jedoch zusätzlich
eine vollständige Auditwelt aus bestehenden Artikulationen, Dichtekennzahlen,
Posture-Klassen, Parserstatus und einer langen `pipeline_line`.

Der Vertrag ist nun auf die tatsächlich benötigte Wahrheit reduziert:

```text
Render-MusicXML
    -> höchstens zwölf identisch klassifizierte und sortierte Kandidaten
    -> Single-Clear-Staccato-Writer
```

Erhalten bleiben unverändert:

- dieselben Klassen und Scores,
- dieselben Anker-/Placement-Bedingungen,
- dieselbe Sortierung und Begrenzung auf zwölf Zeilen,
- dasselbe Filterkriterium für den einzigen klaren Staccato-Kandidaten,
- der produktive Writer und sein Placement-Readback.

Entfernt werden ausschließlich seine nicht mehr projizierten Dichte-, Posture-
und Berichtberechnungen. Der Builder besitzt kein eigenes Reportfeld mehr.

## Erhaltene produktive Autoritäten

Unverändert bleiben insbesondere:

- der sichtbare Pausen-/Raw-Cursor-Hard-Gate,
- die produktiven Rest-, Artikulations- und Retention-Writer,
- `VoiceFiberBehindBarsDecisionLedger`,
- der produktive P2-Retentionpfad,
- `FragmentedRestChainTransactionalWriteTrial` und seine mögliche
  Render-MusicXML-Materialisierung,
- MusicXML- und PDF-Quellenwahl,
- typisierter Finalreport,
- Blocker-, Release- und Exitentscheidung,
- sämtliche musikalischen Regeln, Gewichte und Passreihenfolgen.

## Authority Map

Vorher:

```text
produktive Writer/Entscheidungen
    -> mehrere erneute XML-/Event-Scans
    -> historische Auditobjekte
    -> pipeline_lines.push(...)
    -> keine Projektion und kein Consumer
```

Nachher:

```text
produktive Writer/Entscheidungen
    -> direkt nächste produktive Pipelinephase
```

Der Artikulationspfad lautet nun:

```text
Render-MusicXML
    -> candidate-only articulation classification
    -> SingleClearStaccatoRenderWriteWithPlacementReadback
```

## Drei-Achsen-Bilanz

### Schöner

Die produktive Kandidatenentscheidung ist nicht länger mit einer zweiten,
unsichtbaren Qualitäts- und Posture-Erzählung vermischt. Die Stage-Schnittstelle
transportiert keine Auditobjekte ohne Consumer mehr.

### Schneller

Mehrere vollständige XML-/Event-/Atomic-Note-Scans, Toplisten, Sortierungen,
Retention-Gruppierungen und reine Formatierungen entfallen garantiert. Aus
einem einzelnen Gesamtlauf wird bewusst kein belastbarer Sekundenwert
abgeleitet.

### Weniger komplex

Sechs geschlossene Auditfamilien und ihre Transportflächen verschwinden ohne
Ersatzabstraktion, Shadowpfad oder neuen Sink. Der Artikulationsbuilder besitzt
nur noch den tatsächlich verwendeten Kandidatenvertrag.

## Zeilenbilanz

Physische Rust-Zeilen unter `src`:

- v0.10.1593: 121.406
- v0.10.1594: 120.326
- Netto: **-1.080**

Dateibilanz der sieben produktiven Änderungen:

- Orchestrator: 4.939 -> 4.864
- Foundation-Tail: 130 -> 122
- Pianistic-Quality-Core: 759 -> 398
- Articulation-Readback-Tail: 2.240 -> 1.927
- Rest-Glyph-Topology-Tail: 1.149 -> 1.065
- Audio-Salience-Conflict-Tail: 1.896 -> 1.804
- Retention-Core: 416 -> 269

## Beweisgrenze

Dieser Take verändert keine musikalische Policy und keinen Writervertrag. Die
reale Abnahme benötigt deshalb genau einen Strict-Take-Lauf. MusicXML und PDF
müssen gegenüber v0.10.1593 bis auf den Versionsstring identisch bleiben.
