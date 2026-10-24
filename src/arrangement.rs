// v0.10.726 real crate-root module split from the old score grouping.
// No behavior change: original module body, visibility and call paths are preserved.
    use std::collections::{BTreeMap, BTreeSet};
    #[path = "rhythmic_representation_transport.rs"]
    pub(crate) mod representation_transport;
    #[path = "arrangement_rhythmic_courses.rs"]
    mod rhythmic_courses;
    use rhythmic_courses::{ArrangementRhythmicCourses, CourseMeasure};
    pub(crate) use rhythmic_courses::continuation::OwnedRhythmicContinuation;
    #[path = "physical_rhythmic_births.rs"]
    pub(crate) mod physical_births;
    use physical_births::{BirthDrafts, PhysicalBirthKind, PhysicalRhythmicBirths};
    pub(crate) struct ArrangementRhythmicMint { _sealed: () }
    use crate::master_state_identity::{GenesisInputBinding, BlockTransformationReceipt, MasterGenesisIdentity};
    /// Sealed mint capability of the arrangement generator (GENESIS #62).
    pub(crate) struct ArrangementGenesisMint {
        _sealed: (),
    }
    /// Sealed mint capability of the visible-rhythm normalizer (BLOCK #52).
    pub(crate) struct VisibleRhythmBlockMint {
        _sealed: (),
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HumanPotentialStructuralTarget {
        measure_ordinal: usize,
        candidate_rank: usize,
        score: i64,
    }
    impl HumanPotentialStructuralTarget {
        pub fn measure_ordinal(self) -> usize {
            self.measure_ordinal
        }
        pub fn candidate_rank(self) -> usize {
            self.candidate_rank
        }
        pub fn score(self) -> i64 {
            self.score
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ArrangementMaterialDispositionKind {
        VisiblePitched,
        HiddenPitched,
        RestifiedPitched,
        RestOrOther,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ArrangementMaterialDispositionRow {
        pub measure_ordinal: usize,
        pub raw_note_index: usize,
        pub disposition: ArrangementMaterialDispositionKind,
        pub projected_staff: u8,
        pub human_potential_moved: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ArrangementRepresentationLineageRow {
        pub measure_ordinal: usize,
        pub pre_ledger_raw_note_index: usize,
        pub post_ledger_raw_note_index: usize,
        pub fragment_ordinal: usize,
    }

    #[derive(Debug, Clone, Default)]
    pub struct ArrangementStats {
        pub(crate) rhythmic_courses: Option<std::sync::Arc<ArrangementRhythmicCourses>>,
        pub(crate) owned_rhythm: Option<std::sync::Arc<OwnedRhythmicContinuation>>,
        pub(crate) owned_rhythm_unknown: Option<String>,
        pub(crate) physical_births: Vec<PhysicalRhythmicBirths>,
        pub(crate) physical_birth_capture_unknown: usize,
        pub measures_seen: usize,
        pub notes_seen: usize,
        pub staff_added: usize,
        pub voice_added: usize,
        pub stems_added: usize,
        pub measures_simplified: usize,
        pub notes_hidden: usize,
        pub rests_hidden: usize,
        pub chord_tones_hidden: usize,
        pub notes_restified_as_rests: usize,
        pub visible_notes_normalized: usize,
        pub visible_rhythm_rests_inserted: usize,
        pub visible_rhythm_values_split: usize,
        pub chord_cohorts_source_bound_split: usize,
        pub visible_rhythm_measures_completed: usize,
        pub bass_anchors_restored: usize,
        pub accompaniment_notes_restored: usize,
        pub left_sparse_measures: usize,
        pub left_gap_notes_restored: usize,
        pub section_rule_measures: usize,
        pub section_rule_notes_restored: usize,
        pub human_potential_measures: usize,
        pub human_potential_notes_moved: usize,
        pub human_potential_budget: usize,
        pub human_potential_candidates_generated: usize,
        pub human_potential_candidates_checked: usize,
        pub human_potential_candidates_rejected: usize,
        pub human_potential_eligible_measures: usize,
        pub human_potential_local_safe_candidates: usize,
        pub human_potential_no_pitch_band: usize,
        pub human_potential_no_candidates: usize,
        pub human_potential_reject_duplicate_onset: usize,
        pub human_potential_reject_right_substance: usize,
        pub human_potential_reject_range: usize,
        pub human_potential_reject_jump: usize,
        pub human_potential_global_guard_rejected: usize,
        pub human_potential_global_guard_blockers: usize,
        pub human_potential_global_guard_warnings: usize,
        pub human_potential_global_guard_hints: usize,
        pub human_potential_global_guard_critical: usize,
        pub human_potential_global_guard_fidelity: usize,
        pub human_potential_global_guard_retention: usize,
        pub human_potential_global_candidates_checked: usize,
        pub human_potential_global_fallback_rank: usize,
        /// Strukturelle MaterialDisposition des bestehenden Arrangement-Owners.
        /// Die Zeilen tragen bewusst keine SourceIdentity. Sie beschreiben nur
        /// die bereits getroffene lokale Entscheidung je MusicXML-Note vor dem
        /// sichtbaren RhythmLedger: pitched sichtbar, pitched verborgen,
        /// pitch-entfernt/restifiziert oder Rest/sonstiger Traeger.
        pub material_disposition_rows: Vec<ArrangementMaterialDispositionRow>,
        /// Rein struktureller Forward-Carrier des bestehenden VisibleRhythmLedger:
        /// ursprünglicher RawNote-Index vor dem Ledger -> resultierende Fragment-Indizes danach.
        /// Diese Zeilen tragen absichtlich keine SourceIdentity und keine musikalische Entscheidung.
        pub representation_lineage_rows: Vec<ArrangementRepresentationLineageRow>,
        /// Exakte Anzahl der RawNote-Fragmente je Takt direkt hinter dem VisibleRhythmLedger.
        /// Dient nur dazu, die Gueltigkeit der strukturellen Forward-Indizes spaeter fail-closed zu pruefen.
        pub representation_post_raw_note_counts: Vec<usize>,
        /// Strukturell aus dem aktuell analysierten MusicXML gebundene HumanPotential-Ziele.
        /// Die privaten Felder verhindern, dass externe Caller eine rohe Taktnummer als Ziel konstruieren.
        pub human_potential_structural_targets: Vec<HumanPotentialStructuralTarget>,
        /// v0.1.111: kurze, maschinenlesbare Top-Kandidaten plus CrossStaff-Mutationsaudit fuer Diagnose/Weiterarbeit.
        /// Diese Strings sind reine Reportprojektion und werden nicht mehr in Produktselektoren zurueckgeparst.
        pub human_potential_candidate_details: Vec<String>,
        /// v0.1.111: konkrete globale Blocker-Beispiele aus der Guard-Pruefung.
        pub human_potential_global_blocker_details: Vec<String>,
        /// v0.1.113: explizit angenommene HumanPotential-Kandidaten fuer Analyse/Status.
        pub human_potential_accepted_candidate_details: Vec<String>,
        /// v0.1.113: hoch bewertete, aber global abgelehnte Kandidaten mit Grund.
        pub human_potential_rejected_high_potential_details: Vec<String>,
        /// v0.1.115: lokale HintRepair-/Audit-Zwischenstufe fuer starke HumanPotential-Kandidaten.
        pub human_potential_hint_repair_details: Vec<String>,
    }
    #[derive(Debug)]
    pub struct ArrangementResult {
        pub xml: String,
        pub stats: ArrangementStats,
        pub(crate) arrangement_source_genesis: Option<MasterGenesisIdentity>,
        pub(crate) visible_rhythm_block_identity: Option<BlockTransformationReceipt>,
    }
    #[derive(Clone, Copy, Default)]
    pub struct ArrangementProfile<'a> {
        pub(crate) rhythmic_courses: Option<&'a crate::midi_seed::MidiSeedRhythmicCourses>,
        /// Wenn MIDI-Quellmaterial bzw. WAV-Referenzevidenz vorhanden ist, wird konservativer
        /// ausgedünnt: Die Fassung soll spielbar bleiben, aber den Klangcharakter
        /// der Quelle nicht unnötig verlieren.
        pub preserve_reference: bool,
        /// Eigenständige HumanPotential-Prüfung: vorhandene rechte-Hand-Stütztöne
        /// dürfen ins untere System gelegt werden, aber nur als separat geprüfte
        /// Variante und ohne neue Noten.
        pub human_potential_enabled: bool,
        /// Lokales Kandidatenbudget fuer die HumanPotential-Kombinatorik.
        /// 0 bedeutet: interner Standardwert.
        pub human_potential_candidate_budget: usize,
        /// Maximalzahl gleichzeitig verschobener Stuetztöne pro Takt.
        /// 0 bedeutet: interner Standardwert.
        pub human_potential_max_moves: usize,
        /// Generische TargetMeasureIsolation als nicht faelschbarer struktureller Locator.
        /// None bedeutet: alle im aktuellen Dokument strukturell erkannten Ziele diagnostisch betrachten.
        /// Some(...) kann nur aus einem zuvor von diesem Arrangement-Owner erzeugten Target stammen.
        pub human_potential_structural_target: Option<HumanPotentialStructuralTarget>,
    }
    impl std::fmt::Debug for ArrangementProfile<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ArrangementProfile")
                .field("preserve_reference", &self.preserve_reference)
                .field("human_potential_enabled", &self.human_potential_enabled)
                .field("human_potential_candidate_budget", &self.human_potential_candidate_budget)
                .field("human_potential_max_moves", &self.human_potential_max_moves)
                .field("human_potential_structural_target", &self.human_potential_structural_target)
                .finish()
        }
    }
    #[derive(Debug, Clone)]
    struct MeasureSpan {
        start: usize,
        end: usize,
    }
    #[derive(Debug, Clone)]
    struct RawNote {
        start: usize,
        end: usize,
        text: String,
    }
    #[derive(Debug, Clone)]
    struct NoteMeta {
        raw_idx: usize,
        start: i64,
        duration: i64,
        staff: PianoStaff,
        voice: String,
        is_rest: bool,
        is_chord: bool,
        midi: Option<i32>,
    }
    #[derive(Debug, Clone, Copy, Default)]
    struct LeftHandSupportStats {
        sparse_measure: bool,
        notes_restored: usize,
        section_rule_measure: bool,
        section_notes_restored: usize,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum PianoStaff {
        Right,
        Left,
    }
    impl PianoStaff {
        fn number(self) -> &'static str {
            match self {
                PianoStaff::Right => "1",
                PianoStaff::Left => "2",
            }
        }
        fn default_voice(self) -> &'static str {
            match self {
                PianoStaff::Right => "1",
                PianoStaff::Left => "2",
            }
        }
        fn stem(self) -> &'static str {
            match self {
                PianoStaff::Right => "up",
                PianoStaff::Left => "down",
            }
        }
    }
    #[derive(Debug, Clone, Copy)]
    struct RhythmContext {
        divisions: i64,
        beats: i64,
        beat_type: i64,
    }
    impl Default for RhythmContext {
        fn default() -> Self {
            Self { divisions: 96, beats: 4, beat_type: 4 }
        }
    }
    impl RhythmContext {
        fn update_from_measure(self, measure_text: &str) -> Self {
            Self {
                divisions: extract_first_i64(measure_text, "divisions").unwrap_or(self.divisions).max(1),
                beats: extract_first_i64(measure_text, "beats").unwrap_or(self.beats).max(1),
                beat_type: extract_first_i64(measure_text, "beat-type").unwrap_or(self.beat_type).max(1),
            }
        }
        fn expected_duration(self) -> i64 {
            self.divisions * self.beats * 4 / self.beat_type
        }
        fn beat_duration(self) -> i64 {
            (self.divisions * 4 / self.beat_type).max(1)
        }
    }
    /// PianoArrangementModel: erzeugt aus einer formal korrekten MusicXML-Datei
    /// eine bewusst lesbarere Pianistenfassung.
    ///
    /// Wichtig: Die Zeitachsen werden nicht zerstört. Überzählige Innenstimmen,
    /// dichte Akkordtöne und störende Hilfspausen werden nicht gelöscht, sondern
    /// per `print-object="no" print-spacing="no"` aus dem Notenbild genommen.
    /// Dadurch bleibt die Datei technisch stabil, während das MuseScore-PDF deutlich
    /// näher an eine spielbare Klavierfassung rückt.
    pub fn arrange_piano_musicxml_with_profile(xml: &str, profile: ArrangementProfile) -> ArrangementResult {
        let profile_contract = format!("{profile:?}");
        let received_courses = profile.rhythmic_courses.map(|seed| {
            ArrangementRhythmicCourses::receive(seed, xml)
                .expect("Exact seed course custody at arrangement entry")
        });
        let mut continued_occurrences = Vec::new();
        let mut creation_history = BTreeMap::new();
        let mut course_offset = 0;
        let spans = find_measure_spans(xml);
        if spans.is_empty() {
            let arrangement_source_genesis = MasterGenesisIdentity::mint_genesis(
                ArrangementGenesisMint { _sealed: () },
                &profile_contract,
                GenesisInputBinding::input_document(xml),
                xml,
            );
            return ArrangementResult {
                xml: xml.to_string(),
                stats: ArrangementStats::default(),
                arrangement_source_genesis: Some(arrangement_source_genesis),
                visible_rhythm_block_identity: None,
            };
        }
        let mut out = String::with_capacity(xml.len() + 1024);
        let mut last = 0_usize;
        let mut stats = ArrangementStats::default();
        let mut rhythm_context = RhythmContext::default();
        for span in spans {
            out.push_str(&xml[last..span.start]);
            let measure_text = &xml[span.start..span.end];
            rhythm_context = rhythm_context.update_from_measure(measure_text);
            let mut course_measure = received_courses.as_ref().map(|courses| {
                courses.measure(measure_text, course_offset, rhythm_context.expected_duration())
                    .expect("Complete received measure custody")
            });
            let (arranged, local) = arrange_measure(measure_text, profile, rhythm_context,
                stats.measures_seen + 1, course_measure.as_mut());
            if let Some(courses) = course_measure {
                let (occurrences, history) = courses.take();
                continued_occurrences.extend(occurrences);
                creation_history.extend(history);
            }
            course_offset += rhythm_context.expected_duration();
            stats.measures_seen += 1;
            stats.notes_seen += local.notes_seen;
            stats.staff_added += local.staff_added;
            stats.voice_added += local.voice_added;
            stats.stems_added += local.stems_added;
            stats.measures_simplified += local.measures_simplified;
            stats.notes_hidden += local.notes_hidden;
            stats.rests_hidden += local.rests_hidden;
            stats.chord_tones_hidden += local.chord_tones_hidden;
            stats.visible_notes_normalized += local.visible_notes_normalized;
            stats.visible_rhythm_rests_inserted += local.visible_rhythm_rests_inserted;
            stats.visible_rhythm_values_split += local.visible_rhythm_values_split;
            stats.chord_cohorts_source_bound_split += local.chord_cohorts_source_bound_split;
            stats.visible_rhythm_measures_completed += local.visible_rhythm_measures_completed;
            stats.bass_anchors_restored += local.bass_anchors_restored;
            stats.accompaniment_notes_restored += local.accompaniment_notes_restored;
            stats.left_sparse_measures += local.left_sparse_measures;
            stats.left_gap_notes_restored += local.left_gap_notes_restored;
            stats.section_rule_measures += local.section_rule_measures;
            stats.section_rule_notes_restored += local.section_rule_notes_restored;
            stats.human_potential_measures += local.human_potential_measures;
            stats.human_potential_notes_moved += local.human_potential_notes_moved;
            stats.human_potential_budget = stats.human_potential_budget.max(local.human_potential_budget);
            stats.human_potential_candidates_generated += local.human_potential_candidates_generated;
            stats.human_potential_candidates_checked += local.human_potential_candidates_checked;
            stats.human_potential_candidates_rejected += local.human_potential_candidates_rejected;
            stats.human_potential_eligible_measures += local.human_potential_eligible_measures;
            stats.human_potential_local_safe_candidates += local.human_potential_local_safe_candidates;
            stats.human_potential_no_pitch_band += local.human_potential_no_pitch_band;
            stats.human_potential_no_candidates += local.human_potential_no_candidates;
            stats.human_potential_reject_duplicate_onset += local.human_potential_reject_duplicate_onset;
            stats.human_potential_reject_right_substance += local.human_potential_reject_right_substance;
            stats.human_potential_reject_range += local.human_potential_reject_range;
            stats.human_potential_reject_jump += local.human_potential_reject_jump;
            stats.human_potential_global_guard_rejected += local.human_potential_global_guard_rejected;
            stats.human_potential_global_guard_blockers += local.human_potential_global_guard_blockers;
            stats.human_potential_global_guard_warnings += local.human_potential_global_guard_warnings;
            stats.human_potential_global_guard_hints += local.human_potential_global_guard_hints;
            stats.human_potential_global_guard_critical += local.human_potential_global_guard_critical;
            stats.human_potential_global_guard_fidelity += local.human_potential_global_guard_fidelity;
            stats.human_potential_global_guard_retention += local.human_potential_global_guard_retention;
            stats.human_potential_global_candidates_checked += local.human_potential_global_candidates_checked;
            stats.human_potential_global_fallback_rank = stats.human_potential_global_fallback_rank.max(local.human_potential_global_fallback_rank);
            stats
                .material_disposition_rows
                .extend(local.material_disposition_rows.iter().copied());
            stats
                .representation_lineage_rows
                .extend(local.representation_lineage_rows.iter().copied());
            stats
                .representation_post_raw_note_counts
                .extend(local.representation_post_raw_note_counts.iter().copied());
            extend_structural_targets_limited(
                &mut stats.human_potential_structural_targets,
                &local.human_potential_structural_targets,
                18,
            );
            extend_limited(&mut stats.human_potential_candidate_details, &local.human_potential_candidate_details, 18);
            extend_limited(&mut stats.human_potential_global_blocker_details, &local.human_potential_global_blocker_details, 18);
            extend_limited(&mut stats.human_potential_accepted_candidate_details, &local.human_potential_accepted_candidate_details, 18);
            extend_limited(&mut stats.human_potential_rejected_high_potential_details, &local.human_potential_rejected_high_potential_details, 18);
            out.push_str(&arranged);
            last = span.end;
        }
        out.push_str(&xml[last..]);
        stats.rhythmic_courses = received_courses.map(|courses| {
            std::sync::Arc::new(courses.finish(continued_occurrences, creation_history, &out)
                .expect("Complete initial arrangement course continuation"))
        });
        let arrangement_source_genesis = MasterGenesisIdentity::mint_genesis(
            ArrangementGenesisMint { _sealed: () },
            &profile_contract,
            GenesisInputBinding::input_document(xml),
            &out,
        );
        ArrangementResult {
            xml: out,
            stats,
            arrangement_source_genesis: Some(arrangement_source_genesis),
            visible_rhythm_block_identity: None,
        }
    }
    fn extend_structural_targets_limited(
        target: &mut Vec<HumanPotentialStructuralTarget>,
        source: &[HumanPotentialStructuralTarget],
        limit: usize,
    ) {
        for item in source {
            if target.len() >= limit {
                break;
            }
            if !target.contains(item) {
                target.push(*item);
            }
        }
    }
    fn extend_limited(target: &mut Vec<String>, source: &[String], limit: usize) {
        for item in source {
            if target.len() >= limit {
                break;
            }
            if !target.iter().any(|existing| existing == item) {
                target.push(item.clone());
            }
        }
    }
    /// VisibleRhythmLedger v0.1.68: normalisiert eine bereits arrangierte oder
    /// atomar gepatchte MusicXML-Datei erneut auf sichtbare rhythmische Lesbarkeit.
    /// Das ist absichtlich separat von der Pianistik: erst wird repariert/normalisiert,
    /// danach darf der harte Validator blockieren.
    pub fn normalize_visible_rhythm_musicxml(xml: &str) -> ArrangementResult {
        let spans = find_measure_spans(xml);
        if spans.is_empty() {
            let visible_rhythm_block_identity = BlockTransformationReceipt::mint_block_receipt(
                VisibleRhythmBlockMint { _sealed: () },
                xml,
                xml,
                None,
            );
            return ArrangementResult {
                xml: xml.to_string(),
                stats: ArrangementStats::default(),
                arrangement_source_genesis: None,
                visible_rhythm_block_identity,
            };
        }
        let mut out = String::with_capacity(xml.len() + 2048);
        let mut last = 0_usize;
        let mut stats = ArrangementStats::default();
        let mut rhythm_context = RhythmContext::default();
        let mut births = BirthDrafts::default();
        for span in spans {
            out.push_str(&xml[last..span.start]);
            let measure_text = &xml[span.start..span.end];
            rhythm_context = rhythm_context.update_from_measure(measure_text);
            let (rebuilt, ledger_stats) = apply_visible_rhythm_ledger(measure_text, rhythm_context, None, true);
            births.append_measure(ledger_stats.births.clone(), span.start, out.len());
            stats.measures_seen += 1;
            stats.representation_lineage_rows.extend(
                ledger_stats
                    .fragment_lineage_rows
                    .iter()
                    .map(|row| ArrangementRepresentationLineageRow {
                        measure_ordinal: stats.measures_seen,
                        pre_ledger_raw_note_index: row.pre_raw_idx,
                        post_ledger_raw_note_index: row.post_raw_idx,
                        fragment_ordinal: row.fragment_ordinal,
                    }),
            );
            stats
                .representation_post_raw_note_counts
                .push(ledger_stats.post_raw_note_count);
            stats.visible_notes_normalized += ledger_stats.notes_normalized;
            stats.visible_rhythm_rests_inserted += ledger_stats.rests_inserted;
            stats.visible_rhythm_values_split += ledger_stats.values_split;
        stats.chord_cohorts_source_bound_split += ledger_stats.chord_cohorts_source_bound_split;
            stats.visible_rhythm_measures_completed += ledger_stats.measures_completed;
            if ledger_stats.rests_inserted > 0 || ledger_stats.values_split > 0 || ledger_stats.notes_normalized > 0 {
                stats.measures_simplified += 1;
            }
            out.push_str(&rebuilt);
            last = span.end;
        }
        out.push_str(&xml[last..]);
        stats.physical_births.extend(births.seal(xml, &out).expect("Actual normalizer births must bind their execution"));
        let visible_rhythm_block_identity = BlockTransformationReceipt::mint_block_receipt(
            VisibleRhythmBlockMint { _sealed: () },
            xml,
            &out,
            None,
        );
        if let Some(receipt) = &visible_rhythm_block_identity {
            for capture in &mut stats.physical_births {
                if capture.bind_original_visible_execution(&VisibleRhythmBlockMint { _sealed: () }, receipt).is_err() {
                    stats.physical_birth_capture_unknown = stats.physical_birth_capture_unknown.saturating_add(1);
                }
            }
        }
        ArrangementResult {
            xml: out,
            stats,
            arrangement_source_genesis: None,
            visible_rhythm_block_identity,
        }
    }
    fn find_measure_spans(xml: &str) -> Vec<MeasureSpan> {
        let mut spans = Vec::new();
        let mut search_from = 0_usize;
        while let Some(rel_start) = xml[search_from..].find("<measure") {
            let start = search_from + rel_start;
            let Some(open_end_rel) = xml[start..].find('>') else {
                break;
            };
            let content_start = start + open_end_rel + 1;
            let Some(close_rel) = xml[content_start..].find("</measure>") else {
                break;
            };
            let end = content_start + close_rel + "</measure>".len();
            spans.push(MeasureSpan { start, end });
            search_from = end;
        }
        spans
    }
    fn arrange_measure(text: &str, profile: ArrangementProfile, rhythm_context: RhythmContext, measure_number: usize, mut course_measure: Option<&mut CourseMeasure>) -> (String, ArrangementStats) {
        let raw_notes = collect_raw_notes(text);
        if raw_notes.is_empty() {
            return (text.to_string(), ArrangementStats::default());
        }
        let metas = collect_note_metas(text, &raw_notes);
        let (hidden, support_stats) = select_hidden_notes_for_pianist_profile(&metas, profile);
        let mut stats = ArrangementStats {
            ..ArrangementStats::default()
        };
        if profile.preserve_reference {
            stats.bass_anchors_restored = count_visible_bass_anchors(&metas, &hidden);
            stats.accompaniment_notes_restored = count_visible_left_accompaniment_notes(&metas, &hidden);
            stats.left_sparse_measures = if support_stats.sparse_measure { 1 } else { 0 };
            stats.left_gap_notes_restored = support_stats.notes_restored;
            stats.section_rule_measures = if support_stats.section_rule_measure { 1 } else { 0 };
            stats.section_rule_notes_restored = support_stats.section_notes_restored;
        }
        let human_potential_target_matches = profile
            .human_potential_structural_target
            .is_none_or(|target| target.measure_ordinal == measure_number);
        let human_potential = if profile.preserve_reference
            && profile.human_potential_enabled
            && human_potential_target_matches
        {
            select_human_potential_left_hand_moves_v2(
                &metas,
                &hidden,
                profile.human_potential_candidate_budget,
                profile.human_potential_max_moves,
                profile
                    .human_potential_structural_target
                    .map_or(0, |target| target.candidate_rank),
                measure_number,
            )
        } else {
            HumanPotentialSelection::default()
        };
        let human_potential_moves = human_potential.moves.clone();
        let human_potential_active = !human_potential_moves.is_empty();
        stats.human_potential_budget = human_potential.budget;
        stats.human_potential_candidates_generated = human_potential.candidates_generated;
        stats.human_potential_candidates_checked = human_potential.candidates_checked;
        stats.human_potential_candidates_rejected = human_potential.candidates_rejected;
        stats.human_potential_eligible_measures = human_potential.eligible_measures;
        stats.human_potential_local_safe_candidates = human_potential.local_safe_candidates;
        stats.human_potential_no_pitch_band = human_potential.no_pitch_band;
        stats.human_potential_no_candidates = human_potential.no_candidates;
        stats.human_potential_reject_duplicate_onset = human_potential.reject_duplicate_onset;
        stats.human_potential_reject_right_substance = human_potential.reject_right_substance;
        stats.human_potential_reject_range = human_potential.reject_range;
        stats.human_potential_reject_jump = human_potential.reject_jump;
        stats.human_potential_global_fallback_rank = human_potential.selected_rank;
        stats.human_potential_structural_targets = human_potential.structural_targets.clone();
        stats.human_potential_candidate_details = human_potential.candidate_details.clone();
        if human_potential_active {
            stats.human_potential_measures = 1;
            stats.human_potential_notes_moved = human_potential_moves.len();
        }
        let mut rebuilt = String::with_capacity(text.len() + raw_notes.len() * 80);
        let mut last = 0_usize;
        for (idx, note) in raw_notes.iter().enumerate() {
            rebuilt.push_str(&text[last..note.start]);
            let meta = metas.iter().find(|m| m.raw_idx == idx);
            let (arranged, changed) = arrange_note(&note.text);
            // v0.1.111: HumanPotential arbeitet als echtes RestaffExistingOnly/CrossStaff-Voice. Die vorhandene Note
            // wird an ihrer ursprünglichen Cursor-Position auf das untere System
            // notiert; sie wird nicht zusätzlich als zweite Stimme eingefügt.
            // Deshalb dürfen die vorhandenen LH-Pausen hier nicht pauschal als
            // unsichtbare Timeline-Ereignisse versteckt werden: genau das erzeugte
            // in v0.1.109 MEASURE_CURSOR_OVERFULL/RHYTHM_LEDGER-Blocker.
            let replaces_left_rest = false;
            let moved_by_human_potential = human_potential_moves.contains(&idx);
            let arranged = if moved_by_human_potential {
                // v0.1.111: RestaffExistingOnly-Fix. Der vorhandene
                // Stützton bleibt exakt an derselben MusicXML-Timeline-Position
                // und wird nur auf das untere System geschrieben. Keine Duplikatnote,
                // kein zusätzlicher backup/voice-Block, keine zusätzliche Dauer.
                move_note_to_left_system_for_human_potential(&arranged)
            } else if replaces_left_rest {
                // Die ursprünglichen Pausen der leeren linken Hand werden durch die
                // neue HumanPotential-Stimme ersetzt. So erscheinen nicht zugleich
                // Volltaktpausen und linke Stütztöne.
                mark_note_hidden(&arranged)
            } else if hidden.contains(&idx) {
                match meta {
                    Some(meta) if meta.is_rest => arranged,
                    Some(meta) if meta.is_chord => mark_note_hidden(&arranged),
                    Some(_) => restify_note_for_rhythm_ledger(&arranged),
                    None => mark_note_hidden(&arranged),
                }
            } else {
                // v0.1.28: Die alte Import-Stimmenachse bleibt formal erhalten.
                // v0.1.65: Wenn Material ausgedünnt wird, darf die Zeit nicht aus
                // dem lesbaren Notenbild verschwinden. Hauptnoten werden deshalb
                // als sichtbare Pausen weitergeführt; nur Akkord-Innentöne dürfen
                // weiterhin unsichtbar bleiben.
                arranged
            };
            stats.notes_seen += 1;
            if changed.staff_added { stats.staff_added += 1; }
            if changed.voice_added { stats.voice_added += 1; }
            if changed.stem_added { stats.stems_added += 1; }
            if replaces_left_rest {
                stats.rests_hidden += 1;
            }
            if hidden.contains(&idx)
                && let Some(meta) = meta {
                    if meta.is_rest {
                        stats.rests_hidden += 1;
                    } else {
                        stats.notes_hidden += 1;
                        if meta.is_chord {
                            stats.chord_tones_hidden += 1;
                        } else {
                            stats.notes_restified_as_rests += 1;
                        }
                    }
                }
            if let Some(meta) = meta {
                let disposition = if meta.is_rest {
                    ArrangementMaterialDispositionKind::RestOrOther
                } else if moved_by_human_potential {
                    ArrangementMaterialDispositionKind::VisiblePitched
                } else if hidden.contains(&idx) && meta.is_chord {
                    ArrangementMaterialDispositionKind::HiddenPitched
                } else if hidden.contains(&idx) {
                    ArrangementMaterialDispositionKind::RestifiedPitched
                } else {
                    ArrangementMaterialDispositionKind::VisiblePitched
                };
                let projected_staff = if moved_by_human_potential {
                    2
                } else {
                    match meta.staff {
                        PianoStaff::Right => 1,
                        PianoStaff::Left => 2,
                    }
                };
                if let Some(courses) = course_measure.as_deref_mut() {
                    courses.disposition(idx, &arranged, disposition, projected_staff, moved_by_human_potential)
                        .expect("Source-bound arrangement disposition");
                }
                stats
                    .material_disposition_rows
                    .push(ArrangementMaterialDispositionRow {
                        measure_ordinal: measure_number,
                        raw_note_index: idx,
                        disposition,
                        projected_staff,
                        human_potential_moved: moved_by_human_potential,
                    });
            }
            rebuilt.push_str(&arranged);
            last = note.end;
        }
        rebuilt.push_str(&text[last..]);
        let (ledger_rebuilt, ledger_stats) = apply_visible_rhythm_ledger(&rebuilt, rhythm_context, course_measure, false);
        rebuilt = ledger_rebuilt;
        stats.representation_lineage_rows.extend(
            ledger_stats
                .fragment_lineage_rows
                .iter()
                .map(|row| ArrangementRepresentationLineageRow {
                    measure_ordinal: measure_number,
                    pre_ledger_raw_note_index: row.pre_raw_idx,
                    post_ledger_raw_note_index: row.post_raw_idx,
                    fragment_ordinal: row.fragment_ordinal,
                }),
        );
        stats
            .representation_post_raw_note_counts
            .push(ledger_stats.post_raw_note_count);
        stats.visible_notes_normalized += ledger_stats.notes_normalized;
        stats.visible_rhythm_rests_inserted += ledger_stats.rests_inserted;
        stats.visible_rhythm_values_split += ledger_stats.values_split;
        stats.chord_cohorts_source_bound_split += ledger_stats.chord_cohorts_source_bound_split;
        stats.visible_rhythm_measures_completed += ledger_stats.measures_completed;
        if stats.notes_hidden > 0
            || stats.rests_hidden > 0
            || stats.chord_tones_hidden > 0
            || ledger_stats.rests_inserted > 0
            || ledger_stats.values_split > 0
        {
            stats.measures_simplified = 1;
        }
        (rebuilt, stats)
    }
    fn collect_note_metas(measure_text: &str, raw_notes: &[RawNote]) -> Vec<NoteMeta> {
        let mut infos = Vec::new();
        let mut cursor = 0_i64;
        let mut last_non_chord_start = 0_i64;
        let mut note_pos = 0_usize;
        let mut scan = 0_usize;
        loop {
            let next_note = measure_text[scan..].find("<note").map(|p| scan + p);
            let next_backup = measure_text[scan..].find("<backup").map(|p| scan + p);
            let next_forward = measure_text[scan..].find("<forward").map(|p| scan + p);
            let mut candidates = Vec::new();
            if let Some(p) = next_note { candidates.push((p, "note")); }
            if let Some(p) = next_backup { candidates.push((p, "backup")); }
            if let Some(p) = next_forward { candidates.push((p, "forward")); }
            candidates.sort_by_key(|(p, _)| *p);
            let Some((pos, kind)) = candidates.first().copied() else { break; };
            match kind {
                "backup" => {
                    if let Some(end) = measure_text[pos..].find("</backup>").map(|e| pos + e + "</backup>".len()) {
                        let segment = &measure_text[pos..end];
                        cursor -= extract_first_i64(segment, "duration").unwrap_or(0).max(0);
                        scan = end;
                    } else {
                        break;
                    }
                }
                "forward" => {
                    if let Some(end) = measure_text[pos..].find("</forward>").map(|e| pos + e + "</forward>".len()) {
                        let segment = &measure_text[pos..end];
                        cursor += extract_first_i64(segment, "duration").unwrap_or(0).max(0);
                        scan = end;
                    } else {
                        break;
                    }
                }
                "note" => {
                    let Some(raw) = raw_notes.get(note_pos) else { break; };
                    let raw_idx = note_pos;
                    note_pos += 1;
                    let duration = extract_first_i64(&raw.text, "duration").unwrap_or(0).max(0);
                    let is_chord = raw.text.contains("<chord") || raw.text.contains("<chord/");
                    let is_rest = raw.text.contains("<rest") || raw.text.contains("<rest/");
                    let start = if is_chord { last_non_chord_start } else { cursor };
                    let staff = extract_first_text(&raw.text, "staff")
                        .and_then(|s| match s.trim() {
                            "1" => Some(PianoStaff::Right),
                            "2" => Some(PianoStaff::Left),
                            _ => None,
                        })
                        .or_else(|| staff_from_pitch(&raw.text))
                        .unwrap_or(PianoStaff::Right);
                    let voice = extract_first_text(&raw.text, "voice").unwrap_or_else(|| staff.default_voice().to_string());
                    let midi = midi_from_note_text(&raw.text);
                    infos.push(NoteMeta {
                        raw_idx,
                        start,
                        duration,
                        staff,
                        voice,
                        is_rest,
                        is_chord,
                        midi,
                    });
                    if !is_chord {
                        last_non_chord_start = start;
                        cursor += duration;
                    }
                    scan = raw.end;
                }
                _ => break,
            }
        }
        infos
    }
    fn select_hidden_notes_for_pianist_profile(metas: &[NoteMeta], profile: ArrangementProfile) -> (BTreeSet<usize>, LeftHandSupportStats) {
        let mut hidden = BTreeSet::new();
        let mut support_stats = LeftHandSupportStats::default();
        // v0.1.30: pianistische Rekonstruktion mit zwei moeglichen Stimmen pro Hand.
        // Ziel: weg vom DAW-Import, aber nicht mehr auf eine duenne Einlinienfassung
        // zusammenschneiden. Jede Hand darf eine Haupt- und eine Nebenstimme behalten.
        hide_dense_chord_tones(metas, &mut hidden, profile);
        hide_parallel_import_notes(metas, &mut hidden, profile);
        hide_secondary_voice_notes(metas, &mut hidden, profile);
        hide_import_artifact_rests();
        let (right_limit, left_limit) = if profile.preserve_reference {
            // WAV bleibt Primärreferenz. Ab v0.1.29 darf jede Hand bis zu zwei
            // lesbare Stimmen tragen; dadurch soll der Notenerhalt wieder Richtung
            // 65-70% steigen, ohne die alte Import-Voice-Unordnung freizugeben.
            (16, 12)
        } else {
            (12, 8)
        };
        hide_excess_measure_density(metas, &mut hidden, PianoStaff::Right, right_limit);
        hide_excess_measure_density(metas, &mut hidden, PianoStaff::Left, left_limit);
        if profile.preserve_reference {
            let before = hidden.len();
            restore_left_hand_bass_continuity(metas, &mut hidden);
            support_stats.notes_restored += before.saturating_sub(hidden.len());
            support_stats.notes_restored += restore_left_hand_sparse_sections(metas, &mut hidden);
            let before_section = hidden.len();
            let material_group = material_group_for_measure(metas, &hidden);
            support_stats.section_notes_restored += restore_section_group_material(metas, &mut hidden, material_group);
            support_stats.section_rule_measure = material_group.is_some() && hidden.len() < before_section;
            support_stats.sparse_measure = is_left_hand_sparse_after_support(metas, &hidden);
        }
        enforce_chord_anchor_visibility(metas, &mut hidden);
        (hidden, support_stats)
    }
    #[derive(Debug, Clone, Default)]
    struct HumanPotentialSelection {
        moves: BTreeSet<usize>,
        budget: usize,
        candidates_generated: usize,
        candidates_checked: usize,
        candidates_rejected: usize,
        eligible_measures: usize,
        local_safe_candidates: usize,
        no_pitch_band: usize,
        no_candidates: usize,
        reject_duplicate_onset: usize,
        reject_right_substance: usize,
        reject_range: usize,
        reject_jump: usize,
        selected_rank: usize,
        structural_targets: Vec<HumanPotentialStructuralTarget>,
        candidate_details: Vec<String>,
    }
    const HUMAN_POTENTIAL_DEFAULT_CANDIDATE_BUDGET: usize = 96;
    const HUMAN_POTENTIAL_DEFAULT_MAX_MOVES_PER_MEASURE: usize = 4;
    const HUMAN_POTENTIAL_MAX_CANDIDATES_PER_MEASURE: usize = 16;
    fn select_human_potential_left_hand_moves_v2(
        metas: &[NoteMeta],
        hidden: &BTreeSet<usize>,
        requested_budget: usize,
        requested_max_moves: usize,
        requested_rank: usize,
        measure_number: usize,
    ) -> HumanPotentialSelection {
        // v0.1.107: HumanPotentialPass v6 GlobalFallback. Die Diagnose v0.1.106
        // zeigte: viele Kandidaten sind lokal sicher, aber der eine Bestkandidat
        // kann global blockieren. Deshalb kann der Caller jetzt einen Rang anfordern,
        // um lokal sichere Fallback-Kandidaten durch den GlobalGuard zu schicken.
        // v0.1.106: HumanPotentialPass v5 Diagnose. Der Fehler in v0.1.103 war, dass
        // Kandidaten praktisch nur bei simultanen Innenstimmen gefunden wurden.
        // Viele reale rechte-Hand-Läufe sind aber einstimmig/arpeggiert: dort ist
        // ein tieferer Stützton trotzdem vorhandenes Material und darf bei leerer
        // linker Hand in das untere System gelegt werden. Keine neuen Noten, keine
        // festen Takte, keine Etüde-6-Sonderlogik.
        let budget = if requested_budget == 0 {
            HUMAN_POTENTIAL_DEFAULT_CANDIDATE_BUDGET
        } else {
            requested_budget.max(1)
        };
        let max_moves = if requested_max_moves == 0 {
            HUMAN_POTENTIAL_DEFAULT_MAX_MOVES_PER_MEASURE
        } else {
            requested_max_moves.clamp(1, 6)
        };
        let visible_right_notes = metas
            .iter()
            .filter(|m| m.staff == PianoStaff::Right && !m.is_rest && m.midi.is_some() && !hidden.contains(&m.raw_idx))
            .collect::<Vec<_>>();
        let visible_left_notes = metas
            .iter()
            .filter(|m| m.staff == PianoStaff::Left && !m.is_rest && m.midi.is_some() && !hidden.contains(&m.raw_idx))
            .collect::<Vec<_>>();
        // v0.1.106: Zunächst nur wirklich leere linke Hände automatisch füllen.
        // "Fast leer" bleibt diagnostisch interessant, ist aber wegen vorhandener
        // linker Stimmen/Pausen deutlich kollisionsanfälliger.
        if visible_right_notes.len() < 8 || !visible_left_notes.is_empty() {
            return HumanPotentialSelection { budget, ..HumanPotentialSelection::default() };
        }
        let right_midis = visible_right_notes.iter().filter_map(|m| m.midi).collect::<Vec<_>>();
        let Some(pitch_band) = human_potential_pitch_band(&right_midis) else {
            return HumanPotentialSelection {
                budget,
                eligible_measures: 1,
                no_pitch_band: 1,
                ..HumanPotentialSelection::default()
            };
        };
        let occupied_left_starts = visible_left_notes
            .iter()
            .map(|m| m.start)
            .collect::<BTreeSet<_>>();
        let mut max_right_by_start: BTreeMap<i64, i32> = BTreeMap::new();
        for meta in &visible_right_notes {
            let midi = meta.midi.unwrap_or(0);
            max_right_by_start
                .entry(meta.start)
                .and_modify(|v| *v = (*v).max(midi))
                .or_insert(midi);
        }
        let mut candidates = visible_right_notes
            .iter()
            .copied()
            .filter(|m| human_potential_note_is_candidate(m, &occupied_left_starts, &max_right_by_start, pitch_band))
            .collect::<Vec<_>>();
        candidates.sort_by_key(|m| {
            (
                std::cmp::Reverse(human_potential_note_score(m, pitch_band)),
                m.start,
                m.raw_idx,
            )
        });
        candidates.truncate(HUMAN_POTENTIAL_MAX_CANDIDATES_PER_MEASURE);
        let generated = candidates.len();
        if generated < 1 {
            return HumanPotentialSelection {
                budget,
                candidates_generated: generated,
                candidates_checked: generated,
                candidates_rejected: generated,
                eligible_measures: 1,
                no_candidates: 1,
                ..HumanPotentialSelection::default()
            };
        }
        let mut checked = 0usize;
        let mut rejected = 0usize;
        let mut local_safe_candidates = 0usize;
        let mut reject_duplicate_onset = 0usize;
        let mut reject_right_substance = 0usize;
        let mut reject_range = 0usize;
        let mut reject_jump = 0usize;
        let mut best: Vec<&NoteMeta> = Vec::new();
        let mut best_score = i64::MIN;
        let mut safe_sets: Vec<(i64, Vec<usize>)> = Vec::new();
        let max_size = max_moves
            .min(candidates.len())
            .min((visible_right_notes.len() / 3).max(1));
        // Erst Einzelstützen zulassen. v0.1.103 verlangte mindestens 2 Kandidaten;
        // dadurch blieben viele sichere, musikalisch sinnvolle Einzelanker liegen.
        for size in 1..=max_size {
            let mut current = Vec::new();
            evaluate_human_potential_combinations(
                &candidates,
                size,
                0,
                budget,
                &mut current,
                &mut checked,
                &mut rejected,
                &mut best,
                &mut best_score,
                &mut safe_sets,
                &mut local_safe_candidates,
                &mut reject_duplicate_onset,
                &mut reject_right_substance,
                &mut reject_range,
                &mut reject_jump,
                pitch_band,
                visible_right_notes.len(),
            );
            if checked >= budget {
                break;
            }
        }
        safe_sets.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        safe_sets.dedup_by(|a, b| a.1.as_slice() == b.1.as_slice());
        let structural_targets = safe_sets
            .iter()
            .take(8)
            .enumerate()
            .map(|(rank, (score, _))| HumanPotentialStructuralTarget {
                measure_ordinal: measure_number,
                candidate_rank: rank,
                score: *score,
            })
            .collect::<Vec<_>>();
        let candidate_details = safe_sets
            .iter()
            .take(8)
            .enumerate()
            .map(|(rank, (score, moves))| human_potential_candidate_detail(measure_number, rank, *score, moves, metas, pitch_band, visible_right_notes.len()))
            .collect::<Vec<_>>();
        let selected_rank = requested_rank.min(safe_sets.len().saturating_sub(1));
        let selected_moves = if let Some((_, moves)) = safe_sets.get(selected_rank) {
            moves.iter().copied().collect()
        } else {
            best.into_iter().map(|m| m.raw_idx).collect()
        };
        HumanPotentialSelection {
            moves: selected_moves,
            budget,
            candidates_generated: generated,
            candidates_checked: checked,
            candidates_rejected: rejected,
            eligible_measures: 1,
            local_safe_candidates,
            reject_duplicate_onset,
            reject_right_substance,
            reject_range,
            reject_jump,
            selected_rank,
            structural_targets,
            candidate_details,
            ..HumanPotentialSelection::default()
        }
    }
    fn human_potential_candidate_detail(
        measure_number: usize,
        rank: usize,
        score: i64,
        moves: &[usize],
        metas: &[NoteMeta],
        band: HumanPotentialPitchBand,
        visible_right_count: usize,
    ) -> String {
        let mut selected = moves
            .iter()
            .filter_map(|idx| metas.iter().find(|m| m.raw_idx == *idx))
            .collect::<Vec<_>>();
        selected.sort_by_key(|m| (m.start, m.raw_idx));
        let mut max_jump = 0;
        for pair in selected.windows(2) {
            let jump = (pair[0].midi.unwrap_or(0) - pair[1].midi.unwrap_or(0)).abs();
            max_jump = max_jump.max(jump);
        }
        let span = selected
            .iter()
            .filter_map(|m| m.midi)
            .fold(None::<(i32, i32)>, |acc, midi| match acc {
                Some((lo, hi)) => Some((lo.min(midi), hi.max(midi))),
                None => Some((midi, midi)),
            })
            .map_or(0, |(lo, hi)| hi - lo);
        let notes = selected
            .iter()
            .map(|m| format!(
                "idx{} midi{} start{} dur{} voice{}",
                m.raw_idx,
                m.midi.unwrap_or(-1),
                m.start,
                m.duration,
                m.voice
            ))
            .collect::<Vec<_>>()
            .join(" | " );
        format!(
            "target_measure={} rank={} score={} moves={} rh={} band_med={} cutoff={} span={} max_jump={} mutation_mode=restaff_existing_note_only inserted_notes=0 inserted_backup=0 inserted_forward=0 [{}]",
            measure_number,
            rank,
            score,
            selected.len(),
            visible_right_count,
            band.median,
            band.lower_cutoff,
            span,
            max_jump,
            notes
        )
    }
    #[derive(Debug, Clone, Copy)]
    struct HumanPotentialPitchBand {
        median: i32,
        max: i32,
        lower_cutoff: i32,
    }
    fn human_potential_pitch_band(midis: &[i32]) -> Option<HumanPotentialPitchBand> {
        if midis.is_empty() {
            return None;
        }
        let mut sorted = midis.to_vec();
        sorted.sort();
        let min = *sorted.first()?;
        let max = *sorted.last()?;
        let median = sorted[sorted.len() / 2];
        if max - min < 7 {
            return None;
        }
        // Untere Linie/Innenstütze, aber nicht nur absolut tiefe Töne. Dadurch
        // funktionieren auch hohe virtuose Stellen, ohne die Oberstimme anzufassen.
        let lower_cutoff = (median + 1).min(max - 5).max(min + 2);
        Some(HumanPotentialPitchBand { median, max, lower_cutoff })
    }
    fn human_potential_note_is_candidate(
        meta: &NoteMeta,
        occupied_left_starts: &BTreeSet<i64>,
        max_right_by_start: &BTreeMap<i64, i32>,
        band: HumanPotentialPitchBand,
    ) -> bool {
        let Some(midi) = meta.midi else { return false; };
        if meta.is_chord || occupied_left_starts.contains(&meta.start) {
            return false;
        }
        // Natürlicher Stützbereich: In normaler Lage A3..A4. In sehr hohen
        // virtuosen Läufen darf die linke Hand als Kreuz-/Stützhand auch relative
        // Untertöne übernehmen; die generische LowerStaffClefReadabilityPolicy setzt
        // dafür später den nötigen temporären Violinschlüssel im unteren System.
        let upper_support_limit = if band.max >= 84 { 88 } else { 69 };
        if midi < 45 || midi > upper_support_limit {
            return false;
        }
        if midi > band.lower_cutoff {
            return false;
        }
        if band.max - midi < 5 {
            return false;
        }
        // Simultane Oberstimme bleibt unangetastet. Bei einstimmigen Läufen ist der
        // gleiche Onset naturgemäß nicht beweiskräftig; dann entscheidet die Linie.
        if let Some(top_same_start) = max_right_by_start.get(&meta.start)
            && *top_same_start > midi && *top_same_start - midi < 3 {
                return false;
            }
        meta.start.rem_euclid(192) == 0
            || meta.start.rem_euclid(96) == 0
            || meta.start.rem_euclid(48) == 0
            || meta.start.rem_euclid(24) == 0
            || meta.duration >= 48
    }
    #[allow(clippy::too_many_arguments)]
    fn evaluate_human_potential_combinations<'a>(
        candidates: &[&'a NoteMeta],
        target_size: usize,
        start_idx: usize,
        budget: usize,
        current: &mut Vec<&'a NoteMeta>,
        checked: &mut usize,
        rejected: &mut usize,
        best: &mut Vec<&'a NoteMeta>,
        best_score: &mut i64,
        safe_sets: &mut Vec<(i64, Vec<usize>)>,
        local_safe_candidates: &mut usize,
        reject_duplicate_onset: &mut usize,
        reject_right_substance: &mut usize,
        reject_range: &mut usize,
        reject_jump: &mut usize,
        band: HumanPotentialPitchBand,
        visible_right_count: usize,
    ) {
        if *checked >= budget {
            return;
        }
        if current.len() == target_size {
            *checked += 1;
            match human_potential_selection_safety_issue(current, visible_right_count) {
                None => {
                    *local_safe_candidates += 1;
                    let score = human_potential_selection_score(current, band);
                    safe_sets.push((score, current.iter().map(|m| m.raw_idx).collect()));
                    if score > *best_score {
                        *best_score = score;
                        *best = current.clone();
                    }
                }
                Some(HumanPotentialSafetyIssue::DuplicateOnset) => {
                    *rejected += 1;
                    *reject_duplicate_onset += 1;
                }
                Some(HumanPotentialSafetyIssue::RightHandSubstance) => {
                    *rejected += 1;
                    *reject_right_substance += 1;
                }
                Some(HumanPotentialSafetyIssue::Range) => {
                    *rejected += 1;
                    *reject_range += 1;
                }
                Some(HumanPotentialSafetyIssue::Jump) => {
                    *rejected += 1;
                    *reject_jump += 1;
                }
            }
            return;
        }
        let remaining_needed = target_size.saturating_sub(current.len());
        if candidates.len().saturating_sub(start_idx) < remaining_needed {
            return;
        }
        for idx in start_idx..candidates.len() {
            if *checked >= budget {
                return;
            }
            current.push(candidates[idx]);
            evaluate_human_potential_combinations(
                candidates,
                target_size,
                idx + 1,
                budget,
                current,
                checked,
                rejected,
                best,
                best_score,
                safe_sets,
                local_safe_candidates,
                reject_duplicate_onset,
                reject_right_substance,
                reject_range,
                reject_jump,
                band,
                visible_right_count,
            );
            current.pop();
        }
    }
    fn human_potential_note_score(meta: &NoteMeta, band: HumanPotentialPitchBand) -> i64 {
        let midi = meta.midi.unwrap_or(band.median);
        let beat_score = if meta.start.rem_euclid(192) == 0 {
            90
        } else if meta.start.rem_euclid(96) == 0 {
            60
        } else if meta.start.rem_euclid(48) == 0 {
            35
        } else if meta.start.rem_euclid(24) == 0 {
            18
        } else {
            10
        };
        let duration_score = meta.duration.clamp(0, 192) / 3;
        let support_score = 90 - ((midi - band.lower_cutoff).abs() as i64 * 5);
        let bassiness_score = (band.max - midi).max(0) as i64 * 3;
        beat_score + duration_score + support_score + bassiness_score
    }
    fn human_potential_selection_score(selected: &[&NoteMeta], band: HumanPotentialPitchBand) -> i64 {
        selected.iter().map(|m| human_potential_note_score(m, band)).sum::<i64>()
            + selected.len() as i64 * 35
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum HumanPotentialSafetyIssue {
        DuplicateOnset,
        RightHandSubstance,
        Range,
        Jump,
    }
    fn human_potential_selection_safety_issue(selected: &[&NoteMeta], visible_right_count: usize) -> Option<HumanPotentialSafetyIssue> {
        let mut by_time = selected.to_vec();
        by_time.sort_by_key(|m| (m.start, m.raw_idx));
        let mut starts = BTreeSet::new();
        if by_time.iter().any(|m| !starts.insert(m.start)) {
            return Some(HumanPotentialSafetyIssue::DuplicateOnset);
        }
        // Rechte Hand muss nach der Abgabe weiterhin substantiell bleiben.
        if selected.len().saturating_mul(2) >= visible_right_count {
            return Some(HumanPotentialSafetyIssue::RightHandSubstance);
        }
        let min_midi = by_time.iter().filter_map(|m| m.midi).min().unwrap_or(0);
        let max_midi = by_time.iter().filter_map(|m| m.midi).max().unwrap_or(0);
        if max_midi - min_midi > 16 {
            return Some(HumanPotentialSafetyIssue::Range);
        }
        for pair in by_time.windows(2) {
            let a = pair[0];
            let b = pair[1];
            let jump = (a.midi.unwrap_or(0) - b.midi.unwrap_or(0)).abs();
            let dt = (b.start - a.start).abs();
            if dt <= 48 && jump > 9 {
                return Some(HumanPotentialSafetyIssue::Jump);
            }
            if dt <= 96 && jump > 14 {
                return Some(HumanPotentialSafetyIssue::Jump);
            }
            if dt <= 192 && jump > 18 {
                return Some(HumanPotentialSafetyIssue::Jump);
            }
        }
        None
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum SectionGroup {
        DenseRightHand,
        LowBassAnchors,
        CadentialSupport,
    }
    fn material_group_for_measure(metas: &[NoteMeta], hidden: &BTreeSet<usize>) -> Option<SectionGroup> {
        // Keine fest verdrahteten Taktbereiche: Die zusätzliche Materialstütze wird
        // ausschließlich aus der tatsächlich vorhandenen Dichte, linken-Hand-Lücke,
        // Basslage und metrischen Ankerpositionen des aktuellen Takts abgeleitet.
        let visible_right = visible_note_count(metas, hidden, PianoStaff::Right);
        let visible_left = visible_note_count(metas, hidden, PianoStaff::Left);
        let hidden_left = hidden_note_count(metas, hidden, PianoStaff::Left);
        if hidden_left == 0 {
            return None;
        }
        let low_hidden_left = metas
            .iter()
            .filter(|m| {
                m.staff == PianoStaff::Left
                    && hidden.contains(&m.raw_idx)
                    && !m.is_rest
                    && m.midi.is_some_and(|midi| midi < 52)
            })
            .count();
        let beat_anchor_hidden_left = metas
            .iter()
            .filter(|m| {
                m.staff == PianoStaff::Left
                    && hidden.contains(&m.raw_idx)
                    && !m.is_rest
                    && m.midi.is_some()
                    && (m.start.rem_euclid(192) == 0 || m.start.rem_euclid(96) == 0)
            })
            .count();
        let sustained_hidden_left = metas
            .iter()
            .filter(|m| {
                m.staff == PianoStaff::Left
                    && hidden.contains(&m.raw_idx)
                    && !m.is_rest
                    && m.midi.is_some()
                    && m.duration >= 96
            })
            .count();
        if low_hidden_left >= 3 && visible_left < 12 {
            Some(SectionGroup::LowBassAnchors)
        } else if visible_right >= 16 && visible_left < 10 && beat_anchor_hidden_left >= 2 {
            Some(SectionGroup::DenseRightHand)
        } else if visible_right >= 6 && visible_left < 10 && sustained_hidden_left >= 2 {
            Some(SectionGroup::CadentialSupport)
        } else {
            None
        }
    }
    fn visible_note_count(metas: &[NoteMeta], hidden: &BTreeSet<usize>, staff: PianoStaff) -> usize {
        metas
            .iter()
            .filter(|m| m.staff == staff && !m.is_rest && m.midi.is_some() && !hidden.contains(&m.raw_idx))
            .count()
    }
    fn hidden_note_count(metas: &[NoteMeta], hidden: &BTreeSet<usize>, staff: PianoStaff) -> usize {
        metas
            .iter()
            .filter(|m| m.staff == staff && !m.is_rest && m.midi.is_some() && hidden.contains(&m.raw_idx))
            .count()
    }
    fn restore_section_group_material(
        metas: &[NoteMeta],
        hidden: &mut BTreeSet<usize>,
        section_group: Option<SectionGroup>,
    ) -> usize {
        let Some(group) = section_group else {
            return 0;
        };
        // Materialregeln duerfen vorhandene Bass-/Begleitnoten pro automatisch
        // erkannter musikalischer Textur deutlicher zurueckholen, bleiben aber
        // gedeckelt: keine neuen Noten,
        // keine Voice-/Staff-Umschreibung und keine globale Notenflut.
        let visible_right = metas
            .iter()
            .filter(|m| m.staff == PianoStaff::Right && !m.is_rest && m.midi.is_some() && !hidden.contains(&m.raw_idx))
            .count();
        let visible_left = metas
            .iter()
            .filter(|m| m.staff == PianoStaff::Left && !m.is_rest && m.midi.is_some() && !hidden.contains(&m.raw_idx))
            .count();
        let target_left: usize = match group {
            SectionGroup::DenseRightHand => {
                if visible_right < 10 { return 0; }
                10
            }
            SectionGroup::LowBassAnchors => 12,
            SectionGroup::CadentialSupport => {
                if visible_right < 6 { return 0; }
                10
            }
        };
        let per_measure_cap: usize = match group {
            SectionGroup::DenseRightHand => 2,
            SectionGroup::LowBassAnchors => 3,
            SectionGroup::CadentialSupport => 2,
        };
        let needed = target_left.saturating_sub(visible_left).min(per_measure_cap);
        if needed == 0 {
            return 0;
        }
        let mut candidates = metas
            .iter()
            .filter(|m| {
                m.staff == PianoStaff::Left
                    && !m.is_rest
                    && m.midi.is_some()
                    && hidden.contains(&m.raw_idx)
            })
            .collect::<Vec<_>>();
        candidates.sort_by_key(|m| {
            (
                section_candidate_priority(m, group),
                if m.start.rem_euclid(192) == 0 { 0 } else if m.start.rem_euclid(96) == 0 { 1 } else { 2 },
                std::cmp::Reverse(m.duration),
                m.midi.unwrap_or(127),
                if m.is_chord { 1 } else { 0 },
                m.start,
            )
        });
        let mut restored = 0usize;
        for meta in candidates.into_iter().take(needed) {
            if hidden.remove(&meta.raw_idx) {
                restored += 1;
            }
        }
        restored
    }
    fn section_candidate_priority(meta: &NoteMeta, group: SectionGroup) -> i32 {
        match group {
            SectionGroup::DenseRightHand => {
                if meta.start.rem_euclid(96) == 0 { 0 } else { 1 }
            }
            SectionGroup::LowBassAnchors => {
                if meta.midi.unwrap_or(127) < 52 { 0 } else if meta.start.rem_euclid(96) == 0 { 1 } else { 2 }
            }
            SectionGroup::CadentialSupport => {
                if meta.duration >= 96 { 0 } else if meta.start.rem_euclid(96) == 0 { 1 } else { 2 }
            }
        }
    }
    fn restore_left_hand_sparse_sections(metas: &[NoteMeta], hidden: &mut BTreeSet<usize>) -> usize {
        // v0.1.32: Leere-Hand-Erkennung ohne globale Aufblähung.
        // Wenn die rechte Hand dicht bleibt, die linke Hand aber nach der
        // Reduktion zu dünn wirkt, werden nur wenige vorhandene Bass-/Begleitanker
        // zurückgeholt. Es werden keine neuen Noten erfunden und keine Voices
        // umgeschrieben; die stabile Stimmenachse aus v0.1.28 bleibt erhalten.
        let visible_right = metas
            .iter()
            .filter(|m| m.staff == PianoStaff::Right && !m.is_rest && !hidden.contains(&m.raw_idx))
            .count();
        let visible_left = metas
            .iter()
            .filter(|m| m.staff == PianoStaff::Left && !m.is_rest && m.midi.is_some() && !hidden.contains(&m.raw_idx))
            .count();
        if visible_right < 10 || visible_left >= 6 {
            return 0;
        }
        let target_left: usize = if visible_right >= 18 { 7 } else { 6 };
        let needed = target_left.saturating_sub(visible_left);
        if needed == 0 {
            return 0;
        }
        let mut candidates = metas
            .iter()
            .filter(|m| {
                m.staff == PianoStaff::Left
                    && !m.is_rest
                    && m.midi.is_some()
                    && hidden.contains(&m.raw_idx)
            })
            .collect::<Vec<_>>();
        candidates.sort_by_key(|m| {
            (
                // zuerst starke Zählzeiten / Halbtakt-Anker
                if m.start.rem_euclid(192) == 0 { 0 } else if m.start.rem_euclid(96) == 0 { 1 } else { 2 },
                // längere Werte stabilisieren die linke Hand besser als Splitter
                std::cmp::Reverse(m.duration),
                // Bass vor Innenfüllung
                m.midi.unwrap_or(127),
                if m.is_chord { 1 } else { 0 },
                m.start,
            )
        });
        let mut restored = 0usize;
        for meta in candidates.into_iter().take(needed) {
            if hidden.remove(&meta.raw_idx) {
                restored += 1;
            }
        }
        restored
    }
    fn is_left_hand_sparse_after_support(metas: &[NoteMeta], hidden: &BTreeSet<usize>) -> bool {
        let left_source = metas
            .iter()
            .any(|m| m.staff == PianoStaff::Left && !m.is_rest && m.midi.is_some());
        if !left_source {
            return true;
        }
        let visible_left = metas
            .iter()
            .filter(|m| m.staff == PianoStaff::Left && !m.is_rest && m.midi.is_some() && !hidden.contains(&m.raw_idx))
            .count();
        visible_left < 3
    }
    fn restore_left_hand_bass_continuity(metas: &[NoteMeta], hidden: &mut BTreeSet<usize>) {
        // v0.1.30: Die linke Hand darf nicht nur formales Staff-Beigemüse sein.
        // Wenn die Ausdünnung Bassmaterial zu stark versteckt hat, werden pro
        // Halbtakt und bei sehr dünner linker Hand gezielt Bassanker und wenige
        // Begleitnoten zurückgeholt. Das ist kein globales "alles wieder zeigen",
        // sondern ein Schutz gegen leere Bassstrecken.
        let left_notes: Vec<&NoteMeta> = metas
            .iter()
            .filter(|m| m.staff == PianoStaff::Left && !m.is_rest && m.midi.is_some())
            .collect();
        if left_notes.is_empty() {
            return;
        }
        let mut anchors = BTreeSet::new();
        for half in 0..2_i64 {
            let half_start = half * 192;
            let half_end = half_start + 192;
            if let Some(anchor) = left_notes
                .iter()
                .copied()
                .filter(|m| m.start >= half_start && m.start < half_end)
                .min_by_key(|m| {
                    (
                        // starke Zählzeiten zuerst
                        if m.start.rem_euclid(96) == 0 { 0 } else { 1 },
                        // Bassanker: tiefste Note
                        m.midi.unwrap_or(127),
                        // echte Notenanker vor <chord/>-Tönen
                        if m.is_chord { 1 } else { 0 },
                        m.start,
                    )
                })
            {
                anchors.insert(anchor.raw_idx);
            }
        }
        for beat in 0..4_i64 {
            let beat_start = beat * 96;
            if let Some(anchor) = left_notes
                .iter()
                .copied()
                .filter(|m| m.start == beat_start)
                .min_by_key(|m| (m.midi.unwrap_or(127), if m.is_chord { 1 } else { 0 }))
            {
                anchors.insert(anchor.raw_idx);
            }
        }
        for raw_idx in anchors {
            hidden.remove(&raw_idx);
        }
        let visible_left = left_notes
            .iter()
            .filter(|m| !hidden.contains(&m.raw_idx))
            .count();
        if visible_left >= 5 {
            return;
        }
        let mut candidates = left_notes
            .iter()
            .copied()
            .filter(|m| hidden.contains(&m.raw_idx))
            .collect::<Vec<_>>();
        candidates.sort_by_key(|m| {
            (
                // starke Zählzeiten und längere Noten sind eher musikalisch gemeint
                if m.start.rem_euclid(96) == 0 { 0 } else { 1 },
                std::cmp::Reverse(m.duration),
                m.start,
                m.midi.unwrap_or(127),
                if m.is_chord { 1 } else { 0 },
            )
        });
        let needed = 5usize.saturating_sub(visible_left);
        for meta in candidates.into_iter().take(needed) {
            hidden.remove(&meta.raw_idx);
        }
    }
    fn count_visible_bass_anchors(metas: &[NoteMeta], hidden: &BTreeSet<usize>) -> usize {
        let mut count = 0usize;
        for half in 0..2_i64 {
            let half_start = half * 192;
            let half_end = half_start + 192;
            let has_visible_anchor = metas.iter().any(|m| {
                m.staff == PianoStaff::Left
                    && !m.is_rest
                    && m.midi.is_some()
                    && m.start >= half_start
                    && m.start < half_end
                    && !hidden.contains(&m.raw_idx)
            });
            if has_visible_anchor {
                count += 1;
            }
        }
        count
    }
    fn count_visible_left_accompaniment_notes(metas: &[NoteMeta], hidden: &BTreeSet<usize>) -> usize {
        metas.iter()
            .filter(|m| {
                m.staff == PianoStaff::Left
                    && !m.is_rest
                    && m.midi.is_some()
                    && !hidden.contains(&m.raw_idx)
            })
            .count()
    }
    fn hide_dense_chord_tones(metas: &[NoteMeta], hidden: &mut BTreeSet<usize>, profile: ArrangementProfile) {
        let mut by_slot: BTreeMap<(PianoStaff, i64), Vec<&NoteMeta>> = BTreeMap::new();
        for meta in metas.iter().filter(|m| !m.is_rest && m.midi.is_some()) {
            by_slot.entry((meta.staff, meta.start)).or_default().push(meta);
        }
        for ((staff, _start), group) in by_slot {
            let max_chord_tones = match (staff, profile.preserve_reference) {
                (PianoStaff::Right, true) => 3,
                (PianoStaff::Left, true) => 2,
                (PianoStaff::Right, false) => 1,
                (PianoStaff::Left, false) => 1,
            };
            if group.len() <= max_chord_tones {
                continue;
            }
            // MusicXML-Chords benötigen ihren nicht-<chord/>-Anker. Darum wird der
            // Anker nie zugunsten eines späteren Chord-Tons versteckt.
            let mut keep = BTreeSet::new();
            if let Some(anchor) = group.iter().copied().find(|m| !m.is_chord) {
                keep.insert(anchor.raw_idx);
            }
            let mut chord_tones: Vec<&NoteMeta> = group.iter().copied().filter(|m| m.is_chord).collect();
            match staff {
                PianoStaff::Right => chord_tones.sort_by_key(|m| std::cmp::Reverse(m.midi.unwrap_or(0))),
                PianoStaff::Left => chord_tones.sort_by_key(|m| m.midi.unwrap_or(127)),
            }
            let remaining = max_chord_tones.saturating_sub(keep.len());
            for meta in chord_tones.into_iter().take(remaining) {
                keep.insert(meta.raw_idx);
            }
            for meta in group {
                if !keep.contains(&meta.raw_idx) {
                    hidden.insert(meta.raw_idx);
                }
            }
        }
    }
    fn hide_parallel_import_notes(metas: &[NoteMeta], hidden: &mut BTreeSet<usize>, profile: ArrangementProfile) {
        let mut by_slot: BTreeMap<(PianoStaff, i64), Vec<&NoteMeta>> = BTreeMap::new();
        for meta in metas
            .iter()
            .filter(|m| !m.is_rest && m.midi.is_some() && !hidden.contains(&m.raw_idx))
        {
            by_slot.entry((meta.staff, meta.start)).or_default().push(meta);
        }
        for ((staff, _start), group) in by_slot {
            if group.len() <= 1 {
                continue;
            }
            let keep_count = match (staff, profile.preserve_reference) {
                (PianoStaff::Right, true) => 2,
                (PianoStaff::Left, true) => 2,
                (PianoStaff::Right, false) => 1,
                (PianoStaff::Left, false) => 1,
            };
            // Bevorzugt echte Stimmenanker, keine losgelösten <chord/>-Töne.
            let mut candidates: Vec<&NoteMeta> = group.iter().copied().filter(|m| !m.is_chord).collect();
            if candidates.is_empty() {
                candidates = group.clone();
            }
            match staff {
                PianoStaff::Right => candidates.sort_by_key(|m| std::cmp::Reverse(m.midi.unwrap_or(0))),
                PianoStaff::Left => candidates.sort_by_key(|m| m.midi.unwrap_or(127)),
            }
            let keep = candidates
                .into_iter()
                .take(keep_count)
                .map(|m| m.raw_idx)
                .collect::<BTreeSet<_>>();
            for meta in group {
                if !keep.contains(&meta.raw_idx) {
                    hidden.insert(meta.raw_idx);
                }
            }
        }
    }
    fn hide_secondary_voice_notes(metas: &[NoteMeta], hidden: &mut BTreeSet<usize>, profile: ArrangementProfile) {
        let mut total_by_staff: BTreeMap<PianoStaff, usize> = BTreeMap::new();
        let mut score_by_staff_voice: BTreeMap<(PianoStaff, String), i32> = BTreeMap::new();
        for meta in metas
            .iter()
            .filter(|m| !m.is_rest && m.midi.is_some() && !hidden.contains(&m.raw_idx))
        {
            *total_by_staff.entry(meta.staff).or_insert(0) += 1;
            let key = (meta.staff, meta.voice.trim().to_string());
            let mut score = 4;
            if meta.voice.trim() == meta.staff.default_voice() {
                score += 12;
            }
            if meta.duration > 96 {
                score += 3;
            }
            if !meta.is_chord {
                score += 2;
            }
            if meta.start.rem_euclid(192) == 0 {
                score += 2;
            }
            *score_by_staff_voice.entry(key).or_insert(0) += score;
        }
        let mut kept_voices: BTreeMap<PianoStaff, BTreeSet<String>> = BTreeMap::new();
        for staff in [PianoStaff::Right, PianoStaff::Left] {
            let staff_total = total_by_staff.get(&staff).copied().unwrap_or(0);
            let dense_limit = match (staff, profile.preserve_reference) {
                (PianoStaff::Right, true) => 18,
                (PianoStaff::Left, true) => 14,
                (PianoStaff::Right, false) => 12,
                (PianoStaff::Left, false) => 8,
            };
            if staff_total <= dense_limit {
                continue;
            }
            let max_voice_lanes = if profile.preserve_reference { 2 } else { 1 };
            let mut ranked = score_by_staff_voice
                .iter()
                .filter(|((candidate_staff, _voice), _score)| *candidate_staff == staff)
                .map(|((_, voice), score)| (voice.clone(), *score))
                .collect::<Vec<_>>();
            ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            let mut keep = BTreeSet::new();
            keep.insert(staff.default_voice().to_string());
            for (voice, _score) in ranked {
                if keep.len() >= max_voice_lanes {
                    break;
                }
                keep.insert(voice);
            }
            kept_voices.insert(staff, keep);
        }
        let mut to_hide = Vec::new();
        for meta in metas.iter() {
            if meta.is_rest || hidden.contains(&meta.raw_idx) {
                continue;
            }
            let Some(keep) = kept_voices.get(&meta.staff) else {
                continue;
            };
            if keep.contains(meta.voice.trim()) {
                continue;
            }
            // Lange Toene koennen musikalische Gegenstimmen sein; kurze Splitter aus
            // dichten Import-Voices sind dagegen meistens Notensatz-Artefakte.
            if meta.duration <= 192 {
                to_hide.push(meta.raw_idx);
            }
        }
        for raw_idx in to_hide {
            hidden.insert(raw_idx);
        }
    }
    fn hide_import_artifact_rests() {
        // v0.1.65 RhythmLedger: Pausen sind nicht mehr bloß technische Füller.
        // Fehlende sichtbare Pausen lassen Takte rhythmisch unvollständig wirken.
        // Deshalb werden importierte Pausen nicht mehr pauschal ausgeblendet.
    }
    fn hide_excess_measure_density(
        metas: &[NoteMeta],
        hidden: &mut BTreeSet<usize>,
        staff: PianoStaff,
        max_visible: usize,
    ) {
        let mut visible: Vec<&NoteMeta> = metas
            .iter()
            .filter(|m| m.staff == staff && !m.is_rest && !hidden.contains(&m.raw_idx))
            .collect();
        if visible.len() <= max_visible {
            return;
        }
        match staff {
            PianoStaff::Right => visible.sort_by_key(|m| {
                (
                    keep_priority_for_voice(m, staff),
                    m.midi.unwrap_or(0),
                    std::cmp::Reverse(m.start),
                )
            }),
            PianoStaff::Left => visible.sort_by_key(|m| {
                (
                    keep_priority_for_voice(m, staff),
                    std::cmp::Reverse(m.midi.unwrap_or(127)),
                    std::cmp::Reverse(m.start),
                )
            }),
        }
        let remove_count = visible.len().saturating_sub(max_visible);
        for meta in visible.into_iter().take(remove_count) {
            hidden.insert(meta.raw_idx);
        }
    }
    fn enforce_chord_anchor_visibility(metas: &[NoteMeta], hidden: &mut BTreeSet<usize>) {
        let mut current_anchor: Option<usize> = None;
        for meta in metas {
            if meta.is_rest {
                continue;
            }
            if !meta.is_chord {
                current_anchor = Some(meta.raw_idx);
                continue;
            }
            if current_anchor.is_none_or(|idx| hidden.contains(&idx)) {
                hidden.insert(meta.raw_idx);
            }
        }
    }
    fn keep_priority_for_voice(meta: &NoteMeta, staff: PianoStaff) -> i32 {
        // Niedriger Wert heißt: eher entbehrlich.
        let mut score = 0;
        if meta.voice.trim() == staff.default_voice() {
            score += 4;
        }
        if meta.duration > 96 {
            score += 2;
        }
        if !meta.is_chord {
            score += 1;
        }
        if meta.start.rem_euclid(192) == 0 {
            score += 2;
        }
        score
    }
    #[derive(Debug, Clone, Copy, Default)]
    struct NoteChange {
        staff_added: bool,
        voice_added: bool,
        stem_added: bool,
    }
    fn arrange_note(note_text: &str) -> (String, NoteChange) {
        if note_text.contains("<rest") || note_text.contains("<rest/") {
            return (note_text.to_string(), NoteChange::default());
        }
        let staff = extract_first_text(note_text, "staff")
            .and_then(|s| match s.trim() {
                "1" => Some(PianoStaff::Right),
                "2" => Some(PianoStaff::Left),
                _ => None,
            })
            .or_else(|| staff_from_pitch(note_text))
            .unwrap_or(PianoStaff::Right);
        let mut out = note_text.to_string();
        let mut changed = NoteChange::default();
        if !contains_simple_tag(&out, "staff") {
            out = insert_before_note_end(&out, &format!("    <staff>{}</staff>\n", staff.number()));
            changed.staff_added = true;
        }
        if !contains_simple_tag(&out, "voice") {
            out = insert_after_duration_or_before_end(
                &out,
                &format!("    <voice>{}</voice>\n", staff.default_voice()),
            );
            changed.voice_added = true;
        }
        if !contains_simple_tag(&out, "stem") && contains_simple_tag(&out, "type") {
            out = insert_after_type_or_before_end(&out, &format!("    <stem>{}</stem>\n", staff.stem()));
            changed.stem_added = true;
        }
        out = sync_note_type_and_dots(&out);
        (out, changed)
    }
    fn move_note_to_left_system_for_human_potential(note_text: &str) -> String {
        if note_text.contains("<rest") || note_text.contains("<rest/") {
            return note_text.to_string();
        }
        // v0.1.109: Nur das System wechseln, die ursprüngliche Voice aber erhalten.
        // Dadurch bleibt die MusicXML-Zeitachse exakt dieselbe; Voice 2 wird nicht
        // zusätzlich gefüllt und kann nicht überlaufen.
        let mut out = ensure_child_text_value_local(note_text, "staff", PianoStaff::Left.number());
        if contains_simple_tag(&out, "stem") {
            out = ensure_child_text_value_local(&out, "stem", PianoStaff::Left.stem());
        } else if contains_simple_tag(&out, "type") {
            out = insert_after_type_or_before_end(&out, &format!("    <stem>{}</stem>\n", PianoStaff::Left.stem()));
        }
        out
    }
    fn staff_from_pitch(note_text: &str) -> Option<PianoStaff> {
        let midi = midi_from_note_text(note_text)?;
        if midi < 60 {
            Some(PianoStaff::Left)
        } else {
            Some(PianoStaff::Right)
        }
    }
    fn midi_from_note_text(note_text: &str) -> Option<i32> {
        let step = extract_first_text(note_text, "step")?;
        let octave = extract_first_text(note_text, "octave")?.trim().parse::<i32>().ok()?;
        let alter = extract_first_text(note_text, "alter")
            .and_then(|v| v.trim().parse::<i32>().ok())
            .unwrap_or(0);
        midi_for(&step, alter, octave)
    }
    fn midi_for(step: &str, alter: i32, octave: i32) -> Option<i32> {
        let base = match step.trim() {
            "C" => 0,
            "D" => 2,
            "E" => 4,
            "F" => 5,
            "G" => 7,
            "A" => 9,
            "B" => 11,
            _ => return None,
        };
        Some((octave + 1) * 12 + base + alter)
    }
    fn collect_raw_notes(text: &str) -> Vec<RawNote> {
        let mut notes = Vec::new();
        let mut search_from = 0_usize;
        while let Some(rel_start) = text[search_from..].find("<note") {
            let start = search_from + rel_start;
            let Some(open_end_rel) = text[start..].find('>') else {
                break;
            };
            let start_tag = &text[start..start + open_end_rel + 1];
            if start_tag.ends_with("/>") {
                let end = start + open_end_rel + 1;
                notes.push(RawNote { start, end, text: text[start..end].to_string() });
                search_from = end;
                continue;
            }
            let content_start = start + open_end_rel + 1;
            let Some(close_rel) = text[content_start..].find("</note>") else {
                break;
            };
            let end = content_start + close_rel + "</note>".len();
            notes.push(RawNote { start, end, text: text[start..end].to_string() });
            search_from = end;
        }
        notes
    }
    fn mark_note_hidden(note_text: &str) -> String {
        if note_text.starts_with("<note ") && note_text.contains("print-object=\"no\"") {
            return note_text.to_string();
        }
        if let Some(open_end) = note_text.find('>') {
            let start_tag = &note_text[..open_end];
            if start_tag.contains("print-object=") {
                return note_text.to_string();
            }
            let mut out = String::with_capacity(note_text.len() + 38);
            out.push_str(start_tag);
            out.push_str(" print-object=\"no\" print-spacing=\"no\"");
            out.push_str(&note_text[open_end..]);
            out
        } else {
            note_text.to_string()
        }
    }
    fn restify_note_for_rhythm_ledger(note_text: &str) -> String {
        crate::structural_write_authority::restify_rhythm_ledger_note_authorized(note_text)
    }
    #[derive(Debug, Clone, Copy)]
    struct VisibleLedgerNormalizedOrigin {
        pre_raw_idx: usize,
        fragment_ordinal: usize,
    }

    #[derive(Debug, Clone, Copy)]
    struct VisibleLedgerFragmentLineageRow {
        pre_raw_idx: usize,
        post_raw_idx: usize,
        fragment_ordinal: usize,
    }

    #[derive(Debug, Clone, Default)]
    struct VisibleLedgerStats {
        births: BirthDrafts,
        capture_births: bool,
        rests_inserted: usize,
        values_split: usize,
        chord_cohorts_source_bound_split: usize,
        notes_normalized: usize,
        measures_completed: usize,
        post_raw_note_count: usize,
        fragment_lineage_rows: Vec<VisibleLedgerFragmentLineageRow>,
    }
    #[derive(Debug, Clone)]
    struct VisibleLedgerEvent {
        raw_idx: usize,
        start: i64,
        duration: i64,
        staff: PianoStaff,
        voice: String,
        visible: bool,
        is_chord: bool,
    }
    fn cross_staff_voice_names_for_ledger(events: &[VisibleLedgerEvent]) -> BTreeSet<String> {
        let mut staffs_by_voice: BTreeMap<String, BTreeSet<PianoStaff>> = BTreeMap::new();
        for event in events {
            staffs_by_voice
                .entry(event.voice.clone())
                .or_default()
                .insert(event.staff);
        }
        staffs_by_voice
            .into_iter()
            .filter_map(|(voice, staffs)| if staffs.len() > 1 { Some(voice) } else { None })
            .collect()
    }
    fn apply_visible_rhythm_ledger(measure_text: &str, rhythm_context: RhythmContext, mut course_measure: Option<&mut CourseMeasure>, capture_births: bool) -> (String, VisibleLedgerStats) {
        let expected = rhythm_context.expected_duration();
        let mut stats = VisibleLedgerStats { capture_births, ..VisibleLedgerStats::default() };
        let (normalized, normalized_origins) =
            normalize_visible_note_values(measure_text, rhythm_context, &mut stats, course_measure.as_deref_mut());
        let raw_notes = collect_raw_notes(&normalized);
        if raw_notes.len() != normalized_origins.len() {
            stats.post_raw_note_count = raw_notes.len();
            return (normalized, stats);
        }
        if raw_notes.is_empty() {
            stats.post_raw_note_count = 0;
            return (normalized, stats);
        }
        if let Some(courses) = course_measure {
            courses.complete(&normalized).expect("Full course coverage after actual normalization");
            stats.fragment_lineage_rows = normalized_origins.iter().enumerate()
                .map(|(post_raw_idx, origin)| VisibleLedgerFragmentLineageRow {
                    pre_raw_idx: origin.pre_raw_idx, post_raw_idx, fragment_ordinal: origin.fragment_ordinal,
                }).collect();
            stats.post_raw_note_count = raw_notes.len();
            return (normalized, stats);
        }
        let metas = collect_note_metas(&normalized, &raw_notes);
        let mut events = Vec::new();
        for meta in metas {
            let Some(raw) = raw_notes.get(meta.raw_idx) else { continue; };
            events.push(VisibleLedgerEvent {
                raw_idx: meta.raw_idx,
                start: meta.start,
                duration: meta.duration,
                staff: meta.staff,
                voice: meta.voice.trim().to_string(),
                visible: is_note_block_visible(&raw.text),
                is_chord: meta.is_chord,
            });
        }
        // v0.1.111: Cross-staff-Stimmen als gemeinsame Voice-Timeline behandeln.
        // HumanPotential verschiebt vorhandene Noten nur per <staff> in das untere
        // System. Die Stimme bleibt gleich; der VisibleRhythmLedger darf daraus
        // keine zweite Staff-Timeline mit zusätzlichen Pausen bauen.
        let visible_events = events
            .into_iter()
            .filter(|e| e.visible && !e.is_chord && e.duration > 0)
            .collect::<Vec<_>>();
        let cross_staff_voices = cross_staff_voice_names_for_ledger(&visible_events);
        let mut by_voice: BTreeMap<(Option<PianoStaff>, String), Vec<VisibleLedgerEvent>> = BTreeMap::new();
        for event in visible_events {
            let key = if cross_staff_voices.contains(&event.voice) {
                (None, event.voice.clone())
            } else {
                (Some(event.staff), event.voice.clone())
            };
            by_voice.entry(key).or_default().push(event);
        }
        let mut before: BTreeMap<usize, String> = BTreeMap::new();
        let mut after: BTreeMap<usize, String> = BTreeMap::new();
        let mut completed_any = false;
        for ((staff_key, voice), mut voice_events) in by_voice {
            voice_events.sort_by_key(|e| (e.start, e.raw_idx));
            let mut cursor = 0_i64;
            let mut last_idx: Option<usize> = None;
            let mut last_staff = staff_key.or_else(|| voice_events.first().map(|e| e.staff)).unwrap_or(PianoStaff::Right);
            for event in &voice_events {
                if event.start > cursor {
                    let rest_staff = staff_key.unwrap_or(last_staff);
                    let rest = make_visible_rest_sequence_at(
                        cursor,
                        event.start - cursor,
                        rest_staff,
                        &voice,
                        rhythm_context,
                    );
                    if !rest.is_empty() {
                        before.entry(event.raw_idx).or_default().push_str(&rest);
                        stats.rests_inserted += count_note_blocks(&rest);
                        completed_any = true;
                    }
                }
                cursor = cursor.max(event.start + event.duration);
                last_idx = Some(event.raw_idx);
                last_staff = event.staff;
            }
            if cursor < expected
                && let Some(idx) = last_idx {
                    let rest_staff = staff_key.unwrap_or(last_staff);
                    let rest = make_visible_rest_sequence_at(
                        cursor,
                        expected - cursor,
                        rest_staff,
                        &voice,
                        rhythm_context,
                    );
                    if !rest.is_empty() {
                        after.entry(idx).or_default().push_str(&rest);
                        stats.rests_inserted += count_note_blocks(&rest);
                        completed_any = true;
                    }
                }
        }
        if before.is_empty() && after.is_empty() {
            stats.fragment_lineage_rows = normalized_origins
                .iter()
                .enumerate()
                .map(|(post_raw_idx, origin)| VisibleLedgerFragmentLineageRow {
                    pre_raw_idx: origin.pre_raw_idx,
                    post_raw_idx,
                    fragment_ordinal: origin.fragment_ordinal,
                })
                .collect();
            stats.post_raw_note_count = raw_notes.len();
            return (normalized, stats);
        }
        let raw_notes = collect_raw_notes(&normalized);
        let mut out = String::with_capacity(normalized.len() + 128 * (before.len() + after.len()));
        let mut last = 0_usize;
        let mut post_raw_idx = 0usize;
        let mut rest_insertion_survivors = Vec::<(usize, usize)>::with_capacity(raw_notes.len());
        let mut output_births = BirthDrafts::default();
        for (idx, note) in raw_notes.iter().enumerate() {
            out.push_str(&normalized[last..note.start]);
            if let Some(ins) = before.get(&idx) {
                if capture_births { output_births.capture_visible_sequence(&VisibleRhythmBlockMint { _sealed: () },
                    None, PhysicalBirthKind::GapRest, out.len(), ins); }
                out.push_str(ins);
                post_raw_idx = post_raw_idx.saturating_add(count_note_blocks(ins));
            }
            let origin = normalized_origins[idx];
            stats.fragment_lineage_rows.push(VisibleLedgerFragmentLineageRow {
                pre_raw_idx: origin.pre_raw_idx,
                post_raw_idx,
                fragment_ordinal: origin.fragment_ordinal,
            });
            rest_insertion_survivors.push((idx, post_raw_idx));
            output_births.carry_emission(&stats.births, note.start..note.end, out.len());
            out.push_str(&normalized[note.start..note.end]);
            post_raw_idx = post_raw_idx.saturating_add(1);
            if let Some(ins) = after.get(&idx) {
                if capture_births { output_births.capture_visible_sequence(&VisibleRhythmBlockMint { _sealed: () },
                    None, PhysicalBirthKind::GapRest, out.len(), ins); }
                out.push_str(ins);
                post_raw_idx = post_raw_idx.saturating_add(count_note_blocks(ins));
            }
            last = note.end;
        }
        out.push_str(&normalized[last..]);

        // r690: VisibleRhythm rest insertion is representation-only. A rest that is
        // textually inserted into MusicXML can advance the shared MusicXML cursor even
        // when the intended gap belongs only to one voice lane. Never accept such a
        // completion if it moves any already existing pitched fragment in musical time
        // or changes its pitch/staff/voice/chord identity. Roll back only the inserted
        // rests for this measure; the preceding note-value/type normalization remains.
        if !visible_rhythm_rest_insertion_preserves_pitched_timeline(
            &normalized,
            &out,
            &rest_insertion_survivors,
        ) {
            stats.rests_inserted = 0;
            stats.measures_completed = 0;
            stats.fragment_lineage_rows = normalized_origins
                .iter()
                .enumerate()
                .map(|(post_raw_idx, origin)| VisibleLedgerFragmentLineageRow {
                    pre_raw_idx: origin.pre_raw_idx,
                    post_raw_idx,
                    fragment_ordinal: origin.fragment_ordinal,
                })
                .collect();
            stats.post_raw_note_count = raw_notes.len();
            return (normalized, stats);
        }

        if completed_any {
            stats.measures_completed = 1;
        }
        stats.births = output_births;
        stats.post_raw_note_count = post_raw_idx;
        (out, stats)
    }

    fn visible_rhythm_rest_insertion_preserves_pitched_timeline(
        before_rest_insertion: &str,
        after_rest_insertion: &str,
        survivors: &[(usize, usize)],
    ) -> bool {
        let before_raw = collect_raw_notes(before_rest_insertion);
        let after_raw = collect_raw_notes(after_rest_insertion);
        if before_raw.len() != survivors.len() {
            return false;
        }
        let before_metas = collect_note_metas(before_rest_insertion, &before_raw);
        let after_metas = collect_note_metas(after_rest_insertion, &after_raw);
        if before_metas.len() != before_raw.len() || after_metas.len() != after_raw.len() {
            return false;
        }
        let before_by_raw = before_metas
            .iter()
            .map(|meta| (meta.raw_idx, meta))
            .collect::<BTreeMap<_, _>>();
        let after_by_raw = after_metas
            .iter()
            .map(|meta| (meta.raw_idx, meta))
            .collect::<BTreeMap<_, _>>();

        survivors.iter().all(|(before_raw_idx, after_raw_idx)| {
            let Some(before) = before_by_raw.get(before_raw_idx).copied() else {
                return false;
            };
            if before.midi.is_none() {
                return true;
            }
            let Some(after) = after_by_raw.get(after_raw_idx).copied() else {
                return false;
            };
            before.start == after.start
                && before.duration == after.duration
                && before.staff == after.staff
                && before.voice.trim() == after.voice.trim()
                && before.is_chord == after.is_chord
                && before.midi == after.midi
        })
    }
    fn normalize_visible_note_values(
        measure_text: &str,
        rhythm_context: RhythmContext,
        stats: &mut VisibleLedgerStats,
        mut course_measure: Option<&mut CourseMeasure>,
    ) -> (String, Vec<VisibleLedgerNormalizedOrigin>) {
        let beat_duration = rhythm_context.beat_duration();
        let raw_notes = collect_raw_notes(measure_text);
        if raw_notes.is_empty() {
            return (measure_text.to_string(), Vec::new());
        }
        let metas = collect_note_metas(measure_text, &raw_notes);
        let mut meta_by_raw: BTreeMap<usize, NoteMeta> = BTreeMap::new();
        for meta in metas {
            meta_by_raw.insert(meta.raw_idx, meta);
        }
        let mut out = String::with_capacity(measure_text.len() + 256);
        let mut origins = Vec::<VisibleLedgerNormalizedOrigin>::new();
        let mut last = 0_usize;
        let mut cohort_end = 0;
        for (idx, note) in raw_notes.iter().enumerate() {
            if idx < cohort_end { continue; }
            out.push_str(&measure_text[last..note.start]);
            let visible = is_note_block_visible(&note.text);
            let is_chord = note.text.contains("<chord") || note.text.contains("<chord/");
            let is_rest = note.text.contains("<rest") || note.text.contains("<rest/");
            let duration = extract_first_i64(&note.text, "duration").unwrap_or(0).max(0);
            let start = meta_by_raw.get(&idx).map_or(0, |m| m.start);
            if visible && !is_chord && duration > 0 {
                let staff = extract_first_text(&note.text, "staff")
                    .and_then(|s| match s.trim() {
                        "1" => Some(PianoStaff::Right),
                        "2" => Some(PianoStaff::Left),
                        _ => None,
                    })
                    .or_else(|| staff_from_pitch(&note.text))
                    .unwrap_or(PianoStaff::Right);
                let voice = extract_first_text(&note.text, "voice")
                    .unwrap_or_else(|| staff.default_voice().to_string());
                let parts = if is_rest {
                    crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
                        start,
                        duration,
                        rhythm_context.expected_duration(),
                        beat_duration,
                    )
                } else {
                    split_duration_for_metric_notation(start, duration, beat_duration.max(1))
                };
                if is_rest && parts.is_empty() {
                    if let Some(courses) = course_measure.as_deref_mut() {
                        courses.push_normalized(idx, &note.text).expect("Unsplit rest custody");
                    }
                    if stats.capture_births { stats.births.capture_continuation(note.start..note.end, out.len(), &note.text); }
                    out.push_str(&note.text);
                    origins.push(VisibleLedgerNormalizedOrigin {
                        pre_raw_idx: idx,
                        fragment_ordinal: 0,
                    });
                    last = note.end;
                    continue;
                }
                let must_split = parts.len() > 1 || !is_exact_notation_duration(duration);
                if must_split {
                    if let Some(courses) = course_measure.as_deref_mut() {
                        let mut end = idx + 1;
                        while end < raw_notes.len() && meta_by_raw.get(&end).is_some_and(|m| m.is_chord) {
                            end += 1;
                        }
                        if end > idx + 1 && parts.len() > 1 {
                            // This is one physical host transaction, not a scorewide
                            // winner. Origin support is checked before realization.
                            courses.cohort_support(idx, end, &parts)
                                .expect("Complete source-bound chord cohort before split");
                            for member in idx + 1..end {
                                assert!(measure_text[raw_notes[member - 1].end..raw_notes[member].start].trim().is_empty(),
                                    "Chord cohort crosses a cursor operation or independent XML object");
                            }
                            let replacements = raw_notes[idx..end].iter().enumerate().map(|(member, raw)| {
                                if member == 0 && is_rest {
                                    make_visible_rest_sequence_from_parts(&parts, staff, voice.trim())
                                } else {
                                    split_visible_pitched_note_with_parts(&raw.text, &parts)
                                }
                            }).collect::<Vec<_>>();
                            let blocks = replacements.iter().map(|text| collect_raw_notes(text)).collect::<Vec<_>>();
                            assert!(blocks.iter().all(|notes| notes.len() == parts.len()), "Incomplete physical cohort split");
                            courses.cohort_normalized(idx, &replacements).expect("Actual cohort split origins");
                            for fragment_ordinal in 0..parts.len() {
                                for (member, notes) in blocks.iter().enumerate() {
                                    out.push_str(&notes[fragment_ordinal].text);
                                    out.push('\n');
                                    origins.push(VisibleLedgerNormalizedOrigin {
                                        pre_raw_idx: idx + member, fragment_ordinal,
                                    });
                                }
                            }
                            stats.values_split += end - idx;
                            stats.chord_cohorts_source_bound_split += 1;
                            last = raw_notes[end - 1].end;
                            cohort_end = end;
                            continue;
                        }
                    }
                    let replacement = if is_rest {
                        make_visible_rest_sequence_from_parts(&parts, staff, voice.trim())
                    } else {
                        split_visible_pitched_note_with_parts(&note.text, &parts)
                    };
                    if let Some(courses) = course_measure.as_deref_mut() {
                        courses.push_normalized(idx, &replacement).expect("Actual split course ancestry");
                    }
                    if stats.capture_births && course_measure.is_none() && parts.len() > 1 {
                        stats.births.capture_visible_sequence(&VisibleRhythmBlockMint { _sealed: () },
                            Some(note.start..note.end), PhysicalBirthKind::MetricFragment, out.len(), &replacement);
                    }
                    if stats.capture_births && parts.len() <= 1 {
                        stats.births.capture_continuation(note.start..note.end, out.len(), &replacement);
                    }
                    out.push_str(&replacement);
                    for fragment_ordinal in 0..parts.len() {
                        origins.push(VisibleLedgerNormalizedOrigin {
                            pre_raw_idx: idx,
                            fragment_ordinal,
                        });
                    }
                    stats.values_split += 1;
                    last = note.end;
                    continue;
                }
            }
            let replacement = if visible && duration > 0 {
                let synced = sync_note_type_and_dots(&note.text);
                if synced != note.text { stats.notes_normalized += 1; }
                synced
            } else { note.text.clone() };
            if let Some(courses) = course_measure.as_deref_mut() {
                courses.push_normalized(idx, &replacement).expect("Unsplit occurrence custody");
            }
            if stats.capture_births { stats.births.capture_continuation(note.start..note.end, out.len(), &replacement); }
            out.push_str(&replacement);
            origins.push(VisibleLedgerNormalizedOrigin {
                pre_raw_idx: idx,
                fragment_ordinal: 0,
            });
            last = note.end;
        }
        out.push_str(&measure_text[last..]);
        (out, origins)
    }
    fn split_visible_pitched_note_with_parts(note_text: &str, parts: &[i64]) -> String {
        if parts.len() <= 1 {
            return sync_note_type_and_dots(note_text);
        }
        let mut out = String::new();
        for (idx, part_duration) in parts.iter().copied().enumerate() {
            let mut block = ensure_child_text_value_local(note_text, "duration", &part_duration.to_string());
            block = remove_child_blocks(&block, "beam");
            block = remove_child_blocks(&block, "notations");
            block = remove_child_blocks(&block, "tie");
            block = sync_note_type_and_dots(&block);
            if idx == 0 {
                block = add_tie_markup(&block, true, false);
            } else if idx + 1 == parts.len() {
                block = add_tie_markup(&block, false, true);
            } else {
                block = add_tie_markup(&block, true, true);
            }
            out.push_str(&block);
            if !out.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }
    fn add_tie_markup(note_text: &str, start: bool, stop: bool) -> String {
        crate::structural_write_authority::add_tie_markup_authorized(note_text, start, stop).0
    }
    fn make_visible_rest_sequence_at(
        start: i64,
        duration: i64,
        staff: PianoStaff,
        voice: &str,
        rhythm_context: RhythmContext,
    ) -> String {
        let parts = crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
            start,
            duration,
            rhythm_context.expected_duration(),
            rhythm_context.beat_duration(),
        );
        make_visible_rest_sequence_from_parts(&parts, staff, voice)
    }
    fn make_visible_rest_sequence_from_parts(parts: &[i64], staff: PianoStaff, voice: &str) -> String {
        let mut out = String::new();
        for part in parts.iter().copied() {
            out.push_str(&make_visible_rest_block(part, staff, voice));
        }
        out
    }
    fn make_visible_rest_block(duration: i64, staff: PianoStaff, voice: &str) -> String {
        let (typ, dots) = type_and_dots_for_duration_local(duration);
        let mut out = String::new();
        out.push_str("      <note>\n");
        out.push_str("        <rest/>\n");
        out.push_str(&format!("        <duration>{duration}</duration>\n"));
        out.push_str(&format!("        <voice>{}</voice>\n", voice.trim()));
        out.push_str(&format!("        <type>{typ}</type>\n"));
        for _ in 0..dots {
            out.push_str("        <dot/>\n");
        }
        out.push_str(&format!("        <staff>{}</staff>\n", staff.number()));
        out.push_str("      </note>\n");
        out
    }
    fn is_note_block_visible(note_text: &str) -> bool {
        !note_text.contains("print-object=\"no\"") && !note_text.contains("print-object='no'")
    }
    fn is_exact_notation_duration(duration: i64) -> bool {
        matches!(duration, 384 | 288 | 192 | 144 | 96 | 72 | 48 | 36 | 24 | 18 | 12 | 9 | 6)
    }
    fn split_duration_for_metric_notation(start: i64, duration: i64, beat_duration: i64) -> Vec<i64> {
        let beat_duration = beat_duration.max(1);
        if duration <= 0 {
            return Vec::new();
        }
        let mut remaining = duration;
        let mut cursor = start;
        let mut parts = Vec::new();
        while remaining > 0 {
            let offset = cursor.rem_euclid(beat_duration);
            let to_next_beat = if offset == 0 { beat_duration } else { beat_duration - offset };
            let take = remaining.min(to_next_beat);
            for part in split_duration_for_visible_notation(take) {
                parts.push(part);
            }
            remaining -= take;
            cursor += take;
            if parts.len() > 64 {
                break;
            }
        }
        parts
    }
    fn split_duration_for_visible_notation(mut duration: i64) -> Vec<i64> {
        let units = [384, 288, 192, 144, 96, 72, 48, 36, 24, 18, 12, 9, 6];
        let mut parts = Vec::new();
        while duration > 0 {
            let mut chosen = None;
            for unit in units {
                if unit <= duration {
                    chosen = Some(unit);
                    break;
                }
            }
            let part = chosen.unwrap_or(6);
            parts.push(part);
            duration = duration.saturating_sub(part);
            if parts.len() > 32 {
                break;
            }
        }
        parts
    }
    fn type_and_dots_for_duration_local(duration: i64) -> (&'static str, usize) {
        match duration {
            384 => ("whole", 0),
            288 => ("half", 1),
            192 => ("half", 0),
            144 => ("quarter", 1),
            96 => ("quarter", 0),
            72 => ("eighth", 1),
            48 => ("eighth", 0),
            36 => ("16th", 1),
            24 => ("16th", 0),
            18 => ("32nd", 1),
            12 => ("32nd", 0),
            9 => ("64th", 1),
            6 => ("64th", 0),
            _ => ("64th", 0),
        }
    }
    fn sync_note_type_and_dots(note_text: &str) -> String {
        crate::structural_write_authority::sync_arrangement_note_type_and_dots_authorized(note_text)
    }
    fn ensure_child_text_value_local(text: &str, tag: &str, value: &str) -> String {
        crate::structural_write_authority::ensure_arrangement_child_text_value_authorized(text, tag, value)
    }
    fn count_note_blocks(text: &str) -> usize {
        text.matches("<note").count()
    }
    fn child_block_range(text: &str, tag: &str) -> Option<(usize, usize)> {
        let open_plain = format!("<{tag}>");
        let open_attr = format!("<{tag} ");
        let open_start = text.find(&open_plain).or_else(|| text.find(&open_attr))?;
        let open_end = text[open_start..].find('>')? + open_start + 1;
        let close = format!("</{tag}>");
        let close_end = text[open_end..].find(&close)? + open_end + close.len();
        Some((open_start, close_end))
    }
    fn remove_child_blocks(text: &str, tag: &str) -> String {
        let mut out = text.to_string();
        while let Some((start, end)) = child_block_range(&out, tag) {
            out.replace_range(start..end, "");
        }
        let empty = format!("<{tag} />");
        out = out.replace(&empty, "");
        let empty = format!("<{tag}/>");
        out.replace(&empty, "")
    }
    fn contains_simple_tag(text: &str, tag: &str) -> bool {
        text.contains(&format!("<{tag}>")) || text.contains(&format!("<{tag} "))
    }
    fn extract_first_i64(text: &str, tag: &str) -> Option<i64> {
        extract_first_text(text, tag).and_then(|value| value.trim().parse::<i64>().ok())
    }
    fn extract_first_text(text: &str, tag: &str) -> Option<String> {
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");
        let start = text.find(&open)? + open.len();
        let end = text[start..].find(&close)? + start;
        Some(text[start..end].trim().to_string())
    }
    fn insert_after_duration_or_before_end(note_text: &str, insertion: &str) -> String {
        if let Some(pos) = note_text.find("</duration>") {
            let at = pos + "</duration>".len();
            return insert_at_line_boundary(note_text, at, insertion);
        }
        insert_before_note_end(note_text, insertion)
    }
    fn insert_after_type_or_before_end(note_text: &str, insertion: &str) -> String {
        if let Some(pos) = note_text.find("</type>") {
            let at = pos + "</type>".len();
            return insert_at_line_boundary(note_text, at, insertion);
        }
        insert_before_note_end(note_text, insertion)
    }
    fn insert_at_line_boundary(note_text: &str, at: usize, insertion: &str) -> String {
        let mut out = String::with_capacity(note_text.len() + insertion.len() + 1);
        out.push_str(&note_text[..at]);
        if !out.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(insertion);
        out.push_str(&note_text[at..]);
        out
    }
    fn insert_before_note_end(note_text: &str, insertion: &str) -> String {
        if let Some(close_pos) = note_text.rfind("</note>") {
            let mut out = String::with_capacity(note_text.len() + insertion.len() + 1);
            out.push_str(&note_text[..close_pos]);
            if !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(insertion);
            out.push_str(&note_text[close_pos..]);
            out
        } else {
            note_text.to_string()
        }
    }
