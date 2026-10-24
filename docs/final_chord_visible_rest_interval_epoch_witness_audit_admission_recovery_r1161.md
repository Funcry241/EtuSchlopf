# r1161 FinalChordVisibleRestIntervalEpochWitnessAuditAdmissionRecovery

## Parent und Recovery

r1161 baut direkt auf dem akzeptierten und dunkelgruenen r1158 `SemanticOccurrenceCorrespondenceReceiptSanitizationClosure` auf.

r1159 wurde vor Installation durch `Take-Tree-Integrity` verworfen, weil sein eingehendes Take-ZIP den lokal-transaktionalen Root-Pfad `Cargo.lock` enthielt. r1160 entfernte diesen Packaging-Fehler, wurde jedoch ebenfalls vor Installation durch den `CORPUS-INDEPENDENT TEST ADMISSION KODEX` verworfen: Ein synthetischer Test enthielt im XML-String die konkrete Measure-Identitaet `<measure number="1">`. Beide Revisionen sind reine Failure-Evidence und weder Authority noch Parent.

r1161 traegt denselben fachlichen AuditOnly-Payload neu und unabhaengig von r1158. Die einzige fachfremde Recovery-Aenderung gegenueber r1160 ist die corpus-unabhaengige Testdarstellung: Das synthetische Test-MusicXML verwendet ein unnummeriertes `<measure>` und laesst den bereits generischen Parser seine relocatable Fallback-Ordinalisierung verwenden. Die Testidentitaet wurde zugleich rein strukturell benannt. Produktive Auditlogik, Reportprojektion, Klassifikationsregeln und Authority-Grenzen bleiben unveraendert.

## Audit-Ziel

Die Projektion beobachtet ausschliesslich bereits vorhandene Snapshots am finalen Report-Rand:

- den realen `initial_render.final_export_xml` als Pre-FinalChord-Master-Quelle,
- den final persistierten Master,
- den final persistierten Render, sofern vorhanden.

Fuer jeden sichtbaren, nicht als Chord-Anker verwendeten und nicht ganzen Takt umfassenden Resttraeger `R` werden Pre-Bind- und final gebundene Same-Staff-/Same-Voice-Pitchbelegung getrennt beobachtet. Mit der final gebundenen Master-Belegung `N` wird nur klassifiziert, ob der Master `R ∩ N` als Forward und `R \ N` als sichtbare Pause realisiert und ob der finale Render dasselbe `R` vollstaendig als Forward darstellt.

Der Render-Eingang unmittelbar vor dessen FinalChord-Projektion ist an dieser Reportgrenze weiterhin nicht verfuegbar und wird als `UNAVAILABLE_AT_REPORT_BOUNDARY` ausgewiesen. Ein positiver gemeinsamer Witness beweist damit die reale Existenz des groesseren Resttraegers im vorhandenen Pre-FinalChord-Master-Snapshot und die divergierenden finalen Realisierungen, aber keinen konkreten Render-Writer und keine musikalische Richtigkeit einer Seite.

## Corpus-Unabhaengige Tests

Die dauerhaften Tests enthalten keine Etuedenidentitaet, keine konkrete Measure-/Taktidentitaet und keine nichtsynthetischen Corpus-Fixtures. Sie erzeugen ausschliesslich inline synthetische, relocatable Strukturklassen:

- partielle Same-Lane-Pitchbelegung mit Master-Split und Render-Whole-R-Forward,
- geaenderte Voice-Bindung zwischen Pre-Bind und finalem Master,
- Cross-Voice-Gegenfall ohne Same-Lane-Kandidat.

## Authority-Grenze

AuditOnly. Kein Selector, kein Writer, kein semantischer Contract, keine Obligation-Genesis, kein Release-Veto, keine neue Identity und keine Aenderung an `.musicxml`, `.render.musicxml` oder `.pdf`.

## Geplanter Lauf

Kein Vollkorpus. Zuerst `Etüde Nr. 6`; nach erfolgreichem Take-Reexec gezielt E2 und E7. Reale Fundstellen bleiben Evaluation/Diagnose und sind nicht Bestandteil der ausfuehrbaren Testspezifikation.
