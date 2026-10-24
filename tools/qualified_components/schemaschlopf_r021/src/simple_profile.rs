//! Enumeration-restriction qualification for the exact bound MusicXML score-schema profiles.
//!
//! The original 3.1 path is preserved. MusicXML 4.0 additionally contains two enumeration
//! restrictions over named enumeration bases; those inherited value spaces are qualified exactly.

use crate::profile::{BoundSchemaProfile, XSD_NAMESPACE, expected_schema_files};
use crate::qualification::{QualificationError, qualify_bound_schema_bundle};
use crate::simple::{SimpleTypeError, parse_bound_enumeration_restriction};
use roxmltree::Document;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub const BOUND_ENUMERATION_RESTRICTION_COUNT: usize = 97;
pub const BOUND_ENUMERATION_VALUE_COUNT: usize = 604;
const MUSICXML40_ENUMERATION_RESTRICTION_COUNT: usize = 104;
const MUSICXML40_ENUMERATION_VALUE_COUNT: usize = 633;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumerationRestrictionQualification {
    restriction_count: usize,
    enumeration_value_count: usize,
    restrictions_by_base: BTreeMap<String, usize>,
}

impl EnumerationRestrictionQualification {
    #[must_use]
    pub const fn restriction_count(&self) -> usize {
        self.restriction_count
    }

    #[must_use]
    pub const fn enumeration_value_count(&self) -> usize {
        self.enumeration_value_count
    }

    #[must_use]
    pub fn restrictions_by_base(&self) -> &BTreeMap<String, usize> {
        &self.restrictions_by_base
    }
}

#[derive(Debug)]
pub enum EnumerationRestrictionQualificationError {
    SchemaQualification(QualificationError),
    Io {
        path: PathBuf,
        detail: String,
    },
    InvalidSchemaDocument {
        file_name: String,
        detail: String,
    },
    MissingRestrictionBase {
        file_name: String,
    },
    InvalidRestrictionBaseQName {
        file_name: String,
        lexical_qname: String,
    },
    UnsupportedEnumerationBase {
        file_name: String,
        lexical_qname: String,
    },
    MissingEnumerationValue {
        file_name: String,
    },
    InvalidEnumerationRestriction {
        file_name: String,
        base_local_name: String,
        detail: String,
    },
    InventoryMismatch {
        expected_restrictions: usize,
        actual_restrictions: usize,
        expected_values: usize,
        actual_values: usize,
        expected_by_base: BTreeMap<String, usize>,
        actual_by_base: BTreeMap<String, usize>,
    },
}

impl fmt::Display for EnumerationRestrictionQualificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaQualification(error) => write!(
                formatter,
                "schema bundle must qualify before enumeration restrictions: {error}"
            ),
            Self::Io { path, detail } => {
                write!(formatter, "I/O error at {}: {detail}", path.display())
            }
            Self::InvalidSchemaDocument { file_name, detail } => write!(
                formatter,
                "{file_name}: invalid schema document during enumeration qualification: {detail}"
            ),
            Self::MissingRestrictionBase { file_name } => write!(
                formatter,
                "{file_name}: enumeration-only restriction has no base QName"
            ),
            Self::InvalidRestrictionBaseQName {
                file_name,
                lexical_qname,
            } => write!(
                formatter,
                "{file_name}: invalid enumeration restriction base QName {lexical_qname:?}"
            ),
            Self::UnsupportedEnumerationBase {
                file_name,
                lexical_qname,
            } => write!(
                formatter,
                "{file_name}: enumeration-only restriction base {lexical_qname:?} is outside the exact bound enumeration profile"
            ),
            Self::MissingEnumerationValue { file_name } => write!(
                formatter,
                "{file_name}: enumeration facet is missing its value attribute"
            ),
            Self::InvalidEnumerationRestriction {
                file_name,
                base_local_name,
                detail,
            } => write!(
                formatter,
                "{file_name}: invalid enumeration restriction over xs:{base_local_name}: {detail}"
            ),
            Self::InventoryMismatch {
                expected_restrictions,
                actual_restrictions,
                expected_values,
                actual_values,
                expected_by_base,
                actual_by_base,
            } => write!(
                formatter,
                "enumeration restriction inventory mismatch; restrictions expected {expected_restrictions}, got {actual_restrictions}; values expected {expected_values}, got {actual_values}; by-base expected {expected_by_base:?}, got {actual_by_base:?}"
            ),
        }
    }
}

