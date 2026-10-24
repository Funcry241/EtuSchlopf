# r1242 SscK2RetirementGovernanceKodex – konsolidierte Baugrundlage

Stand: 04.10.2026. Source-Owner: 6.1. Parent: accepted r1241.
ChangeClass: KodexOnly. Dies ist ein Vertragsartefakt, kein gebauter oder accepted Take.

## 1. Vorrang und Quellen

Diese Konsolidierung ersetzt für die nachstehenden Regeln die widersprüchlichen Teile der beiden 5.6-Entwürfe. Sie verbindet die vollständige 15-ID-Liste des ersten Entwurfs mit den Korrekturen des zweiten. Kein ausgelassener Absatz gilt automatisch weiter.

Ausgewertet wurden die lokalen Uploads:
- Eingefügter Text(20261004-113125).txt: ursprünglicher vollständiger Vertrag.
- r1242_k2_retirement_governance_review.md: Review der früheren Gliederung.
- Eingefügter Text(20261004-114652).txt: überarbeiteter Vertrag.
- r1242_vertrag_5_6_vollpruefung.md: Vollprüfung des ursprünglichen, nicht des überarbeiteten Vertrags.

Source-Abgleich: accepted r1241, lokal als ssc_r1241_parent. Die Vollprüfung ist kein PASS für die neueste Fassung. Frühere Build-/Korpusmessungen beweisen keine künftige Implementierung.

## 2. Scope und unveränderte Daten

r1242 installiert Prüfregeln und Vorbereitungsverträge. Keine Runtime-, Writer-, Selector-, Ranking-, Musik-, Retirement- oder Enforcement-Änderung. Keine Population- oder Restmengenänderung. Keine neue globale Commit-Zählung. Keine Cargo-Änderung. Die drei installierten SSC-v1-Vertragsdateien und die historische Baseline bleiben bytegleich. Freeze und Scanner bleiben in r1242 unverändert.

Die im Parent gemessenen 841 Master + 725 Render = 1.566 OwnerEvents, Permit 10, Deny 0, Unknown 1.556, MissingOriginCapture 0 und N_A 15 sind eine Referenzmessung. Sie sind keine Sollwerte für einen späteren Retirement-Take.

## 3. Autorität und Prüfebenen

E1: Das accepted Parent-Binary prüft Parent→Candidate. Neue r1242-Regeln autorisieren r1242 selbst nicht rückwirkend.

E2: Candidate-Code prüft die Transition und muss den r1241-Backup weiterhin vollständig akzeptieren. Runtime-V1 und DEFERRED-Statuswerte des Parents bleiben zulässig.

E3: Child-Build prüft den Child-Baum. Es besitzt keinen Parent-Baum. Freeze wirkt hier unmittelbar; deshalb keine vorgezogene Freeze-Lockerung.

Neue Governance-, Surface-, Observer- und Firewall-Dateien werden als include_str!-Konstanten gebunden. Das Parent-Gate verlangt die entsprechenden Candidate-Dateien bytegleich zu seinen Konstanten. TA-Pins ergänzen diese Bindung, ersetzen sie aber nicht. Eine absichtliche spätere Vertragsrevision benötigt eine eigene Governance-Änderung; kein Candidate darf Regeln als bloße Dateneingabe lockern.

## 4. Restmenge und dauerhafte Pflichten

Baseline ist die unveränderte Menge der 15 IDs aus ssc_exit_dependencies_r1238_v1.tsv.

- E1/E2: ChildOpen ⊆ ParentOpen ⊆ Baseline.
- E3: ChildOpen ⊆ Baseline.
- Keine neue oder wieder geöffnete ID; keine doppelten, unbekannten oder unlesbaren IDs.
- Eine erfüllte Pflicht darf zunächst offen bleiben.
- Jede gestrichene ID muss ihr vollständiges Prädikat erfüllen.
- Alle Prädikate von Baseline minus Open gelten dauerhaft auf jedem späteren geprüften Baum, auch ohne erneute Restmengenänderung.
- Die erste Streichung benötigt A4 bereits im accepted Parent. Eine unprüfbare Behauptung „A4 accepted“ reicht nicht; der Parent muss die gebundene A4-Runtime-Form tatsächlich enthalten.

