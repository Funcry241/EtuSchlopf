use crate::{
    BOUND_BUILTIN_TYPE_COUNT, EnumerationRestrictionQualification,
    IMPLEMENTED_BUILTIN_TYPE_COUNT, ComplexTypeProfileQualification, NamedRestrictionQualification,
    NumericBoundRestrictionQualification, QualifiedSchemaBundle, RestrictionFacetQualification,
    UnionSimpleTypeQualification, BoundInstanceValidator,
};
use std::fmt::Write as _;
use std::path::Path;

pub const REVISION: &str = "r021";
pub const REVISION_NAME: &str = "MusicXml40ProfileClosure";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleMode {
    Compact,
    Details,
}

#[derive(Debug, Clone, Copy)]
pub struct ConsoleRenderOptions<'a> {
    pub schema_dir: &'a Path,
    pub mode: ConsoleMode,
}

impl<'a> ConsoleRenderOptions<'a> {
    #[must_use]
    pub const fn new(schema_dir: &'a Path, mode: ConsoleMode) -> Self {
        Self { schema_dir, mode }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConsoleProfileClosure<'a> {
    pub unions: &'a UnionSimpleTypeQualification,
    pub complex_types: &'a ComplexTypeProfileQualification,
    pub instance_foundation: &'a BoundInstanceValidator,
}

impl<'a> ConsoleProfileClosure<'a> {
    #[must_use]
    pub const fn new(
        unions: &'a UnionSimpleTypeQualification,
        complex_types: &'a ComplexTypeProfileQualification,
        instance_foundation: &'a BoundInstanceValidator,
    ) -> Self {
        Self {
            unions,
            complex_types,
            instance_foundation,
        }
    }
}

#[must_use]
pub fn render_qualification_success(
    bundle: &QualifiedSchemaBundle,
    enumerations: &EnumerationRestrictionQualification,
    numeric_bounds: &NumericBoundRestrictionQualification,
    named_restrictions: &NamedRestrictionQualification,
    remaining_restrictions: &RestrictionFacetQualification,
    closure: ConsoleProfileClosure<'_>,
    schema_dir: &Path,
) -> String {
    render_qualification_success_with_mode(
        bundle,
        enumerations,
        numeric_bounds,
        named_restrictions,
        remaining_restrictions,
        closure,
        ConsoleRenderOptions::new(schema_dir, ConsoleMode::Compact),
    )
}

#[must_use]
pub fn render_qualification_success_with_mode(
    bundle: &QualifiedSchemaBundle,
    enumerations: &EnumerationRestrictionQualification,
    numeric_bounds: &NumericBoundRestrictionQualification,
    named_restrictions: &NamedRestrictionQualification,
    remaining_restrictions: &RestrictionFacetQualification,
    closure: ConsoleProfileClosure<'_>,
    options: ConsoleRenderOptions<'_>,
) -> String {
    match options.mode {
        ConsoleMode::Compact => {
            render_compact_success(
                bundle,
                enumerations,
                numeric_bounds,
                named_restrictions,
                remaining_restrictions,
                closure,
            )
        }
        ConsoleMode::Details => render_detailed_success(
            bundle,
            enumerations,
            numeric_bounds,
            named_restrictions,
            remaining_restrictions,
            closure,
            options.schema_dir,
        ),
    }
}

fn render_compact_success(
    bundle: &QualifiedSchemaBundle,
    enumerations: &EnumerationRestrictionQualification,
    numeric_bounds: &NumericBoundRestrictionQualification,
    named_restrictions: &NamedRestrictionQualification,
    remaining_restrictions: &RestrictionFacetQualification,
    closure: ConsoleProfileClosure<'_>,
) -> String {
    let inventory = bundle.inventory();
    let mut output = String::new();

    write_header(&mut output);
    let _ = writeln!(output);
    let _ = writeln!(output, "SCHEMA");
    let _ = writeln!(output, "  Profile       {} · hash-bound", bundle.profile().display_name());
    let _ = writeln!(output, "  Bundle        QUALIFIED");
    let _ = writeln!(
        output,
        "  Sources       {0}/{0} exact",
        bundle.documents().len()
    );
    let _ = writeln!(
        output,
        "  Imports       {0}/{0} local",
        inventory.imports
    );
    let _ = writeln!(
        output,
        "  QNames        {0}/{0} resolved",
        inventory.qname_references
    );
    let _ = writeln!(output);
    let _ = writeln!(output, "SIMPLE TYPES");
    let _ = writeln!(
        output,
        "  Built-ins     {IMPLEMENTED_BUILTIN_TYPE_COUNT}/{BOUND_BUILTIN_TYPE_COUNT} implemented"
    );
    let _ = writeln!(
        output,
        "  Enum rules    {}/{} qualified · {} values",
        enumerations.restriction_count(),
        enumerations.restriction_count(),
        enumerations.enumeration_value_count()
    );
    let total_numeric_restrictions =
        numeric_bounds.restriction_count() + named_restrictions.named_numeric_restriction_count();
    let total_numeric_facets =
        numeric_bounds.facet_count() + named_restrictions.named_numeric_facet_count();
    let _ = writeln!(
        output,
        "  Num bounds    {0}/{0} qualified · {1} facets",
        total_numeric_restrictions,
        total_numeric_facets
    );
    let _ = writeln!(
        output,
        "  Named bases   {0}/{0} qualified",
        named_restrictions.alias_count()
    );
    let _ = writeln!(
        output,
        "  Patterns      {0}/{0} qualified",
        remaining_restrictions.pattern_count()
    );
    let _ = writeln!(
        output,
        "  Min length    {0}/{0} qualified",
        remaining_restrictions.min_length_count()
    );
    let _ = writeln!(
        output,
        "  Unions        {0}/{0} qualified",
        closure.unions.union_count()
    );
    let _ = writeln!(
        output,
        "  Profile       QUALIFIED · {0}/{0}",
        closure.unions.simple_type_count()
    );
    let _ = writeln!(output);
    let _ = writeln!(output, "COMPLEX TYPES");
    let _ = writeln!(
        output,
        "  Types         {0}/{0} qualified",
        closure.complex_types.complex_type_count()
    );
    let _ = writeln!(
        output,
        "  Content       {} direct · {} simple ext · {} complex ext",
        closure.complex_types.direct_complex_type_count(),
        closure.complex_types.simple_content_extension_count(),
        closure.complex_types.complex_content_extension_count()
    );
    let _ = writeln!(
        output,
        "  Model groups  {} defs · {} refs",
        closure.complex_types.global_model_group_count(),
        closure.complex_types.model_group_ref_count()
    );
    let _ = writeln!(
        output,
        "  Attr groups   {} defs · {} refs",
        closure.complex_types.global_attribute_group_count(),
        closure.complex_types.attribute_group_ref_count()
    );
    let _ = writeln!(
        output,
        "  Nodes         {} elements · {} attributes",
        closure.complex_types.element_node_count(),
        closure.complex_types.attribute_node_count()
    );
    let _ = writeln!(output, "  Profile       QUALIFIED");
    let _ = writeln!(output);
    let _ = writeln!(output, "INSTANCE");
    let _ = writeln!(
        output,
        "  Foundation    {}",
        if closure.instance_foundation.instance_foundation_qualified() {
            "QUALIFIED · local structure + values"
        } else {
            "NOT QUALIFIED"
        }
    );
    let _ = writeln!(
        output,
        "  Validation    {}",
        if closure.instance_foundation.instance_validation_qualified() {
            "QUALIFIED"
        } else {
            "NOT QUALIFIED"
        }
    );
    let _ = writeln!(output);
    let _ = writeln!(output, "RESULT          INSTANCE VALIDATOR QUALIFIED");
    let _ = writeln!(output, "DETAILS         run.cmd --details");

    output
}

fn render_detailed_success(
    bundle: &QualifiedSchemaBundle,
    enumerations: &EnumerationRestrictionQualification,
    numeric_bounds: &NumericBoundRestrictionQualification,
    named_restrictions: &NamedRestrictionQualification,
    remaining_restrictions: &RestrictionFacetQualification,
    closure: ConsoleProfileClosure<'_>,
    schema_dir: &Path,
) -> String {
    let inventory = bundle.inventory();
    let mut output = String::new();

    write_header(&mut output);
    let _ = writeln!(output);
    let _ = writeln!(output, "SCHEMA");
    let _ = writeln!(output, "  Profile       {} · hash-bound", bundle.profile().display_name());
    let _ = writeln!(output, "  Bundle        QUALIFIED");
    let _ = writeln!(output, "  Bundle ID     {}", bundle.bundle_id());
    let _ = writeln!(output, "  Directory     {}", schema_dir.display());
    let _ = writeln!(
        output,
        "  Sources       {0}/{0} exact",
        bundle.documents().len()
    );
    let _ = writeln!(
        output,
        "  Imports       {0}/{0} local",
        inventory.imports
    );
    let _ = writeln!(
        output,
        "  QNames        {0}/{0} resolved",
        inventory.qname_references
    );
    let _ = writeln!(output);
    let _ = writeln!(output, "SCHEMA INVENTORY");
    let _ = writeln!(output, "  XSD elements  {} kinds · {} nodes",
        inventory.xsd_elements.len(),
        inventory.xsd_elements.values().sum::<usize>());
    let _ = writeln!(output, "  XSD attrs     {} kinds · {} nodes",
        inventory.xsd_attributes.len(),
        inventory.xsd_attributes.values().sum::<usize>());
    let _ = writeln!(
        output,
        "  Symbols       {} global",
        inventory.global_symbols.values().sum::<usize>()
    );
    let _ = writeln!(output);
    let _ = writeln!(output, "SIMPLE TYPES");
    let _ = writeln!(
        output,
        "  Built-ins     {IMPLEMENTED_BUILTIN_TYPE_COUNT}/{BOUND_BUILTIN_TYPE_COUNT} implemented"
    );
    let _ = writeln!(
        output,
        "  Enum rules    {}/{} qualified · {} values",
        enumerations.restriction_count(),
        enumerations.restriction_count(),
        enumerations.enumeration_value_count()
    );
    let total_numeric_restrictions =
        numeric_bounds.restriction_count() + named_restrictions.named_numeric_restriction_count();
    let total_numeric_facets =
        numeric_bounds.facet_count() + named_restrictions.named_numeric_facet_count();
    let _ = writeln!(
        output,
        "  Num bounds    {0}/{0} qualified · {1} facets",
        total_numeric_restrictions,
        total_numeric_facets
    );
    let _ = writeln!(
        output,
        "  Named bases   {0}/{0} qualified",
        named_restrictions.alias_count()
    );
    let _ = writeln!(
        output,
        "  Patterns      {0}/{0} qualified",
        remaining_restrictions.pattern_count()
    );
    let _ = writeln!(
        output,
        "  Min length    {0}/{0} qualified",
        remaining_restrictions.min_length_count()
    );
    let _ = writeln!(
        output,
        "  Unions        {0}/{0} qualified",
        closure.unions.union_count()
    );
    let _ = writeln!(
        output,
        "  Profile       QUALIFIED · {0}/{0}",
        closure.unions.simple_type_count()
    );
    let _ = writeln!(output);
    let _ = writeln!(output, "COMPLEX TYPES");
    let _ = writeln!(
        output,
        "  Types         {0}/{0} qualified",
        closure.complex_types.complex_type_count()
    );
    let _ = writeln!(
        output,
        "  Content       {} direct · {} simple ext · {} complex ext",
        closure.complex_types.direct_complex_type_count(),
        closure.complex_types.simple_content_extension_count(),
        closure.complex_types.complex_content_extension_count()
    );
    let _ = writeln!(
        output,
        "  Model groups  {} defs · {} refs",
        closure.complex_types.global_model_group_count(),
        closure.complex_types.model_group_ref_count()
    );
    let _ = writeln!(
        output,
        "  Attr groups   {} defs · {} refs",
        closure.complex_types.global_attribute_group_count(),
        closure.complex_types.attribute_group_ref_count()
    );
    let _ = writeln!(
        output,
        "  Nodes         {} elements · {} attributes",
        closure.complex_types.element_node_count(),
        closure.complex_types.attribute_node_count()
    );
    let _ = writeln!(output, "  Profile       QUALIFIED");
    let _ = writeln!(output);
    let _ = writeln!(output, "SCHEMA SOURCES");
    for document in bundle.documents() {
        let _ = writeln!(
            output,
            "  {}",
            document.file_name
        );
        let _ = writeln!(
            output,
            "    bytes       {}",
            document.byte_len
        );
        let _ = writeln!(
            output,
            "    sha256      {}",
            document.sha256
        );
        let _ = writeln!(
            output,
            "    namespace   {}",
            document.target_namespace.as_deref().unwrap_or("<none>")
        );
    }
    let _ = writeln!(output);
    let _ = writeln!(output, "INSTANCE");
    let _ = writeln!(
        output,
        "  Foundation    {}",
        if closure.instance_foundation.instance_foundation_qualified() {
            "QUALIFIED · local structure + values"
        } else {
            "NOT QUALIFIED"
        }
    );
    let _ = writeln!(
        output,
        "  Validation    {}",
        if closure.instance_foundation.instance_validation_qualified() {
            "QUALIFIED"
        } else {
            "NOT QUALIFIED"
        }
    );
    let _ = writeln!(output);
    let _ = writeln!(output, "RESULT          INSTANCE VALIDATOR QUALIFIED");

    output
}

fn write_header(output: &mut String) {
    let _ = writeln!(
        output,
        "SchemaSchlopf {} · {REVISION} · {REVISION_NAME}",
        env!("CARGO_PKG_VERSION")
    );
}

#[must_use]
pub fn render_qualification_failure(error: &dyn std::fmt::Display) -> String {
    format!(
        "SchemaSchlopf {} · {REVISION} · {REVISION_NAME}\n\nQUALIFICATION\n  Schema        REJECTED\n  Instance      NOT QUALIFIED\n\nERROR\n  {error}\n\nRESULT          FAIL CLOSED\n",
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MUSICXML31_SCHEMA_DIR, MUSICXML40_SCHEMA_DIR, compile_bound_instance_validator,
        qualify_bound_complex_type_profile, qualify_bound_direct_numeric_restrictions,
        qualify_bound_enumeration_restrictions, qualify_bound_named_restrictions,
        qualify_bound_remaining_restriction_facets, qualify_bound_schema_bundle,
        qualify_bound_union_simple_types, qualify_musicxml31_schema_bundle,
    };

    struct QualifiedFixture {
        bundle: QualifiedSchemaBundle,
        enumerations: EnumerationRestrictionQualification,
        numeric_bounds: NumericBoundRestrictionQualification,
        named_restrictions: NamedRestrictionQualification,
        remaining_restrictions: RestrictionFacetQualification,
        unions: UnionSimpleTypeQualification,
        complex_types: ComplexTypeProfileQualification,
        instance_foundation: BoundInstanceValidator,
        root: std::path::PathBuf,
    }

    fn qualified_bundle() -> QualifiedFixture {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML31_SCHEMA_DIR);
        let bundle = qualify_musicxml31_schema_bundle(&root).expect("bound bundle must qualify");
        let enumerations = qualify_bound_enumeration_restrictions(&root)
            .expect("bound enumeration restrictions must qualify");
        let numeric_bounds = qualify_bound_direct_numeric_restrictions(&root)
            .expect("bound direct numeric restrictions must qualify");
        let named_restrictions = qualify_bound_named_restrictions(&root)
            .expect("bound named restrictions must qualify");
        let remaining_restrictions = qualify_bound_remaining_restriction_facets(&root)
            .expect("bound remaining restrictions must qualify");
        let unions = qualify_bound_union_simple_types(&root)
            .expect("bound union simple types must qualify");
        let complex_types = qualify_bound_complex_type_profile(&root)
            .expect("bound complex-type profile must qualify");
        let instance_foundation = compile_bound_instance_validator(&root)
            .expect("bound instance foundation must compile");
        QualifiedFixture {
            bundle,
            enumerations,
            numeric_bounds,
            named_restrictions,
            remaining_restrictions,
            unions,
            complex_types,
            instance_foundation,
            root,
        }
    }

