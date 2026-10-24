use std::collections::BTreeMap;

pub const XSD_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema";
pub const XML_NAMESPACE: &str = "http://www.w3.org/XML/1998/namespace";
pub const XLINK_NAMESPACE: &str = "http://www.w3.org/1999/xlink";

pub const MUSICXML31_BUNDLE_ID: &str =
    "musicxml-3.1-v3.1-8bbe8e50606bf99317a86e9e1637618d6bdd1997";
pub const MUSICXML31_UPSTREAM_TAG: &str = "v3.1";
pub const MUSICXML31_UPSTREAM_COMMIT: &str =
    "8bbe8e50606bf99317a86e9e1637618d6bdd1997";
pub const MUSICXML31_SCHEMA_DIR: &str = "docs/normative_sources/musicxml/3.1/schema";

/// Exact W3C MusicXML 4.0 release bundle represented by the three XSD files used for score
/// validation. The release itself is the W3C June 2021 MusicXML 4.0 publication; authority is
/// ultimately bound to the exact bytes and hashes below rather than a mutable network resource.
pub const MUSICXML40_BUNDLE_ID: &str = "musicxml-4.0-w3c-2021-06";
pub const MUSICXML40_SCHEMA_DIR: &str = "docs/normative_sources/musicxml/4.0/schema";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BoundSchemaProfile {
    MusicXml31,
    MusicXml40,
}

impl BoundSchemaProfile {
    #[must_use]
    pub const fn bundle_id(self) -> &'static str {
        match self {
            Self::MusicXml31 => MUSICXML31_BUNDLE_ID,
            Self::MusicXml40 => MUSICXML40_BUNDLE_ID,
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::MusicXml31 => "MusicXML 3.1",
            Self::MusicXml40 => "MusicXML 4.0",
        }
    }

    #[must_use]
    pub const fn schema_dir(self) -> &'static str {
        match self {
            Self::MusicXml31 => MUSICXML31_SCHEMA_DIR,
            Self::MusicXml40 => MUSICXML40_SCHEMA_DIR,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ExpectedSchemaFile {
    pub file_name: &'static str,
    pub byte_len: usize,
    pub sha256: &'static str,
    pub target_namespace: Option<&'static str>,
}

pub(crate) const EXPECTED_SCHEMA_FILES: [ExpectedSchemaFile; 3] = [
    ExpectedSchemaFile {
        file_name: "musicxml.xsd",
        byte_len: 335_762,
        sha256: "49ae9e34e04ba5b72004687994f70d598323eb3353eadb62faedcc9e9dc4bd7f",
        target_namespace: None,
    },
    ExpectedSchemaFile {
        file_name: "xml.xsd",
        byte_len: 5_695,
        sha256: "616a3077df5cfc954ac74a75abe9697b95eef7a85dbe09367d995a483e840eb5",
        target_namespace: Some(XML_NAMESPACE),
    },
    ExpectedSchemaFile {
        file_name: "xlink.xsd",
        byte_len: 2_280,
        sha256: "67c7f7a02491babc51e11c10268150b1a76070da36d5abe88e68515009611966",
        target_namespace: Some(XLINK_NAMESPACE),
    },
];

pub(crate) const MUSICXML40_EXPECTED_SCHEMA_FILES: [ExpectedSchemaFile; 3] = [
    ExpectedSchemaFile {
        file_name: "musicxml.xsd",
        byte_len: 379_505,
        sha256: "bfe37ed25a9ec00e6f2591d53df260b84efe12aed209ba3ac0a76f9287665a99",
        target_namespace: None,
    },
    ExpectedSchemaFile {
        file_name: "xml.xsd",
        byte_len: 5_695,
        sha256: "616a3077df5cfc954ac74a75abe9697b95eef7a85dbe09367d995a483e840eb5",
        target_namespace: Some(XML_NAMESPACE),
    },
    ExpectedSchemaFile {
        file_name: "xlink.xsd",
        byte_len: 2_280,
        sha256: "6e601f8eeb41618b50e4c7f944dff754e57ea43b602755470dda24c9c2f6df92",
        target_namespace: Some(XLINK_NAMESPACE),
    },
];

pub(crate) fn expected_schema_files(
    profile: BoundSchemaProfile,
) -> &'static [ExpectedSchemaFile; 3] {
    match profile {
        BoundSchemaProfile::MusicXml31 => &EXPECTED_SCHEMA_FILES,
        BoundSchemaProfile::MusicXml40 => &MUSICXML40_EXPECTED_SCHEMA_FILES,
    }
}

pub(crate) const EXPECTED_IMPORTS: [(&str, &str, &str); 2] = [
    (
        XML_NAMESPACE,
        "http://www.musicxml.org/xsd/xml.xsd",
        "xml.xsd",
    ),
    (
        XLINK_NAMESPACE,
        "http://www.musicxml.org/xsd/xlink.xsd",
        "xlink.xsd",
    ),
];