Unentscheidbare oder unvollständig gelesene Prädikatanteile gelten als nicht erfüllt. Es gibt keine booleschen Erfolgs-Platzhalter für „semantisch äquivalent“, „accepted closure“ oder „kein Ersatz“.

## 5. Vollständige 15-ID-Prädikate

Für jeden Runtime-Retirement-Fall gelten zusätzlich Abschnitt 6 und die dauerhafte Persistenz. Namen-/Body-Abwesenheit allein ist kein Autoritätsbeweis. Ein mechanischer Prüfer muss die betroffenen Deskriptoren, Mint-Owners, Sinks und produktiven Caller erfassen. Read-Fehler, unaufgelöste Module und unbekannte Scan-Grenzen ergeben kein erfolgreiches Prädikat.

| ID | Erfüllungsprädikat | Grenze |
|---|---|---|
| deferred_render_57 | Unter r1242 immer nicht erfüllt; READY und Streichung gesperrt. Erst ein späterer benannter Fixvertrag darf das ändern. | Kein accepted_bool als Nachweis. |
| deferred_render_67 | POP #67 READY; bestehender K-R-Beweis Relocated; eine Target-Definition und ein guarded Call, zusammen zwei lexikalische Target-Verwendungen; exakte Signatur/Splice-Geometrie/Mint; kein Legacy-Inline; File-Write-Budget 0; Pins #2/#69 unverändert. | Historische Relocation-TSV unverändert; kein Rückweg. |
| wav_pending_28 | POP #28 RETIRED_SSC_K2; apply_bound_articulation_batch_authorized und Reexport vollständig entfernt; Deskriptor-/Mint-Pfad und alle produktiven Caller entfernt; wav_accent_productive erfüllt. | Kein Ersatz-Batch-Writer. |
| wav_accent_productive | WAV-Akzentproducer, produktive Ranking-/Stage-Kette, E029-Fragmentpfad, #28-Writerkette und WavTruthAccent-Sink entfernt; keine verbleibende produktive Authority aus dieser Kette. | Vorher gesonderte Produktentscheidung; keine neue Akzententscheidung. |
| wav_salience_master_adoptions | Produktive Aufrufe von run_salience_weighted_candidate_gate, run_discrete_action_salience_translator und run_conflict_staff_split_proof_trial samt exklusiver Decision-/Adoption-Kette entfernt. | Generische Notations-Stage und vom Audit gelesene Outcome-Felder erhalten; WAV-Diagnose ohne Autorität darf bleiben. |
| wav_metronome | ReferenceSignature::estimated_bpm und produktiver WAV-Dauer→BPM→Expression-Polish→Tempo-Direction-Pfad entfernt. | Generischer nicht-WAV-Tempo-Code darf bleiben. Kein erneuter WAV-BPM-Transport unter anderem Namen. |
| e031_tenuto | E031-Owner, TenutoFragment, mint_tenuto_fragment_receipt, zugehöriger Deskriptor/Mint, Materialisierungs- und exklusiver Probe-/Transportpfad vollständig entfernt. | E032-Tie bleibt exakt eingefroren. |
| ra5_accent_reserve | RA5-Funktion, Stage-Aufruf, write_accent_reserve_checkpoint_authorized-Keepalive, Sink/Reexport und exklusive Testreste entfernt. | Kein no-write Stub oder Ersatz-Sink. |
| macro_hand_role | cfg(test)-Owner apply_hand_role_rest_materialization_authorized, HandRoleRestToForward-Variante, Arm, Macro-Aufruf und exklusive Referenzen entfernt. | Kein produktiv gemachter Ersatz. |
| macro_cross_staff | cfg(test)-Owner apply_cross_staff_voice_ledger_authorized, CrossStaffVoiceLedger-Variante, Arm, Macro-Aufruf und exklusive Referenzen entfernt. | Wie vorstehend. |
| macro_terminal_shadow | cfg(test)-Owner apply_terminal_shadow_rest_forward_authorized, TerminalShadowRestToForward-Variante, Arm, Macro-Aufruf und exklusive Referenzen entfernt. | Wie vorstehend. |
| macro_same_pitch_tie | cfg(test)-Owner apply_same_pitch_tie_merge_authorized, SamePitchTieMerge-Variante, Arm, Macro-Aufruf und exklusive Referenzen entfernt. | Wie vorstehend. |
| macro_lower_rhythm_lane | cfg(test)-Owner apply_lower_visible_rhythm_rest_lane_authorized, LowerVisibleRhythmRestLane-Variante, Arm, Macro-Aufruf und exklusive Referenzen entfernt. | Wie vorstehend. |
| master_receipt_legacy_alias_contract | Rohtextvorkommen von MasterExactReceipt, MasterBlockReceipt, MasterPreservingReceipt und MasterAdditiveReceipt jeweils 0 in src/, tests/, tools/; neutrale Receipt-Typen bleiben unterstützt. | Gilt auch für Strings/Tests und verschobene Kodex-Dateien. |
| audit_carrier_grandfathering | NON_PRODUCTIVE_REBUILD_PREFIXES und die bisherigen src/audit_only/ bzw. src/audits/-Präfixausnahmen fehlen im Carrier-/Nominal-Prüfpfad; Inventar läuft unter allgemeinen Regeln. | Kein Ersatz derselben Ausnahme über andere Literale oder Helfer; AuditOnly erhält keine produktive Authority. |

