use schemaschlopf::{
    ConsoleMode, ConsoleProfileClosure, ConsoleRenderOptions, MUSICXML31_SCHEMA_DIR, compile_bound_instance_validator, qualify_bound_complex_type_profile, qualify_bound_direct_numeric_restrictions,
    qualify_bound_enumeration_restrictions, qualify_bound_named_restrictions,
    qualify_bound_remaining_restriction_facets, qualify_bound_union_simple_types,
    qualify_bound_schema_bundle,
    render_qualification_failure, render_qualification_success_with_mode,
};
use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args_os().skip(1);
    let first = args.next();

    let (mode, schema_dir) = match first {
        Some(arg) if arg.as_os_str() == OsStr::new("--details") => (
            ConsoleMode::Details,
            args.next()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(MUSICXML31_SCHEMA_DIR)),
        ),
        Some(arg) => (ConsoleMode::Compact, PathBuf::from(arg)),
        None => (
            ConsoleMode::Compact,
            PathBuf::from(MUSICXML31_SCHEMA_DIR),
        ),
    };

    if args.next().is_some() {
        eprintln!("usage: schemaschlopf [--details] [schema-directory]");
        std::process::exit(2);
    }

    let bundle = match qualify_bound_schema_bundle(&schema_dir) {
        Ok(bundle) => bundle,
        Err(error) => {
            eprint!("{}", render_qualification_failure(&error));
            std::process::exit(1);
        }
    };

    let enumerations = match qualify_bound_enumeration_restrictions(&schema_dir) {
        Ok(qualification) => qualification,
        Err(error) => {
            eprint!("{}", render_qualification_failure(&error));
            std::process::exit(1);
        }
    };

    let numeric_bounds = match qualify_bound_direct_numeric_restrictions(&schema_dir) {
        Ok(qualification) => qualification,
        Err(error) => {
            eprint!("{}", render_qualification_failure(&error));
            std::process::exit(1);
        }
    };

    let named_restrictions = match qualify_bound_named_restrictions(&schema_dir) {
        Ok(qualification) => qualification,
        Err(error) => {
            eprint!("{}", render_qualification_failure(&error));
            std::process::exit(1);
        }
    };

    let remaining_restrictions = match qualify_bound_remaining_restriction_facets(&schema_dir) {
        Ok(qualification) => qualification,
        Err(error) => {
            eprint!("{}", render_qualification_failure(&error));
            std::process::exit(1);
        }
    };

    let unions = match qualify_bound_union_simple_types(&schema_dir) {
        Ok(qualification) => qualification,
        Err(error) => {
            eprint!("{}", render_qualification_failure(&error));
            std::process::exit(1);
        }
    };

    let complex_types = match qualify_bound_complex_type_profile(&schema_dir) {
        Ok(qualification) => qualification,
        Err(error) => {
            eprint!("{}", render_qualification_failure(&error));
            std::process::exit(1);
        }
    };

    let instance_foundation = match compile_bound_instance_validator(&schema_dir) {
        Ok(validator) => validator,
        Err(error) => {
            eprint!("{}", render_qualification_failure(&error));
            std::process::exit(1);
        }
    };

    print!(
        "{}",
        render_qualification_success_with_mode(
            &bundle,
            &enumerations,
            &numeric_bounds,
            &named_restrictions,
            &remaining_restrictions,
            ConsoleProfileClosure::new(&unions, &complex_types, &instance_foundation),
            ConsoleRenderOptions::new(&schema_dir, mode),
        )
    );
}
