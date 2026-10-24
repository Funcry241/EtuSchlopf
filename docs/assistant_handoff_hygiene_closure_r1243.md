# r1243 AssistantHandoffHygieneClosure

Parent: accepted r1242. ChangeClass: OutcomeProjection.

The assistant handoff uses verified exact snapshot-entry references for all non-deleted declared changes. Snapshot completeness, archive SHA-256 and exact entry bytes must agree; otherwise the existing whole-file-or-explicit-omission fallback remains. Deleted entries are explicit. Full source, logs, results, catalog and transition evidence are preserved.

The generation-time state is advisory and cannot assert acceptance. It binds the existing transition projection SHA-256 and distinguishes an in-flight transaction, matching revision without byte verification, nonmatching identity, and missing/invalid/issue-bearing projection. Existing post-acceptance finalization stays unchanged. A plain run without transition projection reports indeterminate rather than inventing a canonical head.

Console projection excludes only the anchored progress-bar lines. All other lines remain verbatim within a 12 KiB head/tail budget; omitted middle content and progress count are explicit. The original full log SHA-256 is retained. The existing 768 KiB handoff hard limit and mandatory cold-start bodies remain unchanged. Representative normal corpus projection target: at most 96 KiB; fallback for unavailable snapshot is not subject to this target.

SSC remaining-set projection is explicitly partial: A1-A6 and other proof debts are not inferred from this file. No open ID is removed. No claim of SSC completion is added.

Catalog, snapshot, handoff staging and outer ZIP elapsed milliseconds are diagnostic only. Timing failures cannot affect admission or release. Timers are written to existing console logs only; handoff staging and outer ZIP measurements complete after the console hash is bound and are therefore local rather than retroactively inserted into that ZIP. Deterministic handoff generation does not include its own elapsed duration.

Runtime performance is not promised: this Take primarily reduces assistant read load. No shared validation cache, Solver, Writer, concurrency, bootstrap admission or governance-semantic change.
