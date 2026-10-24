//! Persisted-byte instance-validation closure for the exact hash-bound MusicXML 3.1 and 4.0 score profiles.
//!
//! r020 established the complete 3.1 closure; r021 reuses that runtime after independently
//! qualifying the exact 4.0 score-schema profile. The shared document-wide rules include ID/IDREF
//! consistency, the four built-in `xsi:*` attributes, and the XML 1.0 mandatory UTF-8 / UTF-16 byte
//! encodings. DTD syntax and internal entities are accepted without creating network authority; unresolved external
//! entity dependencies still fail closed. Schema-location hints never replace the bound XSD bytes.

use crate::complex_profile::{ComplexTypeProfileQualificationError, qualify_bound_complex_type_profile};
use crate::profile::{BoundSchemaProfile, XML_NAMESPACE, XSD_NAMESPACE, expected_schema_files};
use crate::restriction_profile::matches_bound_xsd_pattern;
use crate::simple::{
    AtomicValue, WhiteSpaceMode, apply_white_space, compare_numeric_atomic_values,
    parse_bound_builtin,
};
use roxmltree::{Document, Node, ParsingOptions};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const XSI_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema-instance";
const BOUND_NAMED_SIMPLE_TYPE_COUNT: usize = 133;
const BOUND_NAMED_COMPLEX_TYPE_COUNT: usize = 199;
const BOUND_MODEL_GROUP_COUNT: usize = 24;
const BOUND_ATTRIBUTE_GROUP_COUNT: usize = 45;
const BOUND_GLOBAL_ATTRIBUTE_COUNT: usize = 10;
const BOUND_ROOT_ELEMENT_COUNT: usize = 2;
const MAX_TYPE_RECURSION: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstanceFoundationReport {
    root_local_name: String,
    element_count: usize,
    attribute_count: usize,
}

impl InstanceFoundationReport {
    #[must_use]
    pub fn root_local_name(&self) -> &str {
        &self.root_local_name
    }

    #[must_use]
    pub const fn element_count(&self) -> usize {
        self.element_count
    }

    #[must_use]
    pub const fn attribute_count(&self) -> usize {
        self.attribute_count
    }

    #[must_use]
    pub const fn instance_foundation_qualified(&self) -> bool {
        true
    }

    #[must_use]
    pub const fn instance_validation_qualified(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum InstanceFoundationError {
    ComplexProfile(ComplexTypeProfileQualificationError),
    Io { path: PathBuf, detail: String },
    InvalidSchemaDocument { file_name: String, detail: String },
    UnsupportedSchemaModel { file_name: String, detail: String },
    UnsupportedInstanceEncoding,
    InvalidInstanceXml { detail: String },
    UnsupportedInstanceFeature { path: String, detail: String },
    Validation { path: String, detail: String },
}

impl fmt::Display for InstanceFoundationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComplexProfile(error) => write!(
                formatter,
                "complete complex-type profile must qualify before instance foundation: {error}"
            ),
            Self::Io { path, detail } => {
                write!(formatter, "I/O error at {}: {detail}", path.display())
            }
            Self::InvalidSchemaDocument { file_name, detail } => write!(
                formatter,
                "{file_name}: invalid schema while compiling instance foundation: {detail}"
            ),
            Self::UnsupportedSchemaModel { file_name, detail } => write!(
                formatter,
                "{file_name}: unsupported/invalid bound instance schema model: {detail}"
            ),
            Self::UnsupportedInstanceEncoding => write!(
                formatter,
                "bound instance validation supports the XML 1.0 mandatory UTF-8 and UTF-16 byte encodings"
            ),
            Self::InvalidInstanceXml { detail } => {
                write!(formatter, "invalid MusicXML instance XML: {detail}")
            }
            Self::UnsupportedInstanceFeature { path, detail } => {
                write!(formatter, "{path}: unsupported bound instance feature: {detail}")
            }
            Self::Validation { path, detail } => {
                write!(formatter, "{path}: XSD instance validation failed: {detail}")
            }
        }
    }
}

impl std::error::Error for InstanceFoundationError {}

