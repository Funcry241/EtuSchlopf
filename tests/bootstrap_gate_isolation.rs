use std::fs;
use std::path::PathBuf;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn take_bootstrap_architecture_gate_is_mandatory_and_recovery_free() {
    let root = project_root();
    let run_cmd = fs::read_to_string(root.join("run.cmd")).expect("run.cmd");
    let build_script =
        fs::read_to_string(root.join("src/kodex_build_orchestrator.rs")).expect("build script");
    let launcher = fs::read_to_string(root.join("src/bin/etuschlopf_launcher.rs"))
        .expect("launcher");

    let bootstrap_build = "--bin etuschlopf_take_bootstrap";
    let launcher_build = "--bin etuschlopf_launcher";
    let bootstrap_at = run_cmd.find(bootstrap_build).expect("bootstrap build");
    let launcher_at = run_cmd.find(launcher_build).expect("launcher build");
    assert!(bootstrap_at < launcher_at);

    assert!(!run_cmd.contains("ETUSCHLOPF_ARCHITECTURE_GATE_CONTEXT"));
    assert!(!build_script.contains("ETUSCHLOPF_ARCHITECTURE_GATE_CONTEXT"));
    assert!(!build_script.contains("take_bootstrap_compile"));
    assert!(!build_script.contains("ETUSCHLOPF_TAKE_BOOTSTRAP_GATE_ISOLATED"));
    assert!(build_script.contains("validate_productive_music_identity_provenance_tree"));
    assert!(launcher.contains(
        "Die Take-Startkompilation durchlaeuft die verpflichtende Architekturpruefung; ein besonderer Umgehungskontext bleibt nicht bestehen"
    ));
    assert!(!launcher.contains(
        "The take-bootstrap compile is isolated from productive architecture gates by an exact context plus target-path contract"
    ));

    assert!(!root.join("tools/bootstrap_take.ps1").exists());
}

#[test]
fn wrapper_console_state_is_neutral_and_errors_use_stderr() {
    let root = project_root();
    let run_cmd = fs::read_to_string(root.join("run.cmd")).expect("run.cmd");
    assert!(
        !run_cmd
            .lines()
            .any(|line| line.trim_start().to_ascii_lowercase().starts_with("color "))
    );
    for line in run_cmd.lines().filter(|line| line.contains("FEHLER:")) {
        assert!(line.contains("1>&2"), "wrapper error must use stderr: {line}");
    }

    let bootstrap = fs::read_to_string(root.join("src/bin/etuschlopf_take_bootstrap.rs"))
        .expect("take bootstrap");
    assert!(bootstrap.contains(r#"#[path = "../console_presentation_contract.rs"]"#));
    assert!(bootstrap.contains("write_bootstrap_presentation"));
}

