# Bekannte Fehlerbilder – EtuSchlopf

## 1. Staff-unscharfer Clef-Parser

Symptom:

```text
LegacyAnchorActiveStaff2Clef = G
PredicateActiveStaff2Clef = F4
ScopedClefNodeActiveStaff2Clef = F4
SharedAttributeUpperGStaff2FBeforeM005 = 1
```

Bedeutung: Ein privater Attribute-Segment-Parser hat den oberen G-Schlüssel aus einem gemeinsamen Attributes-Block fälschlich als aktiven Staff-2-Schlüssel gelesen.

Gegenmittel: Staff-skopierte `<clef number="2">`-Lesung verwenden.

## 2. PostWrite-WouldWrite-Falle

Symptom:

```text
SelectedPdfWouldWrite nein
SelectedPdfPostWriteAlreadyMaterialized ja
SourceReadbackAccepted ja
```

Bedeutung: Kein Fehler. Der Zielzustand ist schon materialisiert. Die Probe darf nicht erneut schreiben wollen.

## 3. Report-/Launcher-Hysterese

Symptom: Projektläufe OK, PDF vorhanden, aber Kurzstatus rot oder Exit 2.

Gegenmittel: getrennt prüfen:

- Musik-Gate
- Render-Gate
- Report-Gate
- Launcher-Gate

Keine Musikreparatur bauen, bevor klar ist, welche Ebene wirklich blockiert.

## 4. Korpusbeweis durch Cleanup selbst entwertet

Symptom: Der Vollkorpusbericht nennt zehn aktuelle Projekte, der folgende Visual-Preflight kann aber nur Ziel und Referenz als Einzelbelege verifizieren.

Bedeutung: Die Startbereinigung des zweiten Befehls hat acht Resultatdateien gelöscht und damit den kurz zuvor gültigen Korpusbeweis selbst zerstört.

Gegenmittel: `corpus_run_manifest.json` ist der Schutzvertrag. Solange sein Hash-Vertrag gültig ist, müssen alle zehn gebundenen Projektresultate, der Korpusbericht und das Manifest die Bereinigung überleben.


## 5. Visual-Hotspot als Schreibauftrag missverstanden

Symptom: Ein frischer Korpus wählt Projekt/Locator, obwohl `RuleCandidatesTotal 0` und `NextGenericRuleCheck: no_rule_candidate` gelten.

Bedeutung: Die Auswahl ist ein menschlicher PDF-Prüfhotspot, keine Regel- oder Schreibfreigabe.

Gegenmittel: `TargetClass`, `HumanReviewRequired`, `RuleCandidate`, `WriteCandidate` und `TargetPermissionContract` getrennt lesen. `human_pdf_review_hotspot` besitzt kein Schreibrecht.

## 6. Clippy 0 als allgemeine Warnungsfreiheit missverstanden

Symptom: Vor dem Launcher erscheinen Rust-Compilerwarnungen, während der spätere Clippy-Status 0 meldet.

Bedeutung: Launcher-Bootstrap-Compiler und strikter Clippy-Lauf sind verschiedene Prüfpfade.

Gegenmittel: `warning_truth.compiler_*` und `warning_truth.clippy_*` getrennt auswerten; Exit 0 allein beweist keine Warnungsfreiheit.

## 7. Root-`Cargo.lock` im EtuSchlopf-Take

Symptom:

```text
Take-Tree-Integrity: unbekannter Top-Level-Pfad "Cargo.lock"; fail-closed
```

Bedeutung: Das Take-ZIP wurde aus einem vollständigen Arbeitsbaum gebaut und enthält
den Root-`Cargo.lock`. Dieser Pfad gehört bewusst **nicht** zur verwalteten
Take-Wurzel. Das Gate muss den Take vor Installation ablehnen.

Gegenmittel:

- vor jeder Auslieferung das **fertige ZIP** gegen `MANAGED_DIRS` und
  `MANAGED_ROOT_FILES` aus `managed_source_tree_definition.rs` prüfen;
- Root-`Cargo.lock` niemals in einen EtuSchlopf-Take aufnehmen;
- nach bereits ausgelieferter/abgewiesener Revision kein korrigiertes ZIP mit
  derselben Revision erzeugen, sondern wegen Same-Revision-Ambiguität eine neue
  eindeutige Revision verwenden.


## 8. ChangeClass-/PathRole-Drift bei explizitem Audit-Owner

Symptom:

```text
Take-Change-Control: Klasse AuditOnly darf Pfad src/audit_only/final_notation_accidental_state_clearance_geometry.rs (MusicSensitive) nicht aendern
```

Bedeutung: Der r526-Candidate war fachlich als read-only Audit angelegt und lag unter dem expliziten Owner `src/audit_only/`. Das fruehe Take-Path-Gate klassifizierte den Pfad jedoch zuerst ueber den lexikalischen Marker `notation` als `MusicSensitive`, waehrend der Semantic-Capability-Diff denselben expliziten Audit-Root bereits als `Domain::Audit` behandelt. r526 wurde gemaess dem damals installierten Gate korrekt vor Installation abgelehnt; r525 blieb Parent.

Gegenmittel:

- bereits **vor Candidate-/ZIP-Bau** den final geplanten Diff als `ChangeClass × PathRole × ChangeKind` pruefen;
- fachliche Capability und syntaktische PathRole getrennt behandeln;
- bei Widerspruch klaeren, ob der fachliche Owner falsch ist oder die kanonische Pfadklassifikation den expliziten Owner falsch abbildet;
- weder die ChangeClass nur zum Gate-Passieren hochstufen noch Pfad/Dateiname kosmetisch umbenennen;
- explizite Owner-Roots und lexikalische Risikomarker in allen konsumierenden Gates auf konsistente Prioritaet pruefen;
- abgewiesene Revisionen nicht mit veraenderter Payload wiederverwenden.


