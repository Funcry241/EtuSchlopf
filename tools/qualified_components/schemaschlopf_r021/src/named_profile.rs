//! Qualification and value semantics for the exact named restriction-chain subset.
//!
//! MusicXML 3.1 has thirteen facetless aliases plus `positive-divisions`. MusicXML 4.0 adds
//! three direct aliases and `font-family -> comma-separated-text`, whose inherited pattern
//! semantics are preserved. Pattern-facet restrictions themselves remain in `restriction_profile`.

use crate::profile::{BoundSchemaProfile, XSD_NAMESPACE, expected_schema_files, is_bound_builtin_type};
use crate::qualification::{QualificationError, qualify_bound_schema_bundle};
use crate::restriction_profile::matches_bound_xsd_pattern;
use crate::simple::{
    AtomicValue, SimpleTypeError, WhiteSpaceMode, apply_white_space, parse_bound_builtin,
    parse_bound_numeric_restriction,
};
use roxmltree::Document;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub const BOUND_NAME_ALIAS_RESTRICTION_COUNT: usize = 13;
pub const BOUND_NAMED_NUMERIC_RESTRICTION_COUNT: usize = 1;
pub const BOUND_NAMED_NUMERIC_FACET_COUNT: usize = 1;
const MUSICXML40_NAME_ALIAS_RESTRICTION_COUNT: usize = 16;

#[derive(Clone, Debug, Eq, PartialEq)]
struct NamedNumericRestriction {
    primitive_base: String,
    min_exclusive: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamedRestrictionQualification {
    aliases: BTreeMap<String, String>,
    numeric_restrictions: BTreeMap<String, NamedNumericRestriction>,
}

impl NamedRestrictionQualification {
    #[must_use]
    pub fn alias_count(&self) -> usize {
        self.aliases.len()
    }

    #[must_use]
    pub fn named_numeric_restriction_count(&self) -> usize {
        self.numeric_restrictions.len()
    }

    #[must_use]
    pub fn named_numeric_facet_count(&self) -> usize {
        self.numeric_restrictions.len()
    }

    #[must_use]
    pub fn aliases(&self) -> &BTreeMap<String, String> {
        &self.aliases
    }

    pub fn parse_value(
        &self,
        local_name: &str,
        lexical: &str,
    ) -> Result<AtomicValue, NamedRestrictionValueError> {
        if let Some(base_local_name) = self.aliases.get(local_name) {
            if is_bound_builtin_type(base_local_name) {
                return parse_bound_builtin(base_local_name, lexical)
                    .map_err(NamedRestrictionValueError::Datatype);
            }
            if base_local_name == "comma-separated-text" {
                let normalized = apply_white_space(WhiteSpaceMode::Collapse, lexical);
                if matches_bound_xsd_pattern(r"[^,]+(, ?[^,]+)*", &normalized)
                    .map_err(|detail| NamedRestrictionValueError::ComposedRestriction { detail })?
                {
                    return parse_bound_builtin("token", lexical)
                        .map_err(NamedRestrictionValueError::Datatype);
                }
                return Err(NamedRestrictionValueError::ComposedRestriction {
                    detail: format!(
                        "value {normalized:?} violates inherited comma-separated-text pattern"
                    ),
                });
            }
        }

        if let Some(restriction) = self.numeric_restrictions.get(local_name) {
            return parse_bound_numeric_restriction(
                &restriction.primitive_base,
                lexical,
                None,
                None,
                Some(&restriction.min_exclusive),
            )
            .map_err(NamedRestrictionValueError::Datatype);
        }

        Err(NamedRestrictionValueError::UnknownNamedType {
            local_name: local_name.to_owned(),
        })
    }
}

#[derive(Debug)]
pub enum NamedRestrictionValueError {
    UnknownNamedType { local_name: String },
    Datatype(SimpleTypeError),
    ComposedRestriction { detail: String },
}

impl fmt::Display for NamedRestrictionValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownNamedType { local_name } => {
                write!(formatter, "unknown qualified named simple type {local_name:?}")
            }
            Self::Datatype(error) => write!(formatter, "{error}"),
            Self::ComposedRestriction { detail } => formatter.write_str(detail),
        }
    }
}

impl std::error::Error for NamedRestrictionValueError {}

#[derive(Debug)]
pub enum NamedRestrictionQualificationError {
    SchemaQualification(QualificationError),
    Io { path: PathBuf, detail: String },
    InvalidSchemaDocument { file_name: String, detail: String },
    InvalidStructure { file_name: String, detail: String },
    InventoryMismatch {
        expected_aliases: BTreeMap<String, String>,
        actual_aliases: BTreeMap<String, String>,
        expected_named_numeric: usize,
        actual_named_numeric: usize,
    },
}

