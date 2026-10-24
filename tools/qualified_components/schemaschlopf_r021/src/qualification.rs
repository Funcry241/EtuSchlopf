use crate::profile::{
    BoundSchemaProfile, EXPECTED_IMPORTS, XML_NAMESPACE, XSD_NAMESPACE,
    expected_attribute_inventory_for, expected_element_inventory_for,
    expected_global_symbol_inventory_for, expected_qname_reference_count, expected_schema_files,
    is_bound_builtin_type, is_supported_unqualified_xsd_attribute, is_supported_xsd_element,
};
use roxmltree::{Document, Node};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};


#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SymbolKind {
    Type,
    Element,
    Attribute,
    Group,
    AttributeGroup,
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Type => "type",
            Self::Element => "element",
            Self::Attribute => "attribute",
            Self::Group => "group",
            Self::AttributeGroup => "attributeGroup",
        };
        formatter.write_str(text)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct SymbolKey {
    namespace_uri: Option<String>,
    local_name: String,
    kind: SymbolKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaDocumentSummary {
    pub file_name: String,
    pub byte_len: usize,
    pub sha256: String,
    pub target_namespace: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedQNameReference {
    pub source_file: String,
    pub source_element: String,
    pub source_attribute: String,
    pub lexical_qname: String,
    pub namespace_uri: Option<String>,
    pub local_name: String,
    pub symbol_kind: SymbolKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inventory {
    pub xsd_elements: BTreeMap<String, usize>,
    pub xsd_attributes: BTreeMap<String, usize>,
    pub global_symbols: BTreeMap<String, usize>,
    pub qname_references: usize,
    pub imports: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedSchemaBundle {
    profile: BoundSchemaProfile,
    bundle_id: &'static str,
    documents: Vec<SchemaDocumentSummary>,
    inventory: Inventory,
    qname_references: Vec<ResolvedQNameReference>,
}

impl QualifiedSchemaBundle {
    #[must_use]
    pub const fn profile(&self) -> BoundSchemaProfile {
        self.profile
    }

    #[must_use]
    pub fn bundle_id(&self) -> &'static str {
        self.bundle_id
    }

    #[must_use]
    pub fn documents(&self) -> &[SchemaDocumentSummary] {
        &self.documents
    }

    #[must_use]
    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    #[must_use]
    pub fn qname_references(&self) -> &[ResolvedQNameReference] {
        &self.qname_references
    }

    /// Phase 1 intentionally grants no MusicXML-instance validation authority.
    #[must_use]
    pub const fn instance_validation_qualified(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub enum QualificationError {
    Io {
        path: PathBuf,
        detail: String,
    },
    HashMismatch {
        file_name: String,
        expected: String,
        actual: String,
    },
    ByteLengthMismatch {
        file_name: String,
        expected: usize,
        actual: usize,
    },
    InvalidUtf8 {
        file_name: String,
    },
    UnknownBoundSchemaBundle {
        byte_len: usize,
        sha256: String,
    },
    InvalidSchemaDocument {
        file_name: String,
        detail: String,
    },
    TargetNamespaceMismatch {
        file_name: String,
        expected: Option<String>,
        actual: Option<String>,
    },
    UnsupportedXsdConstruct {
        file_name: String,
        local_name: String,
    },
    UnsupportedXsdAttribute {
        file_name: String,
        element: String,
        attribute: String,
    },
    UnknownExternalResource {
        file_name: String,
        namespace: String,
        schema_location: String,
    },
    ImportTargetMismatch {
        file_name: String,
        imported_file: String,
        expected_namespace: String,
        actual_namespace: Option<String>,
    },
    DuplicateGlobalSymbol {
        file_name: String,
        kind: SymbolKind,
        namespace_uri: Option<String>,
        local_name: String,
    },
    InvalidQName {
        file_name: String,
        lexical_qname: String,
    },
    QNamePrefixUnbound {
        file_name: String,
        lexical_qname: String,
        prefix: String,
    },
    UnsupportedBuiltinType {
        file_name: String,
        local_name: String,
    },
    UnsupportedQNameReferenceContext {
        file_name: String,
        element: String,
        attribute: String,
    },
    UnresolvedQNameReference {
        file_name: String,
        lexical_qname: String,
        kind: SymbolKind,
        namespace_uri: Option<String>,
    },
    InventoryMismatch {
        category: &'static str,
        expected: BTreeMap<String, usize>,
        actual: BTreeMap<String, usize>,
    },
    QNameReferenceCountMismatch {
        expected: usize,
        actual: usize,
    },
    InternalInvariantViolation {
        detail: String,
    },
}

impl fmt::Display for QualificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, detail } => {
                write!(formatter, "I/O error at {}: {detail}", path.display())
            }
            Self::HashMismatch {
                file_name,
                expected,
                actual,
            } => write!(
                formatter,
                "{file_name}: SHA-256 mismatch; expected {expected}, got {actual}"
            ),
            Self::ByteLengthMismatch {
                file_name,
                expected,
                actual,
            } => write!(
                formatter,
                "{file_name}: byte-length mismatch; expected {expected}, got {actual}"
            ),
            Self::InvalidUtf8 { file_name } => {
                write!(formatter, "{file_name}: schema bytes are not UTF-8")
            }
            Self::UnknownBoundSchemaBundle { byte_len, sha256 } => write!(
                formatter,
                "musicxml.xsd is not one of the exact bound MusicXML 3.1 / 4.0 schema bytes; bytes={byte_len}, sha256={sha256}"
            ),
            Self::InvalidSchemaDocument { file_name, detail } => {
                write!(formatter, "{file_name}: invalid schema document: {detail}")
            }
            Self::TargetNamespaceMismatch {
                file_name,
                expected,
                actual,
            } => write!(
                formatter,
                "{file_name}: targetNamespace mismatch; expected {expected:?}, got {actual:?}"
            ),
            Self::UnsupportedXsdConstruct {
                file_name,
                local_name,
            } => write!(
                formatter,
                "{file_name}: unsupported XSD construct xs:{local_name}"
            ),
            Self::UnsupportedXsdAttribute {
                file_name,
                element,
                attribute,
            } => write!(
                formatter,
                "{file_name}: unsupported attribute {attribute} on xs:{element}"
            ),
            Self::UnknownExternalResource {
                file_name,
                namespace,
                schema_location,
            } => write!(
                formatter,
                "{file_name}: external schema resource is not bound locally: namespace={namespace:?}, schemaLocation={schema_location:?}"
            ),
            Self::ImportTargetMismatch {
                file_name,
                imported_file,
                expected_namespace,
                actual_namespace,
            } => write!(
                formatter,
                "{file_name}: local import {imported_file} does not expose expected namespace {expected_namespace:?}; got {actual_namespace:?}"
            ),
            Self::DuplicateGlobalSymbol {
                file_name,
                kind,
                namespace_uri,
                local_name,
            } => write!(
                formatter,
                "{file_name}: duplicate global {kind} {{{namespace_uri:?}}}{local_name}"
            ),
            Self::InvalidQName {
                file_name,
                lexical_qname,
            } => write!(
                formatter,
                "{file_name}: invalid lexical QName {lexical_qname:?}"
            ),
            Self::QNamePrefixUnbound {
                file_name,
                lexical_qname,
                prefix,
            } => write!(
                formatter,
                "{file_name}: QName {lexical_qname:?} uses unbound prefix {prefix:?}"
            ),
            Self::UnsupportedBuiltinType {
                file_name,
                local_name,
            } => write!(
                formatter,
                "{file_name}: builtin XSD type xs:{local_name} is outside the bound profile"
            ),
            Self::UnsupportedQNameReferenceContext {
                file_name,
                element,
                attribute,
            } => write!(
                formatter,
                "{file_name}: QName-valued attribute {attribute:?} is unsupported on xs:{element}"
            ),
            Self::UnresolvedQNameReference {
                file_name,
                lexical_qname,
                kind,
                namespace_uri,
            } => write!(
                formatter,
                "{file_name}: unresolved {kind} QName {lexical_qname:?} in namespace {namespace_uri:?}"
            ),
            Self::InventoryMismatch {
                category,
                expected,
                actual,
            } => write!(
                formatter,
                "{category} inventory mismatch; expected {expected:?}, got {actual:?}"
            ),
            Self::QNameReferenceCountMismatch { expected, actual } => write!(
                formatter,
                "QName-reference count mismatch; expected {expected}, got {actual}"
            ),
            Self::InternalInvariantViolation { detail } => {
                write!(formatter, "internal invariant violation: {detail}")
            }
        }
    }
}

impl std::error::Error for QualificationError {}

struct LoadedSchema {
    file_name: String,
    bytes: Vec<u8>,
    target_namespace: Option<String>,
}

pub fn qualify_musicxml31_schema_bundle(
    schema_dir: impl AsRef<Path>,
) -> Result<QualifiedSchemaBundle, QualificationError> {
    qualify_schema_bundle_as(schema_dir.as_ref(), BoundSchemaProfile::MusicXml31)
}

pub fn qualify_musicxml40_schema_bundle(
    schema_dir: impl AsRef<Path>,
) -> Result<QualifiedSchemaBundle, QualificationError> {
    qualify_schema_bundle_as(schema_dir.as_ref(), BoundSchemaProfile::MusicXml40)
}

pub fn qualify_bound_schema_bundle(
    schema_dir: impl AsRef<Path>,
) -> Result<QualifiedSchemaBundle, QualificationError> {
    let schema_dir = schema_dir.as_ref();
    let path = schema_dir.join("musicxml.xsd");
    let bytes = fs::read(&path).map_err(|error| QualificationError::Io {
        path: path.clone(),
        detail: error.to_string(),
    })?;
    let actual_hash = sha256_hex(&bytes);

    for profile in [BoundSchemaProfile::MusicXml31, BoundSchemaProfile::MusicXml40] {
        let expected = &expected_schema_files(profile)[0];
        if bytes.len() == expected.byte_len && actual_hash == expected.sha256 {
            return qualify_schema_bundle_as(schema_dir, profile);
        }
    }

    Err(QualificationError::UnknownBoundSchemaBundle {
        byte_len: bytes.len(),
        sha256: actual_hash,
    })
}

fn qualify_schema_bundle_as(
    schema_dir: &Path,
    profile: BoundSchemaProfile,
) -> Result<QualifiedSchemaBundle, QualificationError> {
    let files = expected_schema_files(profile);
    let mut loaded = Vec::with_capacity(files.len());

    for expected in files {
        let path = schema_dir.join(expected.file_name);
        let bytes = fs::read(&path).map_err(|error| QualificationError::Io {
            path: path.clone(),
            detail: error.to_string(),
        })?;

        if bytes.len() != expected.byte_len {
            return Err(QualificationError::ByteLengthMismatch {
                file_name: expected.file_name.to_owned(),
                expected: expected.byte_len,
                actual: bytes.len(),
            });
        }

        let actual_hash = sha256_hex(&bytes);
        if actual_hash != expected.sha256 {
            return Err(QualificationError::HashMismatch {
                file_name: expected.file_name.to_owned(),
                expected: expected.sha256.to_owned(),
                actual: actual_hash,
            });
        }

        let text =
            std::str::from_utf8(&bytes).map_err(|_| QualificationError::InvalidUtf8 {
                file_name: expected.file_name.to_owned(),
            })?;
        let document =
            Document::parse(text).map_err(|error| QualificationError::InvalidSchemaDocument {
                file_name: expected.file_name.to_owned(),
                detail: error.to_string(),
            })?;
        let root = document.root_element();
        require_schema_root(expected.file_name, root)?;
        let target_namespace = root.attribute("targetNamespace").map(str::to_owned);
        let expected_namespace = expected.target_namespace.map(str::to_owned);
        if target_namespace != expected_namespace {
            return Err(QualificationError::TargetNamespaceMismatch {
                file_name: expected.file_name.to_owned(),
                expected: expected_namespace,
                actual: target_namespace,
            });
        }

        loaded.push(LoadedSchema {
            file_name: expected.file_name.to_owned(),
            bytes,
            target_namespace,
        });
    }

    qualify_loaded_bundle(profile, &loaded)
}

fn qualify_loaded_bundle(
    profile: BoundSchemaProfile,
    loaded: &[LoadedSchema],
) -> Result<QualifiedSchemaBundle, QualificationError> {
    let target_namespaces: BTreeMap<&str, Option<String>> = loaded
        .iter()
        .map(|schema| (schema.file_name.as_str(), schema.target_namespace.clone()))
        .collect();

    let mut element_inventory = BTreeMap::new();
    let mut attribute_inventory = BTreeMap::new();
    let mut symbols = BTreeSet::new();
    let mut global_symbol_inventory = BTreeMap::new();
    let mut unresolved_references = Vec::new();
    let mut import_count = 0usize;

    for schema in loaded {
        let text = std::str::from_utf8(&schema.bytes).map_err(|_| {
            QualificationError::InvalidUtf8 {
                file_name: schema.file_name.clone(),
            }
        })?;
        let document =
            Document::parse(text).map_err(|error| QualificationError::InvalidSchemaDocument {
                file_name: schema.file_name.clone(),
                detail: error.to_string(),
            })?;
        let root = document.root_element();

        scan_vocabulary(
            &schema.file_name,
            root,
            &mut element_inventory,
            &mut attribute_inventory,
        )?;
        collect_global_symbols(
            &schema.file_name,
            root,
            schema.target_namespace.as_deref(),
            &mut symbols,
            &mut global_symbol_inventory,
        )?;
    }

    for schema in loaded {
        let text = std::str::from_utf8(&schema.bytes).map_err(|_| {
            QualificationError::InvalidUtf8 {
                file_name: schema.file_name.clone(),
            }
        })?;
        let document =
            Document::parse(text).map_err(|error| QualificationError::InvalidSchemaDocument {
                file_name: schema.file_name.clone(),
                detail: error.to_string(),
            })?;
        let root = document.root_element();

        import_count += qualify_imports(&schema.file_name, root, &target_namespaces)?;
        collect_qname_references(&schema.file_name, root, &mut unresolved_references)?;
    }

    let mut resolved_references = Vec::with_capacity(unresolved_references.len());
    for reference in unresolved_references {
        if reference.namespace_uri.as_deref() == Some(XSD_NAMESPACE) {
            if reference.symbol_kind != SymbolKind::Type
                || !is_bound_builtin_type(&reference.local_name)
            {
                return Err(QualificationError::UnsupportedBuiltinType {
                    file_name: reference.source_file,
                    local_name: reference.local_name,
                });
            }
        } else {
            let key = SymbolKey {
                namespace_uri: reference.namespace_uri.clone(),
                local_name: reference.local_name.clone(),
                kind: reference.symbol_kind.clone(),
            };
            if !symbols.contains(&key) {
                return Err(QualificationError::UnresolvedQNameReference {
                    file_name: reference.source_file,
                    lexical_qname: reference.lexical_qname,
                    kind: reference.symbol_kind,
                    namespace_uri: reference.namespace_uri,
                });
            }
        }
        resolved_references.push(reference);
    }

    let expected_elements = expected_element_inventory_for(profile);
    if element_inventory != expected_elements {
        return Err(QualificationError::InventoryMismatch {
            category: "XSD element",
            expected: expected_elements,
            actual: element_inventory,
        });
    }

    let expected_attributes = expected_attribute_inventory_for(profile);
    if attribute_inventory != expected_attributes {
        return Err(QualificationError::InventoryMismatch {
            category: "XSD attribute",
            expected: expected_attributes,
            actual: attribute_inventory,
        });
    }

    let expected_symbols = expected_global_symbol_inventory_for(profile);
    if global_symbol_inventory != expected_symbols {
        return Err(QualificationError::InventoryMismatch {
            category: "global symbol",
            expected: expected_symbols,
            actual: global_symbol_inventory,
        });
    }

    let expected_qnames = expected_qname_reference_count(profile);
    if resolved_references.len() != expected_qnames {
        return Err(QualificationError::QNameReferenceCountMismatch {
            expected: expected_qnames,
            actual: resolved_references.len(),
        });
    }

    let documents = loaded
        .iter()
        .map(|schema| SchemaDocumentSummary {
            file_name: schema.file_name.clone(),
            byte_len: schema.bytes.len(),
            sha256: sha256_hex(&schema.bytes),
            target_namespace: schema.target_namespace.clone(),
        })
        .collect();

    Ok(QualifiedSchemaBundle {
        profile,
        bundle_id: profile.bundle_id(),
        documents,
        inventory: Inventory {
            xsd_elements: element_inventory,
            xsd_attributes: attribute_inventory,
            global_symbols: global_symbol_inventory,
            qname_references: resolved_references.len(),
            imports: import_count,
        },
        qname_references: resolved_references,
    })
}

fn require_schema_root(file_name: &str, root: Node<'_, '_>) -> Result<(), QualificationError> {
    if root.tag_name().namespace() != Some(XSD_NAMESPACE) || root.tag_name().name() != "schema" {
        return Err(QualificationError::InvalidSchemaDocument {
            file_name: file_name.to_owned(),
            detail: format!(
                "root must be xs:schema; got namespace={:?}, local={:?}",
                root.tag_name().namespace(),
                root.tag_name().name()
            ),
        });
    }
    Ok(())
}

fn scan_vocabulary(
    file_name: &str,
    root: Node<'_, '_>,
    element_inventory: &mut BTreeMap<String, usize>,
    attribute_inventory: &mut BTreeMap<String, usize>,
) -> Result<(), QualificationError> {
    for node in root.descendants().filter(|node| node.is_element()) {
        let namespace = node.tag_name().namespace();
        if namespace != Some(XSD_NAMESPACE) {
            continue;
        }

        let local_name = node.tag_name().name();
        if !is_supported_xsd_element(local_name) {
            return Err(QualificationError::UnsupportedXsdConstruct {
                file_name: file_name.to_owned(),
                local_name: local_name.to_owned(),
            });
        }

        *element_inventory.entry(local_name.to_owned()).or_default() += 1;

        for attribute in node.attributes() {
            let key = match attribute.namespace() {
                None if is_supported_unqualified_xsd_attribute(attribute.name()) => {
                    attribute.name().to_owned()
                }
                Some(namespace) if namespace == XML_NAMESPACE && attribute.name() == "lang" => {
                    "xml:lang".to_owned()
                }
                None => {
                    return Err(QualificationError::UnsupportedXsdAttribute {
                        file_name: file_name.to_owned(),
                        element: local_name.to_owned(),
                        attribute: attribute.name().to_owned(),
                    });
                }
                Some(namespace) => {
                    return Err(QualificationError::UnsupportedXsdAttribute {
                        file_name: file_name.to_owned(),
                        element: local_name.to_owned(),
                        attribute: format!("{{{namespace}}}{}", attribute.name()),
                    });
                }
            };
            *attribute_inventory.entry(key).or_default() += 1;
        }
    }
    Ok(())
}

fn collect_global_symbols(
    file_name: &str,
    root: Node<'_, '_>,
    target_namespace: Option<&str>,
    symbols: &mut BTreeSet<SymbolKey>,
    inventory: &mut BTreeMap<String, usize>,
) -> Result<(), QualificationError> {
    for node in root.children().filter(|node| node.is_element()) {
        if node.tag_name().namespace() != Some(XSD_NAMESPACE) {
            continue;
        }

        let kind = match node.tag_name().name() {
            "simpleType" | "complexType" => Some(SymbolKind::Type),
            "element" => Some(SymbolKind::Element),
            "attribute" => Some(SymbolKind::Attribute),
            "group" => Some(SymbolKind::Group),
            "attributeGroup" => Some(SymbolKind::AttributeGroup),
            _ => None,
        };

        let Some(kind) = kind else {
            continue;
        };
        let Some(name) = node.attribute("name") else {
            continue;
        };

        let key = SymbolKey {
            namespace_uri: target_namespace.map(str::to_owned),
            local_name: name.to_owned(),
            kind: kind.clone(),
        };
        if !symbols.insert(key) {
            return Err(QualificationError::DuplicateGlobalSymbol {
                file_name: file_name.to_owned(),
                kind,
                namespace_uri: target_namespace.map(str::to_owned),
                local_name: name.to_owned(),
            });
        }

        *inventory.entry(kind.to_string()).or_default() += 1;
    }
    Ok(())
}

fn qualify_imports(
    file_name: &str,
    root: Node<'_, '_>,
    target_namespaces: &BTreeMap<&str, Option<String>>,
) -> Result<usize, QualificationError> {
    let mut count = 0usize;
    for node in root.descendants().filter(|node| node.is_element()) {
        if node.tag_name().namespace() != Some(XSD_NAMESPACE)
            || node.tag_name().name() != "import"
        {
            continue;
        }

        count += 1;
        let namespace = node.attribute("namespace").unwrap_or("");
        let schema_location = node.attribute("schemaLocation").unwrap_or("");

        let mapping = EXPECTED_IMPORTS.iter().find(|(bound_namespace, bound_location, _)| {
            *bound_namespace == namespace && *bound_location == schema_location
        });

        let Some((expected_namespace, _, imported_file)) = mapping else {
            return Err(QualificationError::UnknownExternalResource {
                file_name: file_name.to_owned(),
                namespace: namespace.to_owned(),
                schema_location: schema_location.to_owned(),
            });
        };

        let Some(actual_namespace) = target_namespaces.get(*imported_file) else {
            return Err(QualificationError::InternalInvariantViolation {
                detail: format!("bound local import {imported_file:?} was not loaded"),
            });
        };
        if actual_namespace.as_deref() != Some(*expected_namespace) {
            return Err(QualificationError::ImportTargetMismatch {
                file_name: file_name.to_owned(),
                imported_file: (*imported_file).to_owned(),
                expected_namespace: (*expected_namespace).to_owned(),
                actual_namespace: actual_namespace.clone(),
            });
        }
    }

    Ok(count)
}

fn collect_qname_references(
    file_name: &str,
    root: Node<'_, '_>,
    output: &mut Vec<ResolvedQNameReference>,
) -> Result<(), QualificationError> {
    for node in root.descendants().filter(|node| node.is_element()) {
        if node.tag_name().namespace() != Some(XSD_NAMESPACE) {
            continue;
        }

        for attribute_name in ["type", "base", "ref", "memberTypes"] {
            let Some(value) = node.attribute(attribute_name) else {
                continue;
            };

            if attribute_name == "memberTypes" {
                for lexical_qname in value.split_whitespace() {
                    output.push(resolve_qname_reference(
                        file_name,
                        node,
                        attribute_name,
                        lexical_qname,
                    )?);
                }
            } else {
                output.push(resolve_qname_reference(
                    file_name,
                    node,
                    attribute_name,
                    value,
                )?);
            }
        }
    }
    Ok(())
}

fn resolve_qname_reference(
    file_name: &str,
    node: Node<'_, '_>,
    attribute_name: &str,
    lexical_qname: &str,
) -> Result<ResolvedQNameReference, QualificationError> {
    let (prefix, local_name) = split_lexical_qname(file_name, lexical_qname)?;

    let namespace_uri = match prefix {
        // XML Namespaces reserves `xml` and binds it implicitly to the XML namespace.
        // Parsers are not required to expose that predefined binding through their
        // namespace lookup API, so XSD QName resolution must handle it explicitly.
        Some("xml") => Some(XML_NAMESPACE.to_owned()),
        Some(prefix) => {
            let Some(namespace) = node.lookup_namespace_uri(Some(prefix)) else {
                return Err(QualificationError::QNamePrefixUnbound {
                    file_name: file_name.to_owned(),
                    lexical_qname: lexical_qname.to_owned(),
                    prefix: prefix.to_owned(),
                });
            };
            Some(namespace.to_owned())
        }
        None => node.lookup_namespace_uri(None).map(str::to_owned),
    };

    let source_element = node.tag_name().name();
    let symbol_kind = match attribute_name {
        "type" | "base" | "memberTypes" => SymbolKind::Type,
        "ref" => match source_element {
            "element" => SymbolKind::Element,
            "attribute" => SymbolKind::Attribute,
            "group" => SymbolKind::Group,
            "attributeGroup" => SymbolKind::AttributeGroup,
            _ => {
                return Err(QualificationError::UnsupportedQNameReferenceContext {
                    file_name: file_name.to_owned(),
                    element: source_element.to_owned(),
                    attribute: attribute_name.to_owned(),
                });
            }
        },
        _ => {
            return Err(QualificationError::UnsupportedQNameReferenceContext {
                file_name: file_name.to_owned(),
                element: source_element.to_owned(),
                attribute: attribute_name.to_owned(),
            });
        }
    };

    Ok(ResolvedQNameReference {
        source_file: file_name.to_owned(),
        source_element: source_element.to_owned(),
        source_attribute: attribute_name.to_owned(),
        lexical_qname: lexical_qname.to_owned(),
        namespace_uri,
        local_name: local_name.to_owned(),
        symbol_kind,
    })
}

fn split_lexical_qname<'a>(
    file_name: &str,
    lexical_qname: &'a str,
) -> Result<(Option<&'a str>, &'a str), QualificationError> {
    if lexical_qname.is_empty() || lexical_qname.chars().any(char::is_whitespace) {
        return Err(QualificationError::InvalidQName {
            file_name: file_name.to_owned(),
            lexical_qname: lexical_qname.to_owned(),
        });
    }

    let mut pieces = lexical_qname.split(':');
    let first = pieces.next().unwrap_or("");
    let second = pieces.next();
    let third = pieces.next();

    match (first.is_empty(), second, third) {
        (false, None, None) => Ok((None, first)),
        (false, Some(local), None) if !local.is_empty() => Ok((Some(first), local)),
        _ => Err(QualificationError::InvalidQName {
            file_name: file_name.to_owned(),
            lexical_qname: lexical_qname.to_owned(),
        }),
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut output, "{byte:02x}");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::{
        BoundSchemaProfile, EXPECTED_SCHEMA_FILES, MUSICXML31_BUNDLE_ID, MUSICXML31_SCHEMA_DIR,
        MUSICXML40_BUNDLE_ID, MUSICXML40_SCHEMA_DIR, XLINK_NAMESPACE, XML_NAMESPACE,
        expected_qname_reference_count,
    };

    #[test]
    fn bound_musicxml_31_bundle_qualifies() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        let bundle = qualify_musicxml31_schema_bundle(root).expect("bound bundle must qualify");

        assert_eq!(bundle.bundle_id(), MUSICXML31_BUNDLE_ID);
        assert_eq!(bundle.documents().len(), 3);
        assert_eq!(bundle.inventory().imports, 2);
        assert_eq!(
            bundle.inventory().qname_references,
            expected_qname_reference_count(BoundSchemaProfile::MusicXml31)
        );
        assert!(!bundle.instance_validation_qualified());
    }


    #[test]
    fn bound_musicxml_40_bundle_qualifies_without_weakening_31_authority() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let bundle = qualify_musicxml40_schema_bundle(&root).expect("bound 4.0 bundle must qualify");
        let detected = qualify_bound_schema_bundle(&root).expect("bound 4.0 bundle must auto-detect");

        assert_eq!(bundle.profile(), BoundSchemaProfile::MusicXml40);
        assert_eq!(bundle.bundle_id(), MUSICXML40_BUNDLE_ID);
        assert_eq!(bundle.documents().len(), 3);
        assert_eq!(bundle.inventory().imports, 2);
        assert_eq!(
            bundle.inventory().qname_references,
            expected_qname_reference_count(BoundSchemaProfile::MusicXml40)
        );
        assert_eq!(detected.profile(), BoundSchemaProfile::MusicXml40);
        assert!(!bundle.instance_validation_qualified());

        let wrong_version = qualify_musicxml31_schema_bundle(&root);
        assert!(wrong_version.is_err());
    }

    #[test]
    fn mixed_musicxml31_and_musicxml40_bundle_fails_closed() {
        let root31 = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        let root40 = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let fixture_root = std::env::temp_dir().join(format!(
            "schemaschlopf_mixed_profile_{}",
            std::process::id()
        ));

        let _ = std::fs::remove_dir_all(&fixture_root);
        std::fs::create_dir_all(&fixture_root).expect("fixture directory must be creatable");
        std::fs::copy(root40.join("musicxml.xsd"), fixture_root.join("musicxml.xsd"))
            .expect("4.0 musicxml.xsd must copy");
        std::fs::copy(root40.join("xml.xsd"), fixture_root.join("xml.xsd"))
            .expect("4.0 xml.xsd must copy");
        std::fs::copy(root31.join("xlink.xsd"), fixture_root.join("xlink.xsd"))
            .expect("3.1 xlink.xsd must copy");

        let result = qualify_bound_schema_bundle(&fixture_root);
        let _ = std::fs::remove_dir_all(&fixture_root);

        assert!(matches!(
            result,
            Err(QualificationError::HashMismatch { file_name, .. })
                if file_name == "xlink.xsd"
        ));
    }

    #[test]
    fn modified_bound_schema_bytes_fail_closed_on_hash_mismatch() {
        let source_root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        let fixture_root = std::env::temp_dir().join(format!(
            "schemaschlopf_hash_mismatch_{}",
            std::process::id()
        ));

        let _ = std::fs::remove_dir_all(&fixture_root);
        std::fs::create_dir_all(&fixture_root).expect("fixture directory must be creatable");

        for expected in EXPECTED_SCHEMA_FILES {
            std::fs::copy(
                source_root.join(expected.file_name),
                fixture_root.join(expected.file_name),
            )
            .expect("bound schema fixture must copy");
        }

        let musicxml_path = fixture_root.join("musicxml.xsd");
        let mut bytes = std::fs::read(&musicxml_path).expect("copied musicxml.xsd must read");
        bytes[0] ^= 0x01;
        std::fs::write(&musicxml_path, bytes).expect("mutated schema fixture must write");

        let result = qualify_musicxml31_schema_bundle(&fixture_root);
        let _ = std::fs::remove_dir_all(&fixture_root);

        assert!(matches!(
            result,
            Err(QualificationError::HashMismatch { file_name, .. })
                if file_name == "musicxml.xsd"
        ));
    }

    #[test]
    fn unknown_xsd_node_fails_closed() {
        let xml = r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:notImplemented/>
        </xs:schema>"#;
        let document = Document::parse(xml).expect("fixture parses");
        let mut elements = BTreeMap::new();
        let mut attributes = BTreeMap::new();

        let result = scan_vocabulary(
            "fixture.xsd",
            document.root_element(),
            &mut elements,
            &mut attributes,
        );

        assert!(matches!(
            result,
            Err(QualificationError::UnsupportedXsdConstruct { .. })
        ));
    }

    #[test]
    fn unknown_xsd_attribute_fails_closed() {
        let xml = r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" surprise="no"/>"#;
        let document = Document::parse(xml).expect("fixture parses");
        let mut elements = BTreeMap::new();
        let mut attributes = BTreeMap::new();

        let result = scan_vocabulary(
            "fixture.xsd",
            document.root_element(),
            &mut elements,
            &mut attributes,
        );

        assert!(matches!(
            result,
            Err(QualificationError::UnsupportedXsdAttribute { .. })
        ));
    }

    #[test]
    fn unknown_external_import_fails_closed() {
        let xml = r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:import namespace="urn:unexpected" schemaLocation="https://example.invalid/unexpected.xsd"/>
        </xs:schema>"#;
        let document = Document::parse(xml).expect("fixture parses");
        let targets = BTreeMap::new();

        let result = qualify_imports("fixture.xsd", document.root_element(), &targets);

        assert!(matches!(
            result,
            Err(QualificationError::UnknownExternalResource { .. })
        ));
    }

    #[test]
    fn qname_with_unbound_prefix_fails_closed() {
        let xml = r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:element name="x" type="missing:kind"/>
        </xs:schema>"#;
        let document = Document::parse(xml).expect("fixture parses");
        let element = document
            .descendants()
            .find(|node| {
                node.is_element()
                    && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                    && node.tag_name().name() == "element"
            })
            .expect("fixture element exists");

        let result = resolve_qname_reference("fixture.xsd", element, "type", "missing:kind");

        assert!(matches!(
            result,
            Err(QualificationError::QNamePrefixUnbound { .. })
        ));
    }

    #[test]
    fn implicit_xml_prefix_resolves_to_xml_namespace() {
        let xml = r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:attribute ref="xml:lang"/>
        </xs:schema>"#;
        let document = Document::parse(xml).expect("fixture parses");
        let attribute = document
            .descendants()
            .find(|node| {
                node.is_element()
                    && node.tag_name().namespace() == Some(XSD_NAMESPACE)
                    && node.tag_name().name() == "attribute"
            })
            .expect("fixture attribute exists");

        let resolved = resolve_qname_reference("fixture.xsd", attribute, "ref", "xml:lang")
            .expect("predefined xml prefix must resolve without an explicit declaration");

        assert_eq!(resolved.namespace_uri.as_deref(), Some(XML_NAMESPACE));
        assert_eq!(resolved.local_name, "lang");
        assert_eq!(resolved.symbol_kind, SymbolKind::Attribute);
    }

    #[test]
    fn lexical_qname_rejects_multiple_colons() {
        assert!(matches!(
            split_lexical_qname("fixture.xsd", "a:b:c"),
            Err(QualificationError::InvalidQName { .. })
        ));
    }

    #[test]
    fn profile_namespace_constants_are_distinct() {
        assert_ne!(XSD_NAMESPACE, XML_NAMESPACE);
        assert_ne!(XSD_NAMESPACE, XLINK_NAMESPACE);
        assert_ne!(XML_NAMESPACE, XLINK_NAMESPACE);
    }
}
