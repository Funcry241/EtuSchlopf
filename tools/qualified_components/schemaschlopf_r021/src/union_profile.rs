//! Union simple-type semantics and full simple-type closure for the exact bound MusicXML 3.1 and 4.0 profiles.
//!
//! r017 covers exactly the five bound `xs:union` simple types. Union member order is the XSD 1.0
//! order: `memberTypes` QNames first, followed by anonymous `xs:simpleType` children. Each member
//! receives the original lexical form and applies its own whitespace semantics. The first member
//! that validates determines the union value.

use crate::named_profile::{
    BOUND_NAME_ALIAS_RESTRICTION_COUNT, BOUND_NAMED_NUMERIC_RESTRICTION_COUNT,
    qualify_bound_named_restrictions,
};
use crate::numeric_profile::{
    BOUND_DIRECT_NUMERIC_RESTRICTION_COUNT, qualify_bound_direct_numeric_restrictions,
};
use crate::profile::{BoundSchemaProfile, XSD_NAMESPACE, expected_schema_files};
use crate::qualification::{QualificationError, qualify_bound_schema_bundle};
use crate::restriction_profile::{
    BOUND_MIN_LENGTH_RESTRICTION_COUNT, BOUND_PATTERN_RESTRICTION_COUNT,
    qualify_bound_remaining_restriction_facets,
};
use crate::simple::{AtomicValue, parse_bound_builtin, parse_bound_enumeration_restriction};
use crate::simple_profile::{
    BOUND_ENUMERATION_RESTRICTION_COUNT, qualify_bound_enumeration_restrictions,
};
use roxmltree::{Document, Node};
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub const BOUND_UNION_SIMPLE_TYPE_COUNT: usize = 5;
pub const BOUND_RESTRICTION_SIMPLE_TYPE_COUNT: usize = 136;
pub const BOUND_SIMPLE_TYPE_COUNT: usize = 141;
const MUSICXML40_RESTRICTION_SIMPLE_TYPE_COUNT: usize = 148;
const MUSICXML40_SIMPLE_TYPE_COUNT: usize = 153;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BoundUnionType {
    FontSize,
    NumberOrNormal,
    PositiveIntegerOrEmpty,
    YesNoNumber,
    XmlLanguageOrEmpty,
}

impl BoundUnionType {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::FontSize => "font-size",
            Self::NumberOrNormal => "number-or-normal",
            Self::PositiveIntegerOrEmpty => "positive-integer-or-empty",
            Self::YesNoNumber => "yes-no-number",
            Self::XmlLanguageOrEmpty => "xml.xsd anonymous language-or-empty union",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnionValue {
    member_index: usize,
    value: AtomicValue,
}

impl UnionValue {
    #[must_use]
    pub const fn member_index(&self) -> usize {
        self.member_index
    }