impl fmt::Display for NamedRestrictionQualificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaQualification(error) => write!(
                formatter,
                "schema bundle must qualify before named restrictions: {error}"
            ),
            Self::Io { path, detail } => {
                write!(formatter, "I/O error at {}: {detail}", path.display())
            }
            Self::InvalidSchemaDocument { file_name, detail } => write!(
                formatter,
                "{file_name}: invalid schema document during named restriction qualification: {detail}"
            ),
            Self::InvalidStructure { file_name, detail } => {
                write!(formatter, "{file_name}: invalid named restriction structure: {detail}")
            }
            Self::InventoryMismatch {
                expected_aliases,
                actual_aliases,
                expected_named_numeric,
                actual_named_numeric,
            } => write!(
                formatter,
                "named restriction inventory mismatch; aliases expected {expected_aliases:?}, got {actual_aliases:?}; named numeric expected {expected_named_numeric}, got {actual_named_numeric}"
            ),
        }
    }
}

impl std::error::Error for NamedRestrictionQualificationError {}

impl From<QualificationError> for NamedRestrictionQualificationError {
    fn from(error: QualificationError) -> Self {
        Self::SchemaQualification(error)
    }
}

pub fn qualify_bound_named_restrictions(
    schema_dir: impl AsRef<Path>,
) -> Result<NamedRestrictionQualification, NamedRestrictionQualificationError> {
    let schema_dir = schema_dir.as_ref();
    let bundle = qualify_bound_schema_bundle(schema_dir)?;
    let profile = bundle.profile();
    let global_simple_types = collect_named_simple_types(schema_dir, profile)?;

    let mut aliases = BTreeMap::new();
    let mut named_numeric_sources = Vec::new();

    for expected in expected_schema_files(profile) {
        let path = schema_dir.join(expected.file_name);
        let text = fs::read_to_string(&path).map_err(|error| {
            NamedRestrictionQualificationError::Io {
                path: path.clone(),
                detail: error.to_string(),
            }
        })?;
        let document = Document::parse(&text).map_err(|error| {
            NamedRestrictionQualificationError::InvalidSchemaDocument {
                file_name: expected.file_name.to_owned(),
                detail: error.to_string(),
            }
        })?;

        for simple_type in document.descendants().filter(|node| {
            node.is_element()
                && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                && node.tag_name().name() == "simpleType"
                && node.attribute("name").is_some()
        }) {
            let Some(restriction) = simple_type.children().find(|node| {
                node.is_element()
                    && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                    && node.tag_name().name() == "restriction"
            }) else {
                continue;
            };

            let facets: Vec<_> = restriction
                .children()
                .filter(|node| {
                    node.is_element()
                        && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                        && node.tag_name().name() != "annotation"
                })
                .collect();

            let name = simple_type
                .attribute("name")
                .expect("filtered named simple type")
                .to_owned();
            let base = restriction.attribute("base").ok_or_else(|| {
                NamedRestrictionQualificationError::InvalidStructure {
                    file_name: expected.file_name.to_owned(),
                    detail: format!("named restriction {name:?} has no base QName"),
                }
            })?;

            if facets.is_empty() {
                if let Some((prefix, local_name)) = base.split_once(':') {
                    if prefix.is_empty()
                        || local_name.is_empty()
                        || local_name.contains(':')
                        || restriction.lookup_namespace_uri(Some(prefix)) != Some(XSD_NAMESPACE)
                    {
                        continue;
                    }
                    parse_bound_builtin(local_name, probe_lexical(local_name)).map_err(|error| {
                        NamedRestrictionQualificationError::InvalidStructure {
                            file_name: expected.file_name.to_owned(),
                            detail: format!(
                                "facetless alias {name:?} has unsupported built-in base {base:?}: {error}"
                            ),
                        }
                    })?;
                    aliases.insert(name, local_name.to_owned());
                    continue;
                }

                if !base.is_empty() && !base.contains(':') && global_simple_types.contains(base) {
                    aliases.insert(name, base.to_owned());
                    continue;
                }
            }

            if facets.iter().all(|facet| {
                matches!(
                    facet.tag_name().name(),
                    "minInclusive" | "maxInclusive" | "minExclusive"
                )
            }) && !base.contains(':')
            {
                named_numeric_sources.push((
                    expected.file_name.to_owned(),
                    name,
                    base.to_owned(),
                    facets
                        .iter()
                        .map(|facet| {
                            (
                                facet.tag_name().name().to_owned(),
                                facet.attribute("value").map(str::to_owned),
                            )
                        })
                        .collect::<Vec<_>>(),
                ));
            }
        }
    }

    let expected_alias_map = expected_aliases_for(profile);
    let expected_alias_count = expected_alias_count(profile);
    if aliases != expected_alias_map || aliases.len() != expected_alias_count {
        return Err(NamedRestrictionQualificationError::InventoryMismatch {
            expected_aliases: expected_alias_map,
            actual_aliases: aliases,
            expected_named_numeric: BOUND_NAMED_NUMERIC_RESTRICTION_COUNT,
            actual_named_numeric: named_numeric_sources.len(),
        });
    }

    let mut numeric_restrictions = BTreeMap::new();
    for (file_name, name, base_name, facets) in named_numeric_sources {
        let primitive_base = aliases.get(&base_name).ok_or_else(|| {
            NamedRestrictionQualificationError::InvalidStructure {
                file_name: file_name.clone(),
                detail: format!(
                    "named numeric restriction {name:?} refers to unresolved base {base_name:?}"
                ),
            }
        })?;

        if !matches!(
            primitive_base.as_str(),
            "decimal" | "integer" | "nonNegativeInteger" | "positiveInteger"
        ) {
            return Err(NamedRestrictionQualificationError::InvalidStructure {
                file_name,
                detail: format!(
                    "named numeric restriction {name:?} resolves to non-numeric xs:{primitive_base}"
                ),
            });
        }

        if facets.len() != 1 || facets[0].0 != "minExclusive" {
            return Err(NamedRestrictionQualificationError::InvalidStructure {
                file_name,
                detail: format!(
                    "bound named numeric profile expects exactly one minExclusive facet on {name:?}, got {facets:?}"
                ),
            });
        }
        let min_exclusive = facets[0].1.clone().ok_or_else(|| {
            NamedRestrictionQualificationError::InvalidStructure {
                file_name: file_name.clone(),
                detail: format!("minExclusive facet on {name:?} has no value"),
            }
        })?;

        // The facet lexical must be a value of the resolved primitive base, and equality with
        // an exclusive lower boundary must be rejected.
        match parse_bound_numeric_restriction(
            primitive_base,
            &min_exclusive,
            None,
            None,
            Some(&min_exclusive),
        ) {
            Err(SimpleTypeError::NumericBoundConstraintViolation { .. }) => {}
            Err(error) => {
                return Err(NamedRestrictionQualificationError::InvalidStructure {
                    file_name,
                    detail: format!(
                        "invalid minExclusive facet on {name:?} after resolving {base_name:?} -> xs:{primitive_base}: {error}"
                    ),
                });
            }
            Ok(_) => {
                return Err(NamedRestrictionQualificationError::InvalidStructure {
                    file_name,
                    detail: format!(
                        "exclusive boundary on {name:?} unexpectedly admitted itself"
                    ),
                });
            }
        }

        numeric_restrictions.insert(
            name,
            NamedNumericRestriction {
                primitive_base: primitive_base.clone(),
                min_exclusive,
            },
        );
    }

    if numeric_restrictions.len() != BOUND_NAMED_NUMERIC_RESTRICTION_COUNT {
        return Err(NamedRestrictionQualificationError::InventoryMismatch {
            expected_aliases: expected_alias_map,
            actual_aliases: aliases,
            expected_named_numeric: BOUND_NAMED_NUMERIC_RESTRICTION_COUNT,
            actual_named_numeric: numeric_restrictions.len(),
        });
    }

    Ok(NamedRestrictionQualification {
        aliases,
        numeric_restrictions,
    })
}

