//! XSD 1.0 simple-type semantic foundation shared by the exact bound MusicXML 3.1 and 4.0 profiles.
//!
//! r014 retains all thirteen bound built-in datatypes and adds direct-builtin numeric-bound
//! restriction value semantics alongside the r013 enumeration subset.
//! Full simple-type profile qualification remains separate.

use std::cmp::Ordering;
use std::fmt;

pub const BOUND_BUILTIN_TYPE_COUNT: usize = 13;
pub const IMPLEMENTED_BUILTIN_TYPE_COUNT: usize = 13;
const _: () = assert!(IMPLEMENTED_BUILTIN_TYPE_COUNT == BOUND_BUILTIN_TYPE_COUNT);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WhiteSpaceMode {
    Preserve,
    Collapse,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImplementedBuiltinType {
    String,
    Token,
    Decimal,
    Integer,
    NonNegativeInteger,
    PositiveInteger,
    NmToken,
    NcName,
    Id,
    IdRef,
    Language,
    Date,
    AnyUri,
}

impl ImplementedBuiltinType {
    #[must_use]
    pub const fn local_name(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Token => "token",
            Self::Decimal => "decimal",
            Self::Integer => "integer",
            Self::NonNegativeInteger => "nonNegativeInteger",
            Self::PositiveInteger => "positiveInteger",
            Self::NmToken => "NMTOKEN",
            Self::NcName => "NCName",
            Self::Id => "ID",
            Self::IdRef => "IDREF",
            Self::Language => "language",
            Self::Date => "date",
            Self::AnyUri => "anyURI",
        }
    }

    #[must_use]
    pub const fn white_space(self) -> WhiteSpaceMode {
        match self {
            Self::String => WhiteSpaceMode::Preserve,
            Self::Token
            | Self::Decimal
            | Self::Integer
            | Self::NonNegativeInteger
            | Self::PositiveInteger
            | Self::NmToken
            | Self::NcName
            | Self::Id
            | Self::IdRef
            | Self::Language
            | Self::Date
            | Self::AnyUri => WhiteSpaceMode::Collapse,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecimalValue {
    negative: bool,
    integer_digits: String,
    fractional_digits: String,
}

impl DecimalValue {
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    #[must_use]
    pub fn integer_digits(&self) -> &str {
        &self.integer_digits
    }

    #[must_use]
    pub fn fractional_digits(&self) -> &str {
        &self.fractional_digits
    }

    /// Canonical lexical representation of the parsed XSD decimal value.
    #[must_use]
    pub fn canonical_lexical(&self) -> String {
        let sign = if self.negative { "-" } else { "" };
        let fraction = if self.fractional_digits.is_empty() {
            "0"
        } else {
            &self.fractional_digits
        };
        format!("{sign}{}.{fraction}", self.integer_digits)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerValue {
    negative: bool,
    digits: String,
}

impl IntegerValue {
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    #[must_use]
    pub fn digits(&self) -> &str {
        &self.digits
    }

    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.digits == "0"
    }

    /// Canonical lexical representation of the parsed integer value.
    #[must_use]
    pub fn canonical_lexical(&self) -> String {
        if self.negative {
            format!("-{}", self.digits)
        } else {
            self.digits.clone()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateValue {
    negative_year: bool,
    year_digits: String,
    month: u8,
    day: u8,
    timezone_minutes: Option<i16>,
}

impl DateValue {
    #[must_use]
    pub const fn is_negative_year(&self) -> bool {
        self.negative_year
    }

    #[must_use]
    pub fn year_digits(&self) -> &str {
        &self.year_digits
    }

    #[must_use]
    pub const fn month(&self) -> u8 {
        self.month
    }

    #[must_use]
    pub const fn day(&self) -> u8 {
        self.day
    }

    #[must_use]
    pub const fn timezone_minutes(&self) -> Option<i16> {
        self.timezone_minutes
    }

    /// Canonical lexical representation of the parsed XSD 1.0 date value.
    #[must_use]
    pub fn canonical_lexical(&self) -> String {
        let year = if self.year_digits.len() < 4 {
            format!("{:0>4}", self.year_digits)
        } else {
            self.year_digits.clone()
        };
        let sign = if self.negative_year { "-" } else { "" };
        let timezone = match self.timezone_minutes {
            None => String::new(),
            Some(0) => "Z".to_owned(),
            Some(minutes) => {
                let sign = if minutes < 0 { '-' } else { '+' };
                let magnitude = minutes.unsigned_abs();
                format!("{sign}{:02}:{:02}", magnitude / 60, magnitude % 60)
            }
        };
        format!(
            "{sign}{year}-{:02}-{:02}{timezone}",
            self.month, self.day
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AtomicValue {
    String(String),
    Decimal(DecimalValue),
    Integer(IntegerValue),
    Date(DateValue),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimpleTypeError {
    BoundBuiltinSemanticsNotImplemented { local_name: String },
    UnknownBuiltinType { local_name: String },
    InvalidLexicalForm {
        local_name: &'static str,
        normalized_lexical: String,
    },
    ValueConstraintViolation {
        local_name: &'static str,
        normalized_lexical: String,
        constraint: &'static str,
    },
    UnsupportedEnumerationBase {
        local_name: String,
    },
    InvalidEnumerationFacetValue {
        base_local_name: String,
        lexical: String,
        detail: String,
    },
    EmptyEnumerationFacetSet {
        base_local_name: String,
    },
    EnumerationConstraintViolation {
        base_local_name: String,
        lexical: String,
    },
    UnsupportedNumericRestrictionBase {
        local_name: String,
    },
    InvalidNumericFacetValue {
        base_local_name: String,
        facet: &'static str,
        lexical: String,
        detail: String,
    },
    NumericBoundConstraintViolation {
        base_local_name: String,
        lexical: String,
        facet: &'static str,
        facet_lexical: String,
    },
}

impl fmt::Display for SimpleTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BoundBuiltinSemanticsNotImplemented { local_name } => write!(
                formatter,
                "xs:{local_name} is part of a bound MusicXML profile but its value semantics are not implemented"
            ),
            Self::UnknownBuiltinType { local_name } => {
                write!(formatter, "unknown XSD built-in datatype xs:{local_name}")
            }
            Self::InvalidLexicalForm {
                local_name,
                normalized_lexical,
            } => write!(
                formatter,
                "xs:{local_name} lexical form is invalid after XSD whiteSpace processing: {normalized_lexical:?}"
            ),
            Self::ValueConstraintViolation {
                local_name,
                normalized_lexical,
                constraint,
            } => write!(
                formatter,
                "xs:{local_name} value violates {constraint}: {normalized_lexical:?}"
            ),
            Self::UnsupportedEnumerationBase { local_name } => write!(
                formatter,
                "xs:{local_name} is outside the r013 direct-builtin enumeration restriction profile"
            ),
            Self::InvalidEnumerationFacetValue {
                base_local_name,
                lexical,
                detail,
            } => write!(
                formatter,
                "enumeration facet value {lexical:?} is invalid for xs:{base_local_name}: {detail}"
            ),
            Self::EmptyEnumerationFacetSet { base_local_name } => write!(
                formatter,
                "enumeration restriction over xs:{base_local_name} has no enumeration values"
            ),
            Self::EnumerationConstraintViolation {
                base_local_name,
                lexical,
            } => write!(
                formatter,
                "value {lexical:?} is not in the enumeration value set over xs:{base_local_name}"
            ),
            Self::UnsupportedNumericRestrictionBase { local_name } => write!(
                formatter,
                "xs:{local_name} is outside the r014 direct-builtin numeric-bound restriction profile"
            ),
            Self::InvalidNumericFacetValue {
                base_local_name,
                facet,
                lexical,
                detail,
            } => write!(
                formatter,
                "{facet} facet value {lexical:?} is invalid for xs:{base_local_name}: {detail}"
            ),
            Self::NumericBoundConstraintViolation {
                base_local_name,
                lexical,
                facet,
                facet_lexical,
            } => write!(
                formatter,
                "value {lexical:?} violates {facet}={facet_lexical:?} for xs:{base_local_name}"
            ),
        }
    }
}

impl std::error::Error for SimpleTypeError {}

#[must_use]
pub fn apply_white_space(mode: WhiteSpaceMode, lexical: &str) -> String {
    match mode {
        WhiteSpaceMode::Preserve => lexical.to_owned(),
        WhiteSpaceMode::Collapse => collapse_xml_schema_white_space(lexical),
    }
}

/// Parse one XSD built-in datatype from the exact bound MusicXML score-schema profiles.
///
/// Known profile types whose semantics are not implemented yet are rejected
/// explicitly. Unknown built-ins are rejected separately. There is no
/// best-effort fallback to string semantics.
pub fn parse_bound_builtin(
    local_name: &str,
    lexical: &str,
) -> Result<AtomicValue, SimpleTypeError> {
    let builtin = match local_name {
        "string" => ImplementedBuiltinType::String,
        "token" => ImplementedBuiltinType::Token,
        "decimal" => ImplementedBuiltinType::Decimal,
        "integer" => ImplementedBuiltinType::Integer,
        "nonNegativeInteger" => ImplementedBuiltinType::NonNegativeInteger,
        "positiveInteger" => ImplementedBuiltinType::PositiveInteger,
        "NMTOKEN" => ImplementedBuiltinType::NmToken,
        "NCName" => ImplementedBuiltinType::NcName,
        "ID" => ImplementedBuiltinType::Id,
        "IDREF" => ImplementedBuiltinType::IdRef,
        "language" => ImplementedBuiltinType::Language,
        "date" => ImplementedBuiltinType::Date,
        "anyURI" => ImplementedBuiltinType::AnyUri,
        _ => {
            return Err(SimpleTypeError::UnknownBuiltinType {
                local_name: local_name.to_owned(),
            });
        }
    };

    let normalized = apply_white_space(builtin.white_space(), lexical);
    match builtin {
        ImplementedBuiltinType::String | ImplementedBuiltinType::Token => {
            Ok(AtomicValue::String(normalized))
        }
        ImplementedBuiltinType::Decimal => parse_decimal(&normalized).map(AtomicValue::Decimal),
        ImplementedBuiltinType::Integer => parse_integer(&normalized, IntegerConstraint::Any)
            .map(AtomicValue::Integer),
        ImplementedBuiltinType::NonNegativeInteger => {
            parse_integer(&normalized, IntegerConstraint::NonNegative).map(AtomicValue::Integer)
        }
        ImplementedBuiltinType::PositiveInteger => {
            parse_integer(&normalized, IntegerConstraint::Positive).map(AtomicValue::Integer)
        }
        ImplementedBuiltinType::NmToken => parse_nmtoken(&normalized).map(AtomicValue::String),
        ImplementedBuiltinType::NcName => parse_ncname_for("NCName", &normalized).map(AtomicValue::String),
        ImplementedBuiltinType::Id => parse_ncname_for("ID", &normalized).map(AtomicValue::String),
        ImplementedBuiltinType::IdRef => parse_ncname_for("IDREF", &normalized).map(AtomicValue::String),
        ImplementedBuiltinType::Language => parse_language(&normalized).map(AtomicValue::String),
        ImplementedBuiltinType::Date => parse_date(&normalized).map(AtomicValue::Date),
        ImplementedBuiltinType::AnyUri => parse_any_uri(&normalized).map(AtomicValue::String),
    }
}

/// Parse a value against one direct-built-in enumeration restriction from the bound profile.
///
/// r013 deliberately limits this operation to the four built-in bases actually used by all
/// enumeration-only restrictions in the exact bound MusicXML score-schema bundles. Enumeration facet
/// lexicals are parsed through the base datatype first, so membership is value-space equality
/// rather than raw lexical-string equality.
pub fn parse_bound_enumeration_restriction(
    base_local_name: &str,
    lexical: &str,
    enumeration_lexicals: &[&str],
) -> Result<AtomicValue, SimpleTypeError> {
    if !matches!(base_local_name, "string" | "token" | "NMTOKEN" | "NCName") {
        return Err(SimpleTypeError::UnsupportedEnumerationBase {
            local_name: base_local_name.to_owned(),
        });
    }
    if enumeration_lexicals.is_empty() {
        return Err(SimpleTypeError::EmptyEnumerationFacetSet {
            base_local_name: base_local_name.to_owned(),
        });
    }

    let candidate = parse_bound_builtin(base_local_name, lexical)?;
    let allowed_values: Vec<AtomicValue> = enumeration_lexicals
        .iter()
        .map(|enumeration_lexical| {
            parse_bound_builtin(base_local_name, enumeration_lexical).map_err(|error| {
                SimpleTypeError::InvalidEnumerationFacetValue {
                    base_local_name: base_local_name.to_owned(),
                    lexical: (*enumeration_lexical).to_owned(),
                    detail: error.to_string(),
                }
            })
        })
        .collect::<Result<_, _>>()?;

    if allowed_values.iter().any(|allowed| allowed == &candidate) {
        return Ok(candidate);
    }

    Err(SimpleTypeError::EnumerationConstraintViolation {
        base_local_name: base_local_name.to_owned(),
        lexical: lexical.to_owned(),
    })
}


pub fn parse_bound_numeric_restriction(
    base_local_name: &str,
    lexical: &str,
    min_inclusive: Option<&str>,
    max_inclusive: Option<&str>,
    min_exclusive: Option<&str>,
) -> Result<AtomicValue, SimpleTypeError> {
    if !matches!(
        base_local_name,
        "decimal" | "integer" | "nonNegativeInteger" | "positiveInteger"
    ) {
        return Err(SimpleTypeError::UnsupportedNumericRestrictionBase {
            local_name: base_local_name.to_owned(),
        });
    }

    let candidate = parse_bound_builtin(base_local_name, lexical)?;
    let facets = [
        ("minInclusive", min_inclusive, OrderingPredicate::GreaterOrEqual),
        ("maxInclusive", max_inclusive, OrderingPredicate::LessOrEqual),
        ("minExclusive", min_exclusive, OrderingPredicate::Greater),
    ];

    for (facet, facet_lexical, predicate) in facets {
        let Some(facet_lexical) = facet_lexical else {
            continue;
        };
        let bound = parse_bound_builtin(base_local_name, facet_lexical).map_err(|error| {
            SimpleTypeError::InvalidNumericFacetValue {
                base_local_name: base_local_name.to_owned(),
                facet,
                lexical: facet_lexical.to_owned(),
                detail: error.to_string(),
            }
        })?;
        let ordering = compare_numeric_atomic_values(&candidate, &bound).ok_or_else(|| {
            SimpleTypeError::UnsupportedNumericRestrictionBase {
                local_name: base_local_name.to_owned(),
            }
        })?;

        if !predicate.accepts(ordering) {
            return Err(SimpleTypeError::NumericBoundConstraintViolation {
                base_local_name: base_local_name.to_owned(),
                lexical: lexical.to_owned(),
                facet,
                facet_lexical: facet_lexical.to_owned(),
            });
        }
    }

    Ok(candidate)
}

#[derive(Clone, Copy)]
enum OrderingPredicate {
    GreaterOrEqual,
    LessOrEqual,
    Greater,
}

impl OrderingPredicate {
    fn accepts(self, ordering: Ordering) -> bool {
        match self {
            Self::GreaterOrEqual => ordering != Ordering::Less,
            Self::LessOrEqual => ordering != Ordering::Greater,
            Self::Greater => ordering == Ordering::Greater,
        }
    }
}

pub(crate) fn compare_numeric_atomic_values(left: &AtomicValue, right: &AtomicValue) -> Option<Ordering> {
    match (left, right) {
        (AtomicValue::Decimal(left), AtomicValue::Decimal(right)) => {
            Some(compare_decimal_values(left, right))
        }
        (AtomicValue::Integer(left), AtomicValue::Integer(right)) => {
            Some(compare_integer_values(left, right))
        }
        _ => None,
    }
}

fn compare_integer_values(left: &IntegerValue, right: &IntegerValue) -> Ordering {
    match (left.negative, right.negative) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => compare_unsigned_digits(&left.digits, &right.digits),
        (true, true) => compare_unsigned_digits(&right.digits, &left.digits),
    }
}

fn compare_decimal_values(left: &DecimalValue, right: &DecimalValue) -> Ordering {
    match (left.negative, right.negative) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => compare_decimal_magnitude(left, right),
        (true, true) => compare_decimal_magnitude(right, left),
    }
}

fn compare_decimal_magnitude(left: &DecimalValue, right: &DecimalValue) -> Ordering {
    let integer_ordering = compare_unsigned_digits(&left.integer_digits, &right.integer_digits);
    if integer_ordering != Ordering::Equal {
        return integer_ordering;
    }

    let max_fraction = left.fractional_digits.len().max(right.fractional_digits.len());
    let left_bytes = left.fractional_digits.as_bytes();
    let right_bytes = right.fractional_digits.as_bytes();

    for index in 0..max_fraction {
        let left_digit = left_bytes.get(index).copied().unwrap_or(b'0');
        let right_digit = right_bytes.get(index).copied().unwrap_or(b'0');
        match left_digit.cmp(&right_digit) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }

    Ordering::Equal
}

fn compare_unsigned_digits(left: &str, right: &str) -> Ordering {
    left.len()
        .cmp(&right.len())
        .then_with(|| left.as_bytes().cmp(right.as_bytes()))
}

fn collapse_xml_schema_white_space(lexical: &str) -> String {
    let mut output = String::with_capacity(lexical.len());
    let mut pending_space = false;

    for ch in lexical.chars() {
        let is_xml_space = matches!(ch, '\u{0009}' | '\u{000A}' | '\u{000D}' | '\u{0020}');
        if is_xml_space {
            if !output.is_empty() {
                pending_space = true;
            }
            continue;
        }
        if pending_space {
            output.push(' ');
            pending_space = false;
        }
        output.push(ch);
    }

    output
}

// XML Schema 1.0 Second Edition binds NMTOKEN to the XML 1.0 Second Edition
// Nmtoken production and NCName to the Namespaces in XML production. These
// explicit tables deliberately preserve that historical character boundary;
// Rust's current Unicode predicates would silently accept a broader set.
const XML10_2E_BASE_CHAR_RANGES: &[(u32, u32)] = &[
    (0x0041, 0x005A),
    (0x0061, 0x007A),
    (0x00C0, 0x00D6),
    (0x00D8, 0x00F6),
    (0x00F8, 0x00FF),
    (0x0100, 0x0131),
    (0x0134, 0x013E),
    (0x0141, 0x0148),
    (0x014A, 0x017E),
    (0x0180, 0x01C3),
    (0x01CD, 0x01F0),
    (0x01F4, 0x01F5),
    (0x01FA, 0x0217),
    (0x0250, 0x02A8),
    (0x02BB, 0x02C1),
    (0x0386, 0x0386),
    (0x0388, 0x038A),
    (0x038C, 0x038C),
    (0x038E, 0x03A1),
    (0x03A3, 0x03CE),
    (0x03D0, 0x03D6),
    (0x03DA, 0x03DA),
    (0x03DC, 0x03DC),
    (0x03DE, 0x03DE),
    (0x03E0, 0x03E0),
    (0x03E2, 0x03F3),
    (0x0401, 0x040C),
    (0x040E, 0x044F),
    (0x0451, 0x045C),
    (0x045E, 0x0481),
    (0x0490, 0x04C4),
    (0x04C7, 0x04C8),
    (0x04CB, 0x04CC),
    (0x04D0, 0x04EB),
    (0x04EE, 0x04F5),
    (0x04F8, 0x04F9),
    (0x0531, 0x0556),
    (0x0559, 0x0559),
    (0x0561, 0x0586),
    (0x05D0, 0x05EA),
    (0x05F0, 0x05F2),
    (0x0621, 0x063A),
    (0x0641, 0x064A),
    (0x0671, 0x06B7),
    (0x06BA, 0x06BE),
    (0x06C0, 0x06CE),
    (0x06D0, 0x06D3),
    (0x06D5, 0x06D5),
    (0x06E5, 0x06E6),
    (0x0905, 0x0939),
    (0x093D, 0x093D),
    (0x0958, 0x0961),
    (0x0985, 0x098C),
    (0x098F, 0x0990),
    (0x0993, 0x09A8),
    (0x09AA, 0x09B0),
    (0x09B2, 0x09B2),
    (0x09B6, 0x09B9),
    (0x09DC, 0x09DD),
    (0x09DF, 0x09E1),
    (0x09F0, 0x09F1),
    (0x0A05, 0x0A0A),
    (0x0A0F, 0x0A10),
    (0x0A13, 0x0A28),
    (0x0A2A, 0x0A30),
    (0x0A32, 0x0A33),
    (0x0A35, 0x0A36),
    (0x0A38, 0x0A39),
    (0x0A59, 0x0A5C),
    (0x0A5E, 0x0A5E),
    (0x0A72, 0x0A74),
    (0x0A85, 0x0A8B),
    (0x0A8D, 0x0A8D),
    (0x0A8F, 0x0A91),
    (0x0A93, 0x0AA8),
    (0x0AAA, 0x0AB0),
    (0x0AB2, 0x0AB3),
    (0x0AB5, 0x0AB9),
    (0x0ABD, 0x0ABD),
    (0x0AE0, 0x0AE0),
    (0x0B05, 0x0B0C),
    (0x0B0F, 0x0B10),
    (0x0B13, 0x0B28),
    (0x0B2A, 0x0B30),
    (0x0B32, 0x0B33),
    (0x0B36, 0x0B39),
    (0x0B3D, 0x0B3D),
    (0x0B5C, 0x0B5D),
    (0x0B5F, 0x0B61),
    (0x0B85, 0x0B8A),
    (0x0B8E, 0x0B90),
    (0x0B92, 0x0B95),
    (0x0B99, 0x0B9A),
    (0x0B9C, 0x0B9C),
    (0x0B9E, 0x0B9F),
    (0x0BA3, 0x0BA4),
    (0x0BA8, 0x0BAA),
    (0x0BAE, 0x0BB5),
    (0x0BB7, 0x0BB9),
    (0x0C05, 0x0C0C),
    (0x0C0E, 0x0C10),
    (0x0C12, 0x0C28),
    (0x0C2A, 0x0C33),
    (0x0C35, 0x0C39),
    (0x0C60, 0x0C61),
    (0x0C85, 0x0C8C),
    (0x0C8E, 0x0C90),
    (0x0C92, 0x0CA8),
    (0x0CAA, 0x0CB3),
    (0x0CB5, 0x0CB9),
    (0x0CDE, 0x0CDE),
    (0x0CE0, 0x0CE1),
    (0x0D05, 0x0D0C),
    (0x0D0E, 0x0D10),
    (0x0D12, 0x0D28),
    (0x0D2A, 0x0D39),
    (0x0D60, 0x0D61),
    (0x0E01, 0x0E2E),
    (0x0E30, 0x0E30),
    (0x0E32, 0x0E33),
    (0x0E40, 0x0E45),
    (0x0E81, 0x0E82),
    (0x0E84, 0x0E84),
    (0x0E87, 0x0E88),
    (0x0E8A, 0x0E8A),
    (0x0E8D, 0x0E8D),
    (0x0E94, 0x0E97),
    (0x0E99, 0x0E9F),
    (0x0EA1, 0x0EA3),
    (0x0EA5, 0x0EA5),
    (0x0EA7, 0x0EA7),
    (0x0EAA, 0x0EAB),
    (0x0EAD, 0x0EAE),
    (0x0EB0, 0x0EB0),
    (0x0EB2, 0x0EB3),
    (0x0EBD, 0x0EBD),
    (0x0EC0, 0x0EC4),
    (0x0F40, 0x0F47),
    (0x0F49, 0x0F69),
    (0x10A0, 0x10C5),
    (0x10D0, 0x10F6),
    (0x1100, 0x1100),
    (0x1102, 0x1103),
    (0x1105, 0x1107),
    (0x1109, 0x1109),
    (0x110B, 0x110C),
    (0x110E, 0x1112),
    (0x113C, 0x113C),
    (0x113E, 0x113E),
    (0x1140, 0x1140),
    (0x114C, 0x114C),
    (0x114E, 0x114E),
    (0x1150, 0x1150),
    (0x1154, 0x1155),
    (0x1159, 0x1159),
    (0x115F, 0x1161),
    (0x1163, 0x1163),
    (0x1165, 0x1165),
    (0x1167, 0x1167),
    (0x1169, 0x1169),
    (0x116D, 0x116E),
    (0x1172, 0x1173),
    (0x1175, 0x1175),
    (0x119E, 0x119E),
    (0x11A8, 0x11A8),
    (0x11AB, 0x11AB),
    (0x11AE, 0x11AF),
    (0x11B7, 0x11B8),
    (0x11BA, 0x11BA),
    (0x11BC, 0x11C2),
    (0x11EB, 0x11EB),
    (0x11F0, 0x11F0),
    (0x11F9, 0x11F9),
    (0x1E00, 0x1E9B),
    (0x1EA0, 0x1EF9),
    (0x1F00, 0x1F15),
    (0x1F18, 0x1F1D),
    (0x1F20, 0x1F45),
    (0x1F48, 0x1F4D),
    (0x1F50, 0x1F57),
    (0x1F59, 0x1F59),
    (0x1F5B, 0x1F5B),
    (0x1F5D, 0x1F5D),
    (0x1F5F, 0x1F7D),
    (0x1F80, 0x1FB4),
    (0x1FB6, 0x1FBC),
    (0x1FBE, 0x1FBE),
    (0x1FC2, 0x1FC4),
    (0x1FC6, 0x1FCC),
    (0x1FD0, 0x1FD3),
    (0x1FD6, 0x1FDB),
    (0x1FE0, 0x1FEC),
    (0x1FF2, 0x1FF4),
    (0x1FF6, 0x1FFC),
    (0x2126, 0x2126),
    (0x212A, 0x212B),
    (0x212E, 0x212E),
    (0x2180, 0x2182),
    (0x3041, 0x3094),
    (0x30A1, 0x30FA),
    (0x3105, 0x312C),
    (0xAC00, 0xD7A3),
];

const XML10_2E_IDEOGRAPHIC_RANGES: &[(u32, u32)] = &[
    (0x4E00, 0x9FA5),
    (0x3007, 0x3007),
    (0x3021, 0x3029),
];

const XML10_2E_DIGIT_RANGES: &[(u32, u32)] = &[
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
];

const XML10_2E_COMBINING_CHAR_RANGES: &[(u32, u32)] = &[
    (0x0300, 0x0345),
    (0x0360, 0x0361),
    (0x0483, 0x0486),
    (0x0591, 0x05A1),
    (0x05A3, 0x05B9),
    (0x05BB, 0x05BD),
    (0x05BF, 0x05BF),
    (0x05C1, 0x05C2),
    (0x05C4, 0x05C4),
    (0x064B, 0x0652),
    (0x0670, 0x0670),
    (0x06D6, 0x06DC),
    (0x06DD, 0x06DF),
    (0x06E0, 0x06E4),
    (0x06E7, 0x06E8),
    (0x06EA, 0x06ED),
    (0x0901, 0x0903),
    (0x093C, 0x093C),
    (0x093E, 0x094C),
    (0x094D, 0x094D),
    (0x0951, 0x0954),
    (0x0962, 0x0963),
    (0x0981, 0x0983),
    (0x09BC, 0x09BC),
    (0x09BE, 0x09BE),
    (0x09BF, 0x09BF),
    (0x09C0, 0x09C4),
    (0x09C7, 0x09C8),
    (0x09CB, 0x09CD),
    (0x09D7, 0x09D7),
    (0x09E2, 0x09E3),
    (0x0A02, 0x0A02),
    (0x0A3C, 0x0A3C),
    (0x0A3E, 0x0A3E),
    (0x0A3F, 0x0A3F),
    (0x0A40, 0x0A42),
    (0x0A47, 0x0A48),
    (0x0A4B, 0x0A4D),
    (0x0A70, 0x0A71),
    (0x0A81, 0x0A83),
    (0x0ABC, 0x0ABC),
    (0x0ABE, 0x0AC5),
    (0x0AC7, 0x0AC9),
    (0x0ACB, 0x0ACD),
    (0x0B01, 0x0B03),
    (0x0B3C, 0x0B3C),
    (0x0B3E, 0x0B43),
    (0x0B47, 0x0B48),
    (0x0B4B, 0x0B4D),
    (0x0B56, 0x0B57),
    (0x0B82, 0x0B83),
    (0x0BBE, 0x0BC2),
    (0x0BC6, 0x0BC8),
    (0x0BCA, 0x0BCD),
    (0x0BD7, 0x0BD7),
    (0x0C01, 0x0C03),
    (0x0C3E, 0x0C44),
    (0x0C46, 0x0C48),
    (0x0C4A, 0x0C4D),
    (0x0C55, 0x0C56),
    (0x0C82, 0x0C83),
    (0x0CBE, 0x0CC4),
    (0x0CC6, 0x0CC8),
    (0x0CCA, 0x0CCD),
    (0x0CD5, 0x0CD6),
    (0x0D02, 0x0D03),
    (0x0D3E, 0x0D43),
    (0x0D46, 0x0D48),
    (0x0D4A, 0x0D4D),
    (0x0D57, 0x0D57),
    (0x0E31, 0x0E31),
    (0x0E34, 0x0E3A),
    (0x0E47, 0x0E4E),
    (0x0EB1, 0x0EB1),
    (0x0EB4, 0x0EB9),
    (0x0EBB, 0x0EBC),
    (0x0EC8, 0x0ECD),
    (0x0F18, 0x0F19),
    (0x0F35, 0x0F35),
    (0x0F37, 0x0F37),
    (0x0F39, 0x0F39),
    (0x0F3E, 0x0F3E),
    (0x0F3F, 0x0F3F),
    (0x0F71, 0x0F84),
    (0x0F86, 0x0F8B),
    (0x0F90, 0x0F95),
    (0x0F97, 0x0F97),
    (0x0F99, 0x0FAD),
    (0x0FB1, 0x0FB7),
    (0x0FB9, 0x0FB9),
    (0x20D0, 0x20DC),
    (0x20E1, 0x20E1),
    (0x302A, 0x302F),
    (0x3099, 0x3099),
    (0x309A, 0x309A),
];

const XML10_2E_EXTENDER_RANGES: &[(u32, u32)] = &[
    (0x00B7, 0x00B7),
    (0x02D0, 0x02D0),
    (0x02D1, 0x02D1),
    (0x0387, 0x0387),
    (0x0640, 0x0640),
    (0x0E46, 0x0E46),
    (0x0EC6, 0x0EC6),
    (0x3005, 0x3005),
    (0x3031, 0x3035),
    (0x309D, 0x309E),
    (0x30FC, 0x30FE),
];

fn char_in_ranges(ch: char, ranges: &[(u32, u32)]) -> bool {
    let scalar = u32::from(ch);
    ranges
        .iter()
        .any(|&(start, end)| (start..=end).contains(&scalar))
}

fn is_xml10_second_edition_letter(ch: char) -> bool {
    char_in_ranges(ch, XML10_2E_BASE_CHAR_RANGES)
        || char_in_ranges(ch, XML10_2E_IDEOGRAPHIC_RANGES)
}

pub(crate) fn is_xml10_second_edition_name_char(ch: char) -> bool {
    is_xml10_second_edition_letter(ch)
        || char_in_ranges(ch, XML10_2E_DIGIT_RANGES)
        || matches!(ch, '.' | '-' | '_' | ':')
        || char_in_ranges(ch, XML10_2E_COMBINING_CHAR_RANGES)
        || char_in_ranges(ch, XML10_2E_EXTENDER_RANGES)
}

fn parse_date(normalized: &str) -> Result<DateValue, SimpleTypeError> {
    if !normalized.is_ascii() {
        return Err(invalid_lexical("date", normalized));
    }

    let (date_lexical, timezone_minutes) = split_date_timezone(normalized)?;
    let (negative_year, date_body) = match date_lexical.strip_prefix('-') {
        Some(body) => (true, body),
        None => (false, date_lexical),
    };

    let mut parts = date_body.split('-');
    let year_lexical = parts.next().unwrap_or_default();
    let month_lexical = parts.next().unwrap_or_default();
    let day_lexical = parts.next().unwrap_or_default();
    if parts.next().is_some()
        || year_lexical.len() < 4
        || !year_lexical.bytes().all(|byte| byte.is_ascii_digit())
        || (year_lexical.len() > 4 && year_lexical.starts_with('0'))
        || month_lexical.len() != 2
        || !month_lexical.bytes().all(|byte| byte.is_ascii_digit())
        || day_lexical.len() != 2
        || !day_lexical.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(invalid_lexical("date", normalized));
    }

    let year_digits = normalize_unsigned_digits(year_lexical);
    if year_digits == "0" {
        return Err(invalid_lexical("date", normalized));
    }

    let month = parse_two_ascii_digits(month_lexical);
    let day = parse_two_ascii_digits(day_lexical);
    if !(1..=12).contains(&month)
        || day == 0
        || day > days_in_xsd10_month(&year_digits, month)
    {
        return Err(invalid_lexical("date", normalized));
    }

    let mut value = DateValue {
        negative_year,
        year_digits,
        month,
        day,
        timezone_minutes,
    };

    match value.timezone_minutes {
        Some(minutes) if minutes > 12 * 60 => {
            value.timezone_minutes = Some(minutes - 24 * 60);
            decrement_xsd10_date(&mut value);
        }
        Some(minutes) if minutes < -(11 * 60 + 59) => {
            value.timezone_minutes = Some(minutes + 24 * 60);
            increment_xsd10_date(&mut value);
        }
        _ => {}
    }

    Ok(value)
}

fn split_date_timezone(normalized: &str) -> Result<(&str, Option<i16>), SimpleTypeError> {
    if let Some(date_lexical) = normalized.strip_suffix('Z') {
        if date_lexical.is_empty() {
            return Err(invalid_lexical("date", normalized));
        }
        return Ok((date_lexical, Some(0)));
    }

    if normalized.len() >= 6 {
        let start = normalized.len() - 6;
        let suffix = &normalized[start..];
        let bytes = suffix.as_bytes();
        if matches!(bytes[0], b'+' | b'-')
            && bytes[1].is_ascii_digit()
            && bytes[2].is_ascii_digit()
            && bytes[3] == b':'
            && bytes[4].is_ascii_digit()
            && bytes[5].is_ascii_digit()
        {
            let hour = i16::from((bytes[1] - b'0') * 10 + (bytes[2] - b'0'));
            let minute = i16::from((bytes[4] - b'0') * 10 + (bytes[5] - b'0'));
            if hour > 14 || minute > 59 || (hour == 14 && minute != 0) {
                return Err(invalid_lexical("date", normalized));
            }
            let magnitude = hour * 60 + minute;
            let signed = if bytes[0] == b'-' {
                -magnitude
            } else {
                magnitude
            };
            let date_lexical = &normalized[..start];
            if date_lexical.is_empty() {
                return Err(invalid_lexical("date", normalized));
            }
            return Ok((date_lexical, Some(signed)));
        }
    }

    Ok((normalized, None))
}

fn parse_two_ascii_digits(digits: &str) -> u8 {
    let bytes = digits.as_bytes();
    (bytes[0] - b'0') * 10 + (bytes[1] - b'0')
}

fn days_in_xsd10_month(year_digits: &str, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_xsd10_leap_year(year_digits) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_xsd10_leap_year(year_digits: &str) -> bool {
    decimal_digits_divisible_by(year_digits, 400)
        || (decimal_digits_divisible_by(year_digits, 4)
            && !decimal_digits_divisible_by(year_digits, 100))
}

fn decimal_digits_divisible_by(digits: &str, divisor: u16) -> bool {
    let remainder = digits.bytes().fold(0_u16, |remainder, byte| {
        (remainder * 10 + u16::from(byte - b'0')) % divisor
    });
    remainder == 0
}

fn increment_xsd10_date(value: &mut DateValue) {
    let month_days = days_in_xsd10_month(&value.year_digits, value.month);
    if value.day < month_days {
        value.day += 1;
        return;
    }

    value.day = 1;
    if value.month < 12 {
        value.month += 1;
        return;
    }

    value.month = 1;
    increment_xsd10_year(&mut value.negative_year, &mut value.year_digits);
}

fn decrement_xsd10_date(value: &mut DateValue) {
    if value.day > 1 {
        value.day -= 1;
        return;
    }

    if value.month > 1 {
        value.month -= 1;
        value.day = days_in_xsd10_month(&value.year_digits, value.month);
        return;
    }

    value.month = 12;
    decrement_xsd10_year(&mut value.negative_year, &mut value.year_digits);
    value.day = 31;
}

fn increment_xsd10_year(negative: &mut bool, digits: &mut String) {
    if *negative {
        if digits == "1" {
            *negative = false;
        } else {
            *digits = decrement_unsigned_decimal_digits(digits);
        }
    } else {
        *digits = increment_unsigned_decimal_digits(digits);
    }
}

fn decrement_xsd10_year(negative: &mut bool, digits: &mut String) {
    if *negative {
        *digits = increment_unsigned_decimal_digits(digits);
    } else if digits == "1" {
        *negative = true;
    } else {
        *digits = decrement_unsigned_decimal_digits(digits);
    }
}

fn increment_unsigned_decimal_digits(digits: &str) -> String {
    let mut bytes = digits.as_bytes().to_vec();
    for index in (0..bytes.len()).rev() {
        if bytes[index] < b'9' {
            bytes[index] += 1;
            return String::from_utf8(bytes)
                .expect("incremented ASCII decimal digits must remain UTF-8");
        }
        bytes[index] = b'0';
    }
    bytes.insert(0, b'1');
    String::from_utf8(bytes).expect("incremented ASCII decimal digits must remain UTF-8")
}

fn decrement_unsigned_decimal_digits(digits: &str) -> String {
    let mut bytes = digits.as_bytes().to_vec();
    for index in (0..bytes.len()).rev() {
        if bytes[index] > b'0' {
            bytes[index] -= 1;
            break;
        }
        bytes[index] = b'9';
    }
    if bytes.len() > 1 && bytes[0] == b'0' {
        bytes.remove(0);
    }
    String::from_utf8(bytes).expect("decremented ASCII decimal digits must remain UTF-8")
}

// XML Schema 1.0 Second Edition defines xs:anyURI as a URI Reference under
// RFC 2396, amended by RFC 2732, after the escaping operation from XLink 1.0
// section 5.4. Keep this deliberately historical: a current RFC 3986 parser
// would silently change the datatype boundary.
fn parse_any_uri(normalized: &str) -> Result<String, SimpleTypeError> {
    let escaped = xlink_54_escape_for_any_uri(normalized);
    if !is_rfc2396_uri_reference(&escaped) {
        return Err(invalid_lexical("anyURI", normalized));
    }
    Ok(normalized.to_owned())
}

fn xlink_54_escape_for_any_uri(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for ch in value.chars() {
        if ch.is_ascii() {
            let byte = ch as u8;
            if is_xlink_54_disallowed_ascii(byte) {
                push_percent_encoded_byte(&mut escaped, byte);
            } else {
                escaped.push(ch);
            }
        } else {
            let mut buffer = [0_u8; 4];
            for byte in ch.encode_utf8(&mut buffer).bytes() {
                push_percent_encoded_byte(&mut escaped, byte);
            }
        }
    }

    escaped
}

fn is_xlink_54_disallowed_ascii(byte: u8) -> bool {
    matches!(
        byte,
        0x00..=0x20
            | 0x7F
            | b'<'
            | b'>'
            | b'"'
            | b'{'
            | b'}'
            | b'|'
            | b'\\'
            | b'^'
            | b'`'
    )
}

fn push_percent_encoded_byte(output: &mut String, byte: u8) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    output.push('%');
    output.push(char::from(HEX[usize::from(byte >> 4)]));
    output.push(char::from(HEX[usize::from(byte & 0x0F)]));
}

fn is_rfc2396_uri_reference(escaped: &str) -> bool {
    debug_assert!(escaped.is_ascii());

    let Some((main, fragment)) = split_uri_fragment(escaped) else {
        return false;
    };
    if !uri_tokens_all(fragment, is_rfc2396_uric_byte) {
        return false;
    }

    if main.is_empty() {
        return true;
    }

    is_rfc2396_absolute_uri(main) || is_rfc2396_relative_uri(main)
}

fn split_uri_fragment(uri: &str) -> Option<(&str, &str)> {
    match uri.split_once('#') {
        Some((main, fragment)) if !fragment.contains('#') => Some((main, fragment)),
        Some(_) => None,
        None => Some((uri, "")),
    }
}

fn is_rfc2396_absolute_uri(uri: &str) -> bool {
    let Some((scheme, rest)) = uri.split_once(':') else {
        return false;
    };
    if !is_rfc2396_scheme(scheme) || rest.is_empty() {
        return false;
    }

    if rest.starts_with("//") || rest.starts_with('/') {
        is_rfc2396_hier_part(rest)
    } else {
        is_rfc2396_opaque_part(rest)
    }
}

fn is_rfc2396_scheme(scheme: &str) -> bool {
    let mut bytes = scheme.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    first.is_ascii_alphabetic()
        && bytes.all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.')
        })
}

fn is_rfc2396_hier_part(hier: &str) -> bool {
    let (path, query) = split_uri_query(hier);
    if !uri_tokens_all(query, is_rfc2396_uric_byte) {
        return false;
    }

    if path.starts_with("//") {
        is_rfc2396_net_path(path)
    } else {
        is_rfc2396_abs_path(path)
    }
}

fn is_rfc2396_opaque_part(opaque: &str) -> bool {
    let mut index = 0;
    if !consume_uri_token(opaque.as_bytes(), &mut index, is_rfc2396_uric_no_slash_byte) {
        return false;
    }
    uri_tokens_all(&opaque[index..], is_rfc2396_uric_byte)
}

fn is_rfc2396_relative_uri(uri: &str) -> bool {
    let (path, query) = split_uri_query(uri);
    if !uri_tokens_all(query, is_rfc2396_uric_byte) {
        return false;
    }

    if path.starts_with("//") {
        is_rfc2396_net_path(path)
    } else if path.starts_with('/') {
        is_rfc2396_abs_path(path)
    } else {
        is_rfc2396_rel_path(path)
    }
}

fn split_uri_query(uri: &str) -> (&str, &str) {
    uri.split_once('?').unwrap_or((uri, ""))
}

fn is_rfc2396_net_path(path: &str) -> bool {
    let Some(after_slashes) = path.strip_prefix("//") else {
        return false;
    };

    let (authority, abs_path) = match after_slashes.find('/') {
        Some(index) => (&after_slashes[..index], &after_slashes[index..]),
        None => (after_slashes, ""),
    };

    is_rfc2396_authority(authority)
        && (abs_path.is_empty() || is_rfc2396_abs_path(abs_path))
}

fn is_rfc2396_authority(authority: &str) -> bool {
    // RFC 2396's server production permits all components to be absent, so
    // the empty authority in the literal network-path reference "//" is valid.
    if authority.is_empty() {
        return true;
    }

    // Most server spellings are also legal reg_name spellings. RFC 2732's
    // bracketed IPv6 literal is the important server-only form.
    uri_tokens_all(authority, is_rfc2396_reg_name_byte)
        || is_rfc2732_bracketed_server(authority)
}

fn is_rfc2732_bracketed_server(authority: &str) -> bool {
    let (userinfo, hostport) = match authority.split_once('@') {
        Some((userinfo, hostport)) if !hostport.contains('@') => (Some(userinfo), hostport),
        Some(_) => return false,
        None => (None, authority),
    };

    if let Some(userinfo) = userinfo
        && !uri_tokens_all(userinfo, is_rfc2396_userinfo_byte)
    {
        return false;
    }

    let Some(close) = hostport.find(']') else {
        return false;
    };
    if !hostport.starts_with('[') {
        return false;
    }

    let ipv6 = &hostport[1..close];
    if !is_rfc2732_ipv6_address(ipv6) {
        return false;
    }

    let tail = &hostport[close + 1..];
    tail.is_empty()
        || tail
            .strip_prefix(':')
            .is_some_and(|port| port.bytes().all(|byte| byte.is_ascii_digit()))
}

fn is_rfc2732_ipv6_address(address: &str) -> bool {
    if address.is_empty() || address.matches("::").count() > 1 {
        return false;
    }

    let compressed = address.contains("::");
    let (left, right) = match address.split_once("::") {
        Some((left, right)) => (left, right),
        None => (address, ""),
    };

    let mut units = 0_usize;
    if !consume_ipv6_side(left, !compressed, &mut units) {
        return false;
    }
    if compressed && !consume_ipv6_side(right, true, &mut units) {
        return false;
    }

    if compressed {
        units < 8
    } else {
        units == 8
    }
}

fn consume_ipv6_side(side: &str, allow_ipv4_tail: bool, units: &mut usize) -> bool {
    if side.is_empty() {
        return true;
    }

    let parts: Vec<&str> = side.split(':').collect();
    for (index, part) in parts.iter().enumerate() {
        if part.is_empty() {
            return false;
        }

        let final_part = index + 1 == parts.len();
        if part.contains('.') {
            if !allow_ipv4_tail || !final_part || !is_rfc2732_ipv4_address(part) {
                return false;
            }
            *units += 2;
        } else {
            if part.len() > 4 || !part.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                return false;
            }
            *units += 1;
        }

        if *units > 8 {
            return false;
        }
    }

    true
}

fn is_rfc2732_ipv4_address(address: &str) -> bool {
    let parts: Vec<&str> = address.split('.').collect();
    parts.len() == 4
        && parts.iter().all(|part| {
            !part.is_empty()
                && part.len() <= 3
                && part.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn is_rfc2396_abs_path(path: &str) -> bool {
    let Some(segments) = path.strip_prefix('/') else {
        return false;
    };
    is_rfc2396_path_segments(segments)
}

fn is_rfc2396_path_segments(segments: &str) -> bool {
    segments.split('/').all(is_rfc2396_segment)
}

fn is_rfc2396_segment(segment: &str) -> bool {
    segment.split(';').all(|part| uri_tokens_all(part, is_rfc2396_pchar_byte))
}

fn is_rfc2396_rel_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }

    let (first, rest) = match path.find('/') {
        Some(index) => (&path[..index], Some(&path[index..])),
        None => (path, None),
    };

    uri_tokens_all_nonempty(first, is_rfc2396_rel_segment_byte)
        && rest.is_none_or(is_rfc2396_abs_path)
}

fn uri_tokens_all(value: &str, allowed_ascii: fn(u8) -> bool) -> bool {
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if !consume_uri_token(bytes, &mut index, allowed_ascii) {
            return false;
        }
    }
    true
}

fn uri_tokens_all_nonempty(value: &str, allowed_ascii: fn(u8) -> bool) -> bool {
    !value.is_empty() && uri_tokens_all(value, allowed_ascii)
}

fn consume_uri_token(
    bytes: &[u8],
    index: &mut usize,
    allowed_ascii: fn(u8) -> bool,
) -> bool {
    let Some(&byte) = bytes.get(*index) else {
        return false;
    };

    if byte == b'%' {
        let Some((&high, &low)) = bytes.get(*index + 1).zip(bytes.get(*index + 2)) else {
            return false;
        };
        if !high.is_ascii_hexdigit() || !low.is_ascii_hexdigit() {
            return false;
        }
        *index += 3;
        return true;
    }

    if !byte.is_ascii() || !allowed_ascii(byte) {
        return false;
    }
    *index += 1;
    true
}

fn is_rfc2396_unreserved_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(byte, b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')')
}

fn is_rfc2396_reserved_2732_byte(byte: u8) -> bool {
    matches!(
        byte,
        b';' | b'/' | b'?' | b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b',' | b'[' | b']'
    )
}

fn is_rfc2396_uric_byte(byte: u8) -> bool {
    is_rfc2396_unreserved_byte(byte) || is_rfc2396_reserved_2732_byte(byte)
}

fn is_rfc2396_uric_no_slash_byte(byte: u8) -> bool {
    is_rfc2396_unreserved_byte(byte)
        || matches!(byte, b';' | b'?' | b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b',')
}

fn is_rfc2396_pchar_byte(byte: u8) -> bool {
    is_rfc2396_unreserved_byte(byte)
        || matches!(byte, b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b',')
}

fn is_rfc2396_rel_segment_byte(byte: u8) -> bool {
    is_rfc2396_unreserved_byte(byte)
        || matches!(byte, b';' | b'@' | b'&' | b'=' | b'+' | b'$' | b',')
}

fn is_rfc2396_reg_name_byte(byte: u8) -> bool {
    is_rfc2396_unreserved_byte(byte)
        || matches!(byte, b'$' | b',' | b';' | b':' | b'@' | b'&' | b'=' | b'+')
}

fn is_rfc2396_userinfo_byte(byte: u8) -> bool {
    is_rfc2396_unreserved_byte(byte)
        || matches!(byte, b';' | b':' | b'&' | b'=' | b'+' | b'$' | b',')
}

fn parse_language(normalized: &str) -> Result<String, SimpleTypeError> {
    let mut subtags = normalized.split('-');
    let Some(primary) = subtags.next() else {
        return Err(invalid_lexical("language", normalized));
    };
    if !(1..=8).contains(&primary.len())
        || !primary.bytes().all(|byte| byte.is_ascii_alphabetic())
    {
        return Err(invalid_lexical("language", normalized));
    }

    for subtag in subtags {
        if !(1..=8).contains(&subtag.len())
            || !subtag.bytes().all(|byte| byte.is_ascii_alphanumeric())
        {
            return Err(invalid_lexical("language", normalized));
        }
    }

    Ok(normalized.to_owned())
}

fn parse_nmtoken(normalized: &str) -> Result<String, SimpleTypeError> {
    if normalized.is_empty() || !normalized.chars().all(is_xml10_second_edition_name_char) {
        return Err(invalid_lexical("NMTOKEN", normalized));
    }
    Ok(normalized.to_owned())
}

fn parse_ncname_for(
    local_name: &'static str,
    normalized: &str,
) -> Result<String, SimpleTypeError> {
    let mut chars = normalized.chars();
    let Some(first) = chars.next() else {
        return Err(invalid_lexical(local_name, normalized));
    };
    if !(is_xml10_second_edition_letter(first) || first == '_') {
        return Err(invalid_lexical(local_name, normalized));
    }
    if chars.any(|ch| ch == ':' || !is_xml10_second_edition_name_char(ch)) {
        return Err(invalid_lexical(local_name, normalized));
    }
    Ok(normalized.to_owned())
}

fn parse_decimal(normalized: &str) -> Result<DecimalValue, SimpleTypeError> {
    let (negative, body) = split_optional_sign(normalized, "decimal")?;
    let mut pieces = body.split('.');
    let integer_part = pieces.next().unwrap_or_default();
    let fractional_part = pieces.next();
    if pieces.next().is_some() {
        return Err(invalid_lexical("decimal", normalized));
    }

    let fractional_part = fractional_part.unwrap_or("");

    if integer_part.is_empty() && fractional_part.is_empty() {
        return Err(invalid_lexical("decimal", normalized));
    }
    if !integer_part.bytes().all(|byte| byte.is_ascii_digit())
        || !fractional_part.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(invalid_lexical("decimal", normalized));
    }
    let integer_digits = normalize_unsigned_digits(if integer_part.is_empty() {
        "0"
    } else {
        integer_part
    });
    let fractional_digits = fractional_part.trim_end_matches('0').to_owned();
    let zero = integer_digits == "0" && fractional_digits.is_empty();

    Ok(DecimalValue {
        negative: negative && !zero,
        integer_digits,
        fractional_digits,
    })
}

