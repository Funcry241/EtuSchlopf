# GlobalHorizontalPageCountPreferenceFrontierCurrentProcessProjectionSanitizerClosure — v0.10.1839 r09

r08 was committed successfully and confirmed that the original console evidence remains correct, but the
normal green FinalReport still persisted `VerticalMaxSystemsPerPage <locator>` and
`ClosestLowerSystemsPerPage <locator>..4`. The cause is narrower than the generic sanitizer itself:
`append_section()` already applies the projection-safe `=` separators, while the normal FinalReport owner
reads the captured line through `current_process_line()` and projects that return value directly.

r09 changes only the audit adapter so `current_process_line()` returns the same already-defined
projection-safe representation before the unchanged FinalReport sanitizer runs. The underlying console
capture is not rewritten, the batch-child transport is unchanged, and `append_section()` remains unchanged.
The result projection therefore retains the numeric `VerticalMaxSystemsPerPage` and
`ClosestLowerSystemsPerPage` evidence without weakening any locator masking rule.

No page-count preference is promoted. Frontier computation, planner/reflow, safety/security gates, writer
graph, MusicXML/PDF bytes, page/system/width decisions, result ownership and corpus success criteria remain
unchanged. This is an OutcomeProjection-only closure.

Install with the known-green project so the same-version transaction commits:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

After a green commit, verify the two persisted fields and then run the corpus without `--take`:

```powershell
.\run.cmd --all
```
