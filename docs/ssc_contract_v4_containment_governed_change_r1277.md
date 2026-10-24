# SSC-Vertrag v4 und Take-Governance `GovernedChange` (final)

Stand: 07.10.2026, Basis: accepted r1276 (`14c5edbd…995b39`). Status: **final zur Umsetzung.** Eingearbeitet ist das Review von GPT 6.1 mit drei Korrekturen:

- Eingrenzung auch für Preserving (A3),
- Governance-Änderungen nur mit Zulassung durch den Parent (B3),
- WAV-Firewall über Eingaben und Abhängigkeiten (B4.1).

Der Vertrag hat zwei Teile:

- **Teil A** legt neu fest, was ein SSC-Permit bedeutet.
- **Teil B** legt neu fest, was ein einzelner Take gemeinsam ändern darf.

Beide Grundsatzentscheidungen hat der Nutzer am 07.10.2026 getroffen. **Budget: 10 Takes für ein vollständiges SSC mit Enforcement.** Das ist ein Budget, keine Obergrenze: Wird es überschritten, wird neu geplant. Abgekürzt wird nichts.

---

## Teil A: SSC-Permit durch Eingrenzung

### A1. Entscheidung

Ein SSC-Permit bedeutet nicht mehr „der Übergang war notwendig“. Es bedeutet: **Der Übergang hat nachweislich nur getan, was sein eingefrorener Owner-Vertrag erlaubt.** Notwendigkeit wird weiter konstruktiv geprüft und berichtet, entscheidet aber nicht über das Permit.

Die Garantie wird bewusst und offen abgeschwächt:

- **Garantiert:**
  - Kein Owner schreibt außerhalb seines Vertrags.
  - Was er erhalten muss, bleibt erhalten.
  - Belege sind an genau ein Event gebunden, nicht wiederverwendbar und vollständig.
- **Nicht mehr garantiert:** dass jede Änderung musikalisch nötig war.

### A2. Verdikte

| Verdikt | Bedingung | Wirkung unter Enforcement |
|---|---|---|
| `Permit` | Integrität ok **und** Eingrenzung bewiesen | Adoption erlaubt |
| `ContainmentDenied` | konstruktiver Witness: eine Änderung außerhalb des Envelopes oder ein Bruch der Invariante | Adoption verweigert |
| `ContainmentUnproven` | kein Envelope oder Beweis nicht führbar | Adoption verweigert |
| `Missing` / `Duplicate` / `Invalid` | wie in v2 | Adoption verweigert |

- **Notwendigkeit** wird je Familie nur gemessen (`NecessityProven` / `NecessityUnproven`). Sie beeinflusst weder das Verdikt noch eine Adoption.
- **Bericht:** Ein Permit für einen Owner, dessen Envelope Tonhöhe, Dauer oder Notenzahl ändern darf, trägt den Vermerk `content_changing`.
- **`OutOfScope`** gibt es im Endzustand nicht mehr. Ein Owner ohne Envelope ist `ContainmentUnproven`.

### A3. Envelopes

Jeder Envelope verlangt **Eingrenzung**: Es ändern sich nur deklarierte Elementarten, Felder und Operationen. Eine Invariante allein reicht nie.

| Envelope | Receipt | Beweis aus exakten Pre/Post-Bytes plus gesiegeltem Receipt |
|---|---|---|
| **Projektion** | Preserving | Zwei Bedingungen gelten zusammen: (a) Die Conserved-Projection des Owners ist in Pre und Post gleich. (b) Jede Differenz liegt in einer deklarierten Elementart, betrifft nur deklarierte Felder und gehört zu einer deklarierten Operation (ändern, einfügen, entfernen); außerhalb ist alles bytegleich. Eine Differenz außerhalb des Envelopes ergibt `ContainmentDenied`, auch wenn die Projektion erhalten bleibt. |
| **Insertion** | Additive | Genau eine zusammenhängende Einfügung der deklarierten Elementklasse an einem erlaubten Ort. Sonst ist alles bytegleich. |
| **Edit** | Exact | Jede Edit des bytegeprüften Edit-Skripts liegt in einem Element der deklarierten Art. Dort ändern sich nur deklarierte Felder. Außerhalb der Edits ist alles bytegleich. |
| **Block** | Block | Außerhalb des gebundenen Spans ist alles bytegleich, sofern ein Span vorhanden ist. Innerhalb unterscheidet sich der Zustand nur in den deklarierten Elementarten und Feldern; der strukturelle Vergleich ignoriert genau diese. |
| **Lane** | Exact (F5/F7/F9) | Nur `voice`, `staff` und `print-object` der gebundenen Noten ändern sich. |

