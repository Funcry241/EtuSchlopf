# TakeIdentityLifecycleBoundSourceWitnessPreservationClosure r521

## Ausgangspunkt

Der installierte Parent ist r514 `FinalNotationTimelineClearanceSystemReflowCutover`.
Der erste r515-Kandidat wurde nach transaktionaler Installation im autoritativen Reexec durch
Strict Clippy abgewiesen und auf r514 zurueckgerollt. Der damalige Bootstrap persistierte
Post-Apply-/Reexec-Fehler jedoch nicht als Rejection-Provenienz. Dadurch blieben der alte und der
korrigierte r515-Payload unter derselben `(Version, Revision)` beide formal nicht abgewiesen und
die nachfolgende Discovery blockierte korrekt, aber nicht selbstheilend, als mehrdeutig.

Der zuerst gebaute r516-Lifecycle-Kandidat wurde vom autoritativen RUST SOURCE FACT OWNER BINDING Gate verworfen, weil er die bestehende path-stabile FNV-Negativpruefung im Bootstrap ersetzte. r521 ist deshalb kein musikalischer Ersatz fuer r515 und kein Gate-Workaround. Es ist ein `KodexOnly`-Lifecycle-Abschluss
auf r514. Seine hoehere Revision kann die unklassifizierten r515- und den gescheiterten r516-Payload ohne manuelles Loeschen
ueberholen. Der fachliche Rest-Placement-Debt-Retirement wird erst auf dem real gruenen r521-Stand
unter neuer Revision erneut angeboten.

Der anschliessende r519-Kandidat bewahrte die FNV-Negativpruefung, wurde jedoch im autoritativen Build-Gate `TAKE BOOTSTRAP PUBLIC RECOVERY ORDER REGRESSION` verworfen, weil die Public-Artifact-Recovery semantisch aus dem gebundenen `fn rollback(...)`-Einstieg in einen Provenienz-Helfer verschoben worden war. r521 bewahrt daher zusaetzlich diese oeffentliche Recovery-vor-Source-Mutation-Kante.

Der danach im Suchraum vorhandene r518-Kandidat `TakeConsoleEssenceDiagnosticBoundaryClosure` wurde noch vor dem Kopieren vom Take-Change-Control verworfen, weil sein tatsaechlicher Payload drei nicht deklarierte Dateien aus dem frueheren Rest-Placement-Debt-Retirement enthielt. r521 basiert deshalb weiterhin ausschliesslich auf dem installierten r514-Parent, ueberholt die niedrigeren unklassifizierten/abgewiesenen Kandidaten revisionsseitig und nimmt keinerlei r515-Debt-Datei in diesen KodexOnly-Lifecycle-Take auf.

## Geschlossene Lifecycle-Luecken

### 1. Failed Candidate bleibt nicht mehr unklassifiziert

Die existierende Candidate-SHA-Provenienz wird nun ueber den gesamten transaktionalen Take-Lauf
mitgefuehrt. Nach sicherem Rollback werden auch folgende Fehler als Rejection persistiert:

- Apply/Verify-Fehler,
- Root-Cargo.lock-Resolution-Fehler,
- nicht startbarer Reexec,
- Reexec mit Exit ungleich 0,
- beim naechsten Bootstrap erkannte unterbrochene Transaktionen in `applying`, `applied`, `reexec`
  oder `rolling_back`.

Damit kann derselbe fehlgeschlagene Payload nicht beim naechsten `--take` als scheinbar neuer
nicht-abgewiesener Peer wieder in die Auswahl gelangen.

### 2. Acceptance ist Projektgedaechtnis fuer Discovery

Der bestehende MultiRevision-Ledger besass bereits Acceptance-Provenienz, wurde aber von der
Take-Discovery nicht gelesen und band Acceptance noch nicht explizit an die Revision. r521 macht
diese Provenienz nutzbar und bindet sie als `(Version, Revision, Payload-SHA)`-Identitaet.

Bei mehreren Dateien mit derselben `(Version, Revision)` gilt nun:

1. exakt rejected SHAs werden wie bisher ignoriert;
2. existiert unter den verbleibenden Peers genau eine bereits akzeptierte SHA, ist diese die
   kanonische Payload;
3. abweichende Same-Identity-Peers werden logisch als `superseded` ignoriert; die Dateien werden
   weder geloescht noch umbenannt;
4. existieren mehrere unterschiedliche Payloads ohne entscheidende Acceptance-/Rejection-
   Provenienz, bleibt die Auswahl fail-closed mehrdeutig.

Die Identitaetspruefung verwendet fuer diese Provenienz SHA-256, nicht Dateiname, Dateipfad oder
einen schwacheren Dateifingerprint.

### 3. Commit-Provenienz ist recovery-faehig

Die offene Take-Transaktion speichert jetzt Candidate-From/To-Version, Take-Revision, ZIP-SHA-256
und den bereits erzeugten semantischen Revision-Snapshot neben dem Backup.

Nach Reexec 0 wird zuerst der Transaktionszustand `committed` persistiert, danach die Acceptance-
Provenienz geschrieben und erst dann Pending-Marker/Transaktionsordner entfernt. Scheitert der
Provenienzschritt oder endet der Prozess dazwischen, kann der naechste Bootstrap den committed
Stand anhand derselben Candidate-Provenienz als akzeptiert vervollstaendigen, ohne die bereits
erfolgreich ausgefuehrte Source zurueckzurollen.

