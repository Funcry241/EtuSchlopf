//! Qualification and value semantics for the remaining restriction facets in the exact bound
//! MusicXML 3.1 and 4.0 simple-type profiles.
//!
//! The regular-expression engine is deliberately profile-bounded: it implements only the XSD
//! 1.0 grammar features exercised by the bound patterns (10 in 3.1, 11 in 4.0). Unsupported regex
//! syntax fails closed during qualification.

use crate::named_profile::{
    NamedRestrictionQualification, NamedRestrictionQualificationError, NamedRestrictionValueError,
    qualify_bound_named_restrictions,
};
use crate::profile::{BoundSchemaProfile, XSD_NAMESPACE, expected_schema_files};
use crate::qualification::{QualificationError, qualify_bound_schema_bundle};
use crate::simple::{
    AtomicValue, SimpleTypeError, WhiteSpaceMode, apply_white_space,
    is_xml10_second_edition_name_char, parse_bound_builtin,
};
use roxmltree::Document;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub const BOUND_PATTERN_RESTRICTION_COUNT: usize = 10;
pub const BOUND_MIN_LENGTH_RESTRICTION_COUNT: usize = 1;
pub const BOUND_REMAINING_RESTRICTION_FACET_COUNT: usize =
    BOUND_PATTERN_RESTRICTION_COUNT + BOUND_MIN_LENGTH_RESTRICTION_COUNT;
const MUSICXML40_PATTERN_RESTRICTION_COUNT: usize = 11;

#[derive(Clone, Debug, Eq, PartialEq)]
enum RestrictionBase {
    Builtin(String),
    Named(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PatternRestriction {
    base: RestrictionBase,
    pattern: String,
    compiled: BoundRegex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MinLengthRestriction {
    base: RestrictionBase,
    min_length: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestrictionFacetQualification {
    patterns: BTreeMap<String, PatternRestriction>,
    min_lengths: BTreeMap<String, MinLengthRestriction>,
    named_restrictions: NamedRestrictionQualification,
}

impl RestrictionFacetQualification {
    #[must_use]
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    #[must_use]
    pub fn min_length_count(&self) -> usize {
        self.min_lengths.len()
    }

    #[must_use]
    pub fn facet_count(&self) -> usize {
        self.patterns.len() + self.min_lengths.len()
    }

    pub fn parse_value(
        &self,
        local_name: &str,
        lexical: &str,
    ) -> Result<AtomicValue, RestrictionFacetValueError> {
        if let Some(restriction) = self.patterns.get(local_name) {
            let (value, normalized) = parse_restriction_base(
                &restriction.base,
                &self.named_restrictions,
                lexical,
            )?;
            if restriction.compiled.matches(&normalized) {
                return Ok(value);
            }
            return Err(RestrictionFacetValueError::PatternConstraintViolation {
                local_name: local_name.to_owned(),
                normalized_lexical: normalized,
                pattern: restriction.pattern.clone(),
            });
        }

        if let Some(restriction) = self.min_lengths.get(local_name) {
            let (value, normalized) = parse_restriction_base(
                &restriction.base,
                &self.named_restrictions,
                lexical,
            )?;
            let actual = normalized.chars().count();
            if actual >= restriction.min_length {
                return Ok(value);
            }
            return Err(RestrictionFacetValueError::MinLengthConstraintViolation {
                local_name: local_name.to_owned(),
                normalized_lexical: normalized,
                minimum: restriction.min_length,
                actual,
            });
        }

        Err(RestrictionFacetValueError::UnknownQualifiedType {
            local_name: local_name.to_owned(),
        })
    }
}

#[derive(Debug)]
pub enum RestrictionFacetValueError {
    UnknownQualifiedType {
        local_name: String,
    },
    Datatype(SimpleTypeError),
    NamedDatatype(NamedRestrictionValueError),
    PatternConstraintViolation {
        local_name: String,
        normalized_lexical: String,
        pattern: String,
    },
    MinLengthConstraintViolation {
        local_name: String,
        normalized_lexical: String,
        minimum: usize,
        actual: usize,
    },
}

impl fmt::Display for RestrictionFacetValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownQualifiedType { local_name } => write!(
                formatter,
                "unknown r016 qualified pattern/minLength simple type {local_name:?}"
            ),
            Self::Datatype(error) => write!(formatter, "{error}"),
            Self::NamedDatatype(error) => write!(formatter, "{error}"),
            Self::PatternConstraintViolation {
                local_name,
                normalized_lexical,
                pattern,
            } => write!(
                formatter,
                "value {normalized_lexical:?} violates XSD pattern {pattern:?} for {local_name:?}"
            ),
            Self::MinLengthConstraintViolation {
                local_name,
                normalized_lexical,
                minimum,
                actual,
            } => write!(
                formatter,
                "value {normalized_lexical:?} has XML-character length {actual}, below minLength={minimum} for {local_name:?}"
            ),
        }
    }
}

impl std::error::Error for RestrictionFacetValueError {}

#[derive(Debug)]
pub enum RestrictionFacetQualificationError {
    SchemaQualification(QualificationError),
    NamedQualification(NamedRestrictionQualificationError),
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
    InventoryMismatch {
        expected_patterns: BTreeMap<String, (String, String)>,
        actual_patterns: BTreeMap<String, (String, String)>,
        expected_min_lengths: BTreeMap<String, (String, String)>,
        actual_min_lengths: BTreeMap<String, (String, String)>,
    },
    UnsupportedPatternSyntax {
        local_name: String,
        pattern: String,
        detail: String,
    },
    SemanticProbeFailed {
        local_name: String,
        detail: String,
    },
}

impl fmt::Display for RestrictionFacetQualificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaQualification(error) => write!(
                formatter,
                "schema bundle must qualify before remaining restriction facets: {error}"
            ),
            Self::NamedQualification(error) => write!(
                formatter,
                "named restriction chains must qualify before remaining restriction facets: {error}"
            ),
            Self::Io { path, detail } => {
                write!(formatter, "I/O error at {}: {detail}", path.display())
            }
            Self::InvalidSchemaDocument { file_name, detail } => write!(
                formatter,
                "{file_name}: invalid schema document during remaining restriction qualification: {detail}"
            ),
            Self::InvalidStructure { file_name, detail } => write!(
                formatter,
                "{file_name}: invalid remaining restriction structure: {detail}"
            ),
            Self::InventoryMismatch {
                expected_patterns,
                actual_patterns,
                expected_min_lengths,
                actual_min_lengths,
            } => write!(
                formatter,
                "remaining restriction inventory mismatch; patterns expected {expected_patterns:?}, got {actual_patterns:?}; minLength expected {expected_min_lengths:?}, got {actual_min_lengths:?}"
            ),
            Self::UnsupportedPatternSyntax {
                local_name,
                pattern,
                detail,
            } => write!(
                formatter,
                "bound pattern for {local_name:?} uses unsupported/invalid XSD regex syntax {pattern:?}: {detail}"
            ),
            Self::SemanticProbeFailed { local_name, detail } => write!(
                formatter,
                "bound remaining restriction {local_name:?} failed its semantic qualification probe: {detail}"
            ),
        }
    }
}

