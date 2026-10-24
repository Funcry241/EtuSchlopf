    #[test]
    fn ssc_f9_original_cursor_owner_mint_survives_adoption_and_report() {
        use crate::master_state_identity::{MasterLineage, OwnerReceipts, ExactEdit};
        crate::master_state_identity::reset_master_lineage_measurement();
        let pre="<score-partwise><part><measure><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><staff>1</staff></note></measure></part></score-partwise>";
        let offset=pre.find("</note>").unwrap();let insertion="<voice>1</voice>";let post=pre.replacen("</note>",&format!("{insertion}</note>"),1);
        let receipt=super::mint_cursor_neutral_receipt(pre,&post,vec![ExactEdit::bound(offset,offset,insertion.to_owned())]).expect("actual sealed #2 mint");
        let genesis=crate::master_state_identity::tests::synthetic_genesis_for_tests(pre);
        let mut state=crate::structural_write_authority::MasterAdoptedState::adopt_genesis_state(pre.to_owned(),MasterLineage::lineage_from_genesis(genesis));
        let mut receipts=OwnerReceipts::default();receipts.push_exact(Some(receipt));state.adopt_materialized_document(post.clone(),receipts,"source-bound serialization obligation");
        crate::master_state_identity::record_final_master_lineage(state.lineage(), state.document());
        let report="TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION\n----------------------------------------\nStatus: present.\nSILENT-VOICE-COVERAGE-WRITE-OUTCOME\n--------------------------------\nStatus: present.\nRendererOutcome: observed_from_existing_writer_audit.\n";
        let report=crate::final_report::DraftReport::new(report.to_owned()).compact(str::to_owned).finalize().unwrap();
        assert!(report.as_str().contains("FamilyF9Permit: 1"),"{}",report.as_str());assert!(report.as_str().contains("Permit: 1"));assert!(report.as_str().contains("InvariantDenied: 0"));assert!(report.as_str().contains("V4Permitted: 1"),"{}",report.as_str());assert_eq!(state.document(),post);
    }
