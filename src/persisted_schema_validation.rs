//! Persisted-byte MusicXML 4.0 product-contract boundary.
//!
//! Gate A belongs to EtuSchlopf and proves the declared product serialization contract on the
//! canonical bytes re-read after persistence. Gate B belongs to SchemaSchlopf r021 and validates
//! exactly the same bytes against the bound MusicXML 4.0 XSD profile. Neither gate receives writer,
//! repair, retry, musical-decision, or network authority.

use roxmltree::Document;
use schemaschlopf::{
    BoundInstanceValidator, MUSICXML40_SCHEMA_DIR, compile_bound_instance_validator,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const EXPECTED_ROOT_LOCAL_NAME: &str = "score-partwise";
const EXPECTED_PRODUCT_VERSION: &str = "4.0";
const SCHEMASCHLOPF_REVISION: &str = "r021";

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisteredArtifacts {
    master: PathBuf,
    render: PathBuf,
    pdf: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ValidatedArtifact {
    label: &'static str,
    path: PathBuf,
    bytes: usize,
    declared_version: String,
    root_local_name: String,
    element_count: usize,
    attribute_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PersistedSchemaValidationReport {
    master: ValidatedArtifact,
    render: ValidatedArtifact,
}

impl PersistedSchemaValidationReport {
    fn summary_line(&self) -> String {
        format!(
            "PersistedMusicXmlProductContractGate: GREEN; DeclaredVersion={EXPECTED_PRODUCT_VERSION}; SchemaSchlopf {SCHEMASCHLOPF_REVISION} XSD40: GREEN; instance_validation_qualified=true; {}={} bytes={} declared_version={} root={} elements={} attributes={}; {}={} bytes={} declared_version={} root={} elements={} attributes={}",
            self.master.label,
            self.master.path.display(),
            self.master.bytes,
            self.master.declared_version,
            self.master.root_local_name,
            self.master.element_count,
            self.master.attribute_count,
            self.render.label,
            self.render.path.display(),
            self.render.bytes,
            self.render.declared_version,
            self.render.root_local_name,
            self.render.element_count,
            self.render.attribute_count,
        )
    }
}

#[cfg(test)]
#[path = "../tests/unit/persisted_product_order.rs"]
mod product_order_tests;

static REGISTERED_ARTIFACTS: OnceLock<RegisteredArtifacts> = OnceLock::new();
static BOUND_VALIDATOR: OnceLock<Result<BoundInstanceValidator, String>> = OnceLock::new();
static FINAL_GATE_RESULT: OnceLock<Result<PersistedSchemaValidationReport, String>> = OnceLock::new();

pub(super) fn register_final_artifacts(
    master: &Path,
    render: &Path,
    pdf: &Path,
) -> Result<(), String> {
    let requested = RegisteredArtifacts {
        master: master.to_path_buf(),
        render: render.to_path_buf(),
        pdf: pdf.to_path_buf(),
    };
    if let Some(existing) = REGISTERED_ARTIFACTS.get() {
        if existing == &requested {
            return Ok(());
        }
        return Err(format!(
            "PersistedMusicXmlProductContractGate: widerspruechliche Artefaktregistrierung: existing master={} render={} pdf={}; requested master={} render={} pdf={}",
            existing.master.display(),
            existing.render.display(),
            existing.pdf.display(),
            requested.master.display(),
            requested.render.display(),
            requested.pdf.display(),
        ));
    }
    REGISTERED_ARTIFACTS.set(requested).map_err(|_| {
        "PersistedMusicXmlProductContractGate: Artefaktregistrierung konnte nicht gesetzt werden"
            .to_string()
    })
}

pub(super) fn is_registered_final_renderer_boundary(
    input_musicxml: &Path,
    output_pdf: &Path,
) -> bool {
    REGISTERED_ARTIFACTS.get().is_some_and(|registered| {
        output_pdf == registered.pdf.as_path()
            && (input_musicxml == registered.master.as_path()
                || input_musicxml == registered.render.as_path())
    })
}

pub(super) fn final_gate_evaluated() -> bool {
    FINAL_GATE_RESULT.get().is_some()
}

/// Before rendering, validate the actual, already persisted PDF source. The
/// mandatory unselected product may be realized only at the later sovereign
/// closure; it is checked independently by the complete final-product gate.
pub(super) fn validate_registered_renderer_source(
    input_musicxml: &Path,
    output_pdf: &Path,
) -> Result<Option<String>, String> {
    let Some(registered) = REGISTERED_ARTIFACTS.get() else {
        return Ok(None);
    };
    validate_renderer_source(registered, input_musicxml, output_pdf).map(Some)
}

fn validate_renderer_source(
    registered: &RegisteredArtifacts,
    input_musicxml: &Path,
    output_pdf: &Path,
) -> Result<String, String> {
    let label = if input_musicxml == registered.master {
        "master"
    } else if input_musicxml == registered.render {
        "render"
    } else {
        return Err("PersistedRendererSourceGate: unregistered input".to_owned());
    };
    if output_pdf != registered.pdf {
        return Err("PersistedRendererSourceGate: unregistered PDF output".to_owned());
    }
    let artifact = validate_persisted(bound_validator()?, label, input_musicxml)?;
    Ok(format!(
        "PersistedRendererSourceGate: GREEN; DeclaredVersion={}; SchemaSchlopf {SCHEMASCHLOPF_REVISION} XSD40: GREEN; source={label}; bytes={}; CompleteProductGate=pending_final_product_closure",
        artifact.declared_version, artifact.bytes,
    ))
}

pub(super) fn validate_registered_final_artifacts() -> Result<Option<String>, String> {
    let Some(registered) = REGISTERED_ARTIFACTS.get() else {
        return Ok(None);
    };
    let result = FINAL_GATE_RESULT.get_or_init(|| validate_registered(registered));
    match result {
        Ok(report) => Ok(Some(report.summary_line())),
        Err(error) => Err(error.clone()),
    }
}

fn bound_validator() -> Result<&'static BoundInstanceValidator, String> {
    let result = BOUND_VALIDATOR.get_or_init(|| {
        let schema_dir = Path::new(MUSICXML40_SCHEMA_DIR);
        compile_bound_instance_validator(schema_dir).map_err(|error| {
            format!(
                "SchemaSchlopf {SCHEMASCHLOPF_REVISION} konnte EtuSchlopfs gebundene MusicXML-4.0-Schemas unter {} nicht qualifizieren/kompilieren: {error}",
                schema_dir.display()
            )
        })
    });
    match result {
        Ok(validator) => Ok(validator),
        Err(error) => Err(error.clone()),
    }
}

fn validate_registered(
    registered: &RegisteredArtifacts,
) -> Result<PersistedSchemaValidationReport, String> {
    let validator = bound_validator()?;
    let master = validate_persisted(validator, "master", &registered.master)?;
    let render = validate_persisted(validator, "render", &registered.render)?;
    Ok(PersistedSchemaValidationReport { master, render })
}

fn validate_declared_product_version(
    label: &'static str,
    path: &Path,
    bytes: &[u8],
) -> Result<String, String> {
    let xml = std::str::from_utf8(bytes).map_err(|error| {
        format!(
            "PersistedMusicXmlProductContractGate: {label} {} ist nicht UTF-8 und kann den MusicXML-{EXPECTED_PRODUCT_VERSION}-Produktvertrag nicht deklarieren: {error}",
            path.display()
        )
    })?;
    let document = Document::parse(xml).map_err(|error| {
        format!(
            "PersistedMusicXmlProductContractGate: {label} {} ist nicht wohlgeformtes XML: {error}",
            path.display()
        )
    })?;
    let root = document.root_element();
    let root_local_name = root.tag_name().name();
    if root_local_name != EXPECTED_ROOT_LOCAL_NAME {
        return Err(format!(
            "PersistedMusicXmlProductContractGate: {label} {} deklariert Root {root_local_name:?}; erwartet {EXPECTED_ROOT_LOCAL_NAME:?}",
            path.display()
        ));
    }
    let declared_version = root.attribute("version").unwrap_or("none");
    if declared_version != EXPECTED_PRODUCT_VERSION {
        return Err(format!(
            "PersistedMusicXmlProductContractGate: {label} {} deklariert MusicXML-Version {declared_version:?}; Produktvertrag erwartet exakt {EXPECTED_PRODUCT_VERSION:?}",
            path.display()
        ));
    }
    Ok(declared_version.to_string())
}

fn validate_persisted(
    validator: &BoundInstanceValidator,
    label: &'static str,
    path: &Path,
) -> Result<ValidatedArtifact, String> {
    if !path.is_file() {
        return Err(format!(
            "PersistedMusicXmlProductContractGate: erwartete persistierte {label}-MusicXML fehlt oder ist keine regulaere Datei: {}",
            path.display()
        ));
    }
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "PersistedMusicXmlProductContractGate: persistierte {label}-Bytes konnten nicht gelesen werden {}: {error}",
            path.display()
        )
    })?;

    // Gate A: EtuSchlopf serialization authority. This must run on the persisted bytes before
    // SchemaSchlopf validates the same bytes. XSD 4.0 validity alone does not prove this policy.
    let declared_version = validate_declared_product_version(label, path, &bytes)?;

    // Gate B: the sole productive XSD authority. No shadow validator and no repair path exist.
    let report = validator.validate_bytes(&bytes).map_err(|error| {
        format!(
            "SchemaSchlopf {SCHEMASCHLOPF_REVISION} PersistedByteXsd40Gate: XSD-Validierung fehlgeschlagen fuer {label} {}: {error}",
            path.display()
        )
    })?;
    if !report.instance_validation_qualified() {
        return Err(format!(
            "SchemaSchlopf {SCHEMASCHLOPF_REVISION} PersistedByteXsd40Gate: Validator-Report fuer {label} {} ist nicht instance_validation_qualified",
            path.display()
        ));
    }
    Ok(ValidatedArtifact {
        label,
        path: path.to_path_buf(),
        bytes: bytes.len(),
        declared_version,
        root_local_name: report.root_local_name().to_string(),
        element_count: report.element_count(),
        attribute_count: report.attribute_count(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_PARTWISE_40: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1"/>
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
    <measure number="1"/>
  </part>
</score-partwise>"#;


    #[test]
    fn bound_validator_accepts_valid_musicxml40_bytes() {
        let validator = bound_validator().expect("bound validator must compile");
        let report = validator
            .validate_bytes(MINIMAL_PARTWISE_40)
            .expect("valid MusicXML 4.0 bytes");
        assert_eq!(report.root_local_name(), EXPECTED_ROOT_LOCAL_NAME);
        assert!(report.instance_validation_qualified());
        assert_eq!(
            validate_declared_product_version("fixture", Path::new("fixture.musicxml"), MINIMAL_PARTWISE_40)
                .expect("product version"),
            EXPECTED_PRODUCT_VERSION
        );
    }

    #[test]
    fn xsd40_can_accept_declared31_but_product_gate_rejects_it() {
        let validator = bound_validator().expect("bound validator must compile");
        validator
            .validate_bytes(MINIMAL_PARTWISE_31)
            .expect("MusicXML 4.0 XSD intentionally does not prove declared product version");
        let error = validate_declared_product_version(
            "fixture",
            Path::new("fixture.musicxml"),
            MINIMAL_PARTWISE_31,
        )
        .expect_err("EtuSchlopf product gate must reject declared 3.1");
        assert!(error.contains("Produktvertrag erwartet exakt \"4.0\""));
    }

    #[test]
    fn bound_validator_rejects_schema_invalid_musicxml40_bytes() {
        let validator = bound_validator().expect("bound validator must compile");
        assert!(
            validator
                .validate_bytes(br#"<score-partwise version="4.0"><part id="P1"/></score-partwise>"#)
                .is_err()
        );
    }
}