impl std::error::Error for RestrictionFacetQualificationError {}

impl From<QualificationError> for RestrictionFacetQualificationError {
    fn from(error: QualificationError) -> Self {
        Self::SchemaQualification(error)
    }
}

impl From<NamedRestrictionQualificationError> for RestrictionFacetQualificationError {
    fn from(error: NamedRestrictionQualificationError) -> Self {
        Self::NamedQualification(error)
    }
}

pub fn qualify_bound_remaining_restriction_facets(
    schema_dir: impl AsRef<Path>,
) -> Result<RestrictionFacetQualification, RestrictionFacetQualificationError> {
    let schema_dir = schema_dir.as_ref();

    // Facet semantics never extend authority to a different schema byte set.
    let bundle = qualify_bound_schema_bundle(schema_dir)?;
    let profile = bundle.profile();
    let named_restrictions = qualify_bound_named_restrictions(schema_dir)?;

    let mut actual_patterns = BTreeMap::new();
    let mut actual_min_lengths = BTreeMap::new();

    for expected in expected_schema_files(profile) {
        let path = schema_dir.join(expected.file_name);
        let text = fs::read_to_string(&path).map_err(|error| {
            RestrictionFacetQualificationError::Io {
                path: path.clone(),
                detail: error.to_string(),
            }
        })?;
        let document = Document::parse(&text).map_err(|error| {
            RestrictionFacetQualificationError::InvalidSchemaDocument {
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
            if facets.len() != 1 {
                continue;
            }

            let facet = facets[0];
            if !matches!(facet.tag_name().name(), "pattern" | "minLength") {
                continue;
            }

            let name = simple_type
                .attribute("name")
                .expect("filtered named simple type")
                .to_owned();
            let base = restriction.attribute("base").ok_or_else(|| {
                RestrictionFacetQualificationError::InvalidStructure {
                    file_name: expected.file_name.to_owned(),
                    detail: format!("restriction {name:?} has no base QName"),
                }
            })?;
            let value = facet.attribute("value").ok_or_else(|| {
                RestrictionFacetQualificationError::InvalidStructure {
                    file_name: expected.file_name.to_owned(),
                    detail: format!(
                        "{} facet on {name:?} has no value attribute",
                        facet.tag_name().name()
                    ),
                }
            })?;

            let target = if facet.tag_name().name() == "pattern" {
                &mut actual_patterns
            } else {
                &mut actual_min_lengths
            };
            if target
                .insert(name.clone(), (base.to_owned(), value.to_owned()))
                .is_some()
            {
                return Err(RestrictionFacetQualificationError::InvalidStructure {
                    file_name: expected.file_name.to_owned(),
                    detail: format!("duplicate remaining restriction type {name:?}"),
                });
            }
        }
    }

    let expected_patterns = expected_patterns_for(profile);
    let expected_min_lengths = expected_min_lengths();
    if actual_patterns != expected_patterns || actual_min_lengths != expected_min_lengths {
        return Err(RestrictionFacetQualificationError::InventoryMismatch {
            expected_patterns,
            actual_patterns,
            expected_min_lengths,
            actual_min_lengths,
        });
    }

    let mut patterns = BTreeMap::new();
    for (local_name, (base, pattern)) in actual_patterns {
        let restriction_base = resolve_restriction_base(&base, &named_restrictions).map_err(
            |detail| RestrictionFacetQualificationError::InvalidStructure {
                file_name: "musicxml.xsd".to_owned(),
                detail: format!("pattern restriction {local_name:?}: {detail}"),
            },
        )?;
        let compiled = BoundRegex::compile(&pattern).map_err(|detail| {
            RestrictionFacetQualificationError::UnsupportedPatternSyntax {
                local_name: local_name.clone(),
                pattern: pattern.clone(),
                detail,
            }
        })?;
        patterns.insert(
            local_name,
            PatternRestriction {
                base: restriction_base,
                pattern,
                compiled,
            },
        );
    }

    let mut min_lengths = BTreeMap::new();
    for (local_name, (base, facet_lexical)) in actual_min_lengths {
        let restriction_base = resolve_restriction_base(&base, &named_restrictions).map_err(
            |detail| RestrictionFacetQualificationError::InvalidStructure {
                file_name: "musicxml.xsd".to_owned(),
                detail: format!("minLength restriction {local_name:?}: {detail}"),
            },
        )?;
        let min_length = facet_lexical.parse::<usize>().map_err(|_| {
            RestrictionFacetQualificationError::InvalidStructure {
                file_name: "musicxml.xsd".to_owned(),
                detail: format!(
                    "minLength restriction {local_name:?} has unsupported facet value {facet_lexical:?}"
                ),
            }
        })?;
        min_lengths.insert(
            local_name,
            MinLengthRestriction {
                base: restriction_base,
                min_length,
            },
        );
    }

    let qualification = RestrictionFacetQualification {
        patterns,
        min_lengths,
        named_restrictions,
    };

    let expected_pattern_count = match profile {
        BoundSchemaProfile::MusicXml31 => BOUND_PATTERN_RESTRICTION_COUNT,
        BoundSchemaProfile::MusicXml40 => MUSICXML40_PATTERN_RESTRICTION_COUNT,
    };
    if qualification.pattern_count() != expected_pattern_count
        || qualification.min_length_count() != BOUND_MIN_LENGTH_RESTRICTION_COUNT
        || qualification.facet_count() != expected_pattern_count + BOUND_MIN_LENGTH_RESTRICTION_COUNT
    {
        return Err(RestrictionFacetQualificationError::InvalidStructure {
            file_name: "musicxml.xsd".to_owned(),
            detail: "post-construction remaining restriction counts changed unexpectedly".to_owned(),
        });
    }

    for local_name in qualification
        .patterns
        .keys()
        .chain(qualification.min_lengths.keys())
    {
        let probe = semantic_probe(local_name);
        qualification.parse_value(local_name, probe).map_err(|error| {
            RestrictionFacetQualificationError::SemanticProbeFailed {
                local_name: local_name.to_owned(),
                detail: error.to_string(),
            }
        })?;
    }

    Ok(qualification)
}

fn resolve_restriction_base(
    lexical_qname: &str,
    named_restrictions: &NamedRestrictionQualification,
) -> Result<RestrictionBase, String> {
    if let Some((prefix, local_name)) = lexical_qname.split_once(':') {
        if prefix == "xs" && matches!(local_name, "token" | "date") {
            return Ok(RestrictionBase::Builtin(local_name.to_owned()));
        }
        return Err(format!("unsupported direct base QName {lexical_qname:?}"));
    }

    if named_restrictions.aliases().contains_key(lexical_qname) {
        return Ok(RestrictionBase::Named(lexical_qname.to_owned()));
    }

    Err(format!("unqualified named base {lexical_qname:?}"))
}

fn parse_restriction_base(
    base: &RestrictionBase,
    named_restrictions: &NamedRestrictionQualification,
    lexical: &str,
) -> Result<(AtomicValue, String), RestrictionFacetValueError> {
    // Every base used by the eleven bound r016 restrictions has whiteSpace=collapse:
    // xs:token, xs:date and smufl-glyph-name -> xs:NMTOKEN.
    let normalized = apply_white_space(WhiteSpaceMode::Collapse, lexical);
    let value = match base {
        RestrictionBase::Builtin(local_name) => parse_bound_builtin(local_name, lexical)
            .map_err(RestrictionFacetValueError::Datatype)?,
        RestrictionBase::Named(local_name) => named_restrictions
            .parse_value(local_name, lexical)
            .map_err(RestrictionFacetValueError::NamedDatatype)?,
    };
    Ok((value, normalized))
}

fn expected_patterns_for(profile: BoundSchemaProfile) -> BTreeMap<String, (String, String)> {
    let rows: &[(&str, &str, &str)] = match profile {
        BoundSchemaProfile::MusicXml31 => &[
            ("color", "xs:token", r"#[\dA-F]{6}([\dA-F][\dA-F])?"),
            ("comma-separated-text", "xs:token", r"[^,]+(, ?[^,]+)*"),
            ("smufl-accidental-glyph-name", "smufl-glyph-name", r"acc\c+"),
            ("smufl-coda-glyph-name", "smufl-glyph-name", r"coda\c*"),
            ("smufl-lyrics-glyph-name", "smufl-glyph-name", r"lyrics\c+"),
            ("smufl-pictogram-glyph-name", "smufl-glyph-name", r"pict\c+"),
            ("smufl-segno-glyph-name", "smufl-glyph-name", r"segno\c*"),
            ("time-only", "xs:token", r"[1-9][0-9]*(, ?[1-9][0-9]*)*"),
            ("yyyy-mm-dd", "xs:date", r"[^:Z]*"),
            ("ending-number", "xs:token", r"([ ]*)|([1-9][0-9]*(, ?[1-9][0-9]*)*)"),
        ],
        BoundSchemaProfile::MusicXml40 => &[
            ("color", "xs:token", r"#[\dA-F]{6}([\dA-F][\dA-F])?"),
            ("comma-separated-text", "xs:token", r"[^,]+(, ?[^,]+)*"),
            ("smufl-accidental-glyph-name", "smufl-glyph-name", r"(acc|medRenFla|medRenNatura|medRenShar|kievanAccidental)(\c+)"),
            ("smufl-coda-glyph-name", "smufl-glyph-name", r"coda\c*"),
            ("smufl-lyrics-glyph-name", "smufl-glyph-name", r"lyrics\c+"),
            ("smufl-pictogram-glyph-name", "smufl-glyph-name", r"pict\c+"),
            ("smufl-segno-glyph-name", "smufl-glyph-name", r"segno\c*"),
            ("smufl-wavy-line-glyph-name", "smufl-glyph-name", r"(wiggle\c+)|(guitar\c*VibratoStroke)"),
            ("time-only", "xs:token", r"[1-9][0-9]*(, ?[1-9][0-9]*)*"),
            ("yyyy-mm-dd", "xs:date", r"[^:Z]*"),
            ("ending-number", "xs:token", r"([ ]*)|([1-9][0-9]*(, ?[1-9][0-9]*)*)"),
        ],
    };
    rows.iter()
        .map(|(name, base, pattern)| {
            ((*name).to_owned(), ((*base).to_owned(), (*pattern).to_owned()))
        })
        .collect()
}

fn expected_min_lengths() -> BTreeMap<String, (String, String)> {
    [("measure-text", "xs:token", "1")]
        .into_iter()
        .map(|(name, base, value)| {
            (name.to_owned(), (base.to_owned(), value.to_owned()))
        })
        .collect()
}

fn semantic_probe(local_name: &str) -> &'static str {
    match local_name {
        "color" => "#00A1FF",
        "comma-separated-text" => "left, right",
        "smufl-accidental-glyph-name" => "accidentalSharp",
        "smufl-coda-glyph-name" => "coda",
        "smufl-lyrics-glyph-name" => "lyricsElision",
        "smufl-pictogram-glyph-name" => "pictChimes",
        "smufl-segno-glyph-name" => "segno",
        "smufl-wavy-line-glyph-name" => "wiggleVibratoSmallestFast",
        "time-only" => "3, 2, 4",
        "yyyy-mm-dd" => "2026-09-05",
        "ending-number" => "1, 2",
        "measure-text" => "x",
        _ => unreachable!("semantic probe requested only for exact r016 inventory"),
    }
}

pub(crate) fn matches_bound_xsd_pattern(pattern: &str, lexical: &str) -> Result<bool, String> {
    let compiled = BoundRegex::compile(pattern)?;
    Ok(compiled.matches(lexical))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BoundRegex {
    expression: Expression,
}

impl BoundRegex {
    fn compile(pattern: &str) -> Result<Self, String> {
        let mut parser = RegexParser::new(pattern);
        let expression = parser.parse_expression()?;
        if let Some(ch) = parser.peek() {
            return Err(format!(
                "unexpected trailing character {ch:?} at position {}",
                parser.position
            ));
        }
        Ok(Self { expression })
    }

    fn matches(&self, lexical: &str) -> bool {
        let input: Vec<char> = lexical.chars().collect();
        match_expression(&self.expression, &input, 0).contains(&input.len())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Expression {
    branches: Vec<Branch>,
}

type Branch = Vec<Piece>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Piece {
    atom: Atom,
    min: usize,
    max: Option<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Atom {
    Literal(char),
    Class(CharacterClass),
    Group(Box<Expression>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CharacterClass {
    negated: bool,
    items: Vec<CharacterClassItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CharacterClassItem {
    Literal(char),
    Range(char, char),
    Digit,
    NameChar,
}

struct RegexParser {
    chars: Vec<char>,
    position: usize,
}

impl RegexParser {
    fn new(pattern: &str) -> Self {
        Self {
            chars: pattern.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
        Some(ch)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut branches = vec![self.parse_branch()?];
        while self.peek() == Some('|') {
            self.bump();
            branches.push(self.parse_branch()?);
        }
        Ok(Expression { branches })
    }

    fn parse_branch(&mut self) -> Result<Branch, String> {
        let mut pieces = Vec::new();
        while let Some(ch) = self.peek() {
            if matches!(ch, '|' | ')') {
                break;
            }
            pieces.push(self.parse_piece()?);
        }
        Ok(pieces)
    }

    fn parse_piece(&mut self) -> Result<Piece, String> {
        let atom = self.parse_atom()?;
        let (min, max) = match self.peek() {
            Some('?') => {
                self.bump();
                (0, Some(1))
            }
            Some('*') => {
                self.bump();
                (0, None)
            }
            Some('+') => {
                self.bump();
                (1, None)
            }
            Some('{') => self.parse_exact_quantifier()?,
            _ => (1, Some(1)),
        };
        Ok(Piece { atom, min, max })
    }

    fn parse_exact_quantifier(&mut self) -> Result<(usize, Option<usize>), String> {
        let start = self.position;
        if self.bump() != Some('{') {
            return Err(format!(
                "expected exact XSD quantifier opening brace at position {start}"
            ));
        }
        let mut digits = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                digits.push(ch);
                self.bump();
            } else {
                break;
            }
        }
        if digits.is_empty() || self.bump() != Some('}') {
            return Err(format!(
                "only exact XSD quantifiers {{n}} are used by the bound profile; invalid quantifier at position {start}"
            ));
        }
        let count = digits
            .parse::<usize>()
            .map_err(|_| format!("quantifier at position {start} is too large"))?;
        Ok((count, Some(count)))
    }

    fn parse_atom(&mut self) -> Result<Atom, String> {
        let position = self.position;
        match self.bump() {
            Some('(') => {
                let expression = self.parse_expression()?;
                if self.bump() != Some(')') {
                    return Err(format!("unclosed group starting at position {position}"));
                }
                Ok(Atom::Group(Box::new(expression)))
            }
            Some('[') => self.parse_character_class(position).map(Atom::Class),
            Some('\\') => self.parse_class_escape(position).map(|item| {
                Atom::Class(CharacterClass {
                    negated: false,
                    items: vec![item],
                })
            }),
            Some(ch) if is_profile_regex_metacharacter(ch) => Err(format!(
                "unsupported/unexpected regex metacharacter {ch:?} at position {position}"
            )),
            Some(ch) => Ok(Atom::Literal(ch)),
            None => Err("expected regex atom at end of pattern".to_owned()),
        }
    }

    fn parse_character_class(&mut self, start: usize) -> Result<CharacterClass, String> {
        let negated = if self.peek() == Some('^') {
            self.bump();
            true
        } else {
            false
        };
        let mut items = Vec::new();

        loop {
            match self.peek() {
                Some(']') => {
                    self.bump();
                    break;
                }
                None => {
                    return Err(format!(
                        "unclosed character class starting at position {start}"
                    ));
                }
                _ => {}
            }

            let item_start = self.position;
            let first = self.parse_character_class_item()?;
            if let CharacterClassItem::Literal(first_char) = first {
                if self.peek() == Some('-')
                    && self.chars.get(self.position + 1).copied() != Some(']')
                {
                    self.bump();
                    let second = self.parse_character_class_item()?;
                    let CharacterClassItem::Literal(second_char) = second else {
                        return Err(format!(
                            "bound profile does not use escaped class-range endpoints at position {item_start}"
                        ));
                    };
                    if first_char > second_char {
                        return Err(format!(
                            "descending character range {first_char:?}-{second_char:?} at position {item_start}"
                        ));
                    }
                    items.push(CharacterClassItem::Range(first_char, second_char));
                    continue;
                }
                items.push(CharacterClassItem::Literal(first_char));
            } else {
                items.push(first);
            }
        }

        if items.is_empty() {
            return Err(format!(
                "empty character class starting at position {start} is outside XSD 1.0 grammar"
            ));
        }

        Ok(CharacterClass { negated, items })
    }

    fn parse_character_class_item(&mut self) -> Result<CharacterClassItem, String> {
        let position = self.position;
        match self.bump() {
            Some('\\') => self.parse_class_escape(position),
            Some('[') => Err(format!(
                "nested/class-subtraction syntax is outside the bound profile at position {position}"
            )),
            Some(']') => Err(format!(
                "unexpected character-class terminator at position {position}"
            )),
            Some(ch) => Ok(CharacterClassItem::Literal(ch)),
            None => Err("unexpected end of character class".to_owned()),
        }
    }

    fn parse_class_escape(&mut self, position: usize) -> Result<CharacterClassItem, String> {
        match self.bump() {
            Some('d') => Ok(CharacterClassItem::Digit),
            Some('c') => Ok(CharacterClassItem::NameChar),
            Some(ch) => Err(format!(
                "escape \\{ch} at position {position} is outside the exact bound pattern profile"
            )),
            None => Err(format!("dangling escape at position {position}")),
        }
    }
}

fn is_profile_regex_metacharacter(ch: char) -> bool {
    matches!(ch, '.' | '?' | '*' | '+' | '{' | '}' | '(' | ')' | '[' | ']' | '|')
}

fn match_expression(expression: &Expression, input: &[char], start: usize) -> BTreeSet<usize> {
    let mut output = BTreeSet::new();
    for branch in &expression.branches {
        output.extend(match_branch(branch, input, start));
    }
    output
}

fn match_branch(branch: &Branch, input: &[char], start: usize) -> BTreeSet<usize> {
    let mut positions = BTreeSet::from([start]);
    for piece in branch {
        let mut next = BTreeSet::new();
        for position in positions {
            next.extend(match_piece(piece, input, position));
        }
        if next.is_empty() {
            return next;
        }
        positions = next;
    }
    positions
}

fn match_piece(piece: &Piece, input: &[char], start: usize) -> BTreeSet<usize> {
    let upper = piece
        .max
        .unwrap_or_else(|| input.len().saturating_sub(start).saturating_add(1));
    let mut accepted = BTreeSet::new();
    let mut current = BTreeSet::from([start]);
    if piece.min == 0 {
        accepted.insert(start);
    }

    for repetition in 1..=upper {
        let mut next = BTreeSet::new();
        for position in current {
            next.extend(match_atom(&piece.atom, input, position));
        }
        if next.is_empty() {
            break;
        }
        if repetition >= piece.min {
            accepted.extend(next.iter().copied());
        }
        current = next;
    }

    accepted
}

fn match_atom(atom: &Atom, input: &[char], start: usize) -> BTreeSet<usize> {
    match atom {
        Atom::Literal(expected) => {
            if input.get(start) == Some(expected) {
                BTreeSet::from([start + 1])
            } else {
                BTreeSet::new()
            }
        }
        Atom::Class(class) => {
            let Some(ch) = input.get(start).copied() else {
                return BTreeSet::new();
            };
            if class.matches(ch) {
                BTreeSet::from([start + 1])
            } else {
                BTreeSet::new()
            }
        }
        Atom::Group(expression) => match_expression(expression, input, start),
    }
}

impl CharacterClass {
    fn matches(&self, ch: char) -> bool {
        if !is_xml10_second_edition_char(ch) {
            return false;
        }
        let member = self.items.iter().any(|item| item.matches(ch));
        if self.negated { !member } else { member }
    }
}

impl CharacterClassItem {
    fn matches(&self, ch: char) -> bool {
        match self {
            Self::Literal(expected) => ch == *expected,
            Self::Range(start, end) => (*start..=*end).contains(&ch),
            Self::Digit => is_unicode_3_1_decimal_digit(ch),
            Self::NameChar => is_xml10_second_edition_name_char(ch),
        }
    }
}

fn is_xml10_second_edition_char(ch: char) -> bool {
    matches!(
        ch as u32,
        0x0009 | 0x000A | 0x000D | 0x0020..=0xD7FF | 0xE000..=0xFFFD | 0x10000..=0x10FFFF
    )
}

// XSD 1.0 Second Edition defines `\d` as `\p{Nd}` and normatively references the Unicode
// database current for the Recommendation. These are the 248 Nd code points from Unicode 3.1.0
// DerivedGeneralCategory-3.1.0.txt. Keeping the table explicit prevents a host Unicode upgrade
// from silently widening the historical schema profile.
const UNICODE_3_1_ND_RANGES: &[(u32, u32)] = &[
    (0x0030, 0x0039),
    (0x0660, 0x0669),
    (0x06F0, 0x06F9),
    (0x0966, 0x096F),
    (0x09E6, 0x09EF),
    (0x0A66, 0x0A6F),
    (0x0AE6, 0x0AEF),
    (0x0B66, 0x0B6F),
    (0x0BE7, 0x0BEF),
    (0x0C66, 0x0C6F),
    (0x0CE6, 0x0CEF),
    (0x0D66, 0x0D6F),
    (0x0E50, 0x0E59),
    (0x0ED0, 0x0ED9),
    (0x0F20, 0x0F29),
    (0x1040, 0x1049),
    (0x1369, 0x1371),
    (0x17E0, 0x17E9),
    (0x1810, 0x1819),
    (0xFF10, 0xFF19),
    (0x1D7CE, 0x1D7FF),
];

fn is_unicode_3_1_decimal_digit(ch: char) -> bool {
    let codepoint = ch as u32;
    UNICODE_3_1_ND_RANGES
        .iter()
        .any(|&(start, end)| (start..=end).contains(&codepoint))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MUSICXML31_SCHEMA_DIR, MUSICXML40_SCHEMA_DIR};

    fn qualification() -> RestrictionFacetQualification {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        qualify_bound_remaining_restriction_facets(&root)
            .expect("bound remaining restriction facets must qualify")
    }

    #[test]
    fn exact_remaining_restriction_inventory_qualifies() {
        let qualification = qualification();
        assert_eq!(qualification.pattern_count(), 10);
        assert_eq!(qualification.min_length_count(), 1);
        assert_eq!(qualification.facet_count(), 11);
    }

    #[test]
    fn unicode_3_1_nd_table_has_exactly_248_codepoints() {
        let count = UNICODE_3_1_ND_RANGES
            .iter()
            .map(|(start, end)| *end - *start + 1)
            .sum::<u32>();
        assert_eq!(count, 248);
    }

    #[test]
    fn color_uses_xsd_digit_category_without_later_unicode_widening() {
        let qualification = qualification();
        assert!(qualification.parse_value("color", "#12A0FF").is_ok());
        assert!(qualification.parse_value("color", "#١٢A0FF").is_ok());
        assert!(qualification.parse_value("color", "#12A0FF80").is_ok());
        assert!(qualification.parse_value("color", "#12A0FF8").is_err());

        // U+1946 became Nd after Unicode 3.1; libxml2 2.15.4's newer Unicode tables admit it,
        // but the historical XSD 1.0 profile must not widen silently.
        assert!(qualification.parse_value("color", "#᥆2A0FF").is_err());
    }

    #[test]
    fn comma_separated_text_observes_token_collapse_before_pattern() {
        let qualification = qualification();
        assert!(qualification
            .parse_value("comma-separated-text", " alpha,   beta ")
            .is_ok());
        assert!(qualification
            .parse_value("comma-separated-text", "alpha,beta")
            .is_ok());
        assert!(qualification
            .parse_value("comma-separated-text", "alpha,")
            .is_err());
    }

    #[test]
    fn smufl_patterns_reuse_historical_xml_namechar_semantics() {
        let qualification = qualification();
        assert!(qualification
            .parse_value("smufl-accidental-glyph-name", "accSharp")
            .is_ok());
        assert!(qualification
            .parse_value("smufl-coda-glyph-name", "coda")
            .is_ok());
        assert!(qualification
            .parse_value("smufl-lyrics-glyph-name", "lyricsA")
            .is_ok());
        assert!(qualification
            .parse_value("smufl-pictogram-glyph-name", "pictA")
            .is_ok());
        assert!(qualification
            .parse_value("smufl-segno-glyph-name", "segno")
            .is_ok());
        assert!(qualification
            .parse_value("smufl-accidental-glyph-name", "acc")
            .is_err());
    }

    #[test]
    fn time_only_requires_positive_ascii_integer_components() {
        let qualification = qualification();
        assert!(qualification.parse_value("time-only", "1").is_ok());
        assert!(qualification.parse_value("time-only", "12, 3,45").is_ok());
        assert!(qualification.parse_value("time-only", "0").is_err());
        assert!(qualification.parse_value("time-only", "01").is_err());
        assert!(qualification.parse_value("time-only", "1,").is_err());
    }

    #[test]
    fn yyyy_mm_dd_requires_a_valid_date_and_rejects_timezone_lexicals() {
        let qualification = qualification();
        assert!(qualification.parse_value("yyyy-mm-dd", "2026-09-05").is_ok());
        assert!(qualification.parse_value("yyyy-mm-dd", "2026-02-30").is_err());
        assert!(qualification
            .parse_value("yyyy-mm-dd", "2026-09-05Z")
            .is_err());
        assert!(qualification
            .parse_value("yyyy-mm-dd", "2026-09-05+02:00")
            .is_err());
    }

    #[test]
    fn ending_number_accepts_empty_or_positive_ascii_integer_list() {
        let qualification = qualification();
        assert!(qualification.parse_value("ending-number", "").is_ok());
        assert!(qualification.parse_value("ending-number", "   ").is_ok());
        assert!(qualification.parse_value("ending-number", "1, 2,10").is_ok());
        assert!(qualification.parse_value("ending-number", "0").is_err());
        assert!(qualification.parse_value("ending-number", "1,").is_err());
    }

    #[test]
    fn min_length_counts_xml_characters_after_token_collapse() {
        let qualification = qualification();
        assert!(qualification.parse_value("measure-text", "x").is_ok());
        assert!(qualification.parse_value("measure-text", "  𝄞  ").is_ok());
        assert!(qualification.parse_value("measure-text", "").is_err());
        assert!(qualification.parse_value("measure-text", " \t\r\n ").is_err());
    }

    #[test]
    fn profile_regex_is_implicitly_anchored() {
        let regex = BoundRegex::compile("abc").expect("profile literal regex must compile");
        assert!(regex.matches("abc"));
        assert!(!regex.matches("xabc"));
        assert!(!regex.matches("abcx"));
    }

    #[test]
    fn unsupported_regex_syntax_fails_closed() {
        assert!(BoundRegex::compile(".").is_err());
        assert!(BoundRegex::compile(r"\s+").is_err());
        assert!(BoundRegex::compile("a{1,2}").is_err());
    }

    #[test]
    fn unknown_remaining_restriction_type_fails_closed() {
        let qualification = qualification();
        assert!(matches!(
            qualification.parse_value("not-bound", "x"),
            Err(RestrictionFacetValueError::UnknownQualifiedType { .. })
        ));
    }
    #[test]
    fn musicxml40_wavy_line_pattern_is_inside_the_bound_regex_semantics() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let qualification = qualify_bound_remaining_restriction_facets(&root)
            .expect("bound 4.0 remaining restrictions must qualify");
        assert_eq!(qualification.pattern_count(), 11);
        assert!(qualification.parse_value("smufl-wavy-line-glyph-name", "wiggleVibratoSmallestFast").is_ok());
        assert!(qualification.parse_value("smufl-wavy-line-glyph-name", "notAWavyGlyph").is_err());
    }

}