#[derive(Clone, Copy)]
enum IntegerConstraint {
    Any,
    NonNegative,
    Positive,
}

fn parse_integer(
    normalized: &str,
    constraint: IntegerConstraint,
) -> Result<IntegerValue, SimpleTypeError> {
    let local_name = match constraint {
        IntegerConstraint::Any => "integer",
        IntegerConstraint::NonNegative => "nonNegativeInteger",
        IntegerConstraint::Positive => "positiveInteger",
    };

    let (negative, body) = split_optional_sign(normalized, local_name)?;
    if body.is_empty() || !body.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid_lexical(local_name, normalized));
    }

    let digits = normalize_unsigned_digits(body);
    let zero = digits == "0";

    match constraint {
        IntegerConstraint::Any => {}
        IntegerConstraint::NonNegative if negative && !zero => {
            return Err(SimpleTypeError::ValueConstraintViolation {
                local_name,
                normalized_lexical: normalized.to_owned(),
                constraint: "minInclusive=0",
            });
        }
        IntegerConstraint::Positive if negative => {
            return Err(invalid_lexical(local_name, normalized));
        }
        IntegerConstraint::Positive if zero => {
            return Err(SimpleTypeError::ValueConstraintViolation {
                local_name,
                normalized_lexical: normalized.to_owned(),
                constraint: "minInclusive=1",
            });
        }
        IntegerConstraint::NonNegative | IntegerConstraint::Positive => {}
    }

    Ok(IntegerValue {
        negative: negative && !zero,
        digits,
    })
}

