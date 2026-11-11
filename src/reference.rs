//! References: a reference *is* a coordinate (DESIGN.md §3).
//!
//! There is no label table and no second pass. A reference written in a
//! scroll is parsed into a typed [`Reference`]; validation is a single
//! pass that fetches every referenced coordinate and reports the ones
//! that do not resolve, together with the coordinate of the scroll that
//! contains the faulty reference. Display numbers ("Theorem 3.2") are a
//! render-time overlay and are never stored.

use libphext::phext::{Coordinate, PhextParseError};

/// A typed cross-reference: the coordinate of the scroll it points at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reference {
    /// The target scroll's coordinate.
    pub target: Coordinate,
}

impl Reference {
    /// Parse a reference from its written form in prose.
    ///
    /// Delegates to `Coordinate`'s `TryFrom` impl, so malformed addresses
    /// surface as [`PhextParseError`] — a lint error, not a panic (the
    /// 0.5.0 behavior; 0.4.1 panicked here).
    pub fn parse(text: &str) -> Result<Reference, PhextParseError> {
        Ok(Reference {
            target: Coordinate::try_from(text)?,
        })
    }
}

/// One dangling reference found by validation: where it is written, and
/// the coordinate it fails to reach.
#[derive(Clone, Debug)]
pub struct DanglingReference {
    /// Coordinate of the scroll containing the bad reference.
    pub source: Coordinate,
    /// The reference that does not resolve.
    pub reference: Reference,
}

/// Validate every reference in a corpus in one pass.
///
/// Intended mechanism: build the reference graph with
/// `libphext::phext::navmap`, then `fetch` each target.
pub fn validate(corpus: &str) -> Vec<DanglingReference> {
    let _ = corpus;
    todo!("reference::validate — navmap targets minus fetch hits")
}