    #[test]
    fn compact_console_exposes_final_instance_validation_boundary() {
        let fixture = qualified_bundle();
        let output = render_qualification_success(
            &fixture.bundle,
            &fixture.enumerations,
            &fixture.numeric_bounds,
            &fixture.named_restrictions,
            &fixture.remaining_restrictions,
            ConsoleProfileClosure::new(
                &fixture.unions,
                &fixture.complex_types,
                &fixture.instance_foundation,
            ),
            &fixture.root,
        );

        assert!(output.contains("Bundle        QUALIFIED"));
        assert!(output.contains("Foundation    QUALIFIED · local structure + values"));
        assert!(output.contains("Validation    QUALIFIED"));
        assert!(!output.contains("Validation    NOT QUALIFIED"));
        assert!(output.contains("RESULT          INSTANCE VALIDATOR QUALIFIED"));
    }

    #[test]
    fn compact_console_exposes_complete_simple_type_boundary() {
        let fixture = qualified_bundle();
        let output = render_qualification_success(
            &fixture.bundle,
            &fixture.enumerations,
            &fixture.numeric_bounds,
            &fixture.named_restrictions,
            &fixture.remaining_restrictions,
            ConsoleProfileClosure::new(
                &fixture.unions,
                &fixture.complex_types,
                &fixture.instance_foundation,
            ),
            &fixture.root,
        );

        assert!(output.contains("Built-ins     13/13 implemented"));
        assert!(output.contains("Enum rules    97/97 qualified · 604 values"));
        assert!(output.contains("Num bounds    15/15 qualified · 26 facets"));
        assert!(output.contains("Named bases   13/13 qualified"));
        assert!(output.contains("Patterns      10/10 qualified"));
        assert!(output.contains("Min length    1/1 qualified"));
        assert!(output.contains("Unions        5/5 qualified"));
        assert!(output.contains("Profile       QUALIFIED · 141/141"));
        assert!(!output.contains("Profile       NOT QUALIFIED"));
        assert!(output.contains("Types         206/206 qualified"));
        assert!(output.contains("Content       125 direct · 77 simple ext · 4 complex ext"));
        assert!(output.contains("Model groups  24 defs · 60 refs"));
        assert!(output.contains("Attr groups   45 defs · 325 refs"));
    }