Ein Preserving-Owner, der mehrere Elementarten berührt (zum Beispiel E051: Beams und Schlüssel), deklariert jede davon einzeln samt Feldern und Operation.

### A4. Envelope-Tabelle

Für jeden Transformations-, Preserving- und Additive-Descriptor gibt es genau einen Eintrag: Descriptor, Receipt-Art, erlaubte Elementarten, Felder und Operationen, Invariante sowie das Flag `content_changing`. Dafür gelten feste Regeln:

1. **Herkunft:** Ein Eintrag wird aus dem eingefrorenen Owner-Vertrag abgeleitet, also aus Signatur, dokumentiertem Zweck und Code. Er wird **nie aus der Korpusverteilung** abgeleitet. Korpusdaten dienen nur der Messung (`causal_class_first_v1` gilt).
2. **Keine Wildcards:** Angaben wie „beliebiges Element“, „alle Felder“ oder „beliebige Operation“ sind verboten. Darf ein Envelope Tonhöhe, Dauer oder Notenzahl ändern, muss er das einzeln deklarieren und begründen.
3. **Erweitern** ist nur über die vom Parent zugelassene Änderungsart `envelope_extend` erlaubt (siehe B3.2), mit Begründung aus dem Owner-Zweck. **Verengen** ist jederzeit erlaubt.
4. **Bindung beim Mint:** Die Zuordnung zum Eintrag entsteht beim versiegelten Mint über den Descriptor, nie über Strings. Ein Scope auf fremdem Receipt-Typ oder fremdem Descriptor ergibt `ProofFamilyMismatch`.

### A5. Regel bei `ContainmentDenied`

Ein Denied ist ein echter Befund und wird nicht verdeckt. Entschieden wird so:

1. Zuerst wird **Checker gegen eingefrorenen Owner-Zweck** verglichen.
2. Liegt die beobachtete Änderung im dokumentierten Zweck des Owners, ist der Envelope zu eng. Er wird per `envelope_extend` mit Begründung aus dem Zweck erweitert.
3. Liegt sie außerhalb des Zwecks, schreibt der Owner falsch. Die Korrektur ist eine Musikänderung und geschieht **nur mit ausdrücklicher Nutzerfreigabe pro Take** (`music_effect=declared`).
4. Ist der Zweck nicht eindeutig, entscheidet der Nutzer. Bis dahin bleibt das Event `ContainmentDenied`.

### A6. Unverändert aus v2/v3

Event-Identity sowie Domain- und Descriptor-Bindung, versiegelter Capture, Fingerprint-Barrieren, released Nenner unabhängig von Records, Released-Bilanz, Duplicate- und Missing-Klassen, kein Wiederverwenden von V1-Verdikten, keine WAV-Zertifikate (verschärft in B4.1), keine Selektor-, Ranking- oder Musikautorität aus SSC.

### A7. Enforcement

1. Das Verdikt wird **vor** der Adoption berechnet: in `OwnerReceipt::commit` (Master) und in `RenderLineage::commit` (Render).
2. Jedes Verdikt außer `Permit` heißt: Adoption verweigert, kein Teilcommit, kein Fallback auf einen anderen Kandidaten. Auf dem Master-Pfad ist das eine Souveränitätsverletzung.
3. Das Permit gilt genau für diesen Receipt. Receipts sind move-only, ein Replay ist damit ausgeschlossen.
4. **Eingeschaltet wird erst**, wenn im Zehnerkorpus alle released Events `Permit` sind und die Musik bytegleich bleibt: 20 XML, alle PDF-Seiten gerendert gleich, released Lineage gleich.

