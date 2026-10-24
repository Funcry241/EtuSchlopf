# EtuSchlopf v0.10.1839 r528
## ChangeClassPathRoleChangeKindColdStartReadbackClosure

Parent ist der real installierte und vollkorpus-gruene Stand r525 `SixteenthRestVerticalClearanceDebtRetirement`.

Der danach angebotene r526 `FinalNotationAccidentalStateClearanceGeometryAudit` wurde vom bestehenden Take-Change-Control vor Installation korrekt abgewiesen:

```text
Take-Change-Control: Klasse AuditOnly darf Pfad src/audit_only/final_notation_accidental_state_clearance_geometry.rs (MusicSensitive) nicht aendern
```

r526 ist rejected, kein Parent und wird nicht wiederverwendet.

## Root Cause

Der neue r526-Owner lag explizit unter `src/audit_only/` und war fachlich read-only. Trotzdem klassifizierte `take_change_control::path_role()` den Pfad als `MusicSensitive`, weil dort `is_music_sensitive_path()` vor `is_audit_path()` ausgewertet wurde und der Dateiname den Marker `notation` enthielt.

Parallel behandelt `semantic_capability_diff::domain_for_path()` denselben expliziten Audit-Root bereits mit der umgekehrten Owner-Prioritaet:

```text
Kodex -> Audit -> sonstige Source/Music
```

Damit bestand eine Policy-Drift zwischen frueher syntaktischer PathRole und spaeter semantischer Capability-Domain.

## Architekturentscheidung

`PathRole` beschreibt den expliziten Repository-Owner eines Pfades. Lexikalische Musik-Risikomarker bleiben eine konservative Fallback-Klassifikation fuer Source, die keinem expliziten Audit-Owner gehoert.

Die Reihenfolge in `path_role()` wird deshalb auf

```text
Metadata
Kodex
Audit
MusicSensitive
OtherSource
Unknown
```

ausgerichtet.

Das ist keine Freigabe fuer MusicMutation unter `src/audit_only/`:

- `AuditOnly`-Added-Lines bleiben gegen Mutations-/Writer-Tokens fail-closed;
- der Semantic-Capability-Diff klassifiziert Audit-Owner weiterhin separat und erlaubt keine Writer-Family-Capability unter `AuditOnly`;
- der produktive Write-Path-Scanner bleibt unveraendert und verlangt weiterhin null externe MusicXML-Write-Surfaces;
- StructuralWriteAuthority, Writer-Reachability, Repair-/Recovery- und MusicMutation-Gates bleiben unveraendert.

Neue Regressionen binden zugleich:

1. explizite `src/audit_only/`, `src/audits/` und `src/proof_artifacts/`-Owner behalten `PathRole::Audit`, auch wenn der Dateiname `notation`, `musicxml` oder `writer` enthaelt;
2. dieselben lexikalischen Marker bleiben ausserhalb eines expliziten Audit-Owners `MusicSensitive`;
3. `AuditOnly` darf einen reinen neuen Audit-Owner mit musikbezogenem Gegenstandsname deklarieren;
4. der bereits vorhandene Negativtest, der Mutations-Token selbst unter `src/audit_only/` blockiert, bleibt unveraendert aktiv.

## Essenz-Closure

E31 wird ohne neue E32 erweitert:

> **ChangeClass, PathRole und ChangeKind muessen bereits vor dem Candidate-Bau gemeinsam geprueft werden.**

Fachliche Capability und syntaktische PathRole sind getrennte Dimensionen. Bei einem Widerspruch darf weder die ChangeClass nur zum Gate-Passieren hochgestuft noch Pfad oder Dateiname kosmetisch veraendert werden. Zuerst ist zu klaeren, ob der fachliche Owner falsch ist oder die kanonische Pfadklassifikation den expliziten Owner falsch abbildet.

Der maschinenlesbare Bootstrap erhaelt deshalb eine `pre_candidate_sequence` und wiederholt die Matrix auf dem finalen Paketdiff vor Semantic-/Capability-Diff. Das bekannte r526-Gegenbeispiel wird in `010_known_failure_patterns.md` repository-resident dokumentiert.

## ChangeClass

`KodexOnly`.

Keine MusicXML-, Render-, PDF-, PassOrder-, Demand-, Systembreak-, Notations-, Writer-, Repair-, Recovery- oder Musiksemantik wird geaendert. Der r526-Audit selbst ist **nicht** Bestandteil dieses Takes; er darf erst nach real gruener r528-Abnahme unter einer neuen Revision erneut gebaut werden.


## r527 Cold-Start-Readback-Rejection

Der erste Kodex-Closure-Candidate r527 wurde zwar vom PRE-TAKE-DIFF-GATE akzeptiert, scheiterte aber im realen Reexec fail-closed im bestehenden Test `tests/architecture_essence_cold_start_readback.rs`. Ursache war kein Widerspruch gegen die neue PathRole-Regel, sondern ein nicht mitgezogener repository-residenter Readback-Witness: Der Test bindet den bisherigen E31-Satz `Ein Pfad wird niemals nur deshalb erlaubt, um einen Candidate durch das Gate zu bringen` wortnah. r527 hatte diesen Satz durch eine staerkere Formulierung ersetzt und damit den vorhandenen Witness unnoetig gebrochen. Der Bootstrap rollte r527 vollstaendig auf r525 zurueck; r527 bleibt rejected und wird nicht wiederverwendet.

Die r528-Closure erhaelt deshalb den bestehenden Satz unveraendert als weiterhin wahre Teilregel und **ergaenzt** daneben die neue ChangeClass-/PathRole-Regel. Der Readback-Test selbst wird nicht in den KodexOnly-Take gezogen: Er ist nach dem aktuellen PathRole-Vertrag ein Audit-/Readback-Owner. So bleibt ChangeClass-Purity erhalten und der bestehende Witness wird durch kompatible Essenzformulierung bewahrt.
