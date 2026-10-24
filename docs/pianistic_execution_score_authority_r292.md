# r292 – PianisticExecutionScoreAuthorityChangeControlClosure

> **Change-Control-Closure:** Der zuvor ausgelieferte r291-Kandidat wurde vor Installation abgewiesen, weil der deklarierte incoming FNV64 für `src/audit_only/pianistic_execution_score.rs` nicht dem tatsächlich gepackten Dateiinhalt entsprach. r291 wurde nie installiert und ist kein Parent. r292 enthält denselben fachlichen AuditOnly-Schnitt direkt auf r290; alle eingehenden FNV64-Bindungen wurden aus den final gepackten Bytes neu berechnet und vollständig gegengeprüft.


## Elternstand

r292 basiert direkt auf dem real installierten, corpusweit 10/10 grünen und auf 50 PDF-Seiten visuell abgenommenen r290 `RendererFeasibilityPageCountFrontierPreferenceOwnerReuseAbschluss`.

r290 schließt den Registernotations-/Renderer-Feasibility-Strang im aktuellen Beweisumfang ab. Das neue offene Risiko ist eine andere Frage: Die vorhandenen lokalen HAN-/Piano-Warnungen beweisen nicht, dass für die vollständige Klavierfassung tatsächlich ein zusammenhängender menschlicher Spielplan existiert.

r292 beginnt diesen neuen Strang ausdrücklich **ohne Spielbarkeitsurteil und ohne Musikmutation**.

## Fachlicher Schnitt

Der neue read-only Owner `PianisticExecutionScoreAuthority` projiziert ausschließlich die bereits erzeugte Master-MusicXML in einen später beweisbaren Ausführungsgegenstand.

Er besitzt in r292 nur:

- exakt auf 96 Ticks pro Viertel normalisierte musikalische Zeit;
- Tempo-Punkte aus `<sound tempo="...">`;
- deterministisch daraus abgeleitete physische Zeit in Mikrosekunden (floor-Projektion; die exakte Autorität bleiben Tick + TempoMap);
- gepitchte MusicXML-Tonsegmente;
- Sound-Tie-Semantik über `<tie type="start|stop">`;
- daraus zusammengeführte Attack-/Hold-/Release-Ereignisse;
- getrennte Fingerprints für klingende Ausführungswahrheit und Staff-/Voice-Herkunftsaffinität.

Ein gebundenes Folgesegment mit `tie type="stop"` erzeugt **keinen zweiten Attack**. Es verlängert ausschließlich Hold und Release des bereits offenen Ausführungsereignisses.

## Harte Autoritätsgrenze

r292 definiert ausdrücklich nicht:

- welche Hand ein Ereignis spielt;
- welchen Finger ein Ereignis spielt;
- ob Staff 1 rechte Hand oder Staff 2 linke Hand bedeutet;
- ob Voice eine Hand besitzt;
- Handspanne;
- Sprunggrenze;
- Difficulty Score;
- PianistProfile;
- CapabilityModel;
- TransitionCapability;
- Feasibility;
- Reparaturkandidat;
- MusicXML-/Master-/Render-Writer.

Daher lautet der einzige zulässige Spielbarkeitsstatus in r292:

```text
PlayabilityClaim NOT_EVALUATED
```

`source_staff` und `source_voice` bleiben Provenienz-/Notationsaffinitäten. Der musikalische Ausführungsfingerprint ignoriert diese Affinitäten; ein separater SourceAffinity-Fingerprint bewahrt sie als Herkunftsevidenz. Damit wird die spätere physische Wahrheit nicht heimlich an `Staff == Hand` oder `Voice == Hand` gebunden.

## Fail-closed Modellgrenze

Die Projektion erfindet keine Zeit oder Tie-Semantik. Sie wird als `MODEL_INPUT_UNAVAILABLE` ausgewiesen, wenn unter anderem:

- die Master-MusicXML nicht lesbar oder nicht `score-partwise` ist;
- nicht exakt ein Klavier-Part vorliegt;
- ein gepitchter Grace-Ton ohne explizit modellierte Dauer vorkommt;
- eine Zeitangabe nicht exakt auf die kanonischen 96 Ticks pro Viertel projiziert werden kann;
- die TempoMap leer ist oder den Score-Ursprung nicht abdeckt;
- widersprüchliche Tempi am selben Tick liegen;
- Tie-Stops verwaist, Tie-Starts offen oder Tie-Ketten zeitlich nicht lückenlos sind.

