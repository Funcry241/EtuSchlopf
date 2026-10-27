//! Exact parent-bound preparation of the existing #58 event before its sink.
//! No generic pin exception, musical selector or new writer authority.
use std::path::Path;
const MARKER: &str = "docs/final_notation_checkpoint_preparation_r1460.md";
const SUCCESSORS: &[(&str, &str)] = &[
    ("docs/final_notation_checkpoint_preparation_r1460.md", "40ca4fee8cd6ed4fecdd9c1eacc96122123a716db8a0a399d6008780eff9eb47"),
    ("docs/selector_derivation_contract_checkpoint58_preparation.md", "f53591c573286496130a2b74e2e37cca4af1fc38121662eb580424b77942f2af"),
    ("src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs", "8c8ee03bc4e84c8f2c503e4b4c34311efbf9f022059fe6deb66a9255b0bba193"),
    ("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs", "72a4088ddbdde09c7961e9be6d9026658ae117685a632f3fd3dff6379523801c"),
    ("src/structural_write_authority/final_notation_phrase_projection_transaction.rs", "2668d319715ba9434a0f165adb76087423b2a4616060c476b1c83af3a7982c28"),
    ("src/structural_write_authority/final_pdf_source_commit.rs", "2de96a90cb3cd02b864a5c69c02993cdc559832d1825e82ed2b5991a4afdbbca"),
    ("src/structural_write_authority/master_adoption/render_adopted_state.rs", "9f8bb3e6d95b0d8af3a2bf7829a1d32647e23d0cf9092b0a2f812e39821920c7"),
    ("src/structural_write_authority/master_adoption/render_adopted_state_tests.rs", "72f719c5e267b43d1b2b865af274da6f96c9fb127d2b6fce56dde63ac90267e1"),
    ("src/structural_write_authority/mod.rs", "810f90ed9d23ece0193fb17fdac5fcc929268298c053cd15beb367fe92051fbc"),
    ("src/structural_write_authority/orchestrator_artifact_sink_authority.rs", "469e9fea533c47196cbdef1bff8e20d56f3d2bdd6a71f9a7d5daa520cf0fe0e7"),
];
const PARENTS: &[(&str, &str)] = &[
    ("src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs", "1d2e7a03743139be98bb4dc5e84778d7e7ce2d49755a95032502d6c30f83e9b1"),
    ("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs", "c8e3ab28bb11574b5a72c00ff9e1cb038f3a97d413ece3e624528a645c1e28cd"),
    ("src/structural_write_authority/final_notation_phrase_projection_transaction.rs", "4f40c408bf01fdc0fe79e560a9acf9cbc0a6506f3f377ba97037fa0af840b111"),
    ("src/structural_write_authority/final_pdf_source_commit.rs", "c41307b755e0f0bce865ef34f1635671d0dbecbc440498bae95de5082d168840"),
    ("src/structural_write_authority/master_adoption/render_adopted_state.rs", "36705d704c0307e85347c6bbb1dbb69df2e53d8575ff53f79d71be0ad1f59c85"),
    ("src/structural_write_authority/master_adoption/render_adopted_state_tests.rs", "9a932b5f5badbcf758af43dc4562d512033cfee627a2642badd7eee5be1a1dc3"),
    ("src/structural_write_authority/mod.rs", "8683a04cd67a61b4c809126795756aa1fc2ae5a0cd1e8e49ab7aa9ed3dd5e13f"),
    ("src/structural_write_authority/orchestrator_artifact_sink_authority.rs", "9f2096067d0dcfa2eefe56551b006eb3ba7b25f7cb203d0ae5ce24e3e4900e4f"),
];
pub(crate) fn render_successor(bytes: &[u8]) -> bool { crate::sha256::hex(bytes)=="9f8bb3e6d95b0d8af3a2bf7829a1d32647e23d0cf9092b0a2f812e39821920c7" }
pub(crate) fn stage_successor(bytes: &[u8]) -> bool { crate::sha256::hex(bytes)=="72a4088ddbdde09c7961e9be6d9026658ae117685a632f3fd3dff6379523801c" }
fn installed(root: &Path) -> bool {
    root.join(MARKER).exists() || std::fs::read(root.join("src/structural_write_authority/master_adoption/render_adopted_state.rs")).is_ok_and(|bytes| render_successor(&bytes))
        || std::fs::read(root.join("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs")).is_ok_and(|bytes| stage_successor(&bytes))
}
fn read(root: &Path, path: &str) -> Result<Vec<u8>, String> {
    std::fs::read(root.join(path)).or_else(|error| {
        if path=="ZIP_KIND.txt" && error.kind()==std::io::ErrorKind::NotFound {
            std::fs::read(root.join("root_files/ZIP_KIND.txt"))
        } else { Err(error) }
    }).map_err(|error| format!("Checkpoint58 {path}: {error}"))
}
// User-authorized limited E007 transport prerequisite. Only these three exact
// implementations may succeed the accepted pins; every other #58 binding stays exact.
fn phrase_transport_successor(path: &str, digest: &str) -> bool {
    const EXACT: &[(&str, &str)] = &[
        ("src/structural_write_authority/mod.rs", "5d550e4ca788a368e740c65c76ff92d8194caf9bdba9638f5e4ed04c2d402b44"),
        ("src/structural_write_authority/final_notation_phrase_projection_transaction.rs", "c174c1d328c01d0628fd1675043bc856db5b735c96f29047017a89e01c899057"),
        ("src/structural_write_authority/final_pdf_source_commit.rs", "b6ca7c96cdaab00a246b18916c7c6a69463c00ad751f3a93102ef149a59cfc47"),
    ];
    EXACT.iter().any(|(bound_path, bound_digest)| path == *bound_path && digest == *bound_digest)
}
// The bridge companion requires these two prepared, reviewed byte images
// together. This is no editable checksum file or generic transport exception.
const BRIDGE_TRANSPORT_SUCCESSORS: &[(&str, &str)] = &[
    ("src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs", "b96623d3f09f026acd026aed1e22afb885b230b0b53b8344275763b1ba29a26b"),
    ("src/structural_write_authority/mod.rs", "b4035a8ad5c0b35e934e6aef38d737863238a7d4f19a2877a25d537bde98ed68"),
];
fn bridge_transport_pair(observed: &[(&str, &str)]) -> Result<bool, String> {
    let matches = BRIDGE_TRANSPORT_SUCCESSORS.iter().filter(|(path, digest)|
        observed.iter().any(|(actual_path, actual_digest)| path == actual_path && digest == actual_digest)).count();
    if matches == 0 { return Ok(false); }
    if matches != BRIDGE_TRANSPORT_SUCCESSORS.len() || observed.len() != BRIDGE_TRANSPORT_SUCCESSORS.len() {
        return Err("Checkpoint58 bridge transport requires the complete exact two-path successor".into());
    }
    Ok(true)
}