    #[must_use]
    pub fn value(&self) -> &AtomicValue {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum UnionMember {
    Builtin {
        local_name: String,
    },
    Enumeration {
        base_local_name: String,
        enumeration_lexicals: Vec<String>,
    },
}

impl UnionMember {
    fn parse(&self, lexical: &str) -> Result<AtomicValue, String> {
        match self {
            Self::Builtin { local_name } => {
                parse_bound_builtin(local_name, lexical).map_err(|error| error.to_string())
            }
            Self::Enumeration {
                base_local_name,
                enumeration_lexicals,
            } => {
                let values: Vec<&str> = enumeration_lexicals.iter().map(String::as_str).collect();
                parse_bound_enumeration_restriction(base_local_name, lexical, &values)
                    .map_err(|error| error.to_string())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BoundUnion {
    members: Vec<UnionMember>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnionSimpleTypeQualification {
    unions: BTreeMap<BoundUnionType, BoundUnion>,
    simple_type_count: usize,
    restriction_simple_type_count: usize,
}

impl UnionSimpleTypeQualification {
    #[must_use]
    pub fn union_count(&self) -> usize {
        self.unions.len()
    }

    #[must_use]
    pub const fn simple_type_count(&self) -> usize {
        self.simple_type_count
    }

    #[must_use]
    pub const fn restriction_simple_type_count(&self) -> usize {
        self.restriction_simple_type_count
    }

    #[must_use]
    pub const fn simple_type_profile_qualified(&self) -> bool {
        true
    }

    pub fn parse_value(
        &self,
        union_type: BoundUnionType,
        lexical: &str,
    ) -> Result<UnionValue, UnionSimpleTypeValueError> {
        let union = self
            .unions
            .get(&union_type)
            .expect("qualified union inventory contains every bound union");
        let mut member_errors = Vec::with_capacity(union.members.len());

        for (member_index, member) in union.members.iter().enumerate() {
            match member.parse(lexical) {
                Ok(value) => {
                    return Ok(UnionValue {
                        member_index,
                        value,
                    });
                }
                Err(detail) => member_errors.push(detail),
            }
        }

        Err(UnionSimpleTypeValueError::NoMemberMatched {
            union_type,
            lexical: lexical.to_owned(),
            member_errors,
        })
    }
}

#[derive(Debug)]
pub enum UnionSimpleTypeValueError {
    NoMemberMatched {
        union_type: BoundUnionType,
        lexical: String,
        member_errors: Vec<String>,
    },
}

impl fmt::Display for UnionSimpleTypeValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMemberMatched {
                union_type,
                lexical,
                member_errors,
            } => write!(
                formatter,
                "value {lexical:?} matches no member of bound union {}; member failures: {member_errors:?}",
                union_type.label()
            ),
        }
    }
}

impl std::error::Error for UnionSimpleTypeValueError {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EnumerationSignature {
    base: String,
    values: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct UnionSignature {
    member_types: Vec<String>,
    anonymous_members: Vec<EnumerationSignature>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct RestrictionClassCounts {
    enumeration: usize,
    numeric: usize,
    alias: usize,
    pattern: usize,
    min_length: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct SimpleTypeFileCounts {
    total: usize,
    restrictions: usize,
    unions: usize,
}

#[derive(Debug)]
pub enum UnionSimpleTypeQualificationError {
    SchemaQualification(QualificationError),
    PrerequisiteQualification {
        stage: &'static str,
        detail: String,
    },
    Io {
        path: PathBuf,
        detail: String,
    },
    InvalidSchemaDocument {
        file_name: String,
        detail: String,
    },
    InvalidStructure {
        file_name: String,
        detail: String,
    },
    UnionInventoryMismatch {
        expected: String,
        actual: String,
    },
    SimpleTypeClosureMismatch {
        expected_by_file: String,
        actual_by_file: String,
        expected_restrictions: String,
        actual_restrictions: String,
    },
    SemanticProbeFailed {
        union_type: BoundUnionType,
        lexical: String,
        expected_member_index: usize,
        detail: String,
    },
}

impl fmt::Display for UnionSimpleTypeQualificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaQualification(error) => write!(
                formatter,
                "schema bundle must qualify before union simple types: {error}"
            ),
            Self::PrerequisiteQualification { stage, detail } => write!(
                formatter,
                "{stage} must qualify before union/simple-type closure: {detail}"
            ),
            Self::Io { path, detail } => {
                write!(formatter, "I/O error at {}: {detail}", path.display())
            }
            Self::InvalidSchemaDocument { file_name, detail } => write!(
                formatter,
                "{file_name}: invalid schema document during union qualification: {detail}"
            ),
            Self::InvalidStructure { file_name, detail } => {
                write!(formatter, "{file_name}: invalid union/simple-type structure: {detail}")
            }
            Self::UnionInventoryMismatch { expected, actual } => write!(
                formatter,
                "bound union inventory mismatch; expected {expected}, got {actual}"
            ),
            Self::SimpleTypeClosureMismatch {
                expected_by_file,
                actual_by_file,
                expected_restrictions,
                actual_restrictions,
            } => write!(
                formatter,
                "simple-type closure mismatch; by-file expected {expected_by_file}, got {actual_by_file}; restriction classes expected {expected_restrictions}, got {actual_restrictions}"
            ),
            Self::SemanticProbeFailed {
                union_type,
                lexical,
                expected_member_index,
                detail,
            } => write!(
                formatter,
                "bound union {} failed semantic probe {lexical:?}; expected member {expected_member_index}: {detail}",
                union_type.label()
            ),
        }
    }
}

impl std::error::Error for UnionSimpleTypeQualificationError {}

impl From<QualificationError> for UnionSimpleTypeQualificationError {
    fn from(error: QualificationError) -> Self {
        Self::SchemaQualification(error)
    }
}

pub fn qualify_bound_union_simple_types(
    schema_dir: impl AsRef<Path>,
) -> Result<UnionSimpleTypeQualification, UnionSimpleTypeQualificationError> {
    let schema_dir = schema_dir.as_ref();

    // The final simple-type claim is valid only on exact bound bytes and only after every
    // previously qualified restriction family succeeds again in this same runtime proof path.
    let bundle = qualify_bound_schema_bundle(schema_dir)?;
    let profile = bundle.profile();
    qualify_bound_enumeration_restrictions(schema_dir).map_err(|error| {
        UnionSimpleTypeQualificationError::PrerequisiteQualification {
            stage: "enumeration restrictions",
            detail: error.to_string(),
        }
    })?;
    qualify_bound_direct_numeric_restrictions(schema_dir).map_err(|error| {
        UnionSimpleTypeQualificationError::PrerequisiteQualification {
            stage: "direct numeric restrictions",
            detail: error.to_string(),
        }
    })?;
    qualify_bound_named_restrictions(schema_dir).map_err(|error| {
        UnionSimpleTypeQualificationError::PrerequisiteQualification {
            stage: "named restriction chains",
            detail: error.to_string(),
        }
    })?;
    qualify_bound_remaining_restriction_facets(schema_dir).map_err(|error| {
        UnionSimpleTypeQualificationError::PrerequisiteQualification {
            stage: "remaining restriction facets",
            detail: error.to_string(),
        }
    })?;

    let mut actual_signatures = BTreeMap::new();
    let mut unions = BTreeMap::new();
    let mut counts_by_file = BTreeMap::new();
    let mut restriction_classes = RestrictionClassCounts::default();

    for expected in expected_schema_files(profile) {
        let path = schema_dir.join(expected.file_name);
        let text = fs::read_to_string(&path).map_err(|error| {
            UnionSimpleTypeQualificationError::Io {
                path: path.clone(),
                detail: error.to_string(),
            }
        })?;
        let document = Document::parse(&text).map_err(|error| {
            UnionSimpleTypeQualificationError::InvalidSchemaDocument {
                file_name: expected.file_name.to_owned(),
                detail: error.to_string(),
            }
        })?;

        let mut file_counts = SimpleTypeFileCounts::default();

        for simple_type in document.descendants().filter(|node| {
            node.is_element()
                && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                && node.tag_name().name() == "simpleType"
        }) {
            file_counts.total += 1;
            let variety_children: Vec<_> = simple_type
                .children()
                .filter(|node| {
                    node.is_element()
                        && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                        && node.tag_name().name() != "annotation"
                })
                .collect();
            if variety_children.len() != 1 {
                return Err(UnionSimpleTypeQualificationError::InvalidStructure {
                    file_name: expected.file_name.to_owned(),
                    detail: format!(
                        "simpleType {:?} has {} non-annotation XSD variety children",
                        simple_type.attribute("name"),
                        variety_children.len()
                    ),
                });
            }

            match variety_children[0].tag_name().name() {
                "restriction" => {
                    file_counts.restrictions += 1;
                    classify_restriction(
                        expected.file_name,
                        simple_type,
                        variety_children[0],
                        &mut restriction_classes,
                    )?;
                }
                "union" => {
                    file_counts.unions += 1;
                    let union_type = identify_union_type(expected.file_name, simple_type)?;
                    let (signature, bound_union) = compile_union(
                        expected.file_name,
                        &document,
                        variety_children[0],
                    )?;
                    if actual_signatures
                        .insert(union_type, signature)
                        .is_some()
                        || unions.insert(union_type, bound_union).is_some()
                    {
                        return Err(UnionSimpleTypeQualificationError::InvalidStructure {
                            file_name: expected.file_name.to_owned(),
                            detail: format!("duplicate bound union {}", union_type.label()),
                        });
                    }
                }
                "list" => {
                    return Err(UnionSimpleTypeQualificationError::InvalidStructure {
                        file_name: expected.file_name.to_owned(),
                        detail: "xs:list is outside the exact bound simple-type profile".to_owned(),
                    });
                }
                other => {
                    return Err(UnionSimpleTypeQualificationError::InvalidStructure {
                        file_name: expected.file_name.to_owned(),
                        detail: format!("unsupported simpleType variety xs:{other}"),
                    });
                }
            }
        }

        counts_by_file.insert(expected.file_name.to_owned(), file_counts);
    }

    let expected_signatures = expected_union_signatures();
    if actual_signatures != expected_signatures {
        return Err(UnionSimpleTypeQualificationError::UnionInventoryMismatch {
            expected: format!("{expected_signatures:?}"),
            actual: format!("{actual_signatures:?}"),
        });
    }

    let expected_by_file = expected_simple_type_counts_by_file_for(profile);
    let expected_restrictions = expected_restriction_class_counts_for(profile);
    if counts_by_file != expected_by_file || restriction_classes != expected_restrictions {
        return Err(UnionSimpleTypeQualificationError::SimpleTypeClosureMismatch {
            expected_by_file: format!("{expected_by_file:?}"),
            actual_by_file: format!("{counts_by_file:?}"),
            expected_restrictions: format!("{expected_restrictions:?}"),
            actual_restrictions: format!("{restriction_classes:?}"),
        });
    }

    let simple_type_count = counts_by_file.values().map(|counts| counts.total).sum();
    let restriction_simple_type_count = counts_by_file
        .values()
        .map(|counts| counts.restrictions)
        .sum();
    let (expected_simple_types, expected_restriction_simple_types) = expected_simple_type_counts(profile);
    if simple_type_count != expected_simple_types
        || restriction_simple_type_count != expected_restriction_simple_types
        || unions.len() != BOUND_UNION_SIMPLE_TYPE_COUNT
    {
        return Err(UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: "bound schema bundle".to_owned(),
            detail: format!(
                "post-construction simple-type counts changed unexpectedly: total={simple_type_count}, restrictions={restriction_simple_type_count}, unions={}",
                unions.len()
            ),
        });
    }

    let qualification = UnionSimpleTypeQualification {
        unions,
        simple_type_count,
        restriction_simple_type_count,
    };

    for (union_type, lexical, expected_member_index) in semantic_probes() {
        let value = qualification.parse_value(union_type, lexical).map_err(|error| {
            UnionSimpleTypeQualificationError::SemanticProbeFailed {
                union_type,
                lexical: lexical.to_owned(),
                expected_member_index,
                detail: error.to_string(),
            }
        })?;
        if value.member_index() != expected_member_index {
            return Err(UnionSimpleTypeQualificationError::SemanticProbeFailed {
                union_type,
                lexical: lexical.to_owned(),
                expected_member_index,
                detail: format!("selected member {}", value.member_index()),
            });
        }
    }

    Ok(qualification)
}

fn identify_union_type(
    file_name: &str,
    simple_type: Node<'_, '_>,
) -> Result<BoundUnionType, UnionSimpleTypeQualificationError> {
    match (file_name, simple_type.attribute("name")) {
        ("musicxml.xsd", Some("font-size")) => Ok(BoundUnionType::FontSize),
        ("musicxml.xsd", Some("number-or-normal")) => Ok(BoundUnionType::NumberOrNormal),
        ("musicxml.xsd", Some("positive-integer-or-empty")) => {
            Ok(BoundUnionType::PositiveIntegerOrEmpty)
        }
        ("musicxml.xsd", Some("yes-no-number")) => Ok(BoundUnionType::YesNoNumber),
        ("xml.xsd", None) => Ok(BoundUnionType::XmlLanguageOrEmpty),
        (_, name) => Err(UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: format!("unexpected union simpleType {name:?}"),
        }),
    }
}

fn compile_union(
    file_name: &str,
    document: &Document<'_>,
    union_node: Node<'_, '_>,
) -> Result<(UnionSignature, BoundUnion), UnionSimpleTypeQualificationError> {
    let member_types: Vec<String> = union_node
        .attribute("memberTypes")
        .unwrap_or("")
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    let mut members = Vec::new();

    for lexical_qname in &member_types {
        members.push(resolve_member_type(
            file_name,
            document,
            union_node,
            lexical_qname,
        )?);
    }

    let mut anonymous_members = Vec::new();
    for simple_type in union_node.children().filter(|node| {
        node.is_element()
            && node.tag_name().namespace() == Some(XSD_NAMESPACE)
            && node.tag_name().name() == "simpleType"
    }) {
        let (signature, member) = enumeration_member(file_name, simple_type)?;
        anonymous_members.push(signature);
        members.push(member);
    }

    if members.is_empty() {
        return Err(UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: "bound union has no members".to_owned(),
        });
    }

    Ok((
        UnionSignature {
            member_types,
            anonymous_members,
        },
        BoundUnion { members },
    ))
}

fn resolve_member_type(
    file_name: &str,
    document: &Document<'_>,
    union_node: Node<'_, '_>,
    lexical_qname: &str,
) -> Result<UnionMember, UnionSimpleTypeQualificationError> {
    if let Some((prefix, local_name)) = lexical_qname.split_once(':') {
        if prefix.is_empty()
            || local_name.is_empty()
            || local_name.contains(':')
            || union_node.lookup_namespace_uri(Some(prefix)) != Some(XSD_NAMESPACE)
            || !matches!(local_name, "decimal" | "positiveInteger" | "language")
        {
            return Err(UnionSimpleTypeQualificationError::InvalidStructure {
                file_name: file_name.to_owned(),
                detail: format!("unsupported union member QName {lexical_qname:?}"),
            });
        }
        return Ok(UnionMember::Builtin {
            local_name: local_name.to_owned(),
        });
    }

    if !matches!(lexical_qname, "css-font-size" | "yes-no") {
        return Err(UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: format!("unsupported named union member {lexical_qname:?}"),
        });
    }

    let simple_type = document
        .descendants()
        .find(|node| {
            node.is_element()
                && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                && node.tag_name().name() == "simpleType"
                && node.attribute("name") == Some(lexical_qname)
        })
        .ok_or_else(|| UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: format!("named union member {lexical_qname:?} was not found"),
        })?;
    let (_, member) = enumeration_member(file_name, simple_type)?;
    Ok(member)
}

fn enumeration_member(
    file_name: &str,
    simple_type: Node<'_, '_>,
) -> Result<(EnumerationSignature, UnionMember), UnionSimpleTypeQualificationError> {
    let restriction = simple_type
        .children()
        .find(|node| {
            node.is_element()
                && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                && node.tag_name().name() == "restriction"
        })
        .ok_or_else(|| UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: "union member simpleType is not an enumeration restriction".to_owned(),
        })?;
    let lexical_base = restriction.attribute("base").ok_or_else(|| {
        UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: "union enumeration member has no base QName".to_owned(),
        }
    })?;
    let Some((prefix, base_local_name)) = lexical_base.split_once(':') else {
        return Err(UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: format!("union enumeration base {lexical_base:?} is not an XSD QName"),
        });
    };
    if prefix.is_empty()
        || base_local_name.is_empty()
        || base_local_name.contains(':')
        || restriction.lookup_namespace_uri(Some(prefix)) != Some(XSD_NAMESPACE)
        || !matches!(base_local_name, "string" | "token")
    {
        return Err(UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: format!("unsupported union enumeration base {lexical_base:?}"),
        });
    }

    let facets: Vec<_> = restriction
        .children()
        .filter(|node| {
            node.is_element()
                && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                && node.tag_name().name() != "annotation"
        })
        .collect();
    if facets.is_empty() || !facets.iter().all(|node| node.tag_name().name() == "enumeration") {
        return Err(UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: "union member restriction is not enumeration-only".to_owned(),
        });
    }
    let values: Vec<String> = facets
        .iter()
        .map(|node| {
            node.attribute("value")
                .map(str::to_owned)
                .ok_or_else(|| UnionSimpleTypeQualificationError::InvalidStructure {
                    file_name: file_name.to_owned(),
                    detail: "union enumeration facet has no value".to_owned(),
                })
        })
        .collect::<Result<_, _>>()?;

    let signature = EnumerationSignature {
        base: lexical_base.to_owned(),
        values: values.clone(),
    };
    Ok((
        signature,
        UnionMember::Enumeration {
            base_local_name: base_local_name.to_owned(),
            enumeration_lexicals: values,
        },
    ))
}

