# r738 SourceSelectionLegacyGuardCompatibilityRetirementWave8

## Zweck

r738 setzt die gezielte Cleanup-Arbeit vor dem spaeteren Voice-/Tie-/Duration-Cutover auf dem dunkelgruenen r737 fort. Der Take aendert keine produktive Musik-, SourceSelection-, Voice-, Tie-, Duration-, Chord-, Writer-, MusicXML-, Render- oder PDF-Entscheidung.

Der Schnitt pensioniert ausschliesslich zwei historische Compatibility-Pins im SourceSelection-Testvertrag:

1. Der aktuelle Guard-Contract akzeptierte neben der produktiven `CandidateDecisionKey`-Authority noch einen alten Fallback-Pfad ohne `CandidateDecisionKey`, der weiterhin `candidate.score` als historische Guard-Komponente erwartete. Dieser Fallback ist mit der aktuellen Architektur nicht mehr zulaessig und wird entfernt. Der Test verlangt jetzt fail-closed die vorhandene `CandidateDecisionKey`-Struktur und deren issue-first Reihenfolge.
2. Der ausschliesslich historische r637-Closure-Test und das nur von diesem Test gelesene Dokument `source_selection_candidate_guard_differential_closure_r637.md` werden pensioniert. Beide beschrieben nur den damaligen AuditOnly-Nachweis und tragen heute keine produktive oder negative Runtime-Capability mehr.

## Erhaltene aktuelle Vertraege

Unveraendert bleiben die drei aktuellen SourceSelection-Vertraege im Testmodul:

- Hand/Rhythm-Primary-Repair bleibt vor LocalBestRefinement,
- `CandidateDecisionKey` bleibt issue-first und enthaelt keinen `candidate.score`-/CanonicalSourceRetention-Guard,
- `PIANO_FAST_LARGE_JUMP` bleibt bei der bestehenden 24-Halbton-Kritikalitaetsgrenze.

Ebenfalls unveraendert bleiben `SourceSelectionFullCausalAudit`, r643 Technical-Lane-Shadow/Comparator, r644 `SourceDurationTechnicalLanePlan`, die Representation-Neutrality-/Mutation-Frontier-Evidence sowie `MusicXmlVoiceAuthority: none` und `TieWriterAuthority: none`.

T3/T16/T22 werden in diesem Take nicht repariert. Der r639-Historical-Fixture `source_selection_staff_rhythm_authority_coupling_audit.rs` bleibt ausdruecklich erhalten, weil er fuer den spaeteren SourceDuration-/Tie-Cutover den historischen Duration-/Chord-Carrier-Gegenbeweis bindet.

## Architekturwirkung

Vor r738 sagte der Testvertrag sinngemaess: aktuelle `CandidateDecisionKey`-Authority **oder** historische Legacy-Guard-Form sind akzeptabel. Nach r738 gilt nur noch die aktuelle Authority. Das reduziert keine Produktcapability; es entfernt eine scheinbar konkurrierende historische Vertragsform vor dem Voice-Cutover.

## Bilanz

Rust LOC: `198.028 -> 197.984`, also **-44** physische Rust-Zeilen netto.

Gegen die feste Retirement-Baseline 203.261: **-5.277 / -60.000**.
