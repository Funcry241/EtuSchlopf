// Shared policy-free Rust source facts for kodex scanners.
// This compile unit is deliberately separate from masking/FNV so the take bootstrap
// can depend on the narrower base primitive set without compiling unused source-fact mechanics.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn collect_rust_files(root: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, out)?;
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            out.push(path);
        }
    }
    Ok(())
}

pub fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

pub fn line_starts(text: &str) -> Vec<usize> {
    let mut starts = vec![0usize];
    for (index, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(index + 1);
        }
    }
    starts
}

pub fn line_number_at(offset: usize, starts: &[usize]) -> usize {
    match starts.binary_search(&offset) {
        Ok(index) => index + 1,
        Err(index) => index.max(1),
    }
}

pub fn token_boundary_before(bytes: &[u8], index: usize) -> bool {
    index == 0 || !(bytes[index - 1].is_ascii_alphanumeric() || bytes[index - 1] == b'_')
}

pub fn token_boundary_after(bytes: &[u8], index: usize) -> bool {
    index >= bytes.len() || !(bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
}
