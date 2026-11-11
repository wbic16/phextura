//! Bibliography: entries are scrolls, citations are coordinates
//! (DESIGN.md §4).
//!
//! There is no `.bib` file, no `.bbl`, and no external program. An entry
//! is one scroll in the bibliography division, its fields written as
//! plain `key: value` lines. A citation in body text is simply the
//! entry's coordinate. Rendering order and punctuation come from a style
//! scroll, not from a citation-style programming language.

use libphext::phext::Coordinate;

/// One bibliography entry, parsed from its scroll.
#[derive(Clone, Debug)]
pub struct Entry {
    /// The coordinate of the entry's scroll — also its citation target.
    pub coordinate: Coordinate,
    /// Field lines in scroll order, e.g. `("author", "A. Grothendieck")`.
    pub fields: Vec<(String, String)>,
}

impl Entry {
    /// Look up a field by name (`author`, `title`, `year`, …).
    pub fn field(&self, name: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// Parse an entry from its scroll text at `coordinate`.
    pub fn parse(coordinate: Coordinate, scroll: &str) -> Entry {
        let _ = (coordinate, scroll);
        todo!("bibliography::Entry::parse — split field lines")
    }
}

/// Group likely-duplicate entries for an author's review.
///
/// Intended mechanism: `libphext::phext::soundex_v1` on author names as
/// a candidate generator only — a phonetic match is a hint for a human,
/// never proof that two entries are the same work.
pub fn duplicate_candidates(entries: &[Entry]) -> Vec<(Coordinate, Coordinate)> {
    let _ = entries;
    todo!("bibliography::duplicate_candidates — soundex-grouped author names")
}
