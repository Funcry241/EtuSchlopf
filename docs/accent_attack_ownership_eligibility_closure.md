# AccentAttackOwnershipEligibilityClosure

## r02 – ProofReentryReader Authority Placement Closure

Der r01-Kandidat wurde vor Apply vom Semantic-Capability-Diff abgewiesen, weil der geschuetzte `run_rest_and_articulation_render_stage()` als Verdrahtungspunkt der neuen Musikentscheidung veraendert worden war. r02 laesst diesen Orchestrator byteidentisch zu 1787 und berechnet dieselbe gemeinsame AttackEligibility jeweils innerhalb der bereits autorisierten Accent-Owner. Die WavTruth-Grenze projiziert vor ihrem Write die heutige kombinierte WavTruth-/Reserve-Entscheidung; die spaetere Reserve-Grenze prueft auf dem nach WavTruth committed XML dieselbe relationale TieTail-Eindeutigkeitsdomain. Der Downstream-Suppressor bleibt unveraendert als Sicherheitsbeweis bestehen.

## Version

`v0.10.1788 r01`

## Ausgangslage

Der autoritative Stand `v0.10.1787 r03` besitzt korpusweit drei produktive `ConfusingTiedTailAccentSuppressRenderWrite`-Writes. Writer-Traces beweisen zwei upstream Producer derselben falschen Wahrheit:

- Etüde 4: `WavTruthBindingClearAccentRenderWrite` setzt den Akzent auf einen gebundenen Tie-Tail; der unmittelbar folgende Suppressor entfernt ihn.
- Etüde 6: `AccentReserveTopSixMicroWriteCandidate` setzt den Akzent auf einen gebundenen Tie-Tail; der Suppressor entfernt ihn.
- Etüde 7: `WavTruthBindingClearAccentRenderWrite` setzt den Akzent auf einen gebundenen Tie-Tail; der Suppressor entfernt ihn.

Der gemeinsame Fehler ist fehlende Attack-Ownership-Evidenz vor dem Accent-Write. Ein pauschales `tie stop => kein Accent` wäre jedoch zu breit: In anderen Korpusprojekten existieren mehrere gebundene Tail-Akzente, die der bestehende Suppressor gerade wegen fehlender globaler Eindeutigkeit bewusst nicht verändert.

## Closure

1788 führt deshalb keinen neuen Writer ein. Vor den beiden bestehenden Accent-Ownern wird read-only ein gemeinsamer `AccentAttackOwnershipPlan` berechnet:

1. Die bisherige WavTruth-Top-12-Auswahl wird unverändert bestimmt.
2. Die bisherige AccentReserve-Top-6-Entscheidung wird so projiziert, wie sie nach dem bisherigen WavTruth-Write materialisiert worden wäre.
3. Auf dieser kombinierten geplanten Artikulationswahrheit wird exakt dieselbe relationale TieTail-Domain wie beim bestehenden Suppressor ausgewertet: sichtbarer gebundener Tail mit Accent, eindeutig angrenzender gleicher Attack-Owner mit `tie start`, Attack ohne Accent.
4. Nur wenn diese Domain scoreweit genau einen Kandidaten besitzt und dieser Kandidat von WavTruth oder AccentReserve neu geschrieben worden wäre, wird exakt dieser eine geplante Accent vor der Materialisierung gesperrt.
5. Alle Mehrdeutigkeitsfälle bleiben unverändert. Der bestehende `ConfusingTiedTailAccentSuppressRenderWrite` bleibt in 1788 produktiv vorhanden und dient als downstream Beweis: erwartet wird korpusweit `Written 0`.

Damit wird nicht die historische Fundstelle selektiert, sondern die bestehende relationale Suppressor-Domain upstream als gemeinsame Attack-Eligibility der beiden Producer benutzt.

## Erwartete Wirkung

Für die drei bewiesenen Gegenbeispiele:

```text
Etüde 4  WavTruth Tail-Accent:      geplant, aber vor Write gesperrt
Etüde 6  AccentReserve Tail-Accent: geplant, aber vor Write gesperrt
Etüde 7  WavTruth Tail-Accent:      geplant, aber vor Write gesperrt

ConfusingTiedTailAccentSuppress:
3 produktive Writes -> 0
```

Die finalen Render-Artefakte sollen unverändert bleiben, weil 1788 nur diejenigen drei Accent-Writes verhindert, die 1787 unmittelbar danach wieder entfernt. Der Suppressor wird in dieser Version ausdrücklich noch nicht pensioniert.

## Change Class

`MusicMutation` — die produktiven Accent-Owner ändern ihre Materialisierungsentscheidung, obwohl vollständige finale Output-Invarianz gegen 1787 erwartet wird.

## Erste Abnahme

Zuerst WavTruth-Pfad:

```powershell
.\run.cmd --take --strict --writer-trace=all "Etüde Nr. 7"
```

Danach AccentReserve-Pfad:

```powershell
.\run.cmd --strict --writer-trace=all "Etüde Nr. 6"
```

Erst danach Vollkorpus und direkter 1787->1788-Outputvergleich.
