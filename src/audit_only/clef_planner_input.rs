//! Exact byte observation at the existing clef planner entry.
//! Compression and stderr failures have no musical effect or write authority.

use std::fmt::Write as _;
use std::io::Write as _;

fn frame(xml: &str, input_fnv64: u64) -> std::io::Result<String> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
    encoder.write_all(xml.as_bytes())?;
    let compressed = encoder.finish()?;
    let hash = crate::sha256::hex(xml.as_bytes());
    let mut out = format!(
        "ClefPlannerInputBegin: Schema=1; InputFnv64={input_fnv64:016x}; InputBytes={}; InputSha256={hash}; Encoding=zlib_hex; CompressedBytes={}; SourceRole=actual_function_entry; WriterCommit=not_observed; Authority=none\n",
        xml.len(), compressed.len(),
    );
    for chunk in compressed.chunks(64) {
        out.push_str("ClefPlannerInputHex: ");
        for byte in chunk {
            let _ = write!(out, "{byte:02x}");
        }
        out.push('\n');
    }
    let _ = writeln!(out, "ClefPlannerInputEnd: InputSha256={hash}");
    Ok(out)
}

pub(crate) fn emit(xml: &str, input_fnv64: u64) {
    let out = match frame(xml, input_fnv64) {
        Ok(out) => out,
        Err(_) => format!(
            "ClefPlannerInputUnavailable: InputFnv64={input_fnv64:016x}; InputBytes={}; Reason=encoding_failed; Authority=none\n",
            xml.len(),
        ),
    };
    // One lock covers the complete frame. I/O cannot veto the original planner.
    let _ = std::io::stderr().lock().write_all(out.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read as _;

    fn readback(log: &str) -> Vec<u8> {
        let hex = log.lines().filter_map(|line| line.strip_prefix("ClefPlannerInputHex: ")).collect::<String>();
        let compressed = hex.as_bytes().chunks_exact(2).map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
        }).collect::<Vec<_>>();
        let mut bytes = Vec::new();
        flate2::read::ZlibDecoder::new(&compressed[..]).read_to_end(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn compressed_utf8_crlf_empty_and_long_bytes_roundtrip_without_mutation() {
        for xml in [String::new(), "<score>\r\n<!-- ÄÖ𝄞 Takt m42 -->\n</score>\r\n".into(), "<note>ö𝄞</note>\r\n".repeat(10000)] {
            let before = xml.clone();
            let log = frame(&xml, 0xcbf29ce484222325).unwrap();
            assert_eq!(readback(&log), xml.as_bytes());
            assert_eq!(xml, before);
            assert!(log.contains(&format!("InputSha256={}", crate::sha256::hex(xml.as_bytes()))));
            assert!(log.contains(&format!("InputBytes={};", xml.len())));
            assert!(log.lines().filter_map(|line| line.strip_prefix("ClefPlannerInputHex: ")).all(|row| row.len() <= 128 && row.len() % 2 == 0));
        }
    }

    #[test]
    fn equal_calls_remain_separate_and_carry_the_supplied_binding() {
        let first = frame("<score/>", 0x123456789abcdef0).unwrap();
        let second = frame("<score/>", 0x123456789abcdef0).unwrap();
        assert_eq!(first, second);
        let log = format!("{first}{second}");
        assert_eq!(log.matches("ClefPlannerInputBegin: ").count(), 2);
        assert_eq!(log.matches("ClefPlannerInputEnd: ").count(), 2);
        assert!(first.contains("InputFnv64=123456789abcdef0;"));
        assert!(first.contains("SourceRole=actual_function_entry; WriterCommit=not_observed; Authority=none"));
    }

    #[test]
    fn existing_report_rewrites_preserve_compressed_frames() {
        let xml = "<score>\r\n<!-- Takt 17; m42; ÄÖ𝄞 -->\n</score>\r\n";
        let original = frame(xml, 0).unwrap();
        let log = crate::report_sanitizer::sanitize_report_hardcoded_measure_artifacts(original.clone());
        let log = crate::report_sanitizer::sanitize_report_hygiene_polish(log);
        let log = crate::report_sanitizer::sanitize_behind_bars_active_authority_language_v1284(log);
        assert_eq!(readback(&log), xml.as_bytes());
        assert_eq!(log, original);
    }
}
