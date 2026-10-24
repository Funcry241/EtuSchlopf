# ConfusingTiedTailAccentSuppress Recovery Authority Retirement

## r02 – FinalReportProjection ReadOnly Evidence Binding Closure

Der r01-Compile stoppte mit E0425, weil die Rest-/Artikulations-Stage weiterhin korrekt ein read-only `TiedTailAccentSuppressionRenderWriteAudit` erzeugt, der Release-Orchestrator dieses Audit beim Destructuring aber mit `: _` verwarf. Die FinalReportProjection benoetigt dieselbe Retirement-Evidence weiterhin fuer die historische `CONFUSING-TIED-TAIL-ACCENT-SUPPRESS-RENDER-WRITE`-Sektion. r02 bindet ausschliesslich diese vorhandene read-only Evidence wieder an ihren Namen. Kein produktiver Suppressor-Writer, keine Suppression-Capability und kein `AccentVerticalPlacement`-Checkpoint wird wieder eingefuehrt.

## Ausgangslage

v0.10.1787 besass korpusweit drei produktive spaete Suppressor-Writes:

- Etuede 4: Producer `WavTruthBindingClearAccentRenderWrite`
- Etuede 6: Producer `AccentReserveTopSixMicroWriteCandidate`
- Etuede 7: Producer `WavTruthBindingClearAccentRenderWrite`

Alle drei setzten einen Accent auf einen eindeutig relationell gebundenen Tie-Tail, den `ConfusingTiedTailAccentSuppressRenderWrite` unmittelbar danach wieder entfernte.

## Upstream Closure in 1788

v0.10.1788 fuehrt einen gemeinsamen read-only `AccentAttackOwnershipPlan` in den beiden bestehenden Accent-Ownern ein. Er benutzt exakt die historische scoreweit eindeutige TieTail-Domain und blockiert nur den Accent, den der spaete Suppressor sonst wieder entfernt haette.

Einzelbeweise:

- Etuede 4: Suppressor 1 -> 0
- Etuede 6: Suppressor 1 -> 0
- Etuede 7: Suppressor 1 -> 0

Vollkorpus v0.10.1788 r02:

- 10/10 dunkelgruen
- `Written nein` fuer den Suppressor in 10/10
- `RemovedAccentRows 0` in 10/10
- mehrdeutige historische Locator-Faelle bleiben unveraendert und werden nicht als Write-Domain interpretiert

## Statischer Unerreichbarkeitsbeweis

Die historische produktive Domain war:

1. sichtbarer pitched Tie-Tail mit `tie stop`
2. exakt ein unmittelbar vorhergehender gleicher Pitch mit `tie start`
3. Attack ohne Accent
4. Tail mit Accent
5. scoreweit exakt ein solcher Kandidat

Vor der alten Suppressor-Grenze gilt nach 1788:

- Master-MusicXML besitzt keine Accent-Materialisierung.
- `AssertiveVisibleArticulationRenderWrite` wird in dieser Stage mit `user_gate_open=false` aufgerufen und schreibt nicht.
- `WavTruthBindingClearAccentRenderWrite` und `AccentReserveTopSixMicroWriteCandidate` sind die produktiven Accent-Additionsowner.
- Beide verwenden dieselbe `AccentAttackOwnershipPlan`-/TieTail-Eindeutigkeitsdomain und materialisieren den eindeutig verwirrenden Tail-Accent nicht mehr.
- Nach `AccentReserve` existiert kein Writer mehr, der einen Accent hinzufuegt. Spaetere Artikulationspfade koennen Accent nur entfernen bzw. Tenuto ergaenzen.

Damit gilt fuer die historische Recovery-Domain D:

`Image(P_remaining) ∩ D = ∅`

## Retirement

v0.10.1789 entfernt:

- produktiven `apply_bound_attack_owner_accent_suppression_render_write_candidate`
- source-bound Suppression-Seal/Materialize-Capability
- `apply_accent_suppression_batch_authorized`
- `AccentSuppressionBatchCapability`
- dedizierten `AccentVerticalPlacement`-Rendercheckpoint

Erhalten bleiben:

- der relationale TieTail-Selector als read-only gemeinsame Ownership-Wahrheit
- Report-Evidence als retired/no-write
- ein committed-state Detector fuer die historische *eindeutige* Write-Domain

Neuer Gate:

`CommittedUniqueTiedTailAccentCandidates = 0`

Mehrdeutige 2/3-Kandidatenfaelle liefern 0, weil die historische produktive Suppressor-Authority sie ebenfalls nie schreiben durfte.

## Abnahme

Einzelprobe:

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Dann:

```powershell
.\run.cmd --strict --all
```

Als `RefactorOnly` muss gegen v0.10.1788 r02 gelten:

- Master-MusicXML invariant nach Versionsnormalisierung
- Render-MusicXML semantisch invariant
- PDF 57/57 pixelidentisch
- 10/10 committed unique TieTail candidates = 0
