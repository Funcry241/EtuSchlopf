// Path-stable negative-check contract for the scanner primitive.
// Productive implementation ownership lives in tools/RUST_KODEX_GATE_CORE.
#[path = "../../tools/RUST_KODEX_GATE_CORE/src/rust_source_scan_primitives.rs"]
mod implementation;
#[path = "../../tools/RUST_KODEX_GATE_CORE/src/rust_source_fact_primitives.rs"]
mod source_facts;

// The implementation owner is byte-pinned by SourceIntegrityKodex. Re-export its
// complete pinned public base surface as one import so narrow compile units do not
// manufacture per-symbol unused-import warnings for owner capabilities they do not consume.
pub use implementation::*;
pub use source_facts::{
    collect_rust_files, line_number_at, line_starts, normalize_path, token_boundary_after,
    token_boundary_before,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a64_known_vectors_and_incremental_update_match() {
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(fnv1a64(b"foobar"), 0x8594_4171_f739_67e8);
        let partial = fnv1a64_update(FNV1A64_OFFSET_BASIS, b"foo");
        assert_eq!(fnv1a64_update(partial, b"bar"), fnv1a64(b"foobar"));
    }

    #[test]
    fn lexical_mask_preserves_length_and_line_boundaries() {
        let source = "fn f() {\r\n    // alpha\r\n    let text = \"beta\";\r\n}\r\n";
        let masked = mask_non_code(source);
        assert_eq!(masked.len(), source.len());
        assert_eq!(masked.matches('\n').count(), source.matches('\n').count());
        assert_eq!(masked.matches('\r').count(), source.matches('\r').count());
        assert!(masked.contains("fn f()"));
        assert!(!masked.contains("alpha"));
        assert!(!masked.contains("beta"));
    }

    #[test]
    fn nested_comments_strings_and_raw_strings_are_blank() {
        let source = r###"
            fn visible() {
                /* outer /* nested */ end */
                let normal = "comment // and /* marker */";
                let raw = r##"raw // and /* marker */"##;
                let bytes = br#"byte raw // marker"#;
                let c_text = c"c string // marker";
                let c_raw = cr#"c raw /* marker */"#;
                let r#type = 1;
                visible_call(r#type);
            }
        "###;
        let masked = mask_non_code(source);
        assert!(masked.contains("fn visible()"));
        assert!(masked.contains("let r#type = 1;"));
        assert!(masked.contains("visible_call(r#type);"));
        for hidden in [
            "outer",
            "nested",
            "comment //",
            "raw //",
            "byte raw",
            "c string",
            "c raw",
        ] {
            assert!(!masked.contains(hidden));
        }
    }

    #[test]
    fn character_literals_are_blank_but_lifetimes_and_labels_remain() {
        let source = "fn keep<'a, 'ä>(value: &'a str) -> &'a str { 'label: loop { let a = 'x'; let b = b'\\n'; let c = 'é'; let d = '\\x1b'; let e = '\\u{1b}'; break 'label value; } }";
        let masked = mask_non_code(source);
        assert!(masked.contains("fn keep<'a, 'ä>"));
        assert!(masked.contains("&'a str"));
        assert!(masked.contains("'label: loop"));
        assert!(masked.contains("break 'label value"));
        assert!(!masked.contains("'x'"));
        assert!(!masked.contains("b'\\n'"));
        assert!(!masked.contains("'é'"));
        assert!(!masked.contains("'\\x1b'"));
        assert!(!masked.contains("'\\u{1b}'"));
    }

    #[test]
    fn policy_free_source_facts_preserve_existing_scanner_semantics() {
        let starts = line_starts("alpha\nbeta\ngamma");
        assert_eq!(starts, vec![0, 6, 11]);
        assert_eq!(line_number_at(0, &starts), 1);
        assert_eq!(line_number_at(6, &starts), 2);
        assert_eq!(line_number_at(8, &starts), 2);
        assert_eq!(
            normalize_path(std::path::Path::new(r"src\build_gate\scan.rs")),
            "src/build_gate/scan.rs"
        );

        let bytes = b"alpha beta_2";
        assert!(token_boundary_before(bytes, 0));
        assert!(token_boundary_after(bytes, 5));
        assert!(!token_boundary_before(bytes, 2));
        assert!(!token_boundary_after(bytes, 8));

        let unique = format!(
            "etuschlopf-rust-source-facts-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock after epoch")
                .as_nanos()
        );
        let root = std::env::temp_dir().join(unique);
        let nested = root.join("nested");
        std::fs::create_dir_all(&nested).expect("create source-fact fixture");
        std::fs::write(root.join("root.rs"), "fn root() {}\n").expect("write root fixture");
        std::fs::write(nested.join("nested.rs"), "fn nested() {}\n").expect("write nested fixture");
        std::fs::write(nested.join("ignored.txt"), "ignored\n").expect("write ignored fixture");
        let mut files = Vec::new();
        collect_rust_files(&root, &mut files).expect("collect Rust source files");
        files.sort();
        let mut expected = vec![root.join("root.rs"), nested.join("nested.rs")];
        expected.sort();
        assert_eq!(files, expected);
        std::fs::remove_dir_all(root).expect("remove source-fact fixture");
    }

    #[test]
    fn malformed_or_unterminated_non_code_is_fail_closed_to_eof() {
        let string = mask_non_code("fn f() { let text = \"unterminated");
        assert!(string.starts_with("fn f() { let text = "));
        assert!(!string.contains("unterminated"));

        let comment = mask_non_code("fn f() { /* unterminated");
        assert!(comment.starts_with("fn f() { "));
        assert!(!comment.contains("unterminated"));

        let byte_character = mask_non_code("fn f() { let byte = b'x");
        assert!(byte_character.starts_with("fn f() { let byte = "));
        assert!(!byte_character.contains('x'));
    }
}
