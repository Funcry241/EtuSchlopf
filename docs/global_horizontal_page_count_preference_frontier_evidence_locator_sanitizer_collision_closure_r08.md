# GlobalHorizontalPageCountPreferenceFrontierEvidenceLocatorSanitizerCollisionClosure — v0.10.1839 r08

The committed r07 activation proved that the page-count preference frontier is persisted through
the established result path. Its normal FinalReport projection also exposed a representation-only
collision: the generic locator sanitizer interprets the substring `Page ` inside the field names
`VerticalMaxSystemsPerPage ` and `ClosestLowerSystemsPerPage ` as a page locator and therefore
redacts the following numeric evidence to `<locator>`.

r08 does not weaken or modify that sanitizer. It changes only the audit projection of the already
captured frontier line so those two persisted field separators use `=`:
`VerticalMaxSystemsPerPage=...` and `ClosestLowerSystemsPerPage=...`.

The original console evidence remains byte-for-byte unchanged. The frontier calculation, accepted
system capacity, page-count feasibility, planners, reflow, safety gates, security gates, writer
graph, result paths, MusicXML, PDF, page/system breaks, widths, corpus success criteria, and all
productive decisions remain unchanged. This is an OutcomeProjection-only representation closure.

Install with a known-green project so the same-version take transaction commits:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

After the successful commit, run the full corpus without `--take`:

```powershell
.\run.cmd --all
```