fn classify_restriction(
    file_name: &str,
    simple_type: Node<'_, '_>,
    restriction: Node<'_, '_>,
    counts: &mut RestrictionClassCounts,
) -> Result<(), UnionSimpleTypeQualificationError> {
    let facets: Vec<&str> = restriction
        .children()
        .filter(|node| {
            node.is_element()
                && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                && node.tag_name().name() != "annotation"
        })
        .map(|node| node.tag_name().name())
        .collect();

    if facets.is_empty() {
        counts.alias += 1;
    } else if facets.iter().all(|name| *name == "enumeration") {
        counts.enumeration += 1;
    } else if facets
        .iter()
        .all(|name| matches!(*name, "minInclusive" | "maxInclusive" | "minExclusive"))
    {
        counts.numeric += 1;
    } else if facets.len() == 1 && facets[0] == "pattern" {
        counts.pattern += 1;
    } else if facets.len() == 1 && facets[0] == "minLength" {
        counts.min_length += 1;
    } else {
        return Err(UnionSimpleTypeQualificationError::InvalidStructure {
            file_name: file_name.to_owned(),
            detail: format!(
                "restriction simpleType {:?} has unclassified facets {facets:?}",
                simple_type.attribute("name")
            ),
        });
    }
    Ok(())
}

