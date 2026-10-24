// Shared policy-free source facts for Rust source kodex scanners.
// Domain owners retain their own findings, policies, persistence, and failure semantics.

pub const FNV1A64_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV1A64_PRIME: u64 = 0x0100_0000_01b3;

pub const fn fnv1a64(bytes: &[u8]) -> u64 {
    fnv1a64_update(FNV1A64_OFFSET_BASIS, bytes)
}

pub const fn fnv1a64_update(mut hash: u64, bytes: &[u8]) -> u64 {
    let mut index = 0usize;
    while index < bytes.len() {
        hash ^= bytes[index] as u64;
        hash = hash.wrapping_mul(FNV1A64_PRIME);
        index += 1;
    }
    hash
}

pub fn mask_non_code(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut masked = bytes.to_vec();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes.get(index..index + 2) == Some(b"//") {
            let end = bytes[index + 2..]
                .iter()
                .position(|byte| *byte == b'\n')
                .map_or(bytes.len(), |offset| index + 2 + offset);
            blank_non_code_range(&mut masked, index, end);
            index = end;
            continue;
        }
        if bytes.get(index..index + 2) == Some(b"/*") {
            let mut cursor = index + 2;
            let mut depth = 1usize;
            while cursor < bytes.len() && depth > 0 {
                if bytes.get(cursor..cursor + 2) == Some(b"/*") {
                    depth += 1;
                    cursor += 2;
                } else if bytes.get(cursor..cursor + 2) == Some(b"*/") {
                    depth -= 1;
                    cursor += 2;
                } else {
                    cursor += 1;
                }
            }
            blank_non_code_range(&mut masked, index, cursor);
            index = cursor;
            continue;
        }
        if let Some((content_start, hash_count)) = raw_string_open(bytes, index) {
            let mut cursor = content_start;
            while cursor < bytes.len() {
                if bytes[cursor] == b'"'
                    && bytes
                        .get(cursor + 1..cursor + 1 + hash_count)
                        .is_some_and(|suffix| suffix.iter().all(|byte| *byte == b'#'))
                {
                    cursor += 1 + hash_count;
                    break;
                }
                cursor += 1;
            }
            blank_non_code_range(&mut masked, index, cursor);
            index = cursor;
            continue;
        }
        if let Some(quote_index) = string_quote_index(bytes, index) {
            let end = quoted_literal_end(bytes, quote_index, b'"').unwrap_or(bytes.len());
            blank_non_code_range(&mut masked, index, end);
            index = end;
            continue;
        }
        if let Some((quote_index, can_be_lifetime)) = character_quote(bytes, index)
            && (!can_be_lifetime || !apostrophe_starts_lifetime(bytes, quote_index))
        {
            let end = quoted_literal_end(bytes, quote_index, b'\'').unwrap_or(bytes.len());
            blank_non_code_range(&mut masked, index, end);
            index = end;
            continue;
        }
        index += 1;
    }
    String::from_utf8(masked).expect("Rust source remains UTF-8 after lexical blanking")
}

fn blank_non_code_range(bytes: &mut [u8], start: usize, end: usize) {
    let end = end.min(bytes.len());
    for byte in &mut bytes[start..end] {
        if !matches!(*byte, b'\n' | b'\r') {
            *byte = b' ';
        }
    }
}

fn raw_string_open(bytes: &[u8], index: usize) -> Option<(usize, usize)> {
    let mut cursor = index;
    if matches!(bytes.get(cursor), Some(&b'b' | &b'c')) {
        if bytes.get(cursor + 1) != Some(&b'r') {
            return None;
        }
        cursor += 1;
    }
    if bytes.get(cursor) != Some(&b'r') {
        return None;
    }
    cursor += 1;
    let hash_start = cursor;
    while bytes.get(cursor) == Some(&b'#') {
        cursor += 1;
    }
    let hash_count = cursor - hash_start;
    (bytes.get(cursor) == Some(&b'"')).then_some((cursor + 1, hash_count))
}

fn string_quote_index(bytes: &[u8], index: usize) -> Option<usize> {
    if bytes.get(index) == Some(&b'"') {
        Some(index)
    } else if matches!(bytes.get(index), Some(&b'b' | &b'c'))
        && bytes.get(index + 1) == Some(&b'"')
    {
        Some(index + 1)
    } else {
        None
    }
}

fn character_quote(bytes: &[u8], index: usize) -> Option<(usize, bool)> {
    if bytes.get(index) == Some(&b'\'') {
        Some((index, true))
    } else if bytes.get(index) == Some(&b'b') && bytes.get(index + 1) == Some(&b'\'') {
        Some((index + 1, false))
    } else {
        None
    }
}

fn apostrophe_starts_lifetime(bytes: &[u8], quote_index: usize) -> bool {
    let Ok(tail) = std::str::from_utf8(&bytes[quote_index + 1..]) else {
        return false;
    };
    let mut identifier_bytes = 0usize;
    for (offset, ch) in tail.char_indices() {
        let accepted = if offset == 0 {
            is_identifier_start(ch)
        } else {
            is_identifier_continue(ch)
        };
        if !accepted {
            break;
        }
        identifier_bytes = offset + ch.len_utf8();
    }
    identifier_bytes > 0
        && bytes.get(quote_index + 1 + identifier_bytes) != Some(&b'\'')
}

fn quoted_literal_end(bytes: &[u8], quote_index: usize, quote: u8) -> Option<usize> {
    let mut cursor = quote_index + 1;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'\\' => cursor = (cursor + 2).min(bytes.len()),
            byte if byte == quote => return Some(cursor + 1),
            b'\n' | b'\r' if quote == b'\'' => return None,
            _ => cursor += 1,
        }
    }
    None
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}