impl std::error::Error for EnumerationRestrictionQualificationError {}

impl From<QualificationError> for EnumerationRestrictionQualificationError {
    fn from(error: QualificationError) -> Self {
        Self::SchemaQualification(error)
    }
}

pub fn qualify_bound_enumeration_restrictions(
    schema_dir: impl AsRef<Path>,
) -> Result<EnumerationRestrictionQualification, EnumerationRestrictionQualificationError> {
    let schema_dir = schema_dir.as_ref();

    // This is deliberately first: enumeration semantics never extend authority to different
    // schema bytes merely because their XML shape happens to look familiar.
    let bundle = qualify_bound_schema_bundle(schema_dir)?;
    let profile = bundle.profile();
    let direct_enumeration_bases = collect_direct_enumeration_bases(schema_dir, profile)?;

    let mut restriction_count = 0_usize;
    let mut enumeration_value_count = 0_usize;
    let mut restrictions_by_base = BTreeMap::new();

    for expected in expected_schema_files(profile) {
        let path = schema_dir.join(expected.file_name);
        let text = fs::read_to_string(&path).map_err(|error| {
            EnumerationRestrictionQualificationError::Io {
                path: path.clone(),
                detail: error.to_string(),
            }
        })?;
        let document = Document::parse(&text).map_err(|error| {
            EnumerationRestrictionQualificationError::InvalidSchemaDocument {
                file_name: expected.file_name.to_owned(),
                detail: error.to_string(),
            }
        })?;

        for simple_type in document.descendants().filter(|node| {
            node.is_element()
                && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                && node.tag_name().name() == "simpleType"
        }) {
            let Some(restriction) = simple_type.children().find(|node| {
                node.is_element()
                    && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                    && node.tag_name().name() == "restriction"
            }) else {
                continue;
            };

            let facet_nodes: Vec<_> = restriction
                .children()
                .filter(|node| {
                    node.is_element()
                        && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                        && node.tag_name().name() != "annotation"
                })
                .collect();

            if facet_nodes.is_empty()
                || !facet_nodes
                    .iter()
                    .all(|node| node.tag_name().name() == "enumeration")
            {
                continue;
            }

            let lexical_base = restriction.attribute("base").ok_or_else(|| {
                EnumerationRestrictionQualificationError::MissingRestrictionBase {
                    file_name: expected.file_name.to_owned(),
                }
            })?;
            let (primitive_base_local_name, base_enumeration_lexicals, count_base_name) =
                if lexical_base.contains(':') {
                    (
                        resolve_direct_xsd_builtin_base(
                            expected.file_name,
                            restriction,
                            lexical_base,
                        )?,
                        None,
                        lexical_base
                            .split_once(':')
                            .map(|(_, local)| local.to_owned())
                            .unwrap_or_else(|| lexical_base.to_owned()),
                    )
                } else {
                    let base = direct_enumeration_bases.get(lexical_base).ok_or_else(|| {
                        EnumerationRestrictionQualificationError::UnsupportedEnumerationBase {
                            file_name: expected.file_name.to_owned(),
                            lexical_qname: lexical_base.to_owned(),
                        }
                    })?;
                    (
                        base.primitive_base_local_name.clone(),
                        Some(base.enumeration_lexicals.as_slice()),
                        lexical_base.to_owned(),
                    )
                };

            let enumeration_lexicals: Vec<&str> = facet_nodes
                .iter()
                .map(|node| {
                    node.attribute("value").ok_or_else(|| {
                        EnumerationRestrictionQualificationError::MissingEnumerationValue {
                            file_name: expected.file_name.to_owned(),
                        }
                    })
                })
                .collect::<Result<_, _>>()?;

            // Each declared enumeration value must itself be a valid value of the base type.
            // Running each through the same membership path also proves that the runtime
            // restriction evaluator and the schema-facet qualification agree.
            for lexical in &enumeration_lexicals {
                if let Some(base_values) = base_enumeration_lexicals {
                    let base_values: Vec<&str> = base_values.iter().map(String::as_str).collect();
                    parse_bound_enumeration_restriction(
                        &primitive_base_local_name,
                        lexical,
                        &base_values,
                    )
                    .map_err(|error| invalid_restriction_error(
                        expected.file_name,
                        &primitive_base_local_name,
                        error,
                    ))?;
                }
                parse_bound_enumeration_restriction(
                    &primitive_base_local_name,
                    lexical,
                    &enumeration_lexicals,
                )
                .map_err(|error| invalid_restriction_error(
                    expected.file_name,
                    &primitive_base_local_name,
                    error,
                ))?;
            }

            restriction_count += 1;
            enumeration_value_count += enumeration_lexicals.len();
            *restrictions_by_base.entry(count_base_name).or_insert(0) += 1;
        }
    }

    let expected_by_base = expected_restrictions_by_base_for(profile);
    let (expected_restrictions, expected_values) = expected_counts(profile);
    if restriction_count != expected_restrictions
        || enumeration_value_count != expected_values
        || restrictions_by_base != expected_by_base
    {
        return Err(EnumerationRestrictionQualificationError::InventoryMismatch {
            expected_restrictions,
            actual_restrictions: restriction_count,
            expected_values,
            actual_values: enumeration_value_count,
            expected_by_base,
            actual_by_base: restrictions_by_base,
        });
    }

    Ok(EnumerationRestrictionQualification {
        restriction_count,
        enumeration_value_count,
        restrictions_by_base,
    })
}

