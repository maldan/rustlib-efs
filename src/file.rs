pub fn size(path: &str) -> u64 {
    match std::fs::metadata(path) {
        Ok(m) => m.len(),
        _ => 0,
    }
}

pub fn as_text(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        _ => "".to_string(),
    }
}

pub fn as_binary(path: &str) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(s) => s,
        _ => vec![],
    }
}
