// v0.10.726 real crate-root module split from the old score grouping.
// No behavior change: original module body, visibility and call paths are preserved.

    use std::collections::BTreeSet;
    use crate::arrangement::representation_transport::{RepresentationStep, RhythmicRepresentationTransport};
    
    #[derive(Debug, Clone, Default)]
    pub struct HorizontalLayoutSeedSlot {
        pub starts_new_system: bool,
        pub starts_new_page: bool,
    }

    #[derive(Debug, Clone, Default)]
    pub struct HorizontalLayoutSeedPlan {
        pub slots: Vec<HorizontalLayoutSeedSlot>,
    }

    #[derive(Debug)]
    pub struct LayoutResult {
        pub(crate) rhythmic_transport: RhythmicRepresentationTransport,
        pub xml: String,
        pub horizontal_layout_seed: HorizontalLayoutSeedPlan,
        /// PRESERVING receipt (E056), minted by this owner only if the
        /// conserved musical content of input and output is identical.
        pub(crate) preserving_receipt: Option<crate::master_state_identity::PreservingTransformationReceipt>,
    }

    /// Sealed mint capability of `apply_pianist_layout` (PRESERVING E056).
    pub(crate) struct PianistLayoutPreservingMint {
        _sealed: (),
    }
    
    #[derive(Debug, Clone)]
    pub struct AutoLayoutInput {
        pub measures_seen: usize,
        pub notes_seen: usize,
        pub notes_hidden: usize,
        pub rests_hidden: usize,
        pub chord_tones_hidden: usize,
        pub beam_groups: usize,
        pub beamed_notes: usize,
        pub reference_active: bool,
    }
    
    #[derive(Debug, Clone)]
    pub struct LayoutPlan {
        pub target_pages: usize,
    }

    impl LayoutPlan {
        pub fn manual(target_pages: usize) -> Self {
            Self { target_pages: target_pages.max(1) }
        }
    }

    /// Waehlt die Ziel-Seitenzahl fuer die Pianistenfassung automatisch.
    ///
    /// Ziel ist nicht maximale Kompaktheit, aber auch nicht zerfaserter Leerraum.
    /// Die Heuristik verteilt dichte Fassungen luftig, begrenzt kurze Etueden aber
    /// typischerweise auf 5-6 gut gefuellte Seiten.
    pub fn choose_auto_pianist_layout(input: AutoLayoutInput) -> LayoutPlan {
        let measures = input.measures_seen.max(1);
        let visible_notes = input.notes_seen.saturating_sub(input.notes_hidden);
        let visible_notes_per_measure = visible_notes as f64 / measures as f64;
        let beamed_notes_per_measure = input.beamed_notes as f64 / measures as f64;
        let beam_groups_per_measure = input.beam_groups as f64 / measures as f64;
        let hidden_total = input.notes_hidden + input.rests_hidden + input.chord_tones_hidden;
    
        let mut pages = if measures <= 16 {
            3
        } else if measures <= 28 {
            4
        } else if measures <= 36 {
            5
        } else {
            6
        };
    
        if input.reference_active {
            // Referenztreue Fassungen sollen nicht zusammengequetscht werden.
            // v0.10.262: Der Guard darf kurze Stücke aber nicht in künstliche
            // Drei-Takt-Seiten zerfasern. Referenztreue schützt Inhalt; die
            // Seitenfüllung bleibt ein eigenständiger Behind Bars-Guard.
            pages = pages.max(6);
        }
    
        if visible_notes_per_measure >= 18.0 || beamed_notes_per_measure >= 7.0 {
            pages = pages.max(6);
        }
    
        if beam_groups_per_measure >= 4.0 {
            pages = pages.max(6);
        }
    
        if hidden_total >= measures * 12 {
            pages = pages.max(6);
        }
    
        // BehindBarsPaginationDensityGuard v0.10.262:
        // Automatische Seitenziele dürfen nicht nur stabil sein, sondern müssen
        // pro Seite eine Mindest-Füllung behalten. Sonst entstehen künstliche
        // Drei-Takt-Seiten mit großem Weißraum. Der Guard ist rein datengetrieben:
        // Er verwendet nur Taktzahl, Sichtdichte und explizite Layout-Vorgaben;
        // keine Takt- oder Etüden-Sonderfälle.
        // BehindBarsSystemDensityNoArtificialBreaks v0.10.264:
        // Ein Seitenumbruch ist nicht schon deshalb sinnvoll, weil das Zielseiten-
        // Raster rechnerisch stabil ist. Kurze Stücke dürfen keine künstlichen
        // Unterfüllungsseiten erzeugen, insbesondere keine Titelseiten- oder
        // Mittelseiten-Limits mit nur zwei Systemen, solange ein weiteres System
        // ohne musikalische Änderung hineinpasst. Der Guard bleibt datengetrieben:
        // er nutzt nur Stücklänge und Dichte, keine konkreten Taktzahlen.
        // NonTerminalPageBackfillDensity v0.10.265:
        // Nicht-Endseiten dürfen nicht unterfüllt abbrechen, wenn das folgende
        // System noch zurückgezogen werden kann. Für kurze Stücke wird deshalb die
        // Zielseitenzahl aus einer Backfill-Mindestfüllung abgeleitet. Das ist
        // weiterhin nur Layout: keine Noten-, Tonhöhen-, Onset- oder Dauermutation.
        let short_score_density_guard = measures <= 20;
        let min_measures_per_auto_page = if short_score_density_guard {
            // v0.10.265: kurze Stücke sollen Nicht-Endseiten aktiv zurückfüllen.
            // Bei Zweitakt-Systemen entspricht das mindestens drei Systemen pro
            // Seite, sofern die Dichte nicht extrem ist. Dadurch verschwinden
            // künstliche mittlere Zwei-System-Seiten, ohne Takte zu benennen.
            6usize
        } else if visible_notes_per_measure >= 24.0 || beamed_notes_per_measure >= 9.0 {
            4usize
        } else {
            5usize
        };
        let density_max_pages = ceil_div(measures, min_measures_per_auto_page).max(1);
        if pages > density_max_pages {
            pages = density_max_pages;
        }
    
        // PrintQualityCheck: kurze Etueden duerfen nicht durch Auto-Layout zerfasern.
        // Mehr Seiten koennen weiterhin explizit per --layout-pages angefordert werden.
        pages = pages.clamp(1, 6);
    
        LayoutPlan { target_pages: pages }
    }
    
    /// Plant konservative MusicXML-Layout-Hinweise fuer eine lesbare Pianistenfassung.
    ///
    /// MuseScore komprimiert kurze Stuecke sonst gern auf zu wenige Seiten. Diese Schicht
    /// veraendert keine musikalischen Dauern und keine Notenhoehen; sie erzeugt nur den
    /// strukturellen Layout-Seed. Die horizontale Geometrie materialisiert spaeter genau
    /// ein zentraler StructuralWriteAuthority-Writer.
    pub fn apply_pianist_layout(xml: &str, target_pages: usize) -> LayoutResult {
        let effective_target = target_pages.max(1);
        let (part_start, part_end) = first_part_range(xml).unwrap_or((0, xml.len()));
        let original_part = &xml[part_start..part_end];
        let (part, print_edits) = strip_etuschlopf_layout_prints(original_part);
    
        let measures = collect_measure_numbers(&part);
        let last_measure = measures.iter().next_back().copied().unwrap_or(0);
    
        let page_breaks = compute_page_breaks(last_measure, effective_target);
        let system_breaks = compute_system_breaks(last_measure, &page_breaks);
        let horizontal_layout_seed =
            build_horizontal_layout_seed(&part, &page_breaks, &system_breaks);
    
        // Take 6: base page/system decisions enter only the structural seed.
    
        // Horizontal geometry is no longer materialized here; one central
    
        // materializer writes the closed plan after all horizontal-layout decisions.
    
        let rewritten_part = part;
    
        let mut rewritten = String::with_capacity(xml.len() + rewritten_part.len().saturating_sub(original_part.len()));
        rewritten.push_str(&xml[..part_start]);
        rewritten.push_str(&rewritten_part);
        rewritten.push_str(&xml[part_end..]);
    
        let preserving_receipt =
            crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
                PianistLayoutPreservingMint { _sealed: () },
                xml,
                &rewritten,
            );
        let rhythmic_transport = RhythmicRepresentationTransport::layout(
            &PianistLayoutPreservingMint { _sealed: () },
            vec![RepresentationStep::recorded(xml, &rewritten,
                print_edits.into_iter().map(|range|
                    (range.start + part_start..range.end + part_start, String::new())).collect())]);
        let preserving_receipt = preserving_receipt.map(|receipt|
            receipt.with_recorded_representation(rhythmic_transport.clone()));
        LayoutResult {
            rhythmic_transport,
            xml: rewritten,
            horizontal_layout_seed,
            preserving_receipt,
        }
    }
    
    fn first_part_range(xml: &str) -> Option<(usize, usize)> {
        let mut pos = 0usize;
        while let Some(rel) = xml[pos..].find("<part") {
            let start = pos + rel;
            if is_part_tag(xml, start) {
                let tail = &xml[start..];
                let end_rel = tail.find("</part>")?;
                return Some((start, start + end_rel));
            }
            pos = start + "<part".len();
        }
        None
    }
    
    fn is_part_tag(text: &str, start: usize) -> bool {
        let next = text.as_bytes().get(start + "<part".len()).copied();
        matches!(next, Some(b' ' | b'\t' | b'\n' | b'\r' | b'>'))
    }
    
    fn strip_etuschlopf_layout_prints(part: &str) -> (String, Vec<std::ops::Range<usize>>) {
        let mut edits: Vec<std::ops::Range<usize>> = Vec::new();
        for pattern in ["\n      <print new-page=\"yes\"/>", "\n      <print new-system=\"yes\"/>",
            "<print new-page=\"yes\"/>", "<print new-system=\"yes\"/>"] {
            for (start, matched) in part.match_indices(pattern) {
                let range = start..start + matched.len();
                if !edits.iter().any(|prior| prior.start < range.end && range.start < prior.end) {
                    edits.push(range);
                }
            }
        }
        edits.sort_by_key(|range| range.start);
        let rewritten = part.replace("\n      <print new-page=\"yes\"/>", "")
            .replace("\n      <print new-system=\"yes\"/>", "")
            .replace("<print new-page=\"yes\"/>", "")
            .replace("<print new-system=\"yes\"/>", "");
        (rewritten, edits)
    }
    
    fn collect_measure_numbers(part: &str) -> BTreeSet<usize> {
        let mut numbers = BTreeSet::new();
        let mut pos = 0usize;
    
        while let Some(rel) = part[pos..].find("<measure") {
            let start = pos + rel;
            if !is_measure_tag(part, start) {
                pos = start + "<measure".len();
                continue;
            }
            let Some(end_rel) = part[start..].find('>') else {
                break;
            };
            let tag_end = start + end_rel + 1;
            let tag = &part[start..tag_end];
            if let Some(number) = parse_measure_number(tag) {
                numbers.insert(number);
            }
            pos = tag_end;
        }
    
        numbers
    }
    
    fn compute_page_breaks(last_measure: usize, target_pages: usize) -> BTreeSet<usize> {
        let mut breaks = BTreeSet::new();
        if last_measure < 2 || target_pages < 2 {
            return breaks;
        }
    
        for page_idx in 1..target_pages {
            let break_measure = 1 + ceil_div(last_measure * page_idx, target_pages);
            if break_measure > 1 && break_measure <= last_measure {
                breaks.insert(break_measure);
            }
        }
    
        breaks
    }
    
    fn compute_system_breaks(last_measure: usize, page_breaks: &BTreeSet<usize>) -> BTreeSet<usize> {
        let mut breaks = BTreeSet::new();
        if last_measure < 4 {
            return breaks;
        }
    
        let mut starts = Vec::with_capacity(page_breaks.len() + 1);
        starts.push(1usize);
        starts.extend(page_breaks.iter().copied());
    
        for (idx, start) in starts.iter().copied().enumerate() {
            let end_exclusive = starts.get(idx + 1).copied().unwrap_or(last_measure + 1);
            // v0.1.88: homogeneres Seitenbild: grundsätzlich zwei Takte pro
            // System. Das vermeidet sehr unterschiedliche Schluss-/Mittelsysteme
            // und bleibt generisch, weil es nur aus der Taktfolge abgeleitet ist.
            let mut measure = start + 2;
            while measure < end_exclusive {
                if !page_breaks.contains(&measure) {
                    breaks.insert(measure);
                }
                measure += 2;
            }
        }
    
        breaks
    }
    
    fn build_horizontal_layout_seed(
        part: &str,
        page_breaks: &BTreeSet<usize>,
        system_breaks: &BTreeSet<usize>,
    ) -> HorizontalLayoutSeedPlan {
        let mut slots = Vec::new();
        let mut pos = 0usize;

        while let Some(rel) = part[pos..].find("<measure") {
            let start = pos + rel;
            if !is_measure_tag(part, start) {
                pos = start + "<measure".len();
                continue;
            }
            let Some(end_rel) = part[start..].find('>') else {
                break;
            };
            let tag_end = start + end_rel + 1;
            let opening_tag = &part[start..tag_end];
            let measure_end = part[tag_end..]
                .find("</measure>")
                .map_or(tag_end, |rel| tag_end + rel);
            let body = &part[tag_end..measure_end];
            let number = parse_measure_number(opening_tag);

            let starts_new_page = body.contains("new-page=\"yes\"")
                || body.contains("new-page='yes'")
                || number.is_some_and(|value| page_breaks.contains(&value));
            let starts_new_system = body.contains("new-system=\"yes\"")
                || body.contains("new-system='yes'")
                || number.is_some_and(|value| system_breaks.contains(&value));

            slots.push(HorizontalLayoutSeedSlot {
                starts_new_system,
                starts_new_page,
            });
            pos = tag_end;
        }

        HorizontalLayoutSeedPlan { slots }
    }

    fn is_measure_tag(text: &str, start: usize) -> bool {
        let next = text.as_bytes().get(start + "<measure".len()).copied();
        matches!(next, Some(b' ' | b'\t' | b'\n' | b'\r' | b'>'))
    }
    
    fn parse_measure_number(tag: &str) -> Option<usize> {
        parse_attr(tag, "number")?.parse::<usize>().ok()
    }
    
    fn parse_attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
        let needle = format!("{name}=");
        let start = tag.find(&needle)? + needle.len();
        let quote = tag.as_bytes().get(start).copied()?;
        if quote != b'\"' && quote != b'\'' {
            return None;
        }
        let value_start = start + 1;
        let end_rel = tag[value_start..].bytes().position(|b| b == quote)?;
        Some(&tag[value_start..value_start + end_rel])
    }
    
    fn ceil_div(n: usize, d: usize) -> usize {
        if d == 0 {
            return n;
        }
        n.div_ceil(d)
    }