Die fünf Test-Witnesses können einzeln entfernt werden. Die gemeinsame define_replacement_authority!-Definition verschwindet erst nach dem letzten Witness. Alle acht produktiven Visible-Lane-Owner bleiben eingefroren. Generische Vorkommen von „metronome“ oder der verbleibende Stage-Name sind keine Retirement-Anker.

## 6. R4×R5-Ausnahme für Retirement

Pflichtbindungen: SscExitObligations, RetiredAuthoritySymbols, RemovedCallerEdges und SurvivingCallerStructuralWitness. Der konkrete Parent/Candidate-Diff muss reinen Verlust der benannten Writer-Authority nachweisen, ohne neue Writer-Records, Deskriptoren oder Ersatz-Capabilities. Capability-Erfassung ist ein mechanischer Nachweis innerhalb ihrer belegten Scan-Grenzen, kein allgemeiner Beweis beliebiger Programmsemantik.

Zulässig sind vollständig entfernte Owner-Funktionen oder syntaktisch vollständige, exklusive Call-/Guard-Bereiche in überlebenden Callern. Sämtliche zugehörigen Zweige müssen geschlossen behandelt werden. Ausschließlich vom entfernten Bereich benötigte Bindungen dürfen mit verschwinden.

Für überlebende Statements bleiben relative Reihenfolge, umschließende Guards/Zweige und Verschachtelung unverändert. Keine verbleibende Wirkung wird unconditional. Keine entfernte Bindung darf außerhalb des entfernten Bereichs gelesen werden. Keine neue Selector-, Ranking-, Target-, Repair- oder Writer-Anweisung. Zeilenmultimengen sind hierfür unzureichend; eine blockbezogene Strukturanalyse ist Pflicht. Bei nicht belegbarer Erhaltung wird die Ausnahme abgewiesen.

Zusätzlich: keine Body-Verschiebung, kein umbenannter Autoritätsträger, kein neu erreichbarer Ersatz. Ein unveränderter Body-Fingerprint ist ein Zusatzcheck; ein veränderter Fingerprint beweist keine Entfernung.