// Closed prerequisite for the required unchanged XML product. The separate
// RefactorOnly successor must match this complete reviewed implementation;
// no generic pin renewal or new Render origin is admitted.
const REQUIRED_PRODUCT_IMAGES: &[(&str, &str)] = &[
    ("src/structural_write_authority/master_adoption/render_adopted_state.rs", "5c5906371898fbe88a380546345e3f1967ba37c9beaca2defc8485f5220a9b09"),
    ("src/structural_write_authority/master_adoption/required_render_product.rs", "d0b85f406fc49e18329f398945c8b12a544ee750cc7970d8068a9a12202c8f31"),
    ("src/pipeline_shards/pipeline_orchestrator_release_report_exit_stage.rs", "f2f0744120ea8ef19cc2db4a12ce565c3802350e9dbc19d2beddca6d0788be87"),
    ("docs/selector_derivation_contract_required_render_product_implementation.md", "91d0feef6749a5b02be0f363428b2daaa7e440f185152276db4fbd790c5ed147"),
];
fn required_product_tuple_matches(observed: &[(&str,Option<&str>)]) -> bool {
    observed.len()==REQUIRED_PRODUCT_IMAGES.len() && REQUIRED_PRODUCT_IMAGES.iter().all(|(path,digest)|
        observed.iter().filter(|(p,_)|p==path).count()==1
        && observed.iter().any(|(p,d)|p==path && *d==Some(*digest)))
}
fn required_product_state(root: &Path) -> Result<bool, String> {
    let observed = REQUIRED_PRODUCT_IMAGES.iter().map(|(path,_)|
        writer_image(root,path).map(|digest|(*path,digest))).collect::<Result<Vec<_>,_>>()?;
    let signalled = observed.iter().zip(REQUIRED_PRODUCT_IMAGES).any(|((path,actual),(_,expected))|
        actual.as_deref()==Some(*expected) || (*path==REQUIRED_PRODUCT_IMAGES[1].0 && actual.is_some()));
    if !signalled { return Ok(false); }
    let refs=observed.iter().map(|(p,d)|(*p,d.as_deref())).collect::<Vec<_>>();
    if !required_product_tuple_matches(&refs) {
        return Err("RequiredRenderProduct requires its complete exact source-bound implementation; partial or drifted images are forbidden".into());
    }
    Ok(true)
}
fn required_product_bound_image(root: &Path, path: &str, digest: &str) -> Result<bool,String> {
    Ok(REQUIRED_PRODUCT_IMAGES.iter().any(|(p,d)|*p==path && *d==digest) && required_product_state(root)?)
}
fn required_product_transition(parent: &Path, child: &Path, class: &str) -> Result<(),String> {
    let before=required_product_state(parent)?;
    let after=required_product_state(child)?;
    if before && !after { return Err("RequiredRenderProduct admitted implementation cannot disappear".into()); }
    if !before && after && (class!="RefactorOnly"
        || !writer_identity(parent,"r1529","r1528","SscK2GovernanceRevisionKodex","KodexOnly")?
        || !writer_identity(child,"r1530","r1529","RequiredRenderProductRealization","RefactorOnly")?) {
        return Err("RequiredRenderProduct requires the exact admitted prerequisite parent and separate RefactorOnly successor".into());
    }
    Ok(())
}

pub(crate) fn validate_tree(root: &Path) -> Result<(), String> {
    if !installed(root) { return Ok(()); }
    let writer_retired = writer_retirement_state(root)?;
    let bridge_observed = BRIDGE_TRANSPORT_SUCCESSORS.iter().map(|(path,_)|
        Ok((*path, crate::sha256::hex(&read(root,path)?)))).collect::<Result<Vec<_>,String>>()?;
    let bridge_refs = bridge_observed.iter().map(|(path,digest)|(*path,digest.as_str())).collect::<Vec<_>>();
    let bridge_complete = if writer_retired { false } else { bridge_transport_pair(&bridge_refs)? };
    for (path, expected) in SUCCESSORS {
        let digest = crate::sha256::hex(&read(root,path)?);
        let bridge_bound = bridge_complete && BRIDGE_TRANSPORT_SUCCESSORS.iter()
            .any(|(bound_path,bound_digest)|path==bound_path && digest==*bound_digest);
        let writer_bound = writer_retired && WRITER_RETIREMENT_IMAGES.iter().any(|(p,_,after)|p==path && after.is_some_and(|sha|sha==digest));
        if digest != *expected && !phrase_transport_successor(path, &digest) && !bridge_bound && !writer_bound && !required_product_bound_image(root, path, &digest)? {
            return Err(format!("Checkpoint58 exact prepared successor changed: {path}"));
        }
    }
    Ok(())
}
pub(crate) fn applies(parent: &Path, child: &Path) -> bool {
    installed(parent) || installed(child)
}
pub(crate) fn validate_transition(parent: &Path, child: &Path, class: &str) -> Result<(), String> {
    required_product_transition(parent,child,class)?;
    writer_retirement_transition(parent,child,class)?;
    let before=installed(parent);let after=installed(child);
    if before && !after { return Err("Checkpoint58 prepared event cannot disappear".into()); }
    if !before && after {
        if class!="RefactorOnly" { return Err("Checkpoint58 requires exact RefactorOnly successor".into()); }
        let identity=String::from_utf8(read(parent,"ZIP_KIND.txt")?).map_err(|error| error.to_string())?;
        if identity!="kind=EtuSchlopfTake\nversion=0.10.1839\nrevision=r1459\nname=SscK2GovernanceRevisionKodex\nparent_revision=r1458\nchange_class=KodexOnly\n" {
            return Err("Checkpoint58 requires exact accepted r1459 parent identity".into());
        }
        let child_identity=String::from_utf8(read(child,"ZIP_KIND.txt")?).map_err(|error| error.to_string())?;
        if child_identity!="kind=EtuSchlopfTake\nversion=0.10.1839\nrevision=r1460\nname=FinalNotationCheckpointPrepareBeforeWrite\nparent_revision=r1459\nchange_class=RefactorOnly\n" {
            return Err("Checkpoint58 requires exact r1460 successor identity".into());
        }
        let declared=String::from_utf8(read(child,"src/build_gate/take_change_class.txt")?).map_err(|error| error.to_string())?;
        let paths=declared.lines().filter_map(|line| line.strip_prefix("change="))
            .filter_map(|line| line.split('|').nth(1)).map(str::to_owned).collect();
        if !exact_changed_paths(&paths) { return Err("Checkpoint58 complete changed-path set differs".into()); }
        for (path, expected) in PARENTS {
            if crate::sha256::hex(&read(parent,path)?)!=*expected {
                return Err(format!("Checkpoint58 original parent changed: {path}"));
            }
        }
        if parent.join(MARKER).exists() { return Err("Checkpoint58 marker already present".into()); }
    }
    validate_tree(child)
}
pub(crate) fn verified_first_transition(parent: &Path, child: &Path) -> Result<bool,String> {
    if installed(parent) || !installed(child) { return Ok(false); }
    validate_transition(parent,child,"RefactorOnly")?;
    Ok(true)
}
pub(crate) fn exact_changed_paths(paths: &std::collections::BTreeSet<String>) -> bool {
    let expected=SUCCESSORS.iter().map(|(path,_)| (*path).to_owned())
        .chain(["ZIP_KIND.txt".to_owned()]).collect();
    paths==&expected
}