## 9. Essenz-Closure ohne vollstaendige Cold-Start-Witness-Preservation

Symptom:

```text
error: 1 target failed:
    `--test architecture_essence_cold_start_readback`
```

Bedeutung: r527 hatte die neue E31-Regel fachlich korrekt erweitert, dabei aber einen bereits repository-resident gebundenen Satz ersetzt, den der bestehende Cold-Start-Readback als Witness erwartet. PRE-TAKE und Build konnten deshalb gruen sein, waehrend die integrierten Rust-Tests fail-closed abbrechen. Der Bootstrap rollte r527 vollstaendig auf r525 zurueck.

Gegenmittel:

- bei Essenz-/Bootstrap-Aenderungen die zugehoerigen Readback-Witnesses bereits vor Candidate-Bau inventarisieren;
- eine weiterhin wahre gebundene Teilregel bevorzugt erhalten und neue Semantik additiv ergaenzen;
- einen Audit-/Readback-Test nicht nur zur Bequemlichkeit in eine `KodexOnly`-Closure ziehen, wenn seine kanonische PathRole `Audit` ist;
- `ChangeClass × PathRole × ChangeKind` gilt damit auch fuer die Regression-Witnesses selbst;
- abgewiesene Revisionen nicht wiederverwenden.

## 10. Persistenter Scope mit falschem sichtbaren Carrier

Symptome aus dem r599-r605-Zweig:

```text
MusicXML octave-shift continue vorhanden, PDF aber pixelidentisch
```

oder:

```text
frueher Re-Cue-Readback GREEN, spaeter Legacy-Cleanup entfernt den Words-Cue
```

oder:

```text
Continue an spaeterem finalen new-system/new-page traegt zusaetzlich 8va/8vb und erzeugt Doppelanzeige
```

Bedeutung: Semantischer Scope, maschineller Zustands-Carrier, sichtbarer Cue und finale Layoutwahrheit sind getrennte Schichten. `continue` allein garantiert keinen inner-systemisch sichtbaren Renderer-Cue. Ein frueh sichtbarer Cue kann von einem spaeteren Writer wieder entfernt werden. Die endgueltige `new-system`-/`new-page`-Wahrheit kann erst beim finalen HorizontalLayout-Owner feststehen.

Gegenmittel:

- Scope-/Linienidentitaet nicht fuer einen Re-Cue neu starten;
- keine Layoutentscheidung aus einem noch nicht finalen Zwischenzustand ableiten;
- sichtbare Re-Cues am finalen Layout-Wiedereintritt nur dann projizieren, wenn der Renderer sie nicht bereits traegt;
- Last-Write-Owner inventarisieren und den final persistierten Zustand erneut fail-closed lesen;
- PDF-/Renderer-Readback als eigene Evidenz verwenden: Formatfaehigkeit ist keine Sichtbarkeitsgarantie.

Der r605-Abschluss materialisiert diese Trennung ohne Takt-/Etueden-Sonderlogik: Continue bleibt fuer alle 12 qualifizierten stillen Bruecken bestehen; der zusaetzliche Words-Cue bleibt nur inner-systemisch und wird am finalen staff-lokalen System-/Seitenwiedereintritt im bereits autorisierten HorizontalLayout-Write entfernt.

## 11. Darstellungsnormalisierung verschiebt klingende Timeline

Symptom:

```text
vorhandene gepitchte Note: onset=A, duration=D
neu eingefuegte Darstellung/Pause belegt A..A+D
dieselbe source-gebundene Note erscheint danach bei onset=A+D
```

Bedeutung: Die Darstellungs- oder Rhythmusnormalisierung hat nicht nur Notation umgeformt, sondern vorhandene klingende Semantik veraendert. Ein spaeterer Lineage- oder AttackOnset-Check darf diese Mutation nicht durch zusaetzliche Berechtigung nachtraeglich legitimieren.

Gegenmittel:

- vorhandene gepitchte Survivor vor und nach der Darstellungsnormalisierung auf Onset, Dauer, Pitch, Staff, Voice und Chord-Rolle vergleichen;
- kollidiert eine neu erzeugte Pause oder Darstellung mit dieser Survivor-Timeline, die Darstellungsinsertion verwerfen;
- keine Etueden-, Takt- oder Pitch-Sonderregel einfuehren;
- Source-/Attack-Provenienz vorwaerts tragen und nicht aus dem veraenderten Endzustand zurueckerraten;
- den roten Lauf zuerst als Gegenbeweis verstehen, statt eine neue Mutationserlaubnis nur zum Gate-Passieren zu erteilen.

## 12. Mutierender Zwischenstand ist technisch gruen, aber nicht Public-Artifact-releasefaehig

Symptom:

```text
PRE-TAKE-DIFF-GATE ... OK
Clippy OK
Build OK
Rust-Tests alle Targets OK
Programm/PDF OK
FEHLER: Public-Artifact-Kandidatenvalidierung/Commit fehlgeschlagen; ...
Public-Artifact-Recovery: neuer Bericht ist nicht aktuell blocker-/warnungsfrei ...
ROLLBACK
```

Bedeutung: Ein MusicMutation-Kandidat kann Build, Tests, Programmlauf und PDF-Erzeugung korrekt durchlaufen und trotzdem kein zulaessiger Release-Zwischenstand sein. Der gemeinsame formale Gruenvertrag des Public-Artifact-Commits verlangt im Ergebnisbericht 0 Blocker und 0 Warnungen. Wenn eine fachlich richtige fruehere Mutation erst einen gekoppelten Restfehler sichtbar macht, den ein spaeter geplanter Take schliessen soll, ist die geplante Take-Grenze falsch: Der Zwischenzustand ist unter dem aktuellen Release-Vertrag nicht commitfaehig.

Gegenmittel:

