# RegisterNotation Silent Bridge Final-Layout Re-Cue Projection Closure · r605

## Status

**Installiert und im Vollkorpus qualifiziert.** r605 ist der produktiv qualifizierte Musik-GreenBase dieses RegisterNotation-Abschlusses und basiert auf dem installierten r602 `RegisterNotationSilentBridgeVisibleRecueLastWriteClosure`.

r603 und r604 besitzen keine Authority:

- r603 wurde nach Installation beim Strict-Clippy-Gate wegen eines neu erzeugten
  `dead_code`-Gegenfalls zurückgerollt.
- r604 schloss diesen Clippy-Gegenfall, bestand Pre-Take, Clippy, Build und alle
  Rust-Tests, scheiterte aber im realen Etüde-1-Programmlauf am finalen
  MusicXmlExportGuard.
- Der Take-Mechanismus rollte r604 vollständig zurück; GreenBase blieb r602.

## Frische r604-Evidenz

Der r604-Lauf beweist die zeitliche Ursache:

```text
Clippy GREEN
Build GREEN
Rust-Tests GREEN
SchemaSchlopf r021 XSD40 GREEN

anschliessend:
OCTAVE_CONTINUE_RECUE_DUPLICATES_LAYOUT_RESTART
```

Die tatsächlich persistierte Etüde-1-Master-XML enthielt an den beiden finalen
Layout-Neustarts weiterhin:

```text
T11 new-system=yes + octave-shift continue + words 8va
T13 new-page=yes   + octave-shift continue + words 8va
```

Gleichzeitig meldete die Pipeline:

```text
HorizontalLayoutFinalNotationPlanningCutover:
SystemBreakChanges 6
```

Damit ist die r604-Früherkennung logisch am falschen Zeitpunkt:
RegisterNotation materialisiert die Re-Cues im frühen
`MasterNotationAndSalience`-Owner. Der endgültige Horizontal-Layout-Plan wird
erst später im Release-Owner ausgewählt und als letzte persistierte
MusicXML-Transformation materialisiert. Ein zu diesem frühen Zeitpunkt noch
nicht existierender `new-system`-/`new-page`-Marker kann daher keine verlässliche
Re-Cue-Entscheidungsquelle sein.

## Produktentscheidung

r605 behält die musikalische r602-Entscheidung vollständig:

```text
270 semantische Registerpläne
201 sichtbare Ottava-Spannen
201 Starts
12 Continues
201 Stops
```

Alle zwölf qualifizierten r283-Brücken erhalten zunächst weiterhin denselben
sichtbaren, darstellungsidentischen Re-Cue wie r602.

Die endgültige Sichtprojektion wird aber an die tatsächlich geschlossene
Horizontal-Layout-Authority gebunden:

```text
finaler Slot ist inner-systemisch
    -> Continue bleibt
    -> Words-Re-Cue bleibt

finaler Slot startet new-system/new-page und der Continue liegt vor dem ersten gepitchten Ereignis desselben Staffs
    -> Continue bleibt
    -> Linien-ID bleibt
    -> semantischer Scope bleibt
    -> zusätzlicher Words-Re-Cue wird im selben finalen Layout-Write entfernt
    -> Renderer-Wiederanzeige ist der sichtbare Carrier
```

Diese Projektion benutzt ausschließlich den bereits geschlossenen,
identity-freien `HorizontalLayoutMaterializationDirective` des konkreten
strukturellen Slots. Es entstehen keine Takt-, Etüden-, Pitch-, Staff- oder
Voice-Sonderfälle und keine neue Pause-Schwelle.

## Owner-/Last-Write-Grenze

r605 erzeugt ausdrücklich **keinen neuen nachgelagerten Reparatur-Writer**.

Stattdessen wird die layoutabhängige Re-Cue-Projektion in denselben bereits
autorisierten finalen Horizontal-Layout-Materializer integriert, der ohnehin
als letzte persistierte MusicXML-Transformation die geschlossenen
`new-system`-/`new-page`-Direktiven schreibt. Ein späterer inner-systemischer Continue im selben Restart-Takt bleibt davon ausdrücklich unberührt; ebenso beeinflussen bereits klingende Ereignisse des anderen Staffs diese Zuordnung nicht.

Damit gilt:

