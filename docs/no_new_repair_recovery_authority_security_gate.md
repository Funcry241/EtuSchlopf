# v0.10.1790 – NoNewRepairRecoveryAuthoritySecurityGate

## Ziel

Produktive neue oder reaktivierte Repair-/Recovery-Autoritaeten duerfen nicht mehr durch eine normale MusicMutation in den produktiven Writer-Graph gelangen. Das Gate erzwingt damit die bestehende Ownership-Architektur: ein neuer Befund wird am bestehenden fachlichen Owner, dessen DecisionScope oder dessen Preconditions geschlossen statt durch einen spaeten Reparatur-Writer kompensiert.

## Security-Entscheidung

`Semantic-Capability-Diff` vergleicht die Writer-Reachability des installierten und des eingehenden Baums. Entsteht fuer eine Music-Funktion erstmals produktive Writer-Reachability, lehnt der Build fail-closed ab, wenn entweder Funktion/Quelldatei eine `repair`- oder `recovery`-Identitaet traegt **oder** der neue Writer-Aufrufer unter `src/` ausserhalb von `src/structural_write_authority/` liegt. Damit laesst sich eine neue spaete Reparaturschicht nicht durch Umbenennen in `normalize`, `polish` o. ae. tarnen. Neue kanonische Schreibautoritaet muss an der bestehenden StructuralWriteAuthority-Grenze entstehen. Bereits vorhandene historische Authorities werden dadurch nicht pauschal umklassifiziert; read-only Beobachtungs-/Auditfunktionen ohne produktive Writer-Reachability bleiben zulaessig.

## Grenzen

Das Gate ist absichtlich keine musikalische Heuristik und erzeugt keinen Writer. Es erkennt keine semantisch verschleierte Reparatur anhand bloßer Wirkung; weiterhin schuetzen Productive-Write-Path, WriterReachability, StructuralWriteAuthority und die bestehenden Capability-Gates die produktiven Senken. Der neue Vertrag schliesst die explizite Repair-/Recovery-Familie zusaetzlich als Security-Negativregel.

## Regression

Die Scanner-Tests enthalten einen negativen Fall fuer eine neu writer-erreichbare `late_clef_repair`-Funktion, einen zweiten negativen Fall fuer eine umbenannte spaete `late_clef_normalizer`-Schicht ausserhalb der StructuralWriteAuthority, einen positiven Fall fuer einen neuen kanonischen StructuralWriteAuthority-Owner sowie einen positiven Fall fuer eine read-only `historical_repair_observation`.

## Adversarial Matrix

Die gepinnte Security-Matrix fuehrt die neue Angriffsklasse `new_productive_repair_authority -> no_new_repair_authority -> blocked`. Der Build-Owner verlangt sowohl den produktiven Validator-Aufruf als auch den negativen Scanner-Test und den fail-closed Gate-Marker im gepinnten Semantic-Capability-Diff.
