# ETUSCHLOPF – ESSENZ v0

## Zweck

Die EtuSchlopf-Essenz bewahrt die verdichtete technische Erfahrung des Projekts.

Sie beantwortet nicht primär:

„Was ist verboten?“

Das entscheiden weiterhin ausschließlich die kanonischen Contracts, Authorities und Gates.

Die Essenz beantwortet:

„Welche scheinbar plausiblen Lösungswege haben sich als gefährlich erwiesen, woran erkenne ich sie früh und in welchem Lösungsraum sollte ich stattdessen suchen?“

Die Essenz besitzt keine MusicMutation-, Writer-, Repair-, Release-, Proof- oder sonstige produktive Autorität.

Sie berät vor dem Patch.

Der Kodex entscheidet nach dem Patch.


## Regelhierarchie und Normativität

Die Essenz unterscheidet bewusst mehrere Regeltypen. Sie dürfen weder als gleichrangige Axiome abgeflacht noch zu einem zweiten normativen Regelwerk zusammenkopiert werden.

### 1. Nummerierte Essenz-Regeln

Die nummerierte Essenzfamilie besteht exakt aus:

```text
E00 ... E32
```

Diese Regeln sind `advisory only`. Sie verdichten Architekturwissen, Fehlmuster, bevorzugte Lösungsräume und Pre-Patch-Fragen. Sie erzeugen keine produktive Authority.

### 2. Separate unnummerierte Essenz-Verträge

Zusätzlich enthält dieselbe Essenzquelle vier eigenständige unnummerierte Verträge:

```text
Take-Identity-Lifecycle-Vertrag
Proof-Preservation-und-Validation-Placement-Vertrag
Completion-Boundary-Vertrag
Musikalischer-Erhalt-und-Metriksemantik-Vertrag
```

Sie sind keine zusätzlichen `E33...`-Regeln und bleiben ebenfalls `advisory only`.

### 3. Normative Architektur- und Governance-Wahrheit

Normative Erlaubnis oder Ablehnung entsteht ausschließlich aus den aktuellen repository-kanonischen Contracts, Authorities und Gates.

Der Kodex ist deshalb kein handgepflegtes, gleichrangig nummeriertes Axiomenset. Seine Wahrheit ist über aktuelle Owner, ausführbare Gates, gebundene Witnesses und explizite Governance-Lifecycles verteilt.

### 4. Mechanische Gates

Prüfungen wie Tree-Integrität, deklarierter Diff, FNV, Candidate-Fingerprint, Semantic-Capability-Diff, Writer-Reachability, Dead-Code-Integrität, Build, Tests, Formal Green und Public-Artifact-Commit sind konkrete ausführbare Invarianten ihrer jeweiligen Owner.

Sie dürfen in der Essenz erklärt oder als Preflight-Schritte referenziert werden, aber nicht als zweite normative Kopie der Gate-Wahrheit gepflegt werden.

### 5. Heuristiken und bevorzugte Lösungsräume

Formulierungen wie „bevorzugen“, „zuerst prüfen“ oder „kleiner zentraler Eingriff“ sind Arbeitsheuristiken, sofern kein aktueller kanonischer Contract dieselbe Aussage ausdrücklich als harte Invariante bindet.

Eine Heuristik darf eine fachlich legitime Capability nicht verbieten.

### 6. Konfliktauflösung

Bei scheinbarem Widerspruch gilt:

```text
aktuelles Repository
↓
task-relevanter kanonischer Contract / Authority / Gate
↓
Essenz als advisory Erklärung
↓
Fallhistorie / Übergabe / Erinnerung
```

Die Essenz darf einen harten Gate-Vertrag weder erweitern noch abschwächen.

### Reichweitenpräzisierungen

**Späte Writer.** Eine Materialisierung ist nicht allein deshalb unzulässig, weil sie spät im Pipeline-Lifecycle liegt. Unzulässig ist neue nichtkanonische Writer-, Repair- oder Recovery-Authority beziehungsweise eine Writer-Erreichbarkeit außerhalb der zuständigen kanonischen Authority.

**Writer-Reachability und Retirement.** Neue nichtkanonische Erreichbarkeit bleibt fail-closed. Gleichzeitig kann eine Capability über den ausdrücklich vorgesehenen repository-kanonischen Retirement-/Relocation-Lifecycle legitim pensioniert oder verschoben werden. Schutz gegen Widening bedeutet kein allgemeines Verbot von Governance-Retirement.

**E16 und E25.** Kleine zentrale Änderungen und strukturelle Änderungen ohne Klangänderung sind bevorzugte Lösungsräume, wenn Ursache und fachlicher Owner dazu passen. Sie sind keine absoluten Verbote gegen notwendige lokale beziehungsweise klingende Produktänderungen.

**E22 und gebundene Witnesses.** Historische Fallidentität und Patchchronik werden abstrahiert. Ein aktuell repository-gebundener Governance-Witness ist dagegen Teil des heutigen Contracts und bleibt bis zu einer expliziten Kodex-Migration erhalten.

**Tooling und Ein-Befehl-Regel.** Der kanonische Produkt-/Take-Workflow bleibt Rust-basiert hinter dem Ein-Befehl-Einstieg. Das verbietet nicht pauschal externe read-only Diagnose- oder Renderer-Experimente, solange sie keine konkurrierende Produkt-, Take-, Music- oder Writer-Authority erzeugen.

**Strict Clippy und Suppressions.** Strict Clippy bleibt ein realer Gate-Bestandteil. Zusätzliche pauschale Verbote gegen jede denkbare Lint-Suppression werden daraus nicht erfunden. Spezifisch gebundene Dead-Code-Suppressions- und Lint-Integrity-Verträge bleiben dagegen hart und werden ausschließlich über ihre aktuellen kanonischen Owner beurteilt.

---

# E00 – Repository vor Erinnerung

**Aussage**

Die Architekturkorrektheit von EtuSchlopf darf niemals von Gesprächskontext, menschlicher Erinnerung oder Assistenten-Memory abhängen.

**Warum**

Projektgeschichte beschleunigt Entscheidungen erheblich, ist aber nicht garantiert verfügbar.

**Typische Fehlform**

Ein Bearbeiter kennt eine alte Architekturgrenze nicht und rekonstruiert einen lokal plausiblen, aber bereits verworfenen Lösungsweg.

**Kanonischer Lösungsraum**

Alle für Architekturentscheidungen notwendigen Normen und wesentlichen Erfahrungsregeln müssen aus dem autoritativen Repository rekonstruierbar sein.

**Vor dem Patch prüfen**

Ist die benötigte Architekturinformation aus dem aktuellen Projektstand ableitbar, auch wenn keinerlei externe Erinnerung vorhanden ist?

**Harter Bezug**

Repository- und Gate-Wahrheit haben Vorrang vor Erinnerung und Übergabetext.

---

# E01 – Ursache vor Reparatur

**Aussage**

Ein sichtbarer Endfehler wird bevorzugt dort geschlossen, wo seine Ursache entsteht.

**Warum**

Eine späte Korrektur erzeugt leicht eine zweite Wahrheit, einen neuen Writer oder eine Repair-Schicht.

**Typische Fehlform**

`forward → orphan chord` wird am Ende erkannt und durch erneutes Einsetzen eines Hosts repariert.

**Kanonischer Lösungsraum**

Den Owner identifizieren, der den benötigten strukturellen Zustand unzulässig entfernt oder verändert.

**Vor dem Patch prüfen**

Welcher erste produktive Write erzeugt den ungültigen Zustand?

**Harter Bezug**

Keine neue Repair-Schicht. Keine neue **nichtkanonische** Late-Writer-Autorität; eine fachlich kanonische Materialisierung ist nicht allein wegen ihrer späten Pipelineposition verboten.

---

# E02 – Fachlicher Owner vor technischer Nähe

**Aussage**

Die beste Stelle für einen Patch ist nicht die Stelle, an der ein Fehler sichtbar wird, sondern die Stelle, die fachlich über die betreffende Entscheidung verfügen darf.

**Warum**

Reader, Renderer, Audit oder Serializer können technisch näher am Symptom liegen, ohne fachlicher Owner zu sein.

**Typische Fehlform**

Eine nachgelagerte Funktion erhält Entscheidungs- oder Schreibautorität, weil dort alle benötigten Daten bequem verfügbar sind.

**Kanonischer Lösungsraum**

Ursache → fachlicher Owner → Capability → reale produktive Call-Site.

**Vor dem Patch prüfen**

Wer besitzt diese Entscheidung bereits?

**Harter Bezug**

StructuralWriteAuthority und bestehende fachliche Authorities dürfen nicht umgangen werden.

---

# E03 – Detection ≠ Decision ≠ Materialization

**Aussage**

Erkennen, entscheiden und materialisieren sind unterschiedliche Verantwortlichkeiten.

**Warum**

Ein Kandidat darf sichtbar sein, ohne dass seine bloße Detektion seine Materialisierung autorisiert.

**Typische Fehlform**

Ein Detektor filtert einen Kandidaten weg, weil die spätere Materialisierung unerwünscht wäre.

**Kanonischer Lösungsraum**

Detektion bleibt vollständig. Der bestehende Decision-Owner entscheidet. Nur die zuständige Authority materialisiert.

**Vor dem Patch prüfen**

Ändert der Patch die Erkennung, obwohl eigentlich nur die Autorisierung falsch ist?

**Harter Bezug**

Keine Verlagerung von Entscheidungsautorität in Detektoren.

---

# E04 – Proof erzeugt keine Wahrheit

**Aussage**

Proof, Audit, Report und Readback beobachten Wahrheit. Sie stellen keine musikalische Wahrheit her.

**Warum**

Sobald Proof- oder Reportpfade Musik verändern dürfen, verschwimmen Beweis und Gegenstand des Beweises.

**Typische Fehlform**

Ein Reader wird verändert, damit er „den richtigen Snapshot sieht“, oder ein Proof-Reentry erhält MusicMutation.

**Kanonischer Lösungsraum**

fachlicher Writer → tatsächlicher letzter Write → persistierter finaler Snapshot → unabhängiger read-only Proof.

**Vor dem Patch prüfen**

Soll eine Proof-, Audit-, Report-, Release- oder Reader-Funktion irgendeine MusicMutation tragen oder auslösen?

Falls ja: Architektur stoppen und Owner neu bestimmen.

**Harter Bezug**

Proof/Report/Audit/Release bleiben read-only gegenüber Musik.

---

# E05 – Capability ist graphbasiert

**Aussage**

Die Capability einer Funktion ergibt sich nicht nur aus ihrem Funktionskörper, sondern auch daraus, wer sie produktiv erreichen kann. Capability-Identitaet wird deshalb semantisch verfolgt, nicht ueber Datei-, Funktions- oder Symbolnamen.