fn expected_union_signatures() -> BTreeMap<BoundUnionType, UnionSignature> {
    [
        (
            BoundUnionType::FontSize,
            UnionSignature {
                member_types: vec!["xs:decimal".to_owned(), "css-font-size".to_owned()],
                anonymous_members: vec![],
            },
        ),
        (
            BoundUnionType::NumberOrNormal,
            UnionSignature {
                member_types: vec!["xs:decimal".to_owned()],
                anonymous_members: vec![EnumerationSignature {
                    base: "xs:token".to_owned(),
                    values: vec!["normal".to_owned()],
                }],
            },
        ),
        (
            BoundUnionType::PositiveIntegerOrEmpty,
            UnionSignature {
                member_types: vec!["xs:positiveInteger".to_owned()],
                anonymous_members: vec![EnumerationSignature {
                    base: "xs:string".to_owned(),
                    values: vec![String::new()],
                }],
            },
        ),
        (
            BoundUnionType::YesNoNumber,
            UnionSignature {
                member_types: vec!["yes-no".to_owned(), "xs:decimal".to_owned()],
                anonymous_members: vec![],
            },
        ),
        (
            BoundUnionType::XmlLanguageOrEmpty,
            UnionSignature {
                member_types: vec!["xs:language".to_owned()],
                anonymous_members: vec![EnumerationSignature {
                    base: "xs:string".to_owned(),
                    values: vec![String::new()],
                }],
            },
        ),
    ]
    .into_iter()
    .collect()
}