fn split_optional_sign<'a>(
    lexical: &'a str,
    local_name: &'static str,
) -> Result<(bool, &'a str), SimpleTypeError> {
    if let Some(rest) = lexical.strip_prefix('-') {
        if rest.is_empty() {
            return Err(invalid_lexical(local_name, lexical));
        }
        Ok((true, rest))
    } else if let Some(rest) = lexical.strip_prefix('+') {
        if rest.is_empty() {
            return Err(invalid_lexical(local_name, lexical));
        }
        Ok((false, rest))
    } else {
        Ok((false, lexical))
    }
}

fn normalize_unsigned_digits(digits: &str) -> String {
    let trimmed = digits.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_owned()
    } else {
        trimmed.to_owned()
    }
}

fn invalid_lexical(local_name: &'static str, normalized: &str) -> SimpleTypeError {
    SimpleTypeError::InvalidLexicalForm {
        local_name,
        normalized_lexical: normalized.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn string_value(local_name: &str, lexical: &str) -> String {
        match parse_bound_builtin(local_name, lexical).expect("value must parse") {
            AtomicValue::String(value) => value,
            other => panic!("expected string value, got {other:?}"),
        }
    }

    fn decimal_value(lexical: &str) -> DecimalValue {
        match parse_bound_builtin("decimal", lexical).expect("decimal must parse") {
            AtomicValue::Decimal(value) => value,
            other => panic!("expected decimal value, got {other:?}"),
        }
    }

    fn integer_value(local_name: &str, lexical: &str) -> IntegerValue {
        match parse_bound_builtin(local_name, lexical).expect("integer must parse") {
            AtomicValue::Integer(value) => value,
            other => panic!("expected integer value, got {other:?}"),
        }
    }

    fn date_value(lexical: &str) -> DateValue {
        match parse_bound_builtin("date", lexical).expect("date must parse") {
            AtomicValue::Date(value) => value,
            other => panic!("expected date value, got {other:?}"),
        }
    }

    #[test]
    fn string_preserves_xml_whitespace_exactly() {
        assert_eq!(string_value("string", "  a\tb\n c\r"), "  a\tb\n c\r");
    }

    #[test]
    fn token_collapses_only_xml_schema_whitespace_characters() {
        assert_eq!(string_value("token", " \ta\r\n  b  "), "a b");
        assert_eq!(string_value("token", "a\u{00A0}b"), "a\u{00A0}b");
    }

    #[test]
    fn decimal_accepts_xsd_lexical_forms_and_normalizes_value() {
        assert_eq!(decimal_value(" +0012.3400 ").canonical_lexical(), "12.34");
        assert_eq!(decimal_value(".5").canonical_lexical(), "0.5");
        assert_eq!(decimal_value("210.").canonical_lexical(), "210.0");
    }

    #[test]
    fn decimal_value_erases_lexical_precision_and_negative_zero() {
        assert_eq!(decimal_value("2.0"), decimal_value("2.00"));
        assert_eq!(decimal_value("-0.000").canonical_lexical(), "0.0");
    }

    #[test]
    fn decimal_rejects_exponent_and_multiple_decimal_points() {
        for lexical in ["1e3", "1E3", "1.2.3", ".", "+", "-"] {
            assert!(parse_bound_builtin("decimal", lexical).is_err(), "{lexical}");
        }
    }

    #[test]
    fn integer_accepts_unbounded_digit_sequences_without_machine_integer_conversion() {
        let lexical = "+0001234567890123456789012345678901234567890";
        assert_eq!(
            integer_value("integer", lexical).canonical_lexical(),
            "1234567890123456789012345678901234567890"
        );
    }

    #[test]
    fn integer_rejects_decimal_and_exponent_lexical_forms() {
        for lexical in ["1.0", ".0", "1e3", "1E3", ""] {
            assert!(parse_bound_builtin("integer", lexical).is_err(), "{lexical}");
        }
    }

    #[test]
    fn integer_collapses_whitespace_before_lexical_check() {
        assert_eq!(
            integer_value("integer", " \t -00120 \r\n").canonical_lexical(),
            "-120"
        );
    }

    #[test]
    fn non_negative_integer_accepts_signed_zero_but_rejects_negative_nonzero() {
        assert_eq!(
            integer_value("nonNegativeInteger", "-000").canonical_lexical(),
            "0"
        );
        assert!(matches!(
            parse_bound_builtin("nonNegativeInteger", "-1"),
            Err(SimpleTypeError::ValueConstraintViolation { .. })
        ));
    }

    #[test]
    fn positive_integer_accepts_optional_plus_and_rejects_zero_and_negative_sign() {
        assert_eq!(
            integer_value("positiveInteger", "+00042").canonical_lexical(),
            "42"
        );
        assert!(matches!(
            parse_bound_builtin("positiveInteger", "0"),
            Err(SimpleTypeError::ValueConstraintViolation { .. })
        ));
        assert!(matches!(
            parse_bound_builtin("positiveInteger", "-0"),
            Err(SimpleTypeError::InvalidLexicalForm { .. })
        ));
    }

    #[test]
    fn nmtoken_uses_xml_10_second_edition_name_chars_after_collapse() {
        assert_eq!(string_value("NMTOKEN", "  12:part-name.β  "), "12:part-name.β");
        assert_eq!(string_value("NMTOKEN", "\u{0301}"), "\u{0301}");
        assert_eq!(string_value("NMTOKEN", "\u{00B7}"), "\u{00B7}");
    }

    #[test]
    fn nmtoken_rejects_empty_internal_space_and_characters_outside_xml_10_second_edition() {
        for lexical in ["", "a b", "a/b", "\u{9FA6}"] {
            assert!(parse_bound_builtin("NMTOKEN", lexical).is_err(), "{lexical:?}");
        }
    }

    #[test]
    fn ncname_accepts_letter_or_underscore_start_and_namechar_tail() {
        assert_eq!(string_value("NCName", "  _part-12.β  "), "_part-12.β");
        assert_eq!(string_value("NCName", "\u{4E00}9"), "\u{4E00}9");
        assert_eq!(string_value("NCName", "A\u{0301}"), "A\u{0301}");
    }

    #[test]
    fn ncname_rejects_colon_and_non_start_name_chars() {
        for lexical in ["", ":part", "part:name", "9part", "-part", ".part", "\u{0301}part"] {
            assert!(parse_bound_builtin("NCName", lexical).is_err(), "{lexical:?}");
        }
    }

    #[test]
    fn xml_10_second_edition_boundary_is_not_widened_to_later_xml_editions() {
        assert_eq!(string_value("NMTOKEN", "\u{9FA5}"), "\u{9FA5}");
        assert!(parse_bound_builtin("NMTOKEN", "\u{9FA6}").is_err());
        assert_eq!(string_value("NCName", "\u{3007}"), "\u{3007}");
    }

    #[test]
    fn id_uses_ncname_value_space_after_collapse() {
        assert_eq!(string_value("ID", "  _score-12.β  "), "_score-12.β");
        assert_eq!(string_value("ID", "\u{4E00}9"), "\u{4E00}9");
    }

    #[test]
    fn idref_uses_ncname_value_space_after_collapse() {
        assert_eq!(string_value("IDREF", "  target_1  "), "target_1");
        assert_eq!(string_value("IDREF", "\u{9FA5}"), "\u{9FA5}");
    }

    #[test]
    fn id_and_idref_reject_non_ncname_values_without_losing_type_identity() {
        for (local_name, lexical) in [
            ("ID", "9start"),
            ("ID", "part:name"),
            ("IDREF", ""),
            ("IDREF", "\u{9FA6}"),
        ] {
            assert!(matches!(
                parse_bound_builtin(local_name, lexical),
                Err(SimpleTypeError::InvalidLexicalForm {
                    local_name: reported,
                    ..
                }) if reported == local_name
            ));
        }
    }

    #[test]
    fn id_and_idref_collapse_internal_xml_whitespace_to_an_invalid_ncname_space() {
        for local_name in ["ID", "IDREF"] {
            assert!(matches!(
                parse_bound_builtin(local_name, "  left\t right  "),
                Err(SimpleTypeError::InvalidLexicalForm {
                    local_name: reported,
                    normalized_lexical,
                }) if reported == local_name && normalized_lexical == "left right"
            ));
        }
    }

    #[test]
    fn language_accepts_xsd10_second_edition_lexical_space_after_collapse() {
        assert_eq!(string_value("language", "  en  "), "en");
        assert_eq!(string_value("language", "de-DE"), "de-DE");
        assert_eq!(string_value("language", "i-klingon"), "i-klingon");
        assert_eq!(string_value("language", "ABCDEFGH-12345678"), "ABCDEFGH-12345678");
    }

    #[test]
    fn language_rejects_outside_xsd10_second_edition_pattern() {
        for lexical in [
            "",
            "123",
            "abcdefghi",
            "en-",
            "en--US",
            "en_Us",
            "é",
            "en-123456789",
        ] {
            assert!(matches!(
                parse_bound_builtin("language", lexical),
                Err(SimpleTypeError::InvalidLexicalForm {
                    local_name,
                    ..
                }) if local_name == "language"
            ));
        }
    }

    #[test]
    fn language_preserves_case_and_internal_hyphenation_after_collapse() {
        assert_eq!(string_value("language", "\tZH-hant-TW\r\n"), "ZH-hant-TW");
    }

    #[test]
    fn date_accepts_xsd10_lexical_space_after_collapse_without_machine_year_limit() {
        assert_eq!(date_value("  2001-10-26  ").canonical_lexical(), "2001-10-26");
        assert_eq!(
            date_value("12345678901234567890-12-31").canonical_lexical(),
            "12345678901234567890-12-31"
        );
        assert_eq!(date_value("2001-01-01+00:00").canonical_lexical(), "2001-01-01Z");
        assert_eq!(date_value("2001-01-01-00:00").canonical_lexical(), "2001-01-01Z");
    }

    #[test]
    fn date_rejects_year_zero_plus_sign_and_invalid_calendar_fields() {
        for lexical in [
            "0000-01-01",
            "-0000-01-01",
            "+2001-01-01",
            "02001-01-01",
            "2001-1-01",
            "2001-01-1",
            "2001-00-01",
            "2001-13-01",
            "2001-04-31",
            "1900-02-29",
        ] {
            assert!(matches!(
                parse_bound_builtin("date", lexical),
                Err(SimpleTypeError::InvalidLexicalForm {
                    local_name: "date",
                    ..
                })
            ), "{lexical}");
        }
    }

    #[test]
    fn date_uses_xsd10_bce_leap_year_sequence() {
        assert_eq!(date_value("-0004-02-29").canonical_lexical(), "-0004-02-29");
        assert!(parse_bound_builtin("date", "-0003-02-29").is_err());
        assert_eq!(date_value("2000-02-29").canonical_lexical(), "2000-02-29");
        assert!(parse_bound_builtin("date", "2100-02-29").is_err());
    }

    #[test]
    fn date_arbitrary_precision_year_uses_decimal_divisibility_for_leap_days() {
        assert_eq!(
            date_value("100000000000000000000000000000000000000000-02-29")
                .canonical_lexical(),
            "100000000000000000000000000000000000000000-02-29"
        );
        assert!(
            parse_bound_builtin(
                "date",
                "100000000000000000000000000000000000000001-02-29"
            )
            .is_err()
        );
    }

    #[test]
    fn date_timezone_lexical_bounds_are_exact() {
        assert_eq!(
            date_value("2001-01-01+14:00").canonical_lexical(),
            "2000-12-31-10:00"
        );
        assert_eq!(
            date_value("2001-01-01-14:00").canonical_lexical(),
            "2001-01-02+10:00"
        );
        for lexical in [
            "2001-01-01+14:01",
            "2001-01-01-14:01",
            "2001-01-01+15:00",
            "2001-01-01+5:00",
        ] {
            assert!(parse_bound_builtin("date", lexical).is_err(), "{lexical}");
        }
    }

    #[test]
    fn date_normalizes_to_the_xsd10_recoverable_timezone_value() {
        assert_eq!(
            date_value("2002-10-10+13:00"),
            date_value("2002-10-09-11:00")
        );
        assert_eq!(
            date_value("2002-10-10-12:00"),
            date_value("2002-10-11+12:00")
        );
        assert_eq!(
            date_value("2002-10-10+12:01").canonical_lexical(),
            "2002-10-09-11:59"
        );
    }

    #[test]
    fn date_timezone_normalization_crosses_the_xsd10_no_year_zero_boundary() {
        assert_eq!(
            date_value("0001-01-01+14:00").canonical_lexical(),
            "-0001-12-31-10:00"
        );
        assert_eq!(
            date_value("-0001-12-31-14:00").canonical_lexical(),
            "0001-01-01+10:00"
        );
    }

    #[test]
    fn any_uri_accepts_empty_relative_absolute_and_fragment_references_after_collapse() {
        for lexical in [
            "",
            "relative/path",
            "/absolute/path",
            "http://example.com/a?b=c#d",
            "#fragment",
        ] {
            assert_eq!(string_value("anyURI", lexical), lexical);
        }
        assert_eq!(string_value("anyURI", "  relative/path  "), "relative/path");
    }

    #[test]
    fn any_uri_xlink_escaping_allows_spaces_and_excluded_ascii_without_rewriting_value() {
        assert_eq!(string_value("anyURI", "a b"), "a b");
        assert_eq!(string_value("anyURI", "urn:test:<value>"), "urn:test:<value>");
        assert_eq!(string_value("anyURI", "left\t right"), "left right");
    }

    #[test]
    fn any_uri_xlink_escaping_uses_utf8_for_non_ascii_characters() {
        let lexical = "urn:example:Grüße";
        assert_eq!(string_value("anyURI", lexical), lexical);
        assert_eq!(
            xlink_54_escape_for_any_uri(lexical),
            "urn:example:Gr%C3%BC%C3%9Fe"
        );
    }

    #[test]
    fn any_uri_accepts_valid_percent_escapes_and_rfc2396_unreserved_marks() {
        for lexical in [
            "a%20b",
            "urn:x:!~*'()",
            "mailto:user@example.com",
            "../score.xml",
        ] {
            assert_eq!(string_value("anyURI", lexical), lexical);
        }
    }

    #[test]
    fn any_uri_rejects_malformed_percent_escapes_and_colon_edge_forms() {
        for lexical in ["%", "%0", "%GG", ":a", "b:"] {
            assert!(matches!(
                parse_bound_builtin("anyURI", lexical),
                Err(SimpleTypeError::InvalidLexicalForm {
                    local_name: "anyURI",
                    ..
                })
            ), "{lexical:?}");
        }
    }

    #[test]
    fn any_uri_accepts_rfc2732_bracketed_ipv6_literals() {
        for lexical in [
            "http://[2001:db8::1]/score.xml",
            "http://[::1]/",
            "http://[::FFFF:129.144.52.38]:80/",
            "http://user@[2001:db8:0:0:0:0:2:1]:/",
        ] {
            assert_eq!(string_value("anyURI", lexical), lexical);
        }
    }

    #[test]
    fn any_uri_rejects_malformed_rfc2732_ipv6_literals() {
        for lexical in [
            "http://[::1",
            "http://[gggg::1]/",
            "http://[1:2:3:4:5:6:7:8:9]/",
            "http://[1::2::3]/",
            "http://[192.0.2.1::]/",
            "http://[::ffff:1234.1.1.1]/",
        ] {
            assert!(parse_bound_builtin("anyURI", lexical).is_err(), "{lexical:?}");
        }
    }

    #[test]
    fn any_uri_literal_rfc2396_grammar_accepts_empty_server_network_path() {
        assert_eq!(string_value("anyURI", "//"), "//");
        assert_eq!(string_value("anyURI", "///"), "///");
    }

    #[test]
    fn any_uri_query_and_fragment_follow_rfc2396_uri_reference_grammar() {
        assert_eq!(string_value("anyURI", "a?x/y?z"), "a?x/y?z");
        assert_eq!(string_value("anyURI", "a#x/y?z"), "a#x/y?z");
        assert!(parse_bound_builtin("anyURI", "?query").is_err());
        assert!(parse_bound_builtin("anyURI", "a#one#two").is_err());
        assert!(parse_bound_builtin("anyURI", "a#%GG").is_err());
    }

    #[test]
    fn enumeration_restriction_compares_token_values_after_base_whitespace_collapse() {
        assert_eq!(
            parse_bound_enumeration_restriction("token", "  start\t", &["start", "stop"])
                .expect("collapsed token must match"),
            AtomicValue::String("start".to_owned())
        );
    }

    #[test]
    fn enumeration_restriction_preserves_string_value_space_instead_of_raw_trimming() {
        assert!(matches!(
            parse_bound_enumeration_restriction("string", " value ", &["value"]),
            Err(SimpleTypeError::EnumerationConstraintViolation { .. })
        ));
        assert!(
            parse_bound_enumeration_restriction("string", " value ", &[" value "]).is_ok()
        );
    }

    #[test]
    fn enumeration_restriction_uses_ncname_and_nmtoken_base_semantics() {
        assert!(
            parse_bound_enumeration_restriction("NCName", "  alpha  ", &["alpha", "beta"])
                .is_ok()
        );
        assert!(
            parse_bound_enumeration_restriction("NMTOKEN", "  1:part  ", &["1:part"])
                .is_ok()
        );
        assert!(parse_bound_enumeration_restriction("NCName", "1:part", &["1:part"]).is_err());
    }

    #[test]
    fn enumeration_restriction_rejects_invalid_facet_values_fail_closed() {
        assert!(matches!(
            parse_bound_enumeration_restriction("NCName", "valid", &["valid", "9invalid"]),
            Err(SimpleTypeError::InvalidEnumerationFacetValue { .. })
        ));
    }

    #[test]
    fn enumeration_restriction_rejects_empty_set_and_unbound_base_profile() {
        assert!(matches!(
            parse_bound_enumeration_restriction("token", "x", &[]),
            Err(SimpleTypeError::EmptyEnumerationFacetSet { .. })
        ));
        assert!(matches!(
            parse_bound_enumeration_restriction("decimal", "1", &["1"]),
            Err(SimpleTypeError::UnsupportedEnumerationBase { .. })
        ));
    }

    #[test]
    fn unknown_builtin_fails_closed_instead_of_falling_back_to_string() {
        assert!(matches!(
            parse_bound_builtin("boolean", "true"),
            Err(SimpleTypeError::UnknownBuiltinType { .. })
        ));
    }


    #[test]
    fn numeric_bounds_compare_decimal_value_space_not_lexical_forms() {
        assert!(parse_bound_numeric_restriction(
            "decimal", "2.00", Some("2"), Some("2.000"), None
        ).is_ok());
        assert!(parse_bound_numeric_restriction(
            "decimal", "-179.999", Some("-180"), Some("180"), None
        ).is_ok());
    }

    #[test]
    fn numeric_bounds_compare_arbitrarily_large_integer_values_without_narrowing() {
        let huge = "1234567890123456789012345678901234567890";
        assert!(parse_bound_numeric_restriction(
            "integer",
            huge,
            Some("999999999999999999999999999999999999999"),
            None,
            None,
        ).is_ok());
    }

    #[test]
    fn numeric_bounds_enforce_inclusive_and_exclusive_edges() {
        assert!(parse_bound_numeric_restriction(
            "decimal", "0", Some("0"), None, None
        ).is_ok());
        assert!(matches!(
            parse_bound_numeric_restriction("decimal", "0", None, None, Some("0")),
            Err(SimpleTypeError::NumericBoundConstraintViolation {
                facet: "minExclusive",
                ..
            })
        ));
        assert!(parse_bound_numeric_restriction(
            "decimal", "0.0001", None, None, Some("0")
        ).is_ok());
        assert!(matches!(
            parse_bound_numeric_restriction("integer", "10", None, Some("9"), None),
            Err(SimpleTypeError::NumericBoundConstraintViolation {
                facet: "maxInclusive",
                ..
            })
        ));
    }

    #[test]
    fn numeric_facet_values_must_belong_to_the_base_value_space() {
        assert!(matches!(
            parse_bound_numeric_restriction("positiveInteger", "2", Some("0"), None, None),
            Err(SimpleTypeError::InvalidNumericFacetValue {
                facet: "minInclusive",
                ..
            })
        ));
    }

    #[test]
    fn numeric_restriction_rejects_non_numeric_bases_fail_closed() {
        assert!(matches!(
            parse_bound_numeric_restriction("token", "2", Some("1"), None, None),
            Err(SimpleTypeError::UnsupportedNumericRestrictionBase { .. })
        ));
    }

    #[test]
    fn implementation_count_matches_declared_r014_builtin_boundary() {
        let implemented = [
            ImplementedBuiltinType::String,
            ImplementedBuiltinType::Token,
            ImplementedBuiltinType::Decimal,
            ImplementedBuiltinType::Integer,
            ImplementedBuiltinType::NonNegativeInteger,
            ImplementedBuiltinType::PositiveInteger,
            ImplementedBuiltinType::NmToken,
            ImplementedBuiltinType::NcName,
            ImplementedBuiltinType::Id,
            ImplementedBuiltinType::IdRef,
            ImplementedBuiltinType::Language,
            ImplementedBuiltinType::Date,
            ImplementedBuiltinType::AnyUri,
        ];
        assert_eq!(implemented.len(), IMPLEMENTED_BUILTIN_TYPE_COUNT);
    }
}
