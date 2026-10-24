# r1396 — Freeze three independent error classes and the closure build brief

Parent: accepted r1395, SHA-256
48e2bc8d70b3f840f6e8ab0a49ebae93038f5041186eefde2c0fdba21a9b8e10.
ChangeClass: KodexOnly. No Rust, no Cargo metadata, no renderer command, no
runtime behavior and no musical output change.

## Purpose

Selector-sensitive implementation code may only cite an error-class contract
that already exists in an accepted parent, and a definition must not be created
in the same take as its implementation. Three pending implementation tasks had
no such parent class. This take freezes them in advance so that each later
implementation needs exactly one take:

| New definition | Later implementation |
| --- | --- |
| `selector_derivation_contract_forced_renderer_fallback_precedence.md` | forced variants never reordered ahead of unforced ones; forced/unforced evidence per conversion |
| `selector_derivation_contract_runtime_worker_memory_capacity.md` | measured memory bounds the automatic default worker count |
| `selector_derivation_contract_octave_display_boundary_incoming_tie.md` | transfer of the octave/incoming-tie boundary safety of rejected r1362 |

The partwide renderer channel-capacity class (r1326) already exists; its repair
needs no new definition.

## Evidence summary (diagnostic only)

- r1394 and r1395 corpus exports: 10/10, FinalExit 0; nine PDFs use
  `-o <pdf> <input>`, one uses `-f -o <pdf> <input>` with
  `VariantsAttempted 1/11; FailedBeforeSuccess 0`. In the source the forced
  variant is entry 8 of 11; only the winner-reuse caches can move it first.
- Native MuseScore 4.7.4 Linux: that project's final render XML fails unforced
  with exit 40; a control project succeeds.
- Ledger: r1362, r1363 and r1365 were rejected at `reexec_exit` (exit 1,
  transactional rollback), before the later bootstrap stabilization takes.

None of these observations is a class feature. The definitions are derived from
converter semantics, resource arithmetic and import pairing semantics.

## Advisory brief

`docs/assistant_memory/042_bauauftrag_abschluss_unforced_pdf.md` records the
closure order and acceptance criteria. `000_start_here.md` points to it.

## Candidate validation

The complete candidate tree equals the r1395 source snapshot except for the
declared files and the change manifest. The root Cargo.lock is excluded, nested
locks are unchanged. The unchanged parent
`take_change_control::validate_incoming_take` accepts the candidate locally as
KodexOnly. Windows take acceptance remains pending.

    ./run.cmd --take --all
