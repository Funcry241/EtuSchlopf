use crate::mqtt_client::MqttSessionStateClient;
use crate::observation_protocol::{
    ObservedSourceSnapshotV2, SessionSnapshotV2, SnapshotActivityV2, SnapshotSourceRoleV2,
    SnapshotSourceStatusV2, SnapshotStageV2,
};
use crate::session_json::{decode_session_snapshot_json_v2, mqtt_topic_for_session};
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

const UI_WIDTH: usize = 78;
const LABEL_WIDTH: usize = 17;
const STATE_WIDTH: usize = 30;
const AGE_WIDTH: usize = 25;

#[derive(Clone, Copy)]
enum ConnectionState {
    Connecting,
    Live,
    Reconnecting,
}

pub fn spawn_mqtt_ui(
    address: SocketAddr,
    current_session_id: Option<String>,
) -> io::Result<thread::JoinHandle<()>> {
    thread::Builder::new()
        .name("etuschlopf-watch-ui-mqtt-client".to_string())
        .spawn(move || run_ui(address, current_session_id))
}

fn run_ui(address: SocketAddr, mut current_session_id: Option<String>) {
    let mut sessions = BTreeMap::<String, SessionSnapshotV2>::new();
    let mut connection = ConnectionState::Connecting;
    render(&sessions, connection, address);

    loop {
        match MqttSessionStateClient::connect(address) {
            Ok(mut client) => {
                connection = ConnectionState::Live;
                render(&sessions, connection, address);
                loop {
                    match client.read_next() {
                        Ok(Some(publication)) => {
                            if let Ok(snapshot) = decode_session_snapshot_json_v2(&publication.payload) {
                                let topic_matches = mqtt_topic_for_session(&snapshot.observation_session_id)
                                    .as_deref()
                                    == Some(publication.topic.as_str());
                                if topic_matches {
                                    let session_id = snapshot.observation_session_id.clone();
                                    if current_session_id.is_none() {
                                        current_session_id = Some(session_id.clone());
                                        sessions.clear();
                                    }
                                    if current_session_id.as_deref() == Some(session_id.as_str()) {
                                        sessions.insert(session_id, snapshot);
                                        render(&sessions, connection, address);
                                    }
                                }
                            }
                        }
                        Ok(None) => {}
                        Err(_) => break,
                    }
                }
            }
            Err(_) => {}
        }
        connection = ConnectionState::Reconnecting;
        render(&sessions, connection, address);
        thread::sleep(Duration::from_millis(500));
    }
}

fn render(
    sessions: &BTreeMap<String, SessionSnapshotV2>,
    connection: ConnectionState,
    address: SocketAddr,
) {
    print!("\x1b[2J\x1b[H");
    println!("ETUSCHLOPF WATCH");
    println!("{}", "═".repeat(UI_WIDTH));
    println!("{}   MQTT {}", connection_label(connection), address);
    println!();

    if sessions.is_empty() {
        println!("Warte auf den ersten Runtime-Snapshot …");
        println!("Die Laufansicht öffnet sich automatisch.");
        let _ = io::stdout().flush();
        return;
    }

    for (index, session) in sessions.values().enumerate() {
        if index != 0 {
            println!();
        }
        render_session(session);
    }
    let _ = io::stdout().flush();
}

fn render_session(session: &SessionSnapshotV2) {
    let summary = session_summary(session);
    let left = format!("SESSION {}", short_session_id(&session.observation_session_id));
    let right = format!("{} {}", status_symbol(summary), status_label(summary));
    print_spread(&left, &right);
    println!("{}", "─".repeat(UI_WIDTH));

    let corpus = find_role(session, SnapshotSourceRoleV2::Corpus);
    let launcher = find_role(session, SnapshotSourceRoleV2::Launcher);

    if let Some(corpus) = corpus {
        let (label, state) = corpus_display(corpus);
        render_source_row(&label, &state, corpus);
    }

    if let Some(launcher) = launcher {
        render_source_row("Launcher", &display_state(launcher), launcher);
    }

    let mut projects = session
        .sources
        .iter()
        .filter(|observed| observed.source.source_role == SnapshotSourceRoleV2::Project)
        .collect::<Vec<_>>();
    projects.sort_by(|left, right| source_sort_key(left).cmp(&source_sort_key(right)));

    if !projects.is_empty() && (corpus.is_some() || launcher.is_some()) {
        println!();
    }

    for project in projects {
        let label = project
            .source
            .project_label
            .as_deref()
            .unwrap_or(project.source.source_id.as_str());
        render_source_row(label, &display_state(project), project);
    }
}