**Warum**

Ein unveränderter Writer kann durch einen neuen Caller neue effektive Autorität erhalten.

**Typische Fehlform**

„Ich habe keinen neuen Writer gebaut; ich rufe nur eine vorhandene Funktion von einer zusätzlichen Stelle auf.“

**Kanonischer Lösungsraum**

Produktiven Call-Graph Parent ↔ Candidate vergleichen. Verschieben, Umbenennen, Wrappen, Aufteilen oder semantiknahes Neuformulieren darf weder Capability-Retirement vortaeuschen noch dieselbe Capability unter neuer Identitaet wieder einfuehren.

Zusaetzlich die **Root-Erkennung des aktuellen Gates** selbst beachten: Ein semantisch enger Helper kann durch writer-artige Struktur oder einen vom Gate erkannten Writer-Root-Marker effektive Capability erhalten. Ist diese Root-Grenze fachlich falsch, wird sie nicht kosmetisch umbenannt, sondern der unnoetige Wrapper beziehungsweise die falsche Write-Nahe physisch entfernt. Multi-Revision-Semantik bleibt danach der Schutz gegen blosse Tarnung.

**Vor dem Patch prüfen**

Welche Writer-Roots erreichen die betroffene Funktion vorher und nachher?

**Harter Bezug**

Writer-Root-Reachability ist Bestandteil der Capability-Prüfung. **Capabilities werden nach Semantik verfolgt, nicht nach Namen oder Ort.**

---

# E06 – Keine neue nichtkanonische Writer-Erreichbarkeit

**Aussage**

Neue produktive Writer-Erreichbarkeit darf ausschließlich aus der dafür kanonischen Authority entstehen. StructuralWriteAuthority besitzt dabei nicht nur den Writer, sondern auch den produktiven Einstieg in den Mutation-Graphen.

**Warum**

Legacy-, Repair-, Pipeline-, Analyse-, Metric-, Calibration-, Audit- oder Render-Polish-Roots können sonst indirekt neue Schreibautorität erhalten, obwohl der letzte physische Delta-Writer formal innerhalb der StructuralWriteAuthority liegt.

**Typische Fehlform**

Ein bestehender MusicMutation-Helfer erhält einen neuen produktiven Caller aus einem nichtkanonischen Root.

**Kanonischer Lösungsraum**

Die benötigte Invariante innerhalb der bereits zuständigen kanonischen StructuralWriteAuthority schließen. Wenn keine neue produktive Aufrufautoritaet erforderlich ist, die bestehende Topologie unveraendert lassen und nur reine Evidenz beziehungsweise die bereits vorhandene Entscheidung innerhalb dieser Topologie korrigieren.

**Vor dem Patch prüfen**

Writer-Root-Menge vorher ↔ nachher. Jede neue Kante erklaeren. Insbesondere pruefen, ob bereits der produktive Einstieg in den Mutation-Graphen innerhalb der kanonischen StructuralWriteAuthority liegt.

**Harter Bezug**

`writer_reachability_widened_to_noncanonical_root` und `new_writer_caller_outside_structural_write_authority` muessen fail-closed bleiben. **StructuralWriteAuthority besitzt nicht nur den Writer, sondern auch den produktiven Einstieg in den Mutation-Graphen.**

---

# E07 – Authority muss tatsächlich leben

**Aussage**

Eine geplante Architektur existiert erst, wenn ihre produktive Call-Chain tatsächlich erreichbar ist.

**Warum**

Eine schön definierte Funktion ohne produktiven Caller besitzt keine wirksame Rolle.

**Typische Fehlform**

Neue terminale Prüfung oder Authority wird implementiert, aber nie produktiv aufgerufen.

**Kanonischer Lösungsraum**

Definition → reale Call-Site → produktiver Caller → Root → tatsächlicher Lauf.

**Vor dem Patch prüfen**

Kann die neue Funktion im realen Programmlauf erreicht werden?

**Harter Bezug**

`dead_code` und `unused import` können Architekturdiagnosen sein und dürfen nicht kosmetisch beseitigt werden.

---

# E08 – Dead Code zuerst verstehen

**Aussage**

`dead_code` ist in Architekturarbeiten zunächst ein Beweisproblem, kein Aufräumproblem.

**Warum**

Es kann zeigen, dass die behauptete produktive Call-Chain gar nicht existiert.

**Typische Fehlform**

Annotation, Export oder künstlicher Caller wird nur hinzugefügt, damit Clippy grün wird.

**Kanonischer Lösungsraum**

Erst die fehlende Liveness verstehen. Danach entweder produktiv korrekt anbinden oder die Idee verwerfen.

Bei Retirement drei Ebenen getrennt beweisen: **produktive Reachability**, **verbleibende Capability-Oberflaeche** und **verbleibende administrative/destruktive Authority**. Ein nicht mehr aufgerufener Writer/Reader ist noch nicht physisch tot, wenn Keepalive, Reexport oder Funktionszeiger ihn kuenstlich am Leben halten. Umgekehrt kann nach Entfernung der API noch ein Cleanup-/Delete-Owner existieren.

**Vor dem Patch prüfen**

Warum hält der Compiler diese Architektur für unbenutzt?

**Harter Bezug**

Strict Clippy ist Teil des Architekturbeweises.

---

# E09 – Identity ist keine versteckte Entscheidungs-Capability

**Aussage**

Etüdenname, Taktnummer, Herkunftsidentität oder semantisch äquivalente Locator dürfen keine neue produktive MusicMutation-Entscheidung ermöglichen. **Syntaktische Identity-Freiheit genügt dabei nicht:** Auch eine nach Kenntnis eines Ziel-Witness schrittweise verengte Merkmalskonjunktion ist unzulässige Proxy-Identität, wenn die zusätzlichen Merkmale nicht unabhängig aus der kausalen Fehlerklasse folgen.

**Warum**

Diagnoseidentität kann sonst unbemerkt zum Selektor für produktive Musiklogik werden. Dass `Etüde 6 / Takt 1` nicht als Literal im Code steht, schützt nicht davor, denselben Fall indirekt als `drei Gruppen + führende Pause + ...` wieder einzubauen. Eine solche target-conditioned Merkmalsverengung kann formal allgemein aussehen und trotzdem nur die bekannte Falladresse verschleiern.

**Typische Fehlform**

Ein bekannter Witness ist auffällig. Danach werden strukturelle Merkmale so lange ergänzt, bis Gegenbeispiele aus dem Korpus verschwinden und nur der bekannte Witness übrig bleibt. Die resultierende Einzigartigkeit wird anschließend fälschlich als Beweis einer allgemeinen Fehlerklasse behandelt.

**Kanonischer Lösungsraum**

Die **Fehlerklasse wird zuerst und witness-blind aus kausalen Merkmalen definiert**. Diese Definition wird als eigener `error_class_definition`-Vertrag **ohne gleichzeitig neuen selector-sensitiven Code** eingefroren. Erst ein spaeterer Take darf die eingefrorene Klasse als Klassifikationslogik abbilden und ihre Verteilung im vollständigen Korpus messen. Diese Verteilung darf die Definition falsifizieren oder zeigen, dass Evidenz fehlt; sie darf aber nicht als Feature-Optimizer benutzt werden, um die Klasse nachträglich auf einen bekannten Fall zuzuschneiden. Ein bekannter realer Witness darf nach der Definition als Evaluationsbeispiel dienen, aber nicht als dauerhafte normative Testidentitaet. Ein aus ihm abgeleiteter ausfuehrbarer Test muss seine Herkunft verlieren: synthetischer Minimalfall, keine Etueden-/Taktidentitaet, keine realen Corpus-Artefakte und dieselbe Entscheidung nach Renaming, Relocation und unabhaengiger Duplication.

Fehlerklasse und Repair-Selector sind getrennte Verträge. Ein späterer Repair-Selector darf eine bereits im akzeptierten Parent definierte Fehlerklasse nur wegen unabhängig begründeter Authority-, Safety-, Materialization- oder Invariantenbedingungen weiter einschränken. **Bloß diskriminierende Kontextmerkmale besitzen kein Reparaturrecht. Einzigartigkeit darf Ergebnis sein, niemals Konstruktionsziel.**

**Multiplicity-Invarianz gehoert zur Identity-Freiheit.** Fuer voneinander unabhaengige lokale Kandidaten darf die Existenz eines zweiten gleichartigen Kandidaten die Entscheidung des ersten weder aufheben noch auf einen global gerankten "ersten" Kandidaten umlenken. Scoreweite Kardinalitaet (`len()==1`), `first/next` nach globalem Ranking, `take(1)` oder `max_actions=1` duerfen keine Ersatz-Identity bzw. Schreibautoritaet bilden. Eindeutigkeit ist nur innerhalb einer explizit bewiesenen gemeinsamen Konflikt-/Transaktionsgruppe zulaessig. Die Duplikationsprobe ist der kanonische Gegencheck: `Decision(a | {a}) == Decision(a | {a,b})` und analog fuer `b`, sofern `a` und `b` unabhaengig sind.

Produktiver Verifier arbeitet mit strukturellen Zuständen. Menschliche Lokalisation entsteht erst hinter einer read-only Diagnosegrenze. Ist Source-/Attack-Identität vor einer bekannten Mutation bereits gebunden, wird sie explizit vorwärts mitgeführt; eine spätere Rekonstruktion aus Pitch, Onset, Staff, Voice oder räumlicher Nähe ist kein Ersatz für verlorene Provenienz.

**Provenienz traegt Identitaet; Readback beweist Zustand.** Eine legitime Repraesentationsaenderung darf einen neuen Representation-Seal/Receipt verlangen, aber sie autorisiert nicht, bekannte Identity hinter der Transformation aus dem Output neu zu erfinden.

**Vor dem Patch prüfen**

Für jedes neue Selektor-Merkmal getrennt fragen: **„Warum gehört dieses Merkmal kausal zur Fehlerklasse, wenn der bereits bekannte Ziel-Witness aus dem Korpus entfernt würde?“** Gibt es darauf keine vom Witness unabhängige Antwort, darf das Merkmal die Fehlerklasse nicht verengen. Zusätzlich prüfen: Wird gerade eine Fehlerklasse definiert oder nur ein Repair-Scope innerhalb einer bereits akzeptierten Klasse?

**Harter Bezug**

Keine neue produktive Identity-/Feedback-Capability. Die kausale Fehlerklassendefinition muss als `causal_class_first_v1`-`error_class_definition` bereits in einem akzeptierten Parent vorliegen, bevor selector-sensitive Klassifikations- oder Repair-Logik entstehen darf. Klassifikations- und Repair-Verträge müssen auf diesen unveränderten Parent-Vertrag zeigen.

---

# E10 – Diagnose darf den Gegenstand nicht verändern