fn resolve_direct_xsd_builtin_base(
    file_name: &str,
    restriction: roxmltree::Node<'_, '_>,
    lexical_qname: &str,
) -> Result<String, EnumerationRestrictionQualificationError> {
    let Some((prefix, local_name)) = lexical_qname.split_once(':') else {
        return Err(
            EnumerationRestrictionQualificationError::InvalidRestrictionBaseQName {
                file_name: file_name.to_owned(),
                lexical_qname: lexical_qname.to_owned(),
            },
        );
    };
    if prefix.is_empty() || local_name.is_empty() || local_name.contains(':') {
        return Err(
            EnumerationRestrictionQualificationError::InvalidRestrictionBaseQName {
                file_name: file_name.to_owned(),
                lexical_qname: lexical_qname.to_owned(),
            },
        );
    }

    let namespace = restriction.lookup_namespace_uri(Some(prefix));
    if namespace != Some(XSD_NAMESPACE)
        || !matches!(local_name, "string" | "token" | "NMTOKEN" | "NCName")
    {
        return Err(
            EnumerationRestrictionQualificationError::UnsupportedEnumerationBase {
                file_name: file_name.to_owned(),
                lexical_qname: lexical_qname.to_owned(),
            },
        );
    }

    Ok(local_name.to_owned())
}

fn invalid_restriction_error(
    file_name: &str,
    base_local_name: &str,
    error: SimpleTypeError,
) -> EnumerationRestrictionQualificationError {
    EnumerationRestrictionQualificationError::InvalidEnumerationRestriction {
        file_name: file_name.to_owned(),
        base_local_name: base_local_name.to_owned(),
        detail: error.to_string(),
    }
}

#[derive(Clone, Debug)]
struct DirectEnumerationBase {
    primitive_base_local_name: String,
    enumeration_lexicals: Vec<String>,
}

