# Optional PDF proof lifecycle: causal error-class definition

Stage A against natively accepted r1530. Native acceptance remains pending.
No source selection, runtime implementation, musical write or gate change.

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: a PDF commit is created only when PDF rendering is requested and eligible, but a later continuation demands that commit from source selection alone
CausalFeature: choosing the final authorized XML representation and requesting a PDF are independent facts; retaining Render as the final selected XML does not create a PDF request or a PDF commit
CausalFeature: a continuation of an optional proof must use the same prerequisite as its creation; absence during a non-requested or blocked renderer path is distinct from absence during an eligible requested renderer path
NonCausalContextExcluded: filename, project identity, corpus membership, measure, pitch, staff, voice, note count, note duration, density, timing profile and elapsed time

## Necessity and owner

The missing dependency is the effective PDF-request/eligibility context already
computed in Metric/FinalNotation and explicitly carried into the release owner.
The defect is not an incorrect choice between Master and Render, missing
mandatory XML, or an authorization to fall back to a different musical source.

A later repair may narrow PDF-proof continuation to the exact existing creation
precondition. It must preserve source selection, authorized layout adoption,
final XML snapshot binding, persisted-byte checks, product closure and reports.
When PDF rendering is eligible and requested, a missing or invalid committed
PDF source must still fail closed. When rendering is blocked, no PDF is
invented and the pre-existing blocking reason must remain visible.

## Independent neighbors and validation

- Master or Render selected; PDF not requested: preserve the selected XML and
  both mandatory XML artifacts; do not demand or create a PDF commitment.
- Render selected; PDF requested and eligible: preserve the strict existing
  commitment requirement and the authorized proof continuation.
- Render selected; rendering blocked before the renderer: do not consume an
  absent optional PDF proof; preserve every blocker and final non-OK outcome.
- Missing/stale/invalid mandatory XML: reject independently of PDF options.
- Equivalent local requests renamed, moved or duplicated: identical local
  decision; no global first request, winner or cardinality condition.

Normative fixtures must be synthetic, with actual Master and Render selection
paths. Test the source label, both XML files, renderer outcome, exit status,
unchanged original inputs and repeated-run determinism, not merely exit 0.
A requested PDF probe must retain the commitment path; a successful PDF export
requires a real supported renderer and is a separate boundary.

## Exploratory evidence known before this definition

The accepted r1530 engine fails the independent synthetic Render/no-PDF probe
at the E007 proof-continuation requirement. A local one-condition experiment
narrowing that continuation to pdf_render_requested && render_xml_selected_for_pdf
passes both Master/no-PDF and Render/no-PDF, including repetitions, without
changing the selected source. This is pre-freeze exploration, not a post-freeze
class-distribution claim or native acceptance.