```text
RegisterNotationPlan
    -> 12 Continue + 12 sichtbare Re-Cues

Final HorizontalLayout DecisionPlan
    -> exakte new-system/new-page Wahrheit

StructuralWriteAuthority final materialization
    -> Horizontal-Layout schreiben
    -> im selben strukturellen Slot nur bei finalem Restart den generierten
       Continue-Words-Re-Cue entfernen
    -> keine zweite Persistenzschicht

SnapshotClose
    -> finale Bytes
```

## Fail-closed Final-Export-Gate

Der frühe `normalize_final_export_xml`-Vertrag bleibt r602-kompatibel und
verlangt weiterhin für jeden frisch materialisierten Continue genau einen
sichtbaren Re-Cue. Damit kann kein früher Pass einen Cue verlieren.

Erst der finale persisted-byte `MusicXmlExportGuard` arbeitet
layout-aware:

```text
inner-systemischer Continue
    -> exakt ein passender 8va/15ma/8vb/15mb-Words-Re-Cue erforderlich

Continue am finalen new-system/new-page-Wiedereintritt vor dem ersten gepitchten Ereignis desselben Staffs
    -> zusätzlicher Words-Re-Cue verboten

späterer Continue im selben Restart-Takt
    -> weiterhin exakt ein passender Words-Re-Cue erforderlich
```

Damit wird die Layoutwahrheit erst nach ihrer tatsächlichen Materialisierung
verwendet.

## Falsifikation

r605 ist verworfen, wenn:

- Clippy, Build oder Tests rot werden,
- T11/T13 in Etüde 1 weiterhin einen zusätzlichen Words-Re-Cue enthalten,
- ein inner-systemischer Re-Cue verschwindet,
- Continue-/Start-/Stop-Topologie oder Linien-ID verändert werden,
- XSD40 rot wird,
- eine zweite nachgelagerte Writer-Schicht erforderlich würde.

## Erwartung

Für Etüde 1:

```text
Continues insgesamt 2
finale Layout-Restart-Continues 2
inner-systemische Words-Re-Cues 0
```

Für den Vollkorpus auf dem bereits beobachteten r602-Layout:

```text
Continues insgesamt 12
finale Layout-Restart-Continues 5
inner-systemische Words-Re-Cues 7
```

Die Zahlen 5/7 sind ausschließlich erwartete Korpusbeobachtung und keine
produktive Identitätslogik. Die produktive Regel verwendet nur den finalen
strukturellen Layout-Directive des jeweiligen Slots.

## Autoritativer Qualifikationsabschluss

Der reale r605-Einzel- und Vollkorpuslauf hat die Falsifikationskriterien bestanden.

```text
FullCorpus                 10/10 GREEN
FinalExit                  0
Build                      GREEN
Strict-Clippy              GREEN
Rust-Tests                 1187/1187 GREEN
MusicXML 4.0 Master/Render 20/20 GREEN
SchemaSchlopf r021 XSD40   20/20 GREEN

Semantische Registerplaene 270
Sichtspannen                201
Starts                      201
Continues                    12
Stops                       201
```

Die 12 Continues zerfallen im finalen Layout beobachtet in genau zwei Carrier-Klassen:

```text
5 finaler new-system/new-page-Wiedereintritt
  -> Continue/Scope bleiben bestehen
  -> kein zusaetzlicher EtuSchlopf-Words-Cue
  -> Renderer-Wiederanzeige ist der sichtbare Carrier

7 inner-systemische Wiedereintritte
  -> Continue/Scope bleiben bestehen
  -> darstellungsidentischer sichtbarer Re-Cue bleibt erhalten
```

Die 5/7-Zahlen sind Korpusbeobachtung, keine produktive Identitaetslogik. Die produktive Entscheidung benutzt ausschliesslich die finale staff-lokale HorizontalLayout-Wahrheit.

Der PDF-Vergleich gegen r597 umfasst 51 Seiten. 45 Seiten bleiben pixelidentisch; 6 Seiten unterscheiden sich und tragen zusammen exakt die 7 beabsichtigten inner-systemischen Re-Cues. Etuede 10 Seite 6 besitzt wegen des neuen `8vb`-Cue eine deterministische vertikale Renderer-Neuverteilung, ohne Aenderung der System-/Seiten-Topologie oder weiterer MusicXML-Musiksemantik.

Damit ist die r605-Falsifikation geschlossen. Ein weiterer MusicMutation-Take aus diesem Thema ist nicht erforderlich.