const fn expected_simple_type_counts(profile: BoundSchemaProfile) -> (usize, usize) {
    match profile {
        BoundSchemaProfile::MusicXml31 => (BOUND_SIMPLE_TYPE_COUNT, BOUND_RESTRICTION_SIMPLE_TYPE_COUNT),
        BoundSchemaProfile::MusicXml40 => (MUSICXML40_SIMPLE_TYPE_COUNT, MUSICXML40_RESTRICTION_SIMPLE_TYPE_COUNT),
    }
}

fn expected_simple_type_counts_by_file_for(
    profile: BoundSchemaProfile,
) -> BTreeMap<String, SimpleTypeFileCounts> {
    let musicxml = match profile {
        BoundSchemaProfile::MusicXml31 => SimpleTypeFileCounts {
            total: 135,
            restrictions: 131,
            unions: 4,
        },
        BoundSchemaProfile::MusicXml40 => SimpleTypeFileCounts {
            total: 147,
            restrictions: 143,
            unions: 4,
        },
    };
    [
        ("musicxml.xsd".to_owned(), musicxml),
        (
            "xml.xsd".to_owned(),
            SimpleTypeFileCounts { total: 3, restrictions: 2, unions: 1 },
        ),
        (
            "xlink.xsd".to_owned(),
            SimpleTypeFileCounts { total: 3, restrictions: 3, unions: 0 },
        ),
    ]
    .into_iter()
    .collect()
}

