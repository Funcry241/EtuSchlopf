# SourceSelection Exploratory Audit Retirement / Demand Witness Closure r730

Parent: r728 `ResidualLowerOctaveRetirementFullDiffRecovery`

Rejected predecessor: r729 `SourceSelectionExploratoryRhythmDemandAuditRetirement`

ChangeClass: RefactorOnly

## Gegenbefund aus r729

Der r729-Reexec wurde vor einem gruenen Abschluss vom Compiler gestoppt. Nach Entfernung der r642 `SourceDurationVoiceLaneDemandAudit`-Laufzeitberechnung blieben vier Verwendungen von `voice_demand` im weiterhin aktiven r643-Technical-Lane-Comparator uebrig. Der Befund beweist: Nicht die alte r642-Reportcapability, wohl aber vier ihrer abgeleiteten Demand-Fakten gehoeren weiterhin zum unabhaengigen Gegenbeweis des r643-Shadowplans.

## Closure

r730 rekonstruiert diese vier Fakten in einem kleinen internen `SourceDurationLaneDemandWitness`:

- `retained_events`,
- `exact_interval_chord_cohorts`,
- `upper_staff_max_concurrent_cohorts`,
- `lower_staff_max_concurrent_cohorts`.

Die Berechnung bleibt vom Lane-Allocator getrennt. Fuer die Concurrency-Werte wird dieselbe bisherige Semantik beibehalten: exakte `(staff, source_onset, source_end)`-Kohorten, measure-lokales Clipping ueber Taktgrenzen, Half-Open-Intervallgrenzen durch `end=-1` vor `start=+1` am gleichen Tick und Maximum je Staff ueber alle Staff/Measure-Scopes. Damit bleibt `SourceDurationTechnicalLaneMatchesObservedConcurrencyMinimum` ein unabhaengiger Vergleich und wird nicht zu einer Selbstbestaetigung des Lane-Plans.

Zwei fokussierte Tests sichern die noch lebende Witness-Capability: gleiche Source-Intervalle bilden genau eine Kohorte; beruehrende Intervalle benoetigen eine Lane, echte Ueberlappung zwei.

## Weiterhin pensioniert

Nicht wieder eingefuehrt werden:

- r641 `RhythmAuthorityFrontierAudit` samt Reportprojektion und Erkundungstests,
- r642 `SourceDurationVoiceLaneDemandAudit` als Report-/Frontiercapability,
- Staff-Scope-Verteilungsmetriken (`single/double/triple/four_or_more/multi_lane`),
- Cross-Measure-Zaehler der alten r642-Projektion,
- `SourceDurationSingleLaneMaterializationSafe` und die uebrigen historischen r642-Reportzeilen.

## Unveraendert

Aktiv bleiben r643 Shadowplan, Fingerprint, CoverageExact, CollisionFree, beobachtetes Concurrency-Minimum, r644 `SourceDurationTechnicalLanePlan` und dessen Shadow-Aequivalenz. MusicXML-Voice- und Tie-Writer-Authority bleiben `none`. Keine MusicMutation, keine Writer-/Selection-/Release-/Renderer-/Recovery-/Trust-Aenderung.

## Rust-Bilanz

`201.856 -> 201.468`, Delta **-388** gegen r728.

Kumuliert seit r723: **-1.793 / -60.000**.

## Abnahme

Der Take muss Tree-Integrity, Change-Control, Semantic-Capability-Diff, Strict-Clippy, Build und Tests unveraendert fail-closed passieren. Der normale Projektlauf muss `FinalExit 0`, `RunSeverity dark_green` und `ReleaseDecision released` behalten. Erwartete fachliche Aenderung ist nur das Fehlen der pensionierten r641/r642-Reportzeilen; MusicXML, Render-MusicXML und PDF duerfen sich nicht aendern.