**Aussage**

Ein Diagnose-Take darf zusätzliche Beobachtung liefern, aber keine neue Entscheidungsmacht erzeugen.

**Warum**

Sonst verändert die Diagnose genau den Zustand, den sie untersuchen soll.

**Typische Fehlform**

Zur besseren Fehlerlokalisation werden neue produktive Parameter oder Feedbackkanäle eingeführt.

**Kanonischer Lösungsraum**

Strukturelle, capability-neutrale Beobachtung; menschliche Interpretation nach dem produktiven Pfad.

**Vor dem Patch prüfen**

Ist diese Information für den Write notwendig oder ausschließlich für den Menschen interessant?

**Harter Bezug**

Diagnostic und MusicMutation bleiben getrennte Trust Domains.

---

# E11 – Letzter Write ist eine reale Eigenschaft

**Aussage**

Der „letzte Write“ darf nicht aus der vorgesehenen Pipeline-Reihenfolge angenommen werden. Er muss am realen produktiven Call-Graph bestimmt werden.

**Warum**

Nachgelagerte Kanonisierung oder Render-Mutation kann einen vorher korrekten Zustand wieder verändern.

**Typische Fehlform**

Ein Temporal-Gate prüft einen Zwischenstand und erklärt ihn zur finalen Wahrheit.

**Kanonischer Lösungsraum**

Tatsächlicher letzter Write → finaler persistierter Snapshot → Readback. Ein Validator wird an die Bytes **seiner tatsächlichen Representation-Epoche** gebunden. Ein später Proof auf einem älteren In-Memory-Zustand ist kein terminaler Beweis des inzwischen materialisierten Artefakts.

**Vor dem Patch prüfen**

Welche produktive Funktion verändert die betreffenden Bytes tatsächlich zuletzt? Welche konkrete Representation-Epoche soll dieser Proof beweisen, und konsumiert er genau deren persistierte Bytes?

**Harter Bezug**

Terminale Proofs müssen nach dem letzten Write liegen und an die dort tatsächlich persistierte Representation gebunden sein.

---

# E12 – Persistierte Bytes sind die finale Wahrheit

**Aussage**

Ein erfolgreicher In-Memory-Zustand genügt nicht, wenn anschließend andere Bytes persistiert werden.

**Warum**

Renderer-, Serialisierungs- oder Kanonisierungsstufen können semantisch relevante Unterschiede erzeugen.

**Typische Fehlform**

Eine Prüfung läuft auf einem korrekten Objekt vor der finalen Materialisierung.

**Kanonischer Lösungsraum**

Final persistieren → read-only erneut einlesen → prüfen.

Dabei Zustand und Identitaet nicht vermischen: Readback darf beweisen, dass die **Post-Transformation-Repraesentation** exakt materialisiert wurde. Bereits kausal gebundene Occurrence-/Owner-Identitaet kommt aus der transportierten Provenienz. Aendert ein legitimer Clef-, Chord- oder anderer Normalisierungsschritt nur die Repraesentation, wird der Zustands-Seal fuer diese Epoche erneuert, nicht die Identity neu gesucht.

Ebenso gilt: **Proof-Gültigkeit folgt tatsächlichen Dependencies, nicht jeder Representation-Grenze.** Eine autorisierte lokale Transformation invalidiert nur die Beweise, deren effektive Voraussetzungen sie verändert. Unveraenderte, bereits geschlossene Wahrheit wird erhalten; geänderte Dependencies werden gezielt neu bewiesen. Unabhängige Assurance und echte Persistenz-/Trust-Grenzen bleiben davon unberührt.

**Vor dem Patch prüfen**

Prüfen wir wirklich dieselben Bytes, die freigegeben werden?

**Harter Bezug**

Finaler Snapshot und terminaler Readback müssen gekoppelt sein.

---

# E13 – Reader nicht an gewünschte Wahrheit anpassen

**Aussage**

Wenn ein Readback den falschen Zustand meldet, wird nicht zuerst der Reader passend gemacht.

**Warum**

Der Reader könnte korrekt einen realen Persistenzfehler zeigen.

**Typische Fehlform**

„Der Reader sieht den falschen Snapshot; verschieben wir ihn oder ändern seine Interpretation.“

**Kanonischer Lösungsraum**

Zuerst tatsächlichen letzten Write und Persistenzpfad bestimmen.

**Vor dem Patch prüfen**

Ist der Reader falsch oder zeigt er eine unbequeme, aber reale Wahrheit?

**Harter Bezug**

Reader bleiben unabhängige Beobachter.

---

# E14 – Keine neue Repair-Schicht

**Aussage**

Neue Fehler werden nicht durch eine neue späte Reparaturschicht geschlossen, wenn ein bestehender fachlicher Owner die Ursache verhindern kann.

**Warum**

Repair-Schichten erzeugen konkurrierende Wahrheiten, Feedbackschleifen und schwer beweisbare Reihenfolgen.

**Typische Fehlform**

„Nach der eigentlichen Pipeline korrigieren wir nur noch diesen Sonderfall.“

**Kanonischer Lösungsraum**

Invariante beim bestehenden Owner oder innerhalb der bestehenden Authority durchsetzen.

**Vor dem Patch prüfen**

Verhindert der Patch einen falschen Write oder repariert er ihn erst danach?

**Harter Bezug**

No-New-Repair-Authority.

---

# E15 – Keine etüdenspezifische produktive Logik

**Aussage**

Produktive Entscheidungen werden durch musikalische oder strukturelle Eigenschaften bestimmt, niemals durch Etüdenidentität oder konkrete Taktnummern. Eine Zieladresse bleibt auch dann unzulässig, wenn sie nicht als Name oder Nummer, sondern als nachträglich zugeschnittene Kombination zufälliger Kontextmerkmale kodiert wird.

**Warum**

Einzelfallwissen maskiert fehlende allgemeine Invarianten. Proxy-Identität ist besonders gefährlich, weil der Code äußerlich generisch und korpusweit aussehen kann, während seine Merkmale tatsächlich rückwärts aus dem bekannten Ziel konstruiert wurden.

**Typische Fehlform**

„Für diesen problematischen Takt brauchen wir nur …“ oder: „Wenn wir noch die vorangehende Pause / exakt drei Gruppen / diese Randlage hinzunehmen, bleibt nur der gewünschte Fall übrig.“

**Kanonischer Lösungsraum**

Die allgemeine Strukturklasse **vor** der Korpusverteilung identifizieren, als kausale Invariante formulieren und ohne neuen Selector-Code in einem akzeptierten Parent einfrieren. Erst danach alle Treffer gleich behandeln und die beobachtete Verteilung als Erkenntnis akzeptieren, auch wenn der bekannte Witness nicht einzigartig ist. Eine Reparatur darf später enger sein als die Fehlerklasse, aber nur aufgrund unabhängig belegter Authority-/Safety-Grenzen; siehe E09.

Zusaetzlich gilt Kandidaten-Multiplicity unabhaengig von der Korpusidentitaet: Eine allgemeine lokale Regel muss bei einem zweiten unabhaengigen Vorkommen **beide** Vorkommen nach derselben lokalen Authority beurteilen. Ein globaler Cap, eine scoreweite Einzigkeitsbedingung oder eine Rangfolge ueber mehrere unabhaengige Takte ist keine Verallgemeinerung, sondern kann einen bekannten Einzelfall als strukturellen Fingerabdruck konservieren.

**Vor dem Patch prüfen**

Würde dieselbe Klassendefinition unverändert gelten, wenn der bekannte Ziel-Witness vollständig aus dem Korpus entfernt würde? Wurde ein Merkmal hinzugefügt, weil es kausal nötig ist – oder weil es einen unerwünschten Gegenfall eliminiert?

**Harter Bezug**

Keine produktiven Etüden-/Taktselektoren und keine semantisch äquivalente Proxy-Identität durch target-conditioned Merkmalskonjunktion.

---

# E16 – Kleine zentrale Änderung vor großer lokaler Reparatur

**Aussage**

Je zentraler die korrekte Authority liegt, desto kleiner darf und sollte ein fachlicher Patch häufig sein.

**Warum**

Viele lokale Änderungen deuten oft darauf hin, dass die falsche Ebene verändert wird.

**Typische Fehlform**

Mehrere Renderer-, Pipeline- und Auditmodule werden gleichzeitig angepasst, um einen einzelnen Strukturfehler zu schließen.

**Kanonischer Lösungsraum**

Die eine vorhandene Autorisierung finden, an der der unzulässige Edit verhindert werden kann.

**Vor dem Patch prüfen**

Kann dieselbe Invariante mit weniger neuen Call-Kanten innerhalb einer bestehenden Authority ausgedrückt werden?

**Harter Bezug**

Minimale Capability- und Reachability-Differenz bevorzugen.

---

# E17 – Finaler Diff schlägt ursprünglichen Plan

**Aussage**

Die Take-Deklaration beschreibt den tatsächlich fertigen Source-Baum, nicht die ursprüngliche Absicht.

**Warum**

Während der Arbeit können Dateien hinzukommen, wegfallen oder unverändert bleiben.

**Typische Fehlform**

`undeclared` oder `declared_but_unchanged`, weil das Manifest aus dem ersten Plan stammt.

**Kanonischer Lösungsraum**

Patch fertigstellen → Parent gegen finalen Candidate diffen → Deklaration daraus erzeugen oder exakt dagegen prüfen. Der Vergleichs-Parent ist dabei der vom aktuellen Repository-/Take-Zustand belegte **PhysicalStart**, nicht eine erinnerte Langzeitbasis oder nur die Revision eines aelteren Exports. Nach einem fail-closed Rollback ist der rejected Candidate keine neue Parent-Authority.

**Vor dem Patch prüfen**

Keine. Diese Prüfung gehört bewusst ans Ende des Patches.

**Harter Bezug**

`undeclared = 0`, `declared_but_unchanged = 0`.

---

# E18 – Candidate nach Beweis einfrieren

**Aussage**

Der Source-Baum, der bewiesen wurde, muss derselbe Source-Baum sein, der verpackt wird.

**Warum**

Nachträgliche Änderungen können Gate-Ergebnis, Manifest und ZIP auseinanderlaufen lassen.

**Typische Fehlform**

Nach Clippy oder Diff-Prüfung wird „nur noch eine Kleinigkeit“ verändert.

**Kanonischer Lösungsraum**

Finaler Candidate-Fingerprint → Prüfungen → erneuter Fingerprint → ZIP exakt aus diesem Baum.

**Vor dem Patch prüfen**

Nicht relevant; dies ist eine Pre-Package-Invariante.

**Harter Bezug**

Archiv und bewiesener Candidate müssen byteidentisch zusammengehören.

---

