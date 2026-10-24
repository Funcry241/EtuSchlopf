use std::fs;

#[test]
fn external_sensor_stages_the_complete_shared_midi_source_contract() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let benchmark = fs::read_to_string(
        root.join("src/audit_only/wav_pitch_external_sensor_benchmark.rs"),
    )
    .expect("wav pitch external sensor benchmark source");
    let tool = fs::read_to_string(root.join("tools/wav_pitch_external_sensor_audit/src/main.rs"))
        .expect("wav pitch external sensor tool source");
    let midi = fs::read_to_string(root.join("src/midi.rs")).expect("productive midi source");
    let source_event =
        fs::read_to_string(root.join("src/source_event.rs")).expect("source event owner");

    assert!(midi.contains("project_source_event_causality"));
    assert!(midi.contains("SourceEventCausalLedger"));
    assert!(source_event.contains("pub(crate) fn project_source_event_causality"));
    assert!(source_event.contains("pub(crate) struct SourceEventCausalLedger"));

    assert!(benchmark.contains("&current.join(\"src/midi.rs\")"));
    assert!(benchmark.contains("&target_source.join(\"src/shared_midi.rs\")"));
    assert!(benchmark.contains("&current.join(\"src/source_event.rs\")"));
    assert!(benchmark.contains("&target_source.join(\"src/shared_source_event.rs\")"));

    assert!(tool.contains("#[path = \"shared_source_event.rs\"]\nmod source_event;"));
    assert!(tool.contains("#[path = \"shared_midi.rs\"]\nmod midi;"));
    assert!(tool.contains("[\"src/midi.rs\", \"src/source_event.rs\"]"));
    assert!(!tool.contains("mod source_event {\n    pub(crate) const CANONICAL_DIVISIONS"));
}