---

## Teil B: Take-Governance `GovernedChange`

### B1. Entscheidung

Die Regel „ein Take, eine Klasse“ wird gelockert. Unter ihr musste zusammengehörige Arbeit mehrfach in Vorläufer- und Folge-Takes zerlegt werden; das hat massiv gebremst.

Erhalten bleibt genau eine Schutzeigenschaft: **Kein Take darf ein Gate abschwächen und die Lücke im selben Take unsichtbar ausnutzen, und kein neuer Validator gibt sich selbst frei.**

### B2. Neue Klasse `GovernedChange`

Ein `GovernedChange`-Take darf Kodex-, Laufzeit-, Test- und Metadatenpfade gemeinsam ändern. Die bestehenden Klassen (`KodexOnly`, `RefactorOnly` …) bleiben unverändert gültig.

### B3. Bedingungen

1. **Abschwächung nur sichtbar.**
   - Jede gelockerte, verengte, erweiterte oder entfernte Prüfung steht als eigene Manifestzeile mit ihrer Änderungsart: `govern=<art>|<pfad>::<prüfung>|<stabiler_grund>`.
   - Entfernte Prüffunktionen brauchen zusätzlich den Eintrag im Retirement-Ledger, wie bisher.
   - Alle `govern=`-Zeilen erscheinen im Ergebnisbericht.
   - Eine geänderte Prüfung ohne passende Zeile ist eine Ablehnung.
2. **Zulassung durch den Parent, nicht durch das Kind.**
   - Eine Manifestzeile allein lockert nichts. Wirksam ist eine Regeländerung nur, wenn der **kompilierte Parent-Validator** ihre Änderungsart kennt **und** die konkrete Änderung **und** ihre Nutzung im selben Take prüft und zulässt.
   - Der neue Validator des Kindes darf keine Änderungsart, keine Ausnahme und keine Zulassung für sich selbst begründen.
   - Der Parent kennt nur Arten, die ein früherer, angenommener Take installiert hat. Take 1 installiert diese abschließend:

     | Art | Was der Parent prüft |
     |---|---|
     | `envelope_extend` | Neuer oder erweiterter Eintrag der Envelope-Tabelle; Wildcard-Verbot, ein Eintrag pro Descriptor, `content_changing`-Deklaration, Begründung aus dem Owner-Zweck |
     | `envelope_narrow` | Verengung eines Eintrags; immer zulässig |
     | `surface_extend` | Neue Variante der SSC-Typoberfläche aus der Vertragsliste; bestehende Varianten unverändert, keine Wildcards |
     | `proof_rule` | Änderung an Beweis-Code innerhalb der Inhaltsregeln aus B4.1 |
     | `verdict_wiring` | Verdikt-Berechnung und Enforcement nur an den Adoptionsgrenzen aus A7; keine neue Writer-Autorität |
     | `retire` | Entfernen einer Prüfung mit Retirement-Ledger-Eintrag; nie für die Schutzbedingungen in B3 |

   - Jede Regeländerung **außerhalb** dieser Arten braucht weiterhin einen eigenen vorgelagerten Take, der die neue Art installiert. Das ist die Ausnahme und kostet einen Take.
   - Zusätzlich gilt: Der neue Baum besteht die Gates des Parents, außer den zugelassenen `govern=`-Zeilen, und seine eigenen neuen Gates.
3. **Musikwirkung deklariert.**
   - Das Manifest trägt `music_effect=none` oder `music_effect=declared|<begründung>`.
   - `none` heißt: 20 XML bytegleich, alle PDF-Seiten gerendert gleich, released Lineage gleich. Sonst wird bei der Abnahme abgelehnt.
   - `declared` verlangt eine ausdrückliche Nutzerfreigabe der Musikänderung, pro Take.
4. **Unverändert:** Rejection-Ledger mit harter Sperre exakter Replays, Trust-Anchor-Pins, kein `--force`, kein Update der Root-`Cargo.lock`, Windows-Abnahme mit FinalExit 0 und 10/10, Corpus-Independence, keine neue Writer-Autorität außerhalb von StructuralWriteAuthority.