## 7. Population und RSP

69 Population-Zeilen bleiben erhalten. Nur prerequisite_state darf in genau benannten Zeilen wechseln. Alle übrigen Bytes/Spalten bleiben unverändert. Die übrigen sieben RSP-Tabellen bleiben gepinnt.

- #28: PENDING_WAV_DECISION→RETIRED_SSC_K2, nur unter Abschnitt 5.
- #67: DEFERRED_RENDER_PREREQUISITE→READY, nur unter vollständigem K-R-Beweis.
- #55: PENDING_WRITE_PATH_RENAME_VISIBILITY→READY, nur unter Abschnitt 8.
- #57: keine READY-Transition unter r1242.
- Rücktransitionen und alle anderen Statuswechsel verboten.

RSP akzeptiert E029/E031 unabhängig exakt vorhanden oder vollständig absent: beide vorhanden, nur E029 absent, nur E031 absent, beide absent. Enum, Verbrauchsarme, Owner, Mint und Reexport müssen konsistent sein. E032 bleibt vorhanden und unverändert. Partielle Absenz wird abgewiesen. Die fünf Witness-Formen werden jeweils unabhängig behandelt; Zählprüfungen müssen entfernte Witnesses korrekt berücksichtigen.

## 8. #55 – Sichtbarkeit ohne Inventar-Selbstautorisierung

#55 ist keine Retirement-ID und bleibt außerhalb N_A. READY verlangt einen vollständig erfassten Render-Publish-Pfad einschließlich Rename-Erfolg und Copy-Fallback, gebunden an den kanonischen Artifact-Target und dieselbe logische Publikation. Kein weiterer ungescannter physischer Publish-Write im Owner-Pfad.

Die Transition darf keine neue Surface oder Budgetsteigerung unter den unveränderten Baseline-Tabellen verdecken. Eine bloße Scanner-Erweiterung erfüllt diesen Vertrag deshalb nicht automatisch.

Unter r1242 ist nur eine Closure innerhalb bereits zulässiger gescannter Publish-Sinks und ihrer eingefrorenen Budgets statusfähig; rohe rename/copy-Publikation außerhalb dieser Sinks muss fehlen. Ein notwendiger produktiver Umbau gehört in einen eigenen Runtime-Take und wird auf Byte-/Lineage-Neutralität geprüft. Scanner-/Freeze-Kodexanpassungen dürfen erst nach belegter Admission-Verträglichkeit in den späteren Statustake.

Falls die tatsächliche Closure eine neue Baseline-Surface oder ein neues Budget benötigt, bleibt #55 gesperrt, bis ein eigener, zeilengenauer Kodexvertrag diesen Übergang erlaubt. r1242 gibt keine generelle Baseline-Öffnung. Welche Route implementierbar ist, ist vor einem #55-Take konkret am Source zu belegen; derzeit kein Erfüllungsclaim.

## 9. Ursprüngliche Runtime-Schulden A1–A6

Die Bezeichnungen werden nicht neu vergeben:

| Schuld | Verbindliche Closure |
|---|---|
| A1 | Released Identity-Mengen bilden den Nenner unabhängig von Records; fehlender Record erhöht MissingEvidence. |
| A2 | Ohne released Scope explizites ReleaseScopeMissing; kein Candidate-Fallback. Master-/Render-Scope werden getrennt vollständig geprüft. |
| A3 | ADDITIVE bindet exakte Pre- und Post-Bytes an beide Receipt-Fingerprints. Fingerprint ist kein Identity-Ersatz. |
| A4 | Schema/Exit-Header und eindeutige IDs validieren; Baseline=Open disjunkt Retired; NAOpenGroups=Open.len; retirement=none genau bei leerem Retired. Fehler als unreadable, keine geratene Zahl. |
| A5 | Nicht dekomprimierbarer exakter Pre-State erzeugt eine explizite Integrity-Klasse; kein stilles continue, kein Verschwinden aus der Bilanz. |
| A6 | Released Render-Identities begrenzen und tragen Render-Nenner/Records unabhängig; Orphan-Render-Kandidaten zählen nicht. |

