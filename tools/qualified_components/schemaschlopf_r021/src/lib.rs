//! SchemaSchlopf.
//!
//! SchemaSchlopf qualifies exact, hash-bound MusicXML 3.1 and 4.0 schema bundles and validates
//! persisted MusicXML instance bytes against the selected complete bound profile. The validator is
//! read-only, offline, deterministic, and fail-closed outside its explicitly qualified profile.

mod console;
mod profile;
mod qualification;
mod simple;
mod simple_profile;
mod numeric_profile;
mod named_profile;
mod restriction_profile;
mod union_profile;
mod complex_profile;
mod instance_profile;
#[cfg(feature = "take")]
mod assistant_export;
#[cfg(feature = "take")]
mod take;

pub use console::{
    ConsoleMode, ConsoleProfileClosure, ConsoleRenderOptions, REVISION, REVISION_NAME, render_qualification_failure,
    render_qualification_success, render_qualification_success_with_mode,
};
pub use profile::{
    BoundSchemaProfile, MUSICXML31_BUNDLE_ID, MUSICXML31_SCHEMA_DIR, MUSICXML31_UPSTREAM_COMMIT,
    MUSICXML31_UPSTREAM_TAG, MUSICXML40_BUNDLE_ID, MUSICXML40_SCHEMA_DIR, XSD_NAMESPACE,
};
#[cfg(feature = "take")]
pub use assistant_export::{AssistantExportSummary, run_assistant_export};
#[cfg(feature = "take")]
pub use take::{run_take_bootstrap, seal_source_manifest};
pub use simple::{
    AtomicValue, BOUND_BUILTIN_TYPE_COUNT, DateValue, DecimalValue, IMPLEMENTED_BUILTIN_TYPE_COUNT,
    ImplementedBuiltinType, IntegerValue, SimpleTypeError, WhiteSpaceMode, apply_white_space,
    parse_bound_builtin, parse_bound_enumeration_restriction, parse_bound_numeric_restriction,
};
pub use simple_profile::{
    BOUND_ENUMERATION_RESTRICTION_COUNT, BOUND_ENUMERATION_VALUE_COUNT,
    EnumerationRestrictionQualification, EnumerationRestrictionQualificationError,
    qualify_bound_enumeration_restrictions,
};
pub use numeric_profile::{
    BOUND_DIRECT_NUMERIC_FACET_COUNT, BOUND_DIRECT_NUMERIC_RESTRICTION_COUNT,
    NumericBoundRestrictionQualification, NumericBoundRestrictionQualificationError,
    qualify_bound_direct_numeric_restrictions,
};
pub use named_profile::{
    BOUND_NAME_ALIAS_RESTRICTION_COUNT, BOUND_NAMED_NUMERIC_FACET_COUNT,
    BOUND_NAMED_NUMERIC_RESTRICTION_COUNT, NamedRestrictionQualification,
    NamedRestrictionQualificationError, NamedRestrictionValueError,
    qualify_bound_named_restrictions,
};
pub use restriction_profile::{
    BOUND_MIN_LENGTH_RESTRICTION_COUNT, BOUND_PATTERN_RESTRICTION_COUNT,
    BOUND_REMAINING_RESTRICTION_FACET_COUNT, RestrictionFacetQualification,
    RestrictionFacetQualificationError, RestrictionFacetValueError,
    qualify_bound_remaining_restriction_facets,
};
pub use union_profile::{
    BOUND_RESTRICTION_SIMPLE_TYPE_COUNT, BOUND_SIMPLE_TYPE_COUNT,
    BOUND_UNION_SIMPLE_TYPE_COUNT, BoundUnionType, UnionSimpleTypeQualification,
    UnionSimpleTypeQualificationError, UnionSimpleTypeValueError, UnionValue,
    qualify_bound_union_simple_types,
};
pub use instance_profile::{
    BoundInstanceValidator, InstanceFoundationError, InstanceFoundationReport,
    compile_bound_instance_validator,
};
pub use complex_profile::{
    BOUND_ANONYMOUS_COMPLEX_TYPE_COUNT, BOUND_ATTRIBUTE_GROUP_REF_COUNT,
    BOUND_ATTRIBUTE_NODE_COUNT, BOUND_CHOICE_COUNT, BOUND_COMPLEX_CONTENT_EXTENSION_COUNT,
    BOUND_COMPLEX_TYPE_COUNT, BOUND_DEFAULT_ATTRIBUTE_COUNT, BOUND_DIRECT_COMPLEX_TYPE_COUNT,
    BOUND_ELEMENT_NODE_COUNT, BOUND_FIXED_ATTRIBUTE_COUNT, BOUND_GLOBAL_ATTRIBUTE_COUNT,
    BOUND_GLOBAL_ATTRIBUTE_GROUP_COUNT, BOUND_GLOBAL_ELEMENT_COUNT, BOUND_GLOBAL_MODEL_GROUP_COUNT,
    BOUND_MODEL_GROUP_REF_COUNT, BOUND_NAMED_COMPLEX_TYPE_COUNT, BOUND_REQUIRED_ATTRIBUTE_USE_COUNT,
    BOUND_SEQUENCE_COUNT, BOUND_SIMPLE_CONTENT_EXTENSION_COUNT, ComplexTypeProfileQualification,
    ComplexTypeProfileQualificationError, qualify_bound_complex_type_profile,
};
pub use qualification::{
    Inventory, QualificationError, QualifiedSchemaBundle, ResolvedQNameReference,
    SchemaDocumentSummary, SymbolKind, qualify_bound_schema_bundle,
    qualify_musicxml31_schema_bundle, qualify_musicxml40_schema_bundle,
};

/// `true` because r020 closed the document-wide persisted-byte instance semantics and r021
/// preserves that closure while adding a second exact bound score-schema profile.
///
/// The claim is limited to MusicXML 3.1 and MusicXML 4.0 bundles whose exact bytes qualify.
/// Unsupported input still fails closed and creates no wider generic-XSD authority.
pub const INSTANCE_VALIDATION_QUALIFIED: bool = true;

/// MusicXML 4.0 uses the same complete persisted-byte instance-validation closure when and only
/// when the exact bound 4.0 score-schema byte bundle qualifies.
pub const MUSICXML40_INSTANCE_VALIDATION_QUALIFIED: bool = true;