const WRITER_RETIREMENT_IMAGES: &[(&str, Option<&str>, Option<&str>)] = &[
    ("ZIP_KIND.txt", Some("5a2d136c3d16503fa4fe44e3ffb03d59c25de0989496f352ed0301bacd5c0fa5"), Some("9e61f2e71efa67aa3c65144bbd573de1a28efd5ebc2396b70acdfdfebceb6702")),
    ("docs/four_beam_writer_retirement_r1487.md", None, Some("6872d752b5ce5448c66d7d1ad3f3310f10abadaa66bc93bc320bbb858bb037f7")),
    ("src/audit_only/notation_voice_pre_serialization_constraint_closure_audit.rs", Some("dc36b7eae7f169210510c058c3cdc2def0f29342adff9c4bc88ee83e080943cc"), Some("c096470040b83d86057852f8765719e11e961d9f50204baf94ba01dde0720647")),
    ("src/audit_only/writer_trace.rs", Some("c6fbee8957b3402ac351069c34120bdfe75c020b80a95c355bbfb23c4a0c5700"), Some("41213f6830d5190eb02b5c14e6e3a8aecf8299a36d754c9a58a0a1e78e63e06f")),
    ("src/build_gate/destructive_transformation_exclusions_v1.tsv", Some("a5b3fc6b277de4ff5aab555fc89e80f46f81bb2d379ea7c9d20712c9a1c2ee2b"), Some("8b9da465f15e0aa5c533508afb2b6c34f22df8e2e0be5baf192c8b4de7eaf89f")),
    ("src/build_gate/destructive_transformation_surface_baseline_v1.tsv", Some("2b4824a188c380c4fe03bd66a9c8628c564ed1b72b5fe67020d505100f548999"), Some("ed4d144dbdd424e2360d40b7c05b96fc02a9f203e3be77ee983aa573183a36de")),
    ("src/build_gate/render_legacy_selection_v1.tsv", Some("664b86a3dba3aa08fdfb5138eebc6122a1d5e126555bc6ffb119c72502c51032"), Some("6513dd0667439ed2e82a9a99ab03b6885e9744a15ddc011f9bb44cf2859962d9")),
    ("src/build_gate/render_origins_v1.tsv", Some("c2702e3971d92c247c5789a3750bb1a3c154011cd1133445fdc7f3c7f7e959c8"), Some("625d59e3ba5b4f0db25ada01ff6ec7d8c5076b610a424a33cebfda9917eeb767")),
    ("src/build_gate/ssc_containment_envelopes_v1.tsv", Some("535fcedabf3a867c28d732f15722cd843a26d219eb48ffdd157bbd25033825f9"), Some("bf48f878e4aafb71f257bbe5b599e01b9110d26125604525e16973246dea9c4c")),
    ("src/build_gate/ssc_proof_bundle_preserving_direction_contract_v1.txt", Some("d21581d8f3bb59d67be1e6225ad8c0ebd6b9959b56fc6010e2b4ce4b753b1355"), Some("bffcacd603c36eae35c082b90325fc0fb32866df23bf6fe1639880f299a73901")),
    ("src/master_state_identity.rs", Some("49a9757503c5ff6ede3ce7308f1bd07c013fcdf8f49f2264183a89979cbc8932"), Some("e3244aba7b97d3bc2baaaf9b6212469231ed5cc13b0b678bd5af8e553301b7ed")),
    ("src/master_state_identity/render_lineage.rs", Some("f06c4704e15a8a5bcdf133acd938274a7c0d72060a4a3a34f7cb7b8de605d628"), Some("5afa9340a021e5561806a6765bd8f7448e8ed66d6cfa76f969547c967c266928")),
    ("src/master_state_identity/tests.rs", Some("5f76922456d7ce8e10f2a9509a07363e912c890bb4a1cec628a6eec0d7e10899"), Some("0aafb3438d9019539f2ed6292b55929ba25d3eb40f878da3edd3e0c4a897c79d")),
    ("src/pipeline_shards/composer_beam_candidate_materialization_core.rs", Some("998def5996a6421a6e88695947eeb493f6bdef4208ced5daed7699315c97c064"), Some("23d6e97c2e4cf3efb96aa51fe833f40c5d8c10c261e778b1ccbb49814dfc4002")),
    ("src/pipeline_shards/composer_metric_beam_planning_core.rs", Some("05ab121313089f9cb0bb2e6965da0b54bd459396a5d8c73b993d77a6e26542a0"), Some("64e9cb5ac42c3b5ca59cb61bd2b479a98da00ef1377a31c2205b154530bc162c")),
    ("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", Some("a0d39f17e679050e3d524d2ee382800c9d7fc763cece3928d303e4dac923e9af"), Some("b02ec606893c2ab12c3431bca17eb7a516ad8e515397dcecb6b389195107c94f")),
    ("src/pipeline_shards/final_report_projection_owner.rs", Some("1d65b3e78bfc96db47bbbaf7ee9c5cc991b8bfcd8b792d8a0bd27ca30de5afe8"), Some("7ebf6a940617c1fb499fc5cda175fb641842a9c83dd53b26196863fbd9e5b7f0")),
    ("src/pipeline_shards/lower_dual_edge_beat_beam_continuity_repair.rs", Some("74de06586f3e902904e6aea64c5fe75ab2f96bb6a7ffbde9d839199f68befb56"), Some("c08fdb1307cdaf0fdcbbb664dc33d48d6fc71dcea553fd3fdf57a4b6c2476d35")),
    ("src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs", Some("b96623d3f09f026acd026aed1e22afb885b230b0b53b8344275763b1ba29a26b"), Some("45881d2ad13012e1a809eeab8fa1d06cda2b75024e441f16975ea9333f81c9b2")),
    ("src/pipeline_shards/pipeline_orchestrator_metric_final_notation_stage.rs", Some("85150dd6972d0f21e2660bd4fe99151f985893de270f4ee96a5b2724129154d2"), Some("b10cdbed4f48e880010499acc958b3de17ae1ae9621e72046d2b6703c4e41647")),
    ("src/pipeline_shards/pipeline_orchestrator_release_report_exit_stage.rs", Some("9bff3ee977aa56743b8e7b785db63c155cd3c473f76c7428f72413c764d8d142"), Some("fab6af93c2761a17e42f685f5a36df6a8e3fe981797ed6ca157869edca8ee908")),
    ("src/ssc_evidence_proof.rs", Some("5540cfc75db0f398ed3a3879c5ec35f688563248f9ed10daea354c0898436064"), Some("847b5dbd6cb0f4934c9dc5ade0eada97dc0aa375d797825adf519e7f8c6c0478")),
    ("src/structural_write_authority/beam_plan.rs", Some("b0c675533e51b066f172cfc74a8ae212123caff1af6795f275f7dde88654f5a3"), Some("c6603385567752a60c3bf611af52d625331d15a26560b9de5d6e3b9af97d8c46")),
    ("src/structural_write_authority/final_complete_beat_primary_beam_continuity.rs", Some("740df2ca65178dc8a9e7f8906e18b1f353b9547699021ae252fc4e51bbda7667"), None),
    ("src/structural_write_authority/gould_metric_beam_authorized_writer.rs", Some("d90127369670ce8aed2f958eea591a7056d66b3e7f9a9d3e731d88efc8dd9106"), None),
    ("src/structural_write_authority/gould_metric_beam_central_authorization.rs", Some("0249acf5d558c9f3f6bae08a02e493cce0205d3e07bf07c42195028c6ced3c0f"), None),
    ("src/structural_write_authority/gould_metric_beam_detector.rs", Some("a47b518ee079a98a833c85e1228cad1ec6462976278de2928541c7961ebbf8a4"), None),
    ("src/structural_write_authority/master_adoption/render_adopted_state.rs", Some("9f8bb3e6d95b0d8af3a2bf7829a1d32647e23d0cf9092b0a2f812e39821920c7"), Some("3d1cb04f1bc935ef1ef596d85b97855d32fff57f8780c5490d0c9355ca38f7ed")),
    ("src/structural_write_authority/master_adoption/render_adopted_state_tests.rs", Some("72f719c5e267b43d1b2b865af274da6f96c9fb127d2b6fce56dde63ac90267e1"), Some("6269191178e9aa9577cc7aae1760fcb3f4e35a61ae4376ff3cca18ef5f03b9a7")),
    ("src/structural_write_authority/materialization_verifier.rs", Some("16698cdab435fc425e320ca49680bde6a7a61e70d318e6537772bcd9ad9b2c50"), Some("d7c688ba68ab88fb765b23eaa110d1738c2e4aae05b20586016421cc0958a8ce")),
    ("src/structural_write_authority/metric_sovereign_caller_tests.rs", Some("7de7f9d1ca42d677e3a4ba95ce497a66f6f47dbc339c21373f6ef0742d326c28"), Some("65025ee9279e263c1cbe5272d3a2b79393ebd28bce9c836417baeb493667d200")),
    ("src/structural_write_authority/mod.rs", Some("b4035a8ad5c0b35e934e6aef38d737863238a7d4f19a2877a25d537bde98ed68"), Some("bd63d4abfc4821c026d075e5f3b35647a7ffb566f7c4a1650235bdd37631cb1d")),
    ("src/structural_write_authority/orchestrator_artifact_sink_authority.rs", Some("469e9fea533c47196cbdef1bff8e20d56f3d2bdd6a71f9a7d5daa520cf0fe0e7"), Some("6465337b6aa6f832f6f410ed6028550497c5837a06b874cb0ba2d468e5bca40b")),
    ("src/structural_write_authority/renderer_direction_relocation_writer_boundary.rs", Some("b5db2222a5d17bf3a505f3ba812acc2c82d14bc92bfc7791b85f09607fe0577d"), Some("406adcfedd2a641c48893d8daee65d770885fc7fd3a501ec0a020974c1f7b105")),
    ("src/structural_write_authority/ssc_proof_bundle_tests.rs", Some("af26683cce3c935f73d75abfde26ad5a959ddc459d86f0dbd51f4430416f5ef7"), Some("3cf8fb26d7b536b3dbcaf6620a1c3fe512aff04d0d558f64c676b7360d5422b8")),
    ("tests/support/bridge_frozen_voice_compatibility.rs", Some("922bc9fb9e3a5cace7eb1b165360e2dba4a33fe62ce1a17022e689f92fec6a60"), Some("f41d698a58caa0ccfebd86cc8e0bea08a4a4c2913bde20d53dc1f147c1875a08")),
    ("tests/support/render_authority/m1_real_lower_candidate_receipt_and_checkpoint_commit.rs", Some("039fe9c3eb4d1513bcd2e1bb679766afa1d86eeb36a48b81e8dd0d9f98a32695"), None),
    ("tests/support/render_authority/m3_actual_closure_receipt_and_checkpoint_commit.rs", Some("cc7e3b5641ab5aebb886c8d7bd0ec28be637ff8f6a5c7fc1dfe33437a4870c42"), None),
    ("tests/support/same_beat_existing_beam_edge_protection.rs", Some("52b29488e032e24cbed5e898cfb4f739d0d32c5a673455908a5e54273f97af4d"), Some("cdd14a6ea760f1dcd02bd0698f2d26b10af4e2c9a959e261aef572fc853997f7")),
 ];
