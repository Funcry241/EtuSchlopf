# GlobalHorizontalPageCountPreferenceFrontierEvidencePersistenceActivation — v0.10.1839 r07

r06 committed the behavior-neutral transport prewire through an exit-0 Etüde-6 install run.
r07 changes only the audit adapter from disabled to enabled.

The already computed `HorizontalLayoutGlobalPageCountPreferenceShadow` line is now captured
read-only and projected through the pre-existing result paths that r06 prewired. No planner,
reflow, page-count choice, system/page break decision, width decision, MusicXML/PDF mutation,
safety gate, security gate, writer family, materializer, or corpus success criterion changes.

The projection contract remains closed:
- zero matching lines => no frontier section,
- exactly one matching line => persist one read-only evidence section,
- more than one matching line => fail closed,
- `PolicyWrite nein`,
- batch-child stdout is consumed only for the typed frontier line and is not promoted to a
  general second console/report truth.

ChangeClass is `OutcomeProjection`. This revision is intended to be installed with a known-green
project (Etüde Nr. 6) so the take transaction commits. Only after that commit should `run.cmd --all`
be used without `--take` to collect the intentionally mixed 3/10-green corpus frontier evidence.