fn collect_named_simple_types(
    schema_dir: &Path,
    profile: BoundSchemaProfile,
) -> Result<std::collections::BTreeSet<String>, NamedRestrictionQualificationError> {
    let mut names = std::collections::BTreeSet::new();
    for expected in expected_schema_files(profile) {
        let path = schema_dir.join(expected.file_name);
        let text = fs::read_to_string(&path).map_err(|error| {
            NamedRestrictionQualificationError::Io {
                path: path.clone(),
                detail: error.to_string(),
            }
        })?;
        let document = Document::parse(&text).map_err(|error| {
            NamedRestrictionQualificationError::InvalidSchemaDocument {
                file_name: expected.file_name.to_owned(),
                detail: error.to_string(),
            }
        })?;
        for node in document.descendants().filter(|node| {
            node.is_element()
                && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                && node.tag_name().name() == "simpleType"
        }) {
            if let Some(name) = node.attribute("name") {
                names.insert(name.to_owned());
            }
        }
    }
    Ok(names)
}

const fn expected_alias_count(profile: BoundSchemaProfile) -> usize {
    match profile {
        BoundSchemaProfile::MusicXml31 => BOUND_NAME_ALIAS_RESTRICTION_COUNT,
        BoundSchemaProfile::MusicXml40 => MUSICXML40_NAME_ALIAS_RESTRICTION_COUNT,
    }
}