- vor einem gestuften MusicMutation-Cut nicht nur Build/PDF, sondern den erwarteten finalen Resultatstatus je Zwischenstufe pruefen;
- `Programm/PDF OK` niemals mit Release-Green gleichsetzen;
- wenn A einen Blocker/eine Warnung freilegt, die B schliessen muss, A entweder audit-/read-only lassen oder A+B als kleinste release-atomare Invariantengruppe schneiden;
- nicht pauschal mehrere unabhaengige Reparaturen zusammenziehen: Atomizitaet gilt nur fuer durch den Release-Vertrag gekoppelte Invarianten;
- nach Rollback bleibt der letzte akzeptierte gruene Stand Parent; der technisch weit gelaufene Kandidat liefert nur Diagnosewissen;
- ohne den verworfenen Ergebnisbericht nicht behaupten, welche konkrete Warn-/Blockerklasse den Commit verhindert hat.

Verwandte Vorabfehler derselben Arbeitsserie:

- Test ruft produktiven Writer direkt auf -> `new_writer_caller_outside_structural_write_authority`;
- neue Hilfs-/Limit-Funktion wird als `CalibrationThreshold` inventarisiert;
- `take_change_class.txt` traegt einen geerbten statt final gegen den Parent erzeugten Diff.

Diese drei Faelle bleiben eigenstaendige Gateklassen; gemeinsam mit der Public-Artifact-Grenze bilden sie den verpflichtenden Pre-Package-/Pre-Release-Selbstcheck fuer kuenftige Authority-Cuts.
## 13. AuditOnly versucht neue Logik oder neuen Datentransport in gemischter Produkt-Source

Symptom aus r752:

```text
Take-Change-Control: AuditOnly darf in gemischter Source src/pipeline_shards/candidate_material_lineage_spine_core.rs nur loeschen oder eine bestehende geschweifte use-Liste strikt verkleinern; unzulaessige neue Zeile: fn append_frozen_sounding_temporal_lane_audit(
```

Bedeutung: `AuditOnly` ist nicht nur eine Aussage ueber die Absicht einer Funktion. Fuer eine bestehende gemischte produktive Rust-Source gilt ein strenger Cross-Role-Rueckbauvertrag: AuditOnly darf dort keine neue Logik, keinen neuen Helper und keinen neuen Daten-/Reporttransport einfuehren. Auch eine rein lesende Funktion bleibt eine neue Source-Capability an einem produktiven Owner und ist damit unter `AuditOnly` unzulaessig.

Gegenmittel:

- neue Audit-Implementierung vollstaendig unter einem kanonisch als `PathRole::Audit` klassifizierten Owner halten;
- vor dem Patch nicht nur den Zielpfad des Audits, sondern **jede benoetigte Datenkante** inventarisieren;
- private produktive State nicht durch einen neuen AuditOnly-Helper, Call-Site, Feld, Callback oder Report-Bridge aus einer gemischten Source herausreichen;
- existiert bereits ein autorisierter audit-sichtbarer Transport, diesen wiederverwenden;
- existiert kein solcher Transport, den fehlenden Transport als eigene Architekturgrenze behandeln und die dafuer passende ChangeClass/Authority bestimmen; nicht die ChangeClass nur zum Gate-Passieren hochstufen;
- ein verworfener Audit-Candidate bleibt Diagnosewissen; Parent bleibt der letzte akzeptierte gruene Stand.

Vorabfrage fuer kuenftige Audit-Takes: **Kann das neue Audit ausschliesslich mit Daten implementiert werden, die bereits an einem Audit-Owner erreichbar sind?** Wenn nein, ist der geplante Take noch kein reines AuditOnly.

## 14. Struct-Erweiterung ohne vollstaendige Initializer-Closure

Symptom aus r754:

```text
error[E0063]: missing field `frozen_sounding_temporal_lane_plan` in initializer of `pipeline::CandidateMaterialLineageSpine`
```

Bedeutung: Ein neues Pflichtfeld wurde am Struct eingefuehrt und besitzt sogar eine definierte Default-Repräsentation, aber ein bereits bestehendes explizites `Struct { ... }`-Literal initialisiert das Feld nicht. Rust uebernimmt den Default dort nicht implizit. Der Candidate kann PRE-TAKE-Gates passieren und erst im Strict-Clippy/Build an einem rein mechanischen Compilefehler scheitern.

Gegenmittel:

- bei jeder Struct-Felderweiterung vor Packaging alle expliziten `StructName { ... }`-Initialisierer in `src/` und `tests/` suchen;
- fuer jedes neue Pflichtfeld jeden solchen Initialisierer explizit schliessen oder bewusst `..Default::default()` verwenden, sofern der bestehende Architekturvertrag das erlaubt;
- `derive(Default)` oder `impl Default` nicht mit automatischer Vervollstaendigung expliziter Struct-Literale verwechseln;
- die Anzahl gefundener Initialisierer als Preflight-Witness festhalten, damit die Recovery nicht nur den zuerst vom Compiler gemeldeten Fund repariert;
- ein nach Compilefehler verworfener Candidate bleibt Nicht-Parent; der Recovery wird unabhaengig auf dem letzten akzeptierten gruene Parent gebaut.


## 15. MusicMutation erweitert indirekt Audit-WriterReachability

Symptom aus r757:

```text
Semantic-Capability-Diff: MusicMutation darf Audit-WriterReachability nur streng verengen, niemals erweitern oder umhaengen
```

Bedeutung: Bei `MusicMutation` ist nicht nur der direkte Writer-Owner relevant. Wenn unter einem unveraenderten Audit-/Report-Caller neue produktive Writer-Roots erreichbar werden, erweitert der Candidate dessen abgeleitete Schreibreichweite und ist unzulaessig. Auch eine neue Funktion innerhalb `structural_write_authority` kann deshalb verboten sein, wenn sie als neue Root in der globalen Reachability-Menge auftaucht.