    #[test]
    fn compact_console_projects_numeric_bounds_exactly_once() {
        let fixture = qualified_bundle();
        let output = render_qualification_success(
            &fixture.bundle,
            &fixture.enumerations,
            &fixture.numeric_bounds,
            &fixture.named_restrictions,
            &fixture.remaining_restrictions,
            ConsoleProfileClosure::new(
                &fixture.unions,
                &fixture.complex_types,
                &fixture.instance_foundation,
            ),
            &fixture.root,
        );

        assert_eq!(output.matches("Num bounds").count(), 1);
    }

    #[test]
    fn compact_console_omits_low_level_schema_dump() {
        let fixture = qualified_bundle();
        let output = render_qualification_success(
            &fixture.bundle,
            &fixture.enumerations,
            &fixture.numeric_bounds,
            &fixture.named_restrictions,
            &fixture.remaining_restrictions,
            ConsoleProfileClosure::new(
                &fixture.unions,
                &fixture.complex_types,
                &fixture.instance_foundation,
            ),
            &fixture.root,
        );

        assert!(output.contains("DETAILS         run.cmd --details"));
        assert!(!output.contains("SCHEMA SOURCES"));
        assert!(!output.contains("49ae9e34e04ba5b72004687994f70d598323eb3353eadb62faedcc9e9dc4bd7f"));
        assert!(!output.contains("XSD elements"));
    }