Zusätzlich: Duplicate-Evidence wird nach released Identity gezählt, nicht als mehrere Events. V1/V2-Vollständigkeit ist eine separate Governance-Pflicht, keine Umbenennung von A6.

Alle sechs Schulden müssen vor produktivem Enforcement geschlossen sein, A6 insbesondere vor Render-Enforcement. A4 muss vor erster ID-Streichung im accepted Parent geschlossen sein. Reiner physischer Retirement ohne ID-/Statusänderung kann vorher erfolgen.

## 10. V2 – Integrität, Scope und Proof-Verdikt

Released-Menge R ist domain-gebunden: R_master und R_render werden nicht anhand nackter numerischer Werte zusammengeworfen. Für jedes i∈R wird genau eine Bilanzklasse vergeben:

ReleasedOwnerEvents = MissingEvidence + DuplicateEvidenceIdentities + InvalidEvidenceIdentities + ClassifiedEvidence.

ClassifiedEvidence = OutOfScope + InvariantUnproven + InvariantDenied + NecessityUnproven + Permit.

E(i)=0 → Missing; E(i)>1 → Duplicate; E(i)=1 und ungültige/unlesbare Bindung → Invalid; sonst Classified. OrphanCandidateEvidence liegt außerhalb R. Mehrere Integritätsgründe dürfen diagnostisch erfasst werden, aber die Identity zählt nur einmal. ReleaseScopeMissing bleibt zusätzlich explizit und sperrt Readiness; ein Nenner 0 ersetzt keinen Scope-Nachweis.

Integrität umfasst originäre opaque Identity, domain, exakte Pre/Post-Zustände, Receipt-Fingerprints und eindeutige Bindung. Dekompressionsfehler zählt beispielsweise als InvalidEvidence mit Grund PreStateUnreadable. Bei Integrity!=Bound kein Proof-Verdikt.

Bei Bound: OutOfScope ist explizite Scope-Schuld; fehlende Invariante InvariantUnproven; gültiges konstruktives Gegenzeugnis InvariantDenied; bewiesene Invariante ohne Necessity NecessityUnproven; beide konstruktiv bewiesen Permit.

Vor JEDEM Proof-Verdikt müssen alle vorhandenen Proofs, Gegenzeugnisse und Necessity-Witnesses dieselbe Event-Bindung und zugehörige Family besitzen. Fremdes/stales Gegenzeugnis → InvalidEvidence, niemals InvariantDenied. Contradictory Proof+Counterexample → expliziter Integritätskonflikt, keine beliebige Auswahl. Unzulässige Kombinationen werden vollständig gezählt.

Scope wird aus dem originären Receipt/Descriptor abgeleitet, nicht vom Caller gewählt. Keine positiven/negativen Proofs aus WAV-Salience, Validator-/Audit-Zählern, Reporttext, NonLinkable-Feedback oder späterem XML als Origin-Ersatz.

V1-WouldPermit/WouldDeny/Unknown sind keine V2-Permits oder Gegenzeugnisse. V2 benötigt eine neue konstruktive Ausstellung mit vollständiger Bindung.

Bis zum Enforcement bleibt Shadow observation_only: Integritäts-/Proof-Schuld wird gezählt und sperrt Enforcement-Readiness/Final Exit, verändert aber keine produktive Auswahl, Writes oder Release-Entscheidung. Enforcement für eine spätere Family-Allowlist benötigt einen separaten Kodex-Parent. OutOfScope ist außerhalb einer solchen Allowlist verbleibende Exit-Schuld; weder Permit noch stillschweigende globale Durchlass-Erlaubnis.