fn expected_restriction_class_counts_for(profile: BoundSchemaProfile) -> RestrictionClassCounts {
    match profile {
        BoundSchemaProfile::MusicXml31 => RestrictionClassCounts {
            enumeration: BOUND_ENUMERATION_RESTRICTION_COUNT,
            numeric: BOUND_DIRECT_NUMERIC_RESTRICTION_COUNT + BOUND_NAMED_NUMERIC_RESTRICTION_COUNT,
            alias: BOUND_NAME_ALIAS_RESTRICTION_COUNT,
            pattern: BOUND_PATTERN_RESTRICTION_COUNT,
            min_length: BOUND_MIN_LENGTH_RESTRICTION_COUNT,
        },
        BoundSchemaProfile::MusicXml40 => RestrictionClassCounts {
            enumeration: 104,
            numeric: 16,
            alias: 16,
            pattern: 11,
            min_length: 1,
        },
    }
}

fn semantic_probes() -> [(BoundUnionType, &'static str, usize); 10] {
    [
        (BoundUnionType::FontSize, "12.5", 0),
        (BoundUnionType::FontSize, " medium ", 1),
        (BoundUnionType::NumberOrNormal, "12.5", 0),
        (BoundUnionType::NumberOrNormal, " normal ", 1),
        (BoundUnionType::PositiveIntegerOrEmpty, "1", 0),
        (BoundUnionType::PositiveIntegerOrEmpty, "", 1),
        (BoundUnionType::YesNoNumber, " yes ", 0),
        (BoundUnionType::YesNoNumber, "1", 1),
        (BoundUnionType::XmlLanguageOrEmpty, "en-US", 0),
        (BoundUnionType::XmlLanguageOrEmpty, "", 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MUSICXML31_SCHEMA_DIR, MUSICXML40_SCHEMA_DIR};

    fn qualification() -> UnionSimpleTypeQualification {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        qualify_bound_union_simple_types(&root).expect("bound union profile must qualify")
    }

    #[test]
    fn exact_union_inventory_closes_all_141_bound_simple_types() {
        let qualification = qualification();
        assert_eq!(qualification.union_count(), BOUND_UNION_SIMPLE_TYPE_COUNT);
        assert_eq!(qualification.simple_type_count(), BOUND_SIMPLE_TYPE_COUNT);
        assert_eq!(
            qualification.restriction_simple_type_count(),
            BOUND_RESTRICTION_SIMPLE_TYPE_COUNT
        );
        assert!(qualification.simple_type_profile_qualified());
        assert_eq!(
            BOUND_RESTRICTION_SIMPLE_TYPE_COUNT + BOUND_UNION_SIMPLE_TYPE_COUNT,
            BOUND_SIMPLE_TYPE_COUNT
        );
    }

    #[test]
    fn font_size_accepts_decimal_then_css_font_size_in_declared_order() {
        let qualification = qualification();
        let decimal = qualification
            .parse_value(BoundUnionType::FontSize, "12.5")
            .expect("decimal member must validate");
        assert_eq!(decimal.member_index(), 0);
        assert!(matches!(decimal.value(), AtomicValue::Decimal(_)));

        let css = qualification
            .parse_value(BoundUnionType::FontSize, " medium ")
            .expect("css-font-size member must validate after token collapse");
        assert_eq!(css.member_index(), 1);
        assert_eq!(css.value(), &AtomicValue::String("medium".to_owned()));
    }

    #[test]
    fn number_or_normal_uses_decimal_before_anonymous_normal_member() {
        let qualification = qualification();
        let decimal = qualification
            .parse_value(BoundUnionType::NumberOrNormal, "1")
            .expect("decimal member must validate");
        assert_eq!(decimal.member_index(), 0);

        let normal = qualification
            .parse_value(BoundUnionType::NumberOrNormal, " normal ")
            .expect("anonymous token enumeration must validate");
        assert_eq!(normal.member_index(), 1);
        assert_eq!(normal.value(), &AtomicValue::String("normal".to_owned()));
    }

    #[test]
    fn positive_integer_or_empty_preserves_member_specific_whitespace() {
        let qualification = qualification();
        let integer = qualification
            .parse_value(BoundUnionType::PositiveIntegerOrEmpty, " 1 ")
            .expect("positiveInteger member collapses whitespace");
        assert_eq!(integer.member_index(), 0);

        let empty = qualification
            .parse_value(BoundUnionType::PositiveIntegerOrEmpty, "")
            .expect("anonymous xs:string empty enumeration must validate");
        assert_eq!(empty.member_index(), 1);
        assert_eq!(empty.value(), &AtomicValue::String(String::new()));

        assert!(
            qualification
                .parse_value(BoundUnionType::PositiveIntegerOrEmpty, " ")
                .is_err(),
            "union must not globally collapse a lexical before trying the xs:string member"
        );
    }

    #[test]
    fn yes_no_number_uses_named_enumeration_before_decimal() {
        let qualification = qualification();
        let yes = qualification
            .parse_value(BoundUnionType::YesNoNumber, " yes ")
            .expect("yes-no member must validate");
        assert_eq!(yes.member_index(), 0);
        assert_eq!(yes.value(), &AtomicValue::String("yes".to_owned()));

        let number = qualification
            .parse_value(BoundUnionType::YesNoNumber, "2.5")
            .expect("decimal member must validate");
        assert_eq!(number.member_index(), 1);
        assert!(matches!(number.value(), AtomicValue::Decimal(_)));
    }

    #[test]
    fn xml_language_or_empty_preserves_anonymous_string_whitespace() {
        let qualification = qualification();
        let language = qualification
            .parse_value(BoundUnionType::XmlLanguageOrEmpty, " en-US ")
            .expect("xs:language member must validate after collapse");
        assert_eq!(language.member_index(), 0);
        assert_eq!(language.value(), &AtomicValue::String("en-US".to_owned()));

        let empty = qualification
            .parse_value(BoundUnionType::XmlLanguageOrEmpty, "")
            .expect("anonymous empty xs:string member must validate");
        assert_eq!(empty.member_index(), 1);

        assert!(
            qualification
                .parse_value(BoundUnionType::XmlLanguageOrEmpty, " ")
                .is_err(),
            "member-local whitespace must keep one space distinct from the empty xs:string value"
        );
    }

    #[test]
    fn invalid_union_lexicals_fail_closed_when_no_member_matches() {
        let qualification = qualification();
        assert!(qualification.parse_value(BoundUnionType::FontSize, "normal").is_err());
        assert!(
            qualification
                .parse_value(BoundUnionType::NumberOrNormal, "NORMAL")
                .is_err()
        );
        assert!(
            qualification
                .parse_value(BoundUnionType::YesNoNumber, "true")
                .is_err()
        );
    }
    #[test]
    fn musicxml40_simple_type_profile_closes_with_the_same_five_unions() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let qualification = qualify_bound_union_simple_types(&root)
            .expect("bound 4.0 simple types must qualify");
        assert_eq!(qualification.union_count(), 5);
        assert_eq!(qualification.simple_type_count(), 153);
        assert_eq!(qualification.restriction_simple_type_count(), 148);
    }

}