#[cfg(test)]
mod tests {
    use super::apply_pianist_layout;

    #[test]
    fn existing_rich_system_break_is_not_duplicated() {
        let xml = "<?xml version=\"1.0\"?>\n<score-partwise version=\"4.0\">\n  <part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list>\n  <part id=\"P1\">\n    <measure number=\"\
1\"><note><rest/><duration>384</duration></note></measure>\n    <measure number=\"\
2\"><note><rest/><duration>384</duration></note></measure>\n    <measure number=\"\
3\" width=\"2300\">\n      <print new-system=\"yes\"><staff-layout number=\"2\"><staff-distance>134</staff-distance></staff-layout></print>\n      <note><rest/><duration>384</duration></note>\n    </measure>\n    <measure number=\"\
4\"><note><rest/><duration>384</duration></note></measure>\n  </part>\n</score-partwise>";

        let result = apply_pianist_layout(xml, 1);
        assert_eq!(result.xml.matches("new-system=\"yes\"").count(), 1);
        assert!(result.xml.contains("<staff-distance>134</staff-distance>"));
        assert!(!result.xml.contains("<print new-system=\"yes\"/>"));
    }

    #[test]
    fn missing_system_break_is_inserted_once() {
        let xml = "<?xml version=\"1.0\"?>\n<score-partwise version=\"4.0\">\n  <part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list>\n  <part id=\"P1\">\n    <measure number=\"\
1\"><note><rest/><duration>384</duration></note></measure>\n    <measure number=\"\
2\"><note><rest/><duration>384</duration></note></measure>\n    <measure number=\"\
3\"><note><rest/><duration>384</duration></note></measure>\n    <measure number=\"\
4\"><note><rest/><duration>384</duration></note></measure>\n  </part>\n</score-partwise>";

        let result = apply_pianist_layout(xml, 1);
        assert_eq!(result.xml.matches("new-system=\"yes\"").count(), 0);
        assert!(result
            .horizontal_layout_seed
            .slots
            .get(2)
            .is_some_and(|slot| slot.starts_new_system));
    }


}

