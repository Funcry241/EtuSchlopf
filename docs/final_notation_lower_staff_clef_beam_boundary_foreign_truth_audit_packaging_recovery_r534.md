# r534 – FinalNotationLowerStaffClefBeamBoundaryForeignTruthAuditPackagingRecovery

Parent ist weiterhin der corpusbewiesene GreenBase `v0.10.1839 r532`
`FinalNotationClearanceSystemStructuralSlackWarningDebtClosure`.

## Rejection-Provenienz

Der zuvor ausgelieferte Candidate r533 wurde vor jeder Installation durch
`Take-Tree-Integrity` fail-closed abgewiesen, weil sein ZIP versehentlich den
lokal-transaktionalen Root-Pfad `Cargo.lock` enthielt. r533 ist kein Parent und
wird nicht wiederverwendet.

r534 wird deshalb frisch aus dem gebundenen r532-Source-Snapshot aufgebaut. Der
fachliche Audit-Payload entspricht dem verworfenen r533-Befund; die Recovery
betrifft ausschließlich Paketstruktur und eindeutige Revisionsidentität. Das
fertige incoming ZIP enthält keinen Root-`Cargo.lock`.

## Anlass

Im finalen PDF von Etüde 8 / Takt 25 / unteres System läuft ein sichtbarer
Primärbalken über einen expliziten Wechsel zum G2-Schlüssel. Die bestehende
`LowerStaffClefIslandDecompositionRule` teilt eigene BeamPlan-Gruppen bereits an
einer solchen Schlüsselgrenze. Gleichzeitig behandelt die spätere
`NotationPolishBeamDecisionTrichotomyClosure` eine strukturell vollständige
fremde `begin ... end`-Kette noch ohne Kenntnis dieser Grenze als zu erhaltende
Fremdwahrheit.

r534 verändert diese Semantik nicht. Der Take beobachtet ausschließlich die
finale MusicXML-Oberfläche und klassifiziert vollständige sichtbare Beam-Level
auf Staff 2, die eine explizite Staff-2-Clef-Grenze überspannen. Zusätzlich wird
beobachtet, ob an derselben Timelineposition eine Staff-2-Octave-Shift-
Zustandsänderung vorliegt.

## Erwartete r532-Korpusklasse

Die vor dem Take offline aus den zehn r532-Render-MusicXMLs gelesene Klasse
besteht aus sechs Measure-Grenzen:

- Etüde 7 / Takt 16
- Etüde 8 / Takt 25
- Etüde 8 / Takt 26
- Etüde 9 / Takt 18
- Etüde 10 / Takt 27
- Etüde 10 / Takt 32

Etüde 10 / Takt 32 trägt dabei zwei Beam-Level über derselben Grenze. Alle sechs
beobachteten Measure-Grenzen fallen zugleich mit einer unteren Octave-Shift-
Zustandsänderung zusammen. Diese Korpusbeobachtung erteilt ausdrücklich kein
neues produktives Recht; die Clef-Grenzenregel existiert bereits unabhängig.

## Authority

`src/audit_only/final_notation_lower_staff_clef_beam_boundary_foreign_truth.rs`
ist reiner Audit-Owner.

Der Take verändert nicht:

- Pitch, Onset, Dauer, Staff, Voice oder Hand;
- Clef-Entscheidung oder Clef-Materialisierung;
- Octave-Shift-Entscheidung oder Materialisierung;
- BeamPlan oder fremde Beam-Wahrheit;
- MusicXML-/PDF-Writer;
- Layout, Demand, Capacity oder Systempartition.

Insbesondere entsteht keine Audit→Writer-Abhängigkeit. Ein späterer produktiver
Take muss ausschließlich aus den bereits vorhandenen produktiven Clef-/Beam-
Authorities begründet werden.
