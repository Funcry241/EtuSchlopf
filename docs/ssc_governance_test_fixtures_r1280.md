# r1280: Eingefrorene Test-Fixtures für die SSC-Governance

Benannte `KodexOnly`-Revision `SscK2GovernanceRevisionKodex` auf accepted r1279. Laufzeit, Musik, SSC-Bilanz und alle Governance-Regeln bleiben unverändert.

**Befund (eigener Konstruktionsfehler aus r1277/r1278):** Einige vom Parent kompilierte Tests in `ssc_k2_retirement_governance_tests.rs` und `ssc_governed_change_tests.rs` lasen die laufenden Dateien:

- die vier Beweisdateien;
- `master_state_identity.rs` und `final_report.rs`;
- die Envelope-Tabelle, die Surface-Erweiterungen und das Wiring.

Ein `GovernedChange`-Take muss genau diese Dateien ändern (über `proof_rule` und `envelope_extend`), darf Kodex-Tests aber nie ändern. Der lokal gebaute Take 2 ließ dadurch 7 Tests scheitern. Auf Windows wäre er abgelehnt worden.

**Änderung:**

- Die Tests lesen jetzt eingefrorene Fixtures mit dem Stand r1279 (`src/build_gate/ssc_test_fixture_r1279_*.txt`) und leere Kopfzeilen der verwalteten Datendateien.
- Die Fixtures stehen in der K2-CODE-Liste. Sie können sich also nur in einer benannten Governance-Revision ändern.
- Tests, die bewusst den laufenden Stand prüfen, lesen weiter die echten Dateien. Das sind die #57-Item-Pins und der installierte #57-Writer.

**Wirkung:** Die Tests prüfen weiterhin dieselben Regeln mit denselben Gegenbeispielen. Das Echt-Gate im Build prüft den laufenden Baum unverändert.

**Lehre:** Gate-Tests nutzen eingefrorene Fixtures, nie Dateien, die ein Folge-Take legitim ändern darf.