const WRITER_RETIREMENT_ANCHOR_LINES: &[(&str, &str)] = &[
    ("const _: () = assert!(RENDER_ORIGINS_V1_FNV1A64 == 0x1a478383a778765a || RENDER_ORIGINS_V1_FNV1A64 == 0x81e20fee4cbb052b);", "const _: () = assert!(RENDER_ORIGINS_V1_FNV1A64 == 0xc0dbba030b26e185);"),
    ("const _: () = assert!(RENDER_LEGACY_SELECTION_V1_FNV1A64 == 0x3809ad499de8a062);", "const _: () = assert!(RENDER_LEGACY_SELECTION_V1_FNV1A64 == 0x5621ccf7cc7b1977);"),
    ("const _: () = assert!(DESTRUCTIVE_TRANSFORMATION_EXCLUSIONS_V1_FNV1A64 == 0xc8fefaa66e688b64);", "const _: () = assert!(DESTRUCTIVE_TRANSFORMATION_EXCLUSIONS_V1_FNV1A64 == 0x2fa140d9eaee8d66);"),
    ("const _: () = assert!(DESTRUCTIVE_TRANSFORMATION_SURFACE_BASELINE_V1_FNV1A64 == 0x0bb8e11dacf3320d);", "const _: () = assert!(DESTRUCTIVE_TRANSFORMATION_SURFACE_BASELINE_V1_FNV1A64 == 0xcc02d2ddd91aa3dc);"),
    ("const _: () = assert!(fnv1a64(K2_PROOF_BUNDLE_CONTRACT_BYTES) == 0xfa91f33dcf980c19);", "const _: () = assert!(fnv1a64(K2_PROOF_BUNDLE_CONTRACT_BYTES) == 0x882ee2ee2c665196);"),
 ];
