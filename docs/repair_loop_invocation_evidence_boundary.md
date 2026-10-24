# Repair Loop Invocation Evidence Boundary

## Scope

`v0.10.1687 r01` ist Take 1 des auf maximal drei Takes begrenzten
`RepairLoopOwnershipClosure`-Superwurfs. Der Ausgangsstand `v0.10.1686` ist
korpusautoritativ bestaetigt: 10/10 Projekte OK, 0 Blockerprojekte,
0 Warnprojekte, BuildExit 0, ClippyExit 0, FinalExit 0, RunSeverity dark_green
und ReleaseDecision released.

Dieser Take aendert keine Musikregel und entfernt noch keinen Repair. Er macht
ausschliesslich drei bereits produktiv vorhandene Repair-Schleifen getrennt
messbar, deren Einzelinvocations bisher durch spaetere Audit-Ueberschreibung
nicht vollstaendig im Ergebnisbericht erhalten blieben.

## 1. Shadow-Rest / StrictStaff / Shadow-Rest

Die produktive Reihenfolge bleibt exakt unveraendert:

`UpperStaffShadowRestVoice1KeepWithoutShadowLaneFix (pre)`
-> `StrictStaffOverfullCoverageNormalization`
-> bedingt `UpperStaffShadowRestVoice1KeepWithoutShadowLaneFix (post)`

Neu ist nur `ShadowRestRepairLoopInvocationEvidence`. Es konserviert getrennt:

- ob der Pre-Aufruf stattfand,
- seine CandidateRows und sein Written-Ergebnis,
- ob StrictStaff schrieb und wie viele Kandidaten adoptiert wurden,
- ob der Post-Aufruf stattfand,
- dessen CandidateRows und Written-Ergebnis.

Die bestehende finale Shadow-Rest-Auditprojektion bleibt unveraendert; die
Evidence verhindert lediglich, dass der spaetere Aufruf die Information ueber
den frueheren Aufruf fuer den Repair-Loop-Audit verdeckt.

## 2. Short-Rest-Glyph / Fragmented-Rest / Short-Rest-Glyph

Die produktive Reihenfolge bleibt exakt unveraendert:

`ShortRestGlyphSpecificPlacementCompileFix (pre)`
-> `FragmentedRestChainTransactionalWrite`
-> bei Fragmented-Rest-Write `ShortRestGlyph... (post)`

`RestArticulationRepairLoopInvocationEvidence` konserviert getrennt:

- Pre-Invocation, Kandidaten und Write,
- Fragmented-Rest-Invocation, Adopted und Write,
- Post-Invocation, Kandidaten und Write.

Der bestehende `short_rest_glyph_specific_placement_write_audit` darf weiterhin
wie zuvor auf den spaeteren Auditstand umgebunden werden. Diese bisherige
Verhaltens- und Reportsemantik wird nicht veraendert.

## 3. Staccato Primary / Persistence Fallback

Die bestehende Reihenfolge bleibt unveraendert:

`SingleClearStaccatoRenderWriteWithPlacementReadback`
-> Final-Artifact-Readback
-> bei fehlendem Ziel bedingt `FinalArtifactStaccatoPersistenceFix`
-> finaler Readback

Die neue Evidence zeichnet auf:

- Primary-Invocation, CandidateCount und Written,
- ob das Primary-Ziel bereits auf Platte persistiert war,
- ob der Persistence-Fallback versucht wurde,
- ob der Fallback tatsaechlich schrieb,
- ob das Ziel im finalen Artefakt persistiert ist.

Die bereits vorhandene Persistence-Pruefung wird fuer die Branch-Entscheidung
nur in eine lokale boolesche Beobachtung gebunden. Die Bedingung selbst bleibt
identisch. Der zusaetzliche finale Aufruf derselben Predicate-Funktion ist
read-only und dient ausschliesslich der Evidence.

## Ergebnisprojektion

Der kompakte Ergebnisbericht erhaelt genau eine neue Sektion:

`REPAIR-LOOP-INVOCATION-EVIDENCE`

Sie enthaelt stabile, korpusaggregierbare Zeilen fuer die drei Schleifen. Die
Sektion besitzt keine Writer- oder Musikmutationsfaehigkeit und beeinflusst
keine Releaseentscheidung.

## Explizit unveraendert

Dieser Take aendert nicht:

- Reihenfolge irgendeines produktiven Passes,
- Aufrufbedingung irgendeines bestehenden Repairs,
- XML-/MusicXML-Mutation,
- Structural-Write-Authority-Aufrufe,
- Candidate-Erkennung oder Scores,
- Schwellenwerte,
- Staff/Voice/Pitch/Onset/Duration,
- Commit/Rollback,
- Render-/PDF-Auswahl,
- Releaseentscheidung,
- Projekt-, Etueden- oder Taktselektion.

Insbesondere wird in Take 1 weder der doppelte Short-Rest-Glyph-Aufruf noch
der Shadow-Rest-Pre-Aufruf noch der Staccato-Persistence-Fallback entfernt.

## Zielsystembeweis

Da dieser Take gerade die korpusweite Invocation-Verteilung fuer Take 2
ermitteln soll, erfolgt die Abnahme direkt als Ein-Befehl-Vollkorpuslauf:

```powershell
.\run.cmd --take --strict --all
```

Erwartung:

- 10/10 Projekte,
- BuildExit 0,
- ClippyExit 0,
- FinalExit 0,
- 0 Blockerprojekte,
- 0 Warnprojekte,
- gegen `v0.10.1686` versionsbereinigt identische Master-/Render-MusicXML,
- PDF visuell/pixelweise unveraendert,
- pro Projekt genau eine `REPAIR-LOOP-INVOCATION-EVIDENCE`-Sektion.

Erst aus diesem 10/10-Beweis wird Take 2 ausgewaehlt. Take 1 entfernt bewusst
noch keinen produktiven Repair.