# E19 – Gate-Fehler nicht einzeln wegoptimieren

**Aussage**

Ein Gatefehler wird als Architekturinformation interpretiert, nicht als nächstes Hindernis auf dem Weg zum gleichen Patch.

**Warum**

Das bloße Entfernen des sichtbaren Symptoms kann dieselbe verbotene Capability in anderer Form wiederherstellen.

**Typische Fehlform**

Nach `new_writer_caller_outside_structural_write_authority` wird nur der sichtbare Caller umbenannt, verschoben oder indirekter gemacht.

**Kanonischer Lösungsraum**

Zurück zu Ursache → Owner → Capability → Call-Graph → Authority. Ein roter Lauf wird zuerst als Gegenbeweis verstanden. Zusätzliche Berechtigungen, gelockerte Prüfungen oder neue Capability sind nur zulässig, wenn sie fachlich unabhängig vom roten Symptom begründet und durch den zuständigen Contract autorisiert sind; sie dürfen nicht lediglich denselben Candidate grün machen.

**Vor dem Patch prüfen**

Hat ein vorheriger Kandidat bereits gezeigt, dass der gewählte Lösungsraum grundsätzlich falsch ist?

**Harter Bezug**

Semantic-Capability- und No-New-Repair-Gates sind Architekturfeedback.

---

# E20 – Gate nicht umgehen, sondern vorwegnehmen

**Aussage**

Bekannte Gate-Regeln sollen bereits vor dem ZIP bewiesen werden.

**Warum**

Ein Gate, das erst beim Nutzer den nächsten bekannten Architekturfehler entdeckt, schützt zwar die Integrität, arbeitet aber ineffizient.

**Typische Fehlform**

Patch → ZIP → `--take` → bekannte Writer-/Identity-/Manifest-Verletzung.

**Kanonischer Lösungsraum**

Derselbe reale Contract-Code beziehungsweise dieselbe kanonische Prüflogik wird bereits im Candidate-Preflight ausgeführt. Regressionstests pruefen neue Decision-Grenzen bevorzugt ohne einen produktiven Writer-Caller zu eroeffnen. Take-Manifestzeilen werden aus den finalen Kandidatenbytes gegen den autoritativen Parent erzeugt und vor Packaging rueckgeprueft; Parent-Deklarationen werden nicht fortgeschrieben. Auch der Release-Endzustand gehoert zum Preflight: Fuer einen mutierenden Zwischenstand muss vorab begruendet sein, dass seine final persistierten Artefakte den formalen Ergebnisvertrag bereits mit 0 Blockern und 0 Warnungen erfuellen koennen.

Ein vollstaendiger Source-Snapshot und ein eingehender Take-Baum sind getrennte Artefakte: Der Snapshot darf z. B. repository-vollstaendige Dateien enthalten, die laut aktuellem Managed-Tree-Vertrag im Take verboten sind. Vor Packaging wird deshalb immer der tatsaechliche incoming Tree materialisiert und separat geprueft. Bei scheinbar toten Rollout-Konstanten muessen ausserdem **alle** Konsumenten inventarisiert werden: Ein permanenter Wert kann zugleich maschinengepruefter Single-Owner-/Contract-Witness sein. Dann wird nur seine Steuerrolle pensioniert, nicht der weiterhin gebundene Besitznachweis.

**Vor dem Patch prüfen**

Welche harten Gates sind für diese Capability-Klasse relevant?

**Harter Bezug**

Preflight darf Gate-Wahrheit nicht duplizieren oder abschwächen.

---

# E21 – Negative Erfahrung braucht einen positiven Nachbarn

**Aussage**

Zu jeder fundamentalen verbotenen Architekturform soll ein benachbarter zulässiger Lösungsraum bekannt sein.

**Warum**

Ein System aus ausschließlich Verboten wird immer restriktiver, ohne Entwicklung zu führen.

**Typische Fehlform**

„Kein neuer Caller“ wird als allgemeine Regel verstanden, obwohl neue Caller innerhalb der korrekten Authority zulässig sein können.

**Kanonischer Lösungsraum**

BAD Pattern und CANONICAL Pattern gemeinsam dokumentieren und testen.

**Vor dem Patch prüfen**

Wissen wir nicht nur, was verboten ist, sondern auch, wie dieselbe fachliche Absicht zulässig umgesetzt werden kann?

**Harter Bezug**

Negative und positive Architekturregressionen.

---

# E22 – Historie abstrahieren, nicht konservieren

**Aussage**

Ein historischer Fehlschlag wird nur dann Teil der Essenz, wenn daraus eine allgemeine Regel ableitbar ist.

**Warum**

Eine vollständige Take-Chronik würde die Essenz unlesbar machen und Einzelfallwissen mit Architekturwissen vermischen.

**Typische Fehlform**

„r493 hat X gezeigt“ wird unmittelbar als globale Regel gespeichert.

**Kanonischer Lösungsraum**

Historischer Fall → allgemeines Muster → minimale Essenz-Regel.

Rejection-Provenienz (z. B. Stage, Grund, Zeitpunkt oder `TimeToRejectMs`) ist dabei Diagnose-/Lernevidenz: Sie darf Muster sichtbar machen, aber niemals Parent-, Ranking-, Selector-, Writer-, Repair- oder Release-Authority erteilen.

**Vor dem Patch prüfen**

Gilt die Erkenntnis auch für einen unbekannten zukünftigen Fall?

**Harter Bezug**

Essenz bleibt klein und generalisierbar. Aktuell kodexgebundene Governance-Witnesses sind keine zu konservierende Fallchronik, sondern Bestandteil des heutigen Contracts bis zu einer expliziten Migration.

---

# E23 – Fallwissen bleibt Fallwissen

**Aussage**

Ein realer Gegenbeweis darf dauerhaft relevant sein, ohne eine globale Architekturregel zu werden.

**Warum**

Nicht jede wichtige Erkenntnis ist generalisierbar.

**Beispielklasse**

Ein bestimmter Filter wurde als Ursache eines konkreten Fehlers ausgeschlossen.

**Kanonischer Lösungsraum**

Solches Wissen gehört in Diagnose-/Fallhistorie oder Evaluation, nicht automatisch in den globalen Architecture Contract und nicht als konkrete dauerhafte Regression in die normative Testsuite. Nur eine daraus abstrahierte allgemeine Invariante darf als synthetischer Test fortbestehen.

**Vor dem Patch prüfen**

Ist dies eine universelle Invariante oder nur eine widerlegte Hypothese dieses Falls?

**Harter Bezug**

Keine Aufblähung globaler Contracts durch lokale Historie.

---

# E24 – Korpus beweist Breite, nicht Ursache

**Aussage**

Die Beweisbreite folgt der tatsächlich invalidierten Semantik. Ein grüner Vollkorpus ist ein starker Breitenbeweis, wenn eine Änderung corpusweite Generalisierung, Auswahl, Musik-/Layoutverhalten oder eine explizit corpusgebundene Evidenzdimension berührt; er ist aber **kein pauschaler Pflichtschritt jedes Takes**.

**Warum**

Ein Patch kann zufällig alle vorhandenen Stücke bestehen und dennoch einen falschen Owner oder eine neue unzulässige Capability besitzen. Umgekehrt erzeugt ein rein lokaler Kodex-/Handover-/Test-Fixture-Cut ohne produktive Musik- oder Auswahlwirkung durch einen reflexhaften Vollkorpus keine zusätzliche kausale Aussage, kostet aber erheblich Zeit und kann den eigentlichen Beweis verdecken.

**Typische Fehlform**

„10/10 grün, also ist die Architektur korrekt.“

Ebenso falsch ist:

„Jeder Take braucht unabhängig vom Änderungsumfang zwingend einen Vollkorpus.“

**Kanonischer Lösungsraum**

Die Proof-Reihenfolge ist:

1. Architektur-/Change-Control-Gates passend zur Änderungsklasse,
2. kleinster gezielter synthetischer Invariantennachweis, welcher die betroffene Invariante direkt und corpusunabhaengig beweist,
3. breitere Tests nur soweit die Änderung deren Evidenzgrundlage tatsächlich invalidiert,
4. vollständiger Korpus genau dann, wenn corpusweite Generalisierung oder Verteilung selbst Teil des Beweises ist.

Ein Vollkorpus ist insbesondere erforderlich oder fachlich geboten, wenn produktive Musik-, Writer-, Repair-, Selection-, Layout- oder corpusweite Entscheidungssemantik über den gezielten Witness hinaus verändert werden kann; wenn eine neue oder geänderte Fehlerklasse, Multiplicity-Invarianz oder Korpusverteilung gemessen werden muss; wenn ein aktueller expliziter Contract den Vollkorpus als Acceptance-/Breitengate fordert; oder wenn gezielte Evidenz unerwartete breitere Auswirkungen zeigt.

Ein Vollkorpus ist **nicht allein deshalb erforderlich**, weil ein Take existiert. Rein kodexseitige Control-Plane-, Handover- oder Dokumentationsänderungen dürfen nach grünem Architektur-Gate, Strict-Clippy/Build, vollständigen All-Target-Tests und dem gezielten synthetischen Invariantennachweis schließen, sofern keine produktive oder corpusweite Evidenzdimension invalidiert wurde. Test-Fixtures besitzen dabei keine Sonderstellung: normative Fixtures muessen synthetisch sein und duerfen keine konkrete Corpus-Herkunft konservieren.

Für die primäre Ursachensuche wird zuerst der kleinste reproduzierbare Fall verwendet. Bei neuer Fehlerklassifikation gilt zusätzlich: **Klasse zuerst kausal definieren, dann Korpusverteilung messen.** Die Verteilung darf eine Hypothese widerlegen, aber nicht iterativ als Trainingssignal benutzt werden, bis nur der bekannte Witness übrig bleibt.

Bei produktiven lokalen Kandidaten prüft der Korpus außerdem **Multiplicity-Invarianz**, sobald diese Breite durch die Änderung betroffen ist: Das Hinzufügen oder Vorhandensein weiterer unabhängiger gleichartiger Kandidaten darf die lokale Entscheidung eines bereits zulässigen Kandidaten nicht verändern. Ein Korpus mit mehreren Treffern ist damit kein Grund, die Regel scoreweit auf genau einen Gewinner zu reduzieren; vielmehr ist zu beweisen, ob die Treffer unabhängig sind oder zu derselben Konflikt-/Transaktionsgruppe gehören.

**Vor dem Patch prüfen**

Welche konkrete Evidenzdimension wird durch diesen Diff invalidiert? Ist die Architektur bereits bewiesen, bevor wir ihre Breitenwirkung messen? Bei Selektorarbeit zusätzlich: Messen wir gerade die Verteilung einer vorher definierten Klasse – oder optimieren wir die Klassendefinition rückwärts auf ein gewünschtes Korpusergebnis?

