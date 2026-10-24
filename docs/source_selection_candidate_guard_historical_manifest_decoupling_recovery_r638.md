# r638 SourceSelectionCandidateGuardHistoricalManifestDecouplingRecovery

## Anlass

Der erste r638-MusicMutation-Kandidat `SourceSelectionReassignmentCreatedSpanJumpDeletionAuthorityRetirement` wurde beim Reexec korrekt zurueckgerollt. Build und Clippy waren gruen; die integrierten Tests stoppten ausschliesslich am bereits in r637 eingefuehrten Testtarget `source_selection_candidate_guard_differential_audit`.

Die Ursache liegt im r637-Test `r637_is_audit_only_and_introduces_no_productive_source_change`: Er las bei jedem spaeteren Build `src/build_gate/take_change_class.txt` als waere diese Datei ein unveraenderliches r637-Artefakt. Sobald ein legitimer spaeterer Take eine andere ChangeClass oder andere Change-Pfade besitzt, muss dieser historische r637-Test daher rot werden. Damit band r637 versehentlich seine damalige Take-Manifestinstanz an alle zukuenftigen Revisionen.

## Korrektur

r638 ist `AuditOnly`. Es aendert keine produktive Musik-, Selection-, Candidate-, Score-, Threshold-, Repair-, Writer-, MusicXML-, Render- oder PDF-Semantik.

Die ersten drei r637-Regressionen bleiben unveraendert und pruefen weiterhin die produktiven Eigenschaften:

1. Hand/Rhythmus-Vorstufe vor `LocalBestRefinement`,
2. Guard-Reihenfolge Criticality vor Score ohne CanonicalSourceRetention-Key,
3. bestehende 24-Halbton-Grenze fuer kritische `PIANO_FAST_LARGE_JUMP`.

Nur der vierte Test wird korrigiert: Die historische Aussage `r637 war AuditOnly und fuehrte keine produktive Source-Aenderung ein` wird gegen die bereits in r637 persistierte historische Closure-Dokumentation geprueft, nicht gegen das jeweils aktuelle `take_change_class.txt`.

Damit bleibt der r637-Beweis erhalten, verliert aber seine unbeabsichtigte Autoritaet ueber spaetere Take-Manifeste.

## Abgrenzung

Dieser r638-Take enthaelt bewusst **nicht** die zuvor geplante Span-/Jump-Semantikaenderung. Der zurueckgerollte MusicMutation-Kandidat bleibt verworfen. Erst nach erfolgreicher Installation dieses AuditOnly-Recovery-Takes darf die unabhaengige, bereits analysierte Reassignment-Causality als neuer MusicMutation-Take erneut eingebracht werden.
