# r731 – PostCutoverAuditScaffoldingRetirementWave4

## Ausgangspunkt

Parent ist der real ausgefuehrte, dunkelgruene und released r730
`SourceSelectionExploratoryAuditRetirementDemandWitnessClosure`.

Wave 4 pensioniert drei historische Audit-Scaffolds, deren fachliche Erkenntnis
bereits in spaetere produktive Owners bzw. Cutovers ueberfuehrt und dort durch
produktive Regressionen gebunden wurde. Der Take fuehrt keine neue Musikregel,
keinen Writer, keinen Threshold und keine Release-Entscheidung ein.

## Pensionierte Audit-Inseln

### 1. Accidental-State-Clearance-Geometrie

r529 beobachtete die carried-state Rest/Accidental-Geometrie rein lesend.
r530 ueberfuehrte die positiv abgegrenzte Klasse in den bereits bestehenden
`final_notation_timeline_facts`-Owner und band positive sowie negative
Gegenbeispiele regressiv. Der r529-Audit besitzt seitdem kein exklusives
produktives Recht.

Pensioniert werden der alte Audit-Owner und seine reine Renderer-Readback-
Reportprojektion. Der produktive Clearance-Owner, seine Geometriebedingung,
Layout-Demand-Kette und Regressionen bleiben unveraendert.

### 2. Lower-Staff-Clef/Beam-Foreign-Truth

r534 beobachtete lesend vorhandene fremde Beam-Ketten ueber unteren
Clef-Grenzen. r535 schloss die bereits bestehende Clef-Segmentierung in der
produktiven Foreign-Truth-Trichotomie und band positive/negative Regressionen.

Pensioniert werden nur der r534-Audit und seine Reportprojektion. BeamPlan,
Clef-Entscheidung, Octave-Shift, Foreign-Truth-Preservation und produktive
Regressionen bleiben aktiv.

### 3. Oktavspannentopologie / klingende Zwischeninseln

r256/r510 waren rein lesende Vorstufen der spaeteren Registernotations-
Architektur. r511 ueberfuehrte die klingende Zwischeninsel als
`KlingendeZwischeninselKontinuitaet` in den bestehenden `RegisterNotationPlan`;
r578 konsolidierte die lokale Spannenauswahl weiter.

Der aktuelle RegisterNotation-Owner prueft selbst geschlossene Plan-Topologie
(`nullspannen`, Ereignisse ausserhalb der Spanne, systemfremde Ereignisse,
widerspruechliche Ueberlappungen), besitzt deterministische Planfingerprints
und bindet die Kontinuitaetsklasse regressiv. Der alte XML-Topologiebericht
besitzt kein Writer-, Repair-, Selection- oder Release-Recht.

Pensioniert werden der alte Audit-Owner und sein Reportabschnitt.
`RegisterNotationPlan`, Sichtzustandspfad, StructuralWriteAuthority,
ExportGuard und produktive Registerregressionen bleiben unveraendert.

## Grenzen

Unveraendert bleiben insbesondere:

- Pitch, Onset, Dauer, Staff, Voice und Hand;
- `final_notation_timeline_facts` und Clearance-Demand;
- Beam-/Clef-/Octave-Shift-Authorities;
- `RegisterNotationPlan` und seine Materialisierung;
- StructuralWriteAuthority;
- Renderer-/PDF-Writer;
- FinalReport-Pflichtsektionen;
- Take-, Recovery-, Change-Control- und Semantic-Capability-Gates.

Historische r256/r510/r529/r530/r534/r535/r511/r578-Dokumente bleiben als
Projektgeschichte erhalten. Entfernt wird nur die heute nicht mehr exklusive
Runtime-/Report-Scaffolding-Capability.

## ChangeClass

`RefactorOnly`.