const WRITER_RETIREMENT_ANCHOR: &str = "src/identity_gate_trust_anchor.rs";
fn writer_image(root: &Path, path: &str) -> Result<Option<String>, String> {
    match std::fs::read(root.join(path)) {
        Ok(bytes) => Ok(Some(crate::sha256::hex(&bytes))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("Checkpoint58 writer retirement {path}: {error}")),
    }
}
fn writer_tuple_matches(observed: &[(&str,Option<&str>)], successor: bool) -> bool {
    let expected=WRITER_RETIREMENT_IMAGES.iter().filter(|(p,_,_)|*p!="ZIP_KIND.txt").collect::<Vec<_>>();
    observed.len()==expected.len() && expected.iter().all(|(path,before,after)|
        observed.iter().filter(|(p,_)|p==path).count()==1
        && observed.iter().any(|(p,sha)|p==path && sha==if successor { after } else { before }))
}
fn writer_images_match(root: &Path, successor: bool) -> Result<bool, String> {
    let observed=WRITER_RETIREMENT_IMAGES.iter().filter(|(p,_,_)|*p!="ZIP_KIND.txt")
        .map(|(p,_,_)|Ok((*p,writer_image(root,p)?))).collect::<Result<Vec<_>,String>>()?;
    let refs=observed.iter().map(|(p,sha)|(*p,sha.as_deref())).collect::<Vec<_>>();
    Ok(writer_tuple_matches(&refs,successor))
}
// r1488: tokens of the retired authority (sinks, branch/fork, mints, descriptors,
// entry points). After installation they must stay absent from every runtime source.
const WRITER_RETIREMENT_ABSENT: &[&str] = &[
    "StructuralLowerBeam", "LowerBeeQueenBeam", "GouldMetricBeamCommit", "FinalCompleteBeat", "FinalNotationB2",
    "FinalCompleteBeatPreservingMint", "GouldMetricBeamPreservingMint", "ScorewideMetricBeamPreservingMint",
    "StructuralBeamClosurePreservingMint", "FINAL_COMPLETE_BEAT_PRESERVING", "GOULD_METRIC_BEAM_PRESERVING",
    "SCOREWIDE_METRIC_BEAM_PRESERVING", "STRUCTURAL_BEAM_CLOSURE_PRESERVING",
    "materialize_final_complete_beat_primary_beam_plan_authorized", "apply_authorized_gould_metric_beam_write",
    "commit_verified_gould_metric_beam_candidate", "build_gould_central_write_authority_cutover",
    "resolve_scorewide_metric_beam_continuity_candidates", "materialize_lower_metric_beam_continuity_candidate",
    "close_structural_beam_gestures", "write_structural_lower_beam_checkpoint_authorized",
    "write_lower_bee_queen_beam_checkpoint_authorized",
];
// Paths whose retired image stays exact after installation: the six closed kodex data
// tables and the files that were already exact Checkpoint58 successors before r1486.
fn writer_retirement_exact_after_installation(path: &str) -> bool {
    path.starts_with("src/build_gate/") || SUCCESSORS.iter().any(|(p,_)|*p==path)
}
fn writer_installed_images_match(root: &Path) -> Result<bool, String> {
    for (path, _, after) in WRITER_RETIREMENT_IMAGES {
        if *path == "ZIP_KIND.txt" || path.starts_with("docs/") { continue; }
        let actual = writer_image(root,path)?;
        let bound = after.is_none() || writer_retirement_exact_after_installation(path);
        if bound && actual.as_deref() != *after {
            let product_bound = match actual.as_deref() {
                Some(digest) => required_product_bound_image(root,path,digest)?,
                None => false,
            };
            if !product_bound { return Ok(false); }
        }
    }
    Ok(true)
}
pub(crate) fn writer_retirement_state(root: &Path) -> Result<bool, String> {
    // Content defines the state; metadata alone never grants authority. The signal is
    // unchanged from r1486: any exact retired source/data image. The six closed kodex
    // tables keep that image permanently, so the signal survives later living changes.
    let mut signalled = false;
    for (path, before, after) in WRITER_RETIREMENT_IMAGES {
        if path.starts_with("src/") && before != after && after.is_some()
            && writer_image(root,path)?.as_deref() == *after { signalled = true; break; }
    }
    if !signalled { return Ok(false); }
    // r1488: the installed retirement binds the closed kodex data, the six absences and the
    // earlier Checkpoint58 successors exactly, and keeps the retired authority absent. Other
    // living runtime files are no longer frozen to their r1487 bytes; their ordinary gates apply.
    // The first transition r1486 -> r1487 still demands the complete tuple (see below).
    if !writer_installed_images_match(root)? {
        return Err("Checkpoint58 writer retirement requires the exact retired kodex data, absences and Checkpoint58 successors; partial/mixed/drifted images are forbidden".into());
    }
    super::absent(&super::runtime_sources(root)?, WRITER_RETIREMENT_ABSENT)?;
    Ok(true)
}
fn writer_anchor_successor(before: &str, after: &str) -> bool {
    let mut expected = before.to_owned();
    for (old, new) in WRITER_RETIREMENT_ANCHOR_LINES {
        // Match complete lines, once each. Unrelated literals and every other byte
        // remain parent-owned, including all validator pins installed by r1486.
        let old = format!("{old}\n");
        if expected.lines().filter(|line| *line == old.trim_end_matches('\n')).count()!=1 { return false; }
        expected = expected.replacen(&old, &format!("{new}\n"), 1);
    }
    expected == after
}
fn writer_identity(root: &Path, revision: &str, parent: &str, name: &str, class: &str) -> Result<bool,String> {
    Ok(read(root,"ZIP_KIND.txt")? == format!("kind=EtuSchlopfTake\nversion=0.10.1839\nrevision={revision}\nname={name}\nparent_revision={parent}\nchange_class={class}\n").as_bytes())
}
pub(crate) fn writer_retirement_transition(parent: &Path, child: &Path, class: &str) -> Result<bool,String> {
    let before = writer_retirement_state(parent)?;
    let after = writer_retirement_state(child)?;
    if before && !after { return Err("Checkpoint58 writer retirement cannot revert".into()); }
    if before { return Ok(false); }
    if !after { return Ok(false); }
    if !writer_images_match(child,true)? {
        return Err("Checkpoint58 writer retirement requires the complete exact successor; partial/mixed/drifted images are forbidden".into());
    }
    if class != "GovernedChange" || !writer_images_match(parent,false)?
        || !writer_identity(parent,"r1486","r1485","SscK2GovernanceRevisionKodex","KodexOnly")?
        || !writer_identity(child,"r1487","r1486","FourBeamWriterRetirement","GovernedChange")? {
        return Err("Checkpoint58 writer retirement requires the exact r1486 parent and GovernedChange r1487 child".into());
    }
    let before = String::from_utf8(read(parent,WRITER_RETIREMENT_ANCHOR)?).map_err(|e|e.to_string())?;
    let after = String::from_utf8(read(child,WRITER_RETIREMENT_ANCHOR)?).map_err(|e|e.to_string())?;
    if !writer_anchor_successor(&before,&after) { return Err("Checkpoint58 writer retirement anchor must be the exact parent-derived successor".into()); }
    writer_retirement_manifest_paths(child)?;
    Ok(true)
}
pub(crate) fn writer_retirement_bound_path(parent: &Path, child: &Path, path: &str, class: &str) -> bool {
    (path == WRITER_RETIREMENT_ANCHOR || WRITER_RETIREMENT_IMAGES.iter().any(|(p,_,_)|*p==path))
        && writer_retirement_transition(parent,child,class)==Ok(true)
}
pub(crate) fn writer_retirement_kodex_surface(path: &str) -> bool {
    // The Take path gate must independently prove the complete parent-bound
    // transition before any of these data surfaces can pass semantic diff policy.
    WRITER_RETIREMENT_IMAGES.iter().any(|(p,_,_)|*p==path && p.starts_with("src/build_gate/"))
}

