//! Read-only renderer system-break readback from the final MuseScore PDF.
//!
//! This audit intentionally owns no layout policy and computes no replacement
//! demand/capacity model. It compares the system/page starts materialized in the
//! final MusicXML with measure-number text physically present at system starts
//! in the renderer-written PDF. Unsupported PDF dialects degrade to explicit
//! audit-unavailable evidence. Beam geometry is also a productive admission input:
//! it observes only already authenticated occurrence cohorts and never constructs
//! musical identity or replaces the existing original owner or physical writer.

use flate2::read::ZlibDecoder;
use roxmltree::Document;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::Path;
use crate::final_notation_timeline_facts::FinalNotationTimelineMeasureFacts as
    MeasureRhythmicColumnEvidence;

const READBACK_BASIS: &str = "renderer_pdf_measure_number_text_position_v1";
const LEFT_MARGIN_MIN: f64 = 30.0;
const LEFT_MARGIN_MAX: f64 = 70.0;
const HEADER_EXCLUSION_FROM_TOP: f64 = 55.0;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct PdfTextEvent {
    pub(super) page_index: usize,
    pub(super) x: f64,
    pub(super) y_from_bottom: f64,
    pub(super) page_height: f64,
    pub(super) text: String,
    pub(super) head_bounds: Option<[f64; 4]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RendererSystemBreakReadbackAudit {
    pub(crate) planned_system_starts: Vec<usize>,
    pub(crate) rendered_system_starts: Vec<usize>,
    pub(crate) planned_page_starts: Vec<usize>,
    pub(crate) rendered_page_starts: Vec<usize>,
    pub(crate) inserted_system_breaks: Vec<usize>,
    pub(crate) collapsed_system_breaks: Vec<usize>,
    pub(crate) inserted_page_breaks: Vec<usize>,
    pub(crate) collapsed_page_breaks: Vec<usize>,
    pub(crate) measure_count: usize,
    pub(crate) pdf_pages: usize,
    pub(crate) ottava_context: crate::registernotation::reader_policy::PdfContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlannerSystemLoadEvidence {
    start_measure: usize,
    end_measure: usize,
    load: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlannerDemandBinding {
    accepted_system_capacity: usize,
    systems: Vec<PlannerSystemLoadEvidence>,
    measure_demands: Vec<usize>,
}

impl RendererSystemBreakReadbackAudit {
    pub(crate) fn drifted(&self) -> bool {
        !self.inserted_system_breaks.is_empty()
            || !self.collapsed_system_breaks.is_empty()
            || !self.inserted_page_breaks.is_empty()
            || !self.collapsed_page_breaks.is_empty()
    }

    fn planned_groups(&self) -> String {
        format_system_groups(&self.planned_system_starts, self.measure_count)
    }

    fn rendered_groups(&self) -> String {
        format_system_groups(&self.rendered_system_starts, self.measure_count)
    }
}

#[derive(Debug, Clone)]
struct PdfObject {
    body: Vec<u8>,
}

impl PdfObject {
    fn dictionary_text(&self) -> &str {
        let end = find_subslice(&self.body, b"stream").unwrap_or(self.body.len());
        std::str::from_utf8(&self.body[..end]).unwrap_or("")
    }

    fn decoded_stream(&self) -> anyhow::Result<Option<Vec<u8>>> {
        let Some(stream_marker) = find_subslice(&self.body, b"stream") else {
            return Ok(None);
        };
        let mut start = stream_marker + b"stream".len();
        if self.body.get(start) == Some(&b'\r') {
            start += 1;
        }
        if self.body.get(start) == Some(&b'\n') {
            start += 1;
        }
        let end_rel = find_subslice(&self.body[start..], b"endstream")
            .ok_or_else(|| anyhow::anyhow!("PDF stream besitzt kein endstream"))?;
        let mut stream = self.body[start..start + end_rel].to_vec();
        while matches!(stream.last(), Some(&b'\r') | Some(&b'\n')) {
            stream.pop();
        }
        if self.dictionary_text().contains("/FlateDecode") {
            let mut decoded = Vec::new();
            ZlibDecoder::new(stream.as_slice()).read_to_end(&mut decoded)?;
            Ok(Some(decoded))
        } else {
            Ok(Some(stream))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Matrix {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

impl Matrix {
    const IDENTITY: Self = Self {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 0.0,
        f: 0.0,
    };

    fn multiply(self, rhs: Self) -> Self {
        Self {
            a: self.a * rhs.a + self.c * rhs.b,
            b: self.b * rhs.a + self.d * rhs.b,
            c: self.a * rhs.c + self.c * rhs.d,
            d: self.b * rhs.c + self.d * rhs.d,
            e: self.a * rhs.e + self.c * rhs.f + self.e,
            f: self.b * rhs.e + self.d * rhs.f + self.f,
        }
    }

    fn point(self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.c * y + self.e,
            self.b * x + self.d * y + self.f,
        )
    }
}

pub(crate) fn audit_paths(
    planned_musicxml: &Path,
    rendered_pdf: &Path,
) -> anyhow::Result<RendererSystemBreakReadbackAudit> {
    let planned_xml = std::fs::read_to_string(planned_musicxml)?;
    let (measure_count, planned_system_starts, planned_page_starts) =
        planned_breaks_from_musicxml(&planned_xml)?;
    let pdf_bytes = std::fs::read(rendered_pdf)?;
    let (events, pdf_pages) = extract_pdf_text_events(&pdf_bytes)?;
    if pdf_pages == 0 {
        anyhow::bail!("RendererSystemBreakReadback: keine PDF-Seiten lesbar");
    }

    let mut rendered_by_page = BTreeMap::<usize, BTreeSet<usize>>::new();
    for event in events {
        let top = event.page_height - event.y_from_bottom;
        if !(LEFT_MARGIN_MIN..=LEFT_MARGIN_MAX).contains(&event.x)
            || top < HEADER_EXCLUSION_FROM_TOP
        {
            continue;
        }
        let trimmed = event.text.trim();
        if trimmed.is_empty() || !trimmed.chars().all(|ch| ch.is_ascii_digit()) {
            continue;
        }
        let Ok(measure) = trimmed.parse::<usize>() else {
            continue;
        };
        if measure <= 1 || measure > measure_count {
            continue;
        }
        rendered_by_page
            .entry(event.page_index)
            .or_default()
            .insert(measure);
    }

    let mut rendered_system_starts = vec![1usize];
    for measures in rendered_by_page.values() {
        rendered_system_starts.extend(measures.iter().copied());
    }
    rendered_system_starts.sort_unstable();
    rendered_system_starts.dedup();

    let mut rendered_page_starts = vec![1usize];
    for page in 2..=pdf_pages {
        let first = rendered_by_page
            .get(&page)
            .and_then(|measures| measures.iter().next().copied())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "RendererSystemBreakReadback: Seite {page} besitzt keinen lesbaren Systemanfang"
                )
            })?;
        rendered_page_starts.push(first);
    }

    let planned_system_set = planned_system_starts.iter().copied().collect::<BTreeSet<_>>();
    let rendered_system_set = rendered_system_starts.iter().copied().collect::<BTreeSet<_>>();
    let planned_page_set = planned_page_starts.iter().copied().collect::<BTreeSet<_>>();
    let rendered_page_set = rendered_page_starts.iter().copied().collect::<BTreeSet<_>>();

    let ottava_context = crate::audit_only::renderer_ottava_context::observe(&planned_xml, &pdf_bytes);
    Ok(RendererSystemBreakReadbackAudit {
        inserted_system_breaks: rendered_system_set
            .difference(&planned_system_set)
            .copied()
            .collect(),
        collapsed_system_breaks: planned_system_set
            .difference(&rendered_system_set)
            .copied()
            .collect(),
        inserted_page_breaks: rendered_page_set
            .difference(&planned_page_set)
            .copied()
            .collect(),
        collapsed_page_breaks: planned_page_set
            .difference(&rendered_page_set)
            .copied()
            .collect(),
        planned_system_starts,
        rendered_system_starts,
        planned_page_starts,
        rendered_page_starts,
        measure_count,
        pdf_pages,
        ottava_context,
    })
}

pub(crate) fn current_process_projection_lines(
    joint_planner_line: Option<&str>,
    final_selected_plan_demand_line: Option<&str>,
) -> Vec<String> {
    match current_process_audit() {
        Ok(Some(audit)) => vec![
            format!(
                "RendererSystemBreakReadback: Status {} | Basis {} | PlannedSystems {} | RenderedSystems {} | InsertedSystemBreaks {} | CollapsedSystemBreaks {} | PlannedPages {} | RenderedPages {} | InsertedPageBreaks {} | CollapsedPageBreaks {} | PolicyWrite nein",
                if audit.drifted() { "DRIFT" } else { "MATCH" },
                READBACK_BASIS,
                audit.planned_system_starts.len(),
                audit.rendered_system_starts.len(),
                format_usize_list(&audit.inserted_system_breaks),
                format_usize_list(&audit.collapsed_system_breaks),
                audit.planned_page_starts.len(),
                audit.rendered_page_starts.len(),
                format_usize_list(&audit.inserted_page_breaks),
                format_usize_list(&audit.collapsed_page_breaks),
            ),
            format!(
                "RendererSystemBreakReadbackPlan: SystemStarts {} | PageStarts {} | SystemGroups {}",
                format_usize_list(&audit.planned_system_starts),
                format_usize_list(&audit.planned_page_starts),
                audit.planned_groups(),
            ),
            format!(
                "RendererSystemBreakReadbackRendered: SystemStarts {} | PageStarts {} | SystemGroups {} | PdfPages {}",
                format_usize_list(&audit.rendered_system_starts),
                format_usize_list(&audit.rendered_page_starts),
                audit.rendered_groups(),
                audit.pdf_pages,
            ),
            planner_demand_binding_projection(
                &audit,
                final_selected_plan_demand_line.or(joint_planner_line),
            ),
            current_process_rhythmic_shape_projection(&audit),
        ],
        Ok(None) => Vec::new(),
        Err(error) => vec![format!(
            "RendererSystemBreakReadback: Status UNAVAILABLE | Basis {} | Reason {} | PolicyWrite nein",
            READBACK_BASIS,
            one_line(&error.to_string()),
        )],
    }
}

fn planner_demand_binding_projection(
    audit: &RendererSystemBreakReadbackAudit,
    demand_binding_line: Option<&str>,
) -> String {
    let Some(demand_binding_line) = demand_binding_line else {
        return "RendererSystemBreakReadbackDemandBinding: Status UNAVAILABLE | Reason joint_planner_evidence_missing | ModelRecompute nein | ThresholdChange nein | PolicyWrite nein".to_string();
    };
    let source = if demand_binding_line
        .starts_with("ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding:")
    {
        "final_selected_plan_existing_joint_planner_evidence"
    } else {
        "existing_joint_planner_evidence"
    };
    match parse_planner_demand_binding(demand_binding_line)
        .and_then(|binding| validate_planner_demand_binding(audit, binding))
    {
        Ok(binding) => {
            let system_rows = binding
                .systems
                .iter()
                .map(|row| {
                    let split_points = audit
                        .inserted_system_breaks
                        .iter()
                        .copied()
                        .filter(|measure| {
                            row.start_measure < *measure && *measure <= row.end_measure
                        })
                        .collect::<Vec<_>>();
                    format!(
                        "{}=load{}/headroom{}/split{}",
                        format_measure_range(row.start_measure, row.end_measure),
                        row.load,
                        binding.accepted_system_capacity.saturating_sub(row.load),
                        if split_points.is_empty() {
                            "none".to_string()
                        } else {
                            format_usize_list(&split_points)
                        },
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            let affected_rows = binding
                .systems
                .iter()
                .filter_map(|row| {
                    let split_points = audit
                        .inserted_system_breaks
                        .iter()
                        .copied()
                        .filter(|measure| {
                            row.start_measure < *measure && *measure <= row.end_measure
                        })
                        .collect::<Vec<_>>();
                    if split_points.is_empty() {
                        None
                    } else {
                        Some(format!(
                            "{}=load{}/headroom{}/insertedAt{}",
                            format_measure_range(row.start_measure, row.end_measure),
                            row.load,
                            binding.accepted_system_capacity.saturating_sub(row.load),
                            format_usize_list(&split_points),
                        ))
                    }
                })
                .collect::<Vec<_>>();
            let measure_rows = binding
                .measure_demands
                .iter()
                .enumerate()
                .map(|(index, demand)| format!("{}={}", index + 1, demand))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "RendererSystemBreakReadbackDemandBinding: Status BOUND | Source {} | AcceptedSystemCapacity {} | PlannedSystemEvidence {} | BreakAffectedSystems {} | MeasureDemands {} | MeasureDemandRecompose exact | ModelRecompute nein | ThresholdChange nein | PolicyWrite nein",
                source,
                binding.accepted_system_capacity,
                system_rows,
                if affected_rows.is_empty() {
                    "none".to_string()
                } else {
                    affected_rows.join(",")
                },
                measure_rows,
            )
        }
        Err(error) => format!(
            "RendererSystemBreakReadbackDemandBinding: Status UNAVAILABLE | Reason {} | ModelRecompute nein | ThresholdChange nein | PolicyWrite nein",
            one_line(&error.to_string()),
        ),
    }
}



fn current_process_rhythmic_shape_projection(
    audit: &RendererSystemBreakReadbackAudit,
) -> String {
    match current_process_planned_musicxml()
        .and_then(|xml| rhythmic_column_evidence_from_musicxml(&xml))
        .and_then(|evidence| validate_rhythmic_column_evidence(audit, evidence))
    {
        Ok(evidence) => rhythmic_shape_projection(audit, &evidence),
        Err(error) => format!(
            "RendererSystemBreakReadbackRhythmicShape: Status UNAVAILABLE | Basis musicxml_timeline_columns_v1 | Reason {} | DemandModelWrite nein | CapacityChange nein | PolicyWrite nein",
            one_line(&error.to_string()),
        ),
    }
}

fn current_process_planned_musicxml() -> anyhow::Result<String> {
    let cli = crate::cli::Cli::normalized_from_env();
    if cli.all || cli.corpus_emergence || cli.input.is_none() {
        anyhow::bail!("current_process_musicxml_unavailable");
    }
    let source = crate::source::resolve_source_plan(
        cli.input.as_deref(),
        cli.out.as_deref(),
        cli.musicxml_out.as_deref(),
        cli.pdf_out.as_deref(),
    )?;
    let planned_musicxml = if source.render_musicxml_out.is_file() {
        source.render_musicxml_out.as_path()
    } else if source.musicxml_out.is_file() {
        source.musicxml_out.as_path()
    } else {
        anyhow::bail!("planned_musicxml_missing");
    };
    Ok(std::fs::read_to_string(planned_musicxml)?)
}

fn rhythmic_shape_projection(
    audit: &RendererSystemBreakReadbackAudit,
    evidence: &[MeasureRhythmicColumnEvidence],
) -> String {
    let ranges = match system_ranges(&audit.planned_system_starts, audit.measure_count) {
        Ok(ranges) => ranges,
        Err(error) => {
            return format!(
                "RendererSystemBreakReadbackRhythmicShape: Status UNAVAILABLE | Basis musicxml_timeline_columns_v1 | Reason {} | DemandModelWrite nein | CapacityChange nein | PolicyWrite nein",
                one_line(&error.to_string()),
            );
        }
    };

    let mut split_lengths = Vec::new();
    let mut all_inserted_before_final_measure = !audit.inserted_system_breaks.is_empty();
    let system_rows = ranges
        .iter()
        .map(|(start, end)| {
            let rows = &evidence[start.saturating_sub(1)..*end];
            let columns = rows
                .iter()
                .map(|row| row.columns.to_string())
                .collect::<Vec<_>>()
                .join("+");
            let note_elements = rows
                .iter()
                .map(|row| row.note_elements.to_string())
                .collect::<Vec<_>>()
                .join("+");
            let pitched_columns = rows
                .iter()
                .map(|row| row.pitched_columns.to_string())
                .collect::<Vec<_>>()
                .join("+");
            let max_stack = rows
                .iter()
                .map(|row| row.max_notes_per_column)
                .max()
                .unwrap_or(0);
            let clearance_edges = rows
                .iter()
                .map(|row| row.following_accidental_clearance_edges.to_string())
                .collect::<Vec<_>>()
                .join("+");
            let split_points = audit
                .inserted_system_breaks
                .iter()
                .copied()
                .filter(|measure| *start < *measure && *measure <= *end)
                .collect::<Vec<_>>();
            if !split_points.is_empty() {
                split_lengths.push(end.saturating_sub(*start).saturating_add(1));
                all_inserted_before_final_measure &=
                    split_points.iter().all(|measure| *measure == *end);
            }
            format!(
                "{}=len{}/cols{}/pitchedCols{}/notes{}/maxStack{}/clearanceEdges{}/split{}",
                format_measure_range(*start, *end),
                end.saturating_sub(*start).saturating_add(1),
                columns,
                pitched_columns,
                note_elements,
                max_stack,
                clearance_edges,
                if split_points.is_empty() {
                    "none".to_string()
                } else {
                    format_usize_list(&split_points)
                },
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let two_measure_systems = ranges
        .iter()
        .filter(|(start, end)| end.saturating_sub(*start).saturating_add(1) == 2)
        .count();
    let three_measure_systems = ranges
        .iter()
        .filter(|(start, end)| end.saturating_sub(*start).saturating_add(1) == 3)
        .count();
    let split_two_measure_systems = split_lengths.iter().filter(|length| **length == 2).count();
    let split_three_measure_systems = split_lengths.iter().filter(|length| **length == 3).count();

    format!(
        "RendererSystemBreakReadbackRhythmicShape: Status OBSERVED | Basis musicxml_timeline_columns_v1 | TwoMeasureSystems {} | SplitTwoMeasureSystems {} | ThreeMeasureSystems {} | SplitThreeMeasureSystems {} | AllInsertedBreaksBeforeFinalMeasure {} | PlannedSystemShapeEvidence {} | DemandModelWrite nein | CapacityChange nein | PolicyWrite nein",
        two_measure_systems,
        split_two_measure_systems,
        three_measure_systems,
        split_three_measure_systems,
        if audit.inserted_system_breaks.is_empty() {
            "n/a"
        } else if all_inserted_before_final_measure {
            "ja"
        } else {
            "nein"
        },
        system_rows,
    )
}

fn validate_rhythmic_column_evidence(
    audit: &RendererSystemBreakReadbackAudit,
    evidence: Vec<MeasureRhythmicColumnEvidence>,
) -> anyhow::Result<Vec<MeasureRhythmicColumnEvidence>> {
    if evidence.len() != audit.measure_count {
        anyhow::bail!(
            "RendererSystemBreakReadbackRhythmicShape: Measure-Evidence {} stimmt nicht mit MeasureCount {} ueberein",
            evidence.len(),
            audit.measure_count
        );
    }
    for (index, row) in evidence.iter().enumerate() {
        let expected_measure = index + 1;
        if row.measure != expected_measure {
            anyhow::bail!(
                "RendererSystemBreakReadbackRhythmicShape: Measure {} steht an Slot {}",
                row.measure,
                expected_measure
            );
        }
        if row.pitched_columns > row.columns || row.max_notes_per_column > row.note_elements {
            anyhow::bail!(
                "RendererSystemBreakReadbackRhythmicShape: ungueltige Spalten-Evidence in Takt {}",
                row.measure
            );
        }
    }
    Ok(evidence)
}

fn rhythmic_column_evidence_from_musicxml(
    xml: &str,
) -> anyhow::Result<Vec<MeasureRhythmicColumnEvidence>> {
    crate::final_notation_timeline_facts::observe_final_notation_timeline_measure_facts(xml)
}

fn parse_planner_demand_binding(line: &str) -> anyhow::Result<PlannerDemandBinding> {
    const JOINT_PREFIX: &str =
        "ETUSCHLOPF: HorizontalLayoutGlobalPageSystemPreferenceCandidate:";
    const FINAL_SELECTED_PREFIX: &str =
        "ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding:";
    let prefix = if line.starts_with(FINAL_SELECTED_PREFIX) {
        FINAL_SELECTED_PREFIX
    } else if line.starts_with(JOINT_PREFIX) {
        JOINT_PREFIX
    } else {
        anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: unbekannte Planner-Demand-Evidence"
        );
    };
    let fields = line[prefix.len()..]
        .split(" | ")
        .map(str::trim)
        .collect::<Vec<_>>();
    let accepted_system_capacity =
        parse_unique_usize_field(&fields, "AcceptedSystemCapacity ")?;
    if accepted_system_capacity == 0 {
        anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: AcceptedSystemCapacity ist null"
        );
    }
    let system_loads = parse_unique_text_field(&fields, "SelectedSystemLoads ")?;
    if system_loads == "none" {
        anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: SelectedSystemLoads fehlen"
        );
    }
    let mut systems = Vec::new();
    for token in system_loads.split(',') {
        let (range, load) = token.split_once('=').ok_or_else(|| {
            anyhow::anyhow!(
                "RendererSystemBreakReadbackDemandBinding: ungueltige Systemlast {token}"
            )
        })?;
        let load = load.parse::<usize>().map_err(|_| {
            anyhow::anyhow!(
                "RendererSystemBreakReadbackDemandBinding: ungueltige Last {load}"
            )
        })?;
        let (start_measure, end_measure) = parse_measure_range(range)?;
        systems.push(PlannerSystemLoadEvidence {
            start_measure,
            end_measure,
            load,
        });
    }

    let measure_demands_text =
        parse_unique_text_field(&fields, "SelectedMeasureDemands ")?;
    if measure_demands_text == "none" {
        anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: SelectedMeasureDemands fehlen"
        );
    }
    let mut measure_demands = Vec::new();
    for (index, token) in measure_demands_text.split(',').enumerate() {
        let (measure, demand) = token.split_once('=').ok_or_else(|| {
            anyhow::anyhow!(
                "RendererSystemBreakReadbackDemandBinding: ungueltiger Measure-Demand {token}"
            )
        })?;
        let measure = measure.parse::<usize>().map_err(|_| {
            anyhow::anyhow!(
                "RendererSystemBreakReadbackDemandBinding: ungueltige Measure-Nummer {measure}"
            )
        })?;
        let expected_measure = index + 1;
        if measure != expected_measure {
            anyhow::bail!(
                "RendererSystemBreakReadbackDemandBinding: Measure-Demand {} steht an Slot {}",
                measure,
                expected_measure
            );
        }
        let demand = demand.parse::<usize>().map_err(|_| {
            anyhow::anyhow!(
                "RendererSystemBreakReadbackDemandBinding: ungueltiger Measure-Demand {demand}"
            )
        })?;
        measure_demands.push(demand);
    }

    Ok(PlannerDemandBinding {
        accepted_system_capacity,
        systems,
        measure_demands,
    })
}

fn validate_planner_demand_binding(
    audit: &RendererSystemBreakReadbackAudit,
    binding: PlannerDemandBinding,
) -> anyhow::Result<PlannerDemandBinding> {
    let expected_ranges = system_ranges(&audit.planned_system_starts, audit.measure_count)?;
    if binding.systems.len() != expected_ranges.len() {
        anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: Planner-Systemzeilen {} stimmen nicht mit PlannedSystems {} ueberein",
            binding.systems.len(),
            expected_ranges.len()
        );
    }
    if binding.measure_demands.len() != audit.measure_count {
        anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: Measure-Demands {} stimmen nicht mit MeasureCount {} ueberein",
            binding.measure_demands.len(),
            audit.measure_count
        );
    }
    for (row, expected) in binding.systems.iter().zip(expected_ranges.iter()) {
        if (row.start_measure, row.end_measure) != *expected {
            anyhow::bail!(
                "RendererSystemBreakReadbackDemandBinding: Planner-System {} stimmt nicht mit Readback-Plan {} ueberein",
                format_measure_range(row.start_measure, row.end_measure),
                format_measure_range(expected.0, expected.1),
            );
        }
        if row.load > binding.accepted_system_capacity {
            anyhow::bail!(
                "RendererSystemBreakReadbackDemandBinding: System {} Last {} ueberschreitet Planner-Capacity {}",
                format_measure_range(row.start_measure, row.end_measure),
                row.load,
                binding.accepted_system_capacity
            );
        }
        let recomposed_load = binding.measure_demands
            [row.start_measure.saturating_sub(1)..row.end_measure]
            .iter()
            .copied()
            .fold(0usize, usize::saturating_add);
        if recomposed_load != row.load {
            anyhow::bail!(
                "RendererSystemBreakReadbackDemandBinding: Measure-Demand-Summe {} fuer System {} stimmt nicht mit Planner-Systemlast {} ueberein",
                recomposed_load,
                format_measure_range(row.start_measure, row.end_measure),
                row.load
            );
        }
    }
    Ok(binding)
}

fn parse_unique_usize_field(fields: &[&str], prefix: &str) -> anyhow::Result<usize> {
    let value = parse_unique_text_field(fields, prefix)?;
    value.parse::<usize>().map_err(|_| {
        anyhow::anyhow!(
            "RendererSystemBreakReadbackDemandBinding: Feld {} ist nicht numerisch",
            prefix.trim()
        )
    })
}

fn parse_unique_text_field<'a>(fields: &'a [&str], prefix: &str) -> anyhow::Result<&'a str> {
    let matches = fields
        .iter()
        .filter_map(|field| field.strip_prefix(prefix))
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [value] if !value.is_empty() => Ok(*value),
        [] => anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: Feld {} fehlt",
            prefix.trim()
        ),
        _ => anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: Feld {} ist mehrdeutig",
            prefix.trim()
        ),
    }
}