/// Exact XSD vocabulary used by both bound MusicXML 3.1 and 4.0 score schemas.
pub(crate) fn is_supported_xsd_element(local_name: &str) -> bool {
    matches!(
        local_name,
        "schema"
            | "import"
            | "element"
            | "attribute"
            | "attributeGroup"
            | "group"
            | "complexType"
            | "simpleType"
            | "sequence"
            | "choice"
            | "restriction"
            | "extension"
            | "simpleContent"
            | "complexContent"
            | "enumeration"
            | "pattern"
            | "union"
            | "minInclusive"
            | "maxInclusive"
            | "minExclusive"
            | "minLength"
            | "annotation"
            | "documentation"
    )
}

pub(crate) fn is_supported_unqualified_xsd_attribute(local_name: &str) -> bool {
    matches!(
        local_name,
        "name"
            | "type"
            | "value"
            | "ref"
            | "minOccurs"
            | "base"
            | "maxOccurs"
            | "use"
            | "default"
            | "memberTypes"
            | "targetNamespace"
            | "elementFormDefault"
            | "attributeFormDefault"
            | "namespace"
            | "schemaLocation"
            | "block"
            | "final"
            | "fixed"
    )
}

pub(crate) fn expected_element_inventory_for(
    profile: BoundSchemaProfile,
) -> BTreeMap<String, usize> {
    let rows: &[(&str, usize)] = match profile {
        BoundSchemaProfile::MusicXml31 => &[
            ("schema", 3), ("import", 2), ("element", 447), ("attribute", 327),
            ("attributeGroup", 370), ("group", 84), ("complexType", 206),
            ("simpleType", 141), ("sequence", 111), ("choice", 32),
            ("restriction", 136), ("extension", 81), ("simpleContent", 77),
            ("complexContent", 4), ("enumeration", 604), ("pattern", 10),
            ("union", 5), ("minInclusive", 13), ("maxInclusive", 11),
            ("minExclusive", 2), ("minLength", 1), ("annotation", 570),
            ("documentation", 570),
        ],
        BoundSchemaProfile::MusicXml40 => &[
            ("schema", 3), ("import", 2), ("element", 480), ("attribute", 366),
            ("attributeGroup", 381), ("group", 93), ("complexType", 231),
            ("simpleType", 153), ("sequence", 121), ("choice", 36),
            ("restriction", 148), ("extension", 88), ("simpleContent", 83),
            ("complexContent", 5), ("enumeration", 633), ("pattern", 11),
            ("union", 5), ("minInclusive", 14), ("maxInclusive", 12),
            ("minExclusive", 2), ("minLength", 1), ("annotation", 620),
            ("documentation", 620),
        ],
    };
    rows.iter()
        .map(|(name, count)| ((*name).to_owned(), *count))
        .collect()
}

pub(crate) fn expected_attribute_inventory_for(
    profile: BoundSchemaProfile,
) -> BTreeMap<String, usize> {
    let rows: &[(&str, usize)] = match profile {
        BoundSchemaProfile::MusicXml31 => &[
            ("name", 1160), ("type", 747), ("value", 641), ("ref", 400),
            ("minOccurs", 218), ("base", 217), ("maxOccurs", 86), ("use", 52),
            ("default", 16), ("memberTypes", 5), ("targetNamespace", 2),
            ("elementFormDefault", 2), ("attributeFormDefault", 2), ("namespace", 2),
            ("schemaLocation", 2), ("block", 2), ("final", 2), ("xml:lang", 1),
            ("fixed", 1),
        ],
        BoundSchemaProfile::MusicXml40 => &[
            ("name", 1273), ("type", 819), ("value", 673), ("ref", 416),
            ("minOccurs", 236), ("base", 236), ("maxOccurs", 95), ("use", 60),
            ("default", 16), ("memberTypes", 5), ("targetNamespace", 2),
            ("elementFormDefault", 2), ("attributeFormDefault", 2), ("namespace", 2),
            ("schemaLocation", 2), ("block", 2), ("final", 2), ("xml:lang", 1),
            ("fixed", 1),
        ],
    };
    rows.iter()
        .map(|(name, count)| ((*name).to_owned(), *count))
        .collect()
}

pub(crate) fn expected_global_symbol_inventory_for(
    profile: BoundSchemaProfile,
) -> BTreeMap<String, usize> {
    let rows: &[(&str, usize)] = match profile {
        BoundSchemaProfile::MusicXml31 => &[
            ("type", 332), ("attributeGroup", 45), ("group", 24),
            ("attribute", 10), ("element", 2),
        ],
        BoundSchemaProfile::MusicXml40 => &[
            ("type", 369), ("attributeGroup", 46), ("group", 27),
            ("attribute", 10), ("element", 2),
        ],
    };
    rows.iter()
        .map(|(name, count)| ((*name).to_owned(), *count))
        .collect()
}

pub(crate) const fn expected_qname_reference_count(profile: BoundSchemaProfile) -> usize {
    match profile {
        BoundSchemaProfile::MusicXml31 => 1_371,
        BoundSchemaProfile::MusicXml40 => 1_478,
    }
}

pub(crate) fn is_bound_builtin_type(local_name: &str) -> bool {
    matches!(
        local_name,
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
    )
}