Gegenmittel:

- vor Packaging Parent und Candidate als Writer-Root-Mengenvergleich pruefen;
- im Normalfall `after_roots == before_roots` verlangen;
- keine neue Writer-Root nur deshalb einfuehren, weil sie im kanonischen StructuralWriteAuthority-Pfad liegt;
- bestehende Sink-Roots unveraendert wiederverwenden;
- Macro-/bestehende Sinks nicht durch neue explizite Root-Identitaeten ersetzen;
- MultiRevision-Warnungen als Hinweis auf Capability-Verschiebung ernst nehmen.

Der r757-Fall zeigt: **Writer-Ownership und Writer-Reachability sind zwei getrennte Beweise.**


## 16. Bestehende Writer-Root, aber neuer Caller ausserhalb StructuralWriteAuthority

Symptom aus r758:

```text
NO-NEW-REPAIR-AUTHORITY KODEX GATE
... reason=new_writer_caller_outside_structural_write_authority
writer_roots=...set_pitched_note_duration_type_clear_tie_and_beams_authorized
```

Bedeutung: Die Gleichheit der globalen Writer-Root-Menge reicht nicht. Eine neu eingefuehrte Funktion ausserhalb `src/structural_write_authority/` darf auch **keine bereits bestehende** Writer-Root neu erreichen. Ein vorhandener autorisierter Writer ist keine allgemeine Utility-Funktion fuer neue Pipeline-Caller.

Gegenmittel:

- fuer jede neue oder semantisch geaenderte produktive Funktion ausserhalb StructuralWriteAuthority die transitive WriterReachability pruefen;
- `writer_roots(function) == empty` fuer neue nicht-Structural-Helper verlangen;
- vorhandene Writer nicht als Duration-/XML-Hilfsbibliothek missbrauchen;
- Kandidatenprojektion writerfrei halten und nur den bereits vorhandenen Sink schreiben lassen;
- bei benoetigter reiner Transformation bestehende Policy/Werte wiederverwenden, aber keinen Writer-Call als Berechnung missbrauchen;
- MultiRevision-Warnungen weiter beobachten, aber keine Capability durch Verschieben oder Umbenennen tarnen.

Der r758-Fall ergaenzt r757: **Root-Mengen-Gleichheit und Caller-Topologie sind zwei getrennte Gate-Beweise.**


## 17. MusicMutation fuehrt neue FeedbackIdentityChannel ueber rohe Taktidentitaet ein

Symptom aus r759:

```text
Semantic-Capability-Diff: Klasse MusicMutation verletzt die semantische Capability-Policy:
  - added FeedbackIdentityChannel ... detail=measure_number
```

Bedeutung: Ein neuer produktiver MusicMutation-Helper darf keine neue rohe Feedback-/Identity-Kante etablieren. Schon ein primitiver Identitaetsmarker wie `measure_number` in Signatur oder Funktionskoerper wird als eigene `FeedbackIdentityChannel`-Capability inventarisiert. Das gilt auch dann, wenn die Funktion nur eine bereits gebundene musikalische State serialisieren soll. Reines Umbenennen waere keine Loesung, wenn die Funktion weiterhin semantisch eine neue externe Taktidentitaet einliest; das kann zusaetzlich als MultiRevision-Capability-Evasion sichtbar werden.

Gegenmittel:

- vor Packaging fuer neue/geaenderte produktive Music-Funktionen die FeedbackIdentity-Marker des echten Semantic-Capability-Gates gegen Parent und Candidate vergleichen;
- keine neue rohe Takt-/Projekt-/Feedback-Identitaet aus XML-Attributen, Dateinamen, Reports oder Umgebungsdaten ableiten, wenn die benoetigte Position bereits durch gebundene State/Ordnung bestimmt ist;
- bei reiner Serialisierung die bereits versiegelte interne zeitliche Ordnung konsumieren und XML-Struktur nur positional ablaufen, statt eine zweite Identitaetswahrheit aus `<measure number=...>` zu etablieren;
- wenn echte externe Identity fachlich benoetigt wird, nicht tarnen oder umbenennen, sondern Owner/ChangeClass/Authority explizit klaeren;
- MultiRevision-Warnungen nicht als Freibrief behandeln: Capability-Verschiebung durch semantiknahe Tarnung bleibt verboten.

Der r759-Fall zeigt: **Writerfreiheit allein macht eine neue Music-Projektion noch nicht capability-neutral; Identitaetskanaele sind ein separater Gate-Beweis.**

## 18. Render-Teiloberflaeche: Frozen-Domaene partiell, vorhandene Carrier trotzdem vollstaendig erklaert

Symptom aus r760:

```text
FrozenTemporalProjection attack coverage mismatch: bound=1073 expected=1074
```

Spaetere Forensik am gruenen r756-Master/Render zeigt: Der Render besitzt 1.074 Attack-Starts, und alle 1.074 sind unter `(measure, onset, pitch, staff)` eine echte Teilmenge der 1.274 Master-Attacks; es gibt keinen Render-only-Attack. Damit war 1073/1074 kein Beweis dafuer, dass ein vorhandener Legacy-Pitch-Carrier unerklaert bleiben darf, sondern ein Binding-Defekt innerhalb einer legitimen Frozen-Teilmenge.

Gegenmittel:

- Master: vollstaendige Frozen-Domaene **und** vollstaendige Erklaerung aller vorhandenen pitched Carrier verlangen;
- Render: Frozen-Domaene darf partiell sein, aber jeder vorhandene pitched Render-Carrier muss genau einem Frozen-Attack gehoeren;
- `bound_frozen < active_frozen` kann im Render legitim sein, `owned_source < existing_source` nicht;
- Subset-Semantik niemals als Freigabe fuer ungeklärtes klingendes Legacy-Material interpretieren;
- Parse-, RawCursor- und `rest+tie=0`-Invarianten bleiben in beiden Modi fail-closed.

