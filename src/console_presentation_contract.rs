//! Shared, authority-free console presentation vocabulary.
//!
//! This module is deliberately leaf-like and std-only. It does not own domain
//! results, release decisions, logging destinations, process execution or
//! terminal mutation. Orchestrator boundaries may project already-typed
//! operation results into these values before a sink renders them.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationRole {
    Infrastructure,
    Take,
    Kodex,
    Check,
    Build,
    Test,
    Program,
    Renderer,
    Artifact,
    Export,
    Result,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalStatus {
    Pending,
    Running,
    Ok,
    Failed,
    Blocked,
    Committed,
    RolledBack,
}

/// Local user-attention level only. This is intentionally not RunSeverity and
/// must never derive or replace the authoritative final run outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attention {
    Neutral,
    Notice,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emphasis {
    Primary,
    Secondary,
    Muted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    InteractiveOnly,
    TechnicalOnly,
    Both,
}

/// Structured presentation normally belongs to the ordered run narrative.
/// Diagnostic material is a separate class so sinks can route it explicitly
/// without inferring intent from display text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationChannel {
    Narrative,
    Diagnostic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PresentationLine<'a> {
    pub role: PresentationRole,
    pub status: Option<LocalStatus>,
    pub attention: Attention,
    pub emphasis: Emphasis,
    pub visibility: Visibility,
    pub channel: PresentationChannel,
    pub text: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamDestination {
    Terminal,
    Redirected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphCapability {
    Unicode,
    AsciiOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamCapability {
    pub destination: StreamDestination,
    pub glyphs: GlyphCapability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorPreference {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressPreference {
    Auto,
    Plain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphPreference {
    Auto,
    Ascii,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsolePreferences {
    pub color: ColorPreference,
    pub progress: ProgressPreference,
    pub glyphs: GlyphPreference,
}

impl Default for ConsolePreferences {
    fn default() -> Self {
        Self {
            color: ColorPreference::Auto,
            progress: ProgressPreference::Auto,
            glyphs: GlyphPreference::Auto,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleMode {
    Styled,
    Plain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressMode {
    Inline,
    Lines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphMode {
    Unicode,
    Ascii,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamRenderPolicy {
    pub style: StyleMode,
    pub progress: ProgressMode,
    pub glyphs: GlyphMode,
}

pub fn resolve_stream_render_policy(
    capability: StreamCapability,
    preferences: ConsolePreferences,
) -> StreamRenderPolicy {
    let style = match (capability.destination, preferences.color) {
        (StreamDestination::Redirected, _) | (_, ColorPreference::Never) => StyleMode::Plain,
        (_, ColorPreference::Auto | ColorPreference::Always) => StyleMode::Styled,
    };
    let progress = match (capability.destination, preferences.progress) {
        (StreamDestination::Redirected, _) | (_, ProgressPreference::Plain) => ProgressMode::Lines,
        (_, ProgressPreference::Auto) => ProgressMode::Inline,
    };
    let glyphs = match (capability.glyphs, preferences.glyphs) {
        (GlyphCapability::AsciiOnly, _) | (_, GlyphPreference::Ascii) => GlyphMode::Ascii,
        (_, GlyphPreference::Auto) => GlyphMode::Unicode,
    };
    StreamRenderPolicy {
        style,
        progress,
        glyphs,
    }
}
