//! Qualification of the exact direct-builtin numeric-bound restriction subset.
//!
//! Direct numeric-bound qualification for the exact bound MusicXML score-schema profiles.
//! `positive-divisions` remains qualified separately as the one named-base numeric chain.

use crate::profile::{BoundSchemaProfile, XSD_NAMESPACE, expected_schema_files};
use crate::qualification::{QualificationError, qualify_bound_schema_bundle};
use crate::simple::{SimpleTypeError, parse_bound_numeric_restriction};
use roxmltree::Document;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub const BOUND_DIRECT_NUMERIC_RESTRICTION_COUNT: usize = 14;
pub const BOUND_DIRECT_NUMERIC_FACET_COUNT: usize = 25;
const MUSICXML40_DIRECT_NUMERIC_RESTRICTION_COUNT: usize = 15;
const MUSICXML40_DIRECT_NUMERIC_FACET_COUNT: usize = 27;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumericBoundRestrictionQualification {
    restriction_count: usize,
    facet_count: usize,
    restrictions_by_base: BTreeMap<String, usize>,
}

impl NumericBoundRestrictionQualification {
    #[must_use]
    pub const fn restriction_count(&self) -> usize {
        self.restriction_count
    }

    #[must_use]
    pub const fn facet_count(&self) -> usize {
        self.facet_count
    }

    #[must_use]
    pub fn restrictions_by_base(&self) -> &BTreeMap<String, usize> {
        &self.restrictions_by_base
    }
}

#[derive(Debug)]
pub enum NumericBoundRestrictionQualificationError {
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
    MissingFacetValue {
        file_name: String,
        facet: String,
    },
    InvalidNumericRestriction {
        file_name: String,
        base_local_name: String,
        detail: String,
    },
    InventoryMismatch {
        expected_restrictions: usize,
        actual_restrictions: usize,
        expected_facets: usize,
        actual_facets: usize,
        expected_by_base: BTreeMap<String, usize>,
        actual_by_base: BTreeMap<String, usize>,
    },
}

impl fmt::Display for NumericBoundRestrictionQualificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaQualification(error) => write!(
                formatter,
                "schema bundle must qualify before numeric restrictions: {error}"
            ),
            Self::Io { path, detail } => {
                write!(formatter, "I/O error at {}: {detail}", path.display())
            }
            Self::InvalidSchemaDocument { file_name, detail } => write!(
                formatter,
                "{file_name}: invalid schema document during numeric-bound qualification: {detail}"
            ),
            Self::MissingRestrictionBase { file_name } => write!(
                formatter,
                "{file_name}: numeric-bound restriction has no base QName"
            ),
            Self::InvalidRestrictionBaseQName {
                file_name,
                lexical_qname,
            } => write!(
                formatter,
                "{file_name}: invalid direct numeric restriction base QName {lexical_qname:?}"
            ),
            Self::MissingFacetValue { file_name, facet } => write!(
                formatter,
                "{file_name}: {facet} facet is missing its value attribute"
            ),
            Self::InvalidNumericRestriction {
                file_name,
                base_local_name,
                detail,
            } => write!(
                formatter,
                "{file_name}: invalid numeric restriction over xs:{base_local_name}: {detail}"
            ),
            Self::InventoryMismatch {
                expected_restrictions,
                actual_restrictions,
                expected_facets,
                actual_facets,
                expected_by_base,
                actual_by_base,
            } => write!(
                formatter,
                "direct numeric restriction inventory mismatch; restrictions expected {expected_restrictions}, got {actual_restrictions}; facets expected {expected_facets}, got {actual_facets}; by-base expected {expected_by_base:?}, got {actual_by_base:?}"
            ),
        }
    }
}

impl std::error::Error for NumericBoundRestrictionQualificationError {}

impl From<QualificationError> for NumericBoundRestrictionQualificationError {
    fn from(error: QualificationError) -> Self {
        Self::SchemaQualification(error)
    }
}

