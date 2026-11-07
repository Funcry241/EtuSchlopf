use super::*;

    #[test]
    fn final_chord_readback_requires_the_actual_wrapper_epoch() {
        for renamed in [false, true] {
            let received = OwnedRhythmicContinuation::synthetic_received_for_test();
            let mut evidence = String::new();
            let original = OriginalFunctionalRespellingChains::received(Ok(Arc::new(received.clone())));
            assert!(original.append_final_chord_notation_obligations(
                received.document(), &mut evidence).is_err());
            let (encoded, chains, _) = received.original_export_chains_for_test(renamed);
            chains.append_final_chord_notation_obligations(
                encoded.document(), &mut evidence).unwrap();
            assert!(evidence.contains("FinalChordNotationObligations: bound"));
            let exported = chains.after_original_master_export(encoded.document(), encoded.document(), None);
            assert!(exported.append_final_chord_notation_obligations(
                encoded.document(), &mut String::new()).is_err());
        }
    }
