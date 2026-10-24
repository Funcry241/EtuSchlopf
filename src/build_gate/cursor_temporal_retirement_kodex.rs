//! Parent-compiled, exact one-way retirement of #24/#56 and read-only boundary.
//! No runtime authority. Child cannot modify these compiled rules.
use std::path::Path;
use crate::sha256;
pub(crate) const MARKER: &str = "src/cursor_lane_temporal_check.rs";
const SUCCESSORS: &[(&str, &str)] = &[
    ("ZIP_KIND.txt", "3f32028b99895d95dd61844169ce880bc1c85d106b21c60fa0d705af080187ec"),
    ("docs/render_opus_population_map_r1225.tsv", "3e17108eb93bab93f229ba50425f006b0f772f25befdc9fea9a983e9363da733"),
    ("docs/selector_derivation_contract_cursor_temporal_check_r1446.md", "4e3c5eb53df56f9e398aceb7b74563cf7b6c387341e13c3c557e47664bcfa543"),
    ("src/audit_only/serialized_temporal_replay.rs", "9bf86d21e8e42c1e0b47cad29e226fdd629b7d873b6d69324d2553f2006d662f"),
    ("src/build_gate/destructive_transformation_population_v1.tsv", "77a90a3e30ac4d0728cfeae543e567387d69a53aa867e52455fe81442379f463"),
    ("src/build_gate/render_origins_v1.tsv", "c2702e3971d92c247c5789a3750bb1a3c154011cd1133445fdc7f3c7f7e959c8"),
    ("src/build_gate/ssc_containment_envelopes_v1.tsv", "535fcedabf3a867c28d732f15722cd843a26d219eb48ffdd157bbd25033825f9"),
    ("src/cursor_lane_temporal_check.rs", "613c9599d6b4a3fbddc5713dc187691ff0f657965f5369ad7c3376a53cc73793"),
    ("src/main.rs", "5c0c5e953391cf177c8fb710a54d0a3ea823867713e5f32b2ad7a2b103c11f62"),
    ("src/master_state_identity.rs", "49a9757503c5ff6ede3ce7308f1bd07c013fcdf8f49f2264183a89979cbc8932"),
    ("src/master_state_identity/render_lineage.rs", "f06c4704e15a8a5bcdf133acd938274a7c0d72060a4a3a34f7cb7b8de605d628"),
    ("src/master_state_identity/tests.rs", "5f76922456d7ce8e10f2a9509a07363e912c890bb4a1cec628a6eec0d7e10899"),
    ("src/pipeline.rs", "d3333cd6a742116982440d4ab653950ecbd28692f109a44a62ee60585935c38b"),
    ("src/pipeline_shards/pipeline_orchestrator_clef_layout_support.rs", "1b2417156b87085b9df497cdd85d6afcd2ff2fdfb4154c93d524d1438ec9d226"),
    ("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs", "c8e3ab28bb11574b5a72c00ff9e1cb038f3a97d413ece3e624528a645c1e28cd"),
    ("src/pipeline_shards/pipeline_orchestrator_release_report_exit_stage.rs", "9bff3ee977aa56743b8e7b785db63c155cd3c473f76c7428f72413c764d8d142"),
    ("src/pipeline_shards/pipeline_orchestrator_release_support.rs", "eaa6cda7e6d2143bc401e31653bda57ca3086f319e98714f8f5d82d72ef16802"),
    ("src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs", "588f2194e477e1ae4b5837afeecd65089baac5cbcac53a5535c6303a4826537d"),
    ("src/pipeline_shards/report_hygiene_cursor_localbest_trace_tail.rs", "7dd472227a839b2044243d8c24af3eff23ab0abd80aa38ee1d81fa36e3803bef"),
    ("src/structural_write_authority/final_notation_phrase_projection_transaction.rs", "ca8e798a9b5758fb1d9a4cd584c8c55d9cf45e424d63784cf295a492ecb09726"),
    ("src/structural_write_authority/master_adoption/render_adopted_state_tests.rs", "9a932b5f5badbcf758af43dc4562d512033cfee627a2642badd7eee5be1a1dc3"),
    ("src/structural_write_authority/render_lineage_receipt_tests.rs", "883bda12586434c97afd7ac753b38905f0851d5ddc845a68aa42a361bc00ac06"),
    ("tests/support/render_authority/row57_local_clef_scope_through_trial.rs", "54ac06aa7acd3556d49c5a160519f1fba735a3095088725bdf5dddd2b856769d"),
    ("tests/unit/cursor_temporal_boundary.rs", "57e09db5aa90d5f44fa96963e097f8d1c8b3de40caa9411887ce28d3846c1f19"),
];
pub(crate) fn installed(root: &Path) -> bool { root.join(MARKER).is_file() }
pub(crate) fn validate_tree(root: &Path) -> Result<(), String> {
    if !installed(root) {
        let origins=std::fs::read_to_string(root.join("src/build_gate/render_origins_v1.tsv")).unwrap_or_default();
        if origins.contains("Unchanged Master after temporal check and before #65") {
            return Err("CursorTemporal origin successor requires complete installed checker".into());
        }
        return Ok(());
    }
    let checker=std::fs::read(root.join(MARKER)).map_err(|e| e.to_string())?;
    let expected=SUCCESSORS.iter().find(|(path,_)| *path==MARKER).ok_or("checker binding missing")?.1;
    if sha256::hex(&checker)!=expected { return Err("CursorTemporal checker bytes changed".into()); }
    // This observer now determines productive rejection, regardless of its audit path.
    // Keep the exact implementation bound in every installed tree, after transition too.
    let replay_path="src/audit_only/serialized_temporal_replay.rs";
    let replay=std::fs::read(root.join(replay_path)).map_err(|e| e.to_string())?;
    let replay_digest=SUCCESSORS.iter().find(|(path,_)| *path==replay_path).ok_or("replay binding missing")?.1;
    if sha256::hex(&replay)!=replay_digest { return Err("CursorTemporal productive replay bytes changed".into()); }
    let stage=std::fs::read(root.join("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs")).map_err(|e| e.to_string())?;
    if !stage_successor(&stage) { return Err("CursorTemporal initial-stage boundary changed".into()); }
    let lineage=std::fs::read_to_string(root.join("src/master_state_identity/render_lineage.rs")).map_err(|e| e.to_string())?;
    if !checked_lineage_successor(&lineage) { return Err("CursorTemporal epoch byte-proof binding changed".into()); }
    super::absent(&super::runtime_sources(root)?, &[
        "CursorLaneEndClosureBlockMint", "RenderRow24BlockMint",
        "CURSOR_LANE_END_CLOSURE_BLOCK", "RENDER_ROW_24_BLOCK",
        "repair_cursor_lane_end_invariant_xml", "repair_cursor_lane_end_invariant_measure",
        "cursor_lane_repair_insert_chord_tag", "apply_cursor_lane_reanchor_rest_visibility_contract_to_measure",
    ])?;
    Ok(())
}
pub(crate) fn stage_successor(bytes: &[u8]) -> bool {
    SUCCESSORS.iter().any(|(path,digest)| *path=="src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs" && sha256::hex(bytes)==*digest)
        || super::checkpoint_prepare::stage_successor(bytes)
}
pub(crate) fn validate_transition(parent: &Path, child: &Path, class: &str) -> Result<(), String> {
    let before=installed(parent);let after=installed(child);
    if before && !after { return Err("CursorTemporal retirement cannot revert".into()); }
    if !before && after && class!="MusicMutation" { return Err("CursorTemporal transition requires MusicMutation".into()); }
    if !before && after {
        for (path, expected) in SUCCESSORS {
            let bytes=std::fs::read(child.join(path)).map_err(|e| format!("CursorTemporal successor missing {path}: {e}"))?;
            if sha256::hex(&bytes)!=*expected { return Err(format!("CursorTemporal exact successor drift: {path}")); }
        }
    }
    validate_tree(child)
}

