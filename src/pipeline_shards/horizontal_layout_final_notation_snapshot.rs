// Immutable read-only binding of the final notation artifacts that may become
// the source of a later horizontal-layout plan.
//
// This boundary owns no musical rule, layout policy, writer, renderer decision,
// project identity, measure identity, repair, or recovery. It only freezes the
// exact materialized master/render notation bytes after the existing PDF-source
// selection and its final authorized notation commit, immediately before the
// renderer, so later stages can prove that they did not mutate the selected
// notation source behind the planner boundary.

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalNotationLayoutArtifactSnapshot {
    xml: String,
    fnv64: u64,
}

impl FinalNotationLayoutArtifactSnapshot {
    fn capture(xml: String) -> Self {
        let fnv64 = final_notation_layout_snapshot_fnv64(xml.as_bytes());
        Self { xml, fnv64 }
    }

    fn matches_exact(&self, current_xml: &str) -> bool {
        self.xml == current_xml
            && self.fnv64 == final_notation_layout_snapshot_fnv64(current_xml.as_bytes())
    }

    const fn fingerprint(&self) -> u64 {
        self.fnv64
    }
}

/// The final Master snapshot carries the sovereign released Master state
/// itself (document, GENESIS, full lineage, committed events), not merely
/// re-read bytes; its byte view is derived from that state.
#[derive(Debug, Clone)]
struct FinalNotationLayoutSnapshotSet {
    master: FinalNotationLayoutArtifactSnapshot,
    master_state: structural_write_authority::MasterAdoptedState,
    render: Option<FinalNotationLayoutArtifactSnapshot>,
}

impl FinalNotationLayoutSnapshotSet {
    fn capture(
        master_state: structural_write_authority::MasterAdoptedState,
        render_xml: Option<String>,
    ) -> Self {
        if !master_state
            .lineage()
            .lineage_continuity_holds(master_state.document())
        {
            master_state_identity::master_sovereignty_violation(
                "FinalNotationLayoutSnapshotSet: released Master state has no continuous lineage",
            );
        }
        Self {
            master: FinalNotationLayoutArtifactSnapshot::capture(master_state.document().to_string()),
            master_state,
            render: render_xml.map(FinalNotationLayoutArtifactSnapshot::capture),
        }
    }

    /// The sovereign released Master state this snapshot was frozen from.
    fn released_master_state(&self) -> &structural_write_authority::MasterAdoptedState {
        &self.master_state
    }

    fn selected(
        &self,
        render_selected: bool,
    ) -> Option<&FinalNotationLayoutArtifactSnapshot> {
        if render_selected {
            self.render.as_ref()
        } else {
            Some(&self.master)
        }
    }
}

fn final_notation_layout_snapshot_fnv64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod final_notation_layout_snapshot_tests {
    use super::{
        FinalNotationLayoutArtifactSnapshot, FinalNotationLayoutSnapshotSet,
        final_notation_layout_snapshot_fnv64,
    };
    use crate::master_state_identity::tests::{
        permitted_exact_receipt_for_tests, synthetic_genesis_for_tests,
    };
    use crate::master_state_identity::{MasterLineage, OwnerReceipts};
    use crate::structural_write_authority::MasterAdoptedState;

    fn sovereign_state(document: &str) -> MasterAdoptedState {
        MasterAdoptedState::adopt_genesis_state(
            document.to_string(),
            MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(document)),
        )
    }

    #[test]
    fn snapshot_is_exact_and_rejects_post_close_mutation() {
        let snapshot =
            FinalNotationLayoutArtifactSnapshot::capture("<score>final</score>".to_string());

        assert!(snapshot.matches_exact("<score>final</score>"));
        assert!(!snapshot.matches_exact("<score>changed</score>"));
        assert_eq!(
            snapshot.fingerprint(),
            final_notation_layout_snapshot_fnv64(b"<score>final</score>")
        );
    }

    #[test]
    fn snapshot_set_keeps_master_and_render_sources_distinct() {
        let snapshots = FinalNotationLayoutSnapshotSet::capture(
            sovereign_state("<score>master</score>"),
            Some("<score>render</score>".to_string()),
        );

        assert!(snapshots
            .selected(false)
            .is_some_and(|snapshot| snapshot.matches_exact("<score>master</score>")));
        assert!(snapshots
            .selected(true)
            .is_some_and(|snapshot| snapshot.matches_exact("<score>render</score>")));
    }

    #[test]
    fn missing_render_snapshot_cannot_be_selected() {
        let snapshots =
            FinalNotationLayoutSnapshotSet::capture(sovereign_state("<score>master</score>"), None);

        assert!(snapshots.selected(false).is_some());
        assert!(snapshots.selected(true).is_none());
    }

    #[test]
    fn master_snapshot_keeps_genesis_and_full_lineage() {
        let mut state = sovereign_state("<pitch>0</pitch>");
        let mut receipts = OwnerReceipts::default();
        receipts.push_exact(Some(permitted_exact_receipt_for_tests("<pitch>0</pitch>", "<pitch>1</pitch>")));
        state.adopt_materialized_document("<pitch>1</pitch>".to_string(), receipts, "test");
        let snapshots = FinalNotationLayoutSnapshotSet::capture(state, None);
        let released = snapshots.released_master_state();
        assert_eq!(released.document(), "<pitch>1</pitch>");
        assert_eq!(released.lineage().committed_event_count(), 1);
        assert!(released.lineage().lineage_continuity_holds(released.document()));
        assert!(snapshots
            .selected(false)
            .is_some_and(|snapshot| snapshot.matches_exact(released.document())));
    }
}