Der r760-Fall zeigt: **Partial coverage beschreibt die Domaene der kanonischen Wahrheit, nicht eine Lockerung der Eigentums-/Erklaerungspflicht fuer tatsaechlich vorhandene Carrier.**


## 19. Selbstgeschriebene Repraesentationsfelder duerfen Carrier-Identitaet nicht zyklisch bestimmen

Symptom aus r761:

```text
FrozenTemporalProjection attack start not found:
measure=16 onset=192 pitch=85 staff=1 voice=1

realer pitched Carrier:
voice=3
```

Bedeutung: Wenn derselbe Cut die finale technische Voice aus einer bereits gebundenen Frozen-Lane-Authority neu schreibt, darf die alte XML-Voice nicht zugleich notwendiger Bestandteil der Carrier-Identitaet sein. Sonst verlangt die Reparatur die Korrektheit genau der Repraesentation, die sie ersetzen soll. Dasselbe Prinzip gilt fuer andere Felder, die der Cut selbst autoritativ neu materialisiert: Sie duerfen nur Identitaetsbestandteil bleiben, wenn ihre Erhaltung vor dem Binding bewiesen ist.

Gegenmittel:

- opaque kanonische Identitaet (`SoundingAttackId`/SourceEventId) bis zur Binding-Grenze erhalten;
- physische Carrier nur ueber vor dem Cut erhaltene, runtime-eindeutige Koordinaten lokalisieren;
- Legacy-Voice, alte Tie-Struktur und alte Duration nicht als Wahrheit fuer die neue Materialisierung verwenden;
- Source-seitige Locator-Kollisionen und Frozen-seitige Locator-Kollisionen getrennt fail-closed beweisen;
- jeden gebundenen physischen Carrier genau einem Frozen-Attack zuordnen; zweiter Besitzer ist ein harter Fehler;
- Binding/Ownership/Coverage vollstaendig beweisen, bevor projizierte MusicXML-Fragmente erzeugt werden.

Der r761-Fall zeigt: **Provenienz darf mitversiegelt werden; sie wird dadurch nicht automatisch zur autoritativen Carrier-Identitaet.**



## 20. PhysicalStart muss aus dem aktuellen Gate-Befund stammen, nicht nur aus einem aelteren Export-Snapshot

Symptom aus r764:

```text
uebergebener Export-Snapshot: r762
r764 deklarierte Rueckbau von zwei Pipeline-Dateien + r762-Doku
Take-Change-Control: declared_but_unchanged fuer genau diese drei Pfade
```

Bedeutung: Ein vollstaendiger Assistant-Export ist eine belastbare historische Source-Evidence, aber nach weiteren fehlgeschlagenen/transactionalen Take-Vorgaengen nicht automatisch die letzte physische Byte-Wahrheit des Arbeitsbaums. Fuer einen Recovery-Take ist der aktuelle fail-closed Take-Change-Control-Befund die staerkere PhysicalStart-Evidence.

Gegenmittel:

- fachliche Authority und aktuellen PhysicalStart getrennt fuehren;
- exakte Parent-Bytes nur aus verifizierten Paketen beziehen;
- bei `declared_but_unchanged` nicht das Gate umgehen, sondern die Recovery-Deklaration auf die tatsaechlich noch offenen Pfade reduzieren;
- abgelehnte Kandidaten-SHA und Revision nie wiederverwenden;
- kein produktiver Rueckbau deklarieren, wenn der aktuelle Baum bereits byteidentisch auf Authority steht;
- Green Base erst nach realem `dark_green / released / FinalExit 0`.

Der r765-Fall zeigt: **PhysicalStart ist eine aktuelle Byte-Eigenschaft, keine bloss aus einer Snapshot-Revision abgeleitete Behauptung.**

## 21. Writer-artiger Helper wird unbeabsichtigt zum Capability-Root

Symptom aus r822:

```text
WriterReachability 188 -> 190
neue Roots nur durch zwei neu eingefuehrte Helper mit writer-artiger Identitaet
```

Bedeutung: Ein semantisch enger Continuation-/Proof-Helfer kann durch die aktuelle Root-Erkennung selbst zu einer neuen Structural-Write-Capability werden. Dass sein Funktionskoerper keinen neuen fachlichen Writer enthaelt, reicht dann nicht als Beweis. Ein kosmetischer Rename waere ebenfalls falsch, weil die Multi-Revision-Semantik Capability-Evasion gerade verhindern soll.

Gegenmittel:

- vor Packaging die vom aktuellen Gate erkannte Writer-Root-Menge Parent gegen Candidate vergleichen;
- einen unnoetigen writer-artigen Wrapper nicht tarnen, sondern die falsche Capability-Grenze physisch entfernen;
- reine Continuation-/Proof-Logik unter einem semantisch engeren bestehenden Owner halten, sofern sie keine Write-Authority besitzt;
- Rename, Wrapper-Splitting oder Formatierungswechsel nie als Capability-Retirement-Beweis verwenden.


## 22. Repraesentationswechsel wird faelschlich als neue Identitaet behandelt

Symptom aus r825:

```text
Pre-Clef Receipt vorhanden
FinalNotation/Clef-Write veraendert legitime XML-Repraesentation
spaeterer Writer sieht Post-Clef-Bytes
alter Receipt ist an Pre-Clef-Bytes gebunden
```

Bedeutung: Eine legitime Transformation kann die Repraesentation aendern, ohne die bereits gebundene Occurrence-/Owner-Identitaet zu ersetzen. Wird danach Identitaet aus dem Output neu gesucht, entsteht wieder inverse Rekonstruktion. Wird dagegen nur der alte Byte-Seal weiterverwendet, ist der Zustandsbeweis veraltet.

