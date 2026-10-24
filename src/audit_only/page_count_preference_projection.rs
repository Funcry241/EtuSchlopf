// Read-only transport adapter for the already computed page-count preference evidence.
//
// The r06/r07 disabled-state cutover is retired: this adapter is now an unconditional
// read-only transport for the already accepted page-count preference evidence. Existing
// Music-domain callers and result writers remain unchanged; this adapter only captures the
// already computed typed preference line and projects it into established outputs.

use std::io::BufRead;

const CONSOLE_PREFIX: &str =
    "ETUSCHLOPF: HorizontalLayoutGlobalPageCountPreferenceShadow:";
const RESULT_PREFIX: &str =
    "ETUSCHLOPF: HorizontalLayoutGlobalPageCountPreference:";
const JOINT_CONSOLE_PREFIX: &str =
    "ETUSCHLOPF: HorizontalLayoutGlobalPageSystemPreferenceCandidate:";
const FINAL_SELECTED_PLAN_DEMAND_PREFIX: &str =
    "ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding:";
pub(crate) const SECTION: &str = "HORIZONTAL-LAYOUT-PAGE-COUNT-PREFERENCE";
pub(crate) const SECTION_RULE: &str = "---------------------------------------";

fn unique_preference_line(lines: &[String], origin: &str) -> anyhow::Result<Option<String>> {
    let matches = lines
        .iter()
        .filter(|line| line.starts_with(CONSOLE_PREFIX))
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [line] => Ok(Some((*line).clone())),
        _ => anyhow::bail!(
            "PageCountPreferenceProjection: {origin} ist mehrdeutig ({} Treffer)",
            matches.len()
        ),
    }
}

fn unique_joint_planner_line(lines: &[String], origin: &str) -> anyhow::Result<Option<String>> {
    let matches = lines
        .iter()
        .filter(|line| line.starts_with(JOINT_CONSOLE_PREFIX))
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [line] => Ok(Some((*line).clone())),
        _ => anyhow::bail!(
            "PageCountPreferenceProjection: {origin} Joint-Planner-Evidence ist mehrdeutig ({} Treffer)",
            matches.len()
        ),
    }
}


fn unique_final_selected_plan_demand_line(
    lines: &[String],
    origin: &str,
) -> anyhow::Result<Option<String>> {
    let matches = lines
        .iter()
        .filter(|line| line.starts_with(FINAL_SELECTED_PLAN_DEMAND_PREFIX))
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [line] => Ok(Some((*line).clone())),
        _ => anyhow::bail!(
            "PageCountPreferenceProjection: {origin} Final-Plan-Demand-Evidence ist mehrdeutig ({} Treffer)",
            matches.len()
        ),
    }
}


pub(crate) fn current_process_line(lines: &[String]) -> anyhow::Result<Option<String>> {
    let joint_planner_line =
        unique_joint_planner_line(lines, "current_process_console_capture")?;
    let final_selected_plan_demand_line =
        unique_final_selected_plan_demand_line(lines, "current_process_console_capture")?;
    unique_preference_line(lines, "current_process_console_capture").map(|line| {
        line.map(|line| {
            let mut projected = projection_safe_preference_line(&line);
            for evidence in
                super::renderer_system_break_readback::current_process_projection_lines(
                    joint_planner_line.as_deref(),
                    final_selected_plan_demand_line.as_deref(),
                )
            {
                projected.push('\n');
                projected.push_str(&evidence);
            }
            projected
        })
    })
}

pub(crate) fn batch_child_stdout() -> std::process::Stdio {
    std::process::Stdio::piped()
}

pub(crate) struct BatchChildPreferenceCapture {
    handle: std::thread::JoinHandle<anyhow::Result<Option<String>>>,
}

impl BatchChildPreferenceCapture {
    pub(crate) fn begin(child: &mut std::process::Child) -> anyhow::Result<Self> {
        let stdout = child.stdout.take().ok_or_else(|| {
            anyhow::anyhow!(
                "PageCountPreferenceProjection: aktivierter Batch-Child besitzt kein gepiptes stdout"
            )
        })?;
        let handle = std::thread::spawn(move || {
            let mut matches = Vec::new();
            for line in std::io::BufReader::new(stdout).lines() {
                let line = line.map_err(|error| {
                    anyhow::anyhow!(
                        "PageCountPreferenceProjection: Batch-Child-stdout nicht lesbar: {error}"
                    )
                })?;
                if line.starts_with(CONSOLE_PREFIX) {
                    matches.push(line);
                } else {
                    // Console status records share this already piped stdout; they are
                    // forwarded unchanged and never affect the preference capture.
                    etuschlopf::console_status_record::forward_record_line(&line);
                }
            }
            unique_preference_line(&matches, "batch_child_stdout_capture")
        });
        Ok(Self { handle })
    }

    pub(crate) fn finish(self) -> anyhow::Result<Option<String>> {
        self.handle.join().map_err(|_| {
            anyhow::anyhow!(
                "PageCountPreferenceProjection: Batch-Child-Capture-Thread ist fehlgeschlagen"
            )
        })?
    }
}

fn projection_safe_preference_line(preference_line: &str) -> String {
    preference_line
        .replacen(CONSOLE_PREFIX, RESULT_PREFIX, 1)
        .replace("legacyThin", "nonterminalThin")
        .replace("VerticalMaxSystemsPerPage ", "VerticalMaxSystemsPerPage=")
        .replace(
            "ClosestLowerSystemsPerPage ",
            "ClosestLowerSystemsPerPage=",
        )
}

pub(crate) fn append_section(out: &mut String, preference_line: &str, origin: &str) {
    out.push('\n');
    out.push_str(SECTION);
    out.push('\n');
    out.push_str(SECTION_RULE);
    out.push('\n');
    out.push_str("EvidenceOrigin: ");
    out.push_str(origin);
    out.push('\n');
    out.push_str("PolicyWrite: nein\n");
    out.push_str(&projection_safe_preference_line(preference_line));
    out.push('\n');
    out.push_str("ProjectionStatus: persisted_read_only_evidence\n");
}


#[cfg(test)]
mod renderer_outcome_transport_tests {
    use super::{
        SECTION, projection_safe_preference_line, unique_final_selected_plan_demand_line,
    };

    #[test]
    fn final_selected_plan_demand_line_is_unique_and_independent_from_initial_joint_evidence() {
        let lines = vec![
            "ETUSCHLOPF: HorizontalLayoutGlobalPageSystemPreferenceCandidate: Slots 3".to_string(),
            "ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding: AcceptedSystemCapacity 500 | SelectedSystemLoads 1-2=300,3=100 | SelectedMeasureDemands 1=150,2=150,3=100".to_string(),
        ];
        let resolved = unique_final_selected_plan_demand_line(&lines, "test")
            .expect("final demand evidence")
            .expect("final demand line");
        assert!(resolved.starts_with("ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding:"));
    }

    #[test]
    fn persisted_page_count_projection_uses_first_class_result_vocabulary() {
        let projected = projection_safe_preference_line(
            "ETUSCHLOPF: HorizontalLayoutGlobalPageCountPreferenceShadow: Slots 3 | FullFrontier p3{legacyThin0}",
        );
        assert!(projected.starts_with(
            "ETUSCHLOPF: HorizontalLayoutGlobalPageCountPreference: Slots 3"
        ));
        assert!(projected.contains("nonterminalThin0"));
        assert!(!projected.contains("PageCountPreferenceShadow"));
        assert!(!projected.contains("legacyThin"));
        assert!(!SECTION.contains("SHADOW"));
    }

}
