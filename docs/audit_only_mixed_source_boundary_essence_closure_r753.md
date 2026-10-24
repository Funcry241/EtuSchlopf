# r753 - AuditOnlyMixedSourceBoundaryEssenceClosure

Parent: accepted r751. Rejected r752 is evidence only.

## Trigger

r752 attempted to derive the frozen-sounding temporal lane audit directly from private `CandidateMaterialLineageSpine` state by adding a new read-only function to `src/pipeline_shards/candidate_material_lineage_spine_core.rs`. The Take-Change-Control rejected the candidate before installation because an `AuditOnly` change may not add new logic to that mixed productive source.

## Repository-resident lesson

Audit intent does not authorize a new data edge in productive code. New AuditOnly implementation must remain entirely within paths classified as `PathRole::Audit`. If required evidence exists only as private state in a mixed productive source, an AuditOnly take may consume it only through an already-authorized audit-visible transport. Creating a new helper, call-site, field, callback or report bridge in the mixed source is itself an architecture change and must be classified explicitly rather than hidden inside AuditOnly.

## Consequence for the current SourceDuration work

The 1,274-attack frozen material state cannot be exposed by repeating the r752 pattern. The accepted r751 report already proves the 1,312 -> 1,274 deletion chain and the existence of later representation changes, but it does not publish complete final attack rows sufficient to reconstruct the exact frozen lane plan externally. The next temporal-authority step must therefore first identify an already-authorized audit-visible transport or explicitly design the missing final-state transport/authority boundary with an appropriate change class.

## Non-effects

No product code changes. No music mutation. No writer, repair, selection, duration, tie, voice, carrier, renderer or release authority changes. E00-E31 remain the complete numbered essence rule set; this closure strengthens the operational reading of E31 and the pre-candidate checklist.
