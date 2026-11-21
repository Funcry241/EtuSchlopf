use etuschlopf::console_presentation_contract::{
    resolve_stream_render_policy, ColorPreference, ConsolePreferences, GlyphCapability, GlyphMode,
    GlyphPreference, ProgressMode, ProgressPreference, StreamCapability, StreamDestination, StyleMode,
};

fn terminal() -> StreamCapability {
    StreamCapability {
        destination: StreamDestination::Terminal,
        glyphs: GlyphCapability::Unicode,
    }
}

fn redirected() -> StreamCapability {
    StreamCapability {
        destination: StreamDestination::Redirected,
        glyphs: GlyphCapability::Unicode,
    }
}

#[test]
fn redirected_output_is_plain_and_line_oriented_even_when_color_is_forced() {
    let policy = resolve_stream_render_policy(
        redirected(),
        ConsolePreferences {
            color: ColorPreference::Always,
            progress: ProgressPreference::Auto,
            glyphs: GlyphPreference::Auto,
        },
    );

    assert_eq!(policy.style, StyleMode::Plain);
    assert_eq!(policy.progress, ProgressMode::Lines);
}

#[test]
fn no_color_preference_keeps_terminal_interactive_but_plain() {
    let policy = resolve_stream_render_policy(
        terminal(),
        ConsolePreferences {
            color: ColorPreference::Never,
            progress: ProgressPreference::Auto,
            glyphs: GlyphPreference::Auto,
        },
    );

    assert_eq!(policy.style, StyleMode::Plain);
    assert_eq!(policy.progress, ProgressMode::Inline);
}

#[test]
fn stdout_and_stderr_capabilities_can_be_resolved_independently() {
    let preferences = ConsolePreferences::default();
    let stdout = resolve_stream_render_policy(redirected(), preferences);
    let stderr = resolve_stream_render_policy(terminal(), preferences);

    assert_eq!(stdout.style, StyleMode::Plain);
    assert_eq!(stdout.progress, ProgressMode::Lines);
    assert_eq!(stderr.style, StyleMode::Styled);
    assert_eq!(stderr.progress, ProgressMode::Inline);
}

#[test]
fn ascii_preference_or_capability_forces_ascii_glyphs() {
    let requested = resolve_stream_render_policy(
        terminal(),
        ConsolePreferences {
            glyphs: GlyphPreference::Ascii,
            ..ConsolePreferences::default()
        },
    );
    let incapable = resolve_stream_render_policy(
        StreamCapability {
            destination: StreamDestination::Terminal,
            glyphs: GlyphCapability::AsciiOnly,
        },
        ConsolePreferences::default(),
    );

    assert_eq!(requested.glyphs, GlyphMode::Ascii);
    assert_eq!(incapable.glyphs, GlyphMode::Ascii);
}

#[test]
fn contract_is_leaf_like_and_does_not_import_domain_or_runtime_authority() {
    let source = include_str!("../src/console_presentation_contract.rs");
    for forbidden in [
        "etuschlopf::",
        "crate::",
        "anyhow",
        "musicxml",
        "run_outcome::",
        "run_outcome_release_projection::",
        "Command::new",
        "println!",
        "eprintln!",
    ] {
        assert!(
            !source.contains(forbidden),
            "presentation contract must remain leaf-like: {forbidden}"
        );
    }
}