fn writer_retirement_manifest_paths(root: &Path) -> Result<(),String> {
    let manifest=String::from_utf8(read(root,"src/build_gate/take_change_class.txt")?).map_err(|e|e.to_string())?;
    let paths=manifest.lines().filter_map(|line|line.strip_prefix("change="))
        .filter_map(|line|line.split('|').nth(1)).collect::<std::collections::BTreeSet<_>>();
    let expected=WRITER_RETIREMENT_IMAGES.iter().map(|(p,_,_)|*p)
        .chain([WRITER_RETIREMENT_ANCHOR]).collect::<std::collections::BTreeSet<_>>();
    if paths != expected { return Err("Checkpoint58 writer retirement complete changed-path set differs".into()); }
    Ok(())
}
pub(crate) fn writer_retirement_installed_manifest(root: &Path, class: &str) -> Result<bool,String> {
    if class!="GovernedChange" || !writer_identity(root,"r1487","r1486","FourBeamWriterRetirement","GovernedChange")? { return Ok(false); }
    if !writer_retirement_state(root)? { return Ok(false); }
    writer_retirement_manifest_paths(root)?;
    // Installed hashes are checked by TakeChangeControl first. The exact anchor
    // transition was admitted by the running parent before transactional install;
    // the compiled anchor independently enforces the sole final pin values.
    Ok(true)
}