    #[test]
    fn details_console_retains_full_schema_provenance() {
        let fixture = qualified_bundle();
        let output =
            render_qualification_success_with_mode(
                &fixture.bundle,
                &fixture.enumerations,
                &fixture.numeric_bounds,
                &fixture.named_restrictions,
                &fixture.remaining_restrictions,
                ConsoleProfileClosure::new(
                &fixture.unions,
                &fixture.complex_types,
                &fixture.instance_foundation,
            ),
                ConsoleRenderOptions::new(&fixture.root, ConsoleMode::Details),
            );

        assert!(output.contains("SCHEMA INVENTORY"));
        assert!(output.contains("SCHEMA SOURCES"));
        assert!(output.contains("Bundle ID"));
        assert!(output.contains("49ae9e34e04ba5b72004687994f70d598323eb3353eadb62faedcc9e9dc4bd7f"));
        assert!(output.contains("1371/1371 resolved"));
    }

    #[test]
    fn compact_console_projects_musicxml40_profile_specific_counts() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(MUSICXML40_SCHEMA_DIR);
        let bundle = qualify_bound_schema_bundle(&root).expect("bound 4.0 bundle must qualify");
        let enumerations = qualify_bound_enumeration_restrictions(&root)
            .expect("bound 4.0 enumerations must qualify");
        let numeric_bounds = qualify_bound_direct_numeric_restrictions(&root)
            .expect("bound 4.0 numeric restrictions must qualify");
        let named_restrictions = qualify_bound_named_restrictions(&root)
            .expect("bound 4.0 named restrictions must qualify");
        let remaining_restrictions = qualify_bound_remaining_restriction_facets(&root)
            .expect("bound 4.0 remaining restrictions must qualify");
        let unions = qualify_bound_union_simple_types(&root)
            .expect("bound 4.0 unions must qualify");
        let complex_types = qualify_bound_complex_type_profile(&root)
            .expect("bound 4.0 complex profile must qualify");
        let instance_foundation = compile_bound_instance_validator(&root)
            .expect("bound 4.0 instance validator must compile");

        let output = render_qualification_success(
            &bundle,
            &enumerations,
            &numeric_bounds,
            &named_restrictions,
            &remaining_restrictions,
            ConsoleProfileClosure::new(&unions, &complex_types, &instance_foundation),
            &root,
        );

        assert!(output.contains("Profile       MusicXML 4.0 · hash-bound"));
        assert!(output.contains("Enum rules    104/104 qualified · 633 values"));
        assert!(output.contains("Num bounds    16/16 qualified · 28 facets"));
        assert!(output.contains("Named bases   16/16 qualified"));
        assert!(output.contains("Patterns      11/11 qualified"));
        assert!(output.contains("Profile       QUALIFIED · 153/153"));
        assert!(output.contains("Types         231/231 qualified"));
    }

    #[test]
    fn failure_console_is_explicitly_fail_closed() {
        let output = render_qualification_failure(&"synthetic qualification failure");

        assert!(output.contains("Schema        REJECTED"));
        assert!(output.contains("Instance      NOT QUALIFIED"));
        assert!(output.contains("RESULT          FAIL CLOSED"));
    }
}
