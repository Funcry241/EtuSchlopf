//! Failure-only observation of original owner bytes. No receipt, proof,
//! adoption, musical decision or recovery is supplied by this module.
use std::{cell::RefCell, io::Write, marker::PhantomData, rc::Rc, sync::Arc};

#[derive(Clone)]
struct OriginalBytes {
    label: &'static str,
    before: Arc<str>,
    after: Option<Arc<str>>,
}

thread_local! {
    static ORIGINALS: RefCell<Vec<OriginalBytes>> = const { RefCell::new(Vec::new()) };
}

/// A lexical, thread-bound observation; dropping it restores its predecessor.
pub(crate) struct OriginGuard {
    previous_len: Option<usize>,
    _same_thread: PhantomData<Rc<()>>,
}

pub(crate) fn enter(label: &'static str, before: &str, after: Option<&str>) -> OriginGuard {
    let original = OriginalBytes { label, before: Arc::from(before), after: after.map(Arc::from) };
    let previous_len = ORIGINALS.with(|originals| originals.try_borrow_mut().ok().map(|mut originals| {
        let previous_len = originals.len();
        originals.push(original);
        previous_len
    }));
    OriginGuard { previous_len, _same_thread: PhantomData }
}

impl Drop for OriginGuard {
    fn drop(&mut self) {
        if let Some(previous_len) = self.previous_len {
            ORIGINALS.with(|originals| {
                if let Ok(mut originals) = originals.try_borrow_mut() {
                    originals.truncate(previous_len);
                }
            });
        }
    }
}

fn current() -> Option<OriginalBytes> {
    ORIGINALS.with(|originals| originals.try_borrow().ok()?.last().cloned())
}

/// The original hook remains authoritative and is called first. Observation
/// neither catches nor replaces the panic; unwinding and exit remain original.
pub(crate) fn install() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        original_hook(info);
        let original = current();
        let backtrace = std::backtrace::Backtrace::force_capture();
        let mut stderr = std::io::stderr().lock();
        let _ = writeln!(stderr, "PanicOriginBacktrace: thread={:?}\n{backtrace}", std::thread::current().name());
        if let Some(original) = original {
            let _ = write_original(&mut stderr, &original);
        }
    }));
}

/// Existing receipt-coverage failure only. The documents are not parsed,
/// normalized, corrected or substituted before publication.
pub(crate) fn emit_documents(label: &'static str, before: &str, after: Option<&str>) {
    let original = OriginalBytes { label, before: Arc::from(before), after: after.map(Arc::from) };
    let _ = write_original(&mut std::io::stderr().lock(), &original);
}

fn write_original(out: &mut impl Write, original: &OriginalBytes) -> std::io::Result<()> {
    writeln!(out, "OwnerOriginalBegin: label={:?}; observation_only=true", original.label)?;
    write_bytes(out, "pre", &original.before)?;
    if let Some(after) = &original.after { write_bytes(out, "post", after)?; }
    writeln!(out, "OwnerOriginalEnd: label={:?}", original.label)
}

fn write_bytes(out: &mut impl Write, field: &str, bytes: &str) -> std::io::Result<()> {
    writeln!(out, "OwnerOriginalField: field={field}; bytes={}; sha256={}", bytes.len(), crate::sha256::hex(bytes.as_bytes()))?;
    for (index, chunk) in bytes.as_bytes().chunks(64).enumerate() {
        write!(out, "OwnerOriginalChunk: field={field}; offset={}; hex=", index * 64)?;
        for byte in chunk { write!(out, "{byte:02x}")?; }
        writeln!(out)?;
    }
    Ok(())
}

/// Bounded byte comparison for an already refused continuation. No identity or
/// acceptance is derived; callers keep their original refusal predicates.
pub(crate) fn difference_evidence(expected: &str, observed: &str) -> String {
    let first = expected.as_bytes().iter().zip(observed.as_bytes())
        .position(|(a, b)| a != b)
        .or_else(|| (expected.len() != observed.len()).then_some(expected.len().min(observed.len())));
    let center = first.unwrap_or(expected.len().min(observed.len()));
    let start = center.saturating_sub(16);
    let context = |text: &str| text.as_bytes()[start.min(text.len())..center.saturating_add(16).min(text.len())]
        .iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    format!("observation_only=true; expected_bytes={}; observed_bytes={}; expected_sha256={}; observed_sha256={}; first_difference_byte={first:?}; context_start_byte={start}; expected_context_hex={}; observed_context_hex={}",
        expected.len(), observed.len(), crate::sha256::hex(expected.as_bytes()),
        crate::sha256::hex(observed.as_bytes()), context(expected), context(observed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_bytes_roundtrip_without_normalization_and_with_explicit_integrity() {
        let before = "<score>ä\r\n<!-- probe -->\t</score>";
        let after = format!("{before}{}", " ".repeat(130));
        let original = OriginalBytes { label: "synthetic", before: Arc::from(before), after: Some(Arc::from(after.as_str())) };
        let mut output = Vec::new(); write_original(&mut output, &original).unwrap();
        let output = String::from_utf8(output).unwrap();
        for (field, expected) in [("pre", before), ("post", after.as_str())] {
            let mut bytes = Vec::new();
            for line in output.lines().filter(|line| line.starts_with(&format!("OwnerOriginalChunk: field={field};"))) {
                let hex = line.split_once("hex=").unwrap().1;
                bytes.extend((0..hex.len()).step_by(2).map(|at| u8::from_str_radix(&hex[at..at+2], 16).unwrap()));
            }
            assert_eq!(bytes, expected.as_bytes());
            assert!(output.contains(&format!("sha256={}", crate::sha256::hex(&bytes))));
        }
    }

    #[test]
    fn lexical_owner_observation_is_thread_local_and_restored_during_unwind() {
        assert!(current().is_none());
        let _outer = enter("outer", "<outer/>", None);
        let refusal = std::panic::catch_unwind(|| {
            let _inner = enter("inner", "<before/>", Some("<after/>"));
            assert_eq!(current().unwrap().label, "inner");
            std::thread::spawn(|| assert!(current().is_none())).join().unwrap();
            std::panic::panic_any("synthetic original failure");
        });
        assert_eq!(refusal.unwrap_err().downcast_ref::<&str>(), Some(&"synthetic original failure"));
        assert_eq!(current().unwrap().label, "outer");
    }
    #[test]
    fn byte_difference_evidence_is_bounded_and_preserves_unicode_and_truncation() {
        for (expected, observed, at) in [("ä\r\nabc", "ä\r\naxc", 5), ("abc", "ab", 2), ("", "x", 0)] {
            let evidence = difference_evidence(expected, observed);
            assert!(evidence.contains(&format!("first_difference_byte=Some({at});")));
            assert!(evidence.contains(&format!("expected_sha256={}", crate::sha256::hex(expected.as_bytes()))));
            assert!(evidence.contains(&format!("observed_sha256={}", crate::sha256::hex(observed.as_bytes()))));
        }
        assert!(difference_evidence("same", "same").contains("first_difference_byte=None;"));
        let large = "z".repeat(4096);
        let evidence = difference_evidence(&large, &format!("{large}x"));
        assert!(evidence.len() < 700);
        assert!(evidence.contains("expected_context_hex=7a"));
        assert!(evidence.contains("observation_only=true"));
    }

}