pub(crate) fn bound_path_admissible(path: &str, fingerprint: Option<u64>) -> bool {
    matches!((path, fingerprint),
        ("src/audit_only/serialized_temporal_replay.rs", Some(0x17a3f1d924742ea4)) |
        ("src/main.rs", Some(0x594f6e7c48b20151)) |
        ("src/build_gate/destructive_transformation_population_v1.tsv", Some(0xe75776661286014f)) |
        ("src/build_gate/render_origins_v1.tsv", Some(0x81e20fee4cbb052b)) |
        ("src/build_gate/ssc_containment_envelopes_v1.tsv", Some(0x51f686308e70ad52))
    )
}

pub(crate) fn checked_lineage_successor(source: &str) -> bool {
    super::item(source, "fn", "after_cursor_lane_before_final_chord").is_ok_and(|body| sha256::hex(body.as_bytes())=="2e4ef5d95a3b63699a647228b6acfef8cc9342cfdbdb928927e7a7f248f4c9ad")
}

// Only root ZIP metadata has the transaction backup's root_files layout.
// Use the alternate location only when the direct path is absent; a direct
// mismatch or other read failure must never be masked by a second copy.
fn read_parent_bytes(parent: &Path, path: &str) -> std::io::Result<Vec<u8>> {
    std::fs::read(parent.join(path)).or_else(|error| {
        if path == "ZIP_KIND.txt" && error.kind() == std::io::ErrorKind::NotFound {
            std::fs::read(parent.join("root_files/ZIP_KIND.txt"))
        } else {
            Err(error)
        }
    })
}

pub(crate) fn verified_first_transition(parent: &Path, child: &Path) -> Result<bool,String> {
    if installed(parent) || !installed(child) { return Ok(false); }
    let stage=std::fs::read(parent.join("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs")).map_err(|e| e.to_string())?;
    if sha256::hex(&stage)!="873bbd4b83afb4b6a2f9f3ba094d714dd1c16adfca1d0a7e3062ad8af73b1f92" { return Err("CursorTemporal transition has foreign parent boundary".into()); }
    for (path,expected) in PARENTS {
        let actual=read_parent_bytes(parent,path);
        match (expected,actual) {
            (Some(digest),Ok(bytes)) if sha256::hex(&bytes)==*digest => {},
            (None,Err(error)) if error.kind()==std::io::ErrorKind::NotFound => {},
            _ => return Err(format!("CursorTemporal parent bytes differ: {path}")),
        }
    }
    validate_transition(parent,child,"MusicMutation")?;
    Ok(true)
}