fn parse_measure_range(value: &str) -> anyhow::Result<(usize, usize)> {
    let (start, end) = if let Some((start, end)) = value.split_once('-') {
        (
            start.parse::<usize>().map_err(|_| {
                anyhow::anyhow!(
                    "RendererSystemBreakReadbackDemandBinding: ungueltiger Systemstart {start}"
                )
            })?,
            end.parse::<usize>().map_err(|_| {
                anyhow::anyhow!(
                    "RendererSystemBreakReadbackDemandBinding: ungueltiges Systemende {end}"
                )
            })?,
        )
    } else {
        let measure = value.parse::<usize>().map_err(|_| {
            anyhow::anyhow!(
                "RendererSystemBreakReadbackDemandBinding: ungueltige Systemposition {value}"
            )
        })?;
        (measure, measure)
    };
    if start == 0 || start > end {
        anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: ungueltiger Systembereich {value}"
        );
    }
    Ok((start, end))
}

fn system_ranges(starts: &[usize], measure_count: usize) -> anyhow::Result<Vec<(usize, usize)>> {
    if starts.is_empty() || starts[0] != 1 || measure_count == 0 {
        anyhow::bail!(
            "RendererSystemBreakReadbackDemandBinding: PlannedSystemStarts sind unvollstaendig"
        );
    }
    let mut ranges = Vec::with_capacity(starts.len());
    for (index, start) in starts.iter().copied().enumerate() {
        let end = starts
            .get(index + 1)
            .map_or(measure_count, |next| next.saturating_sub(1));
        if start == 0 || start > end || end > measure_count {
            anyhow::bail!(
                "RendererSystemBreakReadbackDemandBinding: ungueltiger PlannedSystem-Bereich"
            );
        }
        ranges.push((start, end));
    }
    Ok(ranges)
}

fn format_measure_range(start: usize, end: usize) -> String {
    if start == end {
        start.to_string()
    } else {
        format!("{start}-{end}")
    }
}

fn current_process_audit() -> anyhow::Result<Option<RendererSystemBreakReadbackAudit>> {
    let cli = crate::cli::Cli::normalized_from_env();
    if cli.all || cli.corpus_emergence || cli.no_pdf || cli.input.is_none() {
        return Ok(None);
    }
    let source = crate::source::resolve_source_plan(
        cli.input.as_deref(),
        cli.out.as_deref(),
        cli.musicxml_out.as_deref(),
        cli.pdf_out.as_deref(),
    )?;
    if !source.pdf_out.is_file() {
        return Ok(None);
    }
    let planned_musicxml = if source.render_musicxml_out.is_file() {
        source.render_musicxml_out.as_path()
    } else if source.musicxml_out.is_file() {
        source.musicxml_out.as_path()
    } else {
        return Ok(None);
    };
    audit_paths(planned_musicxml, &source.pdf_out).map(Some)
}

fn planned_breaks_from_musicxml(xml: &str) -> anyhow::Result<(usize, Vec<usize>, Vec<usize>)> {
    let document = Document::parse(xml)?;
    let part = document
        .descendants()
        .find(|node| node.has_tag_name("part"))
        .ok_or_else(|| anyhow::anyhow!("RendererSystemBreakReadback: MusicXML part fehlt"))?;
    let measures = part
        .children()
        .filter(|node| node.is_element() && node.has_tag_name("measure"))
        .collect::<Vec<_>>();
    if measures.is_empty() {
        anyhow::bail!("RendererSystemBreakReadback: MusicXML besitzt keine measures");
    }
    for (index, measure) in measures.iter().enumerate() {
        let slot = index + 1;
        let number = measure
            .attribute("number")
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "RendererSystemBreakReadback: nichtnumerische Measure-Nummer an Slot {slot}"
                )
            })?;
        if number != slot {
            anyhow::bail!(
                "RendererSystemBreakReadback: Measure-Nummer {} stimmt nicht mit Slot {} ueberein",
                number,
                slot
            );
        }
    }
    let mut system_starts = vec![1usize];
    let mut page_starts = vec![1usize];
    for (index, measure) in measures.iter().enumerate().skip(1) {
        let slot = index + 1;
        let mut starts_system = false;
        let mut starts_page = false;
        for print in measure
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("print"))
        {
            starts_system |= print.attribute("new-system") == Some("yes");
            starts_page |= print.attribute("new-page") == Some("yes");
        }
        if starts_system || starts_page {
            system_starts.push(slot);
        }
        if starts_page {
            page_starts.push(slot);
        }
    }
    Ok((measures.len(), system_starts, page_starts))
}

