# Authority Retirement Contract Template

`authority_retirement_v1` ist kein Selector- oder Error-Class-Bypass. Der Vertrag gilt nur fuer einen bereits im akzeptierten Parent vorhandenen produktiven Writer-Surface, dessen Authority in einem spaeteren `MusicMutation`-Take streng monoton sinkt.

Konkreter Pfad eines Implementierungsvertrags:

```text
docs/authority_retirement_contract_<surface-name>.md
```

Der Take muss denselben Pfad zusaetzlich im Take-Change-Manifest binden:

```text
authority_retirement_contract=docs/authority_retirement_contract_<surface-name>.md
```

Pflichtfelder:

```text
AuthorityRetirementContract: authority_retirement_v1
AuthorityChange: retirement_only
ParentAuthoritySurface: src/<path>.rs::<existing_parent_function>
RetirementScopePaths: src/<path>.rs[,src/<support-path>.rs]
CandidateClassChange: forbidden
SelectorRelationChange: forbidden
TargetSelectionChange: forbidden
MultiplicityDecisionChange: forbidden
NewPositiveWriteDecision: forbidden
ReplacementWriter: forbidden
WriterCapabilityDirection: decrease_only
CalibrationThresholdChange: forbidden
ProductDeltaScope: removal_of_parent_writes_only
CausalClassFirstBypass: forbidden
CandidateDuplicationBypass: forbidden
```

Fail-closed Bedeutung:

- `ParentAuthoritySurface` muss im installierten Parent existieren.
- `RetirementScopePaths` muss exakt dem Satz der geaenderten nicht-Kodex-Rust-Sourcepfade entsprechen.
- Der Semantic-Capability-Diff muss einen reinen monotonen Verlust produktiver Writer-Authority beweisen: Writer-Capabilities duerfen nur verschwinden oder ihre Writer-Reachability darf sich strikt verengen; neue oder anderweitig modifizierte Writer-Capabilities sind unzulaessig.
- `CalibrationThreshold` darf in demselben Take keinerlei Delta besitzen.
- Candidate-/Selector-/Multiplicity-Entscheidungsrelationen duerfen weder durch Hinzufuegen noch durch Entfernen entsprechender Codezeilen veraendert werden.
- Der normale `causal_class_first_v1`-/`candidate_duplication_v1`-Gate bleibt aktiv. Ein Retirement-Contract kann ihn nicht ueberschreiben.

Damit ist Retirement bewusst enger als eine normale MusicMutation: zuerst wird ausschliesslich die illegitime Schreibauthority entfernt. Selector-/Threshold-Cleanup folgt, falls ueberhaupt gerechtfertigt, in getrennten Takes.
