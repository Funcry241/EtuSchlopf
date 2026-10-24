# FinalNotationCommitOutcomeExtraction

Stand: v0.10.1631 r02
Ausgangspunkt: v0.10.1630 r01

## Strict-Clippy-Closure gegenueber r01

Der reale r01-Lauf wurde vor jeder Installation durch `#![deny(unused_mut)]`
abgewiesen. `chord_host_audit` wird nach der Outcome-Extraktion nicht mehr
mutiert; r02 entfernt daher ausschliesslich das ueberfluessige `mut` an dieser
Bindung. Der Refactor, seine Tests und seine fachliche Wirkung bleiben
unveraendert. r01 ist verworfen und kein autoritativer Ausgangsstand.

## Befund

Die vier FinalNotation-Stages erzeugten einen bereits vollständig geprüften
XML-Kandidaten und setzten dabei private Felder wie `written`,
`RenderXmlWrite`, `NotationMutation` und die jeweiligen Write-Zähler auf den
endgültigen Zustand. Der einzige persistente Dateischreibvorgang fand jedoch
erst danach in `write_final_notation_checkpoint_authorized` statt.

Bei einer Ablehnung durch Binding oder Sink musste die Orchestrierung deshalb
vier Audittypen nachträglich destruktiv zurücksetzen. Der rund 60-zeilige
Rollbackblock vermischte zwei verschiedene Tatsachen:

- Kandidat wurde durch lokale Invarianten und Readback akzeptiert,
- Kandidat wurde tatsächlich in die FinalNotation-Datei committed.

Dadurch besaß `written` vor und nach dem Sink unterschiedliche Bedeutungen.

## Typisierte Commitgrenze

Der Take führt ausschließlich innerhalb des bestehenden zentralen Writers ein:

```text
FinalNotationCommitOutcome
- NotRequested
- Committed
- Rejected
```

Die Stages liefern nun nur noch Kandidatenwahrheiten:

- `candidate_accepted`,
- akzeptierte Änderungszahlen,
- lokale Parse-, Cursor-, Pitch- und Delta-Beweise,
- die fachliche Kandidatenmutation.

Nur das Ergebnis des bestehenden zentral autorisierten Dateisinks setzt den
Commit-Outcome. Die drei konsumierten Berichtslinien projizieren
`MeasuresWritten`, `NotesWritten`, `AnchorsWritten`, `RenderXmlWrite` und
`NotationMutation` ausschließlich bei `Committed`.

Bei `Rejected` bleiben Kandidaten- und Readbackbeweise intern erhalten. Der
Bericht behauptet weiterhin keine Writes, projiziert `ExactCandidateDelta nein`
und behält die bisherigen `authorized_sink_rejected`-Urteile bei. Es entsteht
kein Retry, kein zweiter Sink und keine neue Schreibautorität.

## Entfernte Rücksetzarchitektur

Entfallen ist der gesamte nachträgliche Mutationsblock, der unter anderem:

- vier `written`-Zustände zurücksetzte,
- Write-Zähler nullte,
- Mutationsflags löschte,
- Kandidatenzeilen künstlich rekonstruierte,
- drei Berichtstexte erneut formatierte.

Der Commit-Outcome steuert diese Projektion jetzt ohne Veränderung der bereits
bewiesenen Kandidatendaten.

## Regressionstests

Zwei lokale Verträge sichern die neue Grenze:

1. Ein akzeptierter Kandidat wird vor dem Commit nicht als Write berichtet.
2. Ein abgelehnter Commit behält den Kandidatenbeweis, meldet aber null Writes
   und das bekannte Sink-Rejected-Urteil.

## Unveränderte Autorität

Unverändert bleiben:

- Kandidatenerkennung und Passreihenfolge,
- sämtliche XML-Transformationen,
- Masterbindung und Higher-Beam-Binding,
- Parse-, Raw-Cursor-, Pitch- und exakte Delta-Readbacks,
- der globale Higher-Beam-Übergabestore,
- `write_final_notation_checkpoint_authorized` als einziger Dateisink,
- Writerbericht-Präfixe und erfolgreiche Berichtswerte,
- MusicXML-, PDF-, Release- und Exitsemantik.

## Zeilenbilanz

Physische Rust-Zeilen unter `src`:

- v0.10.1630 r01: 115.811
- v0.10.1631 r01: 115.889
- Netto: **+78**

Die betroffene Datei wächst von 3.546 auf 3.624 Zeilen. Davon entfallen netto
34 Zeilen auf die produktive Commitgrenze und 44 Zeilen auf die beiden neuen
Regressionstests. Der Qualitätsgewinn ist hier ausdrücklich keine
Zeilenreduktion, sondern die Entfernung eines mehrdeutigen Zustands und eines
destruktiven Rollbackprotokolls.

## Change-Klasse

`RefactorOnly`. Es wird keine neue musikalische Regel, kein neuer Writer, kein
neuer Sink, keine neue Reportsektion und keine neue Freigabeentscheidung
eingeführt.

## Abnahme

Einmalig:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Kein Korpuslauf und kein `--force`. Erwartet sind Strict-Clippy/Build/Program/
FinalExit 0, versionsbereinigt identische Master- und Render-MusicXML sowie
sechs pixelidentische PDF-Seiten. Die drei konsumierten Writerzeilen müssen im
erfolgreichen Lauf numerisch und semantisch identisch zu v0.10.1630 bleiben.
Im produktiven Write-Path-Inventar ist ausschließlich `FunctionsScanned`
3.585 -> 3.587 durch die zwei neuen Testfunktionen zulässig; Surface- und
Operationszahlen müssen unverändert bleiben.