Wichtig: Auch ein unvollständiger Projektionseingang erzeugt **kein Urteil `unspielbar`**. r292 besitzt noch keine solche Capability.

## Korpus-Vorabprüfung des r290-Materials

Der aktuelle r290-Export wurde außerhalb des produktiven Rust-Laufs mit derselben Projektionstopologie gegengeprüft. Alle zehn Master-MusicXMLs erfüllen die r292-Grundannahmen:

- genau ein Part;
- 96 Divisions pro Viertel;
- TempoMap ab Tick 0;
- geschlossene Tie-Ketten;
- keine gepitchten Grace-Notes.

Erwartete Grundzählungen:

| Projekt | Tonsegmente | Attack-Ereignisse | Tie-Fortsetzungen | Tempo BPM |
|---|---:|---:|---:|---:|
| Etüde Nr. 1 | 511 | 477 | 34 | 119 |
| Etüde Nr. 2 | 1066 | 1001 | 65 | 125 |
| Etüde Nr. 3 | 872 | 808 | 64 | 125 |
| Etüde Nr. 4 | 872 | 831 | 41 | 127 |
| Etüde Nr. 5 | 1495 | 1425 | 70 | 126 |
| Etüde Nr. 6 | 1303 | 1237 | 66 | 126 |
| Etüde Nr. 7 | 1118 | 1035 | 83 | 125 |
| Etüde Nr. 8 | 1049 | 948 | 101 | 128 |
| Etüde Nr. 9 | 1125 | 1060 | 65 | 127 |
| Etüde Nr. 10 | 1151 | 1061 | 90 | 130 |

Diese Zahlen sind **Voraberwartungen**, nicht Ersatz für den echten Windows-Take-/Projektlauf.

Für Etüde 6 ist insbesondere zu erwarten:

```text
PitchedNoteSegments 1303
AttackEvents 1237
HoldObligations 1237
ReleaseObligations 1237
TieContinuationSegments 66
CanonicalTicksPerQuarter 96
TempoMap Points 1
PlayabilityClaim NOT_EVALUATED
```

## Regressionen

r292 fügt acht allgemeine Rust-Tests hinzu:

1. Ein einfacher Ton erzeugt genau Attack/Hold/Release und korrekte physische Zeit.
2. Eine Tie-Kette verlängert Hold/Release ohne zweiten Attack.
3. Akkordnoten teilen den Attack-Zeitpunkt und verschieben den Cursor nicht mehrfach.
4. `backup` erhält polyphone Score-Zeit korrekt.
5. Ein Tempowechsel wird deterministisch integriert.
6. Nicht exakt auf 96 Ticks projizierbare Divisions werden nicht gerundet, sondern fail-closed ausgewiesen.
7. Staff-/Voice-Umetikettierung ändert nicht den musikalischen Execution-Fingerprint, wohl aber den getrennten Affinity-Fingerprint.
8. Ein verwaister Tie-Stop wird fail-closed ausgewiesen und erzeugt keinen Playability-Claim.

Ausgehend von 1007/1007 in r290 lautet die vorab festgelegte Erwartung **1015/1015**.

## Prüfreihenfolge nach Installation

1. `run.cmd --take "Etüde Nr. 6"`
2. Wenn Take-Gate, Clippy, Build und Tests grün sind: den neuen `PIANISTIC-EXECUTION-SCORE-AUTHORITY-AUDIT` in Etüde 6 prüfen.
3. Erwartung Etüde 6: `ProjectionStatus COMPLETE`, 1303 Tonsegmente, 1237 Attack-Ereignisse, 66 Tie-Fortsetzungen, `PlayabilityClaim NOT_EVALUATED`.
4. Die `.musicxml`, `.render.musicxml` und PDF müssen gegenüber r290 musikalisch/visuell unverändert bleiben; r292 ist AuditOnly.
5. Noch **kein Vollkorpus nötig**, bevor Etüde 6 den neuen Grundvertrag real bestätigt. Der folgende r293 darf erst danach das Capability-/Konfigurationsmodell einführen.

## Nächster geplanter Strang

r292 ist nur Fundament. Der aktuell geplante Gesamtstrang bleibt:

```text
r292 PianisticExecutionScoreAuthorityChangeControlClosure
r293 PianisticConfigurationConstraintKernel
r294 PianisticExecutionPlanWitness
r295 PianisticObstructionAndTempoFrontier
r296 PianisticCorpusFalsification
```

Bis einschließlich r296 ist keine Musikmutation vorgesehen.
