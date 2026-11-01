use super::*;
fn identity(executable:&str,input:&Path,output:&Path,variant:usize)->RendererProbeIdentity {
    let command=render_command_variants(&absolute_path(input),&absolute_path(output)).remove(variant);
    RendererProbeIdentity{executable:executable.to_owned(),command:format_command(executable,&command.args)}
}
#[test]
fn final_consumer_rebinds_only_declared_operands_in_actual_commands() {
    let a=Path::new("shadow directory/score.xml");let b=Path::new("shadow directory/score.pdf");
    let c=Path::new("final directory/score.xml");let d=Path::new("final directory/score.pdf");
    let before=identity("renderer",a,b,0);let after=identity("renderer",c,d,0);
    assert_ne!(before,after);
    assert!(before.same_consumer_for_bound_paths(&after,a,b,c,d));
    assert!(!before.same_consumer_for_bound_paths(&after,a,b,Path::new("foreign.xml"),d));
    assert!(!before.same_consumer_for_bound_paths(&after,Path::new("foreign.xml"),b,c,d));
    assert!(!before.same_consumer_for_bound_paths(&after,a,b,c,Path::new("foreign.pdf")));
}
#[test]
fn another_renderer_variant_or_unknown_command_cannot_reuse_physical_evidence() {
    let a=Path::new("input.xml");let b=Path::new("output.pdf");let before=identity("renderer",a,b,0);
    assert!(!before.same_consumer_for_bound_paths(&identity("other",a,b,0),a,b,a,b));
    assert!(!before.same_consumer_for_bound_paths(&identity("renderer",a,b,1),a,b,a,b));
    let unknown=RendererProbeIdentity{executable:"renderer".to_owned(),command:"renderer --unknown input.xml output.pdf".to_owned()};
    assert!(!before.same_consumer_for_bound_paths(&unknown,a,b,a,b));
}