#[cfg(test)]
mod bridge_transport_tests {
    use super::*;
    #[test]
    fn exact_transport_pair_is_closed_and_order_independent() {
        assert_eq!(bridge_transport_pair(BRIDGE_TRANSPORT_SUCCESSORS), Ok(true));
        let mut reversed = BRIDGE_TRANSPORT_SUCCESSORS.to_vec(); reversed.reverse();
        assert_eq!(bridge_transport_pair(&reversed), Ok(true));
    }
    #[test]
    fn retired_unconditional_render_read_image_is_not_admitted() {
        let mut retired=BRIDGE_TRANSPORT_SUCCESSORS.to_vec();
        retired[0].1="6efa601f56f8b8948bd99e723318d8a222fa4ec4311fbfc3b21a24298210c06b";
        assert!(bridge_transport_pair(&retired).is_err());
    }
    #[test]
    fn unchanged_parent_is_not_promoted_to_a_transport_successor() {
        let old = BRIDGE_TRANSPORT_SUCCESSORS.iter().map(|(path,_)|(*path,"parent-bytes")).collect::<Vec<_>>();
        assert_eq!(bridge_transport_pair(&old), Ok(false));
    }
    #[test]
    fn one_sided_and_drifted_transport_are_rejected() {
        for omitted in 0..BRIDGE_TRANSPORT_SUCCESSORS.len() {
            let partial = BRIDGE_TRANSPORT_SUCCESSORS.iter().enumerate()
                .filter(|(index,_)|*index!=omitted).map(|(_,row)|*row).collect::<Vec<_>>();
            assert!(bridge_transport_pair(&partial).is_err());
            let mut drifted = BRIDGE_TRANSPORT_SUCCESSORS.to_vec(); drifted[omitted].1="changed-bytes";
            assert!(bridge_transport_pair(&drifted).is_err());
        }
    }
    #[test]
    fn copied_digest_at_a_foreign_path_cannot_complete_the_pair() {
        let mut foreign = BRIDGE_TRANSPORT_SUCCESSORS.to_vec(); foreign[0].0="src/foreign.rs";
        assert!(bridge_transport_pair(&foreign).is_err());
        foreign[1].0="src/another.rs";
        assert_eq!(bridge_transport_pair(&foreign), Ok(false));
    }
    #[test]
    fn extra_or_duplicate_rows_do_not_expand_the_two_path_admission() {
        let mut extra = BRIDGE_TRANSPORT_SUCCESSORS.to_vec(); extra.push(("src/foreign.rs","foreign"));
        assert!(bridge_transport_pair(&extra).is_err());
        let duplicate = [BRIDGE_TRANSPORT_SUCCESSORS[0],BRIDGE_TRANSPORT_SUCCESSORS[0]];
        assert!(bridge_transport_pair(&duplicate).is_err());
    }
    #[test]
    fn other_checkpoint_bindings_and_original_event_pins_remain_distinct() {
        assert!(BRIDGE_TRANSPORT_SUCCESSORS.iter().all(|(path,_)|
            !path.contains("render_adopted_state") && !path.contains("initial_render")));
        assert!(!render_successor(BRIDGE_TRANSPORT_SUCCESSORS[0].1.as_bytes()));
        assert!(!stage_successor(BRIDGE_TRANSPORT_SUCCESSORS[1].1.as_bytes()));
        assert!(BRIDGE_TRANSPORT_SUCCESSORS.iter().all(|(path,digest)| !phrase_transport_successor(path,digest)));
    }
    #[test]
    fn real_parent_tree_passes_and_every_existing_checkpoint_pin_stays_closed() {
        let parent = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = std::env::temp_dir().join(format!("bridge-checkpoint-pins-{}",std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (path,_) in SUCCESSORS {
            let file = root.join(path); std::fs::create_dir_all(file.parent().unwrap()).unwrap();
            std::fs::write(&file,read(parent,path).unwrap()).unwrap();
        }
        if writer_retirement_state(parent).unwrap() {
            for (path,_,after) in WRITER_RETIREMENT_IMAGES {
                if after.is_some() {
                    let file=root.join(path);std::fs::create_dir_all(file.parent().unwrap()).unwrap();
                    std::fs::write(file,read(parent,path).unwrap()).unwrap();
                }
            }
        }
        if required_product_state(parent).unwrap() {
            for (path,_) in REQUIRED_PRODUCT_IMAGES {
                let file=root.join(path);std::fs::create_dir_all(file.parent().unwrap()).unwrap();
                std::fs::write(file,read(parent,path).unwrap()).unwrap();
            }
        }
        assert!(validate_tree(&root).is_ok());
        for (path,_) in SUCCESSORS {
            let file = root.join(path); let original = std::fs::read(&file).unwrap();
            let mut altered = original.clone(); altered.push(b' ');
            std::fs::write(&file,altered).unwrap();
            assert!(validate_tree(&root).is_err(),"{path}");
            std::fs::write(&file,original).unwrap();
        }
        assert!(validate_tree(&root).is_ok());
        std::fs::remove_dir_all(root).unwrap();
    }
}

// r1486: one parent-reviewed retirement of four beam writers. This extends the
// existing Checkpoint58 exact successor set, not a configurable admission registry.

#[cfg(test)]
mod writer_retirement_tests {
    use super::*;
    fn observed(successor: bool) -> Vec<(&'static str,Option<&'static str>)> {
        WRITER_RETIREMENT_IMAGES.iter().filter(|(p,_,_)|*p!="ZIP_KIND.txt")
            .map(|(p,b,a)|(*p,if successor {*a} else {*b})).collect()
    }
    #[test]
    fn complete_old_and_new_tuples_are_distinct_and_order_independent() {
        for retired in [false,true] {
            let mut rows=observed(retired);
            assert!(writer_tuple_matches(&rows,retired));
            assert!(!writer_tuple_matches(&rows,!retired));
            rows.reverse();assert!(writer_tuple_matches(&rows,retired));
        }
    }
    #[test]
    fn every_partial_mixed_and_drifted_successor_is_rejected() {
        let target=observed(true);
        let parent=observed(false);
        for index in 0..target.len() {
            let mut rows=target.clone();rows.remove(index);assert!(!writer_tuple_matches(&rows,true));
            let mut rows=target.clone();rows[index]=parent[index];assert!(!writer_tuple_matches(&rows,true));
            let mut rows=target.clone();rows[index].1=Some("drifted");assert!(!writer_tuple_matches(&rows,true));
        }
    }
    #[test]
    fn foreign_duplicate_extra_paths_and_restored_deleted_files_are_rejected() {
        let target=observed(true);
        for index in 0..target.len() {
            let mut rows=target.clone();rows[index].0="src/foreign.rs";assert!(!writer_tuple_matches(&rows,true));
            let mut rows=target.clone();rows.push(target[index]);assert!(!writer_tuple_matches(&rows,true));
            let mut rows=target.clone();rows[index]=target[(index+1)%target.len()];assert!(!writer_tuple_matches(&rows,true));
        }
        assert_eq!(target.iter().filter(|(_,sha)|sha.is_none()).count(),6);
    }
    fn anchor_pair() -> (String,String) {
        let before=format!("// preserved checker pin 0x123456789abcdef0\n{}\n",WRITER_RETIREMENT_ANCHOR_LINES.iter().map(|(a,_)|*a).collect::<Vec<_>>().join("\n"));
        let after=format!("// preserved checker pin 0x123456789abcdef0\n{}\n",WRITER_RETIREMENT_ANCHOR_LINES.iter().map(|(_,b)|*b).collect::<Vec<_>>().join("\n"));
        (before,after)
    }
    #[test]
    fn anchor_allows_only_the_complete_named_parent_derived_rebinding() {
        let (before,after)=anchor_pair();assert!(writer_anchor_successor(&before,&after));
        assert!(!writer_anchor_successor(&before,&before));
        assert!(!writer_anchor_successor(&after,&before));
        assert!(!writer_anchor_successor(&before,&after.replace("123456789abcdef0","0000000000000000")));
        assert!(!writer_anchor_successor(&before,&(after.clone()+"// extra\n")));
        assert!(!writer_anchor_successor(&(before.clone()+WRITER_RETIREMENT_ANCHOR_LINES[0].0+"\n"),&after));
        for (old,new) in WRITER_RETIREMENT_ANCHOR_LINES {
            assert!(!writer_anchor_successor(&before,&after.replace(new,old)));
        }
    }
}


// r1488: the installed retirement keeps the retired authority closed without freezing
// the surviving living runtime files to their r1487 bytes.
#[cfg(test)]
mod writer_retirement_installed_tests {
    use super::*;
    const LIVING: &[&str] = &[
        "src/pipeline_shards/pipeline_orchestrator_metric_final_notation_stage.rs",
        "src/pipeline_shards/composer_beam_candidate_materialization_core.rs",
        "src/audit_only/writer_trace.rs",
        "tests/support/bridge_frozen_voice_compatibility.rs",
    ];
    fn installed_tree(tag: &str) -> std::path::PathBuf {
        let parent = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = std::env::temp_dir().join(format!("writer-retirement-installed-{tag}-{}",std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        // Copy the complete current contract into the synthetic installed tree;
        // the product successor adds bound owner files outside the old tables.
        let product_paths = if required_product_state(parent).unwrap() { REQUIRED_PRODUCT_IMAGES } else { &[] };
        let paths = WRITER_RETIREMENT_IMAGES.iter().filter(|(_,_,after)|after.is_some()).map(|(p,_,_)|*p)
            .chain(SUCCESSORS.iter().map(|(p,_)|*p))
            .chain(product_paths.iter().map(|(p,_)|*p));
        for path in paths {
            let file=root.join(path);std::fs::create_dir_all(file.parent().unwrap()).unwrap();
            std::fs::write(file,read(parent,path).unwrap()).unwrap();
        }
        root
    }
    fn append(root: &Path, path: &str, text: &str) {
        let mut bytes=std::fs::read(root.join(path)).unwrap(); bytes.extend_from_slice(text.as_bytes());
        std::fs::write(root.join(path),bytes).unwrap();
    }
    #[test]
    fn installed_tree_is_retired_and_valid() {
        let root=installed_tree("valid");
        assert_eq!(writer_retirement_state(&root), Ok(true));
        assert!(validate_tree(&root).is_ok());
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn living_runtime_files_may_change_after_installation() {
        for (index, path) in LIVING.iter().enumerate() {
            let root=installed_tree(&format!("living{index}"));
            append(&root,path,"\n// later reviewed change of a surviving owner\nconst _LATER_LIVING_CHANGE: u8 = 0;\n");
            assert_eq!(writer_retirement_state(&root), Ok(true), "{path}");
            assert!(validate_tree(&root).is_ok(), "{path}");
            // The historical first transition still requires the complete r1487 tuple.
            assert_eq!(writer_images_match(&root,true), Ok(false), "{path}");
            std::fs::remove_dir_all(root).unwrap();
        }
    }
    #[test]
    fn reintroduced_retired_authority_is_rejected_but_comments_are_not_authority() {
        for (index, token) in WRITER_RETIREMENT_ABSENT.iter().enumerate() {
            let root=installed_tree(&format!("token{index}"));
            append(&root,LIVING[0],&format!("\n// historical note: {token}\n"));
            assert_eq!(writer_retirement_state(&root), Ok(true), "{token}");
            append(&root,LIVING[0],&format!("\n#[allow(dead_code)]\nfn reintroduced() {{ let _ = stringify!(x); let {token} = 0; let _ = {token}; }}\n"));
            assert!(writer_retirement_state(&root).is_err(), "{token}");
            assert!(validate_tree(&root).is_err(), "{token}");
            std::fs::remove_dir_all(root).unwrap();
        }
    }
    #[test]
    fn restored_files_drifted_kodex_data_and_checkpoint_successors_are_rejected() {
        let bound = WRITER_RETIREMENT_IMAGES.iter()
            .filter(|(p,_,_)|*p!="ZIP_KIND.txt" && !p.starts_with("docs/"))
            .filter(|(p,_,after)|after.is_none() || writer_retirement_exact_after_installation(p))
            .collect::<Vec<_>>();
        assert_eq!(bound.iter().filter(|(_,_,after)|after.is_none()).count(), 6);
        assert_eq!(bound.iter().filter(|(p,_,_)|p.starts_with("src/build_gate/")).count(), 6);
        assert_eq!(bound.iter().filter(|(p,_,after)|after.is_some() && SUCCESSORS.iter().any(|(s,_)|s==p)).count(), 5);
        for (index,(path,_,after)) in bound.iter().enumerate() {
            let root=installed_tree(&format!("bound{index}"));
            if after.is_none() {
                let file=root.join(path);std::fs::create_dir_all(file.parent().unwrap()).unwrap();
                std::fs::write(file,"// restored\n").unwrap();
            } else {
                append(&root,path,"\n");
            }
            assert!(writer_retirement_state(&root).is_err() || validate_tree(&root).is_err(), "{path}");
            std::fs::remove_dir_all(root).unwrap();
        }
    }
}

#[cfg(test)]
mod required_product_contract_tests {
    use super::*;
    #[test]
    fn required_product_accepts_only_complete_exact_tuple_in_any_order() {
        let mut tuple=REQUIRED_PRODUCT_IMAGES.iter().map(|(p,d)|(*p,Some(*d))).collect::<Vec<_>>();
        assert!(required_product_tuple_matches(&tuple));tuple.reverse();
        assert!(required_product_tuple_matches(&tuple));
    }
    #[test]
    fn required_product_rejects_partial_mixed_foreign_duplicate_or_extra_images() {
        let tuple=REQUIRED_PRODUCT_IMAGES.iter().map(|(p,d)|(*p,Some(*d))).collect::<Vec<_>>();
        for i in 0..tuple.len() {
            let mut partial=tuple.clone();partial.remove(i);assert!(!required_product_tuple_matches(&partial));
            let mut drift=tuple.clone();drift[i].1=Some("drift");assert!(!required_product_tuple_matches(&drift));
            drift[i].1=None;assert!(!required_product_tuple_matches(&drift));
            let mut wrong_path=tuple.clone();wrong_path[i].0="foreign.rs";assert!(!required_product_tuple_matches(&wrong_path));
            let mut duplicate=tuple.clone();duplicate[i]=tuple[(i+1)%tuple.len()];assert!(!required_product_tuple_matches(&duplicate));
        }
        let mut extra=tuple.clone();extra.push(tuple[0]);assert!(!required_product_tuple_matches(&extra));
        assert!(!required_product_tuple_matches(&[]));
    }
}
