use crate::profile::{BoundSchemaProfile, XML_NAMESPACE, XSD_NAMESPACE, expected_schema_files};
use crate::{
    QualificationError, UnionSimpleTypeQualificationError, qualify_bound_schema_bundle,
    qualify_bound_union_simple_types,
};
use roxmltree::{Document, Node};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub const BOUND_COMPLEX_TYPE_COUNT: usize = 206;
pub const BOUND_NAMED_COMPLEX_TYPE_COUNT: usize = 199;
pub const BOUND_ANONYMOUS_COMPLEX_TYPE_COUNT: usize = 7;
pub const BOUND_DIRECT_COMPLEX_TYPE_COUNT: usize = 125;
pub const BOUND_SIMPLE_CONTENT_EXTENSION_COUNT: usize = 77;
pub const BOUND_COMPLEX_CONTENT_EXTENSION_COUNT: usize = 4;
pub const BOUND_SEQUENCE_COUNT: usize = 111;
pub const BOUND_CHOICE_COUNT: usize = 32;
pub const BOUND_GLOBAL_MODEL_GROUP_COUNT: usize = 24;
pub const BOUND_MODEL_GROUP_REF_COUNT: usize = 60;
pub const BOUND_GLOBAL_ATTRIBUTE_GROUP_COUNT: usize = 45;
pub const BOUND_ATTRIBUTE_GROUP_REF_COUNT: usize = 325;
pub const BOUND_ELEMENT_NODE_COUNT: usize = 447;
pub const BOUND_GLOBAL_ELEMENT_COUNT: usize = 2;
pub const BOUND_ATTRIBUTE_NODE_COUNT: usize = 327;
pub const BOUND_GLOBAL_ATTRIBUTE_COUNT: usize = 10;
pub const BOUND_REQUIRED_ATTRIBUTE_USE_COUNT: usize = 52;
pub const BOUND_DEFAULT_ATTRIBUTE_COUNT: usize = 16;
pub const BOUND_FIXED_ATTRIBUTE_COUNT: usize = 1;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Counts {
    complex_types: usize,
    named_complex_types: usize,
    anonymous_complex_types: usize,
    direct_complex_types: usize,
    simple_content_extensions: usize,
    complex_content_extensions: usize,
    sequences: usize,
    choices: usize,
    global_model_groups: usize,
    model_group_refs: usize,
    global_attribute_groups: usize,
    attribute_group_refs: usize,
    elements: usize,
    global_elements: usize,
    attributes: usize,
    global_attributes: usize,
    required_attribute_uses: usize,
    default_attributes: usize,
    fixed_attributes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComplexTypeProfileQualification {
    profile: BoundSchemaProfile,
    counts: Counts,
}

impl ComplexTypeProfileQualification {
    #[must_use]
    pub const fn profile(&self) -> BoundSchemaProfile {
        self.profile
    }

    #[must_use]
    pub const fn complex_type_count(&self) -> usize {
        self.counts.complex_types
    }

    #[must_use]
    pub const fn direct_complex_type_count(&self) -> usize {
        self.counts.direct_complex_types
    }

    #[must_use]
    pub const fn simple_content_extension_count(&self) -> usize {
        self.counts.simple_content_extensions
    }

    #[must_use]
    pub const fn complex_content_extension_count(&self) -> usize {
        self.counts.complex_content_extensions
    }

    #[must_use]
    pub const fn global_model_group_count(&self) -> usize {
        self.counts.global_model_groups
    }

    #[must_use]
    pub const fn model_group_ref_count(&self) -> usize {
        self.counts.model_group_refs
    }

    #[must_use]
    pub const fn global_attribute_group_count(&self) -> usize {
        self.counts.global_attribute_groups
    }

    #[must_use]
    pub const fn attribute_group_ref_count(&self) -> usize {
        self.counts.attribute_group_refs
    }

    #[must_use]
    pub const fn element_node_count(&self) -> usize {
        self.counts.elements
    }

    #[must_use]
    pub const fn attribute_node_count(&self) -> usize {
        self.counts.attributes
    }

    #[must_use]
    pub const fn complex_type_profile_qualified(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum ComplexTypeProfileQualificationError {
    SchemaQualification(QualificationError),
    SimpleTypePrerequisite(UnionSimpleTypeQualificationError),
    Io { path: PathBuf, detail: String },
    InvalidSchemaDocument { file_name: String, detail: String },
    InvalidComplexProfile { file_name: String, detail: String },
    InventoryMismatch { expected: String, actual: String },
    DependencyCycle { kind: &'static str, path: Vec<String> },
}

impl fmt::Display for ComplexTypeProfileQualificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaQualification(error) => {
                write!(formatter, "schema bundle must qualify before complex-type profile: {error}")
            }
            Self::SimpleTypePrerequisite(error) => write!(
                formatter,
                "complete simple-type profile must qualify before complex-type profile: {error}"
            ),
            Self::Io { path, detail } => {
                write!(formatter, "I/O error at {}: {detail}", path.display())
            }
            Self::InvalidSchemaDocument { file_name, detail } => {
                write!(formatter, "{file_name}: invalid schema during complex-type qualification: {detail}")
            }
            Self::InvalidComplexProfile { file_name, detail } => {
                write!(formatter, "{file_name}: unsupported/invalid bound complex-type profile: {detail}")
            }
            Self::InventoryMismatch { expected, actual } => write!(
                formatter,
                "bound complex-type inventory mismatch; expected {expected}, got {actual}"
            ),
            Self::DependencyCycle { kind, path } => {
                write!(formatter, "bound {kind} dependency cycle: {}", path.join(" -> "))
            }
        }
    }
}

impl std::error::Error for ComplexTypeProfileQualificationError {}

impl From<QualificationError> for ComplexTypeProfileQualificationError {
    fn from(error: QualificationError) -> Self {
        Self::SchemaQualification(error)
    }
}

impl From<UnionSimpleTypeQualificationError> for ComplexTypeProfileQualificationError {
    fn from(error: UnionSimpleTypeQualificationError) -> Self {
        Self::SimpleTypePrerequisite(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct SymbolName {
    namespace_uri: Option<String>,
    local_name: String,
}

#[derive(Default)]
struct Globals {
    simple_types: BTreeSet<SymbolName>,
    complex_types: BTreeSet<SymbolName>,
    model_groups: BTreeSet<SymbolName>,
    attribute_groups: BTreeSet<SymbolName>,
    attributes: BTreeSet<SymbolName>,
}

pub fn qualify_bound_complex_type_profile(
    schema_dir: impl AsRef<Path>,
) -> Result<ComplexTypeProfileQualification, ComplexTypeProfileQualificationError> {
    let schema_dir = schema_dir.as_ref();

    let bundle = qualify_bound_schema_bundle(schema_dir)?;
    let profile = bundle.profile();
    qualify_bound_union_simple_types(schema_dir)?;

    let globals = collect_globals(schema_dir, profile)?;
    let mut state = QualificationState {
        globals: &globals,
        counts: Counts::default(),
        model_group_graph: BTreeMap::new(),
        attribute_group_graph: BTreeMap::new(),
        complex_extension_graph: BTreeMap::new(),
    };

    for expected in expected_schema_files(profile) {
        let path = schema_dir.join(expected.file_name);
        let text = fs::read_to_string(&path).map_err(|error| ComplexTypeProfileQualificationError::Io {
            path: path.clone(),
            detail: error.to_string(),
        })?;
        let document = Document::parse(&text).map_err(|error| {
            ComplexTypeProfileQualificationError::InvalidSchemaDocument {
                file_name: expected.file_name.to_owned(),
                detail: error.to_string(),
            }
        })?;
        qualify_document(
            expected.file_name,
            expected.target_namespace,
            &document,
            &mut state,
        )?;
    }

    assert_acyclic("model-group", &state.model_group_graph)?;
    assert_acyclic("attribute-group", &state.attribute_group_graph)?;
    assert_acyclic("complex-content extension", &state.complex_extension_graph)?;

    let expected = expected_counts_for(profile);
    if state.counts != expected {
        return Err(ComplexTypeProfileQualificationError::InventoryMismatch {
            expected: format!("{expected:?}"),
            actual: format!("{:?}", state.counts),
        });
    }

    Ok(ComplexTypeProfileQualification { profile, counts: state.counts })
}

fn collect_globals(
    schema_dir: &Path,
    profile: BoundSchemaProfile,
) -> Result<Globals, ComplexTypeProfileQualificationError> {
    let mut globals = Globals::default();
    for expected in expected_schema_files(profile) {
        let path = schema_dir.join(expected.file_name);
        let text = fs::read_to_string(&path).map_err(|error| ComplexTypeProfileQualificationError::Io {
            path: path.clone(),
            detail: error.to_string(),
        })?;
        let document = Document::parse(&text).map_err(|error| {
            ComplexTypeProfileQualificationError::InvalidSchemaDocument {
                file_name: expected.file_name.to_owned(),
                detail: error.to_string(),
            }
        })?;
        let schema = document.root_element();
        for child in schema.children().filter(|node| node.is_element()) {
            if child.tag_name().namespace() != Some(XSD_NAMESPACE) {
                continue;
            }
            let Some(name) = child.attribute("name") else {
                continue;
            };
            let symbol = SymbolName {
                namespace_uri: expected.target_namespace.map(str::to_owned),
                local_name: name.to_owned(),
            };
            match child.tag_name().name() {
                "simpleType" => {
                    globals.simple_types.insert(symbol);
                }
                "complexType" => {
                    globals.complex_types.insert(symbol);
                }
                "group" => {
                    globals.model_groups.insert(symbol);
                }
                "attributeGroup" => {
                    globals.attribute_groups.insert(symbol);
                }
                "attribute" => {
                    globals.attributes.insert(symbol);
                }
                _ => {}
            }
        }
    }
    Ok(globals)
}

struct QualificationState<'a> {
    globals: &'a Globals,
    counts: Counts,
    model_group_graph: BTreeMap<SymbolName, Vec<SymbolName>>,
    attribute_group_graph: BTreeMap<SymbolName, Vec<SymbolName>>,
    complex_extension_graph: BTreeMap<SymbolName, Vec<SymbolName>>,
}

fn qualify_document(
    file_name: &str,
    target_namespace: Option<&str>,
    document: &Document<'_>,
    state: &mut QualificationState<'_>,
) -> Result<(), ComplexTypeProfileQualificationError> {
    let schema = document.root_element();
    qualify_schema_form_defaults(file_name, schema)?;

    for node in document.descendants().filter(|node| node.is_element()) {
        if node.tag_name().namespace() != Some(XSD_NAMESPACE) {
            continue;
        }
        match node.tag_name().name() {
            "complexType" => qualify_complex_type(
                file_name,
                target_namespace,
                node,
                state.globals,
                &mut state.counts,
                &mut state.complex_extension_graph,
            )?,
            "sequence" => {
                state.counts.sequences += 1;
                qualify_particle_occurs(file_name, node)?;
            }
            "choice" => {
                state.counts.choices += 1;
                qualify_particle_occurs(file_name, node)?;
            }
            "group" => qualify_model_group(
                file_name,
                target_namespace,
                schema,
                node,
                state.globals,
                &mut state.counts,
                &mut state.model_group_graph,
            )?,
            "attributeGroup" => qualify_attribute_group(
                file_name,
                target_namespace,
                schema,
                node,
                state.globals,
                &mut state.counts,
                &mut state.attribute_group_graph,
            )?,
            "element" => qualify_element(file_name, schema, node, state.globals, &mut state.counts)?,
            "attribute" => qualify_attribute(file_name, schema, node, state.globals, &mut state.counts)?,
            _ => {}
        }
    }

    Ok(())
}

fn qualify_schema_form_defaults(
    file_name: &str,
    schema: Node<'_, '_>,
) -> Result<(), ComplexTypeProfileQualificationError> {
    let actual = (
        schema.attribute("elementFormDefault"),
        schema.attribute("attributeFormDefault"),
    );
    let expected = match file_name {
        "musicxml.xsd" => (Some("qualified"), Some("unqualified")),
        "xml.xsd" => (None, None),
        "xlink.xsd" => (Some("qualified"), Some("qualified")),
        _ => return invalid(file_name, "unexpected schema file in bound profile"),
    };
    if actual != expected {
        return invalid(
            file_name,
            &format!("schema form defaults mismatch; expected {expected:?}, got {actual:?}"),
        );
    }
    Ok(())
}

fn qualify_complex_type(
    file_name: &str,
    target_namespace: Option<&str>,
    node: Node<'_, '_>,
    globals: &Globals,
    counts: &mut Counts,
    complex_extension_graph: &mut BTreeMap<SymbolName, Vec<SymbolName>>,
) -> Result<(), ComplexTypeProfileQualificationError> {
    counts.complex_types += 1;
    if node.attribute("name").is_some() {
        counts.named_complex_types += 1;
    } else {
        counts.anonymous_complex_types += 1;
    }

    let children: Vec<_> = node
        .children()
        .filter(|child| child.is_element() && child.tag_name().namespace() == Some(XSD_NAMESPACE))
        .filter(|child| child.tag_name().name() != "annotation")
        .collect();
    let simple_content = children.iter().find(|child| child.tag_name().name() == "simpleContent");
    let complex_content = children.iter().find(|child| child.tag_name().name() == "complexContent");

    if let Some(simple_content) = simple_content {
        if complex_content.is_some() || children.len() != 1 {
            return invalid(file_name, "simpleContent complexType has additional structural children");
        }
        let extension = sole_extension(file_name, *simple_content, "simpleContent")?;
        counts.simple_content_extensions += 1;
        let base = resolve_qname(file_name, extension, &required_attr(file_name, extension, "base")?)?;
        if !is_simple_type_reference(&base, globals) {
            return invalid(file_name, &format!("simpleContent extension base {base:?} is not a qualified simple type"));
        }
        qualify_extension_children(file_name, extension, false)?;
        return Ok(());
    }

    if let Some(complex_content) = complex_content {
        if children.len() != 1 {
            return invalid(file_name, "complexContent complexType has additional structural children");
        }
        let extension = sole_extension(file_name, *complex_content, "complexContent")?;
        counts.complex_content_extensions += 1;
        let base = resolve_qname(file_name, extension, &required_attr(file_name, extension, "base")?)?;
        if !globals.complex_types.contains(&base) {
            return invalid(file_name, &format!("complexContent extension base {base:?} is not a qualified complex type"));
        }
        qualify_extension_children(file_name, extension, true)?;
        if let Some(name) = node.attribute("name") {
            let source = SymbolName {
                namespace_uri: target_namespace.map(str::to_owned),
                local_name: name.to_owned(),
            };
            complex_extension_graph.entry(source).or_default().push(base);
        }
        return Ok(());
    }

    counts.direct_complex_types += 1;
    qualify_direct_complex_children(file_name, &children)
}

fn sole_extension<'a, 'input: 'a>(
    file_name: &str,
    content: Node<'a, 'input>,
    label: &str,
) -> Result<Node<'a, 'input>, ComplexTypeProfileQualificationError> {
    let structural: Vec<_> = content
        .children()
        .filter(|child| child.is_element() && child.tag_name().namespace() == Some(XSD_NAMESPACE))
        .filter(|child| child.tag_name().name() != "annotation")
        .collect();
    if structural.len() != 1 || structural[0].tag_name().name() != "extension" {
        return Err(ComplexTypeProfileQualificationError::InvalidComplexProfile {
            file_name: file_name.to_owned(),
            detail: format!("{label} is not exactly one xs:extension"),
        });
    }
    Ok(structural[0])
}

fn qualify_extension_children(
    file_name: &str,
    extension: Node<'_, '_>,
    allow_particle: bool,
) -> Result<(), ComplexTypeProfileQualificationError> {
    let mut particle_seen = false;
    let mut attributes_started = false;
    for child in extension.children().filter(|node| node.is_element()) {
        if child.tag_name().namespace() != Some(XSD_NAMESPACE) || child.tag_name().name() == "annotation" {
            continue;
        }
        match child.tag_name().name() {
            "sequence" | "choice" | "group" if allow_particle && !particle_seen && !attributes_started => {
                particle_seen = true;
            }
            "attribute" | "attributeGroup" => {
                attributes_started = true;
            }
            other => {
                return invalid(file_name, &format!("unsupported extension child xs:{other}"));
            }
        }
    }
    Ok(())
}

fn qualify_direct_complex_children(
    file_name: &str,
    children: &[Node<'_, '_>],
) -> Result<(), ComplexTypeProfileQualificationError> {
    let mut particle_seen = false;
    let mut attributes_started = false;
    for child in children {
        match child.tag_name().name() {
            "sequence" | "choice" | "group" if !particle_seen && !attributes_started => {
                particle_seen = true;
            }
            "attribute" | "attributeGroup" => {
                attributes_started = true;
            }
            other => {
                return invalid(file_name, &format!("unsupported direct complexType child xs:{other}"));
            }
        }
    }
    Ok(())
}

fn qualify_model_group(
    file_name: &str,
    target_namespace: Option<&str>,
    schema: Node<'_, '_>,
    node: Node<'_, '_>,
    globals: &Globals,
    counts: &mut Counts,
    graph: &mut BTreeMap<SymbolName, Vec<SymbolName>>,
) -> Result<(), ComplexTypeProfileQualificationError> {
    if node.parent() == Some(schema) {
        counts.global_model_groups += 1;
        let name = required_attr(file_name, node, "name")?;
        let key = SymbolName {
            namespace_uri: target_namespace.map(str::to_owned),
            local_name: name.clone(),
        };
        let structural: Vec<_> = node
            .children()
            .filter(|child| child.is_element() && child.tag_name().namespace() == Some(XSD_NAMESPACE))
            .filter(|child| child.tag_name().name() != "annotation")
            .collect();
        if structural.len() != 1 || structural[0].tag_name().name() != "sequence" {
            return invalid(file_name, &format!("global model group {name:?} is not exactly one sequence"));
        }
        let mut deps = Vec::new();
        for descendant in structural[0].descendants().filter(|node| node.is_element()) {
            if descendant.tag_name().namespace() == Some(XSD_NAMESPACE)
                && descendant.tag_name().name() == "group"
                && descendant.attribute("ref").is_some()
            {
                deps.push(resolve_qname(file_name, descendant, &required_attr(file_name, descendant, "ref")?)?);
            }
        }
        graph.insert(key, deps);
    } else {
        counts.model_group_refs += 1;
        qualify_particle_occurs(file_name, node)?;
        let target = resolve_qname(file_name, node, &required_attr(file_name, node, "ref")?)?;
        if !globals.model_groups.contains(&target) {
            return invalid(file_name, &format!("unresolved model-group ref {target:?}"));
        }
    }
    Ok(())
}

fn qualify_attribute_group(
    file_name: &str,
    target_namespace: Option<&str>,
    schema: Node<'_, '_>,
    node: Node<'_, '_>,
    globals: &Globals,
    counts: &mut Counts,
    graph: &mut BTreeMap<SymbolName, Vec<SymbolName>>,
) -> Result<(), ComplexTypeProfileQualificationError> {
    if node.parent() == Some(schema) {
        counts.global_attribute_groups += 1;
        let name = required_attr(file_name, node, "name")?;
        let key = SymbolName {
            namespace_uri: target_namespace.map(str::to_owned),
            local_name: name.clone(),
        };
        let mut deps = Vec::new();
        for child in node.children().filter(|node| node.is_element()) {
            if child.tag_name().namespace() != Some(XSD_NAMESPACE) || child.tag_name().name() == "annotation" {
                continue;
            }
            match child.tag_name().name() {
                "attribute" => {}
                "attributeGroup" => {
                    deps.push(resolve_qname(file_name, child, &required_attr(file_name, child, "ref")?)?);
                }
                other => return invalid(file_name, &format!("unsupported attributeGroup child xs:{other}")),
            }
        }
        graph.insert(key, deps);
    } else {
        counts.attribute_group_refs += 1;
        let target = resolve_qname(file_name, node, &required_attr(file_name, node, "ref")?)?;
        if !globals.attribute_groups.contains(&target) {
            return invalid(file_name, &format!("unresolved attribute-group ref {target:?}"));
        }
    }
    Ok(())
}

fn qualify_element(
    file_name: &str,
    schema: Node<'_, '_>,
    node: Node<'_, '_>,
    globals: &Globals,
    counts: &mut Counts,
) -> Result<(), ComplexTypeProfileQualificationError> {
    counts.elements += 1;
    if node.parent() == Some(schema) {
        counts.global_elements += 1;
        let name = required_attr(file_name, node, "name")?;
        if file_name != "musicxml.xsd"
            || !matches!(name.as_str(), "score-partwise" | "score-timewise")
            || node.attribute("block") != Some("extension substitution")
            || node.attribute("final") != Some("#all")
        {
            return invalid(file_name, &format!("unexpected global element declaration {name:?}"));
        }
    }
    required_attr(file_name, node, "name")?;
    if node.attribute("ref").is_some() {
        return invalid(file_name, "bound element declarations never use @ref");
    }
    qualify_particle_occurs(file_name, node)?;

    match (node.attribute("type"), child_named(node, "complexType")) {
        (Some(lexical), None) => {
            let target = resolve_qname(file_name, node, lexical)?;
            if !is_element_type_reference(&target, globals) {
                return invalid(file_name, &format!("element type {target:?} is unresolved/unsupported"));
            }
        }
        (None, Some(_)) => {}
        _ => return invalid(file_name, "element must have exactly one of @type or anonymous complexType"),
    }
    Ok(())
}

fn qualify_attribute(
    file_name: &str,
    schema: Node<'_, '_>,
    node: Node<'_, '_>,
    globals: &Globals,
    counts: &mut Counts,
) -> Result<(), ComplexTypeProfileQualificationError> {
    counts.attributes += 1;
    if node.parent() == Some(schema) {
        counts.global_attributes += 1;
    }
    if node.attribute("use") == Some("required") {
        counts.required_attribute_uses += 1;
    } else if let Some(value) = node.attribute("use")
        && value != "optional"
    {
        return invalid(file_name, &format!("unsupported attribute use {value:?}"));
    }
    if node.attribute("default").is_some() {
        counts.default_attributes += 1;
    }
    if node.attribute("fixed").is_some() {
        counts.fixed_attributes += 1;
    }
    if node.attribute("default").is_some() && node.attribute("fixed").is_some() {
        return invalid(file_name, "attribute cannot have both default and fixed value constraints");
    }
    if node.attribute("default").is_some() && node.attribute("use") == Some("required") {
        return invalid(file_name, "required attribute cannot carry a default value constraint");
    }

    if let Some(reference) = node.attribute("ref") {
        if node.attribute("name").is_some() || node.attribute("type").is_some() || child_named(node, "simpleType").is_some() {
            return invalid(file_name, "attribute @ref cannot be combined with name/type/anonymous simpleType in bound profile");
        }
        let target = resolve_qname(file_name, node, reference)?;
        if !globals.attributes.contains(&target) {
            return invalid(file_name, &format!("unresolved attribute ref {target:?}"));
        }
        return Ok(());
    }

    required_attr(file_name, node, "name")?;
    match (node.attribute("type"), child_named(node, "simpleType")) {
        (Some(lexical), None) => {
            let target = resolve_qname(file_name, node, lexical)?;
            if !is_simple_type_reference(&target, globals) {
                return invalid(file_name, &format!("attribute type {target:?} is not a qualified simple type"));
            }
        }
        (None, Some(_)) => {}
        _ => return invalid(file_name, "attribute must have exactly one of @type or anonymous simpleType"),
    }
    Ok(())
}

fn qualify_particle_occurs(
    file_name: &str,
    node: Node<'_, '_>,
) -> Result<(), ComplexTypeProfileQualificationError> {
    let min = parse_occurs(file_name, node.attribute("minOccurs").unwrap_or("1"), false)?;
    let max = parse_occurs(file_name, node.attribute("maxOccurs").unwrap_or("1"), true)?;
    if let Some(max) = max {
        let min = min.expect("minOccurs never parses as unbounded");
        if min > max {
            return invalid(file_name, &format!("minOccurs {min} exceeds maxOccurs {max}"));
        }
    }
    Ok(())
}

fn parse_occurs(
    file_name: &str,
    lexical: &str,
    allow_unbounded: bool,
) -> Result<Option<usize>, ComplexTypeProfileQualificationError> {
    if allow_unbounded && lexical == "unbounded" {
        return Ok(None);
    }
    if lexical.is_empty() || !lexical.bytes().all(|byte| byte.is_ascii_digit()) {
        return invalid(file_name, &format!("invalid occurrence lexical {lexical:?}"));
    }
    let value = lexical.parse::<usize>().map_err(|_| ComplexTypeProfileQualificationError::InvalidComplexProfile {
        file_name: file_name.to_owned(),
        detail: format!("occurrence value {lexical:?} exceeds implementation range"),
    })?;
    Ok(Some(value))
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
) -> Result<String, ComplexTypeProfileQualificationError> {
    node.attribute(name)
        .map(str::to_owned)
        .ok_or_else(|| ComplexTypeProfileQualificationError::InvalidComplexProfile {
            file_name: file_name.to_owned(),
            detail: format!("xs:{} is missing required @{name}", node.tag_name().name()),
        })
}

fn resolve_qname(
    file_name: &str,
    node: Node<'_, '_>,
    lexical: &str,
) -> Result<SymbolName, ComplexTypeProfileQualificationError> {
    let (prefix, local_name) = match lexical.split_once(':') {
        Some((prefix, local_name)) => {
            if prefix.is_empty() || local_name.is_empty() || local_name.contains(':') {
                return invalid(file_name, &format!("invalid QName {lexical:?}"));
            }
            (Some(prefix), local_name)
        }
        None => {
            if lexical.is_empty() {
                return invalid(file_name, "empty QName");
            }
            (None, lexical)
        }
    };
    let namespace_uri = match prefix {
        Some("xml") => Some(XML_NAMESPACE.to_owned()),
        Some(prefix) => {
            let Some(namespace) = node.lookup_namespace_uri(Some(prefix)) else {
                return invalid(file_name, &format!("unbound QName prefix in {lexical:?}"));
            };
            Some(namespace.to_owned())
        }
        None => node.lookup_namespace_uri(None).map(str::to_owned),
    };
    Ok(SymbolName {
        namespace_uri,
        local_name: local_name.to_owned(),
    })
}

fn is_simple_type_reference(symbol: &SymbolName, globals: &Globals) -> bool {
    if symbol.namespace_uri.as_deref() == Some(XSD_NAMESPACE) {
        return matches!(
            symbol.local_name.as_str(),
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
                | "anyURI"
                | "language"
                | "date"
        );
    }
    globals.simple_types.contains(symbol)
}

fn is_element_type_reference(symbol: &SymbolName, globals: &Globals) -> bool {
    is_simple_type_reference(symbol, globals) || globals.complex_types.contains(symbol)
}

fn assert_acyclic(
    kind: &'static str,
    graph: &BTreeMap<SymbolName, Vec<SymbolName>>,
) -> Result<(), ComplexTypeProfileQualificationError> {
    let mut states: BTreeMap<SymbolName, u8> = BTreeMap::new();
    let mut path = Vec::new();
    for node in graph.keys() {
        visit(kind, node, graph, &mut states, &mut path)?;
    }
    Ok(())
}

fn visit(
    kind: &'static str,
    node: &SymbolName,
    graph: &BTreeMap<SymbolName, Vec<SymbolName>>,
    states: &mut BTreeMap<SymbolName, u8>,
    path: &mut Vec<SymbolName>,
) -> Result<(), ComplexTypeProfileQualificationError> {
    match states.get(node).copied().unwrap_or(0) {
        2 => return Ok(()),
        1 => {
            let start = path.iter().position(|candidate| candidate == node).unwrap_or(0);
            let mut cycle: Vec<String> = path[start..]
                .iter()
                .map(|item| item.local_name.clone())
                .collect();
            cycle.push(node.local_name.clone());
            return Err(ComplexTypeProfileQualificationError::DependencyCycle { kind, path: cycle });
        }
        _ => {}
    }
    states.insert(node.clone(), 1);
    path.push(node.clone());
    if let Some(targets) = graph.get(node) {
        for target in targets {
            if graph.contains_key(target) {
                visit(kind, target, graph, states, path)?;
            }
        }
    }
    path.pop();
    states.insert(node.clone(), 2);
    Ok(())
}

fn expected_counts_for(profile: BoundSchemaProfile) -> Counts {
    match profile {
        BoundSchemaProfile::MusicXml31 => Counts {
            complex_types: BOUND_COMPLEX_TYPE_COUNT,
            named_complex_types: BOUND_NAMED_COMPLEX_TYPE_COUNT,
            anonymous_complex_types: BOUND_ANONYMOUS_COMPLEX_TYPE_COUNT,
            direct_complex_types: BOUND_DIRECT_COMPLEX_TYPE_COUNT,
            simple_content_extensions: BOUND_SIMPLE_CONTENT_EXTENSION_COUNT,
            complex_content_extensions: BOUND_COMPLEX_CONTENT_EXTENSION_COUNT,
            sequences: BOUND_SEQUENCE_COUNT,
            choices: BOUND_CHOICE_COUNT,
            global_model_groups: BOUND_GLOBAL_MODEL_GROUP_COUNT,
            model_group_refs: BOUND_MODEL_GROUP_REF_COUNT,
            global_attribute_groups: BOUND_GLOBAL_ATTRIBUTE_GROUP_COUNT,
            attribute_group_refs: BOUND_ATTRIBUTE_GROUP_REF_COUNT,
            elements: BOUND_ELEMENT_NODE_COUNT,
            global_elements: BOUND_GLOBAL_ELEMENT_COUNT,
            attributes: BOUND_ATTRIBUTE_NODE_COUNT,
            global_attributes: BOUND_GLOBAL_ATTRIBUTE_COUNT,
            required_attribute_uses: BOUND_REQUIRED_ATTRIBUTE_USE_COUNT,
            default_attributes: BOUND_DEFAULT_ATTRIBUTE_COUNT,
            fixed_attributes: BOUND_FIXED_ATTRIBUTE_COUNT,
        },
        BoundSchemaProfile::MusicXml40 => Counts {
            complex_types: 231,
            named_complex_types: 224,
            anonymous_complex_types: 7,
            direct_complex_types: 143,
            simple_content_extensions: 83,
            complex_content_extensions: 5,
            sequences: 121,
            choices: 36,
            global_model_groups: 27,
            model_group_refs: 66,
            global_attribute_groups: 46,
            attribute_group_refs: 335,
            elements: 480,
            global_elements: 2,
            attributes: 366,
            global_attributes: 10,
            required_attribute_uses: 60,
            default_attributes: 16,
            fixed_attributes: 1,
        },
    }
}

fn invalid<T>(
    file_name: &str,
    detail: &str,
) -> Result<T, ComplexTypeProfileQualificationError> {
    Err(ComplexTypeProfileQualificationError::InvalidComplexProfile {
        file_name: file_name.to_owned(),
        detail: detail.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MUSICXML31_SCHEMA_DIR, MUSICXML40_SCHEMA_DIR};

    fn qualification() -> ComplexTypeProfileQualification {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        qualify_bound_complex_type_profile(&root).expect("bound complex-type profile must qualify")
    }

    #[test]
    fn exact_complex_type_surface_qualifies() {
        let qualification = qualification();
        assert_eq!(qualification.complex_type_count(), 206);
        assert_eq!(qualification.direct_complex_type_count(), 125);
        assert_eq!(qualification.simple_content_extension_count(), 77);
        assert_eq!(qualification.complex_content_extension_count(), 4);
        assert!(qualification.complex_type_profile_qualified());
    }

    #[test]
    fn exact_particle_and_group_surface_qualifies() {
        let qualification = qualification();
        assert_eq!(qualification.global_model_group_count(), 24);
        assert_eq!(qualification.model_group_ref_count(), 60);
        assert_eq!(qualification.global_attribute_group_count(), 45);
        assert_eq!(qualification.attribute_group_ref_count(), 325);
        assert_eq!(qualification.element_node_count(), 447);
        assert_eq!(qualification.attribute_node_count(), 327);
    }

    #[test]
    fn occurrence_parser_supports_bound_finite_and_unbounded_forms() {
        assert_eq!(parse_occurs("synthetic", "0", false).unwrap(), Some(0));
        assert_eq!(parse_occurs("synthetic", "8", true).unwrap(), Some(8));
        assert_eq!(parse_occurs("synthetic", "unbounded", true).unwrap(), None);
    }

    #[test]
    fn occurrence_parser_rejects_non_xsd_integer_lexicals_fail_closed() {
        assert!(parse_occurs("synthetic", "-1", false).is_err());
        assert!(parse_occurs("synthetic", "+1", false).is_err());
        assert!(parse_occurs("synthetic", "unbounded", false).is_err());
    }

    #[test]
    fn complex_profile_remains_a_required_final_instance_prerequisite() {
        let qualification = qualification();
        assert!(qualification.complex_type_profile_qualified());
        assert_eq!(
            qualification.complex_type_profile_qualified(),
            crate::INSTANCE_VALIDATION_QUALIFIED
        );
    }
    #[test]
    fn musicxml40_complex_profile_qualifies_without_new_xsd_construct_classes() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let qualification = qualify_bound_complex_type_profile(&root)
            .expect("bound 4.0 complex profile must qualify");
        assert_eq!(qualification.profile(), BoundSchemaProfile::MusicXml40);
        assert_eq!(qualification.complex_type_count(), 231);
        assert_eq!(qualification.global_model_group_count(), 27);
        assert_eq!(qualification.global_attribute_group_count(), 46);
    }

}
