# r634 SourceSelectionFullCausalAuditAndShadowPlan

## Zweck

r634 erweitert den installierten r633 ausschliesslich um Beweisfaehigkeit der bereits bestehenden SourceSelection- und Solver-Entscheidungskette. Es gibt keine neue Selection-Regel und keine neue Musikmutation.

Die Beweisgrenze lautet:

`CanonicalSourceEvent -> bestehende Staff-Moves/Removes -> bestehender SourceSelection-Seed -> entscheidungsrelevante Solver-Frontier -> finaler Solver-Gewinner`

## Event-Level-Kausalitaet

Jedes CanonicalSourceEvent bleibt ueber `source_ordinal` stabil identifizierbar. r634 zeichnet die bereits vorhandenen Entscheidungen auf:

- initiale Pitch-basierte Staff-Zuordnung,
- Piano-Hand-Split,
- Fast-Jump-Staff-Smoothing,
- Wide-Simultaneity-Reassignments,
- Trajectory-Register-Moves,
- Residual-Register-Moves,
- alle sechs bereits in r633 gebundenen Removal-Branches.

Eine Decision-Evidenz bindet, soweit der vorhandene Algorithmus die Information besitzt:

- `source_ordinal`,
- Pass und Pass-Iteration,
- Before-/After-Staff,
- Witness-SourceOrdinals,
- beobachtete Messwerte,
- tatsaechlich verwendete Thresholds/Caps,
- Focus-Score,
- deterministischen Tie-Break-Code,
- bei Removes den unveraenderten r633-Reason.

Die produktive Reihenfolge bleibt unveraendert. Insbesondere bleiben batchweise gesammelte Staff-Moves in derselben `(idx, staff)`-Sortier- und Deduplizierungssemantik wie r633; der Audit liest den tatsaechlichen Before-Staff erst beim Apply.

## Immutable Shadow-Plan

Nach Abschluss der unveraenderten Seed-Paesse replayt `project_source_selection_full_causal_audit()` die Decision-Chain ausgehend vom kanonischen Source-Universum. Fail-closed geprueft werden mindestens:

- lueckenlose/eindeutige CanonicalSourceEvent-Ordinalitaeten,
- genau eine InitialStaff-Entscheidung je Event,
- jeder Staff-Move passt zum jeweils lebenden Before-State,
- jeder Remove passt zum jeweils lebenden Before-State und zum r633-Reason-Ledger,
- Witnesses referenzieren existierende CanonicalSourceEvents,
- Shadow-retained/removed entspricht exakt der produktiven Partition,
- der finale Shadow-Staff entspricht exakt dem produktiven finalen Staff.

Der Shadow-Plan ist read-only. `ShadowPlanAuthority = none`, `WriterAuthority = none`.

## Zwei getrennte Retention-Metriken

`CanonicalSourceRetention` misst roh:

`retained CanonicalSourceEvents / alle CanonicalSourceEvents`

Der bestehende downstream Wert wird als `ArrangementVisibleRetention` separat ausgewiesen. Die beiden Werte werden nicht gleichgesetzt. `CanonicalSourceRetention` beeinflusst in r634 weder Score, Guard, Ranking noch Selection.

## Hotspot-Kausalitaet

Die bestehende Hotspot-Gruppierung und -Priorisierung bleibt byte-semantisch unveraendert. Der Audit projiziert zusaetzlich:

- Quell-Issue-Typ,
- Measure/Staff,
- vorhandenes Basisgewicht,
- vorhandenen Critical-Bonus,
- vorhandenen Haertebeitrag,
- resultierenden Beitrag,
- Aufnahme bzw. Ausschluss durch die bestehende Truncation,
- einen domain-separierten SHA-256-Fingerprint.

Die Gewichte werden nicht geaendert.

## SourceSelection- und Solver-Kausalitaet

Jeder neu aus MIDI erzeugte SourceSelection-Seed erhaelt eine immutable SourceSelection-Candidate-ID, Origin-Stage und das exakte SourceSelection-Profil. Spaetere Solver-/Patch-Schritte duerfen den normalen `solver_profile` weiterhin wie bisher aendern; die SourceSelection-Profilidentitaet bleibt davon getrennt.

Fuer die finale entscheidungsrelevante Solver-Frontier werden projiziert:

- Solver-Iteration,
- aktueller Profilname und vorhandene CandidateLineage,
- zugrunde liegende SourceSelection-Candidate-ID,
- bestehender Candidate-Score,
- Blocker/Warnungen/Hinweise,
- Human-Potential-Notes-Moved,
- Stage-Checkpoints des Gewinnerpfads.

ExactSeedReuse bleibt unveraendert. Byteidentische Seeds koennen weiterhin nur einmal teuer finalisiert werden; die immutable SourceSelection-Identitaet und die jeweilige Profil-/Solver-Lineage bleiben im Audit getrennt.

Die `SolverFrontier` bezeichnet bewusst `final_decision_pool_plus_winner`. Reine Probe-Kandidaten, die im bestehenden Code kein Auswahlrecht besitzen, werden nicht als Teil dieser Entscheidungsfrontier behauptet. r634 fuehrt keinen neuen parent-authority graph ein.

`final_decision_pool_plus_winner` ist eine Mengenvereinigung, keine Behauptung disjunkter Speicherorte. Der reale Solver zeigt ausserdem, dass `solver_iteration` keine immutable Kandidatenidentitaet ist: ein bestehender Solver-Kandidat kann in spaeteren Stufen in-place weiterentwickelt werden und dabei seine Iterationsnummer behalten. Deshalb gilt in r634:

- `solver_iteration` ist historische Iterations-/Lineage-Evidenz, keine eindeutige State-ID,
- ein Solver-Zustand wird durch den domain-separierten Fingerprint seiner vollstaendigen Frontier-Zeile identifiziert,
- feldidentische Duplikate aus Pool/Winner-Vereinigung werden deterministisch dedupliziert,
- verschiedene Zustaende mit derselben Iterationsnummer bleiben als getrennte Frontier-Entitaeten erhalten,
- der finale Gewinner wird gegen seine vollstaendige Frontier-Zeile und deren State-Fingerprint gebunden, nicht nur gegen die Iterationsnummer.

Damit wird eine reale Solver-Eigenschaft beobachtet, ohne Solver-Mutation, Ranking- oder Ownership-Semantik zu aendern.

## Mechanische Warning-Closure

Die sechs Enum-Variant-Namen des r633 Removal-Reason-Enums verlieren ausschliesslich den gemeinsamen Suffix `Reduction`, um die r633-Hygienewarnung `clippy::enum_variant_names` zu schliessen. Die externen Reason-Strings und Discriminators bleiben unveraendert.

## Nicht-Scope

Keine Aenderung an:

- MIDI-Decoding/FIFO-Note-Pairing,
- SourceAuthority,
- SourceSelection-Passreihenfolge,
- Profilwerten, Thresholds, Caps oder Gewichten,
- Staff-/Hand-/Voice-Regeln,
- Removal-Bedingungen,
- Candidate-Score oder Guard,
- WAV-Authority,
- Writer/MusicXML/Render/PDF,
- Musiksemantik.

## Qualifikationsgrenze

r634 ist erst installierbar, wenn der normale lokale fail-closed Take-Lauf mit dem gepinnten Toolchain Build, Tests, Rustfmt/Strict-Clippy und die vorhandenen Sensor-/Release-Gates bestaetigt. Gegenueber r633 kommen vier Regressionstests hinzu; erwarteter integrierter Testbestand ist 1248/1248.
