//! Read-only capture at the existing register-planner entry, before parsing.
//! No rhythmic-group ownership or musical decision is inferred from these bytes.

use std::fmt::Write as _;
use std::sync::Arc;

pub(crate) const SECTION: &str = "REGISTER-PLANNER-INPUT-READBACK";
pub(crate) const SECTION_RULE: &str = "-------------------------------";

#[derive(Debug, Clone)]
pub(crate) struct InputSnapshot {
    xml: Arc<str>,
}

impl InputSnapshot {
    pub(crate) fn capture(xml: &str) -> Self {
        Self { xml: Arc::from(xml) }
    }

    pub(crate) fn append_report(&self, out: &mut String) {
        let bytes = self.xml.as_bytes();
        out.push_str("InputCapture: OBSERVED; Schema 1; Phase register_planner_before_k00.\n");
        out.push_str("CanonicalGestureOwnerBinding: NOT_EVALUATED; MusicalAuthority none.\n");
        let _ = writeln!(out, "InputBytes: {}.", bytes.len());
        let _ = writeln!(out, "InputSha256: {}.", crate::sha256::hex(bytes));
        // Hex survives the report's vocabulary rewrites, including score titles.
        // Fixed-size rows keep the final-report document parser's lines bounded.
        out.push_str("InputHexBegin\n");
        for chunk in bytes.chunks(64) {
            out.push_str("InputHex: ");
            for byte in chunk {
                let _ = write!(out, "{byte:02x}");
            }
            out.push('\n');
        }
        out.push_str("InputHexEnd\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_rewrites_preserve_exact_utf8_input_and_hash() {
        let xml = "<score-partwise>\r\n<!-- Takt 17; m42; ÄÖ𝄞 -->\n</score-partwise>\n";
        let mut report = String::new();
        InputSnapshot::capture(xml).append_report(&mut report);
        let report = crate::report_sanitizer::sanitize_report_hardcoded_measure_artifacts(report);
        let report = crate::report_sanitizer::sanitize_report_hygiene_polish(report);
        let report = crate::report_sanitizer::sanitize_behind_bars_active_authority_language_v1284(report);
        let hex = report.lines().filter_map(|line| line.strip_prefix("InputHex: ")).collect::<String>();
        let decoded = hex.as_bytes().chunks_exact(2).map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII hex"), 16).expect("hex byte")
        }).collect::<Vec<_>>();
        assert_eq!(decoded, xml.as_bytes());
        assert!(report.contains(&format!("InputBytes: {}.", decoded.len())));
        assert!(report.contains(&format!("InputSha256: {}.", crate::sha256::hex(&decoded))));
        assert!(report.contains("CanonicalGestureOwnerBinding: NOT_EVALUATED"));
    }

    #[test]
    fn long_and_empty_inputs_have_complete_bounded_hex_rows() {
        for xml in [String::new(), "ö".repeat(513)] {
            let mut out = String::new();
            InputSnapshot::capture(&xml).append_report(&mut out);
            let rows = out.lines().filter_map(|line| line.strip_prefix("InputHex: ")).collect::<Vec<_>>();
            assert!(rows.iter().all(|row| row.len() <= 128 && row.len() % 2 == 0));
            assert_eq!(rows.concat().len(), xml.len() * 2);
            assert_eq!(out.matches("InputHexBegin\n").count(), 1);
            assert_eq!(out.matches("InputHexEnd\n").count(), 1);
        }
    }

    #[test]
    fn current_report_projection_keeps_one_complete_capture() {
        let snapshot = InputSnapshot::capture("<score><!-- ö --></score>\r\n");
        let mut report = crate::final_report::DraftReport::new(String::new()).compact(str::to_owned);
        for _ in 0..2 {
            report.project_owned_section("register_planner_input", SECTION, SECTION_RULE, |out| {
                snapshot.append_report(out);
            }).expect("declared current report section");
        }
        report.rewrite_document(crate::report_sanitizer::sanitize_report_hardcoded_measure_artifacts);
        report.rewrite_document(crate::report_sanitizer::sanitize_report_hygiene_polish);
        let rendered = report.render_snapshot();
        assert_eq!(rendered.matches(SECTION).count(), 1);
        assert_eq!(rendered.matches("InputHexBegin\n").count(), 1);
        assert_eq!(rendered.matches("InputHexEnd\n").count(), 1);
        let mut expected = String::new();
        snapshot.append_report(&mut expected);
        assert!(rendered.contains(&expected));
    }
}
