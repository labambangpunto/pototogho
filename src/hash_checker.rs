#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashComparison {
    Match,
    Mismatch,
    Empty,
}

/// Membandingkan dua string hash secara aman (case-insensitive & trim whitespace).
pub fn compare_hashes(first: &str, second: &str) -> HashComparison {
    let first_clean = first.trim();
    let second_clean = second.trim();

    if first_clean.is_empty() || second_clean.is_empty() {
        return HashComparison::Empty;
    }

    if first_clean.eq_ignore_ascii_case(second_clean) {
        HashComparison::Match
    } else {
        HashComparison::Mismatch
    }
}
