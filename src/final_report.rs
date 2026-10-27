use anyhow::{bail, Result};
use std::ops::{Deref, DerefMut};

/// Sealed read capability: only finalization can expose observation data.
pub(crate) struct SscObservationReadAccess {
    _private: (),
}

const REQUIRED_FINAL_REPORT_SECTIONS: [&str; 2] = [
    "TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION",
    "SILENT-VOICE-COVERAGE-WRITE-OUTCOME",
];

const REQUIRED_SILENT_VOICE_RENDERER_OUTCOME: &str =
    "RendererOutcome: observed_from_existing_writer_audit.\n";
const NOT_REQUESTED_SILENT_VOICE_RENDERER_OUTCOME: &str = "RendererOutcome: not_requested.\n";
const BLOCKED_SILENT_VOICE_RENDERER_OUTCOME: &str = "RendererOutcome: blocked_before_renderer.\n";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererOutcomeExpectation {
    Observed,
    NotRequested,
    BlockedBeforeRenderer,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ReportRewriteScope {
    pub(crate) external_metadata_opaque: bool,
}

pub struct DraftReport {
    text: String,
    rewrite_scope: ReportRewriteScope,
}

impl DraftReport {
    pub fn new(text: String) -> Self {
        Self {
            text,
            rewrite_scope:
                crate::audit_only::report_sanitizer_rewrite_scope_projection::ACTIVE_REPORT_REWRITE_SCOPE,
        }
    }

    pub fn compact(self, operation: impl FnOnce(&str) -> String) -> CompactedReport {
        let document = ReportDocument::parse(self.text);
        let rewritten = if self.rewrite_scope.external_metadata_opaque {
            let capacity = document.blocks.iter().map(|block| block.text().len()).sum();
            let mut rewritten = String::with_capacity(capacity);
            let mut internal = String::new();
            for block in &document.blocks {
                if block.origin() == ReportFragmentOrigin::ExternalMetadata {
                    if !internal.is_empty() {
                        let text = crate::report_sanitizer::sanitize_report_hardcoded_measure_artifacts(
                            std::mem::take(&mut internal),
                        );
                        rewritten.push_str(
                            &crate::report_sanitizer::sanitize_report_hygiene_polish(text),
                        );
                    }
                    rewritten.push_str(block.text());
                } else {
                    internal.push_str(block.text());
                }
            }
            if !internal.is_empty() {
                let text = crate::report_sanitizer::sanitize_report_hardcoded_measure_artifacts(
                    internal,
                );
                rewritten.push_str(
                    &crate::report_sanitizer::sanitize_report_hygiene_polish(text),
                );
            }
            rewritten
        } else {
            let text = crate::report_sanitizer::sanitize_report_hardcoded_measure_artifacts(
                document.render(),
            );
            crate::report_sanitizer::sanitize_report_hygiene_polish(text)
        };
        CompactedReport {
            document: ReportDocument::parse(operation(&rewritten)),
            rewrite_scope: self.rewrite_scope,
        }
    }
}

impl Deref for DraftReport {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl DerefMut for DraftReport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.text
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ReportSectionId(Box<str>);

impl ReportSectionId {
    fn new(value: &str) -> Self {
        Self(value.trim().into())
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ReportFragmentOrigin {
    InternalReportVocabulary,
    ExternalMetadata,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ReportBlock {
    Raw {
        origin: ReportFragmentOrigin,
        text: String,
    },
    Section {
        id: ReportSectionId,
        text: String,
    },
}

impl ReportBlock {
    fn text(&self) -> &str {
        match self {
            Self::Raw { text, .. } | Self::Section { text, .. } => text,
        }
    }

    fn origin(&self) -> ReportFragmentOrigin {
        match self {
            Self::Raw { origin, .. } => *origin,
            Self::Section { .. } => ReportFragmentOrigin::InternalReportVocabulary,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ReportDocument {
    blocks: Vec<ReportBlock>,
}

fn split_fragments_by_origin(text: &str) -> Vec<(ReportFragmentOrigin, String)> {
    if text.is_empty() {
        return Vec::new();
    }

    let mut fragments = Vec::new();
    let mut internal = String::new();
    for segment in text.split_inclusive('\n') {
        let content = segment
            .strip_suffix("\r\n")
            .or_else(|| segment.strip_suffix('\n'))
            .unwrap_or(segment);
        if content.starts_with("Partitur: ") {
            if !internal.is_empty() {
                fragments.push((
                    ReportFragmentOrigin::InternalReportVocabulary,
                    std::mem::take(&mut internal),
                ));
            }
            fragments.push((
                ReportFragmentOrigin::ExternalMetadata,
                segment.to_string(),
            ));
        } else {
            internal.push_str(segment);
        }
    }
    if !internal.is_empty() {
        fragments.push((
            ReportFragmentOrigin::InternalReportVocabulary,
            internal,
        ));
    }
    fragments
}

fn push_raw_blocks_by_origin(blocks: &mut Vec<ReportBlock>, text: &str) {
    for (origin, text) in split_fragments_by_origin(text) {
        blocks.push(ReportBlock::Raw { origin, text });
    }
}

fn push_section_blocks_by_origin(
    blocks: &mut Vec<ReportBlock>,
    section_id: &str,
    text: &str,
) {
    if section_id != "ETUSCHLOPF-ERGEBNIS" {
        blocks.push(ReportBlock::Section {
            id: ReportSectionId::new(section_id),
            text: text.to_string(),
        });
        return;
    }

    let mut fragments = split_fragments_by_origin(text).into_iter();
    let Some((origin, first_text)) = fragments.next() else {
        return;
    };
    debug_assert_eq!(origin, ReportFragmentOrigin::InternalReportVocabulary);
    blocks.push(ReportBlock::Section {
        id: ReportSectionId::new(section_id),
        text: first_text,
    });
    for (origin, text) in fragments {
        blocks.push(ReportBlock::Raw { origin, text });
    }
}

impl ReportDocument {
    fn parse(text: String) -> Self {
        let lines = report_lines(&text);
        let section_starts = lines
            .windows(2)
            .filter_map(|pair| {
                let heading = pair[0].content(&text);
                let underline = pair[1].content(&text);
                (is_report_section_heading(heading) && is_report_section_underline(underline))
                    .then_some((pair[0].start, heading.trim().to_string()))
            })
            .collect::<Vec<_>>();

        if section_starts.is_empty() {
            let mut blocks = Vec::new();
            push_raw_blocks_by_origin(&mut blocks, &text);
            return Self { blocks };
        }

        let mut blocks = Vec::with_capacity(section_starts.len() * 2 + 1);
        let first_start = section_starts[0].0;
        if first_start > 0 {
            push_raw_blocks_by_origin(&mut blocks, &text[..first_start]);
        }

        for (index, (start, heading)) in section_starts.iter().enumerate() {
            let end = section_starts
                .get(index + 1)
                .map_or(text.len(), |(next_start, _)| *next_start);
            push_section_blocks_by_origin(&mut blocks, heading, &text[*start..end]);
        }

        Self { blocks }
    }

    fn render(&self) -> String {
        let capacity = self.blocks.iter().map(|block| block.text().len()).sum();
        let mut text = String::with_capacity(capacity);
        for block in &self.blocks {
            let _origin = block.origin();
            text.push_str(block.text());
        }
        text
    }

    fn section_count(&self, section_id: &str) -> usize {
        let section_id = section_id.trim();
        self.blocks
            .iter()
            .filter(|block| {
                matches!(
                    block,
                    ReportBlock::Section { id, .. } if id.as_str() == section_id
                )
            })
            .count()
    }

    #[cfg(test)]
    fn external_metadata_block_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.origin() == ReportFragmentOrigin::ExternalMetadata)
            .count()
    }
}

#[derive(Debug, Clone, Copy)]
struct ReportLine {
    start: usize,
    content_end: usize,
}

impl ReportLine {
    fn content<'a>(&self, text: &'a str) -> &'a str {
        &text[self.start..self.content_end]
    }
}

fn report_lines(text: &str) -> Vec<ReportLine> {
    let mut lines = Vec::new();
    let mut start = 0usize;
    for segment in text.split_inclusive('\n') {
        let end = start + segment.len();
        let mut content_end = end;
        if segment.ends_with('\n') {
            content_end -= 1;
            if content_end > start && text.as_bytes()[content_end - 1] == b'\r' {
                content_end -= 1;
            }
        }
        lines.push(ReportLine { start, content_end });
        start = end;
    }
    if text.is_empty() {
        lines.push(ReportLine {
            start: 0,
            content_end: 0,
        });
    }
    lines
}

fn is_report_section_heading(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty()
        && trimmed.chars().any(char::is_alphabetic)
        && !trimmed.chars().any(char::is_lowercase)
}

fn is_report_section_underline(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.len() >= 3 && trimmed.chars().all(|ch| ch == '-' || ch == '=')
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProjectionIdentity {
    Section(&'static str),
}

impl ProjectionIdentity {
    fn count_in(self, document: &ReportDocument) -> usize {
        document.section_count(self.value())
    }

    fn kind(self) -> &'static str {
        "Sektion"
    }

    fn value(self) -> &'static str {
        let Self::Section(section_id) = self;
        section_id
    }
}

pub struct CompactedReport {
    document: ReportDocument,
    rewrite_scope: ReportRewriteScope,
}

impl CompactedReport {
    pub fn project_section(
        &mut self,
        producer_name: &'static str,
        section_id: &'static str,
        producer: impl FnOnce(&mut String),
    ) -> Result<()> {
        self.project_declared(
            producer_name,
            &[ProjectionIdentity::Section(section_id)],
            producer,
        )
    }

    pub fn project_owned_section(
        &mut self,
        producer_name: &'static str,
        section_id: &'static str,
        underline: &'static str,
        body_producer: impl FnOnce(&mut String),
    ) -> Result<()> {
        self.project_declared(
            producer_name,
            &[ProjectionIdentity::Section(section_id)],
            move |out| {
                out.push('\n');
                out.push_str(section_id);
                out.push('\n');
                out.push_str(underline);
                out.push('\n');
                body_producer(out);
            },
        )
    }

    pub fn project_bundle(
        &mut self,
        producer_name: &'static str,
        expected: &[ProjectionIdentity],
        producer: impl FnOnce(&mut String),
    ) -> Result<()> {
        self.project_declared(producer_name, expected, producer)
    }

    fn project_declared(
        &mut self,
        producer_name: &'static str,
        expected: &[ProjectionIdentity],
        producer: impl FnOnce(&mut String),
    ) -> Result<()> {
        if expected.is_empty() {
            bail!(
                "FinalReportProjection: Producer '{}' besitzt keinen deklarierten Projektionsvertrag",
                producer_name
            );
        }
        for (index, identity) in expected.iter().enumerate() {
            if expected[index + 1..].contains(identity) {
                bail!(
                    "FinalReportProjection: Producer '{}' deklariert {} '{}' mehrfach",
                    producer_name,
                    identity.kind(),
                    identity.value()
                );
            }
        }

        let existing_counts = expected
            .iter()
            .map(|identity| identity.count_in(&self.document))
            .collect::<Vec<_>>();
        for (identity, count) in expected.iter().zip(&existing_counts) {
            if *count > 1 {
                bail!(
                    "FinalReportProjection: {} '{}' ist vor Producer '{}' bereits {}-fach vorhanden",
                    identity.kind(),
                    identity.value(),
                    producer_name,
                    count
                );
            }
        }
        let existing = existing_counts.iter().filter(|count| **count == 1).count();
        if existing == expected.len() {
            return Ok(());
        }
        if existing != 0 {
            bail!(
                "FinalReportProjection: Producer '{}' trifft einen unvollstaendigen Vertragszustand ({}/{} Projektionen vorhanden)",
                producer_name,
                existing,
                expected.len()
            );
        }

        let mut generated = String::new();
        producer(&mut generated);
        let generated_document = ReportDocument::parse(generated);

        for identity in expected {
            let generated_count = identity.count_in(&generated_document);
            if generated_count != 1 {
                bail!(
                    "FinalReportProjection: Producer '{}' lieferte fuer {} '{}' {} Treffer statt exakt eines",
                    producer_name,
                    identity.kind(),
                    identity.value(),
                    generated_count
                );
            }
        }

        for block in &generated_document.blocks {
            let ReportBlock::Section { id, .. } = block else {
                continue;
            };
            let declared = expected.iter().any(|identity| {
                matches!(*identity, ProjectionIdentity::Section(section_id) if section_id.trim() == id.as_str())
            });
            if !declared {
                bail!(
                    "FinalReportProjection: Producer '{}' erzeugte die nicht deklarierte Sektion '{}'",
                    producer_name,
                    id.as_str()
                );
            }
        }

        self.document.blocks.extend(generated_document.blocks);
        Ok(())
    }


    pub fn rewrite_document(&mut self, operation: impl Fn(String) -> String) {
        if self.rewrite_scope.external_metadata_opaque {
            let capacity = self
                .document
                .blocks
                .iter()
                .map(|block| block.text().len())
                .sum();
            let mut rewritten = String::with_capacity(capacity);
            let mut internal = String::new();
            for block in &self.document.blocks {
                if block.origin() == ReportFragmentOrigin::ExternalMetadata {
                    if !internal.is_empty() {
                        rewritten.push_str(&operation(std::mem::take(&mut internal)));
                    }
                    rewritten.push_str(block.text());
                } else {
                    internal.push_str(block.text());
                }
            }
            if !internal.is_empty() {
                rewritten.push_str(&operation(internal));
            }
            self.document = ReportDocument::parse(rewritten);
        } else {
            self.document = ReportDocument::parse(operation(self.document.render()));
        }
    }

    pub fn render_snapshot(&self) -> String {
        self.document.render()
    }

    #[cfg(test)]
    pub fn finalize(self) -> Result<FinalizedReport> {
        self.finalize_for_pdf_request(true)
    }

    #[cfg(test)]
    pub fn finalize_for_pdf_request(self, pdf_requested: bool) -> Result<FinalizedReport> {
        self.finalize_for_renderer_outcome(if pdf_requested {
            RendererOutcomeExpectation::Observed
        } else {
            RendererOutcomeExpectation::NotRequested
        })
    }

    pub fn finalize_for_renderer_outcome(self, expected: RendererOutcomeExpectation) -> Result<FinalizedReport> {
        validate_required_contract(&self.document, expected)?;
        let mut text = self.document.render();
        let access = SscObservationReadAccess { _private: () };
        crate::ssc_shadow_report::append_ssc_shadow_observation(&access, &mut text);
        crate::ssc_shadow_measurement::append_ssc_shadow_measurement(&access, &mut text);
        Ok(FinalizedReport { text })
    }
}

#[derive(Debug)]
pub struct FinalizedReport {
    text: String,
}

impl FinalizedReport {
    pub fn as_str(&self) -> &str {
        &self.text
    }
}

fn validate_required_contract(document: &ReportDocument, expected: RendererOutcomeExpectation) -> Result<()> {
    for section in REQUIRED_FINAL_REPORT_SECTIONS {
        let occurrences = document.section_count(section);
        if occurrences != 1 {
            bail!(
                "FinalReportContract: Pflichtsektion '{}' kommt {}-mal statt exakt einmal vor",
                section,
                occurrences
            );
        }
    }

    let report = document.render();
    let observed = report.match_indices(REQUIRED_SILENT_VOICE_RENDERER_OUTCOME).count();
    let not_requested = report.match_indices(NOT_REQUESTED_SILENT_VOICE_RENDERER_OUTCOME).count();
    let blocked = report.match_indices(BLOCKED_SILENT_VOICE_RENDERER_OUTCOME).count();
    let expected_counts = match expected {
        RendererOutcomeExpectation::Observed => (1, 0, 0),
        RendererOutcomeExpectation::NotRequested => (0, 1, 0),
        RendererOutcomeExpectation::BlockedBeforeRenderer => (0, 0, 1),
    };
    if (observed, not_requested, blocked) != expected_counts {
        bail!(
            "FinalReportContract: Silent-Voice-RendererOutcome widerspricht gebundener Ausfuehrungsentscheidung; expected={:?}; observed={}; not_requested={}; blocked={}",
            expected, observed, not_requested, blocked
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        DraftReport, ProjectionIdentity, ReportDocument, ReportFragmentOrigin,
        REQUIRED_FINAL_REPORT_SECTIONS, REQUIRED_SILENT_VOICE_RENDERER_OUTCOME,
    };

    fn section(title: &str, body: &str) -> String {
        format!("{title}\n{}\n{body}\n", "-".repeat(title.len()))
    }

    fn valid_report() -> String {
        let mut report = String::new();
        for title in REQUIRED_FINAL_REPORT_SECTIONS {
            report.push_str(&section(title, "Status: present."));
        }
        report.push_str(REQUIRED_SILENT_VOICE_RENDERER_OUTCOME);
        report
    }

    #[test]
    fn section_document_roundtrip_preserves_every_byte() {
        let report = format!(
            "Preamble\n\n{}\n{}Tail without newline",
            section("FIRST-SECTION", "Body: one."),
            section("SECOND-SECTION", "Body: two."),
        );
        assert_eq!(ReportDocument::parse(report.clone()).render(), report);
    }

    #[test]
    fn external_score_title_is_typed_without_changing_rendered_bytes() {
        let report = format!(
            "ETUSCHLOPF-ERGEBNIS\n==================\n\nWerkzeug: EtuSchlopf 0.10.1839\nPartitur: HumanReview\nStatus: OK\n\n{}",
            section("FIRST-SECTION", "Status: present."),
        );
        let document = ReportDocument::parse(report.clone());
        assert_eq!(document.render(), report);
        assert_eq!(document.external_metadata_block_count(), 1);
        assert!(document.blocks.iter().any(|block| {
            block.origin() == ReportFragmentOrigin::ExternalMetadata
                && block.text() == "Partitur: HumanReview\n"
        }));
    }

    #[test]
    fn legacy_whole_document_rewrite_still_reaches_external_metadata() {
        let report = format!(
            "Preamble\nPartitur: HumanReview\n{}",
            section("FIRST-SECTION", "Status: present."),
        );
        let mut document = ReportDocument::parse(report);
        let rendered = crate::report_sanitizer::sanitize_behind_bars_active_authority_language_v1284(
            document.render(),
        );
        document = ReportDocument::parse(rendered);
        assert_eq!(document.external_metadata_block_count(), 1);
        assert!(document.render().contains("Partitur: BehindBarsRuleCheck\n"));
    }



    #[test]
    fn exact_section_identity_ignores_body_mentions() {
        let report = section(
            "FIRST-SECTION",
            "Text mentions SECOND-SECTION but does not define it.",
        );
        let document = ReportDocument::parse(report);
        assert_eq!(document.section_count("FIRST-SECTION"), 1);
        assert_eq!(document.section_count("SECOND-SECTION"), 0);
    }

    #[test]
    fn declared_section_projection_projects_exactly_once_without_substring_gate() {
        let mut report = DraftReport::new(valid_report()).compact(ToOwned::to_owned);
        report
            .project_section(
                "additional_section",
                "ADDITIONAL-SECTION",
                |out| out.push_str(&section("ADDITIONAL-SECTION", "Status: projected.")),
            )
            .expect("first exact projection");
        report
            .project_section("additional_section", "ADDITIONAL-SECTION", |_| {
                panic!("exactly present section must not be produced twice");
            })
            .expect("existing exact section");
        assert_eq!(
            ReportDocument::parse(report.render_snapshot())
                .section_count("ADDITIONAL-SECTION"),
            1
        );
    }

    #[test]
    fn ledger_owned_section_materializes_identity_around_body_only_producer() {
        let mut report = DraftReport::new(valid_report()).compact(ToOwned::to_owned);
        report
            .project_owned_section(
                "owned_section",
                "OWNED-SECTION",
                "-------------",
                |out| out.push_str("Status: body only.\n"),
            )
            .expect("ledger-owned section projection");
        let snapshot = report.render_snapshot();
        assert!(snapshot.contains("\nOWNED-SECTION\n-------------\nStatus: body only.\n"));
        assert_eq!(
            ReportDocument::parse(snapshot).section_count("OWNED-SECTION"),
            1
        );
    }

    #[test]
    fn ledger_owned_section_rejects_body_that_opens_an_extra_section() {
        let mut report = DraftReport::new(valid_report()).compact(ToOwned::to_owned);
        let error = report
            .project_owned_section(
                "owned_section",
                "OWNED-SECTION",
                "-------------",
                |out| out.push_str("EXTRA-SECTION\n-------------\nStatus: forbidden.\n"),
            )
            .expect_err("body-only producer must not create another section");
        assert!(error.to_string().contains("nicht deklarierte Sektion"));
    }

    #[test]
    fn projection_rejects_a_different_generated_section_identity() {
        let mut report = DraftReport::new(valid_report()).compact(ToOwned::to_owned);
        let error = report
            .project_section("wrong_section", "EXPECTED-SECTION", |out| {
                out.push_str(&section("OTHER-SECTION", "Status: wrong producer."));
            })
            .expect_err("wrong generated section identity must fail");
        assert!(error.to_string().contains("0 Treffer statt exakt eines"));
    }

    #[test]
    fn projection_bundle_rejects_undeclared_extra_section() {
        let mut report = DraftReport::new(valid_report()).compact(ToOwned::to_owned);
        let error = report
            .project_bundle(
                "declared_bundle",
                &[ProjectionIdentity::Section("EXPECTED-SECTION")],
                |out| {
                    out.push_str(&section("EXPECTED-SECTION", "Status: expected."));
                    out.push_str(&section("UNDECLARED-SECTION", "Status: forbidden."));
                },
            )
            .expect_err("undeclared section must fail");
        assert!(error.to_string().contains("nicht deklarierte Sektion"));
    }

    #[test]
    fn projection_bundle_rejects_partial_existing_state() {
        let mut source = valid_report();
        source.push_str(&section("BUNDLE-A", "Status: already present."));
        let mut report = DraftReport::new(source).compact(ToOwned::to_owned);
        let error = report
            .project_bundle(
                "partial_bundle",
                &[
                    ProjectionIdentity::Section("BUNDLE-A"),
                    ProjectionIdentity::Section("BUNDLE-B"),
                ],
                |out| {
                    out.push_str(&section("BUNDLE-A", "Status: generated."));
                    out.push_str(&section("BUNDLE-B", "Status: generated."));
                },
            )
            .expect_err("partial bundle state must fail");
        assert!(error.to_string().contains("unvollstaendigen Vertragszustand"));
    }

    #[test]
    fn finalization_accepts_each_required_contract_exactly_once() {
        let finalized = DraftReport::new(valid_report())
            .compact(ToOwned::to_owned)
            .finalize()
            .expect("valid final report contract");
        assert!(finalized
            .as_str()
            .contains("SILENT-VOICE-COVERAGE-WRITE-OUTCOME"));
    }

    #[test]
    fn finalization_rejects_duplicate_required_sections() {
        let mut report = valid_report();
        report.push_str(&section(REQUIRED_FINAL_REPORT_SECTIONS[0], "duplicate"));
        let error = DraftReport::new(report)
            .compact(ToOwned::to_owned)
            .finalize()
            .expect_err("duplicate section must fail");
        assert!(error.to_string().contains("kommt 2-mal"));
    }

    #[test]
    fn no_pdf_finalization_requires_explicit_not_requested_outcome() {
        let report = valid_report().replace(
            REQUIRED_SILENT_VOICE_RENDERER_OUTCOME,
            super::NOT_REQUESTED_SILENT_VOICE_RENDERER_OUTCOME,
        );
        assert!(DraftReport::new(report.clone()).compact(ToOwned::to_owned)
            .finalize_for_pdf_request(false).is_ok());
        assert!(DraftReport::new(report).compact(ToOwned::to_owned)
            .finalize_for_pdf_request(true).is_err());
        assert!(DraftReport::new(valid_report()).compact(ToOwned::to_owned)
            .finalize_for_pdf_request(false).is_err());
    }

    #[test]
    fn renderer_outcome_absence_duplicates_and_conflicting_modes_are_rejected() {
        let base = valid_report().replace(REQUIRED_SILENT_VOICE_RENDERER_OUTCOME, "");
        for marker in ["", REQUIRED_SILENT_VOICE_RENDERER_OUTCOME,
            super::NOT_REQUESTED_SILENT_VOICE_RENDERER_OUTCOME] {
            let report = format!("{base}{marker}{marker}");
            for requested in [false, true] {
                assert!(DraftReport::new(report.clone()).compact(ToOwned::to_owned)
                    .finalize_for_pdf_request(requested).is_err());
            }
        }
        let conflicting = format!("{}{outcome}", valid_report(),
            outcome = super::NOT_REQUESTED_SILENT_VOICE_RENDERER_OUTCOME);
        for requested in [false, true] {
            assert!(DraftReport::new(conflicting.clone()).compact(ToOwned::to_owned)
                .finalize_for_pdf_request(requested).is_err());
        }
    }

    #[test]
    fn blocked_pdf_report_uses_bound_blocker_outcome_and_remains_readable() {
        let blocked = valid_report().replace(REQUIRED_SILENT_VOICE_RENDERER_OUTCOME,
            super::BLOCKED_SILENT_VOICE_RENDERER_OUTCOME) + "\nErgebnis: BLOCKIERT\nBlocker: 1\n";
        let finalized = DraftReport::new(blocked.clone()).compact(ToOwned::to_owned)
            .finalize_for_renderer_outcome(super::RendererOutcomeExpectation::BlockedBeforeRenderer)
            .expect("requested PDF withheld by blockers still has a valid report");
        assert!(finalized.as_str().contains("Blocker: 1"));
        for other in [super::RendererOutcomeExpectation::Observed, super::RendererOutcomeExpectation::NotRequested] {
            assert!(DraftReport::new(blocked.clone()).compact(ToOwned::to_owned)
                .finalize_for_renderer_outcome(other).is_err());
        }
    }

    #[test]
    fn all_renderer_outcomes_require_one_exact_independent_mode_marker() {
        let base=valid_report().replace(REQUIRED_SILENT_VOICE_RENDERER_OUTCOME, "");
        let modes=[super::RendererOutcomeExpectation::Observed,
            super::RendererOutcomeExpectation::NotRequested,
            super::RendererOutcomeExpectation::BlockedBeforeRenderer];
        let markers=[REQUIRED_SILENT_VOICE_RENDERER_OUTCOME,
            super::NOT_REQUESTED_SILENT_VOICE_RENDERER_OUTCOME,
            super::BLOCKED_SILENT_VOICE_RENDERER_OUTCOME];
        for (i,expected) in modes.iter().enumerate() {
            for (j,marker) in markers.iter().enumerate() {
                let report=format!("{base}{marker}");
                assert_eq!(DraftReport::new(report).compact(ToOwned::to_owned)
                    .finalize_for_renderer_outcome(*expected).is_ok(),i==j);
                let duplicate=format!("{base}{marker}{marker}");
                assert!(DraftReport::new(duplicate).compact(ToOwned::to_owned)
                    .finalize_for_renderer_outcome(*expected).is_err());
            }
            assert!(DraftReport::new(base.clone()).compact(ToOwned::to_owned)
                .finalize_for_renderer_outcome(*expected).is_err());
        }
    }

}
