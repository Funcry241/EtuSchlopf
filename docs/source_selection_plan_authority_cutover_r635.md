# r635 SelectionPlanAuthorityCutover

## Zweck

r635 verschiebt ausschliesslich die produktive Materialisierungsgrenze der bestehenden SourceSelection. Es wird keine neue musikalische Auswahlentscheidung eingefuehrt.

r634 hat fuer jede bestehende SourceSelection-Entscheidung eine stabile `source_ordinal`-gebundene Kausalitaet und einen immutable Shadow-SelectionPlan bewiesen. Der anschliessende unveraenderte Vollkorpuslauf war 10/10 dunkelgruen. Er beobachtete 10.166 CanonicalSourceEvents, 10.089 retained und 77 removed; nur Etuede 6 und Etuede 8 besitzen fruehe SourceSelection-Removals.

## Authority-Schnitt

Vor r635:

    CanonicalSourceEvent
      -> mutable MidiSeedNote working set
      -> bestehende Staff-Moves / Removes
      -> MusicXML seed

r634 beobachtet parallel:

    bestehende Decisions
      -> immutable ShadowPlan
      -> fail-closed Aequivalenzbeweis

Ab r635:

    CanonicalSourceEvent
      -> bestehende Decisions / mutable working set als Policy-Ausfuehrung
      -> immutable SelectionPlan
      -> exakter Aequivalenzvergleich gegen den alten working set
      -> produktive Seed-Rekonstruktion aus SelectionPlan
      -> MusicXML seed

Der alte mutable Bestand entscheidet weiterhin die bestehende Policy und ist in r635 der Comparator. Er besitzt nach geschlossenem Aequivalenzgate jedoch keine Materialisierungsautoritaet mehr.

## Fail-closed Invarianten

`bind_productive_source_selection_plan` verlangt:

1. Plan- und Canonical-Groesse stimmen mit der r634-CausalProjection ueberein.
2. Jede Canonical-Ordinalitaet existiert genau einmal.
3. Jede Plan-Zeile referenziert ein bekanntes CanonicalSourceEvent.
4. `pos`, `len`, `midi` bleiben kanonisch unveraendert.
5. Jede Plan-Zeile besitzt mindestens eine Decision.
6. Retained-Zeilen besitzen genau einen gueltigen finalen Staff 1/2 und keinen Removal-Reason.
7. Removed-Zeilen besitzen keinen finalen Staff und genau einen Removal-Reason.
8. Die Plan-Partition reproduziert exakt `retained_event_count`.
9. Der planmaterialisierte retained Seed ist inklusive Reihenfolge exakt gleich dem r634-Arbeitsbestand.
10. Erst danach werden productive Authority und Plan-Seed-Fingerprint gebunden.

Es gibt keinen Fallback und keine Approximation.

## Keine Policy-Aenderung

Unveraendert bleiben:
- alle SourceSelection-Paesse und ihre Reihenfolge,
- alle Staff-Move- und Removal-Bedingungen,
- alle Thresholds, Caps und Hotspot-Gewichte,
- CandidateProfile und Solver-Ranking,
- Guards und Tie-Breaks,
- ArrangementVisibleRetention,
- CanonicalSourceRetention als reine Auditmetrik,
- Writer-/MusicXML-/Render-/PDF-Semantik.

Insbesondere wird `CanonicalSourceRetention` in r635 nicht in Score, Guard, Ranking oder Selection aufgenommen.

## Report

Die bestehende r634-Reportprojektion bleibt aus Kompatibilitaetsgruenden im historischen Modulpfad. Sie weist nun zusaetzlich aus:

- `SelectionPlanAuthority: productive_seed_materialization`
- `SelectionPlanAuthorityBound`
- `LegacyMutableWorkingSetMaterializationAuthority: none`
- `ProductivePlanMaterializationEquivalentToLegacyWorkingSet`
- `ProductivePlanRetainedEvents`
- `ProductivePlanSeedFingerprintSHA256`

Das Reportmodul selbst besitzt weiterhin keine Writer-Authority.

## Regression

Zwei neue Integrationstests pruefen:
- die Reihenfolge CausalProjection -> Authority-Bindung -> Seed-Rekonstruktion -> MusicXML,
- dass CanonicalSourceRetention weiterhin keine Selection-Policy erzeugt.

Die produktive Authority-/Aequivalenzpruefung selbst wird in jedem realen Etuede-Lauf fail-closed ausgefuehrt; es wird bewusst kein neuer src-interner Test-Caller eingefuehrt, der vom NO-NEW-REPAIR-AUTHORITY-Kodex als zusaetzliche Writer-Root interpretiert werden koennte.

Erwarteter integrierter Testbestand gegen den real gruenen r634-Stand: 1250/1250.
