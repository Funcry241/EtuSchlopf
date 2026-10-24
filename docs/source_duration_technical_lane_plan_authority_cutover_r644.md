# SourceDuration TechnicalLanePlan Authority Cutover r644

Parent: r643 `SourceSelectionSourceDurationTechnicalLaneShadowPlanAudit`

ChangeClass: RefactorOnly

## Zweck

r643 hat bewiesen, dass die retained Source-Intervalle des finalen SelectionPlans
deterministisch, exakt und kollisionsfrei auf technische Lanes abgebildet werden
koennen. r644 macht genau diese technische Zwischenstruktur zu einer
produktiven, immutable Pre-Materialization-Authority.

## Reihenfolge

CanonicalSourceEvents
-> bestehende SourceSelection
-> immutable SelectionPlan (seit r635 produktiv)
-> immutable SourceDurationTechnicalLanePlan (r644)
-> bestehende MidiSeedNote-Materialisierung
-> unveraendertes build_musicxml

Der letzte Pfeil bleibt in r644 unveraendert: `build_musicxml` liest den
TechnicalLanePlan noch nicht.

## Planidentitaet

Ein Technical-Lane-Kohortenschluessel besteht ausschliesslich aus:

- final_staff
- source_onset
- source_end

Mitglieder sind geordnete `source_ordinal`s. Kohorten werden je Staff in
aufsteigender Intervallreihenfolge greedily der ersten freien technischen Lane
zugeordnet. Der Plan-Fingerprint bindet Staff, Start, Ende, Lane, MemberCount
und alle Member-Ordinalitaeten.

## Fail-closed-Vertrag

Vor Bindung muss die SelectionPlan-Authority bereits aktiv und
`productive_plan_equivalence_verified` sein. Der TechnicalLanePlan muss:

- exakt alle retained Events erfassen,
- jede retained SourceOrdinalitaet genau einmal erfassen,
- nur Staff 1 oder 2 enthalten,
- alle Kohorten deterministisch einer Lane zuordnen,
- null Lane-Ueberlappung besitzen,
- die Taktsegmentierungsnachfrage vollstaendig zaehlen.

Der r643-Shadowplan bleibt unabhaengiger Comparator im Audit-Owner. Der Report
weist produktiven und Shadow-Fingerprint sowie deren Aequivalenz separat aus.

## Nicht-Autoritaeten

r644 vergibt ausdruecklich keine:

- MusicXML-Voice-Authority,
- Tie-Writer-Authority,
- Chord-Writer-Authority,
- SourceDuration-Writer-Authority,
- Staff-/Selection-/Removal-Authority,
- Threshold-/Score-/Candidate-Authority.

Damit ist r644 ein Authority-Cutover der technischen Zwischenstruktur, aber
noch keine Notations- oder Musikmutation.