## 11. Atomare Vertragsvarianten

r1242-Runtime bleibt CompleteV1. Der Parent muss später CompleteV1 oder CompleteV2 als gesamte gebundene Runtime-Form zulassen; keine Teilmischung. Beide Vertragsbündel dürfen als Governance-Dateien vorhanden sein, entscheidend ist die aktive Runtime-Form. E2 akzeptiert den vollständigen V1-Parent.

V2-Bündel: ssc_evidence_proof_contract_v2.txt, ssc_future_type_surface_v2.txt, ssc_observer_future_v2.txt, ssc_reference_firewall_v2.txt. Alle vier kompiliert gebunden.

Vor Bau müssen Surface und Observer als exakte Rust-Vorlagen vollständig ausgearbeitet sein, einschließlich OriginReceiptRef, Certificate/Counterexample-Typen, Lifetimes, privaten Konstruktoren, Scope-Ableitung und ungültiger Kombinationen. Pseudocode ist keine bereits freigegebene Runtime-Form. Das bisherige 5.6-Shape ist eine Ausgangsskizze, kein fertiges Observer-Template.

Firewall bindet konkrete Modulpfade und vollständige V2-Symbolmenge. Verdict-/Observation-Transport bleibt auf Proof-/Observation-Core und autorisierte Mess-/Reportsinks beschränkt. Kein Writer/Selector, Callback, bool-Accessor, Trait/Generic/Static-Transport oder Report-Reparse. Bei generischen Variantennamen wie Permit ist die konkrete Scanner-Wirkung auf den ganzen Parent vor dem Pinning zu prüfen; keine pauschale Namensöffnung.

## 12. old_root und Final Exit

old_root ist produktiv weiter benötigte nicht-souveräne Autorität aus den Baseline-Pflichten, WAV-Entscheidungsketten, Audit-/Validator-/NonLinkable-Rückkopplung, ungebundenen laufübergreifenden Artefakt-Reads, ungescannten Publikationen und verbliebenem Alias-/Grandfathering-Vertrag. Reine externe WAV-Eingabe oder reine Diagnose ist nicht allein deshalb old_root. Build-Cache ist kein Beleg einer musikalischen Alt-Abhängigkeit.

Baum-Prädikate des Gates: N_A leer, alle retired Prädikate persistent erfüllt, #55 legal READY und vollständig sichtbarer Publish-Pfad, #57 konstruktiv geschlossen unter späterem Fixvertrag, keine ungebundenen produktiven Persisted-Read-Pfade, keine Kodex-Altreste und keine erreichbare old_root-Authority innerhalb der belegten Prüfgrenzen.

Acceptance-Kriterien, NICHT als statische Gate-Messungen auszugeben: released Bilanz vollständig; Missing/Duplicate/Invalid/ReleaseScopeMissing=0; alle released Events unter konstruktivem Permit; OutOfScope/InvariantUnproven/InvariantDenied/NecessityUnproven=0; Kaltstart sowie unabhängige Regeneration vollständig geprüft.

Ein Exit-Take benötigt beide Nachweisarten. Eine textuelle exit_claim=closed-Zeile beweist keine Korpuswerte. Laufnachweise müssen an Candidate-Hash, Inputs, Outputs und Laufbedingungen gebunden sein. Solange deren konkrete Acceptance-Integration fehlt, kein automatischer Abschlussclaim.

Kaltstart: source-basierter Neubau und vollständiger Korpus ohne alte generierte Produktartefakte. Regeneration: zweiter unabhängiger frischer Lauf mit denselben gebundenen Inputs; MusicXML bytegleich, PDFs normalisiert bzw. pixelgleich, gleiche semantische Effekte. Prozesslokale opaque Identity-Zahlen müssen nicht zwischen Läufen identisch sein. Ein vorhandener Persisted-Read ist nicht ohne Reachability-Beleg als tatsächlich stale wirkender Fehler zu etikettieren.

