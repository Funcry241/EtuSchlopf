use super::beam_plan::BeamState;
use super::{
    BeamPlan, StructuralBeamState, StructuralCarrierRole, StructuralEvent, StructuralRepairIntent,
    StructuralView,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum IdentityFreeTailBeamState {
    Open,
    AlreadyCorrect,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct LocalBeamMember {
    pub index: usize,
    pub chord_member: bool,
    pub primary: StructuralBeamState,
    pub secondary: StructuralBeamState,
}

pub(super) struct LocalBeamCourse {
    pub main: Vec<LocalBeamMember>,
    pub carriers: Vec<usize>,
    pub rests: Vec<usize>,
}

fn same_attack(a: &StructuralEvent, b: &StructuralEvent) -> bool {
    a.onset_in_window == b.onset_in_window
        && a.duration == b.duration
        && a.staff_relation == b.staff_relation
        && a.voice_relation == b.voice_relation
}

pub(super) fn local_beam_course(view: &StructuralView) -> Option<LocalBeamCourse> {
    use StructuralCarrierRole::{HiddenCarrier, Rest, Sounding};

    let beat = view.metric_units_per_quarter();
    let events = view.events();
    let mut course = LocalBeamCourse {
        main: Vec::new(),
        carriers: Vec::new(),
        rests: Vec::new(),
    };
    let mut heads = Vec::new();
    for (index, event) in events.iter().enumerate() {
        match event.carrier_role {
            Sounding if event.visible && !event.rest_like && event.duration > 0 => {
                let mut chord_member = event.chord_member;
                if event.chord_member {
                    // Keep a real chord attached to its sounding head. An
                    // exact hidden rest host is a separate cursor obligation.
                    let host = events[..index]
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, host)| !host.chord_member && same_attack(event, host))?
                        .0;
                    chord_member = events[host].carrier_role != HiddenCarrier;
                    if !chord_member && !course.carriers.contains(&host) {
                        return None;
                    }
                }
                if !chord_member {
                    heads.push(course.main.len());
                }
                course.main.push(LocalBeamMember {
                    index,
                    chord_member,
                    primary: event.primary_beam,
                    secondary: event.secondary_beam,
                });
            }
            HiddenCarrier if event.rest_like && !event.visible && !event.chord_member => {
                course.carriers.push(index)
            }
            Rest if event.rest_like && event.visible && !event.chord_member => {
                course.rests.push(index)
            }
            _ => return None,
        }
    }
    if course.carriers.is_empty() || course.rests.is_empty() {
        return None;
    }
    if !course.carriers.iter().all(|&index| {
        let carrier = &events[index];
        course
            .main
            .iter()
            .filter(|row| {
                !row.chord_member
                    && events[row.index].chord_member
                    && same_attack(carrier, &events[row.index])
            })
            .count()
            == 1
    }) {
        return None;
    }
    let event_indices = course.main.iter().map(|row| row.index).collect::<Vec<_>>();
    let event_at = |offset: usize| &events[event_indices[offset]];
    if heads.windows(2).any(|pair| {
        let a = event_at(pair[0]);
        let b = event_at(pair[1]);
        a.onset_in_window + a.duration != b.onset_in_window
    }) {
        return None;
    }
    let end = heads.last().map(|&last| {
        let event = event_at(last);
        event.onset_in_window + event.duration
    })?;
    if !course.rests.iter().all(|&index| {
        let rest = &events[index];
        rest.duration > 0 && rest.onset_in_window + rest.duration == end
    }) {
        return None;
    }
    let mut has_target = false;
    for run in heads.chunk_by(|a, b| {
        event_at(*a).onset_in_window.div_euclid(beat)
            == event_at(*b).onset_in_window.div_euclid(beat)
    }) {
        let first = event_at(run[0]);
        let last = event_at(*run.last()?);
        let beat_end = (first.onset_in_window.div_euclid(beat) + 1) * beat;
        let metric_run = run.len() >= 2
            && last.onset_in_window + last.duration <= beat_end
            && run.iter().all(|&member| event_at(member).duration < beat);
        let advances_new_chord = run.iter().any(|&member| event_at(member).chord_member);
        if advances_new_chord {
            if !metric_run
                || !run
                    .iter()
                    .any(|&member| event_at(member).staff_relation != first.staff_relation)
            {
                return None;
            }
            has_target = true;
        }
        if !metric_run {
            continue;
        }
        let primary = (0..run.len()).collect::<Vec<_>>();
        let secondary = (0..run.len()).collect::<Vec<_>>();
        let subgroups = secondary.split(|&i| event_at(run[i]).duration > beat / 4);
        let mut groups = vec![(primary.as_slice(), 1)];
        groups.extend(
            subgroups
                .filter(|members| members.len() >= 2)
                .map(|members| (members, 2)),
        );
        let beam = BeamPlan::from_level_groups(run.len(), &groups)?;
        for &member in run {
            course.main[member].primary = StructuralBeamState::Absent;
            course.main[member].secondary = StructuralBeamState::Absent;
        }
        for edit in beam.edits() {
            let state = match edit.state() {
                BeamState::Begin => StructuralBeamState::Begin,
                BeamState::Continue => StructuralBeamState::Continue,
                BeamState::End => StructuralBeamState::End,
            };
            let member = &mut course.main[run[edit.event_ordinal()]];
            if edit.level() == 1 {
                member.primary = state;
            } else {
                member.secondary = state;
            }
        }
    }
    has_target.then_some(course)
}

pub(super) fn classify_tail_beam_state(view: &StructuralView) -> Option<IdentityFreeTailBeamState> {
    let course = local_beam_course(view)?;
    let is_current = course.main.iter().all(|row| {
        let event = &view.events()[row.index];
        row.primary == event.primary_beam && row.secondary == event.secondary_beam
    });
    Some(if is_current {
        IdentityFreeTailBeamState::AlreadyCorrect
    } else {
        IdentityFreeTailBeamState::Open
    })
}

pub(super) fn detect_repair_intent(view: &StructuralView) -> Option<StructuralRepairIntent> {
    matches!(
        classify_tail_beam_state(view),
        Some(IdentityFreeTailBeamState::Open)
    )
    .then_some(StructuralRepairIntent::CompleteTailBeatBeam)
}