Gegenmittel:

- bekannte Identity kausal vorwaerts transportieren;
- nach legitimer Repraesentationsaenderung exakten Readback des neuen Zustands ausfuehren;
- Representation-Seal/Receipt fuer die neue Epoche erneuern, ohne neue Identity aus dem Output zu minten;
- Merksatz: **Provenienz traegt Identitaet. Readback beweist Zustand.**


## 23. Diff-Parent und Release-Historie werden verwechselt

Symptom aus r826:

```text
inhaltlich korrekter Warning-Debt-Cut
Change-Control aber gegen einen zu alten Parent deklariert
declared/actual semantics widersprechen dem real installierten Source-Baum
```

Bedeutung: Der fuer einen konkreten Take relevante Vergleichsstart ist eine aktuelle Byte-Eigenschaft des installierten beziehungsweise nach Rollback wiederhergestellten Source-Baums. Ein langfristig letzter gruen erinnerter Stand oder ein aelterer Export ersetzt diesen PhysicalStart nicht. Umgekehrt wird ein abgelehnter und zurueckgerollter Candidate nicht dadurch Parent, dass er lokal schon einmal installiert war.

Gegenmittel:

- Parent-/PhysicalStart-Bytes aus dem aktuellen Repository bzw. den aktuellen Take-Gates bestimmen;
- `undeclared=[]` und `declared_but_unchanged=[]` gegen genau diesen Baum schliessen;
- nach Rollback den wiederhergestellten gruenen Stand als Basis verwenden;
- Release-Entscheidung, historische Revision und aktuellen Diff-Start nicht zu einer einzigen Begriffsachse vermischen.


## 24. Produktpfad tot, Capability-Oberflaeche lebt kuenstlich weiter

Symptom aus r834:

```text
kein produktiver MXL-Reader/Writer-Call
write_mxl_package/read_mxl_main_musicxml nur noch ueber MxlOptionalApiKeepalive referenziert
```

Bedeutung: "Kein produktiver Producer" beweist noch nicht, dass die zugehoerige Capability physisch verschwunden ist. Keepalive, Reexport, Funktionszeiger, Test-Only-Liveness oder unaufgerufene API koennen eine alte Oberflaeche weiter im Source halten.

Gegenmittel:

- Retirement in drei getrennten Fragen pruefen: produktive Reachability, verbleibende Capability-Oberflaeche, verbleibende destruktive/administrative Authority;
- kuenstliche Liveness nicht als Grund zum Behalten eines nachweislich toten Produktpfads verwenden;
- erst nach kompletter Referenzinventur die tote API physisch entfernen.


## 25. Historische Loesch-Authority liegt in mehreren Ownern und kann Custom-Pfade treffen

Symptome aus r835/r836:

```text
Pipeline loescht obsolete_mxl_out / obsolete_questions_out / _work/<stem>.render.musicxml
Launcher klassifiziert .mxl unabhaengig ebenfalls als stale standard output
legacy _work-Pfad wird aus dem Parent eines custom musicxml_out abgeleitet
```

Bedeutung: Eine alte Destruktionsauthority kann nach dem Ende ihres Producers an mehreren Stellen weiterleben. Ein lokaler Cleanup-Rueckbau ist dann unvollstaendig. Bei aus frei waehlbaren Outputpfaden abgeleiteten Legacy-Zielen kann die Destruktionsflaeche sogar ausserhalb des normalen Projekt-Outputsets liegen.

Gegenmittel:

- fuer jedes historische Artefakt Producer **und alle** Delete-/Cleanup-Owner repo-weit inventarisieren;
- Custom-Outputpfade und daraus abgeleitete Seiteneffekte explizit pruefen;
- Pipeline- und Kodex-/Launcher-Owner gemaess ChangeClass-Grenzen getrennt pensionieren, statt Domains fuer einen Sammel-Take zu vermischen;
- nach dem Cut Project-File-Catalog und Root-Hygiene darauf pruefen, ob liegenbleibende Altartefakte eine reale Release-Schuld erzeugen.


## 26. Permanent wahrer Bool ist zugleich Contract-Witness

Symptom aus r838:

```text
REQUIRE_ZERO_EXTERNAL_MUSICXML_SURFACES: bool = true entfernt
produktiver Zero-Surface-Check weiterhin direkt fail-closed
integrierter --bin etuschlopf Test trotzdem rot
```

Bedeutung: Ein permanent wahrer Bool kann zwei Rollen besitzen: Steuerung eines inzwischen toten Disabled-State **und** maschinengepruefter Single-Owner-/Contract-Marker. Das Entfernen der Steuerfunktion ist dann legitim, das Entfernen des unabhaengigen Besitznachweises aber nicht.

Gegenmittel aus r839:

- vor Retirement alle Konsumenten eines scheinbar toten Schalters inventarisieren, einschliesslich Literal-/Source-Witness-Tests;
- Steuerrolle und Witness-/Ownership-Rolle getrennt bewerten;
- wenn der Marker weiterhin Contract-Evidence ist, Marker erhalten und nur die bedingte Gate-Off-Steuerung entfernen;
- Regressionstest nicht abschwaechen, nur damit der Retirement-Candidate gruen wird;
- Warnungen und harten Reject getrennt lesen: r838 zeigte MultiRevision-Warnungen, der tatsaechliche Reject kam jedoch erst vom gebundenen Bin-Test.


## 27. Proof-Amplification und falsche Representation-Epoch-Bindung

Symptome aus der r849-r859-Serie:

```text
gleicher Full-Proof unmittelbar zweimal auf effektiv identischen Inputs
lokale autorisierte Representation-Aenderung -> komplette bereits geschlossene Proof-Welt wird neu aufgebaut
spaeter layout-aware Validator prueft erneut alte initial_render-Bytes statt die inzwischen persistierte Layout-Epoche
Proof-Retirement macht alten Getter/Reexport tot
```

