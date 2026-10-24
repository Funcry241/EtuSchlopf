# r1181 WholeRestDisplayCompleteUnionCoveragePreservationEnforcement

Parent: accepted/released v0.10.1839 r1180 `WholeRestDisplaySelectedCandidateIntervalWitnessRefactor`.

## Evidence carried by r1180

The accepted witness transport proves that the late early-render `WholeRestDisplayCoordinateReadbackTruthFix` selected a visible rest with duration 96 while its actual other-voice union coverage was only 72. The same row had no same-voice pitched overlap and was nevertheless materialized as a full 96-duration forward.

The witness also shows a separate fully covered 24-duration row. This gives the repair a natural positive/negative boundary without deriving any feature from Etude or measure identity.

## Repair

r1181 narrows only the existing unbound whole-rest selector. The old predicate accepted at least 50 percent *summed* pitched overlap. The repaired predicate requires the already-computed other-voice *union* coverage to span the complete rest duration before this owner may replace the complete visible carrier by a forward.

No partial interval materializer is added here. Incomplete carriers are preserved at this early stage. The later canonical FinalChord interval projection already owns the bound split of occupied versus silent subintervals after final voice binding.

The frozen-voice bound selector is not changed by this take.

## Regression boundary

Existing test function identities are retained. The fully covered synthetic neighbor remains positive. The existing negative overlap test additionally covers a majority-but-incomplete interval. The existing overlap-multiplicity metric test additionally verifies that simultaneous events cannot turn partial union coverage into whole-rest eligibility.

No new direct writer caller is introduced.

## Expected product consequence

For a carrier whose union occupancy is a strict subset of its duration, this stage must report no whole-rest suppression for that carrier. A later bound projection may still split or retire it when its own authority proves the exact intervals. The corpus, not the repair selector, determines how many current rows change.
