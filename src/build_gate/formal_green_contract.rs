//! Shared formal release-summary grammar.
//!
//! Report production and public-artifact commit validation use this one small
//! contract. Hints remain nonblocking; blockers and warnings must both be zero.

pub const FORMALLY_GREEN_RESULT_STATUS: &str = "OK";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormalSummaryCounts {
    pub blockers: u64,
    pub warnings: u64,
    pub hints: u64,
}

impl FormalSummaryCounts {
    pub const fn is_formally_green(self) -> bool {
        self.blockers == 0 && self.warnings == 0
    }
}

pub fn format_result_line(status: &str) -> String {
    format!("Ergebnis: {status}")
}

pub fn format_summary_line(blockers: usize, warnings: usize, hints: usize) -> String {
    format!("Blocker: {blockers} | Warnungen: {warnings} | Hinweise: {hints}")
}

pub fn parse_summary_line(line: &str) -> Option<FormalSummaryCounts> {
    let rest = line.strip_prefix("Blocker: ")?;
    let (blockers, rest) = rest.split_once(" | Warnungen: ")?;
    let (warnings, hints) = rest.split_once(" | Hinweise: ")?;
    Some(FormalSummaryCounts {
        blockers: blockers.parse::<u64>().ok()?,
        warnings: warnings.parse::<u64>().ok()?,
        hints: hints.parse::<u64>().ok()?,
    })
}

pub fn report_header_is_formally_green(report: &str) -> bool {
    let mut result_status = None;
    let mut summary = None;
    for line in report.lines().take(16) {
        if let Some(status) = line.strip_prefix("Ergebnis: ")
            && result_status.replace(status).is_some()
        {
            return false;
        }
        if line.starts_with("Blocker: ") {
            let Some(counts) = parse_summary_line(line) else {
                return false;
            };
            if summary.replace(counts).is_some() {
                return false;
            }
        }
    }
    result_status == Some(FORMALLY_GREEN_RESULT_STATUS)
        && summary.is_some_and(FormalSummaryCounts::is_formally_green)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formal_green_contract_accepts_hints_but_not_warnings_or_blockers() {
        assert!(report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: 0\n"
        ));
        assert!(report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: 1\n"
        ));
        assert!(!report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: WARNUNG\nBlocker: 0 | Warnungen: 1 | Hinweise: 0\n"
        ));
        assert!(!report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: BLOCKIERT\nBlocker: 1 | Warnungen: 0 | Hinweise: 0\n"
        ));
    }

    #[test]
    fn formal_green_contract_rejects_malformed_counts() {
        assert!(!report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: -1\n"
        ));
        assert!(!report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: eins\n"
        ));
    }

    #[test]
    fn formal_green_contract_rejects_duplicate_or_conflicting_header_rows() {
        assert!(!report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nErgebnis: WARNUNG\nBlocker: 0 | Warnungen: 0 | Hinweise: 0\n"
        ));
        assert!(!report_header_is_formally_green(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: 0\nBlocker: 0 | Warnungen: 1 | Hinweise: 0\n"
        ));
    }
}
