use super::*;

    #[test]
    fn real_final_chord_cursor_emissions_keep_their_received_identity() {
        for copies in [1, 2] {
            for partial in [false, true] {
                let state = OwnedRhythmicContinuation::original_forward_obligations_for_test(partial, copies);
                let bound = state.final_chord_notation_obligations(state.document()).unwrap();
                assert_eq!(bound.rows.iter().filter(|row| row.forward).count(), copies);
                for row in &bound.rows {
                    let encoded = state.encoded.iter().find(|encoded|
                        encoded.received.creation == row.received.creation).unwrap();
                    let fragment = encoded.fragments.iter().find(|fragment| fragment.bytes == row.bytes).unwrap();
                    assert_eq!(row.original_emissions, fragment.original_emissions);
                    assert!(!row.original_emissions.is_empty());
                    assert_eq!(row.origin_partition, fragment.correspondence.origin);
                    assert_eq!(row.represented, fragment.correspondence.image);
                }
                let mut out = String::new();
                bound.append_evidence(&mut out);
                assert!(out.contains("canonical_role_authority=none"));
            }
        }
    }

    #[test]
    fn real_final_chord_readback_refuses_foreign_snapshot_and_missing_emission_path() {
        let state = OwnedRhythmicContinuation::synthetic_received_for_test()
            .original_final_chord_for_test();
        let foreign = format!("{} ", state.document());
        assert!(state.final_chord_notation_obligations(&foreign).is_err());
        let mut missing = state.clone();
        missing.encoded[0].fragments[0].original_emissions.clear();
        assert!(missing.final_chord_notation_obligations(missing.document()).is_err());
        assert!(state.final_chord_notation_obligations(state.document()).is_ok());
    }