Bedeutung: Viele Checks sind nicht automatisch viel Sicherheit. Ein zweiter Full-Proof liefert nur dann zusätzliche Assurance, wenn seit dem letzten gültigen Zustand eine seiner tatsächlichen Dependencies geändert wurde, eine echte Trust-/Persistenzgrenze überschritten wurde oder andere Evidenz eine zusätzliche Fehlerklasse absichert. Wird dagegen ein bereits geschlossener Zustand über eine eng autorisierte Representation-Änderung transportiert, darf unveränderte Proof-Wahrheit erhalten bleiben. Ein später Validator muss zugleich an die tatsächlich persistierten Bytes seiner eigenen Representation-Epoche gebunden sein; ein alter In-Memory-Snapshot beweist den neuen Artefaktzustand nicht.

Gegenmittel:

- `EffectiveInputs(V1)` und `EffectiveInputs(V2)` vollständig vergleichen, nicht nur sichtbare Parameter;
- lokale Delta-/Continuation-Proofs kompositionell verwenden und nur geänderte Dependencies neu beweisen;
- persistierte Bytes nach realem letzten Write erneut lesen und den Proof an genau diese Epoche binden;
- Independent Assurance, Renderer-/Side-Effect-Grenzen und echte Readbacks nicht als Redundanz wegoptimieren;
- nach Proof-Retirement tote Getter/Reexports/API mit pensionieren, statt Keepalive-Struktur zu erzeugen;
- wenn Kompression neue Registry, Ledger, globale Dependency-Runtime oder viele Spezialtypen verlangt: stoppen und Architekturannahme neu prüfen.

## 28. Proxy-Identität durch target-conditioned Merkmalsverengung

Symptom: Ein Audit beginnt bei einem bereits bekannten Problem-Witness und fügt formal allgemeine Strukturmerkmale hinzu, bis korpusweit nur noch dieser Witness übrig bleibt. Projektname und Taktnummer stehen nirgends im Selektor, trotzdem entspricht die Merkmalskonjunktion praktisch einer versteckten Zieladresse.

Historischer Gegenbeleg: Der technisch grüne r1030-Candidate `PostRestThreeBeatCrossVoiceBeamDebtIntersectionAudit` nahm den sichtbaren E6/T1-Beamfehler als Ausgangspunkt. Nachdem eine allgemeinere Drei-Beat-Struktur zusätzlich E4/T21 erfasste, wurde die führende Ganz-Zählzeit-Pause als weiteres Merkmal hinzugefügt und machte E6/T1 eindeutig. Die Pause war jedoch nicht unabhängig als kausaler Bestandteil der Beam-Fragmentierungs-Fehlerklasse begründet; sie diente faktisch der Eliminierung des Gegenfalls. r1030 ist deshalb fachlich rejected Failure Evidence und kein Parent.

Bedeutung: **Identity-free Syntax ist nicht automatisch identity-free Semantik.** Ein Selektor kann einen bekannten Fall als Proxy-Identität kodieren, obwohl er weder Etüdennamen noch Taktnummern enthält. Einzigartigkeit im aktuellen Korpus ist kein Beweis für Kausalität.

Gegenmittel:

- Fehlerklasse vor Kenntnis ihrer Korpusverteilung ausschließlich aus kausal notwendigen Merkmalen definieren;
- bekannten Witness gedanklich entfernen und für jedes Merkmal fragen, ob die Klassendefinition unverändert bliebe;
- Korpusverteilung erst nach der Definition messen und nicht als Feature-Optimizer benutzen;
- Einzigartigkeit nur als mögliches Ergebnis akzeptieren, niemals als Konstruktionsziel;
- Fehlerklasse und Repair-Selector getrennt halten;
- Repair-Scope erst in einem späteren Schritt aus einer bereits akzeptierten Fehlerklasse ableiten und nur wegen Authority, Safety, Materialization oder Invarianten weiter verengen;
- rein diskriminierende Kontextmerkmale wie Randlage, vorausgehende Pause oder exakte Gruppenanzahl besitzen ohne eigenen Kausalbeweis kein Reparaturrecht;
- die kausale Fehlerklasse muss zuerst als `error_class_definition` **ohne neuen selector-sensitiven Code** in einem akzeptierten Parent eingefroren werden; erst ein späterer `error_classification`- oder `repair_scope`-Take darf darauf verweisen; der Parent-Vertrag bleibt im Implementierungs-Take unverändert.



## 29. Scoreweite Einzigkeit oder Ranking wird zur Proxy-Identity fuer unabhaengige Kandidaten

Symptom aus dem r1034-Altlastenfund:

```text
mehrere lokal plausible Kandidaten
-> scoreweit sortieren / len()==1 / first()/next() / take(1) / max_actions=1
-> genau einen Kandidaten schreiben oder bei Multiplicity gar keinen
```

Konkrete lebende Beispiele waren unter anderem `RhythmicSpineOneTenutoRenderWrite`, `SingleClearStaccatoRenderWriteWithPlacementReadback`, der Partial-Beat-Post-Rest-Triple-Beam-Owner-Transfer sowie mehrere Rest-/Cross-Staff-Writer mit `max_actions=1`. Beim Rhythmic-Spine-Pfad wurde nach einer strukturellen Kandidatensuche global gerankt, bei Gleichstand sogar nach Taktreihenfolge, danach nur `next()` genommen und innerhalb der Lane eine Hinge-Position bevorzugt. Das ist syntaktisch identity-frei, kann aber faktisch einen einzelnen bekannten Ort reproduzierbar adressieren.