**Harter Bezug**

Korpus ist Evaluation-/Breitenevidenz, nicht Schreibautorität und nicht Spezifikation. Seine Pflicht entsteht aus der invalidierten Breite, nicht aus der bloßen Existenz eines Takes.

---

# E25 – Strukturelle Änderung ohne Klangänderung bevorzugen, wenn fachlich passend

**Aussage**

Wenn die Ursache rein strukturell ist, soll die Lösung keine vorhandene klingende Musik verschieben, verlängern, verkürzen, umstimmen, löschen oder neu erzeugen. Bei einem Konflikt zwischen bereits vorhandener Musik und neu erzeugter Darstellung besitzt die vorhandene Musik Vorrang.

**Warum**

Notation, Serialisierung und strukturelle Carrier können korrigiert werden, ohne musikalischen Inhalt neu zu erfinden. Eine Darstellungsbereinigung erhält dadurch keine implizite SoundingEventMutation-Authority.

**Typische Fehlform**

Ein fehlender struktureller Host wird durch Erfinden eines neuen klingenden Onsets ersetzt oder eine neu eingefügte Pause verschiebt eine bereits vorhandene gepitchte Note auf einen späteren Zeitpunkt.

**Kanonischer Lösungsraum**

Bestehende zulässige Musik erhalten beziehungsweise die kollidierende Darstellungsänderung verwerfen. Erst eine eigenständig begründete und autorisierte MusicMutation darf klingende Semantik verändern.

**Vor dem Patch prüfen**

Ist `SoundingEventMutation` fachlich überhaupt notwendig?

**Harter Bezug**

Keine unnötige Ausweitung der MusicMutation-Capability.

---

# E26 – Structural Carrier sind semantisch relevant

**Aussage**

Ein Rest-, Zeit- oder anderer serieller Carrier kann strukturelle Bedeutung besitzen, auch wenn er selbst nicht klingt.

**Warum**

Nachfolgende Chord-Ereignisse oder Voice-Strukturen können von seiner seriellen Host-Funktion abhängen.

**Typische Fehlform**

Ein sichtbarer Rest wird als „leer“ klassifiziert und zu `<forward>` reduziert, obwohl nachfolgende Chord-Ereignisse ihn als Host benötigen.

**Kanonischer Lösungsraum**

Vor einer Suppression oder Forward-Materialisierung strukturelle Abhängigkeiten prüfen; unzulässigen Edit innerhalb der zuständigen StructuralWriteAuthority verweigern.

**Vor dem Patch prüfen**

Welche nachfolgenden Events verlieren durch diese Entfernung ihren seriellen Host?

**Harter Bezug**

StructuralWriteAuthority autorisiert nur strukturell zulässige Edits.

**Persistenter Scope und sichtbarer Cue**

Ein fortbestehender semantischer Zustand und seine sichtbare Wiederbestaetigung sind nicht dieselbe Sache. Ein struktureller Carrier wie `octave-shift type="continue"` kann den Scope korrekt fortsetzen, ohne dass der konkrete Renderer inner-systemisch einen neuen sichtbaren Cue erzeugt. Umgekehrt kann ein finaler System-/Seitenwiedereintritt bereits einen Renderer-Cue besitzen, sodass ein zusaetzlicher Text-Cue redundant waere.

Der kanonische Loesungsraum lautet deshalb: Scope-Entscheidung beim fachlichen Owner erhalten, sichtbaren Carrier erst aus der dafuer autoritativen finalen Layoutwahrheit projizieren und den tatsaechlichen Renderer-/Persisted-Byte-Readback pruefen. `FORMAT_CAPABILITY` oder ein frueher Zwischenzustand allein beweisen keine sichtbare Wirkung.

---

# E27 – Architekturgrenzen vor Implementierungsdetails

**Aussage**

Bei einem neuen Fehler werden zuerst Authority-, Capability- und Call-Graph-Grenzen bestimmt und erst danach Codezeilen ausgewählt.

**Warum**

Eine frühe Fixierung auf konkrete Funktionen begünstigt lokal bequeme, aber architektonisch falsche Patches.

**Typische Fehlform**

„Diese Funktion ist nah am Fehler, also ändere ich sie.“

**Kanonischer Lösungsraum**

Problemklasse → Owner → Capability → Root → Call-Site → erst dann Implementierung.

**Vor dem Patch prüfen**

Kann der geplante Eingriff bereits ohne Kenntnis konkreter Zeilennummern architektonisch begründet werden?

**Harter Bezug**

Take-Vorprüfung beginnt mit Ursache und Authority, nicht mit Diff.

---

# E28 – Keine zweite Wahrheit für die Essenz

**Aussage**

Die Essenz darf keine eigene Kopie der harten Gate-Regeln entwickeln.

**Warum**

Zwei normative Regelwerke würden auseinanderlaufen.

**Typische Fehlform**

Ein handgepflegtes Essenz-Regelwerk sagt „zulässig“, während der eigentliche Contract dieselbe Struktur ablehnt.

**Kanonischer Lösungsraum**

Die Essenz referenziert beziehungsweise projiziert kanonische Contract-IDs und ergänzt nur Rationale, Fehlmuster, Vorabfragen und bevorzugte Lösungsräume.

**Vor dem Patch prüfen**

Ist diese Aussage Beratung oder versucht sie selbst normative Autorität zu besitzen?

**Harter Bezug**

Contracts/Gates bleiben einzige normative Quelle.

---

# E29 – Essenz muss handlungswirksam sein

**Aussage**

Eine Essenz-Regel ist nur dann wertvoll, wenn sie eine Entscheidung vor dem Patch beeinflussen kann.

**Warum**

Reine Architekturprosa verhindert keine Fehlversuche.

**Typische Fehlform**

Eine lange Dokumentation erklärt vergangene Probleme, liefert aber keine konkrete Vorabfrage.

**Kanonischer Lösungsraum**

Jede Essenz-Regel enthält mindestens eine erkennbare Fehlform und eine konkrete Pre-Patch-Frage.

**Vor dem Patch prüfen**

Welche Entscheidung würde Maus aufgrund dieser Regel anders treffen?

**Harter Bezug**

Essenz dient Effizienz, nicht Dokumentationsumfang.

---

# E30 – Cold-Start ist eine Projektanforderung

**Aussage**

Ein qualifizierter Bearbeiter ohne historische Projekterinnerung muss aus Repository und generierter Essenz einen architektonisch plausiblen Lösungsraum bestimmen können.

**Warum**

Externe Erinnerung kann verloren gehen.

**Typische Fehlform**

Fundamentale Projektgrenzen sind nur langjährigen Beteiligten bekannt.

**Kanonischer Lösungsraum**

Ein automatisch erzeugbarer Architecture-Essence-Bootstrap enthält den aktuellen autoritativen Stand, Authorities, relevante Capability-Grenzen, bekannte Hochrisikomuster und obligatorische Vorabprüfungen.

**Vor dem Patch prüfen**

Würde ein vollständig neuer Bearbeiter die entscheidende alte Falle sehen, bevor er sie implementiert?

**Harter Bezug**

Cold-Start darf die Gates nicht ersetzen; er muss zu ihnen hinführen.

---

# E31 – Billige fail-closed Strukturchecks vor teuren Beweisen

**Aussage**

ChangeClass, PathRole und ChangeKind müssen bereits **vor dem Candidate-Bau gemeinsam** gegen den final geplanten Diff geprüft werden. Erst wenn diese billige strukturelle Matrix konsistent ist, dürfen Candidate-/ZIP-Baum sowie teure Semantic-/Capability-, Call-Graph-, Build- oder Korpusbeweise folgen.

Die fachliche Capability einer Änderung und die syntaktisch vom Take-Gate ermittelte Pfadrolle sind getrennte Dimensionen. Ein fachlich read-only Audit ist nicht automatisch `PathRole::Audit`, und ein syntaktisch als `MusicSensitive` erkannter Pfad beweist umgekehrt noch keine MusicMutation.

**Warum**

Ein strukturell unzulässiger Take kann unabhängig von der fachlichen Qualität des Patches niemals installiert werden. r526 zeigte zusätzlich, dass selbst ein expliziter `src/audit_only/`-Owner durch eine abweichende PathRole-Priorität als `MusicSensitive` klassifiziert werden kann. Wird dieser Widerspruch erst im Nutzerlauf sichtbar, wurde teure Beweisarbeit auf einem Candidate geleistet, dessen ChangeClass-/Pfadvertrag bereits vorher deterministisch widersprüchlich war.

**Typische Fehlform**

Ein fachlich sorgfältig geprüfter `AuditOnly`-Take wird erst beim realen Nutzerlauf mit einer Meldung wie

```text
Take-Change-Control: Klasse AuditOnly darf Pfad src/audit_only/...notation....rs (MusicSensitive) nicht aendern
```

abgelehnt. Anschließend wird nur die ChangeClass auf `MusicMutation` hochgestuft oder das Wort `notation` aus dem Dateinamen entfernt, obwohl weder die fachliche Capability noch der zuständige Owner geklärt wurden. Analog darf ein unbekannter Top-Level-Pfad nicht durch kosmetisches Umbenennen oder Lockern des Tree-Vertrags in den Take gezwungen werden.

**Kanonischer Lösungsraum**

Vor dem Candidate-Packaging den final geplanten Diff bestimmen und **für jeden Pfad gemeinsam** prüfen:

```text
ChangeClass × PathRole × ChangeKind
```

Bei einem Widerspruch nicht das Gate passend machen, sondern zuerst klären:

1. Liegt die Capability tatsächlich beim falschen fachlichen Owner?
2. Oder bildet die kanonische Pfadklassifikation den expliziten Owner falsch beziehungsweise inkonsistent zu anderen Gates ab?

Erst wenn diese Frage geklärt ist, Candidate-/ZIP-Baum materialisieren, Top-Level-Rollen prüfen, tatsächlichen Diff gegen Take-Deklaration schließen und danach Semantic-/Capability-Diff, produktiven Call-Graph, Writer-Reachability, reale Einbindung sowie Build/Strict-Clippy ausführen. Ein Pfad wird niemals nur deshalb erlaubt, um einen Candidate durch das Gate zu bringen. Ebenso werden weder eine ChangeClass nur zum Gate-Passieren hochgestuft noch Pfad oder Dateiname kosmetisch umbenannt.

**Vor dem Patch prüfen**

Kann jeder geplante geänderte Pfad bereits vor Packaging eindeutig als konsistentes Tupel aus `ChangeClass`, kanonischer `PathRole` und `ChangeKind` beschrieben werden? Wenn nein: Ist der Owner falsch oder die Pfadklassifikation? Der geplante incoming Take-Baum muss außerdem weiterhin mit `unknown_top_level_paths = 0` und ohne laut aktuellem Vertrag nicht eingehend verwaltete Root-Dateien darstellbar sein.

