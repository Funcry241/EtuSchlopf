# r93 – Manifest-Driven Corpus Identity Decoupling Test-Fixture Recovery

## Parent

`0.10.1839 r90` `IdentityLocatorCleanupAndCorpusSecurityDecouplingRecovery`.

## Recovery reason

The rejected r91 candidate reached Clippy and Build successfully but failed the integrated Rust tests in both binary targets. The new three-project regression test constructed an intentionally shortened result string that did not satisfy the already-existing complete renderer/writer evidence contract. Because `corpus_run_manifest.rs` is compiled into both binaries, the same faulty test failed both `etuschlopf` and `etuschlopf_launcher`.

r93 does not weaken that evidence contract. The regression test now uses the module's existing canonical `result_text(...)` / `WriterFixture` helper with a zero-write, fully observed outcome. The production corpus decoupling is otherwise the same intended change:

- `FullCorpus` is the exact non-empty project set declared by the run manifest rather than literal ten projects;
- corpus emergence compares declared and evidenced project identities rather than parsing Etude numbers 1..10 from names;
- standalone reports without an authoritative expected-project set cannot claim `FullCorpus`;
- the historical exact `Takt 2/Takt 10 priorisiert` marker is recognized generically as a retired measure-priority line;
- concrete identity fixtures used as regression evidence remain tests only.

No music mutation, writer behavior, calibration or Security policy change.

ChangeClass: `RefactorOnly`.