fn render_source_row(label: &str, state: &str, source: &ObservedSourceSnapshotV2) {
    println!(
        "{:<label_width$} {:<state_width$} {}  {:>age_width$}",
        truncate(label, LABEL_WIDTH),
        truncate(state, STATE_WIDTH),
        status_symbol(source.source.status),
        truncate(&age_pair(source), AGE_WIDTH),
        label_width = LABEL_WIDTH,
        state_width = STATE_WIDTH,
        age_width = AGE_WIDTH,
    );
}

fn corpus_display(source: &ObservedSourceSnapshotV2) -> (String, String) {
    match source.source.progress {
        Some(progress) => (
            format!("Korpus {}/{}", progress.completed, progress.total),
            format!("{} · {} aktiv", display_state(source), progress.active),
        ),
        None => ("Korpus".to_string(), display_state(source)),
    }
}

fn connection_label(connection: ConnectionState) -> &'static str {
    match connection {
        ConnectionState::Connecting => "○ CONNECTING",
        ConnectionState::Live => "● LIVE",
        ConnectionState::Reconnecting => "○ RECONNECTING",
    }
}

fn session_summary(session: &SessionSnapshotV2) -> SnapshotSourceStatusV2 {
    if session
        .sources
        .iter()
        .any(|source| source.source.status == SnapshotSourceStatusV2::Failed)
    {
        return SnapshotSourceStatusV2::Failed;
    }
    if session
        .sources
        .iter()
        .any(|source| source.source.status == SnapshotSourceStatusV2::Running)
    {
        return SnapshotSourceStatusV2::Running;
    }
    if session
        .sources
        .iter()
        .any(|source| source.source.status == SnapshotSourceStatusV2::Cancelled)
    {
        return SnapshotSourceStatusV2::Cancelled;
    }
    SnapshotSourceStatusV2::Completed
}

fn find_role(
    session: &SessionSnapshotV2,
    role: SnapshotSourceRoleV2,
) -> Option<&ObservedSourceSnapshotV2> {
    session
        .sources
        .iter()
        .find(|observed| observed.source.source_role == role)
}

fn source_sort_key(source: &ObservedSourceSnapshotV2) -> (u64, &str) {
    let index = source
        .source
        .source_id
        .strip_prefix("project:")
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(u64::MAX);
    (index, source.source.source_id.as_str())
}

fn display_state(source: &ObservedSourceSnapshotV2) -> String {
    state_path(source.source.stage, source.source.activity)
}

fn state_path(stage: SnapshotStageV2, activity: Option<SnapshotActivityV2>) -> String {
    match activity {
        Some(activity) => format!("{} › {}", stage_label(stage), activity_label(activity)),
        None => stage_label(stage).to_string(),
    }
}

fn activity_label(activity: SnapshotActivityV2) -> &'static str {
    match activity {
        SnapshotActivityV2::ClefPlanning => "Clef Planning",
        SnapshotActivityV2::BeamPlanning => "Beam Planning",
    }
}

