//! Styles: addressed scrolls instead of a global macro namespace
//! (DESIGN.md §6).
//!
//! A style scroll declares presentation facts as plain lines (typefaces,
//! sizes, spacing, page geometry, theorem numbering, citation format).
//! The paper's metadata declares which style coordinates govern which
//! subspaces, so composition is explicit, there is no load order, and two
//! styles defining the same concept cannot collide — coordinates do not
//! collide.

use libphext::phext::Coordinate;

/// A style sheet: the scroll that declares it, and its declarations.
#[derive(Clone, Debug)]
pub struct StyleSheet {
    /// Coordinate of the style scroll. With the scroll's checksum, this
    /// pins the exact style a paper was rendered with.
    pub coordinate: Coordinate,
    /// Declaration lines in scroll order, e.g. `("theorem-numbering", "within-chapter")`.
    pub declarations: Vec<(String, String)>,
}

/// Which style governs a subspace of a paper.
#[derive(Clone, Debug)]
pub struct StyleAssignment {
    /// Root coordinate of the subspace the style applies to.
    pub subspace: Coordinate,
    /// The governing style scroll.
    pub style: Coordinate,
}

/// Parse a style scroll into a [`StyleSheet`].
pub fn parse(coordinate: Coordinate, scroll: &str) -> StyleSheet {
    let _ = (coordinate, scroll);
    todo!("style::parse — split declaration lines")
}