## 13. Dateimenge und Admission-Vertrag

Geändert werden nur Paket-/Take-Deklaration, SSC/RSP/K-R-Kodex samt zugehörigen Tests, TCC und TA sowie erforderliche Take-/README-/Kodex-Dokumentation. Neue Dateien sind Governance-Vertrag und das vollständige V2-Bündel. Fehlerklassendokument für V2 hat SelectorChange: no, SelectorRole: error_class_definition und ParentErrorClassContract: none. V1-Dateien, alle TSV-Daten, Restmenge, Freeze, Scanner, Runtime und Cargo bleiben unverändert.

Jeder geänderte Paketpfad wird exakt deklariert. Eine notwendige Packaging-Lockdatei darf nicht undeclared werden; kein generiertes Root-Lock ohne fachliche Änderung. Zeilenbilanzpflicht. Keine Gate-Stubs, Lint-Suppressionen oder gelöschten Prüfungen ohne exakte Retirement-Ledger-Bindung.

## 14. Mindestprüfungen

- Parent-Admission und Candidate-Reexecution gegen den echten r1241.
- CompleteV1 akzeptiert; CompleteV2 exakt akzeptiert; gemischte/fehlende Vertragsformen abgewiesen.
- Lockerung einer Candidate-Vertragsdatei ohne neue Parent-Regel abgewiesen.
- Alle 15 IDs bekannt; Reopen, Duplikate und Streichung ohne Prädikat/A4 abgewiesen; Wiederkehr pensionierter Authority auch ohne Restmengenänderung abgewiesen.
- Population nur erlaubte Statusspalte; andere Spalten/Zeilen abgewiesen; #57 READY immer abgewiesen.
- E029/E031 alle vier Kombinationen; E032-Änderung und partielle Absenz abgewiesen.
- Witnesses einzeln absent; verbleibende Macro-Nutzer konsistent; acht produktive Owner unverändert.
- Guard-Löschung mit überlebendem Body, Nesting-/Reihenfolgeänderung, gelesene entfernte Bindung, Rename mit No-op-Body und neuer Ersatz-Writer abgewiesen.
- #55 ungeprüfter Fallback, fremde Destination, neue Surface/Budget ohne Vertrag abgewiesen.
- Fremdes Gegenzeugnis, stale Post, defekter komprimierter Pre-State und fehlende Records bleiben vollständig bilanziert; vor Enforcement kein produktives Veto.
- Compile, kanonisches Strict Clippy, Admission und vollständige Tests grün vor Paket-Ausgabe. Keine WIP-Ausgabe.

## 15. Reihenfolge und verbleibende Ausarbeitung

r1242 Governance → gegebenenfalls freigegebener Runtime-Retirement ohne Restmengenänderung → Runtime-Proof-Closure A1–A6 + CompleteV2 → Kodex-Status-/Restmengenänderung und späterer Enforcement-Vertrag → Enforcement-Runtime → Final Exit. Nummern nach r1242 sind nicht garantiert.

WAV Accent, Salience-Adoptionen und Metronom werden ohne gesonderte Produktentscheidung nicht musikalisch pensioniert. Keine solche Entscheidung für den Bau von r1242 nötig.

#57 benötigt einen separaten Read-only-Beweis und später einen benannten Kodex-Fixvertrag. Die vorgeschlagene Bedingung B.old_end<=A.insert_at wird nicht vorweg als notwendige oder endgültige Invariante gepinnt. Ziel ist korrekte Materialisierung aller bereits autorisierten Moves in ursprünglichen Source-Koordinaten; ein neuer Ausschluss sicherer Moves wäre auf Selector-/Musikwirkung zu prüfen.

