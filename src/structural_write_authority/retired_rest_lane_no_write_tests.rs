// Canonical SWA boundary tests; included in the original private helper scope.
#[cfg(test)]
mod r1230_ir7_retired_no_write_tests {
    use super::*;

    #[test]
    fn retired_ir7_keeps_real_cross_staff_rest_lane_bytes_and_never_touches_sink() {
        for ordinal in [23_usize, 719_usize] {
            let xml = format!("<score-partwise><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>192</duration><voice>1</voice><type>half</type><staff>1</staff></note><note><pitch><step>C</step><octave>3</octave></pitch><duration>192</duration><voice>1</voice><type>half</type><staff>2</staff></note><backup><duration>384</duration></backup><note><rest/><duration>384</duration><voice>7</voice><type>whole</type><staff>2</staff></note></measure></part></score-partwise>");
            let path = std::env::temp_dir().join(format!("r1230-ir7-no-write-{}.render.musicxml", std::process::id()));
            std::fs::write(&path, &xml).unwrap();
            let trial = build_cross_staff_hand_role_materialization_trial(&xml, "real retired IR7 fixture", &path).unwrap();
            assert_eq!(find_next_measure_bounds(&xml, 0).expect("relocated numeric scope").2, ordinal);
            assert!(trial.parse_before_ok);
            assert!(trial.backup_forward_before_ok);
            assert_eq!(trial.candidate_attempted, 0);
            assert_eq!(trial.candidate_adopted, 0);
            assert!(!trial.written);
            assert_eq!(std::fs::read_to_string(&path).unwrap(), xml);
            std::fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn retired_ir7_invalid_input_never_creates_artifact() {
        let path = std::env::temp_dir().join(format!("r1230-ir7-invalid-{}.render.musicxml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let trial = build_cross_staff_hand_role_materialization_trial("not XML", "invalid retired IR7", &path).unwrap();
        assert!(!trial.parse_before_ok);
        assert_eq!(trial.candidate_attempted, 0);
        assert!(!trial.written);
        assert!(!path.exists());
    }
}
