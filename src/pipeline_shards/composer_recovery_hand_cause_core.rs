// v0.10.1702 r02 CanonicalMetricBeamObservationBoundary cross-file interface closure.
// PartialBeatMeasureSpan/PartialBeatRawNote/PartialBeatNoteInfo are now owned by
// composer_metric_beam_observation_core; this consumer keeps identical semantics.

type SisterFeedbackVariantACandidateGroup<'a> = (i64, String, String, Vec<&'a PartialBeatNoteInfo>, bool, bool);









fn yes_no(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}






