# r300 – PianisticReferenceAssumptionWitnessIntegration

## Zweck

r300 ist der erste echte modellrelative Wahrheitstest des neuen Spielbarkeitsstrangs. Er bindet die in r299 eingefrorene `REFERENCE_ASSUMPTION`-Autorität an die reale r296-Demand-Topologie von Etüde Nr. 6.

Das Ergebnis darf ausschließlich das explizite Referenzmodell falsifizieren oder bestätigen. `HumanPlayabilityClaim` bleibt `NOT_EVALUATED`.

## Warum vor der globalen Suche eine exakte statische Prüfung läuft

Der r296-Solver ist absichtlich vollständig und budgetiert. Für reale Attack-Gruppen kann eine naive Permutation aller zehn Hand/Finger-Slots jedoch bereits vor einer fachlich interessanten Stelle das State-Budget verbrauchen.

r300 führt deshalb keine heuristische Kürzung ein, sondern prüft zuerst eine **exakte notwendige Bedingung**:

> Für jede simultan aktive physische Tastenmenge muss mindestens eine Verteilung auf linke und rechte Hand existieren, bei der beide Ganzhand-Konfigurationen vom r299-Modell `SUPPORTED` werden.

Bei höchstens zehn aktiven Tasten sind höchstens `2^10 = 1024` beschriftete Links/Rechts-Partitionen zu prüfen. Für Etüde Nr. 6 liegt das reale Maximum bei sieben Tasten.

Scheitert diese vollständige Partitionsexhaustion an nur einer Boundary, kann unter dem r299-Modell global kein r296-v1-Witness existieren. Ein teurer Suchlauf ist dann logisch unnötig und wird nicht als Ressourcenabbruch missverstanden.

## Erwarteter erster Gegenfall in Etüde Nr. 6

Auf dem realen r299-Master wurde vorab unabhängig derselbe r292/r296-Zeit- und Obligationvertrag projiziert.

Erwartung:

- Score-Tick: `984`
- entspricht in der aktuellen Partitur Takt 3, Offset `216/384` (nur menschliche Diagnosezuordnung; keine Taktsonderlogik im Produktivcode)
- aktive MIDI-Tasten: `[35, 49, 59, 63]`
- Tonhöhen: B1, C#3, B3, D#4
- zu prüfende Links/Rechts-Partitionen: `16`
- unterstützte Partitionen unter r299: `0`
- minimale Zahl von r299-Span-Clustern: `3`
- verfügbare Hände: `2`

Die Ursache ist bereits im statischen Referenzmodell sichtbar: Jede einzelne Handkonfiguration darf dort höchstens eine Oktave äußere Spannweite besitzen. Die vier Tasten benötigen unter dieser Annahme drei getrennte Span-Cluster.

## Erwartetes Ergebnis

```text
ModelRelativeExecutionPlanStatus:
NO_REALIZATION_UNDER_REFERENCE_ASSUMPTION

GlobalSearchDisposition:
SKIPPED_EXACT_STATIC_NECESSARY_CONDITION_FAILED

IndependentWitnessReplay:
NOT_RUN

HumanPlayabilityClaim:
NOT_EVALUATED
```

Das ist ein **gültiges negatives Modellurteil**, weil kein heuristischer Suchfehlschlag verwendet wird. Die notwendige Bedingung wird vollständig erschöpft.

## Was das Ergebnis nicht bedeutet

r300 behauptet nicht:

- dass Etüde Nr. 6 für jeden Menschen unspielbar ist,
- dass eine reale Pianistin exakt die r299-Oktavannahme besitzt,
- dass der Score bereits geändert werden darf,
- dass Tempo, Sprungzeit oder dynamische Handbewegung schon bewertet sind,
- dass Takt 3 zwingend der einzige oder musikalisch wichtigste Engpass ist.

Es bedeutet ausschließlich:

> Unter dem expliziten r299-Referenzmodell kann für die aktuelle Master-Partitur kein vollständiger v1-Ausführungswitness existieren.

## Sicherheitsgrenzen

- `ChangeClass: AuditOnly`
- `WriterAuthority: none`
- `MusicMutation: no`
- `MasterApply: no`
- `RenderXmlWrite: no`
- `StaffIsHand: false`
- `VoiceIsHand: false`
- `EvidenceBasis: REFERENCE_ASSUMPTION`
- `ClaimBoundary: MODEL_RELATIVE_ONLY`
- `HumanPlayabilityAuthority: no`

## Entscheidung nach r300

Nach einem real grünen r300 wird **kein automatischer Folgetake** geplant.

Zuerst wird der Gegenfall pianististisch und musikalisch betrachtet:

1. Sind die vier gleichzeitig geforderten Tasten im aktuellen Master musikalisch wirklich beabsichtigt?
2. Ist das r299-Oktav-Limit für die Zielpianistin zu streng, plausibel oder sogar großzügig?
3. Gibt es eine reale Technik außerhalb des v1-Modells, etwa Handübernahme/Fingersubstitution/Sustain, die den Gegenfall auflöst?
4. Falls der Gegenfall auch menschlich plausibel ist: Welche musikalisch minimale Arrangementänderung wäre zulässig?

Erst diese Auswertung entscheidet, ob danach ein Obstruction-/Capability-/Arrangement-Take sinnvoll ist.