Fuer `AuditOnly` zusaetzlich **den Datenzugriff als Teil des geplanten Diffs behandeln**: Neue Audit-Logik muss vollstaendig an einem kanonischen `PathRole::Audit` leben. Braucht sie private State aus einer gemischten produktiven Source, darf AuditOnly dort keinen neuen Helper, keine Call-Site, kein Feld und keinen Report-/Daten-Bridge einfuehren. Entweder existiert bereits ein autorisierter audit-sichtbarer Transport, oder die fehlende Datenkante ist eine eigene Architekturgrenze und verlangt eine explizit passende ChangeClass/Authority. Read-only Absicht allein macht eine neue produktive Datenkante nicht zu AuditOnly.

Bei **Struct-Erweiterungen** gehoert die vollstaendige Initializer-Closure ebenfalls in den billigen Preflight: Vor Packaging alle expliziten `StructName { ... }`-Literale in `src/` und `tests/` inventarisieren und jedes neu hinzugefuegte Pflichtfeld dort schliessen. Ein `Default`-Derive oder ein `impl Default` fuer den Struct beziehungsweise fuer das neue Feld fuellt explizite Rust-Struct-Literale nicht automatisch; ein vergessenes Feld wird sonst erst im teuren Strict-Clippy/Build als `E0063 missing field` sichtbar.

Bei **MusicMutation** gehoert die abgeleitete globale Writer-Root-Menge in denselben billigen Preflight. Ein fachlich richtiger StructuralWriteAuthority-Owner reicht nicht, wenn unter einem unveraenderten Audit-/Report-Caller zusaetzliche Writer-Roots erreichbar werden. Deshalb Parent und Candidate vor Packaging als Mengenvergleich pruefen und im Normalfall `after_roots == before_roots` verlangen; nur eine explizit kodifizierte strenge Verengung ist eine Ausnahme.

Zusaetzlich muss fuer **jede neue oder semantisch geaenderte produktive Funktion ausserhalb `src/structural_write_authority/`** die transitive WriterReachability leer bleiben. Die Wiederverwendung einer bereits im Parent vorhandenen Writer-Root macht einen neuen Pipeline-Caller nicht zulaessig. Writer duerfen nicht als Rechen-/Projektionsutility missbraucht werden; reine Kandidatenprojektion bleibt writerfrei und erst der bereits vorhandene autorisierte Sink schreibt.

Bei **neuen oder semantisch geaenderten produktiven Music-Funktionen** gehoert ausserdem der FeedbackIdentityChannel-Diff in den billigen Preflight. Primitive rohe Identitaetsmarker wie `measure_number`, `source_measure`, `origin_measure`, Projekt-/Feedback-Locators oder vergleichbare externe Identity-Eingaenge duerfen nicht nebenbei durch eine MusicMutation neu entstehen. Wenn eine Projektion nur eine bereits gebundene interne Zeitordnung serialisiert, soll sie diese State/Ordnung konsumieren und XML positional durchlaufen, statt aus externem Markup eine zweite Identitaetswahrheit zu erzeugen. Ist echte externe Identity fachlich notwendig, muss die dafuer zustaendige Authority explizit geklaert werden; kosmetisches Umbenennen ist keine zulaessige Gate-Strategie.

Bei **expliziten Complete-/Partial-Modi** muessen zwei Achsen getrennt werden: Vollstaendigkeit der kanonischen/Frozen-Domaene und vollstaendige Erklaerung der tatsaechlich vorhandenen Source-Carrier. Der Complete-/Master-Pfad beweist beide vollstaendig. Ein Render-Subset darf weniger Frozen-Attacks enthalten beziehungsweise binden, aber jeder im Render tatsaechlich vorhandene pitched Carrier muss weiterhin eindeutig genau einem kanonischen Attack gehoeren. `bound_frozen < active_frozen` kann damit legitim sein; `owned_source < existing_source` bleibt fail-closed. Partial-Modus ist keine Erlaubnis fuer unerklärtes klingendes Legacy-Material. Parse-, Cursor- und semantische Endinvarianten bleiben ebenfalls bindend.

Bei **Carrier-Binding an einer Re-Materialisierungsgrenze** duerfen Felder, die derselbe Cut selbst neu autoritativ schreibt, nicht zyklisch als notwendige Locator-Identitaet dienen, sofern ihre Erhaltung nicht vorher bewiesen ist. Opaque kanonische Identitaet bleibt bis zur Binding-Grenze erhalten; physische XML-Carrier werden nur ueber erhaltene, runtime-eindeutige Koordinaten lokalisiert. Source- und Canonical-Locator-Eindeutigkeit sowie exklusives Carrier-Ownership werden vor der Materialisierung bewiesen. Provenienz darf in einem Freeze/Fingerprint mitversiegelt sein, ohne dadurch zur produktiven Identitaetsautoritaet zu werden.

**Harter Bezug**

Die aktuelle PathRole-Wahrheit liegt im kanonischen Take-Change-Control; die Capability-Wahrheit liegt in den dafür zuständigen Semantic-/Capability- und Reachability-Gates. Die Essenz ordnet nur die frühe Prüfreihenfolge und fordert die Widerspruchsanalyse. Sie darf weder eine PathRole noch eine ChangeClass eigenmächtig umdeuten und lockert keinen Gate-Vertrag.

---


# E32 – Corpus Independence ist eine Systemgrenze

**Aussage**

EtuSchlopf ist ein genereller Notenrenderer. Ein konkreter Korpus – einschliesslich der aktuell zehn Etueden – ist Anwendungs- und Evaluationsevidenz, niemals Spezifikation. Alles Normative muss fachlich sinnvoll bleiben, wenn der gesamte bekannte Korpus durch unbekannte neue Musik ersetzt wird.

**Warum**

Ein realer Korpus ist unverzichtbar, um Qualitaet sichtbar zu machen, kann aber unbemerkt zur versteckten Produktdefinition werden. Dass eine Regel, ein Test oder ein Schwellwert alle heutigen Stuecke zufriedenstellt, beweist keine Generalitaet. Besonders gefaehrlich sind gut gemeinte Regressionstests, reale Snapshots und schrittweise Suchraumverengung: Sie konservieren den bekannten Fall und erschweren spaetere legitime Verbesserungen des Musikbildes.

**Typische Fehlform**

Ein Fehler wird an einer konkreten Etuede und einem konkreten Takt gefunden. Der reale Ausschnitt wird als Fixture oder Golden Master gespeichert, ein Test bindet seine bisherige Darstellung, oder Produktlogik wird mit zusaetzlichen Kontextmerkmalen so lange verengt, bis nur dieser Witness uebrig bleibt. Die Historie wird dadurch Teil der ausfuehrbaren Spezifikation.

**Kanonischer Lösungsraum**

Die Ebenen bleiben getrennt:

1. **Core:** allgemeine Musik-/Notationssemantik ohne Corpus-Identitaet.
2. **Normative Tests:** synthetische, zeitlose Invarianten ohne reale Etueden-/Taktidentitaet oder reale Corpus-Artefakte.
3. **Identity Firewall:** direkte Identitaet, abgeleitete Proxy-Identitaet und scoreweite Singularisierung duerfen keine produktive Decision- oder Write-Authority erzeugen.
4. **Evaluation Corpus:** reale Stuecke duerfen konkret lokalisiert, gerendert, verglichen und visuell beurteilt werden.
5. **Historie:** erklaert, wie eine Erkenntnis entstand, besitzt aber keine normative Authority.

Ein realer Fehler darf Anlass fuer einen Test sein, aber seine Herkunft darf den Test nicht ueberleben. Der dauerhafte Beweis ist eine synthetische Fehlerklasse plus metamorphe Gegenprobe: Renaming, Relocation, irrelevante Perturbation und unabhaengige Duplication duerfen die lokale fachliche Entscheidung nicht aendern. Eine Regel darf im aktuellen Korpus zufaellig nur einmal vorkommen; sie darf ihre Gueltigkeit aber niemals aus dieser Einzigartigkeit beziehen.

Neue normative Testevidenz wird fail-closed zugelassen. Direkte konkrete Corpusidentitaet, absolute reale Taktlokalisierung, historische Regression als Testidentitaet und Abhaengigkeit von nichtsynthetischen Corpus-Fixtures sind kein zulaessiger Weg in die Testsuite. Kodex-interne adversariale Selbsttests duerfen verbotene Payloads ausschliesslich erzeugen, um genau diese Grenze zu pruefen; daraus entsteht keine Musik- oder Test-Spezifikationsauthority.

**Vor dem Patch prüfen**

Wenn die heutigen zehn Etueden nie existiert haetten: Wuerden diese Produktregel, dieser Test, dieses Fixture, dieser Schwellwert und diese Dokumentationsaussage weiterhin dieselbe fachliche Bedeutung besitzen? Wenn nein, gehoert der konkrete Befund in Evaluation/Historie oder muss zuerst zu einer allgemeinen Invariante abstrahiert werden.

**Harter Bezug**

Corpus Independence bindet Produktlogik, normative Tests, Essenz und Kodex gemeinsam. Reale Etueden duerfen Evaluation ausloesen, aber weder direkt noch indirekt Produktverhalten oder normative Testwahrheit definieren.

---


# Take-Identity-Lifecycle-Vertrag

**Aussage**

Eine Take-Identitaet `(Version, Revision)` darf innerhalb der aktiven Take-Historie nur genau eine kanonische Payload besitzen. Unterschiedliche Payloads unter derselben Identitaet sind kein normaler Same-Version-Folgetake, sondern ein Identitaetskonflikt.

Der letzte akzeptierte grüne Stand bleibt die normale Parent-Autorität. Ein abgelehnter Candidate darf Diagnosewissen liefern, wird dadurch aber weder akzeptiert noch automatisch zur neuen Basis.

**Warum**

Fail-Closed-Mehrdeutigkeit schuetzt korrekt vor einer falschen Fortsetzung. Sie darf aber keinen normalen, durch einen zuvor fehlgeschlagenen Kandidaten erreichbaren Zustand erzeugen, aus dem der Ein-Befehl-Workflow nur durch manuelles Suchen, Loeschen oder Umbenennen von ZIP-Dateien weiterkommt.

**Typische Fehlform**

Ein Kandidat wird transaktional installiert, scheitert erst im Reexec/Build/Strict-Clippy und wird auf den Parent zurueckgerollt. Seine SHA wird dabei nicht als rejected persistiert. Ein korrigierter Kandidat wird anschliessend mit derselben `(Version, Revision)` angeboten. Discovery sieht beide Payloads als nicht abgewiesen und blockiert dauerhaft mehrdeutig.

