//! Shared rendering primitives for the textual result report.
//!
//! These helpers remain crate-visible because both the pipeline and focused domain modules
//! render into the same report. Keeping them here prevents formatting machinery from becoming
//! part of the binary entrypoint or the pipeline ownership boundary.

pub fn push_static_ent_line_v765(out: &mut String, suffix: &str) {
    out.push_static_named_report_line_v766("Entscheidung", suffix);
}
pub fn push_static_dec_line_v765(out: &mut String, suffix: &str) {
    out.push_static_named_report_line_v766("Decision", suffix);
}
pub fn push_static_write_policy_line_v765(out: &mut String, suffix: &str) {
    out.push_static_named_report_line_v766("WritePolicy", suffix);
}
pub fn push_static_named_policy_line_v765(out: &mut String, name: &str, suffix: &str) {
    out.push_str(name);
    out.push_static_named_report_line_v766("Policy", suffix);
}
pub fn push_static_section_header_v793(out: &mut String, title: &str, underline: &str) {
    out.push_str(title);
    out.push('\n');
    out.push_str(underline);
    out.push('\n');
}
pub fn push_static_ledger_line_v804(out: &mut String, prefix: &str, suffix: &str) {
    out.push_str(prefix);
    out.push_static_named_report_line_v766("Ledger", suffix);
}
pub fn push_static_ledger_header_v804(out: &mut String, title: &str, columns: &str) {
    out.push_str(title);
    out.push_str("Ledger: ");
    out.push_str(columns);
    out.push('\n');
}
pub trait StaticNamedReportLineV766 {
    fn push_format_args(&mut self, args: std::fmt::Arguments<'_>);
    fn push_static_named_report_line_v766(&mut self, name: &str, suffix: &str);
    fn push_static_section_header_v795(&mut self, title: &str, underline: &str);
}
impl StaticNamedReportLineV766 for String {
    fn push_format_args(&mut self, args: std::fmt::Arguments<'_>) {
        std::fmt::Write::write_fmt(self, args).expect("writing to String cannot fail");
    }
    fn push_static_named_report_line_v766(&mut self, name: &str, suffix: &str) {
        self.push_str(name);
        self.push(':');
        self.push_str(suffix);
        self.push('\n');
    }
    fn push_static_section_header_v795(&mut self, title: &str, underline: &str) {
        push_static_section_header_v793(self, title, underline);
    }
}

#[cfg(test)]
mod tests {
    use super::StaticNamedReportLineV766;

    #[test]
    fn push_format_args_matches_allocating_reference_bytes() {
        let mut actual = String::from("prefix|");
        actual.push_format_args(format_args!("value {} | ratio {:.2}\n", 7, 1.25));
        let expected = format!("prefix|value {} | ratio {:.2}\n", 7, 1.25);
        assert_eq!(actual.as_bytes(), expected.as_bytes());
    }
}