Bedeutung: Eine allgemeine lokale Regel ist nicht multiplicity-invariant, wenn ein zweites **unabhaengiges** gueltiges Vorkommen die Entscheidung des ersten aendert. `1 -> 2 Kandidaten` darf fuer unabhaengige Kandidaten weder `write -> no-write` noch `write first only` erzeugen. Scoreweite Einzigkeit ist keine musikalische Authority. Globales Ranking ist keine lokale Authority. Ein Cap von 1 ist keine Sicherheitsgarantie, wenn kein gemeinsamer Konflikt zwischen den Kandidaten bewiesen ist.

Gegenmittel:

- Kandidaten zuerst in **unabhaengige lokale Entscheidungen** versus **alternative Hypothesen derselben Konflikt-/Transaktionsgruppe** klassifizieren;
- fuer unabhaengige Kandidaten Candidate-Duplication/Multiplicity-Invariance verlangen: `Decision(a | {a}) == Decision(a | {a,b})` und `Decision(b | {b}) == Decision(b | {a,b})`;
- scoreweite `len()==1`, globales `first/next` nach Ranking, `take(1)` und produktive `max_actions=1` duerfen fuer unabhaengige Kandidaten kein WriterOpen erzeugen;
- legitime Ambiguitaetsguards bleiben innerhalb einer explizit gebundenen lokalen Konflikt-/Transaktionsgruppe fail-closed zulaessig;
- bestehende Altlasten nicht blind verbreitern: erst lokale Authority beweisen, sonst Writer pensionieren oder auf AuditOnly zurueckfuehren;
- neue selector-sensitive Implementierungen muessen ab r1035 den ausfuehrbaren `candidate_duplication_v1`-Vertrag erfuellen.

## 30. Writer-Retirement als scheinbare Selector-Neudefinition

Symptom aus r1036-r03: Eine fachlich beabsichtigte Pensionierung vorhandener Writer-Authority veraendert gleichzeitig den Legacy-Selector-Code so stark, dass TakeChangeControl korrekt eine neue selector-sensitive Entscheidung erkennt und einen bereits akzeptierten Error-Class-Contract verlangt.

Bedeutung: Ein Authority-Verlust ist nicht automatisch selector-neutral. Wird beim Retirement gleichzeitig Ranking, `next()`, Target-Auswahl, Multiplicity oder Threshold-Logik entfernt/umgebaut, vermischt der Take mehrere semantische Transitionen. Das darf nicht durch einen Retirement-Ausnahmemodus kaschiert werden.

Gegenmittel ab r1036:

- `authority_retirement_v1` ist nur fuer reinen monotonen Verlust produktiver Writer-Authority zulaessig;
- Candidate-/Selector-/Multiplicity-Relation im Retirement-Take unveraendert lassen, auch wenn sie danach nur noch read-only ist;
- `CalibrationThreshold` im Writer-Retirement unveraendert lassen;
- keine Ersatzwriter-/positive Write-Entscheidung einfuehren;
- `causal_class_first_v1` und `candidate_duplication_v1` niemals umgehen;
- read-only Selector-/Threshold-Altlasten erst in getrennten Folgetakes entfernen.

### r06 Source-Fact-Owner-Binding-Recovery

r1036-r06 passierte den PRE-TAKE-DIFF-GATE, wurde installiert und scheiterte erst im Architektur-Build-Gate, weil TakeChangeControl den path-gebundenen Einzelimport `rust_source_scan_primitives::fnv1a64` zu einem Sammelimport mit `mask_non_code` erweitert hatte. Ein neuer Governance-Check darf bestehende Source-Fact-Owner-Grenzen nicht durch bequeme Direktimporte erweitern. Recovery: bestehenden Import byte-strukturell erhalten und vorhandene TakeChangeControl-Diff-Primitiven verwenden; keine neue Source-Fact-Bindung erzeugen.

### r07 ChangeManifest-Test-Fixture-Vollstaendigkeit

r1036-r07 passierte PRE-TAKE-DIFF-GATE, Architektur-Build-Gate, Clippy und Build, scheiterte aber beim Kompilieren der integrierten Tests mit E0063: Eine bestehende testinterne `ChangeManifest`-Struct-Initialisierung setzte das neu eingefuehrte optionale Feld `authority_retirement_contract` noch nicht. Gegenmittel: Bei Schemaerweiterungen alle direkten Struct-Fixtures mit einem expliziten neutralen Wert aktualisieren; fuer Nicht-Retirement-Fixtures ist das `None`. Keine Governance-Semantik wurde dadurch geaendert.

## 31. Taktnummer als Wort oder Ziffer im Test-XML

Symptom aus dem T34-Fix (nach r1475): Die Gate-Checkliste empfahl `number="probe"` statt `"1"`. Das erfuellt die Corpus-Independence-Regel, bricht aber jeden Binder, der Taktnummern numerisch liest (14 Testfehler). Umgekehrt sagt ein Einzeltakt-Test mit fester Nummer nichts darueber, ob ein Writer von Taktposition, Divisions-Vererbung oder Labels abhaengt.

Gegenmittel ab dem Verschiebungs-Pruefstand:

- Taktnummern in Test-XML nur ueber Variablen oder `tests/support/synthetic_score.rs`, nie als Ziffer oder Wort;
- Writer-Tests mit Einzeltakt-Fixture zusaetzlich durch `assert_shift_invariant` fuehren: umnummeriert, vorangestellte Takte (auch mit anderen Divisions), geerbte Attribute, angehaengt, verdoppelt, Part-Label, Metadaten, Stimmlabel, kombiniert;
- erwartet wird im Falltakt dasselbe Ergebnis wie im Einzeltakt und in allen umgebenden Takten keine Aenderung;
- wird ein Verschiebungstest rot, ist das ein Befund (echte Positions- oder Labelabhaengigkeit), kein Anlass, die Variante zu entfernen.