### B4. Mitgelieferte Vereinfachungen

1. **Inhaltliche statt Ganzdatei-Bindungen.** Die Hash-Bindungen von `ssc_evidence_proof.rs` und `master_state_identity.rs` (#57) werden durch Prüfungen ihres Inhalts ersetzt.

   - **WAV-Firewall.** Ein Token-Check ist nur die Mindestbedingung. Verbindlich sind zusätzlich:
     - **Eingabe-Allowlist:** Zertifikats- und Verdiktbildung liest ausschließlich gesiegelte Pre/Post-Captures, den Receipt mit Descriptor und die eingefrorene Envelope-Tabelle. Andere Parameter, globale Zustände, Dateien oder Umgebungsvariablen sind nicht erlaubt.
     - **Abhängigkeits-Allowlist:** Das Beweismodul importiert und ruft nur Module einer festen Liste (XML-Parsing, Identity, Envelope-Tabelle, Std). Analyse-, WAV-, Salience-, Tempo-, Selektor- und Ranking-Module sind nicht erreichbar, weder direkt noch transitiv über die Liste.
     - **Keine WAV-, Salience- oder `estimated_bpm`-Tokens** im Beweismodul, und die Aufrufer-Zählung bleibt gleich.
     - Eine neue Eingabe oder Abhängigkeit braucht eine `proof_rule`-Zeile, die der Parent gegen diese Listen prüft. Eine Erweiterung der Listen selbst ist keine zugelassene Art.
   - **#57:** Die gebundenen Funktionen werden einzeln gehasht statt der ganzen Datei.

   Pre-Admissions exakter Folgebytes entfallen damit.
2. **Parent nach eigenen Regeln.** Im Transaktions-Reexec wird der Parent-Baum nur noch mit seinen **eigenen** kompilierten Regeln geprüft. Das Kind prüft sich selbst und die Übergänge. Die Ausnahmen „exakter Installations-Parent“ entfallen.
3. **Erweiterbare Typoberfläche.** Neue Varianten werden über die Vertragsliste und die Art `surface_extend` zugelassen statt über ein neues eingefrorenes Dokument pro Erweiterung.

### B5. Prozessregeln (aus r1274–r1276)

- Ein Folge-Take wird erst nach der Annahme seines Parents ausgeliefert. Grund: Der Bootstrap wählt immer die höchste Revision aus Projektordner und `Downloads`.
- Vor jeder Auslieferung werden lokal nachgestellt:
  - die Admission mit dem kompilierten Parent-Validator;
  - der Transaktions-Reexec (Pending-Marker, Backup mit `root_files`);
  - Volltests, Gate-Tests, Clippy und Strict-Build im Transaktionsmodus.

---

## Teil C: Plan

| # | Take | Klasse | Ziel | Abnahme |
|---|---|---|---|---|
| 1a | **Governance-Enabler**: Klasse `GovernedChange` (B2), `govern=`-Manifest und die sechs Änderungsarten mit Parent-Prüfung (B3), Musikdeklaration (B3.3), Parent nach eigenen Regeln (B4.2), erweiterbare Oberfläche (B4.3), Vertrag v4 als Dokument, Kodex-Prüfung der Envelope-Tabelle | `KodexOnly`, letztes Mal unter altem Recht | Ab hier gilt `GovernedChange` | Musik und SSC-Bilanz unverändert |
| 1b | **WAV-Firewall inhaltlich** (B4.1): Eingabe- und Abhängigkeits-Allowlist, Funktions-Hashes #57, Ablösung der Ganzdatei-Bindungen | `GovernedChange` (erster Einsatz) oder Teil von 1a | Beweis-Code ohne Pre-Admission änderbar | Musik und SSC-Bilanz unverändert |
| 2 | **Verdikte v4** (A2) für Projektion, Insertion und Lane, inklusive Preserving-Eingrenzung (A3) für alle 13 aktiven Preserving-Ausschlüsse; Notwendigkeit nur noch als Messung | `GovernedChange` | Permit deutlich über 10 (Obergrenze etwa 264); ehrliche Denied werden sichtbar | Musik neutral, Bilanz vollständig |
| 3 | **Edit- und Block-Envelope** mit Tabelle für alle Exact- und Block-Owner | `GovernedChange` | Alle 1438 released Events sind `Permit`, `ContainmentDenied` oder `ContainmentUnproven` mit Witness; `OutOfScope` = 0 | Musik neutral |
| 4 | **Korrektur-Take** nach A5 | `GovernedChange` | Alle released Events sind `Permit` | wie deklariert |
| 5 | **Enforcement** (A7) mit Negativtests: fremde Identity, Domain oder Descriptor, stale Pre, abweichender Post, kaputter Capture, Replay, Änderung außerhalb des Envelopes, Ablehnung ohne Teilcommit | `GovernedChange` | `PermitEnforcement: active` | Musik bytegleich |
| 6 | **Abschluss**: `exit_claim=closed`, Rückbau der Shadow-only-Formulierungen und -Gates | `GovernedChange` | SSC abgeschlossen | Musik bytegleich |
| Rest bis 10 | Puffer, 2 bis 3 Takes | | | |

**Gelingen 1a und 1b gemeinsam, sind es 7 Kern-Takes, sonst 8.** Das entscheidet sich bei der Aufwandsschätzung für Take 1, nicht beim Bauen.

### C1. Take 1: Umfang und Aufwand

Take 1 ist der größte und riskanteste Take. Er ändert unter altem Recht genau die Prüfungen, die ihn selbst prüfen.

- **Betroffene Bereiche:**
  - `take_change_control.rs`: Klasse, Manifestzeilen, die sechs Arten mit Parent-Prüfung, Musikdeklaration;
  - `semantic_capability_diff.rs`: Anbindung der `govern=`-Zeilen an die bestehenden Gates;
  - `ssc_k2_retirement_governance.rs`: inhaltliche Bindungen, erweiterbare Oberfläche, Wegfall der Installations-Parent-Ausnahmen;
  - neuer Abhängigkeits- und Eingabe-Checker für das Beweismodul (1b);
  - Trust-Anchor-Pins;
  - Vertragsdokument und Envelope-Tabellenformat mit Kodex-Prüfung.
- **Pflichttests:** pro Änderungsart je ein positiver Fall und mindestens zwei negative Fälle (Art fehlt im Parent; Kind gibt sich selbst frei). Dazu kommen pro Allowlist eine verbotene Eingabe und eine verbotene transitive Abhängigkeit, und alle bestehenden Gate-Tests bleiben grün.
- **Trennkriterium 1a/1b:** Lässt sich der Abhängigkeits-Checker nicht unter `KodexOnly` ausliefern, ohne dass der r1276-Parent im Reexec scheitert, wird 1b ein eigener Take. Dann ist es der erste Einsatz von `GovernedChange` mit der Art `proof_rule`.
- **Vorab-Lieferung:** Vor dem Bau kommt eine konkrete Datei- und Testliste mit der Entscheidung 1a/1b, geprüft gegen den lokal nachgestellten r1276-Reexec.

---

## Teil D: Risiken

- **Dokumentweite Owner** (#52 VisibleRhythm, #53, #55, #57, #66) lassen sich nur grob eingrenzen. Ihr Envelope wird breit und ist als `content_changing` deklariert. Lässt sich ein Owner nicht sauber eingrenzen, wird Take 4 größer oder braucht einen Puffer-Take.
- **Preserving-Eingrenzung** kann in Take 2 Denied aufdecken, die bisher hinter dem reinen Projektionsbeweis lagen. Das ist erwünscht und geht in Take 4.
- **Änderungsarten:** Brauchen wir in Take 2 bis 6 eine Regeländerung, die keine der sechs Arten abdeckt, kostet das einen zusätzlichen Take.
- **Musikänderungen** nach A5 hängen an der Nutzerfreigabe und können Takes außerhalb des Plans ziehen.
- **Notwendigkeit wird nicht mehr durchgesetzt.** Wer sie später zurückwill, braucht einen neuen Vertrag.
- **Die Schutzbedingungen in B3 sind nicht verhandelbar.** Bei einer verletzten Bedingung gibt es eine Ablehnung, keine Ausnahme.

---

## Teil E: Festlegungen und offener Punkt

Festgelegt:

1. **Denied-Regel:** wie in A5. Zuerst Checker gegen Owner-Zweck, Erweitern nur innerhalb des Zwecks, Musik nur mit Nutzerfreigabe.
2. **Freigabe von `music_effect=declared`:** nur der Nutzer, ausdrücklich und pro Take.
3. **Budget:** 10 Takes sind ein Budget, keine Obergrenze. Bei Überschreitung wird neu geplant, ohne Abstriche an A oder B3.

Offen:

4. **Bauaufteilung für Take 1:** Wer baut ihn (Opus oder Codex), und wer prüft ihn adversarial?

---

## Umsetzungsnotiz r1277 (Take 1)

Take 1 ist als ein Take gebaut (1a und 1b zusammen), `KodexOnly`, benannte Revision `SscK2GovernanceRevisionKodex`. Laufzeit, Musik und SSC-Bilanz bleiben unverändert.

- **Ort der Parent-Prüfung:** `src/build_gate/ssc_governed_change.rs`. Die Datei gehört zu den K2-Checkerdateien (CODE) und ist im Trust-Anchor gepinnt. Ein `GovernedChange`-Take kann sie deshalb nicht ändern.
- **Pfadpolitik:** `GovernedChange` darf Kodex-Pfade nur für die drei verwalteten Datendateien ändern und, bei `retire`, das Retirement-Ledger, den Trust-Anchor (nur Pin-Literale) sowie die deklarierte Zieldatei. Validator-Code ist nie ein `GovernedChange`-Pfad.
- **`retire`** gilt enger als in der Tabelle B3.2:
  - keine Laufzeitänderung im selben Take;
  - nur reine Zeilenlöschung;
  - Ledger-Eintrag Pflicht;
  - nie in K2-Checkerdateien oder B3-Schutzdateien.

  Der SSC-Abschluss (Take 6) wird deshalb eine benannte `KodexOnly`-Revision.
- **`music_effect=none`** wird im Manifest deklariert und im Bootstrap ausgegeben. Ob die Musik tatsächlich bytegleich ist (20 XML, PDF-Seiten, released Lineage), prüft weiterhin die Abnahme anhand des Exports. Der Bootstrap vergleicht das nicht automatisch.
- **#57** bindet in `master_state_identity.rs` jetzt drei Items: `RENDER_ROW_57_BLOCK`, `struct BlockTransformationReceipt` und `mint_file_bound_block_receipt`. Die übrigen #57-Dateien bleiben ganzdateigebunden.
- **WAV-Firewall:** Für alle vier Beweisdateien gelten
  - ein Tokenverbot;
  - eine Eingabe-Allowlist: keine fs/env/time/process/net-Zugriffe, keine neuen `static`-Zustände, `include_str!` nur für feste Pfade;
  - eine Abhängigkeits-Allowlist: erlaubte crate-Module und -Items, externe Crates nur `roxmltree` und `flate2`, `super::` nur als `super::*`;
  - eine transitive Prüfung der erlaubten Module `master_state_identity*` und `final_report.rs`;
  - die Regel, dass die Eingaben des Observers nur versiegelte `Ssc*`-Typen sind.
- **Selector-Fehlerklassenvertrag:** `docs/selector_derivation_contract_ssc_containment_error_class.md` ist vorab eingefroren. Verdikt-Takes referenzieren ihn als `ParentErrorClassContract`.
- **Probe:** Lokal wurde ein simulierter Take 2 vom r1277-Parent in einem Schritt zugelassen. Er enthielt eine Beweisdatei-Änderung mit `proof_rule`, eine Envelope-Zeile mit `envelope_extend`, eine Surface-Variante mit `surface_extend` und einen Selector-Implementierungsvertrag. Abgelehnt wurden dagegen:
  - ohne `govern`-Zeilen;
  - mit Selbstzulassung über eine Änderung an `ssc_governed_change.rs`;
  - ohne `proof_rule`;
  - als `RefactorOnly`.