fn stage_label(stage: SnapshotStageV2) -> &'static str {
    match stage {
        SnapshotStageV2::LauncherPreparing => "Preparing",
        SnapshotStageV2::LauncherBuilding => "Building",
        SnapshotStageV2::LauncherTests => "Launcher Tests",
        SnapshotStageV2::LauncherRunningTarget => "Running Target",
        SnapshotStageV2::LauncherFinalizing => "Finalizing",
        SnapshotStageV2::ProjectRunPreparation => "Preparation",
        SnapshotStageV2::ProjectArrangementSolver => "Arrangement Solver",
        SnapshotStageV2::ProjectMasterNotationAndSalience => "Master Notation",
        SnapshotStageV2::ProjectInitialRenderTransaction => "Initial Render",
        SnapshotStageV2::ProjectMetricBeamAndFinalNotation => "Final Notation",
        SnapshotStageV2::ProjectReleaseReportAndExit => "Release",
        SnapshotStageV2::CorpusPlanning => "Planning",
        SnapshotStageV2::CorpusPreparingProjects => "Preparing Projects",
        SnapshotStageV2::CorpusRunningProjects => "Running Projects",
        SnapshotStageV2::CorpusCollectingProjectResult => "Collecting Results",
        SnapshotStageV2::CorpusFinalizingCorpusReport => "Finalizing Report",
    }
}

fn status_symbol(status: SnapshotSourceStatusV2) -> &'static str {
    match status {
        SnapshotSourceStatusV2::Running => "●",
        SnapshotSourceStatusV2::Completed => "✓",
        SnapshotSourceStatusV2::Failed => "✕",
        SnapshotSourceStatusV2::Cancelled => "○",
    }
}

fn status_label(status: SnapshotSourceStatusV2) -> &'static str {
    match status {
        SnapshotSourceStatusV2::Running => "RUNNING",
        SnapshotSourceStatusV2::Completed => "COMPLETED",
        SnapshotSourceStatusV2::Failed => "FAILED",
        SnapshotSourceStatusV2::Cancelled => "CANCELLED",
    }
}

fn age_pair(source: &ObservedSourceSnapshotV2) -> String {
    format!(
        "seen {}  state {}",
        format_age(source.last_seen_age_millis),
        format_age(source.last_state_change_age_millis)
    )
}

fn format_age(milliseconds: u64) -> String {
    let seconds = milliseconds / 1000;
    if seconds < 1 {
        return "<1s".to_string();
    }
    if seconds < 60 {
        return format!("{seconds}s");
    }
    let minutes = seconds / 60;
    let remaining = seconds % 60;
    if minutes < 60 {
        return format!("{minutes}:{remaining:02}");
    }
    let hours = minutes / 60;
    format!("{hours}h{:02}", minutes % 60)
}

fn short_session_id(session_id: &str) -> String {
    let count = session_id.chars().count();
    if count <= 18 {
        return session_id.to_string();
    }
    session_id.chars().skip(count.saturating_sub(18)).collect()
}

fn truncate(value: &str, width: usize) -> String {
    let count = value.chars().count();
    if count <= width {
        return value.to_string();
    }
    if width <= 1 {
        return "…".to_string();
    }
    let mut text = value.chars().take(width - 1).collect::<String>();
    text.push('…');
    text
}

fn print_spread(left: &str, right: &str) {
    let left = truncate(left, UI_WIDTH);
    let right = truncate(right, UI_WIDTH);
    let occupied = left.chars().count() + right.chars().count();
    let gap = UI_WIDTH.saturating_sub(occupied).max(1);
    println!("{left}{}{right}", " ".repeat(gap));
}

#[cfg(test)]
mod tests {
    use super::{format_age, state_path, truncate};
    use crate::observation_protocol::{SnapshotActivityV2, SnapshotStageV2};

    #[test]
    fn display_helpers_are_bounded_and_human_readable() {
        assert_eq!(format_age(400), "<1s");
        assert_eq!(format_age(3_000), "3s");
        assert_eq!(format_age(197_000), "3:17");
        assert_eq!(truncate("Etüde Nr. 6", 20), "Etüde Nr. 6");
        assert_eq!(truncate("123456789", 6), "12345…");
    }

    #[test]
    fn activity_keeps_its_parent_stage_visible() {
        assert_eq!(
            state_path(
                SnapshotStageV2::ProjectMetricBeamAndFinalNotation,
                Some(SnapshotActivityV2::BeamPlanning),
            ),
            "Final Notation › Beam Planning"
        );
    }
}
