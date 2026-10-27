use super::*;

const MINIMAL_PARTWISE_40: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
<score-part id="P1">
  <part-name>Piano</part-name>
</score-part>
  </part-list>
  <part id="P1">
    <measure number="{ordinal}"/>
  </part>
</score-partwise>"#;

const MINIMAL_PARTWISE_31: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="3.1">
  <part-list>
<score-part id="P1">
  <part-name>Piano</part-name>
</score-part>
  </part-list>
  <part id="P1">
    <measure number="{ordinal}"/>
  </part>
</score-partwise>"#;

struct Products {
    root: PathBuf,
    paths: RegisteredArtifacts,
}

fn fixture(template: &[u8], ordinal: usize) -> Vec<u8> {
    std::str::from_utf8(template)
        .unwrap()
        .replace("{ordinal}", &ordinal.to_string())
        .into_bytes()
}

impl Products {
    fn new(ordinal: usize) -> Self {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "persisted-product-order-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&root).unwrap();
        let paths = RegisteredArtifacts {
            master: root.join("piece.musicxml"),
            render: root.join("piece.render.musicxml"),
            pdf: root.join("piece.pdf"),
        };
        fs::write(&paths.master, fixture(MINIMAL_PARTWISE_40, ordinal)).unwrap();
        Self { root, paths }
    }
}

impl Drop for Products {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn selected_master_can_render_before_required_product_but_cannot_release_without_it() {
    for ordinal in [1, 19, 317] {
        let f = Products::new(ordinal);
        let source_gate = validate_renderer_source(&f.paths, &f.paths.master, &f.paths.pdf)
            .expect("valid selected Master precedes final unchanged product realization");
        assert!(source_gate.contains("CompleteProductGate=pending_final_product_closure"));
        assert!(!f.paths.render.exists());
        assert!(
            validate_registered(&f.paths)
                .unwrap_err()
                .contains("render-MusicXML fehlt")
        );
        fs::write(&f.paths.render, fixture(MINIMAL_PARTWISE_40, ordinal)).unwrap();
        assert!(
            validate_registered(&f.paths)
                .unwrap()
                .summary_line()
                .contains("GREEN")
        );
        fs::write(&f.paths.render, fixture(MINIMAL_PARTWISE_31, ordinal)).unwrap();
        assert!(validate_registered(&f.paths).is_err());
        fs::remove_file(&f.paths.render).unwrap();
        assert!(validate_registered(&f.paths).is_err());
    }
}

#[test]
fn renderer_source_gate_rejects_missing_wrong_version_invalid_and_unregistered_sources() {
    let ordinal = 73;
    let f = Products::new(ordinal);
    assert!(validate_renderer_source(&f.paths, &f.paths.render, &f.paths.pdf).is_err());
    for invalid in [
        fixture(MINIMAL_PARTWISE_31, ordinal),
        b"<score-partwise version=\"4.0\"/>".to_vec(),
    ] {
        fs::write(&f.paths.master, invalid).unwrap();
        assert!(validate_renderer_source(&f.paths, &f.paths.master, &f.paths.pdf).is_err());
    }
    fs::write(&f.paths.master, fixture(MINIMAL_PARTWISE_40, ordinal)).unwrap();
    assert!(
        validate_renderer_source(&f.paths, &f.root.join("other.musicxml"), &f.paths.pdf).is_err()
    );
    assert!(
        validate_renderer_source(&f.paths, &f.paths.master, &f.root.join("other.pdf")).is_err()
    );
    fs::write(&f.paths.render, fixture(MINIMAL_PARTWISE_40, ordinal)).unwrap();
    assert!(validate_renderer_source(&f.paths, &f.paths.render, &f.paths.pdf).is_ok());
}
