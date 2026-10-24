use super::{
    StructuralBeamState, StructuralCarrierRole, StructuralEvent, StructuralRepairIntent,
    StructuralStaffRelation, StructuralView, StructuralVoiceRelation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum IdentityFreeTailBeamState {
    Beat3EmptyBeat4Open,
    LegacyCrossBeatBeat4Open,
    Beat3CorrectBeat4Open,
    AlreadyCorrect,
    Unsupported,
}

impl IdentityFreeTailBeamState {
    pub(super) fn is_repairable(self) -> bool {
        matches!(
            self,
            Self::Beat3EmptyBeat4Open
                | Self::LegacyCrossBeatBeat4Open
                | Self::Beat3CorrectBeat4Open
        )
    }
}

pub(super) fn classify_tail_beam_state(
    view: &StructuralView,
) -> Option<IdentityFreeTailBeamState> {
    let quarter = i64::from(view.metric_units_per_quarter());
    let sixteenth = quarter / 4;
    if quarter <= 0 || quarter % 4 != 0 {
        return None;
    }

    let event = |onset: i64,
                 duration: i64,
                 role: StructuralCarrierRole,
                 relation: StructuralStaffRelation,
                 visible: bool,
                 rest_like: bool,
                 chord_member: bool|
     -> Option<&StructuralEvent> {
        let matches = view
            .events()
            .iter()
            .filter(|candidate| {
                i64::from(candidate.onset_in_window) == onset
                    && i64::from(candidate.duration) == duration
                    && candidate.carrier_role == role
                    && candidate.staff_relation == relation
                    && candidate.voice_relation == StructuralVoiceRelation::SameRun
                    && candidate.visible == visible
                    && candidate.rest_like == rest_like
                    && candidate.chord_member == chord_member
            })
            .collect::<Vec<_>>();
        if matches.len() == 1 {
            Some(matches[0])
        } else {
            None
        }
    };

    let beat3_first = event(
        0,
        sixteenth * 3,
        StructuralCarrierRole::Sounding,
        StructuralStaffRelation::SameAsAnchor,
        true,
        false,
        false,
    )?;
    let beat3_last = event(
        sixteenth * 3,
        sixteenth,
        StructuralCarrierRole::Sounding,
        StructuralStaffRelation::SameAsAnchor,
        true,
        false,
        false,
    )?;
    let beat4_first = event(
        quarter,
        sixteenth * 2,
        StructuralCarrierRole::Sounding,
        StructuralStaffRelation::SameAsAnchor,
        true,
        false,
        false,
    )?;
    let beat4_cross = event(
        quarter + sixteenth * 2,
        sixteenth,
        StructuralCarrierRole::Sounding,
        StructuralStaffRelation::DifferentFromAnchor,
        true,
        false,
        false,
    )?;
    let beat4_last = event(
        quarter + sixteenth * 3,
        sixteenth,
        StructuralCarrierRole::Sounding,
        StructuralStaffRelation::SameAsAnchor,
        true,
        false,
        true,
    )?;
    event(
        quarter + sixteenth * 3,
        sixteenth,
        StructuralCarrierRole::HiddenCarrier,
        StructuralStaffRelation::SameAsAnchor,
        false,
        true,
        false,
    )?;
    event(
        quarter + sixteenth * 2,
        sixteenth * 2,
        StructuralCarrierRole::Rest,
        StructuralStaffRelation::SameAsAnchor,
        true,
        true,
        false,
    )?;

    if view.events().len() != 7 {
        return None;
    }

    let exact =
        |event: &StructuralEvent, primary: StructuralBeamState, secondary: StructuralBeamState| {
            event.primary_beam == primary && event.secondary_beam == secondary
        };
    let absent = StructuralBeamState::Absent;
    let beat3_empty = exact(beat3_first, absent, absent) && exact(beat3_last, absent, absent);
    let beat3_correct = exact(beat3_first, StructuralBeamState::Begin, absent)
        && exact(beat3_last, StructuralBeamState::End, absent);
    let legacy_cross_beat = exact(beat3_first, absent, absent)
        && exact(beat3_last, StructuralBeamState::Begin, absent)
        && exact(beat4_first, StructuralBeamState::End, absent);
    let beat4_open = exact(beat4_first, absent, absent)
        && exact(beat4_cross, absent, absent)
        && exact(beat4_last, absent, absent);
    let beat4_correct = exact(beat4_first, StructuralBeamState::Begin, absent)
        && exact(
            beat4_cross,
            StructuralBeamState::Continue,
            StructuralBeamState::Begin,
        )
        && exact(
            beat4_last,
            StructuralBeamState::End,
            StructuralBeamState::End,
        );

    Some(if beat3_correct && beat4_correct {
        IdentityFreeTailBeamState::AlreadyCorrect
    } else if beat3_correct && beat4_open {
        IdentityFreeTailBeamState::Beat3CorrectBeat4Open
    } else if beat3_empty && beat4_open {
        IdentityFreeTailBeamState::Beat3EmptyBeat4Open
    } else if legacy_cross_beat
        && exact(beat4_cross, absent, absent)
        && exact(beat4_last, absent, absent)
    {
        IdentityFreeTailBeamState::LegacyCrossBeatBeat4Open
    } else {
        IdentityFreeTailBeamState::Unsupported
    })
}

pub(super) fn detect_repair_intent(
    view: &StructuralView,
) -> Option<StructuralRepairIntent> {
    classify_tail_beam_state(view)
        .filter(|state| state.is_repairable())
        .map(|_| StructuralRepairIntent::CompleteTailBeatBeam)
}