**Kanonischer Lösungsraum**

Candidate-SHA und Take-Provenienz muessen den vollstaendigen Lifecycle begleiten: fruehe Gate-Ablehnung, Apply-/Lock-/Reexec-Fehler und unterbrochene transaktionale Installation werden erst nach sicherem Rollback als rejected persistiert; bis zum erfolgreichen Provenienz-Commit bleibt der rolled-back Transaktionszustand als Recovery-Quelle erhalten. Ein erfolgreicher Commit persistiert die akzeptierte Payload revisionsgebunden als kanonische Provenienz. Discovery darf bei derselben Take-Identitaet exakt diese akzeptierte SHA bevorzugen und abweichende Peer-Payloads logisch als superseded behandeln; nur wenn weder Acceptance- noch Rejection-Provenienz entscheidet, bleibt echte Mehrdeutigkeit fail-closed. Dateien ausserhalb des Projekts werden dafuer nicht geloescht oder umbenannt. Neue Acceptance-/Rejection-Provenienz darf dabei vorhandene path-stabile Negative-Checks nicht ersetzen: wenn keine Provenienz entscheidet, bleibt die bereits autorisierte Mehrdeutigkeitspruefung als ausfuehrbarer Fallback bestehen.

**Vor dem Patch prüfen**

Kann ein Kandidat nach jedem moeglichen Gate-, Apply-, Reexec-, Crash- oder Rollback-Ausgang beim naechsten einzelnen `run.cmd --take ...` eindeutig als accepted, rejected/superseded oder weiterhin unklassifiziert bestimmt werden?

**Harter Bezug**

Fail-Closed und Recovery sind ein gemeinsamer Vertrag: falsche Fortsetzung bleibt blockiert, aber jeder regulaer erreichbare fehlgeschlagene Kandidatenzustand muss deterministisch in eine erneute Ein-Befehl-Fortsetzung ueberfuehrbar sein. Acceptance-/Rejection-Provenienz ist Projektgedaechtnis fuer die Take-Auswahl, keine MusicMutation- oder Writer-Authority. Lifecycle-Recovery erweitert bestehende Security-Checks, sie pensioniert oder verschiebt sie nicht. Ebenso bleibt eine bereits kodexgebundene Public-Recovery-Reihenfolge am autorisierten oeffentlichen Einstieg sichtbar: Provenienz-Helfer duerfen den gemeinsamen Rollback-Kern teilen, aber die vorgeschriebene Recovery-vor-Source-Mutation-Kante nicht aus dem gebundenen Einstieg verlagern.

**Kodex-gebundene Source-Witnesses sind Teil des Architekturvertrags.** Bei Security-, Recovery- und Take-Bootstrap-Ownern kann ein Gate nicht nur Verhalten, sondern einen konkreten Pfad, Call-Site, Funktionsnamen, Quelltextausdruck oder Literal als adversarial Witness binden. Funktionale Aequivalenz ist dann kein zulässiger Ersatz. Vor jeder Aenderung an einem solchen Owner muessen alle konsumierenden Build-/Adversarial-/Source-Integrity-Gates inventarisiert werden; path-, call-site-, order-, FNV- und literal-gebundene Witnesses gelten bis zu einer ausdruecklichen Kodex-Migration als unverrueckbare Bestandteile des Owners. Neue Lifecycle-Provenienz darf um diese Witnesses herum ergaenzen, sie aber weder in Helper extrahieren noch umbenennen, wrappen, verschieben oder durch semantisch gleichwertige Aufrufe ersetzen.

**Preflight folgt dem gesamten Gate-Konsumgraphen.** E31 prueft Paketbaum und deklarierte Diffs billig zuerst; fuer Kodex-/Bootstrap-Takes folgt vor dem Candidate-Packaging zusaetzlich eine statische Witness-Inventur aller Gates, die die geaenderten Owner lesen oder pinnen. Ein Candidate darf nicht erst im teuren Build entdecken, dass ein bereits repository-residenter adversarial Witness verschwunden ist.

---

# Proof-Preservation-und-Validation-Placement-Vertrag

**Aussage**

Ein bereits erfolgreich bewiesener Zustand bleibt über eine lokale autorisierte Transformation gültig, soweit seine tatsächlichen Proof-Dependencies unverändert bleiben. Eine Representation-Grenze allein ist kein Grund für globale Beweisinvalidation.

**Kanonischer Lösungsraum**

- Für zwei Prüfungen werden nicht nur sichtbare Argumente, sondern ihre **Effective Inputs** verglichen: explizite Parameter, Filesystemzustand, Environment, globaler/mutabler Zustand, Authority-/Contract-Epoche, Trust-State und relevante Nebenläufigkeit.
- Gilt `EffectiveInputs(V1) = EffectiveInputs(V2)` und liegt dazwischen kein relevantes Ereignis, ist eine unmittelbare zweite Vollprüfung ein Retirement-Kandidat.
- Ändert eine autorisierte Transformation nur eine begrenzte Representation-Dimension, bleibt der bereits geschlossene Typzustand für alle unveränderten Dependencies erhalten. Nur die geänderte Dimension, ihr Persistenz-Readback und notwendige Bindungen werden neu bewiesen.
- Ein Validator an einer späteren Representation-Epoche konsumiert die tatsächlich persistierten Bytes dieser Epoche; alte In-Memory-Bytes dürfen nicht als Beweis des neuen Artefakts wiederverwendet werden.
- **Independent Assurance ist keine Redundanz.** Eine zweite Prüfung bleibt bestehen, wenn sie andere Evidenz, eine andere Fehlerklasse oder eine echte neue Trust-/Side-Effect-Grenze absichert.
- Im Kodex soll exakte Whole-Byte-Identität einen klaren Owner besitzen. Sub-Gates prüfen ihre tatsächlichen semantischen Dependencies; derselbe FNV über dieselben Bytes gegen denselben Sollwert ist ohne zusätzliche Fehlerklasse Proof-Amplification und erzeugt unnötigen Pin-Owner-Fan-out.
- Proof-Retirement soll bevorzugt API-/Getter-/Wrapper-Retirement nach sich ziehen. Es darf keine neue Registry, kein Ledger, keinen globalen Dependency-Graph und keine neue universelle Continuation-Abstraktion allein zur Kompression erzeugen.

**Vor dem Patch prüfen**

Welcher neue Fakt rechtfertigt diesen Full Proof seit dem vorherigen gültigen Zustand? Welche konkrete Dependency hat sich geändert? Welche zusätzliche Fehlerklasse entdeckt eine zweite Prüfung wirklich? Ist der Proof an die richtige persistierte Representation-Epoche gebunden?

**Harter Bezug**

E04 trennt Proof von Wahrheit; E11/E12 binden Proof an letzten Write und persistierte Bytes; E17 verlangt den finalen realen Diff; E22 verhindert, dass einzelne Take-Geschichten statt der allgemeinen Dependency-Regel konserviert werden. Dieser Vertrag ist `advisory only` und erzeugt keine Proof-, Writer-, MusicMutation-, Repair-, Release- oder Gate-Authority.

---

# Completion-Boundary-Vertrag

**Aussage**

EtuSchlopf trennt auf dem Weg zum Abschluss vier Beweisgrenzen:

```text
Plan
↓
Materialisierung
↓
persistierte Artefakte
↓
Release
```

Ein grüner Zustand einer früheren Grenze erteilt der nächsten Grenze keine neue Authority. Ein korrekter Plan beweist insbesondere weder Writer-/Materialisierungsautorität noch korrekte persistierte Bytes; `Programm/PDF OK` beweist umgekehrt noch nicht den öffentlichen Release.

**Warum**

Ein Arbeitsstrang kann seine ursprüngliche Fehlerklasse bereits geschlossen haben, während nur noch eine andere fachliche Restklasse oder der finale Artefakt-Commit blockiert. Wird trotzdem in der bereits grünen Schicht weitergearbeitet oder eine früher geplante Capability allein der Vollständigkeit halber implementiert, entstehen unnötige Writer-, Repair-, Layout- oder Policy-Flächen.

**Kanonischer Lösungsraum**

- Jede neue Materialisierung wird unabhängig vom vorgelagerten Plan gegen ihre eigene Authority, den realen letzten Write und die final persistierten Bytes bewiesen.
- Ein `MusicMutation`-Zwischenstand muss selbst releasefaehig sein. Wenn Schritt A absichtlich oder vorhersehbar einen Blocker/eine Warnung freilegt, die erst Schritt B schliessen soll, waehrend der Public-Artifact-Vertrag 0 Blocker und 0 Warnungen verlangt, duerfen A und B nicht als zwei unabhaengige releasbare Mutationen geplant werden. A bleibt dann read-only/audit evidence oder die kleinste gekoppelte Invariantengruppe wird in einem atomaren MusicMutation-Cut geschlossen. `Programm/PDF OK` allein ist kein Beweis fuer diese Release-Atomizitaet.
- Kann ein bestehender Producer eine bereits geschlossene Invariante wieder verletzen, gehört diese Invariante in seine Candidate-Zulassung. Bevorzugt wird ein unzulässiger Candidate verworfen; kein nachgelagerter Cleaner und keine neue Repair-Schicht.
- Externes Renderer-Verhalten darf reale Feasibility-Gegenbeweise beziehungsweise No-Goods liefern. Es erzeugt keine musikalische Authority und rechtfertigt kein zweites Wahrheitsmodell, wenn der bestehende Owner die belegte Unzulässigkeit konsumieren kann.
- Sobald eine Fehlerklasse geschlossen ist, bleibt ihre Schicht geschlossen, bis frische Gegen-Evidence sie erneut öffnet. Der tatsächlich verbleibende Fehler bestimmt den nächsten fachlichen Owner.
- Der Abschluss optimiert auf den kleinsten nachweisbar korrekten Zustand, der das musikalische Ziel und den aktuellen Release-Vertrag vollständig erfüllt. Eine früher geplante oder in verworfenen Kandidaten erprobte Capability wird nicht allein deshalb implementiert, weil sie einmal vorgesehen war.
- Sind musikalisches Ziel, persistierte Artefakte, relevante Gates und Release-Vertrag erfüllt, wird der Arbeitsstrang ausdrücklich geschlossen. Weitere Architekturarbeit braucht danach frische Gegen-Evidence oder einen neuen expliziten Produktauftrag.

**Typische Fehlform**

Temporal-, Carrier-, Rest- und Renderer-Invarianten sind bereits grün; übrig bleibt eine Hand-/VoiceRun-Frage oder ausschließlich der Public-Artifact-Commit. Trotzdem wird der Serializer weiter verändert, ein neuer Repair-Pass ergänzt, ein Renderer-Modell nachgebaut oder ein geplanter Cutover vervollständigt, nur weil der Arbeitsstrang ursprünglich dort begonnen hat.