pub fn qualify_bound_direct_numeric_restrictions(
    schema_dir: impl AsRef<Path>,
) -> Result<NumericBoundRestrictionQualification, NumericBoundRestrictionQualificationError> {
    let schema_dir = schema_dir.as_ref();

    // Numeric facet semantics never extend authority to different schema bytes.
    let bundle = qualify_bound_schema_bundle(schema_dir)?;
    let profile = bundle.profile();

    let mut restriction_count = 0_usize;
    let mut facet_count = 0_usize;
    let mut restrictions_by_base = BTreeMap::new();

    for expected in expected_schema_files(profile) {
        let path = schema_dir.join(expected.file_name);
        let text = fs::read_to_string(&path).map_err(|error| {
            NumericBoundRestrictionQualificationError::Io {
                path: path.clone(),
                detail: error.to_string(),
            }
        })?;
        let document = Document::parse(&text).map_err(|error| {
            NumericBoundRestrictionQualificationError::InvalidSchemaDocument {
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

            let facets: Vec<_> = restriction
                .children()
                .filter(|node| {
                    node.is_element()
                        && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                        && node.tag_name().name() != "annotation"
                })
                .collect();

            if facets.is_empty()
                || !facets.iter().all(|node| {
                    matches!(
                        node.tag_name().name(),
                        "minInclusive" | "maxInclusive" | "minExclusive"
                    )
                })
            {
                continue;
            }

            let lexical_base = restriction.attribute("base").ok_or_else(|| {
                NumericBoundRestrictionQualificationError::MissingRestrictionBase {
                    file_name: expected.file_name.to_owned(),
                }
            })?;

            // r014 deliberately ignores named bases. The single bound named-base numeric
            // restriction (`positive-divisions`) is qualified only after named-type resolution.
            let Some((prefix, local_name)) = lexical_base.split_once(':') else {
                continue;
            };
            if prefix.is_empty() || local_name.is_empty() || local_name.contains(':') {
                return Err(
                    NumericBoundRestrictionQualificationError::InvalidRestrictionBaseQName {
                        file_name: expected.file_name.to_owned(),
                        lexical_qname: lexical_base.to_owned(),
                    },
                );
            }

            if restriction.lookup_namespace_uri(Some(prefix)) != Some(XSD_NAMESPACE)
                || !matches!(
                    local_name,
                    "decimal" | "integer" | "nonNegativeInteger" | "positiveInteger"
                )
            {
                continue;
            }

            let mut min_inclusive = None;
            let mut max_inclusive = None;
            let mut min_exclusive = None;

            for facet in &facets {
                let value = facet.attribute("value").ok_or_else(|| {
                    NumericBoundRestrictionQualificationError::MissingFacetValue {
                        file_name: expected.file_name.to_owned(),
                        facet: facet.tag_name().name().to_owned(),
                    }
                })?;
                match facet.tag_name().name() {
                    "minInclusive" => min_inclusive = Some(value),
                    "maxInclusive" => max_inclusive = Some(value),
                    "minExclusive" => min_exclusive = Some(value),
                    _ => unreachable!(),
                }
            }

            // Inclusive boundaries must themselves be members of the resulting interval.
            if let Some(probe) = min_inclusive.or(max_inclusive) {
                parse_bound_numeric_restriction(
                    local_name,
                    probe,
                    min_inclusive,
                    max_inclusive,
                    min_exclusive,
                )
                .map_err(|error| invalid_error(expected.file_name, local_name, error))?;
            }

            // An exclusive boundary must parse as the base type and must reject itself.
            if let Some(exclusive) = min_exclusive {
                match parse_bound_numeric_restriction(
                    local_name,
                    exclusive,
                    None,
                    None,
                    Some(exclusive),
                ) {
                    Err(SimpleTypeError::NumericBoundConstraintViolation { .. }) => {}
                    Err(error) => {
                        return Err(invalid_error(expected.file_name, local_name, error));
                    }
                    Ok(_) => {
                        return Err(
                            NumericBoundRestrictionQualificationError::InvalidNumericRestriction {
                                file_name: expected.file_name.to_owned(),
                                base_local_name: local_name.to_owned(),
                                detail: "minExclusive boundary unexpectedly admitted itself"
                                    .to_owned(),
                            },
                        );
                    }
                }
            }

            restriction_count += 1;
            facet_count += facets.len();
            *restrictions_by_base
                .entry(local_name.to_owned())
                .or_insert(0) += 1;
        }
    }

    let expected_by_base = expected_by_base_for(profile);
    let (expected_restrictions, expected_facets) = expected_counts(profile);
    if restriction_count != expected_restrictions
        || facet_count != expected_facets
        || restrictions_by_base != expected_by_base
    {
        return Err(NumericBoundRestrictionQualificationError::InventoryMismatch {
            expected_restrictions,
            actual_restrictions: restriction_count,
            expected_facets,
            actual_facets: facet_count,
            expected_by_base,
            actual_by_base: restrictions_by_base,
        });
    }

    Ok(NumericBoundRestrictionQualification {
        restriction_count,
        facet_count,
        restrictions_by_base,
    })
}

fn invalid_error(
    file_name: &str,
    base_local_name: &str,
    error: SimpleTypeError,
) -> NumericBoundRestrictionQualificationError {
    NumericBoundRestrictionQualificationError::InvalidNumericRestriction {
        file_name: file_name.to_owned(),
        base_local_name: base_local_name.to_owned(),
        detail: error.to_string(),
    }
}

const fn expected_counts(profile: BoundSchemaProfile) -> (usize, usize) {
    match profile {
        BoundSchemaProfile::MusicXml31 => (
            BOUND_DIRECT_NUMERIC_RESTRICTION_COUNT,
            BOUND_DIRECT_NUMERIC_FACET_COUNT,
        ),
        BoundSchemaProfile::MusicXml40 => (
            MUSICXML40_DIRECT_NUMERIC_RESTRICTION_COUNT,
            MUSICXML40_DIRECT_NUMERIC_FACET_COUNT,
        ),
    }
}

#[cfg(test)]
fn expected_by_base() -> BTreeMap<String, usize> {
    expected_by_base_for(BoundSchemaProfile::MusicXml31)
}

fn expected_by_base_for(profile: BoundSchemaProfile) -> BTreeMap<String, usize> {
    let rows: &[(&str, usize)] = match profile {
        BoundSchemaProfile::MusicXml31 => &[
            ("decimal", 5), ("integer", 2), ("nonNegativeInteger", 1),
            ("positiveInteger", 6),
        ],
        BoundSchemaProfile::MusicXml40 => &[
            ("decimal", 5), ("integer", 2), ("nonNegativeInteger", 1),
            ("positiveInteger", 7),
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
    fn exact_direct_numeric_bound_inventory_qualifies() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        let qualification = qualify_bound_direct_numeric_restrictions(&root)
            .expect("bound direct numeric restrictions must qualify");

        assert_eq!(qualification.restriction_count(), 14);
        assert_eq!(qualification.facet_count(), 25);
        assert_eq!(qualification.restrictions_by_base(), &expected_by_base());
    }

    #[test]
    fn r014_intentionally_excludes_the_single_named_base_numeric_restriction() {
        let expected = expected_by_base();
        assert_eq!(expected.values().sum::<usize>(), 14);
        assert_eq!(BOUND_DIRECT_NUMERIC_RESTRICTION_COUNT, 14);
    }
    #[test]
    fn musicxml40_direct_numeric_inventory_qualifies() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let qualification = qualify_bound_direct_numeric_restrictions(&root)
            .expect("bound 4.0 direct numeric restrictions must qualify");
        assert_eq!(qualification.restriction_count(), 15);
        assert_eq!(qualification.facet_count(), 27);
        assert_eq!(qualification.restrictions_by_base().get("positiveInteger"), Some(&7));
    }

}