Noch vor einem r1242-Paket auszuarbeiten und gegenzuprüfen: exakte Rust-V2-Surface/Observer/Firewall; blockbezogener Retirement-Prüfer mit belegten Scan-Grenzen; genaue maschinenprüfbare Parent-A4-Form. #55 darf ausschließlich die eingeschränkte Route aus Abschnitt 8 erlauben, keine erfundene allgemeine Baseline-Freigabe. Diese Ausarbeitung ist keine bereits bestandene Admission.

## 16. Getrennte Folgeaufträge

### Sonnet – gezielter Read-only-Review

Prüfe ausschließlich diese Konsolidierung gegen accepted r1241. Kein Source, kein Take. Ergebnis pro Regel PASS/BLOCKER mit Source-Beleg und kleinstem Gegenbeispiel. Besonders: dauerhafte 15-ID-Prädikate, compiled Bindung auf E1/E2, Guard-Strukturanalyse, V2-Gegenzeugnisbindung, #55 unter unverändertem Inventar, Parent-A4-Erkennung und atomare Runtime-Version. Trenne r1242-Bausperren von späteren Acceptance-Pflichten. Kein Review einer früheren Fassung als PASS für diese Datei verwenden.

### 5.6 – begrenzte konstruktive Ausarbeitung

Keine neue Gesamtneufassung. Kein Source, kein Take. Liefere exakt drei Anlagen zu dieser Konsolidierung: (1) vollständig formulierbare Rust-Type-Surface und Observer-Vorlage samt Gegenzeugnis-/Family-/Identity-Bindung und totaler Fehlerbilanz; (2) mechanische Prüfstrategie für syntaktisch geschlossene Retirement-Bereiche inklusive adversarieller Guard-Fälle; (3) Source-belegte #55-Route innerhalb bestehender Sinks/Budgets oder präziser Nachweis, welcher neue Kodexvertrag dafür zusätzlich nötig wäre. Keine A1–A6-Umbenennung, keine WAV-Produktentscheidung und keine #57-READY-Freigabe.

6.1 konsolidiert diese Anlagen und implementiert allein. Erst die tatsächlich grünen Prüfungen rechtfertigen eine Paket-Ausgabe.

## r1242-Implementierungspräzisierungen

Die Predicate-Scans betrachten Rust-Code unter src/tests/tools mit explizit ausgenommenen kompiliert gebundenen Kodex-Prüfdateien. Freeze bleibt für den Alias-Rohtextscan eingeschlossen. Historische TSV-/TXT-Registries sind keine ausführbaren Alias-Nutzungen.

K2 bindet Prüfcode, nicht nur Dokumente. Eine explizit benannte SscK2GovernanceRevisionKodex-Revision ist ein vertrauenswürdiger Kodex-Eingriff und wird als solcher sichtbar; normale Takes dürfen die gebundenen Prüfer nicht ändern. Dies ist keine Behauptung, beliebige künftige Governance-Revisionen mathematisch gegen den eigenen Code prüfen zu können.

Retirement-Erasure erlaubt konservative WholeFunction- und Statement-Nodes und weist entfernte Fehlerausgänge ab. Komplexe nicht unterstützte AST-/Macro-/Importformen benötigen einen späteren benannten Governance-Vertrag; sie werden nicht optimistisch angenommen. Der aktuelle WAV-/RA5-Schnitt mit Fehlerausgängen erhält dadurch keine vorweggenommene Zulassung.

#55 wird auf die vollständige bestehende Publikationsfunktion und die gesamte Pipeline-Datei gebunden. Eine S55-A-Future-Fixture ersetzt den rohen Rename/Copy-Publish durch denselben Authority-Sink auf artifact_path, nach TMP-Validation. Die tatsächliche spätere Admission, Byte-Neutralität und I/O-Fehlerwirkung bleiben zu messen.

V2-Templates liegen ausschließlich als gebundene Governance-Dateien vor. Sie ändern heute keine Runtime. V2→V1 ist gesperrt. A4 wird wörtlich einschließlich Schema-Literalen geprüft, nicht nur über maskierten Code.