**Vor dem Patch prüfen**

Welche **frische verbleibende** Fehlerklasse blockiert den Abschluss jetzt tatsächlich? Welcher kanonische Owner besitzt genau diese Entscheidung? Welche bereits grüne Schicht würde der geplante Patch unnötig wieder öffnen? Braucht das Produkt die geplante Capability für den nachweisbaren Abschluss überhaupt noch?

**Harter Bezug**

E01/E02 bestimmen Ursache und Owner; E03 trennt Decision und Materialization; E11/E12 binden letzten Write und persistierte Bytes; E14 verhindert neue Repair-Schichten; E19 interpretiert Gate-Fehler als Architekturinformation; E22 verhindert historische Aufblähung. Dieser Completion-Boundary-Vertrag ist advisory only und erweitert keine MusicMutation-, Writer-, Repair-, Renderer-, Release- oder Proof-Authority.

---

# Musikalischer-Erhalt-und-Metriksemantik-Vertrag

**Aussage**

> **Musikalische Veränderung braucht einen Beweis. Musikalischer Erhalt nicht.**

Darstellungs-, Normalisierungs-, Diagnose- oder Bewertungslogik erhält keine implizite Berechtigung, bereits vorhandene klingende Musik zu verändern. Wenn ein neu erzeugtes Darstellungsartefakt nur durch Verschieben, Verlängern, Verkürzen, Umstimmen oder Entfernen einer vorhandenen klingenden Note möglich wäre, wird ohne eigenständig autorisierte MusicMutation die Darstellungsänderung verworfen.

**Erhalt, Provenienz und Spielbarkeit sind getrennte Aussagen**

- `CanonicalSourceRetention` beschreibt den Erhalt beziehungsweise die Nachvollziehbarkeit kanonischer Quellereignisse im Source-/Materialmodell.
- `ArrangementVisibleRetention` beschreibt die sichtbare Materialmenge der Klavierdarstellung und ist keine Aussage über vollständigen Quellerhalt oder menschliche Spielbarkeit.
- menschliche Spielbarkeit ist eine eigene Evidenzklasse und darf weder aus einer hohen noch aus einer niedrigen Retention-Prozentzahl abgeleitet werden.

Eine hohe Erhaltungsquote beweist keine Spielbarkeit. Eine geringe Erhaltungsquote beweist umgekehrt nicht, dass die entfernte Musik aus Spielbarkeitsgründen entfernt werden musste.

**Unsicherheit ist kein Löschgrund**

Ein unsicheres, konservatives oder noch nicht empirisch gebundenes Spielbarkeitsurteil darf allein keine Note entfernen oder musikalisch verändern. Ein zukünftiges Pianistenprofil kann eine MusicMutation nur dann fachlich begründen, wenn der konkrete Konflikt nachvollziehbar ist: betroffene Hand beziehungsweise Finger-/Übergangssituation, verletzte Grenze und der Beleg, warum eine erhaltende Lösung nicht ausreicht.

**Metrik-Semantik**

> **Eine Kennzahl ist erst dann ein Beweis, wenn Bedeutung, Bezugsgröße und Berechnung eindeutig sind.**

Prozentwerte ohne eindeutigen Nenner oder mit vermischten Bedeutungen sind Diagnosewerte, keine Beweise. Reporting muss Retention-, Provenienz- und Spielbarkeitsmetriken begrifflich getrennt halten; gleichlautende Bezeichnungen für verschiedene Berechnungen sind zu vermeiden.

**Forward-Provenienz**

Ist die Herkunft eines Fragments vor einer bekannten Mutation gebunden, wird diese Identität vorwärts mitgeführt. Eine spätere Rückwärtsableitung aus Pitch, Onset, Staff, Voice, Chord-Lage oder räumlicher Nähe darf verlorene Source-/Attack-Provenienz nicht ersetzen.

**Owner-Grenzen**

Jeder schreibende Verarbeitungsschritt wird gegen seine eigene fachliche Verantwortungsgrenze geprüft. Zwei nacheinander schreibende Schritte dürfen nicht unter einer gemeinsamen Reconciliation verschwimmen, wenn dadurch unklar wird, welcher Owner eine Mutation tatsächlich verursacht hat.

**Vor dem Patch prüfen**

Welche klingende Eigenschaft soll sich tatsächlich ändern? Welcher konkrete fachliche Beweis verlangt diese Änderung? Welche Retention- oder Spielbarkeitskennzahl wird verwendet, was ist ihr Nenner und wie wird sie berechnet? Ist die Source-/Attack-Provenienz vor dem Write bereits bekannt und wird sie explizit weitergetragen?

**Harter Bezug**

Dieser Vertrag ist advisory only. Er erweitert keine MusicMutation-, Solver-, Candidate-, Playability-, Writer- oder Proof-Authority. Er verlangt insbesondere nicht, heute ein Pianistenprofil produktiv zu binden; er verhindert nur, dass eine noch unbewiesene Bewertung stillschweigend zur Lösch- oder Mutationsberechtigung wird.

---

# Verbindliche Vor-Patch-Denkfolge

Für Änderungen mit MusicMutation-, Writer- oder struktureller Materialisierungswirkung gilt als Essenz:

```text
Ursache
↓
fachlicher Owner
↓
Capability-Klasse
↓
kanonische Authority
↓
reale produktive Call-Site
↓
produktiver Caller
↓
Writer-Root
↓
bekannte Fehlmuster prüfen
↓
erst dann Patch

```

# Verbindliche Vor-Candidate-Prüffolge

```text
finaler geplanter Diff
↓
ChangeClass × PathRole × ChangeKind
↓
bei AuditOnly: benoetigte Datenkanten und Transportpfade mitpruefen
↓
private State nur ueber bereits autorisierte audit-sichtbare Transporte konsumieren
↓
Widerspruch? Owner oder kanonische Pfadklassifikation klären
↓
keine ChangeClass-Hochstufung und keine kosmetische Pfad-/Namensänderung zum Gate-Passieren
↓
erst dann Candidate-/ZIP-Bau

```

# Verbindliche Nach-Patch-Beweisfolge

```text
materialisierter Candidate
↓
geplante Take-Dateien bestimmen
↓
geplanten incoming ZIP-Inhaltsbaum erzeugen
↓
Take-Tree-Integrity: erlaubte Top-Level-Rollen, unknown = 0
↓
tatsächlicher finaler Diff ↔ Take-Deklaration
↓
undeclared = 0; declared_but_unchanged = 0
↓
ChangeClass × PathRole × ChangeKind erneut auf dem finalen Paketdiff
↓
Capability-/Semantic-Diff
↓
produktiver Call-Graph
↓
Writer-Root-Reachability Parent ↔ Candidate
↓
keine neue nichtkanonische Erreichbarkeit
↓
keine neue Identity-/Feedback-Capability
↓
reale produktive Einbindung / kein unbeabsichtigter dead_code
↓
tatsächlicher letzter Write
↓
final persistierter Snapshot
↓
read-only Readback
↓
Strict Clippy / Build / Tests
↓
Candidate-Fingerprint unverändert
↓
finales ZIP aus dem bereits geprüften Inhaltsbaum
↓
Take-Gates erneut fail-closed
↓
gezielte Regression
↓
Korpus

```

# Rollenmodell

**Essenz**

Erinnert, warnt, erklärt und zeigt bevorzugte Lösungsräume.

**Contracts / Authorities**

Definieren die Architekturwahrheit.

**Gates**

Beweisen beziehungsweise verwerfen den Candidate.

**Regressionen**

Bewahren bekannte positive und negative Architekturklassen.

**Korpus**

Beweist die Breitenwirkung des zulässigen Candidates.

**Assistenten-Erinnerung**

Beschleunigt die Arbeit, besitzt aber keinerlei normative Bedeutung.

---

# Qualitätsmaßstab der Essenz

Die Essenz ist gut genug, wenn folgende Aussage wahr wird:

> Wird die komplette externe Erinnerung eines Bearbeiters gelöscht, verliert EtuSchlopf Effizienz nur geringfügig, aber keine bereits erarbeitete fundamentale Architekturkompetenz.

Noch konkreter:

> Eine Maus ohne Erinnerung hätte mit dieser Essenz die bekannten Proof-Reentry-, Writer-Root-Reachability-, Identity-, Liveness- und Manifest-Sackgassen bereits vor dem jeweiligen Patch als Hochrisikomuster erkannt.

---

# Nichtziel

Die Essenz ist ausdrücklich kein vollständiges Projekttagebuch.

Sie speichert keine Einzelfalllösung nur deshalb, weil sie einmal wichtig war.

Sie besitzt keine neue Schreibautorität.

Sie darf keine Gate-Regel abschwächen.

Sie darf keine bestehende Authority umgehen.

Sie erzeugt keine Musik.

Sie repariert keine Musik.

Sie entscheidet keinen Release.

Sie macht vorhandene Erfahrung früh verfügbar.

---

# Kurzform

**Erinnerung kann verloren gehen.**

**Essenz bleibt im Projekt.**

**Essenz lenkt.**

**Authority schreibt.**

**Proof beobachtet.**

**Gate entscheidet.**

**Korpus beweist Breite.**

## Authority, Snapshot und PhysicalStart sind drei getrennte Provenienzachsen

Ein akzeptierter Parent (`Authority`), ein exportierter Source-Snapshot (`Snapshot`) und der beim naechsten Take tatsaechlich installierte Arbeitsbaum (`PhysicalStart`) duerfen nicht automatisch gleichgesetzt werden. Recovery verwendet verifizierte Parent-Bytes fuer die fachliche Zielsemantik, aber der deklarationspflichtige Diff wird gegen den **aktuellen** PhysicalStart bestimmt. Ein fail-closed `Take-Change-Control`-Befund kann deshalb einen aelteren Snapshot praezisieren: `declared_but_unchanged` beweist, dass die betreffende Recovery-Aktion im aktuellen Arbeitsbaum bereits vollzogen ist. Dann wird sie im naechsten unabhaengigen Kandidaten nicht erneut deklariert. Rejected Zwischenkandidaten bleiben Failure Evidence und erhalten keine Parent- oder Musik-Authority.

## Authority Retirement

Eine bereits vorhandene produktive Writer-Authority darf nur ueber `authority_retirement_v1` rein monoton sinken. Der Retirement-Take besitzt keine Ausnahme von `causal_class_first_v1` oder `candidate_duplication_v1`: Candidate-Klasse, Selector-Relation, Target-Auswahl, Multiplicity und CalibrationThreshold bleiben unveraendert; neue oder ersetzende Writer-Authority ist verboten. Erst nach dem Writer-Retirement duerfen read-only gewordene Selector-/Threshold-Altlasten separat pensioniert werden.