const PARENTS: &[(&str, Option<&str>)] = &[
    ("ZIP_KIND.txt", Some("33425ae9a23caef738cd4b804d23dc98a80b013935c997db79f912b601d8de8f")),
    ("docs/render_opus_population_map_r1225.tsv", Some("feda4f8b5372410115790f591ccf18ab31c824b613c8e176a511a7124e5d7f25")),
    ("docs/selector_derivation_contract_cursor_temporal_check_r1446.md", None),
    ("src/audit_only/serialized_temporal_replay.rs", Some("704f4db7dc1bdd75037b8a96bcd5eb2294d70ad0bb6f1efe43b713d17259f3ca")),
    ("src/build_gate/destructive_transformation_population_v1.tsv", Some("1309a26ff15f9217a8bd2e724bd1df9adb3b59b3d129663bf741cc1c12c82f72")),
    ("src/build_gate/render_origins_v1.tsv", Some("da4429dbdd5dfce348dcdd0da220fa476bc83a1978ae8d268eeff342c89fc4f8")),
    ("src/build_gate/ssc_containment_envelopes_v1.tsv", Some("aab322bbb0906f59c5ba515ea41ba2b2085ac11b9e53b9168c6b1c2be92e15f6")),
    ("src/cursor_lane_temporal_check.rs", None),
    ("src/main.rs", Some("16909b430b83f4c727f412cac1d1788f825ea0562a5bbef970871d89bc4cf641")),
    ("src/master_state_identity.rs", Some("e0a7e64c2973852cd6261e18a1dd68297a80873a68ce7ac470897de9d427c64d")),
    ("src/master_state_identity/render_lineage.rs", Some("02159439b5142616ec1d7c462e7bc395c4afbdc9de94c67d77733829da2882ef")),
    ("src/master_state_identity/tests.rs", Some("89e80297708de04c6554de52577b9f3f66c6aa71ce3634404e6185ebb49a85e4")),
    ("src/pipeline.rs", Some("fb3c27afd58df1c29e9461d5ee3500328c509a066ecf27a65acddc50c851b92f")),
    ("src/pipeline_shards/pipeline_orchestrator_clef_layout_support.rs", Some("86de04e2ac2c0d8ac9824bb9e74771f12520701c6464822642be2033cea35783")),
    ("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs", Some("873bbd4b83afb4b6a2f9f3ba094d714dd1c16adfca1d0a7e3062ad8af73b1f92")),
    ("src/pipeline_shards/pipeline_orchestrator_release_report_exit_stage.rs", Some("f07a70bacc071f652f4ea750c38b5a1776e63442cc8a4fdfc59a9351f119b9a8")),
    ("src/pipeline_shards/pipeline_orchestrator_release_support.rs", Some("52c203d65ff618a3f85678eb05f115ffd7ff0e668afe8e87d4cabff3d7e38ed9")),
    ("src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs", Some("c09dab5643e7e604f401cd7257c129d55b1ea242df09399ab2e08351f26f7f0f")),
    ("src/pipeline_shards/report_hygiene_cursor_localbest_trace_tail.rs", Some("d36c83245673f1b6825667c1d27b05dafdd6c05ea50ed9f32436b436cf68558a")),
    ("src/structural_write_authority/final_notation_phrase_projection_transaction.rs", Some("3ed22394b3844f80a58344e7b665916f549dcf5163077eb1eab0478412310464")),
    ("src/structural_write_authority/master_adoption/render_adopted_state_tests.rs", Some("5f39b45df694e2edd96a67d8ecf8cc0eee0b4f2c7ebf59c2952c9b55dddab119")),
    ("src/structural_write_authority/render_lineage_receipt_tests.rs", Some("838e99b7e0bdd9aff24a38f81c9888f53db72075e09628992acf06b63e1e06b5")),
    ("tests/support/render_authority/row57_local_clef_scope_through_trial.rs", Some("30ca108eb33d21425f1cef010620827be53a2c9d9aca81a7673ecd99d7b2e4eb")),
    ("tests/unit/cursor_temporal_boundary.rs", None),
];

pub(crate) fn exact_changed_paths(paths: &std::collections::BTreeSet<String>) -> bool {
    let expected=SUCCESSORS.iter().map(|(path,_)| (*path).to_owned()).chain([
        "ZIP_KIND.txt".to_owned(),
        "docs/selector_derivation_contract_cursor_temporal_check_r1446.md".to_owned(),
    ]).collect::<std::collections::BTreeSet<_>>();
    paths==&expected
}

pub(crate) fn row57_test_successor(bytes: &[u8]) -> bool {
    SUCCESSORS.iter().any(|(path,digest)| *path=="tests/support/render_authority/row57_local_clef_scope_through_trial.rs" && sha256::hex(bytes)==*digest)
}
