# WholeRest Candidate Rewrite Authority Vocabulary Closure — r251

## ChangeClass

`RefactorOnly`

## Parent

`0.10.1839 r250 MusicXml40PublishedSchemaReleaseProvenanceClosure`

## ε-native finding

The private `UpperStaffShadowRestVoice1KeepWithoutShadowLaneFixAudit` used the field name `write_allowed`, and the diagnostic projection printed `WriteAllowed ja`. In this function that boolean is enabled before the candidate has passed the later sounding, parse, backup/forward, raw-cursor and visibility invariants. The actual final adoption is represented separately by `written`.

The old identifier therefore described a broader authority than the value actually owned. It did not create a writer bypass, but it made a local candidate-path condition look like final write authorization.

## Closure

Exactly one private field family is renamed:

- `write_allowed` -> `candidate_rewrite_path_enabled`

The associated diagnostic wording changes from:

- `WriteAllowed ja`

to:

- `CandidateRewritePathEnabled yes`

The new name states the real local scope: the candidate rewrite path is enabled, while final adoption still requires the pre-existing invariant conjunction and is represented by `written`.

## Non-change proof contract

This take does not change:

- candidate classification,
- replacement construction,
- StructuralWriteAuthority invocation,
- sounding invariants,
- XML parse checks,
- backup/forward checks,
- raw-cursor checks,
- visibility invariants,
- the `written` decision,
- returned XML,
- Master MusicXML,
- Render MusicXML,
- PDF selection,
- music or notation rules,
- security gates,
- dependencies or toolchain.

No generic authority abstraction, shared runtime, memory/continuity concept, new repair authority or new truth owner is introduced.

## Expected real-system proof

The authoritative proof remains the normal transactional command:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

The expected semantic result is same-version r251 with unchanged productive music behavior and the narrowed diagnostic vocabulary.
