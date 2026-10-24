# r295 PianisticConfigurationConstraintKernelDirectAuthorityClosure

## Anlass und Parent

r293 `PianisticConfigurationConstraintKernel` wurde ausgeliefert, scheiterte im realen Windows-Reexec jedoch unter `#![deny(dead_code)]` mit 21 Fehlern in der neuen Constraint-API. Der Take-Bootstrap rollte danach korrekt auf den vorherigen real installierten Stand r292 `PianisticExecutionScoreAuthorityChangeControlClosure` zurueck.

r294 `PianisticConfigurationConstraintKernelRuntimeAdoptionClosure` wurde anschliessend zu Unrecht als Parent=r293 paketiert. Da r293 nie installiert war, erkannte Take-Change-Control beim realen r292-Tree die drei r293-Basisdateien korrekt als undeclared und blockierte vor Installation.

r295 verwendet weder r293 noch r294 als Autoritaet. Er basiert direkt auf dem real installierten r292 und fuehrt den statischen Constraint-Kern samt Runtime-Adoption in einem einzigen vollstaendig deklarierten AuditOnly-Schnitt ein.

## Statischer Constraint-Kern

Der read-only Owner `PianisticConfigurationConstraintKernel` fuehrt ein:

- `KeyboardGeometry` fuer die Standard-88-Tasten-Klaviatur MIDI 21..108
- explizite `Hand`- und `Finger`-Werte
- ganze `StaticConfigurationRequest`-Konfigurationen
- partielle `PianistCapabilityEnvelope`-Evidenz
- die drei Entscheidungen `SUPPORTED`, `REJECTED`, `UNMODELED`

Harte Grenze:

`Staff != Hand`, `Voice != Hand`, `Hand != Finger`.

Der Score wird in r295 weder einer Hand noch Fingern zugeordnet. Staff/Voice dienen nicht als koerperliche Wahrheit.

## Keine Max-Span-Heuristik

Capability wird nicht aus einer einzelnen Spanne, paarweisen Fingerreichweiten oder einer gewichteten Summenheuristik abgeleitet. Die Frage bezieht sich auf die gekoppelte ganze Finger/Tasten-Konfiguration. Nicht vorhandene Capability-Evidenz ist `UNMODELED`, nicht automatisch `SUPPORTED` oder `REJECTED`.

Mehrfachnutzung eines Fingers beziehungsweise mehrere Finger auf derselben Taste werden in v1 nicht als unmoeglich behauptet, sondern bleiben `UNMODELED`.

## Runtime-Adoption statt dead_code-Ausnahme

r295 verwendet kein `#[allow(dead_code)]` und schwaecht `#![deny(dead_code)]` nicht ab. Die bestehende Auditsektion fuehrt eine deterministische `KernelRuntimeIntegrity`-Selbstpruefung aus und benutzt dadurch den realen Kernelpfad im Nicht-Test-Build.

Geprueft werden:

- MIDI-Grenzen 21..108
- exakt 88 Tasten
- 52 weisse / 36 schwarze Tasten
- beide Handwerte
- alle fuenf Fingerwerte
- explizite Positivkonfiguration -> `SUPPORTED`
- explizite Negativkonfiguration -> `REJECTED`
- nicht abgedeckte valide Konfiguration -> `UNMODELED`
- Evidence-ID und Authority-Bindung der synthetischen Integrity-Envelope

Die Envelope `internal-kernel-integrity-contract-v1` mit Provenance `synthetic-runtime-integrity-evidence-not-a-pianist-profile` ist ausschliesslich interne Selbstpruefung. Sie ist kein reales oder Referenz-Pianistenprofil und wird nicht an Score-Ereignisse gebunden.

## Epistemische Grenze

Fuer Etuede 6 bleibt:

`StaticCapabilityClaim NOT_EVALUATED`

Weiterhin keine:

- scoreweite Hand-/Finger-Enumeration
- Reach- oder Transition-Aussage
- Difficulty-Aussage
- ExecutionPlan
- Playability-Aussage
- Tempo-Frontier
- Reparatur
- Writer- oder Musikmutation

## Regressionen

Gegenueber r292 kommen elf allgemeine Kernel-Regressionen hinzu: die zehn statischen Konfigurations-/Capability-Tests des urspruenglichen Kernels sowie `runtime_kernel_integrity_contract_is_complete_and_nonclaiming`.

Ausgehend von r292 mit 1015/1015 Tests ist der erwartete integrierte Stand 1026/1026.

## Revisionsfolge

r293 bleibt eine ausgelieferte, wegen Reexec Exit 101 nie installierte Revision. r294 bleibt eine ausgelieferte, vom Take-Change-Control vor Installation blockierte Revision. Beide sind historische Kandidaten, keine Parent-Autoritaeten.

Der naechste fachliche Schritt nach real gruener r295-Verifikation ist r296 `PianisticExecutionPlanWitness`.