fn collect_direct_enumeration_bases(
    schema_dir: &Path,
    profile: BoundSchemaProfile,
) -> Result<BTreeMap<String, DirectEnumerationBase>, EnumerationRestrictionQualificationError> {
    let mut output = BTreeMap::new();
    for expected in expected_schema_files(profile) {
        let path = schema_dir.join(expected.file_name);
        let text = fs::read_to_string(&path).map_err(|error| {
            EnumerationRestrictionQualificationError::Io {
                path: path.clone(),
                detail: error.to_string(),
            }
        })?;
        let document = Document::parse(&text).map_err(|error| {
            EnumerationRestrictionQualificationError::InvalidSchemaDocument {
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
            if facets.is_empty() || !facets.iter().all(|node| node.tag_name().name() == "enumeration") {
                continue;
            }
            let Some(lexical_base) = restriction.attribute("base") else {
                continue;
            };
            if !lexical_base.contains(':') {
                continue;
            }
            let Ok(primitive_base_local_name) =
                resolve_direct_xsd_builtin_base(expected.file_name, restriction, lexical_base)
            else {
                continue;
            };
            let enumeration_lexicals = facets
                .iter()
                .map(|node| node.attribute("value").unwrap_or_default().to_owned())
                .collect();
            output.insert(
                simple_type.attribute("name").expect("filtered named simple type").to_owned(),
                DirectEnumerationBase {
                    primitive_base_local_name,
                    enumeration_lexicals,
                },
            );
        }
    }
    Ok(output)
}

const fn expected_counts(profile: BoundSchemaProfile) -> (usize, usize) {
    match profile {
        BoundSchemaProfile::MusicXml31 => (
            BOUND_ENUMERATION_RESTRICTION_COUNT,
            BOUND_ENUMERATION_VALUE_COUNT,
        ),
        BoundSchemaProfile::MusicXml40 => (
            MUSICXML40_ENUMERATION_RESTRICTION_COUNT,
            MUSICXML40_ENUMERATION_VALUE_COUNT,
        ),
    }
}

#[cfg(test)]
fn expected_restrictions_by_base() -> BTreeMap<String, usize> {
    expected_restrictions_by_base_for(BoundSchemaProfile::MusicXml31)
}

fn expected_restrictions_by_base_for(profile: BoundSchemaProfile) -> BTreeMap<String, usize> {
    let rows: &[(&str, usize)] = match profile {
        BoundSchemaProfile::MusicXml31 => &[
            ("string", 45), ("token", 48), ("NMTOKEN", 3), ("NCName", 1),
        ],
        BoundSchemaProfile::MusicXml40 => &[
            ("string", 47), ("token", 51), ("system-relation-number", 1),
            ("note-type-value", 1), ("NMTOKEN", 3), ("NCName", 1),
        ],
    };
    rows.iter()
        .map(|(name, count)| ((*name).to_owned(), *count))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MUSICXML31_SCHEMA_DIR, MUSICXML40_SCHEMA_DIR};

    #[test]
    fn exact_bound_enumeration_restriction_inventory_qualifies() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        let qualification =
            qualify_bound_enumeration_restrictions(&root).expect("bound enumerations must qualify");

        assert_eq!(
            qualification.restriction_count(),
            BOUND_ENUMERATION_RESTRICTION_COUNT
        );
        assert_eq!(
            qualification.enumeration_value_count(),
            BOUND_ENUMERATION_VALUE_COUNT
        );
        assert_eq!(
            qualification.restrictions_by_base(),
            &expected_restrictions_by_base()
        );
    }

    #[test]
    fn r013_inventory_is_exactly_the_direct_builtin_enumeration_subset() {
        let expected = expected_restrictions_by_base();
        assert_eq!(expected.values().sum::<usize>(), 97);
        assert_eq!(expected.get("string"), Some(&45));
        assert_eq!(expected.get("token"), Some(&48));
        assert_eq!(expected.get("NMTOKEN"), Some(&3));
        assert_eq!(expected.get("NCName"), Some(&1));
    }
    #[test]
    fn musicxml40_named_base_enumerations_are_qualified_as_true_restrictions() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let qualification = qualify_bound_enumeration_restrictions(&root)
            .expect("bound 4.0 enumerations must qualify");
        assert_eq!(qualification.restriction_count(), 104);
        assert_eq!(qualification.enumeration_value_count(), 633);
        assert_eq!(qualification.restrictions_by_base().get("system-relation-number"), Some(&1));
        assert_eq!(qualification.restrictions_by_base().get("note-type-value"), Some(&1));
    }

}
