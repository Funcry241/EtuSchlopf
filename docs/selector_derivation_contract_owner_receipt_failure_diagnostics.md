# Owner receipt failure diagnostics: causal error-class definition

Stage A of causal_class_first_v1. This definition follows from the existing sealed
receipt and sovereign adoption contracts. It introduces no classification code,
receipt mint, writer, candidate decision, repair, proof, permission or release rule.

## Causal classes

1. **Changed exact-owner transition without a certifiable exact receipt.** An
   existing owner has a bound pre-state and a changed post-state, but its existing
   exact-receipt verifier refuses certification because the descriptor does not
   carry the exact contract or the origin-bound edit script does not reproduce
   the post-state byte for byte. An unchanged pre/post pair is not this failure.
2. **Discontinuous supplied receipt sequence.** Beginning at the existing adopted
   state's head, at least one supplied receipt has a pre-state fingerprint that
   differs from the current head obtained from the preceding receipt. Existing
   order and continuity checks remain the sole adoption authority.
3. **Uncovered end of a supplied receipt sequence.** Starting from the bound
   pre-state and following the supplied receipts, the final receipt post-state
   fingerprint differs from the document offered for adoption. With no supplied
   receipts, the bound pre-state itself is the sequence's end. An unchanged
   document with no receipts is not this failure.

These classes classify mechanical failure evidence. They do not establish which
musical transformation should be selected, whether a different candidate may be
tried, or whether a change should be adopted. A refusal outside an adoption path
does not by itself establish a failed project or release. A continuous covered
sequence does not establish fresh identity, valid scope, a successful SSC proof
or an adoption permit; those dimensions retain their separate existing checks.

## Observation boundary

A later implementation may project already available owner descriptors, edit
spans, receipt counts, continuity positions, document lengths and fingerprints
as error context. Caller/patch labels may identify an observation, but cannot
enter class membership, selection, repair, ranking or authorization.

Observation must neither consume nor alter receipts, mint a replacement, repair
a chain, change the existing refusal or fatal result, retry another candidate,
or provide input to a productive decision. No class depends on the affected
piece, physical measure number, particular note, execution schedule, number of
independent candidates or the frequency of failures in a corpus.

```text
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
CausalFeature: an existing exact owner attempts certification of a changed bound transition and its existing descriptor or origin-bound byte edit script fails the existing exact-receipt verifier
CausalFeature: a supplied receipt pre-state fingerprint differs from the existing transition head obtained from the bound pre-state and preceding supplied receipts
CausalFeature: the end fingerprint of the supplied receipt sequence differs from the actual document offered for adoption, with the bound pre-state serving as the end when the sequence is empty
NonCausalContextExcluded: project, etude, measure number, pitch, staff, voice label, caller label, patch name, iteration, owner identity as a membership selector, corpus distribution, candidate multiplicity, warning counts, worker count, execution timing and ambient inputs
```

No corpus distribution is measured by this definition take. The classes derive
from the pre-existing verifier and adoption invariants and remain meaningful for
synthetic documents and previously unseen projects. A known runtime failure may
motivate evaluation after this definition is accepted, but supplies no predicate
or normative test fixture.