Unterbrochene nicht-committete Transaktionen werden dagegen auf den Parent zurueckgerollt und
anschliessend als Rejection klassifiziert. Die Candidate-Provenienz bleibt bis zum erfolgreichen
Rejection-Ledger-Commit erhalten; ein bereits sicherer `rolled_back`-Zustand kann deshalb beim
naechsten Ein-Befehl-Lauf die Klassifikation vervollstaendigen.

## Essenz

Die repository-residente Essenz erhaelt einen ausdruecklichen `Take-Identity-Lifecycle-Vertrag`:

- `(Version, Revision)` besitzt genau eine kanonische Payload;
- ein anderer Payload unter derselben Identitaet ist Identitaetskonflikt, kein normaler
  Same-Version-Folgetake;
- fail-closed Mehrdeutigkeit bleibt hart;
- regulaer erreichbare Failed-Candidate-Zustaende muessen trotzdem in den Ein-Befehl-Workflow
  zurueckfuehren;
- accepted/rejected/superseded Provenienz ist Projektgedaechtnis fuer Take-Discovery und erteilt
  keinerlei MusicMutation-/Writer-Authority.

Der nummerierte Essenzindex E00-E31 bleibt unveraendert. Der Lifecycle-Vertrag ist eine explizite
Querschnittsregel; der bestehende Cold-Start-Readback muss deshalb nicht in einen gemischten
Audit-/Kodex-Take gezogen werden.

## Path-stabile Negative-Check-Erhaltung

Die revisionsgebundene SHA-256-Provenienz erweitert die bestehende Discovery, ersetzt aber keine
path-stabile negative Sicherheitspruefung. Wenn Acceptance-Provenienz einen Peer eindeutig
kanonisiert, darf Discovery diesen bekannten Zustand aufloesen. Wenn keine Provenienz entscheidet,
bleibt die bereits autorisierte FNV-basierte Same-Identity-Mehrdeutigkeitspruefung im
`etuschlopf_take_bootstrap.rs` exakt als ausfuehrbarer Fallback erhalten.

Damit bleibt die bestehende `RUST SOURCE FACT OWNER BINDING`-Invariante intakt: neue
Lifecycle-Provenienz darf Security-Fakten konsumieren, aber keine gebundene negative Check-Kante
verschieben, pensionieren oder durch einen semantisch aehnlichen neuen Mechanismus ersetzen.

## Nicht geaendert

- keine Musik-, MusicXML-, Render- oder PDF-Semantik;
- keine StructuralWriteAuthority oder produktive Musik-Call-Graph-Aenderung;
- keine Repair-/Recovery-Authority fuer Musik;
- kein Lockern der Mehrdeutigkeitspruefung;
- keine automatische Loeschung oder Umbenennung von ZIPs im Projekt oder in Downloads;
- keine Sonderbehandlung von r515-SHA-Werten im produktiven Code;
- keine Versions-/Revisions-Sonderregel.

`ChangeClass=KodexOnly`.

## Public-Recovery-Order-Erhaltung

Der r519-Kandidat wurde vom autoritativen `TAKE BOOTSTRAP PUBLIC RECOVERY ORDER`-Gate verworfen, weil die bestehende oeffentliche Rollback-Reihenfolge funktional in `rollback_preserving_provenance()` verschoben worden war. r521 behaelt deshalb `fn rollback(...)` als sichtbaren autorisierten Einstieg: Public-Artifact-Recovery wird dort weiterhin vor jeder Source-Rollback-Mutation ausgefuehrt. Die provenance-erhaltende Variante verwendet dieselbe Reihenfolge; beide delegieren danach an genau einen gemeinsamen Source-Rollback-Kern. Damit erweitert der Lifecycle die Recovery-Provenienz, ohne die path-stabile Recovery-Kante zu verschieben oder einen zweiten Mutationseigentuemer einzufuehren.


## r521 – Bound Source Witness Closure

Der autoritative r520-Kandidat wurde nach PRE-TAKE-DIFF-GATE und Apply im Build-Gate `ADVERSARIAL FAILED-REEXEC ROLLBACK DEFENSE MISSING` vollstaendig zurueckgerollt. Ursache war nicht fehlende Rollback-Semantik, sondern die Entfernung des explizit gebundenen Source-Witness `transaction.rollback(&project_root)?;` aus der autorisierten Failed-Reexec-Call-Site. r521 stellt diesen produktiven Ausdruck exakt am Failed-Reexec-Pfad wieder her und schreibt erst nach dem erfolgreichen Rollback die Rejection-Provenienz in den bestehenden Ledger. Der Lifecycle bleibt eine Erweiterung des bestehenden Sicherheitsgraphen; kein gebundener Source-Witness wird abstrahiert oder ersetzt.

Die Essenz wird deshalb um zwei Beratungsregeln erweitert: Kodex-gebundene Source-Witnesses (Pfad, Call-Site, Order, FNV, Literal) sind bis zu einer ausdruecklichen Migration Teil des Owner-Vertrags; und Kodex-/Bootstrap-Takes muessen vor Packaging den gesamten Gate-Konsumgraphen des geaenderten Owners statisch inventarisieren. Die vollstaendige r514-Witness-Inventur bindet dabei insbesondere sowohl `transaction.rollback(&project_root)?;` im Failed-Reexec-Pfad als auch `transaction.commit()?;` im erfolgreichen Reexec-Pfad; r521 bewahrt beide Ausdruecke am autorisierten Call-Site und legt die neue Provenienz nur additiv darum.