impl From<ComplexTypeProfileQualificationError> for InstanceFoundationError {
    fn from(error: ComplexTypeProfileQualificationError) -> Self {
        Self::ComplexProfile(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct QName {
    namespace_uri: Option<String>,
    local_name: String,
}

impl QName {
    fn display(&self) -> String {
        match self.namespace_uri.as_deref() {
            Some(namespace) => format!("{{{namespace}}}{}", self.local_name),
            None => self.local_name.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SimpleTypeRef {
    Builtin(String),
    Named(QName),
    Anonymous(Box<SimpleTypeDef>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SimpleTypeDef {
    Restriction(RestrictionDef),
    Union(Vec<SimpleTypeRef>),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct RestrictionFacets {
    enumeration: Vec<String>,
    min_inclusive: Option<String>,
    max_inclusive: Option<String>,
    min_exclusive: Option<String>,
    pattern: Option<String>,
    min_length: Option<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RestrictionDef {
    base: SimpleTypeRef,
    facets: RestrictionFacets,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ElementTypeRef {
    Named(QName),
    AnonymousComplex(Box<ComplexTypeDef>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Occurs {
    min: usize,
    max: Option<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ElementDecl {
    name: QName,
    element_type: ElementTypeRef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Particle {
    occurs: Occurs,
    kind: ParticleKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParticleKind {
    Element(ElementDecl),
    Sequence(Vec<Particle>),
    Choice(Vec<Particle>),
    GroupRef(QName),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AttributeUse {
    name: QName,
    simple_type: SimpleTypeRef,
    required: bool,
    default_value: Option<String>,
    fixed_value: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum AttributeItem {
    Use(AttributeUse),
    GroupRef(QName),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ComplexContent {
    Direct(Option<Particle>),
    Simple(SimpleTypeRef),
    ComplexExtension {
        base: QName,
        extension_particle: Option<Particle>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ComplexTypeDef {
    content: ComplexContent,
    attributes: Vec<AttributeItem>,
}

#[derive(Clone, Debug)]
struct SchemaDocumentModel {
    file_name: String,
    target_namespace: Option<String>,
    element_form_qualified: bool,
    attribute_form_qualified: bool,
    xml: String,
}

#[derive(Clone, Debug)]
pub struct BoundInstanceValidator {
    profile: BoundSchemaProfile,
    simple_types: BTreeMap<QName, SimpleTypeDef>,
    complex_types: BTreeMap<QName, ComplexTypeDef>,
    model_groups: BTreeMap<QName, Particle>,
    attribute_groups: BTreeMap<QName, Vec<AttributeItem>>,
    global_attributes: BTreeMap<QName, AttributeUse>,
    roots: BTreeMap<QName, ElementDecl>,
}

#[derive(Clone, Debug)]
struct MatchState {
    position: usize,
    elements: Vec<MatchedElement>,
}

#[derive(Clone, Debug)]
struct MatchedElement {
    child_index: usize,
    declaration: ElementDecl,
}

#[derive(Default)]
struct ValidationCounters {
    elements: usize,
    attributes: usize,
    ids: BTreeMap<String, String>,
    idrefs: Vec<(String, String)>,
}

impl ValidationCounters {
    fn record_id(&mut self, value: String, path: &str) -> Result<(), InstanceFoundationError> {
        if let Some(previous) = self.ids.insert(value.clone(), path.to_owned()) {
            return validation_error(
                path,
                &format!("duplicate xs:ID value {value:?}; first binding at {previous}"),
            );
        }
        Ok(())
    }

    fn record_idref(&mut self, value: String, path: &str) {
        self.idrefs.push((value, path.to_owned()));
    }

    fn verify_idref_closure(&self) -> Result<(), InstanceFoundationError> {
        for (value, path) in &self.idrefs {
            if !self.ids.contains_key(value) {
                return validation_error(path, &format!("xs:IDREF target {value:?} does not exist"));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DecodedInstanceEncoding {
    Utf8,
    Utf16,
}

#[derive(Default)]
struct XsiDirectives {
    type_name: Option<QName>,
    nil: Option<bool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IdentityKind {
    Id,
    IdRef,
}

pub fn compile_bound_instance_validator(
    schema_dir: impl AsRef<Path>,
) -> Result<BoundInstanceValidator, InstanceFoundationError> {
    let schema_dir = schema_dir.as_ref();
    let complex_qualification = qualify_bound_complex_type_profile(schema_dir)?;
    let profile = complex_qualification.profile();

    let documents = load_schema_documents(schema_dir, profile)?;
    let simple_types = compile_named_simple_types(&documents)?;
    let global_attributes = compile_global_attributes(&documents)?;
    let attribute_groups = compile_attribute_groups(&documents, &global_attributes)?;
    let model_groups = compile_model_groups(&documents, &global_attributes)?;
    let complex_types = compile_named_complex_types(&documents, &global_attributes)?;
    let roots = compile_roots(&documents, &global_attributes)?;

    let validator = BoundInstanceValidator {
        profile,
        simple_types,
        complex_types,
        model_groups,
        attribute_groups,
        global_attributes,
        roots,
    };
    validator.verify_inventory()?;
    validator.verify_value_constraints()?;
    Ok(validator)
}

impl BoundInstanceValidator {
    #[must_use]
    pub const fn profile(&self) -> BoundSchemaProfile {
        self.profile
    }

    #[must_use]
    pub const fn instance_foundation_qualified(&self) -> bool {
        true
    }

    #[must_use]
    pub const fn instance_validation_qualified(&self) -> bool {
        true
    }

    pub fn validate_bytes(
        &self,
        instance_bytes: &[u8],
    ) -> Result<InstanceFoundationReport, InstanceFoundationError> {
        let (text, encoding) = decode_instance_bytes(instance_bytes)?;
        self.validate_decoded_str(&text, encoding)
    }

    #[cfg(test)]
    fn validate_str(
        &self,
        instance_xml: &str,
    ) -> Result<InstanceFoundationReport, InstanceFoundationError> {
        self.validate_decoded_str(instance_xml, DecodedInstanceEncoding::Utf8)
    }

    fn validate_decoded_str(
        &self,
        instance_xml: &str,
        encoding: DecodedInstanceEncoding,
    ) -> Result<InstanceFoundationReport, InstanceFoundationError> {
        let options = ParsingOptions {
            allow_dtd: true,
            ..ParsingOptions::default()
        };
        let document = Document::parse_with_options(instance_xml, options).map_err(|error| {
            InstanceFoundationError::InvalidInstanceXml {
                detail: error.to_string(),
            }
        })?;
        verify_supported_dtd_profile(instance_xml).map_err(|detail| {
            InstanceFoundationError::UnsupportedInstanceFeature {
                path: "/".to_owned(),
                detail,
            }
        })?;
        verify_declared_instance_encoding(instance_xml, encoding)?;

        let root = document.root_element();
        let root_name = instance_qname(root);
        let declaration = self.roots.get(&root_name).ok_or_else(|| {
            InstanceFoundationError::Validation {
                path: "/".to_owned(),
                detail: format!("undeclared document element {}", root_name.display()),
            }
        })?;

        let path = format!("/{}", root_name.local_name);
        let mut counters = ValidationCounters::default();
        self.validate_element(declaration, root, &path, &mut counters)?;
        counters.verify_idref_closure()?;

        Ok(InstanceFoundationReport {
            root_local_name: root_name.local_name,
            element_count: counters.elements,
            attribute_count: counters.attributes,
        })
    }

    fn verify_inventory(&self) -> Result<(), InstanceFoundationError> {
        let actual = (
            self.simple_types.len(),
            self.complex_types.len(),
            self.model_groups.len(),
            self.attribute_groups.len(),
            self.global_attributes.len(),
            self.roots.len(),
        );
        let expected = match self.profile {
            BoundSchemaProfile::MusicXml31 => (
                BOUND_NAMED_SIMPLE_TYPE_COUNT,
                BOUND_NAMED_COMPLEX_TYPE_COUNT,
                BOUND_MODEL_GROUP_COUNT,
                BOUND_ATTRIBUTE_GROUP_COUNT,
                BOUND_GLOBAL_ATTRIBUTE_COUNT,
                BOUND_ROOT_ELEMENT_COUNT,
            ),
            BoundSchemaProfile::MusicXml40 => (145, 224, 27, 46, 10, 2),
        };
        if actual != expected {
            return model_error(
                "bound profile",
                &format!("instance runtime inventory expected {expected:?}, got {actual:?}"),
            );
        }
        Ok(())
    }

    fn verify_value_constraints(&self) -> Result<(), InstanceFoundationError> {
        for attribute in self.global_attributes.values() {
            self.verify_attribute_constraint(attribute)?;
        }
        for group in self.attribute_groups.values() {
            for item in group {
                if let AttributeItem::Use(attribute) = item {
                    self.verify_attribute_constraint(attribute)?;
                }
            }
        }
        for complex_type in self.complex_types.values() {
            for item in &complex_type.attributes {
                if let AttributeItem::Use(attribute) = item {
                    self.verify_attribute_constraint(attribute)?;
                }
            }
        }
        for root in self.roots.values() {
            if let ElementTypeRef::AnonymousComplex(complex_type) = &root.element_type {
                self.verify_complex_constraints(complex_type)?;
            }
        }
        Ok(())
    }

    fn verify_complex_constraints(
        &self,
        complex_type: &ComplexTypeDef,
    ) -> Result<(), InstanceFoundationError> {
        for item in &complex_type.attributes {
            if let AttributeItem::Use(attribute) = item {
                self.verify_attribute_constraint(attribute)?;
            }
        }
        if let Some(particle) = own_particle(complex_type) {
            self.verify_particle_constraints(particle)?;
        }
        Ok(())
    }

    fn verify_particle_constraints(
        &self,
        particle: &Particle,
    ) -> Result<(), InstanceFoundationError> {
        match &particle.kind {
            ParticleKind::Element(element) => {
                if let ElementTypeRef::AnonymousComplex(complex_type) = &element.element_type {
                    self.verify_complex_constraints(complex_type)?;
                }
            }
            ParticleKind::Sequence(children) | ParticleKind::Choice(children) => {
                for child in children {
                    self.verify_particle_constraints(child)?;
                }
            }
            ParticleKind::GroupRef(_) => {}
        }
        Ok(())
    }

    fn verify_attribute_constraint(
        &self,
        attribute: &AttributeUse,
    ) -> Result<(), InstanceFoundationError> {
        if let Some(default_value) = attribute.default_value.as_deref() {
            self.parse_simple_value(&attribute.simple_type, default_value, 0)
                .map_err(|detail| InstanceFoundationError::UnsupportedSchemaModel {
                    file_name: "bound profile".to_owned(),
                    detail: format!(
                        "invalid default value {default_value:?} for attribute {}: {detail}",
                        attribute.name.display()
                    ),
                })?;
        }
        if let Some(fixed_value) = attribute.fixed_value.as_deref() {
            self.parse_simple_value(&attribute.simple_type, fixed_value, 0)
                .map_err(|detail| InstanceFoundationError::UnsupportedSchemaModel {
                    file_name: "bound profile".to_owned(),
                    detail: format!(
                        "invalid fixed value {fixed_value:?} for attribute {}: {detail}",
                        attribute.name.display()
                    ),
                })?;
        }
        Ok(())
    }

    fn validate_element(
        &self,
        declaration: &ElementDecl,
        node: Node<'_, '_>,
        path: &str,
        counters: &mut ValidationCounters,
    ) -> Result<(), InstanceFoundationError> {
        let actual_name = instance_qname(node);
        if actual_name != declaration.name {
            return validation_error(
                path,
                &format!(
                    "expected element {}, got {}",
                    declaration.name.display(),
                    actual_name.display()
                ),
            );
        }

        counters.elements += 1;
        let xsi = self.validate_xsi_attributes(node, path, counters)?;
        if xsi.nil.is_some() {
            return validation_error(
                path,
                "xsi:nil is invalid because the bound MusicXML score profile has no nillable element declarations",
            );
        }
        let effective_type = self.effective_element_type(declaration, &xsi, path)?;

        match &effective_type {
            ElementTypeRef::Named(name) => {
                if is_instance_builtin_qname(name) || self.simple_types.contains_key(name) {
                    if node.children().any(|child| child.is_element()) {
                        return validation_error(path, "simple-typed element contains child elements");
                    }
                    if node
                        .attributes()
                        .any(|attribute| attribute.namespace() != Some(XSI_NAMESPACE))
                    {
                        return validation_error(path, "simple-typed element has undeclared attributes");
                    }
                    let lexical = element_text(node);
                    let simple_ref = qname_to_simple_ref(name.clone());
                    let value = self
                        .parse_simple_value(&simple_ref, &lexical, 0)
                        .map_err(|detail| InstanceFoundationError::Validation {
                            path: path.to_owned(),
                            detail,
                        })?;
                    self.record_identity_value(&simple_ref, &value, path, counters)?;
                } else if let Some(complex_type) = self.complex_types.get(name) {
                    self.validate_complex_type(complex_type, node, path, counters)?;
                } else {
                    return validation_error(
                        path,
                        &format!("unresolved element type {}", name.display()),
                    );
                }
            }
            ElementTypeRef::AnonymousComplex(complex_type) => {
                self.validate_complex_type(complex_type, node, path, counters)?;
            }
        }
        Ok(())
    }

    fn effective_element_type(
        &self,
        declaration: &ElementDecl,
        xsi: &XsiDirectives,
        path: &str,
    ) -> Result<ElementTypeRef, InstanceFoundationError> {
        let Some(actual) = xsi.type_name.as_ref() else {
            return Ok(declaration.element_type.clone());
        };
        let ElementTypeRef::Named(declared) = &declaration.element_type else {
            return validation_error(
                path,
                &format!(
                    "xsi:type {} cannot replace an anonymous bound element type",
                    actual.display()
                ),
            );
        };
        if !self.is_known_instance_type(actual) {
            return validation_error(path, &format!("xsi:type {} is not available in the bound schema set", actual.display()));
        }
        if !self.type_is_derived_from(actual, declared, 0)? {
            return validation_error(
                path,
                &format!(
                    "xsi:type {} is not validly derived from declared type {}",
                    actual.display(),
                    declared.display()
                ),
            );
        }
        Ok(ElementTypeRef::Named(actual.clone()))
    }

    fn is_known_instance_type(&self, name: &QName) -> bool {
        is_instance_builtin_qname(name)
            || self.simple_types.contains_key(name)
            || self.complex_types.contains_key(name)
    }

    fn type_is_derived_from(
        &self,
        actual: &QName,
        declared: &QName,
        depth: usize,
    ) -> Result<bool, InstanceFoundationError> {
        if depth > MAX_TYPE_RECURSION {
            return model_error("bound profile", "xsi:type derivation recursion limit exceeded");
        }
        if actual == declared {
            return Ok(true);
        }

        if is_instance_builtin_qname(actual) {
            return Ok(
                is_instance_builtin_qname(declared)
                    && bound_builtin_is_derived_from(
                        actual.local_name.as_str(),
                        declared.local_name.as_str(),
                    ),
            );
        }

        if let Some(simple_type) = self.simple_types.get(actual) {
            let SimpleTypeDef::Restriction(restriction) = simple_type else {
                return Ok(false);
            };
            if let Some(parent) = simple_ref_qname(&restriction.base) {
                return self.type_is_derived_from(&parent, declared, depth + 1);
            }
            return Ok(false);
        }

        if let Some(complex_type) = self.complex_types.get(actual) {
            match &complex_type.content {
                ComplexContent::ComplexExtension { base, .. } => {
                    return self.type_is_derived_from(base, declared, depth + 1);
                }
                ComplexContent::Simple(base) => {
                    if let Some(parent) = simple_ref_qname(base) {
                        return self.type_is_derived_from(&parent, declared, depth + 1);
                    }
                }
                ComplexContent::Direct(_) => {}
            }
        }
        Ok(false)
    }

    fn validate_complex_type(
        &self,
        complex_type: &ComplexTypeDef,
        node: Node<'_, '_>,
        path: &str,
        counters: &mut ValidationCounters,
    ) -> Result<(), InstanceFoundationError> {
        let attributes = self.effective_attributes(complex_type, 0)?;
        self.validate_attributes(&attributes, node, path, counters)?;

        match &complex_type.content {
            ComplexContent::Simple(simple_type) => {
                if node.children().any(|child| child.is_element()) {
                    return validation_error(path, "simpleContent element contains child elements");
                }
                let lexical = element_text(node);
                let value = self
                    .parse_simple_value(simple_type, &lexical, 0)
                    .map_err(|detail| InstanceFoundationError::Validation {
                        path: path.to_owned(),
                        detail,
                    })?;
                self.record_identity_value(simple_type, &value, path, counters)?;
            }
            ComplexContent::Direct(_) | ComplexContent::ComplexExtension { .. } => {
                if has_non_whitespace_text(node) {
                    return validation_error(
                        path,
                        "element-only complex content contains non-whitespace character data",
                    );
                }
                let children: Vec<_> = node.children().filter(|child| child.is_element()).collect();
                let particle = self.effective_particle(complex_type, 0)?;
                match particle {
                    None if children.is_empty() => {}
                    None => return validation_error(path, "complex type declares no child content"),
                    Some(particle) => {
                        let states = self.match_particle(&particle, &children, 0, MatchState {
                            position: 0,
                            elements: Vec::new(),
                        })?;
                        let Some(full) = states.into_iter().find(|state| state.position == children.len()) else {
                            let actual: Vec<_> = children
                                .iter()
                                .map(|child| instance_qname(*child).display())
                                .collect();
                            return validation_error(
                                path,
                                &format!("child sequence does not satisfy bound content model: {actual:?}"),
                            );
                        };
                        for matched in full.elements {
                            let child = children[matched.child_index];
                            let child_name = instance_qname(child);
                            let child_path = format!(
                                "{path}/{}[{}]",
                                child_name.local_name,
                                matched.child_index + 1
                            );
                            self.validate_element(
                                &matched.declaration,
                                child,
                                &child_path,
                                counters,
                            )?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_xsi_attributes(
        &self,
        node: Node<'_, '_>,
        path: &str,
        counters: &mut ValidationCounters,
    ) -> Result<XsiDirectives, InstanceFoundationError> {
        let mut directives = XsiDirectives::default();
        for attribute in node.attributes() {
            if attribute.namespace() != Some(XSI_NAMESPACE) {
                continue;
            }
            counters.attributes += 1;
            match attribute.name() {
                "type" => {
                    let lexical = apply_white_space(WhiteSpaceMode::Collapse, attribute.value());
                    directives.type_name = Some(resolve_instance_qname(node, &lexical).map_err(|detail| {
                        InstanceFoundationError::Validation {
                            path: format!("{path}/@xsi:type"),
                            detail,
                        }
                    })?);
                }
                "nil" => {
                    directives.nil = Some(parse_xsi_boolean(attribute.value()).ok_or_else(|| {
                        InstanceFoundationError::Validation {
                            path: format!("{path}/@xsi:nil"),
                            detail: format!("invalid xsi:nil boolean lexical value {:?}", attribute.value()),
                        }
                    })?);
                }
                "schemaLocation" => {
                    validate_xsi_schema_location(attribute.value()).map_err(|detail| {
                        InstanceFoundationError::Validation {
                            path: format!("{path}/@xsi:schemaLocation"),
                            detail,
                        }
                    })?;
                }
                "noNamespaceSchemaLocation" => {
                    parse_bound_builtin("anyURI", attribute.value()).map_err(|error| {
                        InstanceFoundationError::Validation {
                            path: format!("{path}/@xsi:noNamespaceSchemaLocation"),
                            detail: error.to_string(),
                        }
                    })?;
                }
                other => {
                    return validation_error(
                        path,
                        &format!("undeclared XML Schema instance attribute xsi:{other}"),
                    );
                }
            }
        }
        Ok(directives)
    }

    fn validate_attributes(
        &self,
        expected: &BTreeMap<QName, AttributeUse>,
        node: Node<'_, '_>,
        path: &str,
        counters: &mut ValidationCounters,
    ) -> Result<(), InstanceFoundationError> {
        let mut present = BTreeSet::new();
        for attribute in node.attributes() {
            if attribute.namespace() == Some(XSI_NAMESPACE) {
                continue;
            }
            let name = QName {
                namespace_uri: attribute.namespace().map(str::to_owned),
                local_name: attribute.name().to_owned(),
            };
            let Some(declaration) = expected.get(&name) else {
                return validation_error(path, &format!("undeclared attribute {}", name.display()));
            };
            let value = self
                .parse_simple_value(&declaration.simple_type, attribute.value(), 0)
                .map_err(|detail| InstanceFoundationError::Validation {
                    path: format!("{path}/@{}", name.local_name),
                    detail,
                })?;
            if let Some(fixed) = declaration.fixed_value.as_deref() {
                let fixed_value = self
                    .parse_simple_value(&declaration.simple_type, fixed, 0)
                    .map_err(|detail| InstanceFoundationError::UnsupportedSchemaModel {
                        file_name: "bound profile".to_owned(),
                        detail,
                    })?;
                if value != fixed_value {
                    return validation_error(
                        &format!("{path}/@{}", name.local_name),
                        &format!("value must equal fixed constraint {fixed:?}"),
                    );
                }
            }
            self.record_identity_value(
                &declaration.simple_type,
                &value,
                &format!("{path}/@{}", name.local_name),
                counters,
            )?;
            counters.attributes += 1;
            present.insert(name);
        }

        for (name, declaration) in expected {
            if declaration.required && !present.contains(name) {
                return validation_error(path, &format!("missing required attribute {}", name.display()));
            }
        }
        Ok(())
    }

    fn record_identity_value(
        &self,
        simple_type: &SimpleTypeRef,
        value: &AtomicValue,
        path: &str,
        counters: &mut ValidationCounters,
    ) -> Result<(), InstanceFoundationError> {
        let Some(kind) = self.identity_kind(simple_type, 0)? else {
            return Ok(());
        };
        let AtomicValue::String(identity) = value else {
            return model_error("bound profile", "ID/IDREF value was not represented as a string");
        };
        match kind {
            IdentityKind::Id => counters.record_id(identity.clone(), path),
            IdentityKind::IdRef => {
                counters.record_idref(identity.clone(), path);
                Ok(())
            }
        }
    }

    fn identity_kind(
        &self,
        simple_type: &SimpleTypeRef,
        depth: usize,
    ) -> Result<Option<IdentityKind>, InstanceFoundationError> {
        if depth > MAX_TYPE_RECURSION {
            return model_error("bound profile", "identity-type recursion limit exceeded");
        }
        match simple_type {
            SimpleTypeRef::Builtin(local_name) => Ok(match local_name.as_str() {
                "ID" => Some(IdentityKind::Id),
                "IDREF" => Some(IdentityKind::IdRef),
                _ => None,
            }),
            SimpleTypeRef::Named(name) => {
                let Some(definition) = self.simple_types.get(name) else {
                    return Ok(None);
                };
                match definition {
                    SimpleTypeDef::Restriction(restriction) => {
                        self.identity_kind(&restriction.base, depth + 1)
                    }
                    SimpleTypeDef::Union(_) => Ok(None),
                }
            }
            SimpleTypeRef::Anonymous(definition) => match definition.as_ref() {
                SimpleTypeDef::Restriction(restriction) => {
                    self.identity_kind(&restriction.base, depth + 1)
                }
                SimpleTypeDef::Union(_) => Ok(None),
            },
        }
    }

    fn effective_attributes(
        &self,
        complex_type: &ComplexTypeDef,
        depth: usize,
    ) -> Result<BTreeMap<QName, AttributeUse>, InstanceFoundationError> {
        if depth > MAX_TYPE_RECURSION {
            return model_error("bound profile", "complex attribute recursion limit exceeded");
        }
        let mut output = match &complex_type.content {
            ComplexContent::ComplexExtension { base, .. } => {
                let base_type = self.complex_types.get(base).ok_or_else(|| {
                    InstanceFoundationError::UnsupportedSchemaModel {
                        file_name: "bound profile".to_owned(),
                        detail: format!("unresolved complex extension base {}", base.display()),
                    }
                })?;
                self.effective_attributes(base_type, depth + 1)?
            }
            ComplexContent::Direct(_) | ComplexContent::Simple(_) => BTreeMap::new(),
        };
        self.expand_attribute_items(&complex_type.attributes, &mut output, depth + 1)?;
        Ok(output)
    }

    fn expand_attribute_items(
        &self,
        items: &[AttributeItem],
        output: &mut BTreeMap<QName, AttributeUse>,
        depth: usize,
    ) -> Result<(), InstanceFoundationError> {
        if depth > MAX_TYPE_RECURSION {
            return model_error("bound profile", "attribute-group recursion limit exceeded");
        }
        for item in items {
            match item {
                AttributeItem::Use(attribute) => {
                    output.insert(attribute.name.clone(), attribute.clone());
                }
                AttributeItem::GroupRef(name) => {
                    let group = self.attribute_groups.get(name).ok_or_else(|| {
                        InstanceFoundationError::UnsupportedSchemaModel {
                            file_name: "bound profile".to_owned(),
                            detail: format!("unresolved attribute-group {}", name.display()),
                        }
                    })?;
                    self.expand_attribute_items(group, output, depth + 1)?;
                }
            }
        }
        Ok(())
    }

    fn effective_particle(
        &self,
        complex_type: &ComplexTypeDef,
        depth: usize,
    ) -> Result<Option<Particle>, InstanceFoundationError> {
        if depth > MAX_TYPE_RECURSION {
            return model_error("bound profile", "complex content recursion limit exceeded");
        }
        match &complex_type.content {
            ComplexContent::Direct(particle) => Ok(particle.clone()),
            ComplexContent::Simple(_) => Ok(None),
            ComplexContent::ComplexExtension {
                base,
                extension_particle,
            } => {
                let base_type = self.complex_types.get(base).ok_or_else(|| {
                    InstanceFoundationError::UnsupportedSchemaModel {
                        file_name: "bound profile".to_owned(),
                        detail: format!("unresolved complex extension base {}", base.display()),
                    }
                })?;
                let base_particle = self.effective_particle(base_type, depth + 1)?;
                match (base_particle, extension_particle.clone()) {
                    (None, None) => Ok(None),
                    (Some(particle), None) | (None, Some(particle)) => Ok(Some(particle)),
                    (Some(base_particle), Some(extension_particle)) => Ok(Some(Particle {
                        occurs: Occurs { min: 1, max: Some(1) },
                        kind: ParticleKind::Sequence(vec![base_particle, extension_particle]),
                    })),
                }
            }
        }
    }

    fn match_particle(
        &self,
        particle: &Particle,
        children: &[Node<'_, '_>],
        depth: usize,
        initial: MatchState,
    ) -> Result<Vec<MatchState>, InstanceFoundationError> {
        if depth > MAX_TYPE_RECURSION {
            return model_error("bound profile", "particle recursion limit exceeded");
        }
        let maximum = match particle.occurs.max {
            Some(maximum) => maximum,
            None => children
                .len()
                .saturating_add(particle.occurs.min)
                .saturating_add(1),
        };
        let mut accepted = Vec::new();
        let mut current = vec![initial];

        for repetition in 0..=maximum {
            if repetition >= particle.occurs.min {
                accepted.extend(current.iter().cloned());
            }
            if repetition == maximum || current.is_empty() {
                break;
            }

            let mut next_by_position = BTreeMap::new();
            for state in &current {
                for next in self.match_particle_kind(&particle.kind, children, depth + 1, state.clone())? {
                    next_by_position.entry(next.position).or_insert(next);
                }
            }
            let next: Vec<_> = next_by_position.into_values().collect();
            let no_progress = next.iter().all(|candidate| {
                current
                    .iter()
                    .any(|prior| prior.position == candidate.position)
            });
            current = next;
            if no_progress && repetition + 1 >= particle.occurs.min {
                accepted.extend(current.iter().cloned());
                break;
            }
        }

        let mut unique = BTreeMap::new();
        for state in accepted {
            unique.entry(state.position).or_insert(state);
        }
        Ok(unique.into_values().collect())
    }

    fn match_particle_kind(
        &self,
        kind: &ParticleKind,
        children: &[Node<'_, '_>],
        depth: usize,
        initial: MatchState,
    ) -> Result<Vec<MatchState>, InstanceFoundationError> {
        match kind {
            ParticleKind::Element(declaration) => {
                let Some(child) = children.get(initial.position).copied() else {
                    return Ok(Vec::new());
                };
                if instance_qname(child) != declaration.name {
                    return Ok(Vec::new());
                }
                let mut elements = initial.elements;
                elements.push(MatchedElement {
                    child_index: initial.position,
                    declaration: declaration.clone(),
                });
                Ok(vec![MatchState {
                    position: initial.position + 1,
                    elements,
                }])
            }
            ParticleKind::Sequence(particles) => {
                let mut states = vec![initial];
                for child_particle in particles {
                    let mut next_by_position = BTreeMap::new();
                    for state in states {
                        for next in self.match_particle(child_particle, children, depth + 1, state)? {
                            next_by_position.entry(next.position).or_insert(next);
                        }
                    }
                    states = next_by_position.into_values().collect();
                    if states.is_empty() {
                        break;
                    }
                }
                Ok(states)
            }
            ParticleKind::Choice(particles) => {
                let mut by_position = BTreeMap::new();
                for child_particle in particles {
                    for next in self.match_particle(
                        child_particle,
                        children,
                        depth + 1,
                        initial.clone(),
                    )? {
                        by_position.entry(next.position).or_insert(next);
                    }
                }
                Ok(by_position.into_values().collect())
            }
            ParticleKind::GroupRef(name) => {
                let group = self.model_groups.get(name).ok_or_else(|| {
                    InstanceFoundationError::UnsupportedSchemaModel {
                        file_name: "bound profile".to_owned(),
                        detail: format!("unresolved model group {}", name.display()),
                    }
                })?;
                self.match_particle(group, children, depth + 1, initial)
            }
        }
    }

    fn parse_simple_value(
        &self,
        simple_type: &SimpleTypeRef,
        lexical: &str,
        depth: usize,
    ) -> Result<AtomicValue, String> {
        if depth > MAX_TYPE_RECURSION {
            return Err("simple-type recursion limit exceeded".to_owned());
        }
        match simple_type {
            SimpleTypeRef::Builtin(local_name) => {
                parse_instance_builtin(local_name, lexical)
            }
            SimpleTypeRef::Named(name) => {
                let definition = self
                    .simple_types
                    .get(name)
                    .ok_or_else(|| format!("unresolved simple type {}", name.display()))?;
                self.parse_simple_definition(definition, lexical, depth + 1)
            }
            SimpleTypeRef::Anonymous(definition) => {
                self.parse_simple_definition(definition, lexical, depth + 1)
            }
        }
    }

    fn parse_simple_definition(
        &self,
        definition: &SimpleTypeDef,
        lexical: &str,
        depth: usize,
    ) -> Result<AtomicValue, String> {
        match definition {
            SimpleTypeDef::Restriction(restriction) => {
                let candidate = self.parse_simple_value(&restriction.base, lexical, depth + 1)?;
                let normalized = self.normalized_lexical(&restriction.base, lexical, depth + 1)?;

                if !restriction.facets.enumeration.is_empty() {
                    let mut matched = false;
                    for allowed_lexical in &restriction.facets.enumeration {
                        let allowed = self.parse_simple_value(
                            &restriction.base,
                            allowed_lexical,
                            depth + 1,
                        )?;
                        if allowed == candidate {
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        return Err(format!(
                            "value {lexical:?} violates enumeration restriction"
                        ));
                    }
                }

                self.check_numeric_facet(
                    &restriction.base,
                    &candidate,
                    restriction.facets.min_inclusive.as_deref(),
                    Ordering::Less,
                    "minInclusive",
                    depth + 1,
                )?;
                self.check_numeric_facet(
                    &restriction.base,
                    &candidate,
                    restriction.facets.max_inclusive.as_deref(),
                    Ordering::Greater,
                    "maxInclusive",
                    depth + 1,
                )?;
                if let Some(bound_lexical) = restriction.facets.min_exclusive.as_deref() {
                    let bound = self.parse_simple_value(&restriction.base, bound_lexical, depth + 1)?;
                    let ordering = compare_numeric_atomic_values(&candidate, &bound)
                        .ok_or_else(|| "minExclusive applied to non-numeric value".to_owned())?;
                    if ordering != Ordering::Greater {
                        return Err(format!(
                            "value {lexical:?} violates minExclusive={bound_lexical:?}"
                        ));
                    }
                }

                if let Some(pattern) = restriction.facets.pattern.as_deref() {
                    let matched = matches_bound_xsd_pattern(pattern, &normalized)?;
                    if !matched {
                        return Err(format!(
                            "value {normalized:?} violates XSD pattern {pattern:?}"
                        ));
                    }
                }
                if let Some(minimum) = restriction.facets.min_length {
                    let actual = normalized.chars().count();
                    if actual < minimum {
                        return Err(format!(
                            "value {normalized:?} has XML-character length {actual}, below minLength={minimum}"
                        ));
                    }
                }
                Ok(candidate)
            }
            SimpleTypeDef::Union(members) => {
                let mut failures = Vec::with_capacity(members.len());
                for member in members {
                    match self.parse_simple_value(member, lexical, depth + 1) {
                        Ok(value) => return Ok(value),
                        Err(detail) => failures.push(detail),
                    }
                }
                Err(format!(
                    "value {lexical:?} matches no union member: {failures:?}"
                ))
            }
        }
    }

    fn check_numeric_facet(
        &self,
        base: &SimpleTypeRef,
        candidate: &AtomicValue,
        bound_lexical: Option<&str>,
        forbidden_ordering: Ordering,
        facet_name: &str,
        depth: usize,
    ) -> Result<(), String> {
        let Some(bound_lexical) = bound_lexical else {
            return Ok(());
        };
        let bound = self.parse_simple_value(base, bound_lexical, depth + 1)?;
        let ordering = compare_numeric_atomic_values(candidate, &bound)
            .ok_or_else(|| format!("{facet_name} applied to non-numeric value"))?;
        if ordering == forbidden_ordering {
            return Err(format!(
                "numeric value violates {facet_name}={bound_lexical:?}"
            ));
        }
        Ok(())
    }

    fn normalized_lexical(
        &self,
        simple_type: &SimpleTypeRef,
        lexical: &str,
        depth: usize,
    ) -> Result<String, String> {
        let white_space = self.white_space_mode(simple_type, depth + 1)?;
        Ok(apply_white_space(white_space, lexical))
    }

    fn white_space_mode(
        &self,
        simple_type: &SimpleTypeRef,
        depth: usize,
    ) -> Result<WhiteSpaceMode, String> {
        if depth > MAX_TYPE_RECURSION {
            return Err("simple-type whitespace recursion limit exceeded".to_owned());
        }
        match simple_type {
            SimpleTypeRef::Builtin(local_name) => builtin_white_space(local_name),
            SimpleTypeRef::Named(name) => {
                let definition = self
                    .simple_types
                    .get(name)
                    .ok_or_else(|| format!("unresolved simple type {}", name.display()))?;
                self.definition_white_space(definition, depth + 1)
            }
            SimpleTypeRef::Anonymous(definition) => {
                self.definition_white_space(definition, depth + 1)
            }
        }
    }

    fn definition_white_space(
        &self,
        definition: &SimpleTypeDef,
        depth: usize,
    ) -> Result<WhiteSpaceMode, String> {
        match definition {
            SimpleTypeDef::Restriction(restriction) => {
                self.white_space_mode(&restriction.base, depth + 1)
            }
            SimpleTypeDef::Union(_) => Err(
                "union whitespace is member-specific and cannot be used as a restriction base"
                    .to_owned(),
            ),
        }
    }
}

fn load_schema_documents(
    schema_dir: &Path,
    profile: BoundSchemaProfile,
) -> Result<Vec<SchemaDocumentModel>, InstanceFoundationError> {
    let files = expected_schema_files(profile);
    let mut documents = Vec::with_capacity(files.len());
    for expected in files {
        let path = schema_dir.join(expected.file_name);
        let xml = fs::read_to_string(&path).map_err(|error| InstanceFoundationError::Io {
            path: path.clone(),
            detail: error.to_string(),
        })?;
        let document = Document::parse(&xml).map_err(|error| {
            InstanceFoundationError::InvalidSchemaDocument {
                file_name: expected.file_name.to_owned(),
                detail: error.to_string(),
            }
        })?;
        let schema = document.root_element();
        documents.push(SchemaDocumentModel {
            file_name: expected.file_name.to_owned(),
            target_namespace: expected.target_namespace.map(str::to_owned),
            element_form_qualified: schema.attribute("elementFormDefault") == Some("qualified"),
            attribute_form_qualified: schema.attribute("attributeFormDefault") == Some("qualified"),
            xml,
        });
    }
    Ok(documents)
}

fn compile_named_simple_types(
    documents: &[SchemaDocumentModel],
) -> Result<BTreeMap<QName, SimpleTypeDef>, InstanceFoundationError> {
    let mut output = BTreeMap::new();
    for source in documents {
        let document = parse_source(source)?;
        let schema = document.root_element();
        for node in schema.children().filter(is_xsd_element) {
            if node.tag_name().name() != "simpleType" {
                continue;
            }
            let name = required_attr(&source.file_name, node, "name")?;
            let key = QName {
                namespace_uri: source.target_namespace.clone(),
                local_name: name,
            };
            let definition = compile_simple_type(&source.file_name, node)?;
            output.insert(key, definition);
        }
    }
    Ok(output)
}

fn compile_global_attributes(
    documents: &[SchemaDocumentModel],
) -> Result<BTreeMap<QName, AttributeUse>, InstanceFoundationError> {
    let mut output = BTreeMap::new();
    for source in documents {
        let document = parse_source(source)?;
        let schema = document.root_element();
        for node in schema.children().filter(is_xsd_element) {
            if node.tag_name().name() != "attribute" {
                continue;
            }
            let name = required_attr(&source.file_name, node, "name")?;
            let qname = QName {
                namespace_uri: source.target_namespace.clone(),
                local_name: name,
            };
            let simple_type = compile_attribute_simple_type(&source.file_name, node)?;
            let attribute = AttributeUse {
                name: qname.clone(),
                simple_type,
                required: false,
                default_value: node.attribute("default").map(str::to_owned),
                fixed_value: node.attribute("fixed").map(str::to_owned),
            };
            output.insert(qname, attribute);
        }
    }
    Ok(output)
}

fn compile_attribute_groups(
    documents: &[SchemaDocumentModel],
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<BTreeMap<QName, Vec<AttributeItem>>, InstanceFoundationError> {
    let mut output = BTreeMap::new();
    for source in documents {
        let document = parse_source(source)?;
        let schema = document.root_element();
        for node in schema.children().filter(is_xsd_element) {
            if node.tag_name().name() != "attributeGroup" {
                continue;
            }
            let name = required_attr(&source.file_name, node, "name")?;
            let key = QName {
                namespace_uri: source.target_namespace.clone(),
                local_name: name,
            };
            let items = compile_attribute_items(source, node, global_attributes)?;
            output.insert(key, items);
        }
    }
    Ok(output)
}

fn compile_model_groups(
    documents: &[SchemaDocumentModel],
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<BTreeMap<QName, Particle>, InstanceFoundationError> {
    let mut output = BTreeMap::new();
    for source in documents {
        let document = parse_source(source)?;
        let schema = document.root_element();
        for node in schema.children().filter(is_xsd_element) {
            if node.tag_name().name() != "group" {
                continue;
            }
            let name = required_attr(&source.file_name, node, "name")?;
            let Some(sequence) = structural_children(node).into_iter().next() else {
                return model_error(&source.file_name, "global model group has no sequence");
            };
            let particle = compile_particle(source, sequence, global_attributes)?;
            let key = QName {
                namespace_uri: source.target_namespace.clone(),
                local_name: name,
            };
            output.insert(key, particle);
        }
    }
    Ok(output)
}

fn compile_named_complex_types(
    documents: &[SchemaDocumentModel],
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<BTreeMap<QName, ComplexTypeDef>, InstanceFoundationError> {
    let mut output = BTreeMap::new();
    for source in documents {
        let document = parse_source(source)?;
        let schema = document.root_element();
        for node in schema.children().filter(is_xsd_element) {
            if node.tag_name().name() != "complexType" {
                continue;
            }
            let name = required_attr(&source.file_name, node, "name")?;
            let definition = compile_complex_type(source, node, global_attributes)?;
            let key = QName {
                namespace_uri: source.target_namespace.clone(),
                local_name: name,
            };
            output.insert(key, definition);
        }
    }
    Ok(output)
}

fn compile_roots(
    documents: &[SchemaDocumentModel],
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<BTreeMap<QName, ElementDecl>, InstanceFoundationError> {
    let mut output = BTreeMap::new();
    for source in documents {
        let document = parse_source(source)?;
        let schema = document.root_element();
        for node in schema.children().filter(is_xsd_element) {
            if node.tag_name().name() != "element" {
                continue;
            }
            let declaration = compile_element(source, node, true, global_attributes)?;
            output.insert(declaration.name.clone(), declaration);
        }
    }
    Ok(output)
}

fn parse_source(source: &SchemaDocumentModel) -> Result<Document<'_>, InstanceFoundationError> {
    Document::parse(&source.xml).map_err(|error| InstanceFoundationError::InvalidSchemaDocument {
        file_name: source.file_name.clone(),
        detail: error.to_string(),
    })
}

fn compile_simple_type(
    file_name: &str,
    node: Node<'_, '_>,
) -> Result<SimpleTypeDef, InstanceFoundationError> {
    let children = structural_children(node);
    if children.len() != 1 {
        return model_error(file_name, "simpleType must contain exactly one restriction or union");
    }
    let child = children[0];
    match child.tag_name().name() {
        "restriction" => compile_restriction(file_name, child),
        "union" => compile_union(file_name, child),
        other => model_error(file_name, &format!("unsupported simpleType child xs:{other}")),
    }
}

fn compile_restriction(
    file_name: &str,
    node: Node<'_, '_>,
) -> Result<SimpleTypeDef, InstanceFoundationError> {
    let base = qname_to_simple_ref(resolve_qname(
        file_name,
        node,
        &required_attr(file_name, node, "base")?,
    )?);
    let mut facets = RestrictionFacets::default();
    for facet in node.children().filter(is_xsd_element) {
        if facet.tag_name().name() == "annotation" {
            continue;
        }
        match facet.tag_name().name() {
            "enumeration" => facets
                .enumeration
                .push(required_attr(file_name, facet, "value")?),
            "minInclusive" => {
                set_once(
                    file_name,
                    "minInclusive",
                    &mut facets.min_inclusive,
                    required_attr(file_name, facet, "value")?,
                )?;
            }
            "maxInclusive" => {
                set_once(
                    file_name,
                    "maxInclusive",
                    &mut facets.max_inclusive,
                    required_attr(file_name, facet, "value")?,
                )?;
            }
            "minExclusive" => {
                set_once(
                    file_name,
                    "minExclusive",
                    &mut facets.min_exclusive,
                    required_attr(file_name, facet, "value")?,
                )?;
            }
            "pattern" => {
                set_once(
                    file_name,
                    "pattern",
                    &mut facets.pattern,
                    required_attr(file_name, facet, "value")?,
                )?;
            }
            "minLength" => {
                let lexical = required_attr(file_name, facet, "value")?;
                let value = lexical.parse::<usize>().map_err(|_| {
                    InstanceFoundationError::UnsupportedSchemaModel {
                        file_name: file_name.to_owned(),
                        detail: format!("invalid minLength value {lexical:?}"),
                    }
                })?;
                if facets.min_length.replace(value).is_some() {
                    return model_error(file_name, "duplicate minLength facet");
                }
            }
            other => return model_error(file_name, &format!("unsupported restriction facet xs:{other}")),
        }
    }
    Ok(SimpleTypeDef::Restriction(RestrictionDef { base, facets }))
}

fn compile_union(
    file_name: &str,
    node: Node<'_, '_>,
) -> Result<SimpleTypeDef, InstanceFoundationError> {
    let mut members = Vec::new();
    if let Some(member_types) = node.attribute("memberTypes") {
        for lexical in member_types.split_whitespace() {
            members.push(qname_to_simple_ref(resolve_qname(file_name, node, lexical)?));
        }
    }
    for child in node.children().filter(is_xsd_element) {
        if child.tag_name().name() == "annotation" {
            continue;
        }
        if child.tag_name().name() != "simpleType" {
            return model_error(file_name, "union contains unsupported child");
        }
        members.push(SimpleTypeRef::Anonymous(Box::new(compile_simple_type(
            file_name, child,
        )?)));
    }
    if members.is_empty() {
        return model_error(file_name, "union has no members");
    }
    Ok(SimpleTypeDef::Union(members))
}

fn compile_complex_type(
    source: &SchemaDocumentModel,
    node: Node<'_, '_>,
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<ComplexTypeDef, InstanceFoundationError> {
    let children = structural_children(node);
    if let Some(simple_content) = children
        .iter()
        .find(|child| child.tag_name().name() == "simpleContent")
        .copied()
    {
        let extension = sole_extension(&source.file_name, simple_content)?;
        let base = qname_to_simple_ref(resolve_qname(
            &source.file_name,
            extension,
            &required_attr(&source.file_name, extension, "base")?,
        )?);
        let attributes = compile_attribute_items(source, extension, global_attributes)?;
        return Ok(ComplexTypeDef {
            content: ComplexContent::Simple(base),
            attributes,
        });
    }
    if let Some(complex_content) = children
        .iter()
        .find(|child| child.tag_name().name() == "complexContent")
        .copied()
    {
        let extension = sole_extension(&source.file_name, complex_content)?;
        let base = resolve_qname(
            &source.file_name,
            extension,
            &required_attr(&source.file_name, extension, "base")?,
        )?;
        let extension_particle = first_particle(source, extension, global_attributes)?;
        let attributes = compile_attribute_items(source, extension, global_attributes)?;
        return Ok(ComplexTypeDef {
            content: ComplexContent::ComplexExtension {
                base,
                extension_particle,
            },
            attributes,
        });
    }

    let particle = first_particle(source, node, global_attributes)?;
    let attributes = compile_attribute_items(source, node, global_attributes)?;
    Ok(ComplexTypeDef {
        content: ComplexContent::Direct(particle),
        attributes,
    })
}

fn sole_extension<'a, 'input: 'a>(
    file_name: &str,
    content: Node<'a, 'input>,
) -> Result<Node<'a, 'input>, InstanceFoundationError> {
    let children = structural_children(content);
    if children.len() != 1 || children[0].tag_name().name() != "extension" {
        return model_error(file_name, "simpleContent/complexContent must contain one extension");
    }
    Ok(children[0])
}

fn first_particle(
    source: &SchemaDocumentModel,
    node: Node<'_, '_>,
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<Option<Particle>, InstanceFoundationError> {
    let particles: Vec<_> = node
        .children()
        .filter(is_xsd_element)
        .filter(|child| matches!(child.tag_name().name(), "sequence" | "choice" | "group"))
        .collect();
    if particles.len() > 1 {
        return model_error(&source.file_name, "complex type contains multiple top-level particles");
    }
    particles
        .first()
        .copied()
        .map(|particle| compile_particle(source, particle, global_attributes))
        .transpose()
}

fn compile_particle(
    source: &SchemaDocumentModel,
    node: Node<'_, '_>,
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<Particle, InstanceFoundationError> {
    let occurs = parse_occurs(&source.file_name, node)?;
    let kind = match node.tag_name().name() {
        "element" => ParticleKind::Element(compile_element(
            source,
            node,
            false,
            global_attributes,
        )?),
        "group" => ParticleKind::GroupRef(resolve_qname(
            &source.file_name,
            node,
            &required_attr(&source.file_name, node, "ref")?,
        )?),
        "sequence" => {
            let mut children = Vec::new();
            for child in node.children().filter(is_xsd_element) {
                if child.tag_name().name() == "annotation" {
                    continue;
                }
                if !matches!(child.tag_name().name(), "element" | "group" | "sequence" | "choice") {
                    return model_error(
                        &source.file_name,
                        &format!("unsupported sequence particle xs:{}", child.tag_name().name()),
                    );
                }
                children.push(compile_particle(source, child, global_attributes)?);
            }
            ParticleKind::Sequence(children)
        }
        "choice" => {
            let mut children = Vec::new();
            for child in node.children().filter(is_xsd_element) {
                if child.tag_name().name() == "annotation" {
                    continue;
                }
                if !matches!(child.tag_name().name(), "element" | "group" | "sequence" | "choice") {
                    return model_error(
                        &source.file_name,
                        &format!("unsupported choice particle xs:{}", child.tag_name().name()),
                    );
                }
                children.push(compile_particle(source, child, global_attributes)?);
            }
            ParticleKind::Choice(children)
        }
        other => return model_error(&source.file_name, &format!("unsupported particle xs:{other}")),
    };
    Ok(Particle { occurs, kind })
}

fn compile_element(
    source: &SchemaDocumentModel,
    node: Node<'_, '_>,
    global: bool,
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<ElementDecl, InstanceFoundationError> {
    let local_name = required_attr(&source.file_name, node, "name")?;
    let namespace_uri = if global || element_is_qualified(source, node)? {
        source.target_namespace.clone()
    } else {
        None
    };
    let name = QName {
        namespace_uri,
        local_name,
    };

    let anonymous_complex = child_named(node, "complexType");
    let element_type = match (node.attribute("type"), anonymous_complex) {
        (Some(lexical), None) => ElementTypeRef::Named(resolve_qname(
            &source.file_name,
            node,
            lexical,
        )?),
        (None, Some(complex_type)) => ElementTypeRef::AnonymousComplex(Box::new(
            compile_complex_type(source, complex_type, global_attributes)?,
        )),
        _ => return model_error(
            &source.file_name,
            "element must have exactly one of @type or anonymous complexType",
        ),
    };
    Ok(ElementDecl { name, element_type })
}

fn compile_attribute_items(
    source: &SchemaDocumentModel,
    node: Node<'_, '_>,
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<Vec<AttributeItem>, InstanceFoundationError> {
    let mut items = Vec::new();
    for child in node.children().filter(is_xsd_element) {
        match child.tag_name().name() {
            "attribute" => items.push(AttributeItem::Use(compile_attribute_use(
                source,
                child,
                global_attributes,
            )?)),
            "attributeGroup" => items.push(AttributeItem::GroupRef(resolve_qname(
                &source.file_name,
                child,
                &required_attr(&source.file_name, child, "ref")?,
            )?)),
            _ => {}
        }
    }
    Ok(items)
}

fn compile_attribute_use(
    source: &SchemaDocumentModel,
    node: Node<'_, '_>,
    global_attributes: &BTreeMap<QName, AttributeUse>,
) -> Result<AttributeUse, InstanceFoundationError> {
    let required = match node.attribute("use") {
        None | Some("optional") => false,
        Some("required") => true,
        Some(other) => return model_error(
            &source.file_name,
            &format!("unsupported attribute use {other:?}"),
        ),
    };

    if let Some(reference) = node.attribute("ref") {
        let target = resolve_qname(&source.file_name, node, reference)?;
        let mut attribute = global_attributes.get(&target).cloned().ok_or_else(|| {
            InstanceFoundationError::UnsupportedSchemaModel {
                file_name: source.file_name.clone(),
                detail: format!("unresolved global attribute ref {}", target.display()),
            }
        })?;
        attribute.required = required;
        if let Some(default_value) = node.attribute("default") {
            attribute.default_value = Some(default_value.to_owned());
        }
        if let Some(fixed_value) = node.attribute("fixed") {
            attribute.fixed_value = Some(fixed_value.to_owned());
        }
        return Ok(attribute);
    }

    let local_name = required_attr(&source.file_name, node, "name")?;
    let namespace_uri = if attribute_is_qualified(source, node)? {
        source.target_namespace.clone()
    } else {
        None
    };
    Ok(AttributeUse {
        name: QName {
            namespace_uri,
            local_name,
        },
        simple_type: compile_attribute_simple_type(&source.file_name, node)?,
        required,
        default_value: node.attribute("default").map(str::to_owned),
        fixed_value: node.attribute("fixed").map(str::to_owned),
    })
}

fn compile_attribute_simple_type(
    file_name: &str,
    node: Node<'_, '_>,
) -> Result<SimpleTypeRef, InstanceFoundationError> {
    match (node.attribute("type"), child_named(node, "simpleType")) {
        (Some(lexical), None) => Ok(qname_to_simple_ref(resolve_qname(
            file_name, node, lexical,
        )?)),
        (None, Some(simple_type)) => Ok(SimpleTypeRef::Anonymous(Box::new(
            compile_simple_type(file_name, simple_type)?,
        ))),
        _ => model_error(
            file_name,
            "attribute must have exactly one of @type or anonymous simpleType",
        ),
    }
}

fn element_is_qualified(
    source: &SchemaDocumentModel,
    node: Node<'_, '_>,
) -> Result<bool, InstanceFoundationError> {
    match node.attribute("form") {
        None => Ok(source.element_form_qualified),
        Some("qualified") => Ok(true),
        Some("unqualified") => Ok(false),
        Some(other) => model_error(&source.file_name, &format!("invalid element form {other:?}")),
    }
}

fn attribute_is_qualified(
    source: &SchemaDocumentModel,
    node: Node<'_, '_>,
) -> Result<bool, InstanceFoundationError> {
    match node.attribute("form") {
        None => Ok(source.attribute_form_qualified),
        Some("qualified") => Ok(true),
        Some("unqualified") => Ok(false),
        Some(other) => model_error(&source.file_name, &format!("invalid attribute form {other:?}")),
    }
}

fn parse_occurs(
    file_name: &str,
    node: Node<'_, '_>,
) -> Result<Occurs, InstanceFoundationError> {
    let min = parse_occurs_value(file_name, node.attribute("minOccurs").unwrap_or("1"), false)?
        .expect("minOccurs cannot be unbounded");
    let max = parse_occurs_value(file_name, node.attribute("maxOccurs").unwrap_or("1"), true)?;
    if let Some(maximum) = max
        && min > maximum
    {
        return model_error(file_name, "minOccurs exceeds maxOccurs");
    }
    Ok(Occurs { min, max })
}

fn parse_occurs_value(
    file_name: &str,
    lexical: &str,
    allow_unbounded: bool,
) -> Result<Option<usize>, InstanceFoundationError> {
    if allow_unbounded && lexical == "unbounded" {
        return Ok(None);
    }
    if lexical.is_empty() || !lexical.bytes().all(|byte| byte.is_ascii_digit()) {
        return model_error(file_name, &format!("invalid occurrence value {lexical:?}"));
    }
    let value = lexical.parse::<usize>().map_err(|_| {
        InstanceFoundationError::UnsupportedSchemaModel {
            file_name: file_name.to_owned(),
            detail: format!("occurrence value {lexical:?} exceeds implementation range"),
        }
    })?;
    Ok(Some(value))
}

fn resolve_qname(
    file_name: &str,
    node: Node<'_, '_>,
    lexical: &str,
) -> Result<QName, InstanceFoundationError> {
    let (prefix, local_name) = match lexical.split_once(':') {
        Some((prefix, local_name)) => {
            if prefix.is_empty() || local_name.is_empty() || local_name.contains(':') {
                return model_error(file_name, &format!("invalid QName {lexical:?}"));
            }
            (Some(prefix), local_name)
        }
        None => {
            if lexical.is_empty() {
                return model_error(file_name, "empty QName");
            }
            (None, lexical)
        }
    };
    let namespace_uri = match prefix {
        Some("xml") => Some(XML_NAMESPACE.to_owned()),
        Some(prefix) => Some(
            node.lookup_namespace_uri(Some(prefix))
                .map(str::to_owned)
                .ok_or_else(|| InstanceFoundationError::UnsupportedSchemaModel {
                    file_name: file_name.to_owned(),
                    detail: format!("unbound QName prefix in {lexical:?}"),
                })?,
        ),
        None => node.lookup_namespace_uri(None).map(str::to_owned),
    };
    Ok(QName {
        namespace_uri,
        local_name: local_name.to_owned(),
    })
}

fn parse_instance_builtin(local_name: &str, lexical: &str) -> Result<AtomicValue, String> {
    parse_bound_builtin(local_name, lexical).map_err(|error| error.to_string())
}

fn simple_ref_qname(simple_type: &SimpleTypeRef) -> Option<QName> {
    match simple_type {
        SimpleTypeRef::Builtin(local_name) => Some(QName {
            namespace_uri: Some(XSD_NAMESPACE.to_owned()),
            local_name: local_name.clone(),
        }),
        SimpleTypeRef::Named(name) => Some(name.clone()),
        SimpleTypeRef::Anonymous(_) => None,
    }
}

fn bound_builtin_is_derived_from(actual: &str, declared: &str) -> bool {
    if actual == declared {
        return true;
    }
    match declared {
        "string" => matches!(
            actual,
            "token" | "NMTOKEN" | "NCName" | "ID" | "IDREF" | "language"
        ),
        "token" => matches!(actual, "NMTOKEN" | "NCName" | "ID" | "IDREF" | "language"),
        "NCName" => matches!(actual, "ID" | "IDREF"),
        "decimal" => matches!(actual, "integer" | "nonNegativeInteger" | "positiveInteger"),
        "integer" => matches!(actual, "nonNegativeInteger" | "positiveInteger"),
        "nonNegativeInteger" => actual == "positiveInteger",
        _ => false,
    }
}

fn resolve_instance_qname(node: Node<'_, '_>, lexical: &str) -> Result<QName, String> {
    let lexical = apply_white_space(WhiteSpaceMode::Collapse, lexical);
    let (prefix, local_name) = match lexical.split_once(':') {
        Some((prefix, local_name)) => {
            if prefix.is_empty() || local_name.is_empty() || local_name.contains(':') {
                return Err(format!("invalid QName lexical value {lexical:?}"));
            }
            parse_bound_builtin("NCName", prefix).map_err(|error| error.to_string())?;
            parse_bound_builtin("NCName", local_name).map_err(|error| error.to_string())?;
            (Some(prefix), local_name)
        }
        None => {
            if lexical.is_empty() {
                return Err("empty QName lexical value".to_owned());
            }
            parse_bound_builtin("NCName", &lexical).map_err(|error| error.to_string())?;
            (None, lexical.as_str())
        }
    };
    let namespace_uri = match prefix {
        Some("xml") => Some(XML_NAMESPACE.to_owned()),
        Some(prefix) => Some(
            node.lookup_namespace_uri(Some(prefix))
                .map(str::to_owned)
                .ok_or_else(|| format!("unbound QName prefix in {lexical:?}"))?,
        ),
        None => node.lookup_namespace_uri(None).map(str::to_owned),
    };
    Ok(QName {
        namespace_uri,
        local_name: local_name.to_owned(),
    })
}

fn parse_xsi_boolean(lexical: &str) -> Option<bool> {
    match apply_white_space(WhiteSpaceMode::Collapse, lexical).as_str() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    }
}

fn validate_xsi_schema_location(lexical: &str) -> Result<(), String> {
    let normalized = apply_white_space(WhiteSpaceMode::Collapse, lexical);
    for item in normalized.split_whitespace() {
        parse_bound_builtin("anyURI", item).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn decode_instance_bytes(
    bytes: &[u8],
) -> Result<(String, DecodedInstanceEncoding), InstanceFoundationError> {
    if let Some(rest) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        let text = std::str::from_utf8(rest)
            .map_err(|_| InstanceFoundationError::UnsupportedInstanceEncoding)?;
        return Ok((text.to_owned(), DecodedInstanceEncoding::Utf8));
    }
    if let Some(rest) = bytes.strip_prefix(&[0xFE, 0xFF]) {
        return decode_utf16(rest, false)
            .map(|text| (text, DecodedInstanceEncoding::Utf16));
    }
    if let Some(rest) = bytes.strip_prefix(&[0xFF, 0xFE]) {
        return decode_utf16(rest, true)
            .map(|text| (text, DecodedInstanceEncoding::Utf16));
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|_| InstanceFoundationError::UnsupportedInstanceEncoding)?;
    Ok((text.to_owned(), DecodedInstanceEncoding::Utf8))
}

fn decode_utf16(bytes: &[u8], little_endian: bool) -> Result<String, InstanceFoundationError> {
    if !bytes.len().is_multiple_of(2) {
        return Err(InstanceFoundationError::UnsupportedInstanceEncoding);
    }
    let mut units = Vec::with_capacity(bytes.len() / 2);
    let mut index = 0usize;
    while index < bytes.len() {
        let pair = [bytes[index], bytes[index + 1]];
        units.push(if little_endian {
            u16::from_le_bytes(pair)
        } else {
            u16::from_be_bytes(pair)
        });
        index += 2;
    }
    String::from_utf16(&units).map_err(|_| InstanceFoundationError::UnsupportedInstanceEncoding)
}

fn verify_supported_dtd_profile(text: &str) -> Result<(), String> {
    let Some(subset) = find_internal_dtd_subset(text)? else {
        return Ok(());
    };
    let bytes = subset.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        skip_xml_whitespace(bytes, &mut index);
        if index >= bytes.len() {
            break;
        }
        if let Some(rest) = subset[index..].strip_prefix("<!--") {
            let Some(end) = rest.find("-->") else {
                return Err("unterminated DTD comment".to_owned());
            };
            index += "<!--".len() + end + "-->".len();
            continue;
        }
        if subset[index..].strip_prefix("<!ENTITY").is_none() {
            return Err(
                "internal DTD subset uses declarations outside the qualified general-ENTITY profile"
                    .to_owned(),
            );
        }
        index += "<!ENTITY".len();
        if index >= bytes.len() || !is_xml_whitespace_byte(bytes[index]) {
            return Err("malformed internal ENTITY declaration".to_owned());
        }
        skip_xml_whitespace(bytes, &mut index);
        if index >= bytes.len() || bytes[index] == b'%' {
            return Err("parameter entities are outside the qualified DTD profile".to_owned());
        }
        let name_start = index;
        while index < bytes.len() && !is_xml_whitespace_byte(bytes[index]) {
            index += 1;
        }
        if name_start == index {
            return Err("malformed internal ENTITY declaration".to_owned());
        }
        skip_xml_whitespace(bytes, &mut index);
        if index >= bytes.len() || !matches!(bytes[index], b'\'' | b'"') {
            return Err(
                "external or non-literal ENTITY declarations are outside the qualified DTD profile"
                    .to_owned(),
            );
        }
        let quote = bytes[index];
        index += 1;
        let value_start = index;
        while index < bytes.len() && bytes[index] != quote {
            index += 1;
        }
        if index >= bytes.len() {
            return Err("unterminated internal ENTITY value".to_owned());
        }
        if bytes[value_start..index].contains(&b'%') {
            return Err("parameter-entity references are outside the qualified DTD profile".to_owned());
        }
        index += 1;
        skip_xml_whitespace(bytes, &mut index);
        if index >= bytes.len() || bytes[index] != b'>' {
            return Err("malformed internal ENTITY declaration".to_owned());
        }
        index += 1;
    }
    Ok(())
}

fn find_internal_dtd_subset(text: &str) -> Result<Option<&str>, String> {
    let bytes = text.as_bytes();
    let mut index = 0usize;
    skip_xml_whitespace(bytes, &mut index);
    if let Some(rest) = text[index..].strip_prefix("<?xml") {
        let Some(end) = rest.find("?>") else {
            return Err("unterminated XML declaration".to_owned());
        };
        index += "<?xml".len() + end + "?>".len();
    }

    loop {
        skip_xml_whitespace(bytes, &mut index);
        if index >= bytes.len() {
            return Ok(None);
        }
        if let Some(rest) = text[index..].strip_prefix("<!--") {
            let Some(end) = rest.find("-->") else {
                return Err("unterminated prolog comment".to_owned());
            };
            index += "<!--".len() + end + "-->".len();
            continue;
        }
        if let Some(rest) = text[index..].strip_prefix("<?") {
            let Some(end) = rest.find("?>") else {
                return Err("unterminated prolog processing instruction".to_owned());
            };
            index += "<?".len() + end + "?>".len();
            continue;
        }
        break;
    }

    if text[index..].strip_prefix("<!DOCTYPE").is_none() {
        return Ok(None);
    }
    let mut cursor = index + "<!DOCTYPE".len();
    let mut quote = None::<u8>;
    let mut subset_start = None::<usize>;
    let mut subset_end = None::<usize>;
    let mut bracket_depth = 0usize;
    while cursor < bytes.len() {
        if quote.is_none() && bytes[cursor..].starts_with(b"<!--") {
            let Some(end) = text[cursor + 4..].find("-->") else {
                return Err("unterminated DTD comment".to_owned());
            };
            cursor += 4 + end + 3;
            continue;
        }
        let byte = bytes[cursor];
        if let Some(active_quote) = quote {
            if byte == active_quote {
                quote = None;
            }
            cursor += 1;
            continue;
        }
        if matches!(byte, b'\'' | b'"') {
            quote = Some(byte);
            cursor += 1;
            continue;
        }
        match byte {
            b'[' => {
                if bracket_depth == 0 {
                    subset_start = Some(cursor + 1);
                }
                bracket_depth += 1;
            }
            b']' => {
                if bracket_depth == 0 {
                    return Err("malformed DOCTYPE internal subset".to_owned());
                }
                bracket_depth -= 1;
                if bracket_depth == 0 {
                    subset_end = Some(cursor);
                }
            }
            b'>' if bracket_depth == 0 => {
                return match (subset_start, subset_end) {
                    (None, None) => Ok(None),
                    (Some(start), Some(end)) if start <= end => Ok(Some(&text[start..end])),
                    _ => Err("malformed DOCTYPE internal subset".to_owned()),
                };
            }
            _ => {}
        }
        cursor += 1;
    }
    Err("unterminated DOCTYPE declaration".to_owned())
}

fn skip_xml_whitespace(bytes: &[u8], index: &mut usize) {
    while *index < bytes.len() && is_xml_whitespace_byte(bytes[*index]) {
        *index += 1;
    }
}

const fn is_xml_whitespace_byte(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r' | b'\n')
}

fn verify_declared_instance_encoding(
    text: &str,
    encoding: DecodedInstanceEncoding,
) -> Result<(), InstanceFoundationError> {
    let Some(declared) = xml_declaration_encoding(text) else {
        return Ok(());
    };
    let matches = match encoding {
        DecodedInstanceEncoding::Utf8 => declared.eq_ignore_ascii_case("UTF-8"),
        DecodedInstanceEncoding::Utf16 => declared.eq_ignore_ascii_case("UTF-16"),
    };
    if matches {
        Ok(())
    } else {
        Err(InstanceFoundationError::UnsupportedInstanceEncoding)
    }
}

fn xml_declaration_encoding(text: &str) -> Option<String> {
    let declaration = text.strip_prefix("<?xml")?;
    let end = declaration.find("?>")?;
    let declaration = &declaration[..end];
    let bytes = declaration.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() {
            break;
        }
        let name_start = index;
        while index < bytes.len()
            && !bytes[index].is_ascii_whitespace()
            && bytes[index] != b'='
        {
            index += 1;
        }
        let name = &declaration[name_start..index];
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() || bytes[index] != b'=' {
            return None;
        }
        index += 1;
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() || !matches!(bytes[index], b'\'' | b'"') {
            return None;
        }
        let quote = bytes[index];
        index += 1;
        let value_start = index;
        while index < bytes.len() && bytes[index] != quote {
            index += 1;
        }
        if index >= bytes.len() {
            return None;
        }
        let value = &declaration[value_start..index];
        index += 1;
        if name == "encoding" {
            return Some(value.to_owned());
        }
    }
    None
}

fn qname_to_simple_ref(name: QName) -> SimpleTypeRef {
    if is_instance_builtin_qname(&name) {
        SimpleTypeRef::Builtin(name.local_name)
    } else {
        SimpleTypeRef::Named(name)
    }
}

fn is_instance_builtin_qname(name: &QName) -> bool {
    name.namespace_uri.as_deref() == Some(XSD_NAMESPACE)
        && matches!(
            name.local_name.as_str(),
            "string"
                | "token"
                | "decimal"
                | "integer"
                | "nonNegativeInteger"
                | "positiveInteger"
                | "NMTOKEN"
                | "NCName"
                | "ID"
                | "IDREF"
                | "language"
                | "date"
                | "anyURI"
        )
}

fn builtin_white_space(local_name: &str) -> Result<WhiteSpaceMode, String> {
    match local_name {
        "string" => Ok(WhiteSpaceMode::Preserve),
        "token"
        | "decimal"
        | "integer"
        | "nonNegativeInteger"
        | "positiveInteger"
        | "NMTOKEN"
        | "NCName"
        | "ID"
        | "IDREF"
        | "language"
        | "date"
        | "anyURI" => Ok(WhiteSpaceMode::Collapse),
        _ => Err(format!("unknown bound builtin type {local_name:?}")),
    }
}

fn instance_qname(node: Node<'_, '_>) -> QName {
    QName {
        namespace_uri: node.tag_name().namespace().map(str::to_owned),
        local_name: node.tag_name().name().to_owned(),
    }
}

fn element_text(node: Node<'_, '_>) -> String {
    let mut output = String::new();
    for child in node.children() {
        if child.is_text()
            && let Some(text) = child.text()
        {
            output.push_str(text);
        }
    }
    output
}

fn has_non_whitespace_text(node: Node<'_, '_>) -> bool {
    node.children()
        .filter(|child| child.is_text())
        .filter_map(|child| child.text())
        .any(|text| !text.chars().all(is_xml_space))
}

fn is_xml_space(ch: char) -> bool {
    matches!(ch, '\u{0009}' | '\u{000A}' | '\u{000D}' | '\u{0020}')
}

fn own_particle(complex_type: &ComplexTypeDef) -> Option<&Particle> {
    match &complex_type.content {
        ComplexContent::Direct(particle) => particle.as_ref(),
        ComplexContent::ComplexExtension {
            extension_particle,
            ..
        } => extension_particle.as_ref(),
        ComplexContent::Simple(_) => None,
    }
}

fn structural_children<'a, 'input: 'a>(node: Node<'a, 'input>) -> Vec<Node<'a, 'input>> {
    node.children()
        .filter(is_xsd_element)
        .filter(|child| child.tag_name().name() != "annotation")
        .collect()
}

fn is_xsd_element(node: &Node<'_, '_>) -> bool {
    node.is_element() && node.tag_name().namespace() == Some(XSD_NAMESPACE)
}

fn child_named<'a, 'input: 'a>(
    node: Node<'a, 'input>,
    local_name: &str,
) -> Option<Node<'a, 'input>> {
    node.children().find(|child| {
        child.is_element()
            && child.tag_name().namespace() == Some(XSD_NAMESPACE)
            && child.tag_name().name() == local_name
    })
}

fn required_attr(
    file_name: &str,
    node: Node<'_, '_>,
    name: &str,
) -> Result<String, InstanceFoundationError> {
    node.attribute(name)
        .map(str::to_owned)
        .ok_or_else(|| InstanceFoundationError::UnsupportedSchemaModel {
            file_name: file_name.to_owned(),
            detail: format!("xs:{} is missing required @{name}", node.tag_name().name()),
        })
}

fn set_once(
    file_name: &str,
    facet_name: &str,
    slot: &mut Option<String>,
    value: String,
) -> Result<(), InstanceFoundationError> {
    if slot.replace(value).is_some() {
        return model_error(file_name, &format!("duplicate {facet_name} facet"));
    }
    Ok(())
}

fn model_error<T>(
    file_name: &str,
    detail: &str,
) -> Result<T, InstanceFoundationError> {
    Err(InstanceFoundationError::UnsupportedSchemaModel {
        file_name: file_name.to_owned(),
        detail: detail.to_owned(),
    })
}

fn validation_error<T>(path: &str, detail: &str) -> Result<T, InstanceFoundationError> {
    Err(InstanceFoundationError::Validation {
        path: path.to_owned(),
        detail: detail.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MUSICXML31_SCHEMA_DIR, MUSICXML40_SCHEMA_DIR};

    const MINIMAL_PARTWISE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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

    fn validator() -> BoundInstanceValidator {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        compile_bound_instance_validator(&root).expect("bound instance foundation must compile")
    }

    #[test]
    fn exact_bound_instance_runtime_inventory_compiles() {
        let validator = validator();
        assert_eq!(validator.simple_types.len(), 133);
        assert_eq!(validator.complex_types.len(), 199);
        assert_eq!(validator.model_groups.len(), 24);
        assert_eq!(validator.attribute_groups.len(), 45);
        assert_eq!(validator.global_attributes.len(), 10);
        assert_eq!(validator.roots.len(), 2);
    }

    #[test]
    fn minimal_score_partwise_satisfies_final_instance_validation() {
        let report = validator()
            .validate_str(MINIMAL_PARTWISE)
            .expect("minimal score must satisfy final instance validation");
        assert_eq!(report.root_local_name(), "score-partwise");
        assert_eq!(report.element_count(), 6);
        assert_eq!(report.attribute_count(), 4);
        assert!(report.instance_foundation_qualified());
        assert!(report.instance_validation_qualified());
    }

    #[test]
    fn minimal_score_timewise_uses_the_second_bound_root_after_closure() {
        let xml = r#"<score-timewise version="3.1">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <measure number="1"><part id="P1"/></measure>
</score-timewise>"#;
        let report = validator()
            .validate_str(xml)
            .expect("minimal timewise score must satisfy final instance validation");
        assert_eq!(report.root_local_name(), "score-timewise");
    }

    #[test]
    fn required_content_and_order_are_enforced() {
        let missing_part_list = r#"<score-partwise version="3.1"><part id="P1"><measure number="1"/></part></score-partwise>"#;
        assert!(validator().validate_str(missing_part_list).is_err());

        let wrong_order = r#"<score-partwise version="3.1"><part id="P1"><measure number="1"/></part><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list></score-partwise>"#;
        assert!(validator().validate_str(wrong_order).is_err());
    }

    #[test]
    fn required_attributes_and_local_datatypes_are_enforced() {
        let missing_measure_number = MINIMAL_PARTWISE.replace("<measure number=\"1\"/>", "<measure/>");
        assert!(validator().validate_str(&missing_measure_number).is_err());

        let invalid_id = MINIMAL_PARTWISE.replace("id=\"P1\"", "id=\"1bad\"");
        assert!(validator().validate_str(&invalid_id).is_err());
    }

    #[test]
    fn undeclared_attributes_fail_closed() {
        let xml = MINIMAL_PARTWISE.replace(
            "<score-partwise version=\"3.1\">",
            "<score-partwise version=\"3.1\" mystery=\"x\">",
        );
        assert!(validator().validate_str(&xml).is_err());
    }

    #[test]
    fn document_wide_id_and_idref_closure_is_enforced() {
        let unresolved = MINIMAL_PARTWISE.replace("<part id=\"P1\">", "<part id=\"P2\">");
        let error = validator().validate_str(&unresolved).unwrap_err().to_string();
        assert!(error.contains("IDREF target"));

        let duplicate = MINIMAL_PARTWISE.replace(
            "    </score-part>\n  </part-list>",
            "    </score-part>\n    <score-part id=\"P1\"><part-name>Duplicate</part-name></score-part>\n  </part-list>",
        );
        let error = validator().validate_str(&duplicate).unwrap_err().to_string();
        assert!(error.contains("duplicate xs:ID"));
    }

    #[test]
    fn xsi_builtin_attributes_are_closed_without_schema_hint_authority() {
        let xml = MINIMAL_PARTWISE.replace(
            "<score-partwise version=\"3.1\">",
            "<score-partwise xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" version=\"3.1\" xsi:schemaLocation=\"urn:ignored musicxml.xsd\" xsi:noNamespaceSchemaLocation=\"musicxml.xsd\">",
        );
        assert!(validator().validate_str(&xml).is_ok());

        let nil_false = xml.replace(
            "version=\"3.1\"",
            "version=\"3.1\" xsi:nil=\"false\"",
        );
        assert!(validator().validate_str(&nil_false).is_err());

        let nil_true = xml.replace(
            "version=\"3.1\"",
            "version=\"3.1\" xsi:nil=\"true\"",
        );
        assert!(validator().validate_str(&nil_true).is_err());

        let odd_schema_location = xml.replace(
            "xsi:schemaLocation=\"urn:ignored musicxml.xsd\"",
            "xsi:schemaLocation=\"musicxml.xsd\"",
        );
        assert!(validator().validate_str(&odd_schema_location).is_ok());

        let empty_schema_location = xml.replace(
            "xsi:schemaLocation=\"urn:ignored musicxml.xsd\"",
            "xsi:schemaLocation=\"\"",
        );
        assert!(validator().validate_str(&empty_schema_location).is_ok());
    }

    #[test]
    fn xsi_type_uses_only_valid_derivations_inside_the_bound_type_profile() {
        let valid = MINIMAL_PARTWISE.replace(
            "  <part-list>",
            "  <movement-title xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" xsi:type=\"xs:token\">  Title  </movement-title>\n  <part-list>",
        );
        assert!(validator().validate_str(&valid).is_ok());

        let invalid = valid.replace("xsi:type=\"xs:token\"", "xsi:type=\"xs:decimal\"");
        assert!(validator().validate_str(&invalid).is_err());

        let outside_profile = valid.replace(
            "xsi:type=\"xs:token\"",
            "xsi:type=\"xs:normalizedString\"",
        );
        assert!(validator().validate_str(&outside_profile).is_err());
    }

    #[test]
    fn dtd_syntax_and_internal_entities_are_accepted_offline() {
        let xml = MINIMAL_PARTWISE
            .replace(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE score-partwise PUBLIC \"-//Recordare//DTD MusicXML 3.1 Partwise//EN\" \"http://www.musicxml.org/dtds/partwise.dtd\" [<!ENTITY piano \"Piano\">]>",
            )
            .replace("<part-name>Piano</part-name>", "<part-name>&piano;</part-name>");
        assert!(validator().validate_str(&xml).is_ok());

        let unsupported_subset = MINIMAL_PARTWISE.replace(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE score-partwise [<!ATTLIST score-partwise rogue CDATA \"injected\">]>",
        );
        assert!(validator().validate_str(&unsupported_subset).is_err());
    }

    #[test]
    fn runtime_simple_model_executes_restrictions_and_unions() {
        let validator = validator();
        let named = |local_name: &str| {
            SimpleTypeRef::Named(QName {
                namespace_uri: None,
                local_name: local_name.to_owned(),
            })
        };

        assert!(validator.parse_simple_value(&named("yes-no"), "yes", 0).is_ok());
        assert!(validator.parse_simple_value(&named("yes-no"), "maybe", 0).is_err());
        assert!(validator.parse_simple_value(&named("beam-level"), "8", 0).is_ok());
        assert!(validator.parse_simple_value(&named("beam-level"), "9", 0).is_err());
        assert!(validator.parse_simple_value(&named("color"), "#ABCDEF", 0).is_ok());
        assert!(validator.parse_simple_value(&named("color"), "#abcdef", 0).is_err());
        assert!(validator.parse_simple_value(&named("positive-divisions"), "1", 0).is_ok());
        assert!(validator.parse_simple_value(&named("positive-divisions"), "0", 0).is_err());
        assert!(validator.parse_simple_value(&named("measure-text"), " 1 ", 0).is_ok());
        assert!(validator.parse_simple_value(&named("measure-text"), "   ", 0).is_err());
        assert!(validator.parse_simple_value(&named("yyyy-mm-dd"), "2026-09-05", 0).is_ok());
        assert!(validator.parse_simple_value(&named("yyyy-mm-dd"), "2026-09-05Z", 0).is_err());
        assert!(validator.parse_simple_value(&named("font-size"), "large", 0).is_ok());
        assert!(validator.parse_simple_value(&named("font-size"), "12.5", 0).is_ok());
    }

    #[test]
    fn fixed_and_default_xlink_attribute_constraints_are_compiled_without_psvi_mutation() {
        let validator = validator();
        let link = validator
            .complex_types
            .get(&QName { namespace_uri: None, local_name: "link".to_owned() })
            .expect("link complex type");
        let attributes = validator
            .effective_attributes(link, 0)
            .expect("link attributes must expand");
        let xlink_type = attributes
            .get(&QName {
                namespace_uri: Some("http://www.w3.org/1999/xlink".to_owned()),
                local_name: "type".to_owned(),
            })
            .expect("xlink:type use");
        assert_eq!(xlink_type.fixed_value.as_deref(), Some("simple"));
        let xlink_show = attributes
            .get(&QName {
                namespace_uri: Some("http://www.w3.org/1999/xlink".to_owned()),
                local_name: "show".to_owned(),
            })
            .expect("xlink:show use");
        assert_eq!(xlink_show.default_value.as_deref(), Some("replace"));
    }

    #[test]
    fn utf8_and_utf16_are_the_final_mandatory_xml_byte_profile() {
        assert!(validator().validate_bytes(MINIMAL_PARTWISE.as_bytes()).is_ok());

        let utf16_text = MINIMAL_PARTWISE.replace("encoding=\"UTF-8\"", "encoding=\"UTF-16\"");
        let mut little_endian = vec![0xff, 0xfe];
        let mut big_endian = vec![0xfe, 0xff];
        for unit in utf16_text.encode_utf16() {
            little_endian.extend_from_slice(&unit.to_le_bytes());
            big_endian.extend_from_slice(&unit.to_be_bytes());
        }
        assert!(validator().validate_bytes(&little_endian).is_ok());
        assert!(validator().validate_bytes(&big_endian).is_ok());

        let malformed_utf16 = [0xff, 0xfe, 0x00];
        assert!(matches!(
            validator().validate_bytes(&malformed_utf16),
            Err(InstanceFoundationError::UnsupportedInstanceEncoding)
        ));

        let unsupported_decl = MINIMAL_PARTWISE.replace("UTF-8", "ISO-8859-1");
        assert!(matches!(
            validator().validate_bytes(unsupported_decl.as_bytes()),
            Err(InstanceFoundationError::UnsupportedInstanceEncoding)
        ));
    }
    #[test]
    fn musicxml40_complete_instance_profile_compiles_and_executes_new_simple_chains() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let validator = compile_bound_instance_validator(&root)
            .expect("bound MusicXML 4.0 instance validator must compile");
        assert_eq!(validator.profile(), BoundSchemaProfile::MusicXml40);
        assert_eq!(validator.simple_types.len(), 145);
        assert_eq!(validator.complex_types.len(), 224);
        assert_eq!(validator.model_groups.len(), 27);
        assert_eq!(validator.attribute_groups.len(), 46);
        assert_eq!(validator.global_attributes.len(), 10);
        assert_eq!(validator.roots.len(), 2);

        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1"><part-name>Piano</part-name></score-part>
  </part-list>
  <part id="P1"><measure number="1"/></part>
</score-partwise>"#;
        assert!(validator.validate_bytes(xml.as_bytes()).is_ok());

        let named = |local_name: &str| {
            SimpleTypeRef::Named(QName {
                namespace_uri: None,
                local_name: local_name.to_owned(),
            })
        };
        assert!(validator.parse_simple_value(&named("system-relation"), "only-top", 0).is_ok());
        assert!(validator.parse_simple_value(&named("system-relation"), "only-bottom", 0).is_err());
        assert!(validator.parse_simple_value(&named("swing-type-value"), "eighth", 0).is_ok());
        assert!(validator.parse_simple_value(&named("swing-type-value"), "quarter", 0).is_err());
        assert!(validator.parse_simple_value(&named("font-family"), "Bravura, Arial", 0).is_ok());
        assert!(validator.parse_simple_value(&named("font-family"), ", Arial", 0).is_err());
        assert!(validator.parse_simple_value(&named("staff-line"), "1", 0).is_ok());
        assert!(validator.parse_simple_value(&named("staff-line"), "0", 0).is_err());
        assert!(validator.parse_simple_value(&named("smufl-wavy-line-glyph-name"), "wiggleVibratoSmallestFast", 0).is_ok());
    }

}
