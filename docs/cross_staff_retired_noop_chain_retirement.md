# Cross-Staff Retired No-op Chain Retirement

Stand: v0.10.1586 r02  
Ausgang: v0.10.1585 r01 / Export 525

## r02 – Compiler-Closure

`r01` wurde vor jeder Installation vom Strict-Clippy-Gate verworfen, weil der
nach dem Trial-Retirement ungenutzte Re-Export
`apply_bound_missing_rest_span_authorized` im zentralen Authority-Modul stehen
geblieben war. `r02` entfernt die vollständig unerreichbare Rest-Span-Fläche
komplett: Re-Export, private Funktion, Effektvariante und den nur dafür
benötigten Forward-Sequence-Parser. Der musikalische und renderbezogene
Outputvertrag bleibt unverändert.

## Befund

Die frühe Render-MusicXML-Kette enthielt nach dem weiterhin aktiven
`RenderXmlPianisticCrossStaffVoiceLedgerRewriteTrial` noch acht ausdrücklich
als `retirement boundary` bezeichnete Planer:

1. LedgerMaterialization
2. LedgerProofGap
3. RestForwardGapClosure
4. VoiceSplitHandRoleRecompute
5. BlueprintBlocker
6. VoiceRunBoundaryMapping
7. HandRolePdfRiskRecompute
8. VisualDensityGateAndDecision

Alle acht Builder erzeugten ausschließlich `Default`-Werte. Insbesondere
blieben sämtliche `rows` leer und
`materialization_candidates == 0`.

Trotzdem wurde anschließend
`CrossStaffVisualReviewOneSafeRestMaterializationTrial` ausgeführt. Vor dem
konstruktiv sicheren Ergebnis „kein Kandidat“ erledigte dieser Pfad jedes Mal:

- zwei vollständige `collect_render_xml_note_events`-Durchläufe,
- einen `collect_forward_spans_for_missing_rest_insert`-Durchlauf,
- einen `parse_musicxml`-Durchlauf,
- die Erzeugung von acht Retirement-Reportstrings,
- neun zusätzliche `pipeline_lines`-Einträge,
- eine stets falsche `.written`-Fortschreibung.

Die neun erzeugten Zeilen erscheinen im aktiven kompakten Ergebnisbericht von
Export 525 nicht. Sie besitzen keine Freigabe-, Musik-, PDF- oder
Reportprojektionsautorität.

## Unmöglichkeitsbeweis

Der alte Kontrollfluss war konstruktiv festgelegt:

```text
VisualDensityPlanner::default()
    -> rows = []
    -> materialization_candidates = 0
    -> candidate loop executes zero iterations
    -> candidate = None
    -> trial.written = false
```

Daraus folgte für jeden Lauf:

- kein Cross-Staff-Visual-Trial-Write,
- kein Wechsel des Render-XML-Checkpoints,
- die Behind-Bars-Stufe erhielt immer den bisherigen False-Branch-Quelltext,
- der Legacy-Render-Source-OR-Term war immer `false`,
- der PDF-Evidence-Ledger-Eintrag war immer inaktiv,
- der Render-Mutationszähler erhielt immer `+0`.

v0.10.1586 materialisiert genau diesen bewiesenen Endzustand, statt ihn in
acht Typen, acht Buildern und einem teuren Trial erneut auszurechnen.

## Gelöschte Struktur

Vollständig entfernt sind:

- acht pensionierte Planer,
- sieben ausschließlich dafür existierende Row-Typen,
- der RetiredPlannerSet-Transporttyp,
- sämtliche acht Retirement-Builder,
- Kandidatensuche und Apply-Funktion des unmöglichen Trials,
- der Trial-Typ, Action-Typ und Candidate-Typ,
- der Trial-Aufruf und alle stets falschen `.written`-Verzweigungen,
- die komplette Datei `cross_staff_visual_density_core.rs`,
- der aufrufbare Cross-Staff-Visual-Trial-Sink samt Writer und Fassade.

Das historische Thirty-Surface-Batch-Manifest bleibt unverändert. Für dessen
bestehenden Vertrag verbleibt ausschließlich ein privater, zero-field
`CrossStaffVisualTrialSinkCapability`-Marker. Er besitzt keinen Konstruktor,
keinen Writer, keine Fassade und keinen Consumer und kann daher kein
Schreibrecht erzeugen.

## Drei-Achsen-Bilanz

### Schönheit

Der Code sagt jetzt unmittelbar, was tatsächlich gilt: Nach dem aktiven
Voice-Ledger-Rewrite folgt direkt der Behind-Bars-Audit. Eine aus acht leeren
„Planern“ gebildete Scheinautorität existiert nicht mehr.

### Geschwindigkeit

Pro Projektlauf entfallen garantiert zwei Event-Scans, ein Forward-Scan, ein
MusicXML-Parse sowie mehrere Stringallokationen und Pipeline-Pushes. Der
Gesamtlaufgewinn wird erst durch die reale Windows-Abnahme gemessen; es wird
kein unbelegter Sekundenwert versprochen.

### Komplexität

- eine Rust-Datei weniger,
- acht Planertypen weniger,
- sieben Row-Typen weniger,
- ein Trial-/Action-/Candidate-Komplex weniger,
- ein aufrufbarer Writer-Sink weniger,
- neun tote Pipelinezeilen weniger,
- keine neue Abstraktion und kein Ersatzpfad.

## Outputvertrag

Unverändert bleiben:

- alle aktiven musikalischen Policies,
- alle erreichbaren Writer und ihre Reihenfolge,
- Master- und Render-MusicXML,
- PDF-Quellenauswahl,
- RestPlacement,
- Behind-Bars-, Cross-Staff-HandRole- und Lower-Rest-Lane-Stufen,
- formale Freigabeentscheidung.

Erwartete sichtbare Differenz gegenüber v0.10.1585 ist ausschließlich der
Versionsstring.

## Abnahme

Ein Lauf genügt:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Erforderliche Prüfung:

- Strict-Build und Strict-Clippy grün,
- `FinalExit 0`,
- Master-/Render-MusicXML gegenüber Export 525 nur Versionsstring,
- sechs PDF-Seiten pixelgleich,
- keine Referenz auf die pensionierten Planner-/Trial-Symbole im Quellbaum.
