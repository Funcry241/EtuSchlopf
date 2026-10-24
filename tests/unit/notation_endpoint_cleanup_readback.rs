use super::*;

#[test]
fn endpoint_fragment_scope_rejects_sounding_nonempty_malformed_and_foreign_nodes() {
    for fragment in ["<tie type=\"start\"/>", "<tiedness/>",
        "<notations><tied type=\"stop\"/></notations>",
        "<tied type=\"stop\">payload</tied>", "<tied type=\"stop\"></tied>",
        "<slur/>text", "<!-- <slur/> -->", "<tied type=\"stop\"/><tie type=\"start\"/>",
        "<notations><articulations><staccato/></articulations></notations>",
        "<tied xmlns=\"foreign\"/>", "<tied type=\"stop\"", " "] {
        assert!(!is_notation_endpoint_fragment(fragment), "{fragment}");
    }
    for fragment in ["<tied type=\"start\"/>", "\n<slur type='stop' number='opaque'/> \n",
        "<notations>\n </notations>", "<tied type='stop'/><slur type='start'/>"] {
        assert!(is_notation_endpoint_fragment(fragment), "{fragment}");
    }
}