pub(super) fn extract_pdf_text_events(bytes: &[u8]) -> anyhow::Result<(Vec<PdfTextEvent>, usize)> {
    let objects = parse_pdf_objects(bytes)?;
    let page_ids = page_ids_in_order(&objects)?;
    let page_count = page_ids.len();
    let mut out = Vec::new();
    for (page_offset, page_id) in page_ids.iter().enumerate() {
        let page = objects
            .get(page_id)
            .ok_or_else(|| anyhow::anyhow!("PDF page object {page_id} fehlt"))?;
        let page_dict = page.dictionary_text();
        let contents_id = parse_single_ref_after(page_dict, "/Contents")
            .ok_or_else(|| anyhow::anyhow!("PDF page {page_id} Contents fehlt"))?;
        let resources_id = parse_single_ref_after(page_dict, "/Resources")
            .ok_or_else(|| anyhow::anyhow!("PDF page {page_id} Resources fehlt"))?;
        let page_height = parse_media_box_height(page_dict).unwrap_or(792.0);
        let resources = objects
            .get(&resources_id)
            .ok_or_else(|| anyhow::anyhow!("PDF resources object {resources_id} fehlt"))?;
        let font_refs = parse_font_refs(resources.dictionary_text());
        let mut font_maps = BTreeMap::<String, BTreeMap<u16, char>>::new();
        let mut head_metrics = BTreeMap::new();
        for (font_name, font_id) in font_refs {
            let Some(font_obj) = objects.get(&font_id) else {
                continue;
            };
            let Some(to_unicode_id) =
                parse_single_ref_after(font_obj.dictionary_text(), "/ToUnicode")
            else {
                continue;
            };
            let Some(cmap_obj) = objects.get(&to_unicode_id) else {
                continue;
            };
            let Some(cmap_stream) = cmap_obj.decoded_stream()? else {
                continue;
            };
            let cmap = parse_to_unicode_cmap(&cmap_stream);
            if let Some(metrics) = pdf_notehead_metrics(font_obj, &objects, &cmap) {
                head_metrics.insert(font_name.clone(), metrics);
            }
            if let Some(metrics) = pdf_ottava_metrics(font_obj, &objects, &cmap) {
                head_metrics.entry(font_name.clone()).or_default().extend(metrics);
            }
            font_maps.insert(font_name, cmap);
        }
        let contents = objects
            .get(&contents_id)
            .ok_or_else(|| anyhow::anyhow!("PDF contents object {contents_id} fehlt"))?
            .decoded_stream()?
            .ok_or_else(|| {
                anyhow::anyhow!("PDF contents object {contents_id} besitzt keinen stream")
            })?;
        extract_content_text_events(
            &contents,
            page_offset + 1,
            page_height,
            &font_maps,
            &head_metrics,
            &mut out,
        )?;
    }
    Ok((out, page_count))
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RendererPrimaryBeamCohortGeometry {
    pub(crate) visible_fragments: usize,
    pub(crate) connected_adjacent_pairs: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RendererPrimaryBeamGeometryReadback {
    stem_geometry_by_occurrence: BTreeMap<(usize, u8, usize), RendererStemGeometry>,
    beam_spans_by_measure_staff: BTreeMap<(usize, u8), Vec<RendererBeamSpan>>,
    pub(crate) measure_count: usize,
    pub(crate) bound_measure_staff_cells: usize,
    pub(crate) unresolved_measure_staff_cells: usize,
}

impl RendererPrimaryBeamGeometryReadback {
    pub(crate) fn cohort_geometry(
        &self,
        measure: usize,
        staff: u8,
        pitched_ordinals: &[usize],
    ) -> Option<RendererPrimaryBeamCohortGeometry> {
        if pitched_ordinals.len() < 2 {
            return None;
        }
        let member_stems = pitched_ordinals
            .iter()
            .map(|pitched_ordinal| {
                self.stem_geometry_by_occurrence
                    .get(&(measure, staff, *pitched_ordinal))
                    .copied()
            })
            .collect::<Option<Vec<_>>>()?;
        if member_stems.windows(2).any(|pair| pair[0].x >= pair[1].x) {
            return None;
        }
        let spans = self
            .beam_spans_by_measure_staff
            .get(&(measure, staff))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let mut connected_adjacent_pairs = 0usize;
        for pair in member_stems.windows(2) {
            let left = pair[0];
            let right = pair[1];
            let connected = spans.iter().any(|span| {
                span.connects_stem(left)
                    && span.connects_stem(right)
                    && span.x_left <= left.x + 0.9
                    && span.x_right >= right.x - 0.9
            });
            if connected {
                connected_adjacent_pairs = connected_adjacent_pairs.saturating_add(1);
            }
        }
        Some(RendererPrimaryBeamCohortGeometry {
            visible_fragments: pitched_ordinals
                .len()
                .saturating_sub(connected_adjacent_pairs),
            connected_adjacent_pairs,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RendererStemGeometry {
    x: f64,
    y_min: f64,
    y_max: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RendererBeamSpan {
    x_left: f64,
    x_right: f64,
    y_left: f64,
    y_right: f64,
}

impl RendererBeamSpan {
    fn y_at(self, x: f64) -> f64 {
        if (self.x_right - self.x_left).abs() <= f64::EPSILON {
            return (self.y_left + self.y_right) / 2.0;
        }
        let ratio = (x - self.x_left) / (self.x_right - self.x_left);
        self.y_left + ratio * (self.y_right - self.y_left)
    }

    fn connects_stem(self, stem: RendererStemGeometry) -> bool {
        if stem.x < self.x_left - 0.9 || stem.x > self.x_right + 0.9 {
            return false;
        }
        let y = self.y_at(stem.x);
        y >= stem.y_min - 1.5 && y <= stem.y_max + 1.5
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct PdfStrokeSegment {
    pub(super) page_index: usize,
    pub(super) x1: f64,
    pub(super) y1_top: f64,
    pub(super) x2: f64,
    pub(super) y2_top: f64,
    pub(super) width: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct PdfFilledPolygon {
    page_index: usize,
    points: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct RendererStaffCell {
    pub(super) page_index: usize,
    pub(super) x_left: f64,
    pub(super) x_right: f64,
    pub(super) staff: u8,
    pub(super) measure: usize,
    pub(super) line_y_top: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RenderStemOccurrence {
    measure: usize,
    staff: u8,
    onset: i64,
    pitched_ordinal: usize,
    head_steps: Vec<i32>,
    head_pitches: Vec<Option<i32>>,
}

pub(crate) fn primary_beam_geometry_readback_xml_pdf(
    render_xml: &str,
    rendered_pdf: &Path,
) -> anyhow::Result<RendererPrimaryBeamGeometryReadback> {
    let pdf_bytes = std::fs::read(rendered_pdf)?;
    primary_beam_geometry_readback(render_xml, &pdf_bytes)
}

fn primary_beam_geometry_readback(
    render_xml: &str,
    pdf_bytes: &[u8],
) -> anyhow::Result<RendererPrimaryBeamGeometryReadback> {
    let (measure_count, stem_occurrences) = render_stem_occurrences(render_xml)?;
    let (segments, polygons) = extract_pdf_vector_geometry(pdf_bytes)?;
    let cells = renderer_staff_cells(&segments, measure_count)?;
    let (head_events, _) = extract_pdf_text_events(pdf_bytes)?;
    let mut expected_by_cell = BTreeMap::<(usize, u8), Vec<RenderStemOccurrence>>::new();
    for occurrence in stem_occurrences {
        expected_by_cell
            .entry((occurrence.measure, occurrence.staff))
            .or_default()
            .push(occurrence);
    }
    for occurrences in expected_by_cell.values_mut() {
        occurrences.sort_by_key(|row| (row.onset, row.pitched_ordinal));
    }

    let mut stem_geometry_by_occurrence = BTreeMap::new();
    let mut beam_spans_by_measure_staff = BTreeMap::new();
    let mut bound_measure_staff_cells = 0usize;
    let mut unresolved_measure_staff_cells = 0usize;

    let mut head_bindings = Vec::new();
    for cell in &cells {
        let key = (cell.measure, cell.staff);
        let expected = expected_by_cell.get(&key).cloned().unwrap_or_default();
        let stems = renderer_head_binding(cell, &segments, &head_events, &expected);
        if let Ok(stems) = &stems {
            bound_measure_staff_cells = bound_measure_staff_cells.saturating_add(1);
            for (occurrence, stem) in expected.iter().zip(stems.iter()) {
                head_bindings.push((cell.page_index, occurrence.clone(), stem.heads.clone()));
                stem_geometry_by_occurrence.insert(
                    (occurrence.measure, occurrence.staff, occurrence.pitched_ordinal),
                    stem.geometry,
                );
            }
        } else if !expected.is_empty() {
            unresolved_measure_staff_cells = unresolved_measure_staff_cells.saturating_add(1);
            eprintln!("RendererPrimaryBeamGeometryUnresolved: measure={}; staff={}; reason={}; authority=none", cell.measure, cell.staff, stems.unwrap_err());
        }
        let spans = renderer_cell_beam_spans(cell, &polygons);
        if !spans.is_empty() {
            beam_spans_by_measure_staff.insert(key, spans);
        }
    }

    let bindings = stem_geometry_by_occurrence.iter().map(|(key, stem)| (*key, *stem)).collect::<Vec<_>>();
    let mut invalid = BTreeSet::new();
    for (index, (left_key, left)) in bindings.iter().enumerate() {
        let left_page = cells.iter().find(|c| (c.measure, c.staff) == (left_key.0, left_key.1)).map(|c| c.page_index);
        for (right_key, right) in bindings.iter().skip(index + 1) {
            let right_page = cells.iter().find(|c| (c.measure, c.staff) == (right_key.0, right_key.1)).map(|c| c.page_index);
            if left_page == right_page && same_physical_stem(*left, *right) {
                invalid.insert((left_key.0, left_key.1));
                invalid.insert((right_key.0, right_key.1));
            }
        }
    }
    for (index, (page, left, left_heads)) in head_bindings.iter().enumerate() {
        for (other_page, right, right_heads) in head_bindings.iter().skip(index+1) {
            if page == other_page && left_heads.iter().any(|a| right_heads.iter().any(|b| !renderer_shared_head_compatible(a, b, left, right))) {
                invalid.insert((left.measure, left.staff));
                invalid.insert((right.measure, right.staff));
            }
        }
    }
    for cell in invalid {
        eprintln!("RendererPrimaryBeamGeometryUnresolved: measure={}; staff={}; reason=physical_stem_or_head_reused; authority=none", cell.0, cell.1);
        stem_geometry_by_occurrence.retain(|key, _| (key.0, key.1) != cell);
        bound_measure_staff_cells = bound_measure_staff_cells.saturating_sub(1);
        unresolved_measure_staff_cells = unresolved_measure_staff_cells.saturating_add(1);
    }

    Ok(RendererPrimaryBeamGeometryReadback {
        stem_geometry_by_occurrence,
        beam_spans_by_measure_staff,
        measure_count,
        bound_measure_staff_cells,
        unresolved_measure_staff_cells,
    })
}

fn render_stem_occurrences(
    xml: &str,
) -> anyhow::Result<(usize, Vec<RenderStemOccurrence>)> {
    let document = Document::parse(xml)?;
    let part = document
        .descendants()
        .find(|node| node.is_element() && node.has_tag_name("part"))
        .ok_or_else(|| anyhow::anyhow!("RendererPrimaryBeamGeometryReadback: MusicXML part fehlt"))?;
    let measures = part
        .children()
        .filter(|node| node.is_element() && node.has_tag_name("measure"))
        .collect::<Vec<_>>();
    if measures.is_empty() {
        anyhow::bail!("RendererPrimaryBeamGeometryReadback: MusicXML measures fehlen");
    }
    let mut out = Vec::<RenderStemOccurrence>::new();
    let mut clefs = BTreeMap::<u8, (i32, i32)>::new();
    let mut ottavas = BTreeMap::<(u8, u16), Option<i32>>::new();
    for (measure_index, measure) in measures.iter().enumerate() {
        let measure_number = measure
            .attribute("number")
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "RendererPrimaryBeamGeometryReadback: ungueltige Measure-Nummer an Slot {}",
                    measure_index + 1
                )
            })?;
        if measure_number != measure_index + 1 {
            anyhow::bail!(
                "RendererPrimaryBeamGeometryReadback: Measure {} steht an Slot {}",
                measure_number,
                measure_index + 1
            );
        }
        let clefs_before = clefs.clone();
        let ottavas_before = ottavas.clone();
        let start_occurrence = out.len();
        let mut surface_changes = Vec::new();
        let mut cursor = 0_i64;
        let mut last_note_onset = 0_i64;
        let mut pitched_ordinal = 0usize;
        let mut last_stem_index: Option<usize> = None;
        for node in measure.children().filter(|node| node.is_element()) {
            if node.has_tag_name("backup") {
                last_stem_index = None;
                cursor = cursor.saturating_sub(pdf_readback_child_duration(node).max(0));
                continue;
            }
            if node.has_tag_name("forward") {
                last_stem_index = None;
                cursor = cursor.saturating_add(pdf_readback_child_duration(node).max(0));
                continue;
            }
            if node.has_tag_name("attributes") {
                for clef in node.children().filter(|n| n.has_tag_name("clef")) {
                    let staff = clef.attribute("number").unwrap_or("1").parse::<u8>().unwrap_or(0);
                    let line = pdf_readback_child_text(clef, "line").and_then(|x| x.parse::<i32>().ok());
                    let octave = pdf_readback_child_text(clef, "clef-octave-change").and_then(|x| x.parse::<i32>().ok()).unwrap_or(0);
                    let reference = match pdf_readback_child_text(clef, "sign").as_deref() {
                        Some("G") => Some(32), Some("F") => Some(24), Some("C") => Some(28), _ => None,
                    };
                    let clef = reference.zip(line.filter(|x| (1..=5).contains(x))).filter(|_| (-2..=2).contains(&octave))
                        .map(|(reference,line)| (reference-2*(line-1), octave*7));
                    surface_changes.push((cursor, RenderSurfaceChange::Clef(staff, clef)));
                }
            }
            if node.has_tag_name("direction") {
                let staff = pdf_readback_child_text(node, "staff").and_then(|x| x.parse::<u8>().ok()).unwrap_or(1);
                for shift in node.descendants().filter(|n| n.has_tag_name("octave-shift")) {
                    let size = shift.attribute("size").unwrap_or("8").parse::<i32>().unwrap_or(0);
                    let number = shift.attribute("number").unwrap_or("1").parse::<u16>().unwrap_or(0);
                    let steps = match size { 8 => Some(7), 15 => Some(14), _ => None };
                    let action = match shift.attribute("type") {
                        Some("down") => RenderOctaveChange::Start(steps),
                        Some("up") => RenderOctaveChange::Start(steps.map(|s| -s)),
                        Some("continue") => RenderOctaveChange::Continue,
                        Some("stop") => RenderOctaveChange::Stop,
                        _ => RenderOctaveChange::Start(None),
                    };
                    let offset = pdf_readback_child_text(node,"offset").and_then(|x|x.parse::<i64>().ok()).unwrap_or(0);
                    surface_changes.push((cursor.saturating_add(offset), RenderSurfaceChange::Octave(staff, number, action)));
                }
            }
            if !node.has_tag_name("note") {
                continue;
            }
            let duration = pdf_readback_child_duration(node).max(0);
            let is_chord = node
                .children()
                .any(|child| child.is_element() && child.has_tag_name("chord"));
            let onset = if is_chord { last_note_onset } else { cursor };
            if !is_chord {
                last_note_onset = onset;
                last_stem_index = None;
            }
            let has_pitch = node
                .children()
                .any(|child| child.is_element() && child.has_tag_name("pitch"));
            let event_pitched_ordinal = has_pitch.then_some(pitched_ordinal);
            if has_pitch {
                pitched_ordinal = pitched_ordinal.saturating_add(1);
            }
            let visible = node.attribute("print-object") != Some("no");
            let stem = pdf_readback_child_text(node, "stem");
            let staff = pdf_readback_child_text(node, "staff").and_then(|value| value.parse::<u8>().ok()).unwrap_or(1);
            let head_step = node.children().find(|n| n.has_tag_name("pitch")).and_then(|pitch| {
                let step = match pdf_readback_child_text(pitch, "step").as_deref() {
                    Some("C")=>0,Some("D")=>1,Some("E")=>2,Some("F")=>3,Some("G")=>4,Some("A")=>5,Some("B")=>6,_=>return None,
                };
                let octave = pdf_readback_child_text(pitch, "octave")?.parse::<i32>().ok().filter(|x| (-2..=12).contains(x))?;
                Some(step + octave * 7)
            });
            let head_pitch = node.children().find(|n| n.has_tag_name("pitch")).and_then(|pitch| {
                let semitone = match pdf_readback_child_text(pitch, "step").as_deref() {
                    Some("C") => 0, Some("D") => 2, Some("E") => 4, Some("F") => 5,
                    Some("G") => 7, Some("A") => 9, Some("B") => 11, _ => return None,
                };
                let octave = pdf_readback_child_text(pitch, "octave")?.parse::<i32>().ok().filter(|n| (-2..=12).contains(n))?;
                let alter = pdf_readback_child_text(pitch, "alter").unwrap_or_else(|| "0".to_string()).parse::<i32>().ok().filter(|n| (-2..=2).contains(n))?;
                Some((octave + 1) * 12 + semitone + alter)
            });
            let head_visible = visible && pdf_readback_child_text(node, "notehead").as_deref() != Some("none");
            if is_chord && head_visible && has_pitch
                && let Some(owner) = last_stem_index.and_then(|i| out.get_mut(i)).filter(|o| o.measure == measure_number && o.staff == staff && o.onset == onset)
            {
                owner.head_steps.push(head_step.unwrap_or(i32::MIN / 4));
                owner.head_pitches.push(head_pitch);
            }
            if visible
                && has_pitch
                && !is_chord
                && matches!(stem.as_deref(), Some("up") | Some("down"))
            {
                let staff = pdf_readback_child_text(node, "staff")
                    .and_then(|value| value.parse::<u8>().ok())
                    .unwrap_or(1);
                if let Some(pitched_ordinal) = event_pitched_ordinal {
                    out.push(RenderStemOccurrence {
                        measure: measure_number,
                        staff,
                        onset,
                        pitched_ordinal,
                        head_steps: if head_visible { vec![head_step.unwrap_or(i32::MIN / 4)] } else { Vec::new() },
                        head_pitches: if head_visible { vec![head_pitch] } else { Vec::new() },
                    });
                    last_stem_index = Some(out.len()-1);
                }
            }
            if !is_chord {
                cursor = cursor.saturating_add(duration);
            }
        }
        surface_changes.sort_by_key(|(onset, _)| *onset);
        // Staff surface changes are timed facts. XML voice serialization order
        // must not leak a later clef/ottava into an earlier backed-up voice.
        for occurrence in &mut out[start_occurrence..] {
            let mut context_clefs = clefs_before.clone();
            let mut context_ottavas = ottavas_before.clone();
            for (_, change) in surface_changes.iter().filter(|(at,_)| *at <= occurrence.onset) {
                apply_render_surface_change(change, &mut context_clefs, &mut context_ottavas);
            }
            let shift = context_clefs.get(&occurrence.staff).and_then(|(bottom, clef_octave)| {
                let active = context_ottavas.iter().filter(|((staff,_),_)| *staff == occurrence.staff).map(|(_,amount)| *amount).collect::<Option<Vec<_>>>()?;
                // Multiple simultaneously active octave directions require a
                // separate renderer-profile proof; they are never summed here.
                if active.len() > 1 { return None; }
                Some(bottom + clef_octave + active.first().copied().unwrap_or(0))
            });
            for step in &mut occurrence.head_steps {
                *step = shift.and_then(|shift| step.checked_sub(shift)).unwrap_or(i32::MIN/4);
            }
        }
        clefs = clefs_before;
        ottavas = ottavas_before;
        for (_, change) in &surface_changes { apply_render_surface_change(change, &mut clefs, &mut ottavas); }
    }
    Ok((measures.len(), out))
}

fn pdf_readback_child_duration(node: roxmltree::Node<'_, '_>) -> i64 {
    pdf_readback_child_text(node, "duration")
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
}

fn pdf_readback_child_text(node: roxmltree::Node<'_, '_>, tag: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.has_tag_name(tag))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

pub(super) fn extract_pdf_vector_geometry(
    bytes: &[u8],
) -> anyhow::Result<(Vec<PdfStrokeSegment>, Vec<PdfFilledPolygon>)> {
    let objects = parse_pdf_objects(bytes)?;
    let page_ids = page_ids_in_order(&objects)?;
    let mut segments = Vec::new();
    let mut polygons = Vec::new();
    for (page_offset, page_id) in page_ids.iter().enumerate() {
        let page = objects
            .get(page_id)
            .ok_or_else(|| anyhow::anyhow!("PDF page object {page_id} fehlt"))?;
        let page_dict = page.dictionary_text();
        let contents_id = parse_single_ref_after(page_dict, "/Contents")
            .ok_or_else(|| anyhow::anyhow!("PDF page {page_id} Contents fehlt"))?;
        let page_height = parse_media_box_height(page_dict).unwrap_or(792.0);
        let contents = objects
            .get(&contents_id)
            .ok_or_else(|| anyhow::anyhow!("PDF contents object {contents_id} fehlt"))?
            .decoded_stream()?
            .ok_or_else(|| {
                anyhow::anyhow!("PDF contents object {contents_id} besitzt keinen stream")
            })?;
        extract_content_vector_geometry(
            &contents,
            page_offset + 1,
            page_height,
            &mut segments,
            &mut polygons,
        );
    }
    Ok((segments, polygons))
}

#[derive(Clone, Copy)]
struct PdfVectorGraphicsState {
    ctm: Matrix,
    line_width: f64,
}

fn extract_content_vector_geometry(
    bytes: &[u8],
    page_index: usize,
    page_height: f64,
    segments: &mut Vec<PdfStrokeSegment>,
    polygons: &mut Vec<PdfFilledPolygon>,
) {
    let text = String::from_utf8_lossy(bytes);
    let mut state = PdfVectorGraphicsState {
        ctm: Matrix::IDENTITY,
        line_width: 1.0,
    };
    let mut stack = Vec::<PdfVectorGraphicsState>::new();
    let mut path = Vec::<(f64, f64)>::new();
    let mut subpath_start = None::<(f64, f64)>;

    for raw_line in text.lines() {
        let tokens = raw_line.split_whitespace().collect::<Vec<_>>();
        for (index, token) in tokens.iter().copied().enumerate() {
            match token {
                "q" => stack.push(state),
                "Q" => {
                    if let Some(previous) = stack.pop() {
                        state = previous;
                    }
                }
                "cm" if index >= 6 => {
                    if let Some(matrix) = matrix_from_tokens(&tokens[index - 6..index]) {
                        state.ctm = state.ctm.multiply(matrix);
                    }
                }
                "w" if index >= 1 => {
                    if let Ok(width) = tokens[index - 1].parse::<f64>() {
                        state.line_width = width;
                    }
                }
                "m" if index >= 2 => {
                    if let (Ok(x), Ok(y)) = (
                        tokens[index - 2].parse::<f64>(),
                        tokens[index - 1].parse::<f64>(),
                    ) {
                        path.clear();
                        path.push((x, y));
                        subpath_start = Some((x, y));
                    }
                }
                "l" if index >= 2 => {
                    if let (Ok(x), Ok(y)) = (
                        tokens[index - 2].parse::<f64>(),
                        tokens[index - 1].parse::<f64>(),
                    ) {
                        path.push((x, y));
                    }
                }
                "h" => {
                    if let Some(start) = subpath_start
                        && path.last().copied() != Some(start)
                    {
                        path.push(start);
                    }
                }
                "S" | "s" => {
                    append_pdf_stroke_path(
                        &path,
                        state,
                        page_index,
                        page_height,
                        segments,
                    );
                    path.clear();
                    subpath_start = None;
                }
                "f" | "F" | "f*" => {
                    append_pdf_filled_path(
                        &path,
                        state.ctm,
                        page_index,
                        page_height,
                        polygons,
                    );
                    path.clear();
                    subpath_start = None;
                }
                "B" | "B*" | "b" | "b*" => {
                    append_pdf_stroke_path(
                        &path,
                        state,
                        page_index,
                        page_height,
                        segments,
                    );
                    append_pdf_filled_path(
                        &path,
                        state.ctm,
                        page_index,
                        page_height,
                        polygons,
                    );
                    path.clear();
                    subpath_start = None;
                }
                "n" => {
                    path.clear();
                    subpath_start = None;
                }
                _ => {}
            }
        }
    }
}

fn append_pdf_stroke_path(
    path: &[(f64, f64)],
    state: PdfVectorGraphicsState,
    page_index: usize,
    page_height: f64,
    out: &mut Vec<PdfStrokeSegment>,
) {
    for pair in path.windows(2) {
        let p1 = state.ctm.point(pair[0].0, pair[0].1);
        let p2 = state.ctm.point(pair[1].0, pair[1].1);
        let determinant = (state.ctm.a * state.ctm.d - state.ctm.b * state.ctm.c).abs();
        let scale = determinant.sqrt();
        out.push(PdfStrokeSegment {
            page_index,
            x1: p1.0,
            y1_top: page_height - p1.1,
            x2: p2.0,
            y2_top: page_height - p2.1,
            width: state.line_width * scale,
        });
    }
}

fn append_pdf_filled_path(
    path: &[(f64, f64)],
    ctm: Matrix,
    page_index: usize,
    page_height: f64,
    out: &mut Vec<PdfFilledPolygon>,
) {
    if path.len() < 4 {
        return;
    }
    let mut points = path
        .iter()
        .map(|(x, y)| {
            let point = ctm.point(*x, *y);
            (point.0, page_height - point.1)
        })
        .collect::<Vec<_>>();
    if points.first() == points.last() {
        points.pop();
    }
    if points.len() == 4 {
        out.push(PdfFilledPolygon { page_index, points });
    }
}

pub(super) fn renderer_staff_cells(
    segments: &[PdfStrokeSegment],
    measure_count: usize,
) -> anyhow::Result<Vec<RendererStaffCell>> {
    #[derive(Clone, Copy)]
    struct StaffLine {
        page_index: usize,
        x_left: f64,
        x_right: f64,
        y_top: f64,
    }

    let mut lines = segments
        .iter()
        .filter_map(|segment| {
            let dx = (segment.x2 - segment.x1).abs();
            let dy = (segment.y2_top - segment.y1_top).abs();
            (dy <= 0.2 && dx >= 40.0 && (0.35..=0.70).contains(&segment.width)).then_some(
                StaffLine {
                    page_index: segment.page_index,
                    x_left: segment.x1.min(segment.x2),
                    x_right: segment.x1.max(segment.x2),
                    y_top: (segment.y1_top + segment.y2_top) / 2.0,
                },
            )
        })
        .collect::<Vec<_>>();
    lines.sort_by(|left, right| {
        left.page_index
            .cmp(&right.page_index)
            .then_with(|| left.x_left.total_cmp(&right.x_left))
            .then_with(|| left.x_right.total_cmp(&right.x_right))
            .then_with(|| left.y_top.total_cmp(&right.y_top))
    });

    let mut staff_cells = Vec::<RendererStaffCell>::new();
    let mut consumed = vec![false; lines.len()];
    for index in 0..lines.len() {
        if consumed[index] {
            continue;
        }
        let anchor = lines[index];
        let mut matching = lines
            .iter()
            .enumerate()
            .filter(|(candidate_index, candidate)| {
                !consumed[*candidate_index]
                    && candidate.page_index == anchor.page_index
                    && (candidate.x_left - anchor.x_left).abs() <= 0.8
                    && (candidate.x_right - anchor.x_right).abs() <= 0.8
            })
            .map(|(candidate_index, candidate)| (candidate_index, *candidate))
            .collect::<Vec<_>>();
        matching.sort_by(|left, right| left.1.y_top.total_cmp(&right.1.y_top));
        let mut start = 0usize;
        while start + 4 < matching.len() {
            let window = &matching[start..start + 5];
            let spacing_ok = window
                .windows(2)
                .all(|pair| (4.4..=5.5).contains(&(pair[1].1.y_top - pair[0].1.y_top)));
            if spacing_ok {
                let mut line_y_top = window.iter().map(|(_, line)| line.y_top).collect::<Vec<_>>();
                line_y_top.sort_by(f64::total_cmp);
                staff_cells.push(RendererStaffCell {
                    page_index: anchor.page_index,
                    x_left: anchor.x_left,
                    x_right: anchor.x_right,
                    staff: 0,
                    measure: 0,
                    line_y_top,
                });
                for (candidate_index, _) in window {
                    consumed[*candidate_index] = true;
                }
                start += 5;
            } else {
                start += 1;
            }
        }
    }

    let mut measure_cells = Vec::<(RendererStaffCell, RendererStaffCell)>::new();
    let mut paired = vec![false; staff_cells.len()];
    for index in 0..staff_cells.len() {
        if paired[index] {
            continue;
        }
        let anchor = &staff_cells[index];
        let anchor_center = renderer_staff_center(anchor);
        let candidates = staff_cells
            .iter()
            .enumerate()
            .filter(|(candidate_index, candidate)| {
                *candidate_index != index
                    && !paired[*candidate_index]
                    && candidate.page_index == anchor.page_index
                    && (candidate.x_left - anchor.x_left).abs() <= 1.0
                    && (candidate.x_right - anchor.x_right).abs() <= 1.0
                    && (25.0..=80.0).contains(
                        &(renderer_staff_center(candidate) - anchor_center).abs(),
                    )
            })
            .min_by(|left, right| {
                (renderer_staff_center(left.1) - anchor_center)
                    .abs()
                    .total_cmp(&(renderer_staff_center(right.1) - anchor_center).abs())
            });
        let Some((candidate_index, candidate)) = candidates else {
            continue;
        };
        paired[index] = true;
        paired[candidate_index] = true;
        let (upper, lower) = if renderer_staff_center(anchor) < renderer_staff_center(candidate) {
            (anchor.clone(), candidate.clone())
        } else {
            (candidate.clone(), anchor.clone())
        };
        measure_cells.push((upper, lower));
    }
    measure_cells.sort_by(|left, right| {
        left.0
            .page_index
            .cmp(&right.0.page_index)
            .then_with(|| renderer_staff_center(&left.0).total_cmp(&renderer_staff_center(&right.0)))
            .then_with(|| left.0.x_left.total_cmp(&right.0.x_left))
    });
    if measure_cells.len() != measure_count {
        anyhow::bail!(
            "RendererPrimaryBeamGeometryReadback: PDF staff measure cells {} stimmen nicht mit MusicXML measures {} ueberein",
            measure_cells.len(),
            measure_count
        );
    }
    let mut out = Vec::with_capacity(measure_count.saturating_mul(2));
    for (offset, (mut upper, mut lower)) in measure_cells.into_iter().enumerate() {
        upper.measure = offset + 1;
        upper.staff = 1;
        lower.measure = offset + 1;
        lower.staff = 2;
        out.push(upper);
        out.push(lower);
    }
    Ok(out)
}

fn renderer_staff_center(cell: &RendererStaffCell) -> f64 {
    (cell.line_y_top.first().copied().unwrap_or(0.0)
        + cell.line_y_top.last().copied().unwrap_or(0.0))
        / 2.0
}


fn renderer_cell_beam_spans(
    cell: &RendererStaffCell,
    polygons: &[PdfFilledPolygon],
) -> Vec<RendererBeamSpan> {
    let mut spans = polygons
        .iter()
        .filter_map(|polygon| {
            if polygon.page_index != cell.page_index {
                return None;
            }
            let span = renderer_beam_span(polygon)?;
            (span.x_left >= cell.x_left - 1.0
                && span.x_right <= cell.x_right + 1.0)
                .then_some(span)
        })
        .collect::<Vec<_>>();
    spans.sort_by(|left, right| {
        left.x_left
            .total_cmp(&right.x_left)
            .then_with(|| left.x_right.total_cmp(&right.x_right))
            .then_with(|| left.y_left.total_cmp(&right.y_left))
    });
    let mut deduplicated = Vec::<RendererBeamSpan>::new();
    for span in spans {
        if deduplicated.last().is_none_or(|previous| {
            (span.x_left - previous.x_left).abs() > 0.8
                || (span.x_right - previous.x_right).abs() > 0.8
                || (span.y_left - previous.y_left).abs() > 0.8
                || (span.y_right - previous.y_right).abs() > 0.8
        }) {
            deduplicated.push(span);
        }
    }
    deduplicated
}

fn renderer_beam_span(polygon: &PdfFilledPolygon) -> Option<RendererBeamSpan> {
    if polygon.points.len() != 4 {
        return None;
    }
    let mut edge_lengths = Vec::new();
    for index in 0..4 {
        let left = polygon.points[index];
        let right = polygon.points[(index + 1) % 4];
        edge_lengths.push(((right.0 - left.0).hypot(right.1 - left.1), index));
    }
    edge_lengths.sort_by(|left, right| right.0.total_cmp(&left.0));
    let long_a = edge_lengths[0].0;
    let long_b = edge_lengths[1].0;
    let short_a = edge_lengths[2].0;
    let short_b = edge_lengths[3].0;
    if long_b < 8.0
        || !(1.5..=4.5).contains(&short_a)
        || !(1.5..=4.5).contains(&short_b)
    {
        return None;
    }
    let long_edge_is_horizontal = [edge_lengths[0].1, edge_lengths[1].1]
        .into_iter()
        .all(|index| {
            let left = polygon.points[index];
            let right = polygon.points[(index + 1) % 4];
            (right.1 - left.1).abs() <= (right.0 - left.0).abs() * 0.40 + 0.2
        });
    if !long_edge_is_horizontal || long_a > 400.0 {
        return None;
    }
    let x_left = polygon
        .points
        .iter()
        .map(|point| point.0)
        .fold(f64::INFINITY, f64::min);
    let x_right = polygon
        .points
        .iter()
        .map(|point| point.0)
        .fold(f64::NEG_INFINITY, f64::max);
    let mut left_points = polygon.points.clone();
    left_points.sort_by(|left, right| left.0.total_cmp(&right.0));
    let y_left = (left_points[0].1 + left_points[1].1) / 2.0;
    let y_right = (left_points[2].1 + left_points[3].1) / 2.0;
    Some(RendererBeamSpan {
        x_left,
        x_right,
        y_left,
        y_right,
    })
}

fn parse_pdf_objects(bytes: &[u8]) -> anyhow::Result<BTreeMap<u32, PdfObject>> {
    let mut starts = Vec::<(u32, usize, usize)>::new();
    let mut index = 0usize;
    while index < bytes.len() {
        let line_start = index == 0
            || matches!(
                bytes.get(index.wrapping_sub(1)),
                Some(&b'\n') | Some(&b'\r')
            );
        if !line_start || !bytes[index].is_ascii_digit() {
            index += 1;
            continue;
        }
        let mut cursor = index;
        while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor += 1;
        }
        let Ok(id_text) = std::str::from_utf8(&bytes[index..cursor]) else {
            index += 1;
            continue;
        };
        let Ok(id) = id_text.parse::<u32>() else {
            index += 1;
            continue;
        };
        skip_ascii_space(bytes, &mut cursor);
        if bytes.get(cursor) != Some(&b'0') {
            index += 1;
            continue;
        }
        cursor += 1;
        skip_ascii_space(bytes, &mut cursor);
        if bytes.get(cursor..cursor + 3) != Some(&b"obj"[..]) {
            index += 1;
            continue;
        }
        cursor += 3;
        starts.push((id, index, cursor));
        index = cursor;
    }
    if starts.is_empty() {
        anyhow::bail!("RendererSystemBreakReadback: PDF objects nicht erkannt");
    }
    let mut objects = BTreeMap::new();
    for (position, (id, _header_start, body_start)) in starts.iter().copied().enumerate() {
        let body_end = starts
            .get(position + 1)
            .map_or(bytes.len(), |(_, next_start, _)| *next_start);
        let mut body = bytes[body_start..body_end].to_vec();
        if let Some(endobj) = rfind_subslice(&body, b"endobj") {
            body.truncate(endobj);
        }
        objects.insert(id, PdfObject { body });
    }
    Ok(objects)
}

fn page_ids_in_order(objects: &BTreeMap<u32, PdfObject>) -> anyhow::Result<Vec<u32>> {
    let root = objects
        .iter()
        .find_map(|(id, object)| {
            let dict = object.dictionary_text();
            (dict.contains("/Type /Pages") && dict.contains("/Kids")).then_some(*id)
        })
        .ok_or_else(|| anyhow::anyhow!("RendererSystemBreakReadback: PDF Pages root fehlt"))?;
    let mut pages = Vec::new();
    collect_page_ids(root, objects, &mut BTreeSet::new(), &mut pages)?;
    if pages.is_empty() {
        anyhow::bail!("RendererSystemBreakReadback: PDF Pages tree leer");
    }
    Ok(pages)
}

fn collect_page_ids(
    object_id: u32,
    objects: &BTreeMap<u32, PdfObject>,
    visited: &mut BTreeSet<u32>,
    pages: &mut Vec<u32>,
) -> anyhow::Result<()> {
    if !visited.insert(object_id) {
        anyhow::bail!("RendererSystemBreakReadback: zyklischer PDF Pages tree");
    }
    let object = objects
        .get(&object_id)
        .ok_or_else(|| anyhow::anyhow!("PDF Pages object {object_id} fehlt"))?;
    let dict = object.dictionary_text();
    if dict.contains("/Type /Page") && !dict.contains("/Type /Pages") {
        pages.push(object_id);
        return Ok(());
    }
    let kids = parse_ref_array_after(dict, "/Kids");
    if kids.is_empty() {
        anyhow::bail!("RendererSystemBreakReadback: PDF Pages object {object_id} ohne Kids");
    }
    for kid in kids {
        collect_page_ids(kid, objects, visited, pages)?;
    }
    Ok(())
}

fn parse_font_refs(dict: &str) -> BTreeMap<String, u32> {
    let Some(font_pos) = dict.find("/Font") else {
        return BTreeMap::new();
    };
    let after = &dict[font_pos + "/Font".len()..];
    let Some(open) = after.find("<<") else {
        return BTreeMap::new();
    };
    let after_open = &after[open + 2..];
    let Some(close) = after_open.find(">>") else {
        return BTreeMap::new();
    };
    let body = &after_open[..close];
    let tokens = body.split_whitespace().collect::<Vec<_>>();
    let mut out = BTreeMap::new();
    let mut i = 0usize;
    while i + 3 < tokens.len() {
        if let Some(name) = tokens[i].strip_prefix('/')
            && tokens[i + 2].trim_matches(pdf_ref_delimiter) == "0"
            && tokens[i + 3].trim_matches(pdf_ref_delimiter) == "R"
        {
            if let Ok(id) = tokens[i + 1]
                .trim_matches(pdf_ref_delimiter)
                .parse::<u32>()
            {
                out.insert(name.to_string(), id);
            }
            i += 4;
            continue;
        }
        i += 1;
    }
    out
}

fn cmap_array_code(start: u16, offset: usize) -> Option<u16> {
    let offset = u16::try_from(offset).ok()?;
    Some(start.saturating_add(offset))
}

fn parse_to_unicode_cmap(bytes: &[u8]) -> BTreeMap<u16, char> {
    let text = String::from_utf8_lossy(bytes);
    let mut map = BTreeMap::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.ends_with("beginbfchar") {
            continue;
        }
        if trimmed.ends_with("beginbfrange") {
            continue;
        }
        let mut combined = trimmed.to_string();
        if combined.contains('[') && !combined.contains(']') {
            for next in lines.by_ref() {
                combined.push(' ');
                combined.push_str(next.trim());
                if next.contains(']') {
                    break;
                }
            }
        }
        let hex = extract_angle_hex_tokens(&combined);
        if combined.contains('[') && hex.len() >= 3 {
            if let (Some(start), Some(end)) = (parse_hex_u16(&hex[0]), parse_hex_u16(&hex[1])) {
                for (offset, dst) in hex.iter().skip(2).enumerate() {
                    let Some(code) = cmap_array_code(start, offset) else {
                        break;
                    };
                    if code > end {
                        break;
                    }
                    if let Some(ch) = parse_unicode_hex_char(dst) {
                        map.insert(code, ch);
                    }
                }
            }
        } else if hex.len() == 3 {
            if let (Some(start), Some(end), Some(dest)) = (
                parse_hex_u16(&hex[0]),
                parse_hex_u16(&hex[1]),
                parse_hex_u16(&hex[2]),
            ) {
                for code in start..=end {
                    if let Some(ch) = char::from_u32(u32::from(dest) + u32::from(code - start)) {
                        map.insert(code, ch);
                    }
                }
            }
        } else if hex.len() == 2
            && let (Some(code), Some(ch)) = (
                parse_hex_u16(&hex[0]),
                parse_unicode_hex_char(&hex[1]),
            )
        {
            map.insert(code, ch);
        }
    }
    map
}

fn extract_content_text_events(
    bytes: &[u8],
    page_index: usize,
    page_height: f64,
    font_maps: &BTreeMap<String, BTreeMap<u16, char>>,
    head_metrics: &BTreeMap<String, BTreeMap<char, [f64; 4]>>,
    out: &mut Vec<PdfTextEvent>,
) -> anyhow::Result<()> {
    let text = String::from_utf8_lossy(bytes);
    let mut ctm = Matrix::IDENTITY;
    let mut stack = Vec::<Matrix>::new();
    let mut in_text = false;
    let mut text_matrix = Matrix::IDENTITY;
    let mut current_font = String::new();
    let mut buffer = String::new();
    let mut font_size = 0.0;
    let mut head_bounds = None;
    let mut first_point: Option<(f64, f64)> = None;

    // MuseScore/Qt may place multiple PDF operators on one physical line, for
    // example `/F12 133 Tf 1 0 0 -1 0 0 Tm` and
    // `-6 0 Td <000D> Tj`. Therefore the audit must walk the operator tokens
    // in sequence instead of treating the final token of a line as its only
    // operation. The supported subset remains deliberately narrow/read-only.
    for raw_line in text.lines() {
        let tokens = raw_line.split_whitespace().collect::<Vec<_>>();
        for (index, token) in tokens.iter().copied().enumerate() {
            match token {
                "q" => stack.push(ctm),
                "Q" => {
                    if let Some(previous) = stack.pop() {
                        ctm = previous;
                    }
                }
                "cm" if index >= 6 => {
                    if let Some(matrix) = matrix_from_tokens(&tokens[index - 6..index]) {
                        ctm = ctm.multiply(matrix);
                    }
                }
                "BT" => {
                    in_text = true;
                    text_matrix = Matrix::IDENTITY;
                    current_font.clear();
                    buffer.clear();
                    head_bounds = None;
                    first_point = None;
                }
                "ET" => {
                    if in_text
                        && !buffer.is_empty()
                        && let Some((x, y)) = first_point
                    {
                        out.push(PdfTextEvent {
                            page_index,
                            x,
                            y_from_bottom: y,
                            page_height,
                            text: buffer.clone(),
                            head_bounds,
                        });
                    }
                    in_text = false;
                }
                "Tf" if in_text && index >= 2 => {
                    current_font = tokens[index - 2].trim_start_matches('/').to_string();
                    font_size = tokens[index - 1].parse::<f64>().unwrap_or(0.0);
                }
                "Tm" if in_text && index >= 6 => {
                    if let Some(matrix) = matrix_from_tokens(&tokens[index - 6..index]) {
                        text_matrix = matrix;
                    }
                }
                "Td" if in_text && index >= 2 => {
                    let tx = tokens[index - 2].parse::<f64>().unwrap_or(0.0);
                    let ty = tokens[index - 1].parse::<f64>().unwrap_or(0.0);
                    text_matrix = text_matrix.multiply(Matrix {
                        a: 1.0,
                        b: 0.0,
                        c: 0.0,
                        d: 1.0,
                        e: tx,
                        f: ty,
                    });
                }
                "Tj" if in_text && index >= 1 => {
                    let decoded = decode_hex_text(tokens[index - 1], font_maps.get(&current_font));
                    if !decoded.is_empty() {
                        if first_point.is_none() {
                            let local = text_matrix.point(0.0, 0.0);
                            first_point = Some(ctm.point(local.0, local.1));
                        }
                        if buffer.is_empty() && decoded.chars().count() == 1 {
                            head_bounds = decoded.chars().next()
                                .and_then(|ch| head_metrics.get(&current_font)?.get(&ch))
                                .and_then(|bounds| {
                                    let matrix = ctm.multiply(text_matrix);
                                    if matrix.b.abs() > 0.0001 || matrix.c.abs() > 0.0001 || font_size <= 0.0 {
                                        return None;
                                    }
                                    let left = matrix.point(bounds[0] * font_size, bounds[1] * font_size);
                                    let right = matrix.point(bounds[2] * font_size, bounds[3] * font_size);
                                    Some([left.0.min(right.0), page_height-left.1.max(right.1), left.0.max(right.0), page_height-left.1.min(right.1)])
                                });
                        } else {
                            head_bounds = None;
                        }
                        buffer.push_str(&decoded);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn decode_hex_text(token: &str, cmap: Option<&BTreeMap<u16, char>>) -> String {
    let Some(hex) = token.strip_prefix('<').and_then(|value| value.strip_suffix('>')) else {
        return String::new();
    };
    let Some(cmap) = cmap else {
        return String::new();
    };
    let mut out = String::new();
    let chars = hex.as_bytes();
    let mut i = 0usize;
    while i + 4 <= chars.len() {
        if let Ok(chunk) = std::str::from_utf8(&chars[i..i + 4])
            && let Ok(code) = u16::from_str_radix(chunk, 16)
            && let Some(ch) = cmap.get(&code)
        {
            out.push(*ch);
        }
        i += 4;
    }
    out
}

fn matrix_from_tokens(tokens: &[&str]) -> Option<Matrix> {
    if tokens.len() != 6 {
        return None;
    }
    Some(Matrix {
        a: tokens[0].parse().ok()?,
        b: tokens[1].parse().ok()?,
        c: tokens[2].parse().ok()?,
        d: tokens[3].parse().ok()?,
        e: tokens[4].parse().ok()?,
        f: tokens[5].parse().ok()?,
    })
}

fn parse_single_ref_after(text: &str, key: &str) -> Option<u32> {
    let pos = text.find(key)? + key.len();
    let tokens = text[pos..].split_whitespace().take(3).collect::<Vec<_>>();
    if tokens.len() != 3 {
        return None;
    }
    let first = tokens[0].trim_matches(pdf_ref_delimiter);
    let second = tokens[1].trim_matches(pdf_ref_delimiter);
    let third = tokens[2].trim_matches(pdf_ref_delimiter);
    if second == "0" && third == "R" {
        first.parse().ok()
    } else {
        None
    }
}

fn pdf_ref_delimiter(ch: char) -> bool {
    matches!(ch, '<' | '>' | '[' | ']' | '(' | ')' | '/')
}

fn parse_ref_array_after(text: &str, key: &str) -> Vec<u32> {
    let Some(pos) = text.find(key) else {
        return Vec::new();
    };
    let after = &text[pos + key.len()..];
    let Some(open) = after.find('[') else {
        return Vec::new();
    };
    let after = &after[open + 1..];
    let Some(close) = after.find(']') else {
        return Vec::new();
    };
    let tokens = after[..close].split_whitespace().collect::<Vec<_>>();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 2 < tokens.len() {
        if tokens[i + 1].trim_matches(pdf_ref_delimiter) == "0"
            && tokens[i + 2].trim_matches(pdf_ref_delimiter) == "R"
        {
            if let Ok(id) = tokens[i]
                .trim_matches(pdf_ref_delimiter)
                .parse::<u32>()
            {
                out.push(id);
            }
            i += 3;
        } else {
            i += 1;
        }
    }
    out
}

fn parse_media_box_height(text: &str) -> Option<f64> {
    let pos = text.find("/MediaBox")? + "/MediaBox".len();
    let after = &text[pos..];
    let open = after.find('[')?;
    let after = &after[open + 1..];
    let close = after.find(']')?;
    let values = after[..close]
        .split_whitespace()
        .filter_map(|token| token.parse::<f64>().ok())
        .collect::<Vec<_>>();
    if values.len() >= 4 {
        Some(values[3] - values[1])
    } else {
        None
    }
}

fn extract_angle_hex_tokens(line: &str) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'<' || bytes.get(i + 1) == Some(&b'<') {
            i += 1;
            continue;
        }
        let start = i + 1;
        let Some(rel_end) = bytes[start..].iter().position(|byte| *byte == b'>') else {
            break;
        };
        let end = start + rel_end;
        out.push(String::from_utf8_lossy(&bytes[start..end]).into_owned());
        i = end + 1;
    }
    out
}

fn parse_hex_u16(value: &str) -> Option<u16> {
    u16::from_str_radix(value, 16).ok()
}

fn parse_unicode_hex_char(value: &str) -> Option<char> {
    if value.len() == 4 {
        return char::from_u32(u32::from(parse_hex_u16(value)?));
    }
    if value.len() == 8 {
        let high = parse_hex_u16(&value[..4])?;
        let low = parse_hex_u16(&value[4..])?;
        if (0xD800..=0xDBFF).contains(&high) && (0xDC00..=0xDFFF).contains(&low) {
            let code = 0x1_0000
                + ((u32::from(high) - 0xD800) << 10)
                + (u32::from(low) - 0xDC00);
            return char::from_u32(code);
        }
    }
    None
}

fn format_usize_list(values: &[usize]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn format_system_groups(starts: &[usize], measure_count: usize) -> String {
    if starts.is_empty() || measure_count == 0 {
        return "none".to_string();
    }
    starts
        .iter()
        .enumerate()
        .map(|(index, start)| {
            let end = starts
                .get(index + 1)
                .map_or(measure_count, |next| next.saturating_sub(1));
            if *start == end {
                start.to_string()
            } else {
                format!("{start}-{end}")
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn one_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn skip_ascii_space(bytes: &[u8], cursor: &mut usize) {
    while bytes.get(*cursor).is_some_and(u8::is_ascii_whitespace) {
        *cursor += 1;
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn rfind_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .rposition(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_pdf() -> Vec<u8> {
        let content1 = b"0.060000000 0 0 -0.060000000 0 792 cm\nq 1 0 0 1 708 3000 cm\nBT\n/F1 133 Tf 1 0 0 -1 0 0 Tm\n0 0 Td <0001> Tj\nET Q\n";
        let content2 = b"0.060000000 0 0 -0.060000000 0 792 cm\nq 1 0 0 1 708 1200 cm\nBT\n/F1 133 Tf 1 0 0 -1 0 0 Tm\n0 0 Td <0002> Tj\nET Q\n";
        let cmap = b"2 beginbfrange\n<0001> <0001> <0033>\n<0002> <0002> <0035>\nendbfrange\n";
        format!(
            "%PDF-1.4\n1 0 obj\n<< /Type /Pages /Kids [2 0 R 3 0 R] /Count 2 >>\nendobj\n2 0 obj\n<< /Type /Page /Parent 1 0 R /Contents 4 0 R /Resources 6 0 R /MediaBox [0 0 612 792] >>\nendobj\n3 0 obj\n<< /Type /Page /Parent 1 0 R /Contents 5 0 R /Resources 6 0 R /MediaBox [0 0 612 792] >>\nendobj\n4 0 obj\n<< /Length {} >>\nstream\n{}endstream\nendobj\n5 0 obj\n<< /Length {} >>\nstream\n{}endstream\nendobj\n6 0 obj\n<< /Font << /F1 7 0 R >> >>\nendobj\n7 0 obj\n<< /Type /Font /Subtype /Type0 /ToUnicode 8 0 R >>\nendobj\n8 0 obj\n<< /Length {} >>\nstream\n{}endstream\nendobj\n%%EOF\n",
            content1.len(),
            String::from_utf8_lossy(content1),
            content2.len(),
            String::from_utf8_lossy(content2),
            cmap.len(),
            String::from_utf8_lossy(cmap),
        )
        .into_bytes()
    }

    #[test]
    fn planned_breaks_treat_new_page_as_system_start() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"/><measure number=\"\
2\"/><measure number=\"\
3\"><print new-system=\"yes\"/></measure><measure number=\"\
4\"/><measure number=\"\
5\"><print new-page=\"yes\"/></measure></part></score-partwise>";
        let (count, systems, pages) = planned_breaks_from_musicxml(xml).expect("plan");
        assert_eq!(count, 5);
        assert_eq!(systems, vec![1, 3, 5]);
        assert_eq!(pages, vec![1, 5]);
    }

    #[test]
    fn pdf_readback_handles_multiple_operators_per_line() {
        let (events, pages) = extract_pdf_text_events(&synthetic_pdf()).expect("events");
        assert_eq!(pages, 2);
        let numeric = events
            .iter()
            .filter_map(|event| event.text.parse::<usize>().ok())
            .collect::<Vec<_>>();
        assert_eq!(numeric, vec![3, 5]);
        assert_eq!(events[0].page_index, 1);
        assert_eq!(events[1].page_index, 2);
    }

    #[test]
    fn system_group_projection_is_deterministic() {
        assert_eq!(format_system_groups(&[1, 3, 4, 6], 7), "1-2,3,4-5,6-7");
        assert_eq!(format_usize_list(&[]), "none");
    }

    #[test]
    fn planner_demand_binding_projects_inserted_break_headroom() {
        let audit = RendererSystemBreakReadbackAudit {
            ottava_context: Default::default(),
            planned_system_starts: vec![1, 13, 16],
            rendered_system_starts: vec![1, 13, 15, 16],
            planned_page_starts: vec![1],
            rendered_page_starts: vec![1],
            inserted_system_breaks: vec![15],
            collapsed_system_breaks: Vec::new(),
            inserted_page_breaks: Vec::new(),
            collapsed_page_breaks: Vec::new(),
            measure_count: 18,
            pdf_pages: 1,
        };
        let line = "ETUSCHLOPF: HorizontalLayoutGlobalPageSystemPreferenceCandidate: Slots 18 | AcceptedSystemCapacity 519 | MaxSelectedSystemLoad 511 | SelectedSystemLoads 1-12=400,13-15=511,16-18=460 | SelectedMeasureDemands 1=33,2=33,3=33,4=33,5=33,6=33,7=33,8=33,9=33,10=33,11=33,12=37,13=170,14=170,15=171,16=153,17=153,18=154 | PolicyWrite nein";
        let projection = planner_demand_binding_projection(&audit, Some(line));
        assert!(projection.contains("Status BOUND"));
        assert!(projection.contains("13-15=load511/headroom8/split15"));
        assert!(projection.contains("13-15=load511/headroom8/insertedAt15"));
        assert!(projection.contains("ModelRecompute nein"));
        assert!(projection.contains("MeasureDemands 1=33"));
        assert!(projection.contains("MeasureDemandRecompose exact"));
        assert!(projection.contains("ThresholdChange nein"));
    }

    #[test]
    fn final_selected_plan_demand_binding_projects_adopted_plan() {
        let audit = RendererSystemBreakReadbackAudit {
            ottava_context: Default::default(),
            planned_system_starts: vec![1, 4],
            rendered_system_starts: vec![1, 4],
            planned_page_starts: vec![1],
            rendered_page_starts: vec![1],
            inserted_system_breaks: Vec::new(),
            collapsed_system_breaks: Vec::new(),
            inserted_page_breaks: Vec::new(),
            collapsed_page_breaks: Vec::new(),
            measure_count: 6,
            pdf_pages: 1,
        };
        let line = "ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding: AcceptedSystemCapacity 500 | SelectedSystemLoads 1-3=300,4-6=300 | SelectedMeasureDemands 1=100,2=100,3=100,4=100,5=100,6=100 | Source existing_joint_planner_measure_demands_recomposed_to_final_selected_plan | MeasureDemandRecompose exact | ModelRecompute nein | ThresholdChange nein | PolicyWrite nein";
        let projection = planner_demand_binding_projection(&audit, Some(line));
        assert!(projection.contains("Status BOUND"));
        assert!(projection.contains("Source final_selected_plan_existing_joint_planner_evidence"));
        assert!(projection.contains("1-3=load300/headroom200/splitnone"));
        assert!(projection.contains("MeasureDemandRecompose exact"));
        assert!(projection.contains("ModelRecompute nein"));
        assert!(projection.contains("ThresholdChange nein"));
    }

    #[test]
    fn planner_demand_binding_rejects_measure_demand_recompose_drift() {
        let audit = RendererSystemBreakReadbackAudit {
            ottava_context: Default::default(),
            planned_system_starts: vec![1, 4],
            rendered_system_starts: vec![1, 4],
            planned_page_starts: vec![1],
            rendered_page_starts: vec![1],
            inserted_system_breaks: Vec::new(),
            collapsed_system_breaks: Vec::new(),
            inserted_page_breaks: Vec::new(),
            collapsed_page_breaks: Vec::new(),
            measure_count: 6,
            pdf_pages: 1,
        };
        let line = "ETUSCHLOPF: HorizontalLayoutGlobalPageSystemPreferenceCandidate: Slots 6 | AcceptedSystemCapacity 500 | SelectedSystemLoads 1-3=300,4-6=300 | SelectedMeasureDemands 1=100,2=100,3=101,4=100,5=100,6=100 | PolicyWrite nein";
        let error = parse_planner_demand_binding(line)
            .and_then(|binding| validate_planner_demand_binding(&audit, binding))
            .expect_err("measure demand drift must fail closed");
        assert!(error.to_string().contains("Measure-Demand-Summe"));
    }

    #[test]
    fn planner_demand_binding_rejects_group_drift() {
        let audit = RendererSystemBreakReadbackAudit {
            ottava_context: Default::default(),
            planned_system_starts: vec![1, 4],
            rendered_system_starts: vec![1, 4],
            planned_page_starts: vec![1],
            rendered_page_starts: vec![1],
            inserted_system_breaks: Vec::new(),
            collapsed_system_breaks: Vec::new(),
            inserted_page_breaks: Vec::new(),
            collapsed_page_breaks: Vec::new(),
            measure_count: 6,
            pdf_pages: 1,
        };
        let line = "ETUSCHLOPF: HorizontalLayoutGlobalPageSystemPreferenceCandidate: Slots 6 | AcceptedSystemCapacity 500 | SelectedSystemLoads 1-2=300,3-6=400 | SelectedMeasureDemands 1=150,2=150,3=100,4=100,5=100,6=100 | PolicyWrite nein";
        let projection = planner_demand_binding_projection(&audit, Some(line));
        assert!(projection.contains("Status UNAVAILABLE"));
        assert!(projection.contains("stimmt nicht mit Readback-Plan"));
    }

    #[test]
    fn planner_demand_binding_rejects_ambiguous_capacity_field() {
        let line = "ETUSCHLOPF: HorizontalLayoutGlobalPageSystemPreferenceCandidate: Slots 3 | AcceptedSystemCapacity 500 | AcceptedSystemCapacity 501 | SelectedSystemLoads 1-3=400 | PolicyWrite nein";
        let error = parse_planner_demand_binding(line).expect_err("ambiguous field");
        assert!(error.to_string().contains("ist mehrdeutig"));
    }

    #[test]
    fn rhythmic_columns_collapse_chord_notes_onto_one_column() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>\n<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>\n<note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>\n<note><pitch><step>D</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>\n</measure></part></score-partwise>";
        let evidence = rhythmic_column_evidence_from_musicxml(xml).expect("columns");
        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].columns, 2);
        assert_eq!(evidence[0].pitched_columns, 2);
        assert_eq!(evidence[0].note_elements, 4);
        assert_eq!(evidence[0].max_notes_per_column, 3);
    }

    #[test]
    fn rhythmic_columns_merge_voice_onsets_across_backup() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>\n<note><pitch><step>D</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>\n<backup><duration>8</duration></backup>\n<note><pitch><step>C</step><octave>3</octave></pitch><duration>8</duration><voice>2</voice></note>\n</measure></part></score-partwise>";
        let evidence = rhythmic_column_evidence_from_musicxml(xml).expect("columns");
        assert_eq!(evidence[0].columns, 2);
        assert_eq!(evidence[0].note_elements, 3);
        assert_eq!(evidence[0].max_notes_per_column, 2);
    }

    #[test]
    fn rhythmic_columns_keep_chord_owner_across_forward() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>\n<forward><duration>4</duration></forward>\n<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>\n</measure></part></score-partwise>";
        let evidence = rhythmic_column_evidence_from_musicxml(xml).expect("columns");
        assert_eq!(evidence[0].columns, 1);
        assert_eq!(evidence[0].pitched_columns, 1);
        assert_eq!(evidence[0].note_elements, 2);
        assert_eq!(evidence[0].max_notes_per_column, 2);
    }

    #[test]
    fn rhythmic_columns_reset_chord_owner_across_backup() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>\n<backup><duration>4</duration></backup>\n<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>2</voice></note>\n</measure></part></score-partwise>";
        let error = rhythmic_column_evidence_from_musicxml(xml).expect_err("backup must reset chord owner");
        assert!(error.to_string().contains("chord ohne vorherigen Onset"));
    }

    #[test]
    fn rhythmic_columns_observe_sixteenth_rest_accidental_clearance_edge() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n<note><pitch><step>B</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>\n<note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>\n<note><pitch><step>B</step><alter>-1</alter><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>\n</measure></part></score-partwise>";
        let evidence = rhythmic_column_evidence_from_musicxml(xml).expect("columns");
        assert_eq!(evidence[0].following_accidental_clearance_edges, 1);
    }

    #[test]
    fn rhythmic_columns_do_not_invent_clearance_without_same_position_left_anchor() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n<note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>\n<note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>\n<note><pitch><step>B</step><alter>-1</alter><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>\n</measure></part></score-partwise>";
        let evidence = rhythmic_column_evidence_from_musicxml(xml).expect("columns");
        assert_eq!(evidence[0].following_accidental_clearance_edges, 0);
    }

    #[test]
    fn rhythmic_shape_projection_binds_split_to_planned_system_shape() {
        let audit = RendererSystemBreakReadbackAudit {
            ottava_context: Default::default(),
            planned_system_starts: vec![1, 4],
            rendered_system_starts: vec![1, 3, 4],
            planned_page_starts: vec![1],
            rendered_page_starts: vec![1],
            inserted_system_breaks: vec![3],
            collapsed_system_breaks: Vec::new(),
            inserted_page_breaks: Vec::new(),
            collapsed_page_breaks: Vec::new(),
            measure_count: 6,
            pdf_pages: 1,
        };
        let evidence = vec![
            MeasureRhythmicColumnEvidence {
                measure: 1,
                columns: 16,
                note_elements: 20,
                pitched_columns: 16,
                max_notes_per_column: 2,
                following_accidental_clearance_edges: 0,
            },
            MeasureRhythmicColumnEvidence {
                measure: 2,
                columns: 15,
                note_elements: 19,
                pitched_columns: 15,
                max_notes_per_column: 2,
                following_accidental_clearance_edges: 1,
            },
            MeasureRhythmicColumnEvidence {
                measure: 3,
                columns: 15,
                note_elements: 19,
                pitched_columns: 15,
                max_notes_per_column: 2,
                following_accidental_clearance_edges: 0,
            },
            MeasureRhythmicColumnEvidence {
                measure: 4,
                columns: 12,
                note_elements: 16,
                pitched_columns: 12,
                max_notes_per_column: 2,
                following_accidental_clearance_edges: 0,
            },
            MeasureRhythmicColumnEvidence {
                measure: 5,
                columns: 13,
                note_elements: 17,
                pitched_columns: 13,
                max_notes_per_column: 2,
                following_accidental_clearance_edges: 0,
            },
            MeasureRhythmicColumnEvidence {
                measure: 6,
                columns: 14,
                note_elements: 18,
                pitched_columns: 14,
                max_notes_per_column: 2,
                following_accidental_clearance_edges: 0,
            },
        ];
        let projection = rhythmic_shape_projection(&audit, &evidence);
        assert!(projection.contains("ThreeMeasureSystems 2"));
        assert!(projection.contains("SplitThreeMeasureSystems 1"));
        assert!(projection.contains("AllInsertedBreaksBeforeFinalMeasure ja"));
        assert!(projection.contains("1-3=len3/cols16+15+15"));
        assert!(projection.contains("/clearanceEdges0+1+0/split3"));
        assert!(projection.contains("DemandModelWrite nein"));
    }

    #[test]
    fn cmap_array_offset_is_never_truncated_to_u16() {
        assert_eq!(cmap_array_code(10, 2), Some(12));
        assert_eq!(cmap_array_code(1, usize::from(u16::MAX)), Some(u16::MAX));
        assert_eq!(
            cmap_array_code(1, usize::from(u16::MAX).saturating_add(1)),
            None
        );
    }


    #[test]
    fn primary_beam_cohort_geometry_counts_fragmentation_from_vector_spans() {
        let readback = RendererPrimaryBeamGeometryReadback {
            stem_geometry_by_occurrence: BTreeMap::from([
                ((1, 2, 7), RendererStemGeometry { x: 10.0, y_min: 10.0, y_max: 30.0 }),
                ((1, 2, 8), RendererStemGeometry { x: 20.0, y_min: 10.0, y_max: 30.0 }),
                ((1, 2, 9), RendererStemGeometry { x: 30.0, y_min: 10.0, y_max: 30.0 }),
            ]),
            beam_spans_by_measure_staff: BTreeMap::from([((1, 2), vec![RendererBeamSpan { x_left: 9.0, x_right: 21.0, y_left: 20.0, y_right: 20.0 }])]),
            measure_count: 1,
            bound_measure_staff_cells: 1,
            unresolved_measure_staff_cells: 0,
        };
        let geometry = readback.cohort_geometry(1, 2, &[7, 8, 9]).expect("geometry");
        assert_eq!(geometry.connected_adjacent_pairs, 1);
        assert_eq!(geometry.visible_fragments, 2);
    }

    #[test]
    fn renderer_beam_span_accepts_beam_quad_and_rejects_short_flag_stub() {
        let beam = PdfFilledPolygon {
            page_index: 1,
            points: vec![(10.0, 20.0), (30.0, 20.5), (30.0, 23.0), (10.0, 22.5)],
        };
        let stub = PdfFilledPolygon {
            page_index: 1,
            points: vec![(10.0, 20.0), (15.0, 20.2), (15.0, 22.7), (10.0, 22.5)],
        };
        let span = renderer_beam_span(&beam).expect("beam");
        assert_eq!(span.x_left, 10.0);
        assert_eq!(span.x_right, 30.0);
        assert!(renderer_beam_span(&stub).is_none());
    }

}

// Read actual embedded TrueType glyph bounds. Only the renderer's Identity-H /
// Identity CID-to-GID profile is supported. Missing metrics never authorize a bind.
fn pdf_notehead_metrics(font: &PdfObject, objects: &BTreeMap<u32, PdfObject>, cmap: &BTreeMap<u16, char>) -> Option<BTreeMap<char,[f64;4]>> {
    embedded_glyph_metrics(font,objects,cmap,|ch|matches!(ch,'\u{e0a3}'|'\u{e0a4}'))
}
fn pdf_ottava_metrics(font: &PdfObject, objects: &BTreeMap<u32, PdfObject>, cmap: &BTreeMap<u16, char>) -> Option<BTreeMap<char,[f64;4]>> {
    embedded_glyph_metrics(font,objects,cmap,|ch|matches!(ch,'\u{e510}'|'\u{e511}'|'\u{e512}'|'\u{e513}'|'\u{e514}'|'\u{e515}'|'\u{e516}'|'\u{e51c}'|'\u{e51d}'))
}
fn embedded_glyph_metrics(
    font: &PdfObject,
    objects: &BTreeMap<u32, PdfObject>,
    cmap: &BTreeMap<u16, char>,
    relevant: fn(char)->bool,
) -> Option<BTreeMap<char, [f64; 4]>> {
    let dict = font.dictionary_text();
    if !dict.contains("/Encoding /Identity-H") || !dict.contains("/BaseFont /Leland") {
        return None;
    }
    let descendants = parse_ref_array_after(dict, "/DescendantFonts");
    let [descendant] = descendants.as_slice() else { return None; };
    let descendant = objects.get(descendant)?.dictionary_text();
    if !descendant.contains("/CIDToGIDMap /Identity") { return None; }
    let descriptor = objects.get(&parse_single_ref_after(descendant, "/FontDescriptor")?)?.dictionary_text();
    let bytes = objects.get(&parse_single_ref_after(descriptor, "/FontFile2")?)?.decoded_stream().ok()??;
    let u16_at = |offset: usize| -> Option<u16> {
        Some(u16::from_be_bytes(bytes.get(offset..offset.checked_add(2)?)?.try_into().ok()?))
    };
    let u32_at = |offset: usize| -> Option<usize> {
        usize::try_from(u32::from_be_bytes(bytes.get(offset..offset.checked_add(4)?)?.try_into().ok()?)).ok()
    };
    if bytes.get(..4)? != [0, 1, 0, 0] { return None; }
    let mut tables = BTreeMap::new();
    for index in 0..usize::from(u16_at(4)?) {
        let offset = 12 + 16 * index;
        let start = u32_at(offset + 8)?;
        let length = u32_at(offset + 12)?;
        bytes.get(start..start.checked_add(length)?)?;
        tables.insert(bytes.get(offset..offset+4)?, (start, length));
    }
    let &(head, head_length) = tables.get(b"head".as_slice())?;
    let &(loca, loca_length) = tables.get(b"loca".as_slice())?;
    let &(glyf, glyf_length) = tables.get(b"glyf".as_slice())?;
    let &(maxp, maxp_length) = tables.get(b"maxp".as_slice())?;
    if head_length < 54 || maxp_length < 6 { return None; }
    let units = f64::from(u16_at(head + 18)?);
    if units == 0.0 { return None; }
    let format = u16_at(head + 50)?;
    let glyph_count = u16_at(maxp + 4)?;
    let mut result = BTreeMap::new();
    for (&code, &ch) in cmap {
        if !relevant(ch) || code >= glyph_count { continue; }
        let stride = match format { 0 => 2, 1 => 4, _ => return None };
        let offset = usize::from(code) * stride;
        if offset + 2 * stride > loca_length { return None; }
        let position = |offset| -> Option<usize> {
            if format == 0 { Some(usize::from(u16_at(loca + offset)?) * 2) }
            else { u32_at(loca + offset) }
        };
        let start = position(offset)?;
        let end = position(offset + stride)?;
        if end > glyf_length || end < start + 10 { return None; }
        let signed = |offset| -> Option<f64> {
            Some(f64::from(i16::from_be_bytes(bytes.get(offset..offset+2)?.try_into().ok()?)) / units)
        };
        let bounds = [signed(glyf+start+2)?, signed(glyf+start+4)?, signed(glyf+start+6)?, signed(glyf+start+8)?];
        if bounds[0] >= bounds[2] || bounds[1] >= bounds[3] { return None; }
        result.insert(ch, bounds);
    }
    Some(result)
}

fn same_physical_stem(left: RendererStemGeometry, right: RendererStemGeometry) -> bool {
    (left.x - right.x).abs() < 0.05
        && (left.y_min - right.y_min).abs() < 0.05
        && (left.y_max - right.y_max).abs() < 0.05
}

#[derive(Debug, Clone)]
struct RendererBoundHead {
    bounds: [f64; 4],
    pitches: BTreeSet<i32>,
}
#[derive(Debug, Clone)]
struct RendererHeadBoundStem {
    geometry: RendererStemGeometry,
    heads: Vec<RendererBoundHead>,
}
fn renderer_shared_head_compatible(
    left: &RendererBoundHead,
    right: &RendererBoundHead,
    left_occurrence: &RenderStemOccurrence,
    right_occurrence: &RenderStemOccurrence,
) -> bool {
    left.bounds != right.bounds || (
        left.pitches.len() == 1 && left.pitches == right.pitches
        && left_occurrence.measure == right_occurrence.measure
        && left_occurrence.staff == right_occurrence.staff
        && left_occurrence.onset == right_occurrence.onset
    )
}
fn renderer_head_binding(
    cell: &RendererStaffCell,
    segments: &[PdfStrokeSegment],
    heads: &[PdfTextEvent],
    expected: &[RenderStemOccurrence],
) -> Result<Vec<RendererHeadBoundStem>, &'static str> {
    if expected.is_empty() { return Ok(Vec::new()); }
    let spacing = (cell.line_y_top.last().ok_or("missing_staff_lines")? - cell.line_y_top.first().ok_or("missing_staff_lines")?) / 4.0;
    let bottom = *cell.line_y_top.last().ok_or("missing_staff_lines")?;
    let mut stems = segments.iter().filter_map(|segment| {
        let stem = RendererStemGeometry {
            x: (segment.x1 + segment.x2) / 2.0,
            y_min: segment.y1_top.min(segment.y2_top),
            y_max: segment.y1_top.max(segment.y2_top),
        };
        (segment.page_index == cell.page_index
            && (segment.x2 - segment.x1).abs() <= 0.5
            && stem.y_max - stem.y_min >= 8.0
            && (0.35..=0.65).contains(&segment.width)
            && stem.x > cell.x_left + 1.0 && stem.x < cell.x_right - 1.0)
            .then_some(stem)
    }).collect::<Vec<_>>();
    stems.sort_by(|a,b| a.x.total_cmp(&b.x).then_with(||a.y_min.total_cmp(&b.y_min)).then_with(||a.y_max.total_cmp(&b.y_max)));
    stems.dedup_by(|a,b| same_physical_stem(*a,*b));
    let mut candidates = Vec::<Vec<(usize, Vec<RendererBoundHead>)>>::new();
    for occurrence in expected {
        if occurrence.head_steps.is_empty() || occurrence.head_steps.iter().any(|step| step.abs() > 128)
            || occurrence.head_steps.len() != occurrence.head_pitches.len() {
            return Err("unsupported_expected_heads");
        }
        let mut pitches_by_step = BTreeMap::<i32, BTreeSet<i32>>::new();
        for (&step, pitch) in occurrence.head_steps.iter().zip(&occurrence.head_pitches) {
            pitches_by_step.entry(step).or_default().insert(pitch.ok_or("unsupported_head_pitch")?);
        }
        let ys = pitches_by_step.keys().map(|step| bottom - f64::from(*step)*spacing/2.0).collect::<Vec<_>>();
        let mut choices = Vec::new();
        for (index, stem) in stems.iter().enumerate() {
            for stem_up in [true, false] {
                let extreme = if stem_up { ys.iter().copied().max_by(f64::total_cmp) } else { ys.iter().copied().min_by(f64::total_cmp) }.ok_or("missing_expected_heads")?;
                let endpoint = if stem_up { stem.y_max } else { stem.y_min };
                if (endpoint-extreme).abs() > spacing*0.35 { continue; }
                let mut bound_heads = Vec::new();
                let mut valid = true;
                for (&step, pitches) in &pitches_by_step {
                    let y = bottom - f64::from(step)*spacing/2.0;
                    let mut matching = heads.iter().filter_map(|head| {
                        if head.page_index != cell.page_index { return None; }
                        let bounds = head.head_bounds?;
                        let head_y = head.page_height-head.y_from_bottom;
                        let edge = if occurrence.head_steps.len() > 1 {
                            if (bounds[0]-stem.x).abs() < (bounds[2]-stem.x).abs() { bounds[0] } else { bounds[2] }
                        } else if stem_up { bounds[2] } else { bounds[0] };
                        ((head_y-y).abs() <= spacing*0.15 && (edge-stem.x).abs() <= spacing*0.15
                            && head_y >= stem.y_min-spacing*0.4 && head_y <= stem.y_max+spacing*0.4).then_some(bounds)
                    }).collect::<Vec<_>>();
                    matching.sort_by(|a,b| a[0].total_cmp(&b[0]).then_with(|| a[1].total_cmp(&b[1])).then_with(|| a[2].total_cmp(&b[2])).then_with(|| a[3].total_cmp(&b[3])));
                    matching.dedup();
                    if matching.len() != pitches.len() || pitches.len() > 2 { valid = false; break; }
                    if pitches.len() == 2 {
                        let left = matching.iter().filter(|b| (b[2]-stem.x).abs() <= spacing*0.15 && (b[0]+b[2])/2.0 < stem.x).count();
                        let right = matching.iter().filter(|b| (b[0]-stem.x).abs() <= spacing*0.15 && (b[0]+b[2])/2.0 > stem.x).count();
                        if left != 1 || right != 1 { valid = false; break; }
                    }
                    bound_heads.extend(matching.into_iter().map(|bounds| RendererBoundHead { bounds, pitches: pitches.clone() }));
                }
                if valid { choices.push((index, bound_heads)); break; }
            }
        }
        if choices.is_empty() { return Err("no_complete_head_stem_candidate"); }
        candidates.push(choices);
    }
    fn visit(index: usize, choices: &[Vec<(usize, Vec<RendererBoundHead>)>], stems: &[RendererStemGeometry], expected: &[RenderStemOccurrence], path: &mut Vec<usize>, solutions: &mut Vec<Vec<usize>>) {
        if solutions.len() > 1 { return; }
        if index == choices.len() { solutions.push(path.clone()); return; }
        for (choice_index, (stem_index, evidence)) in choices[index].iter().enumerate() {
            if path.iter().enumerate().any(|(prior, &previous_choice)| {
                let (previous, previous_heads) = &choices[prior][previous_choice];
                previous == stem_index
                    || (expected[prior].onset < expected[index].onset && stems[*previous].x >= stems[*stem_index].x-0.05)
                    || previous_heads.iter().any(|left| evidence.iter().any(|right| !renderer_shared_head_compatible(left, right, &expected[prior], &expected[index])))
            }) { continue; }
            path.push(choice_index);
            visit(index+1, choices, stems, expected, path, solutions);
            path.pop();
        }
    }
    let mut solutions = Vec::new();
    visit(0, &candidates, &stems, expected, &mut Vec::new(), &mut solutions);
    if solutions.len() > 1 { return Err("multiple_complete_assignments"); }
    let solution = solutions.first().ok_or("no_exclusive_temporal_assignment")?;
    Ok(solution.iter().enumerate().map(|(i,&choice)| {
        let (index, heads) = &candidates[i][choice];
        RendererHeadBoundStem { geometry: stems[*index], heads: heads.clone() }
    }).collect())
}
#[cfg(test)]
fn renderer_head_bound_stems(
    cell: &RendererStaffCell,
    segments: &[PdfStrokeSegment],
    heads: &[PdfTextEvent],
    expected: &[RenderStemOccurrence],
) -> Option<Vec<RendererStemGeometry>> {
    renderer_head_binding(cell, segments, heads, expected).ok().map(|stems| stems.into_iter().map(|s| s.geometry).collect())
}

#[cfg(test)]
mod notehead_binding_tests {
    use super::*;
    fn cell() -> RendererStaffCell {
        RendererStaffCell { page_index: 1, x_left: 10.0, x_right: 110.0, staff: 1, measure: 1, line_y_top: vec![100.0,105.0,110.0,115.0,120.0] }
    }
    fn occurrence(steps: Vec<i32>) -> RenderStemOccurrence {
        RenderStemOccurrence { measure: 1, staff: 1, onset: 0, pitched_ordinal: 0, head_pitches: steps.iter().map(|s| Some(60+s)).collect(), head_steps: steps }
    }
    fn head(x: f64, y: f64) -> PdfTextEvent {
        PdfTextEvent { page_index: 1, x, y_from_bottom: 200.0-y, page_height: 200.0, text: "\u{e0a4}".to_owned(), head_bounds: Some([x, y-2.5, x+6.5, y+2.5]) }
    }
    fn stroke(x: f64, top: f64, bottom: f64) -> PdfStrokeSegment {
        PdfStrokeSegment { page_index: 1, x1:x, x2:x, y1_top:top, y2_top:bottom, width:0.5 }
    }
    #[test]
    fn long_stem_requires_real_head_and_rejects_same_width_unattached_line() {
        let expected = [occurrence(vec![0])];
        let actual = stroke(26.25,60.0,119.2);
        let distractor = stroke(50.0,70.0,119.2);
        let stems = renderer_head_bound_stems(&cell(), &[actual,distractor], &[head(20.0,120.0)], &expected).unwrap();
        assert_eq!(stems.len(),1);
        assert_eq!(stems[0].x,26.25);
        assert!(renderer_head_bound_stems(&cell(), &[actual], &[], &expected).is_none());
    }
    #[test]
    fn stroke_duplicates_are_equivalent_but_same_x_different_endpoints_are_not() {
        let expected = [occurrence(vec![0])];
        let a=stroke(26.25,60.0,119.2);
        let b=stroke(26.25,50.0,119.2);
        assert_eq!(renderer_head_bound_stems(&cell(), &[a,a], &[head(20.0,120.0)], &expected).unwrap().len(),1);
        assert!(renderer_head_bound_stems(&cell(), &[a,b], &[head(20.0,120.0)], &expected).is_none());
        assert!(renderer_head_bound_stems(&cell(), &[b,a], &[head(20.0,120.0)], &expected).is_none());
    }
    #[test]
    fn relocated_cell_and_independent_copy_preserve_attachment() {
        let expected=[occurrence(vec![0])];
        let a=stroke(26.25,60.0,119.2);
        let mut moved=cell(); moved.measure=7; moved.x_left+=200.0; moved.x_right+=200.0;
        for y in &mut moved.line_y_top { *y+=40.0; }
        let result=renderer_head_bound_stems(&moved,&[a,stroke(226.25,100.0,159.2)],&[head(20.0,120.0),head(220.0,160.0)],&expected).unwrap();
        assert_eq!(result[0].x,226.25);
    }
    #[test]
    fn chord_with_displaced_head_binds_atomically_and_missing_member_is_unknown() {
        let expected=[occurrence(vec![0,1])];
        let a=stroke(26.25,70.0,119.2);
        let heads=[head(20.0,120.0),head(26.0,117.5)];
        assert!(renderer_head_bound_stems(&cell(),&[a],&heads,&expected).is_some());
        assert!(renderer_head_bound_stems(&cell(),&[a],&heads[..1],&expected).is_none());
    }
    #[test]
    fn equal_counts_and_shared_stroke_without_heads_cannot_bind_two_staffs() {
        let mut content=String::new();
        for top in [100.0,150.0] {
            for line in 0..5 { let y=200.0-top-f64::from(line)*5.0; content.push_str(&format!("0.5 w 10 {y} m 110 {y} l S\n")); }
        }
        content.push_str("0.5 w 40 70 m 40 55 l S\n");
        let pdf=format!("%PDF-1.4\n1 0 obj\n<< /Type /Pages /Kids [2 0 R] /Count 1 >>\nendobj\n2 0 obj\n<< /Type /Page /Contents 3 0 R /Resources 4 0 R /MediaBox [0 0 120 200] >>\nendobj\n3 0 obj\n<< /Length {} >>\nstream\n{}endstream\nendobj\n4 0 obj\n<< /Font << >> >>\nendobj\n%%EOF\n",content.len(),content);
        let xml=synthetic_geometry_score(&["<note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><stem>up</stem><staff>1</staff></note><backup><duration>1</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>1</duration><stem>up</stem><staff>2</staff></note>"]);
        let readback=primary_beam_geometry_readback(&xml,pdf.as_bytes()).unwrap();
        assert_eq!(readback.bound_measure_staff_cells,0);
        assert_eq!(readback.unresolved_measure_staff_cells,2);
    }
}

#[cfg(test)]
mod chromatic_head_binding_tests {
    use super::*;
    fn setup(up: bool) -> (RendererStaffCell, PdfStrokeSegment, Vec<PdfTextEvent>, RenderStemOccurrence) {
        let cell = RendererStaffCell { page_index: 1, x_left: 10.0, x_right: 110.0, staff: 1, measure: 1, line_y_top: vec![100.0,105.0,110.0,115.0,120.0] };
        let stroke = PdfStrokeSegment { page_index:1, x1:26.25, x2:26.25, y1_top:if up {80.0} else {120.8}, y2_top:if up {119.2} else {150.0}, width:0.5 };
        let head = |x| PdfTextEvent { page_index:1, x, y_from_bottom:80.0, page_height:200.0, text:"\u{e0a4}".to_string(), head_bounds:Some([x,117.5,x+6.5,122.5]) };
        let occurrence=RenderStemOccurrence { measure:1,staff:1,onset:0,pitched_ordinal:0,head_steps:vec![0,0],head_pitches:vec![Some(60),Some(61)] };
        (cell,stroke,vec![head(20.0),head(26.0)],occurrence)
    }
    #[test]
    fn opposite_chromatic_heads_bind_in_both_directions() {
        for up in [false,true] {
            let (cell,stroke,heads,expected)=setup(up);
            assert!(renderer_head_binding(&cell,&[stroke],&heads,&[expected]).is_ok());
        }
    }
    #[test]
    fn missing_extra_or_same_side_heads_are_unknown() {
        let (cell,stroke,heads,expected)=setup(true);
        assert!(renderer_head_binding(&cell,&[stroke],&heads[..1],std::slice::from_ref(&expected)).is_err());
        let mut extra=heads.clone();let mut third=heads[0].clone();third.head_bounds.as_mut().unwrap()[0]+=0.1;extra.push(third);
        assert!(renderer_head_binding(&cell,&[stroke],&extra,std::slice::from_ref(&expected)).is_err());
        let mut same_side=vec![heads[0].clone(),heads[0].clone()];same_side[1].head_bounds.as_mut().unwrap()[0]+=0.1;
        assert!(renderer_head_binding(&cell,&[stroke],&same_side,&[expected]).is_err());
    }
    #[test]
    fn identical_pitch_duplicates_require_one_physical_head() {
        let (cell,stroke,heads,mut expected)=setup(true);expected.head_pitches=vec![Some(60),Some(60)];
        let duplicates=vec![heads[0].clone(),heads[0].clone()];
        assert!(renderer_head_binding(&cell,&[stroke],&duplicates,std::slice::from_ref(&expected)).is_ok());
        assert!(renderer_head_binding(&cell,&[stroke],&heads,&[expected]).is_err());
    }
    #[test]
    fn shared_head_requires_same_unique_pitch_and_temporal_staff_cell() {
        let (_,_,heads,mut left)=setup(true);let mut right=left.clone();
        let a=RendererBoundHead { bounds:heads[0].head_bounds.unwrap(),pitches:BTreeSet::from([60]) };let mut b=a.clone();
        assert!(renderer_shared_head_compatible(&a,&b,&left,&right));
        b.pitches=BTreeSet::from([61]);assert!(!renderer_shared_head_compatible(&a,&b,&left,&right));
        b.pitches=BTreeSet::from([60,61]);assert!(!renderer_shared_head_compatible(&a,&b,&left,&right));
        b=a.clone();right.onset+=1;assert!(!renderer_shared_head_compatible(&a,&b,&left,&right));
        right.onset=left.onset;right.staff+=1;assert!(!renderer_shared_head_compatible(&a,&b,&left,&right));
        right.staff=left.staff;left.measure+=1;assert!(!renderer_shared_head_compatible(&a,&b,&left,&right));
    }
    #[test]
    fn same_pitch_two_stems_still_require_a_unique_occurrence_assignment() {
        let (cell,a,heads,mut expected)=setup(true);expected.head_steps=vec![0];expected.head_pitches=vec![Some(60)];
        let mut b=a;b.y1_top-=10.0;
        assert_eq!(renderer_head_binding(&cell,&[a,b],&heads[..1],&[expected]).unwrap_err(),"multiple_complete_assignments");
    }
    #[test]
    fn relocation_and_independent_copy_keep_chromatic_attachment() {
        let (cell,stroke,heads,expected)=setup(true);
        for (dx,dy) in [(150.0,40.0),(310.0,80.0)] {
            let mut c=cell.clone();c.x_left+=dx;c.x_right+=dx;for y in &mut c.line_y_top {*y+=dy;}
            let mut s=stroke;s.x1+=dx;s.x2+=dx;s.y1_top+=dy;s.y2_top+=dy;
            let hs=heads.iter().cloned().map(|mut h|{h.x+=dx;h.y_from_bottom-=dy;let b=h.head_bounds.as_mut().unwrap();b[0]+=dx;b[2]+=dx;b[1]+=dy;b[3]+=dy;h}).collect::<Vec<_>>();
            assert!(renderer_head_binding(&c,&[stroke,s],&hs,std::slice::from_ref(&expected)).is_ok());
        }
    }
    #[test]
    fn pitch_parser_retains_chromatic_and_identical_duplicates_separately() {
        let xml=synthetic_geometry_score(&["<attributes><clef><sign>G</sign><line>2</line></clef></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><stem>up</stem></note><note><chord/><pitch><step>C</step><alter>1</alter><octave>4</octave></pitch><duration>1</duration><stem>up</stem></note><note><chord/><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><stem>up</stem></note>"]);
        let (_,rows)=render_stem_occurrences(&xml).unwrap();
        assert_eq!(rows[0].head_pitches,vec![Some(60),Some(61),Some(60)]);
        assert_eq!(rows[0].head_steps,vec![-2,-2,-2]);
    }
}

#[derive(Clone)]
enum RenderSurfaceChange {
    Clef(u8, Option<(i32,i32)>),
    Octave(u8,u16,RenderOctaveChange),
}
#[derive(Clone)]
enum RenderOctaveChange { Start(Option<i32>), Continue, Stop }
fn apply_render_surface_change(
    change: &RenderSurfaceChange,
    clefs: &mut BTreeMap<u8,(i32,i32)>,
    ottavas: &mut BTreeMap<(u8,u16),Option<i32>>,
) {
    match change {
        RenderSurfaceChange::Clef(staff, Some(clef)) => { clefs.insert(*staff,*clef); }
        RenderSurfaceChange::Clef(staff, None) => { clefs.remove(staff); }
        RenderSurfaceChange::Octave(staff,number,action) => {
            let key = (*staff,*number);
            match action {
                RenderOctaveChange::Start(amount) => { ottavas.insert(key,*amount); }
                RenderOctaveChange::Continue => { ottavas.entry(key).or_insert(None); }
                RenderOctaveChange::Stop => { ottavas.remove(&key); }
            }
        }
    }
}

#[cfg(test)]
mod timed_surface_tests {
    use super::*;
    #[test]
    fn octave_continue_preserves_the_same_registered_direction_until_stop() {
        let note="<note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><stem>up</stem><staff>1</staff></note>";
        let direction=|kind|format!("<direction><direction-type><octave-shift type=\"{kind}\" size=\"8\" number=\"2\"/></direction-type><staff>1</staff></direction>");
        let first=format!("<attributes><clef><sign>G</sign><line>2</line></clef></attributes>{}{note}",direction("down"));
        let second=format!("{}{note}{}{note}",direction("continue"),direction("stop"));
        let xml=synthetic_geometry_score(&[&first,&second]);
        let (_,rows)=render_stem_occurrences(&xml).unwrap();
        assert_eq!(rows.iter().map(|r|r.head_steps.clone()).collect::<Vec<_>>(),vec![vec![-2],vec![-2],vec![5]]);
    }
    #[test]
    fn backed_up_voice_uses_clef_at_its_onset_instead_of_later_serialized_clef() {
        let note=|step|format!("<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>1</duration><stem>up</stem><staff>1</staff></note>");
        let body=format!("<attributes><clef><sign>G</sign><line>2</line></clef></attributes>{}<attributes><clef><sign>F</sign><line>4</line></clef></attributes>{}<backup><duration>2</duration></backup>{}",note("C"),note("C"),note("D"));
        let xml=synthetic_geometry_score(&[&body]);
        let (_,rows)=render_stem_occurrences(&xml).unwrap();
        assert_eq!(rows.iter().map(|r|r.head_steps.clone()).collect::<Vec<_>>(),vec![vec![5],vec![17],vec![6]]);
    }
    #[test]
    fn unmatched_octave_continuation_is_unknown_until_its_matching_stop() {
        let xml=synthetic_geometry_score(&["<attributes><clef><sign>G</sign><line>2</line></clef></attributes><direction><direction-type><octave-shift type=\"continue\" size=\"8\" number=\"3\"/></direction-type><staff>1</staff></direction><note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><stem>up</stem><staff>1</staff></note>"]);
        let (_,rows)=render_stem_occurrences(&xml).unwrap();
        assert!(rows[0].head_steps[0].abs()>128);
    }
}

#[cfg(test)]
mod embedded_glyph_geometry_tests {
    use super::*;
    fn pdf(font_name: &str, include_head: bool) -> Vec<u8> {
        let mut font=vec![0u8;126];
        font[..4].copy_from_slice(&[0,1,0,0]);
        font[4..6].copy_from_slice(&4u16.to_be_bytes());
        for (i,(name,start,length)) in [(b"head",76u32,54u32),(b"loca",130,6),(b"glyf",136,10),(b"maxp",146,6)].iter().enumerate() {
            let offset=12+16*i;
            if font.len()<152 {font.resize(152,0);}
            font[offset..offset+4].copy_from_slice(*name);
            font[offset+8..offset+12].copy_from_slice(&start.to_be_bytes());
            font[offset+12..offset+16].copy_from_slice(&length.to_be_bytes());
        }
        font[94..96].copy_from_slice(&1000u16.to_be_bytes());
        font[134..136].copy_from_slice(&5u16.to_be_bytes());
        font[136..138].copy_from_slice(&1u16.to_be_bytes());
        for (offset,value) in [(138,0i16),(140,-125),(142,325),(144,125)] { font[offset..offset+2].copy_from_slice(&value.to_be_bytes()); }
        font[150..152].copy_from_slice(&2u16.to_be_bytes());
        let mut content=String::new();
        for top in [100.0,150.0] { for line in 0..5 { let y=200.0-top-f64::from(line)*5.0; content.push_str(&format!("0.5 w 10 {y} m 110 {y} l S\n")); } }
        content.push_str("0.5 w 26.25 140 m 26.25 80.8 l S\n");
        if include_head { content.push_str("BT /F1 20 Tf 1 0 0 1 20 80 Tm <0001> Tj ET\n"); }
        let cmap=b"1 beginbfchar\n<0001> <E0A4>\nendbfchar\n";
        let stream=|bytes:&[u8]| { let mut out=format!("<< /Length {} >>\nstream\n",bytes.len()).into_bytes(); out.extend_from_slice(bytes); out.extend_from_slice(b"\nendstream"); out };
        let objects=vec![
            b"<< /Type /Pages /Kids [2 0 R] /Count 1 >>".to_vec(),
            b"<< /Type /Page /Contents 3 0 R /Resources 4 0 R /MediaBox [0 0 120 200] >>".to_vec(),
            stream(content.as_bytes()),b"<< /Font << /F1 5 0 R >> >>".to_vec(),
            format!("<< /Type /Font /Subtype /Type0 /Encoding /Identity-H /BaseFont /{font_name} /DescendantFonts [6 0 R] /ToUnicode 7 0 R >>").into_bytes(),
            b"<< /Subtype /CIDFontType2 /CIDToGIDMap /Identity /FontDescriptor 8 0 R >>".to_vec(),
            stream(cmap),b"<< /FontFile2 9 0 R >>".to_vec(),stream(&font)
        ];
        let mut out=b"%PDF-1.4\n".to_vec();
        for (i,body) in objects.iter().enumerate() { out.extend_from_slice(format!("{} 0 obj\n",i+1).as_bytes()); out.extend_from_slice(body); out.extend_from_slice(b"\nendobj\n"); }
        out
    }
    fn xml() -> String { synthetic_geometry_score(&["<attributes><clef><sign>G</sign><line>2</line></clef></attributes><note><pitch><step>E</step><octave>4</octave></pitch><duration>1</duration><stem>up</stem><staff>1</staff></note>"]) }
    #[test]
    fn complete_cmap_embedded_glyph_and_long_stroke_bind_through_the_real_reader() {
        let readback=primary_beam_geometry_readback(&xml(),&pdf("Leland",true)).unwrap();
        assert_eq!(readback.unresolved_measure_staff_cells,0);
        assert_eq!(readback.stem_geometry_by_occurrence.len(),1);
        let stem=readback.stem_geometry_by_occurrence[&(1,1,0)];
        assert!(stem.y_max-stem.y_min>35.0);
    }
    #[test]
    fn missing_glyph_or_unsupported_font_cannot_authorize_that_identical_stroke() {
        for input in [pdf("Leland",false),pdf("Unsupported",true)] {
            let readback=primary_beam_geometry_readback(&xml(),&input).unwrap();
            assert_eq!(readback.unresolved_measure_staff_cells,1);
            assert!(readback.stem_geometry_by_occurrence.is_empty());
        }
    }
}

#[cfg(test)]
fn synthetic_geometry_score(bodies: &[&str]) -> String {
    let measures = bodies.iter().enumerate().map(|(index, body)| {
        let ordinal=index+1;
        format!("<measure number=\"{ordinal}\">{body}</measure>")
    }).collect::<String>();
    format!("<score-partwise><part>{measures}</part></score-partwise>")
}


// Explicit user authorization, 2026-10-20: preserve the accepted E007
// decision course, including its negative decisions. This historical observer
// is NOT a geometry admission for the corrected beam writer. Its opaque result
// can only bind a purpose-marked historical metric planning course; the PDF
// finalization entry rejects that purpose. Original occurrence identities and
// the unchanged phrase policy remain the musical authorities.
pub(crate) struct HistoricalPhrasePolicyDecisionGeometry {
    observed: RendererPrimaryBeamGeometryReadback,
}
impl HistoricalPhrasePolicyDecisionGeometry {
    pub(crate) fn metric_decision_fragment_count(&self, measure: usize, staff: u8, ordinals: &[usize]) -> Option<usize> {
        self.observed.cohort_geometry(measure,staff,ordinals).map(|g|g.visible_fragments)
    }
}
pub(crate) fn historical_phrase_policy_decision_geometry(xml: &str, pdf: &Path) -> anyhow::Result<HistoricalPhrasePolicyDecisionGeometry> {
    let observed=historical_phrase_primary_beam_geometry_readback(xml,&std::fs::read(pdf)?)?;
    eprintln!("HistoricalPhrasePolicyDecisionGeometry: bound_cells={}; unknown_cells={}; purpose=accepted_E007_decision_only; live_beam_admission=forbidden",observed.bound_measure_staff_cells,observed.unresolved_measure_staff_cells);
    Ok(HistoricalPhrasePolicyDecisionGeometry { observed })
}
#[derive(Clone)]
struct HistoricalPhraseStemOccurrence {
    measure: usize, staff: u8, onset: i64, pitched_ordinal: usize,
}
fn historical_phrase_primary_beam_geometry_readback(
    render_xml: &str,
    pdf_bytes: &[u8],
) -> anyhow::Result<RendererPrimaryBeamGeometryReadback> {
    let (measure_count, stem_occurrences) = historical_phrase_render_stem_occurrences(render_xml)?;
    let (segments, polygons) = extract_pdf_vector_geometry(pdf_bytes)?;
    let cells = renderer_staff_cells(&segments, measure_count)?;
    let mut expected_by_cell = BTreeMap::<(usize, u8), Vec<HistoricalPhraseStemOccurrence>>::new();
    for occurrence in stem_occurrences {
        expected_by_cell
            .entry((occurrence.measure, occurrence.staff))
            .or_default()
            .push(occurrence);
    }
    for occurrences in expected_by_cell.values_mut() {
        occurrences.sort_by_key(|row| (row.onset, row.pitched_ordinal));
    }

    let mut stem_geometry_by_occurrence = BTreeMap::new();
    let mut beam_spans_by_measure_staff = BTreeMap::new();
    let mut bound_measure_staff_cells = 0usize;
    let mut unresolved_measure_staff_cells = 0usize;

    for cell in &cells {
        let key = (cell.measure, cell.staff);
        let expected = expected_by_cell.get(&key).cloned().unwrap_or_default();
        let strictly_unique_onsets = expected
            .windows(2)
            .all(|pair| pair[0].onset < pair[1].onset);
        let stems = historical_phrase_renderer_cell_stems(cell, &segments);
        if strictly_unique_onsets && stems.len() == expected.len() {
            bound_measure_staff_cells = bound_measure_staff_cells.saturating_add(1);
            for (occurrence, stem) in expected.iter().zip(stems.iter().copied()) {
                stem_geometry_by_occurrence.insert(
                    (occurrence.measure, occurrence.staff, occurrence.pitched_ordinal),
                    stem,
                );
            }
        } else if !expected.is_empty() {
            unresolved_measure_staff_cells = unresolved_measure_staff_cells.saturating_add(1);
        }
        let spans = historical_phrase_renderer_cell_beam_spans(cell, &polygons);
        if !spans.is_empty() {
            beam_spans_by_measure_staff.insert(key, spans);
        }
    }

    Ok(RendererPrimaryBeamGeometryReadback {
        stem_geometry_by_occurrence,
        beam_spans_by_measure_staff,
        measure_count,
        bound_measure_staff_cells,
        unresolved_measure_staff_cells,
    })
}

fn historical_phrase_render_stem_occurrences(
    xml: &str,
) -> anyhow::Result<(usize, Vec<HistoricalPhraseStemOccurrence>)> {
    let document = Document::parse(xml)?;
    let part = document
        .descendants()
        .find(|node| node.is_element() && node.has_tag_name("part"))
        .ok_or_else(|| anyhow::anyhow!("RendererPrimaryBeamGeometryReadback: MusicXML part fehlt"))?;
    let measures = part
        .children()
        .filter(|node| node.is_element() && node.has_tag_name("measure"))
        .collect::<Vec<_>>();
    if measures.is_empty() {
        anyhow::bail!("RendererPrimaryBeamGeometryReadback: MusicXML measures fehlen");
    }
    let mut out = Vec::new();
    for (measure_index, measure) in measures.iter().enumerate() {
        let measure_number = measure
            .attribute("number")
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "RendererPrimaryBeamGeometryReadback: ungueltige Measure-Nummer an Slot {}",
                    measure_index + 1
                )
            })?;
        if measure_number != measure_index + 1 {
            anyhow::bail!(
                "RendererPrimaryBeamGeometryReadback: Measure {} steht an Slot {}",
                measure_number,
                measure_index + 1
            );
        }
        let mut cursor = 0_i64;
        let mut last_note_onset = 0_i64;
        let mut pitched_ordinal = 0usize;
        for node in measure.children().filter(|node| node.is_element()) {
            if node.has_tag_name("backup") {
                cursor = cursor.saturating_sub(pdf_readback_child_duration(node).max(0));
                continue;
            }
            if node.has_tag_name("forward") {
                cursor = cursor.saturating_add(pdf_readback_child_duration(node).max(0));
                continue;
            }
            if !node.has_tag_name("note") {
                continue;
            }
            let duration = pdf_readback_child_duration(node).max(0);
            let is_chord = node
                .children()
                .any(|child| child.is_element() && child.has_tag_name("chord"));
            let onset = if is_chord { last_note_onset } else { cursor };
            if !is_chord {
                last_note_onset = onset;
            }
            let has_pitch = node
                .children()
                .any(|child| child.is_element() && child.has_tag_name("pitch"));
            let event_pitched_ordinal = has_pitch.then_some(pitched_ordinal);
            if has_pitch {
                pitched_ordinal = pitched_ordinal.saturating_add(1);
            }
            let visible = node.attribute("print-object") != Some("no");
            let stem = pdf_readback_child_text(node, "stem");
            if visible
                && has_pitch
                && !is_chord
                && matches!(stem.as_deref(), Some("up") | Some("down"))
            {
                let staff = pdf_readback_child_text(node, "staff")
                    .and_then(|value| value.parse::<u8>().ok())
                    .unwrap_or(1);
                if let Some(pitched_ordinal) = event_pitched_ordinal {
                    out.push(HistoricalPhraseStemOccurrence {
                        measure: measure_number,
                        staff,
                        onset,
                        pitched_ordinal,
                    });
                }
            }
            if !is_chord {
                cursor = cursor.saturating_add(duration);
            }
        }
    }
    Ok((measures.len(), out))
}

fn historical_phrase_renderer_cell_stems(cell: &RendererStaffCell, segments: &[PdfStrokeSegment]) -> Vec<RendererStemGeometry> {
    let staff_top = cell.line_y_top.first().copied().unwrap_or(0.0);
    let staff_bottom = cell.line_y_top.last().copied().unwrap_or(0.0);
    let mut stems = segments
        .iter()
        .filter_map(|segment| {
            if segment.page_index != cell.page_index {
                return None;
            }
            let dx = (segment.x2 - segment.x1).abs();
            let dy = (segment.y2_top - segment.y1_top).abs();
            let x = (segment.x1 + segment.x2) / 2.0;
            let y_min = segment.y1_top.min(segment.y2_top);
            let y_max = segment.y1_top.max(segment.y2_top);
            (dx <= 0.5
                && (8.0..=35.0).contains(&dy)
                && (0.35..=0.65).contains(&segment.width)
                && x > cell.x_left + 1.0
                && x < cell.x_right - 1.0
                && y_min >= staff_top - 35.0
                && y_max <= staff_bottom + 35.0)
                .then_some(RendererStemGeometry { x, y_min, y_max })
        })
        .collect::<Vec<_>>();
    stems.sort_by(|left, right| left.x.total_cmp(&right.x));
    let mut deduplicated = Vec::<RendererStemGeometry>::new();
    for stem in stems {
        if deduplicated
            .last()
            .is_none_or(|previous| (stem.x - previous.x).abs() > 0.8)
        {
            deduplicated.push(stem);
        }
    }
    deduplicated
}

fn historical_phrase_renderer_cell_beam_spans(
    cell: &RendererStaffCell,
    polygons: &[PdfFilledPolygon],
) -> Vec<RendererBeamSpan> {
    let staff_top = cell.line_y_top.first().copied().unwrap_or(0.0);
    let staff_bottom = cell.line_y_top.last().copied().unwrap_or(0.0);
    let mut spans = polygons
        .iter()
        .filter_map(|polygon| {
            if polygon.page_index != cell.page_index {
                return None;
            }
            let span = renderer_beam_span(polygon)?;
            let center_y = (span.y_left + span.y_right) / 2.0;
            (span.x_left >= cell.x_left - 1.0
                && span.x_right <= cell.x_right + 1.0
                && center_y >= staff_top - 45.0
                && center_y <= staff_bottom + 45.0)
                .then_some(span)
        })
        .collect::<Vec<_>>();
    spans.sort_by(|left, right| {
        left.x_left
            .total_cmp(&right.x_left)
            .then_with(|| left.x_right.total_cmp(&right.x_right))
            .then_with(|| left.y_left.total_cmp(&right.y_left))
    });
    let mut deduplicated = Vec::<RendererBeamSpan>::new();
    for span in spans {
        if deduplicated.last().is_none_or(|previous| {
            (span.x_left - previous.x_left).abs() > 0.8
                || (span.x_right - previous.x_right).abs() > 0.8
                || (span.y_left - previous.y_left).abs() > 0.8
                || (span.y_right - previous.y_right).abs() > 0.8
        }) {
            deduplicated.push(span);
        }
    }
    deduplicated
}