#[cfg(test)]
fn expected_aliases() -> BTreeMap<String, String> {
    expected_aliases_for(BoundSchemaProfile::MusicXml31)
}

fn expected_aliases_for(profile: BoundSchemaProfile) -> BTreeMap<String, String> {
    let rows: &[(&str, &str)] = match profile {
        BoundSchemaProfile::MusicXml31 => &[
            ("distance-type", "token"), ("divisions", "decimal"),
            ("fifths", "integer"), ("glyph-type", "token"),
            ("line-width-type", "token"), ("millimeters", "decimal"),
            ("mode", "string"), ("semitones", "decimal"),
            ("smufl-glyph-name", "NMTOKEN"), ("staff-line", "integer"),
            ("staff-number", "positiveInteger"), ("string-number", "positiveInteger"),
            ("tenths", "decimal"),
        ],
        BoundSchemaProfile::MusicXml40 => &[
            ("distance-type", "token"), ("divisions", "decimal"),
            ("fifths", "integer"), ("font-family", "comma-separated-text"),
            ("glyph-type", "token"), ("line-width-type", "token"),
            ("millimeters", "decimal"), ("milliseconds", "nonNegativeInteger"),
            ("mode", "string"), ("semitones", "decimal"),
            ("smufl-glyph-name", "NMTOKEN"), ("staff-line", "positiveInteger"),
            ("staff-line-position", "integer"), ("staff-number", "positiveInteger"),
            ("string-number", "positiveInteger"), ("tenths", "decimal"),
        ],
    };
    rows.iter()
        .map(|(name, base)| ((*name).to_owned(), (*base).to_owned()))
        .collect()
}

fn probe_lexical(base_local_name: &str) -> &'static str {
    match base_local_name {
        "string" | "token" => "",
        "decimal" | "integer" | "nonNegativeInteger" | "positiveInteger" => "1",
        "NMTOKEN" | "NCName" | "ID" | "IDREF" => "a",
        "language" => "en",
        "date" => "2000-01-01",
        "anyURI" => "",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MUSICXML31_SCHEMA_DIR, MUSICXML40_SCHEMA_DIR};

    fn qualification() -> NamedRestrictionQualification {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        qualify_bound_named_restrictions(&root).expect("bound named restrictions must qualify")
    }

    #[test]
    fn exact_named_restriction_inventory_qualifies() {
        let qualification = qualification();
        assert_eq!(qualification.alias_count(), 13);
        assert_eq!(qualification.aliases(), &expected_aliases());
        assert_eq!(qualification.named_numeric_restriction_count(), 1);
        assert_eq!(qualification.named_numeric_facet_count(), 1);
    }

    #[test]
    fn facetless_named_types_preserve_their_base_value_semantics() {
        let qualification = qualification();

        assert_eq!(
            qualification.parse_value("divisions", "  2.500  ").unwrap(),
            parse_bound_builtin("decimal", "2.5").unwrap()
        );
        assert_eq!(
            qualification.parse_value("mode", "  dorian  ").unwrap(),
            parse_bound_builtin("string", "  dorian  ").unwrap()
        );
        assert!(qualification.parse_value("staff-number", "0").is_err());
        assert!(qualification.parse_value("smufl-glyph-name", "accSharp").is_ok());
    }

    #[test]
    fn positive_divisions_inherits_decimal_and_adds_min_exclusive_zero() {
        let qualification = qualification();

        assert!(qualification.parse_value("positive-divisions", "0").is_err());
        assert!(qualification.parse_value("positive-divisions", "-0.1").is_err());
        assert!(qualification.parse_value("positive-divisions", "0.0001").is_ok());
        assert!(qualification
            .parse_value(
                "positive-divisions",
                "123456789012345678901234567890.125"
            )
            .is_ok());
    }

    #[test]
    fn unknown_named_type_fails_closed() {
        let qualification = qualification();
        assert!(matches!(
            qualification.parse_value("not-in-profile", "1"),
            Err(NamedRestrictionValueError::UnknownNamedType { .. })
        ));
    }
    #[test]
    fn musicxml40_named_alias_chain_inherits_pattern_semantics() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let qualification = qualify_bound_named_restrictions(&root)
            .expect("bound 4.0 named restrictions must qualify");
        assert_eq!(qualification.alias_count(), 16);
        assert_eq!(qualification.aliases().get("font-family").map(String::as_str), Some("comma-separated-text"));
        assert!(qualification.parse_value("font-family", "Bravura, Arial").is_ok());
        assert!(qualification.parse_value("font-family", ", Arial").is_err());
        assert!(qualification.parse_value("staff-line", "0").is_err());
    }

}
