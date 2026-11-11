//! Rendering: a single traversal of the corpus (DESIGN.md §7).
//!
//! Load (`explode`) → validate (see [`crate::reference`]) → traverse the
//! body subspace in coordinate order (`get_subspace_coordinates`,
//! `next_scroll`), resolving each reference by `fetch` as it is met →
//! present under the governing style scrolls. There is no second pass
//! and nothing to rerun: references resolve by construction.
//!
//! Backends: HTML first — a coordinate becomes a link via
//! `Coordinate::to_urlencoded`. A print backend may implement Knuth's
//! line-breaking mathematics behind this same traversal; that algorithm
//! is kept by choice. It consumes addressed blocks now, not a stream.

use crate::document::Paper;

/// Render a paper to HTML.
pub fn render_html(paper: &Paper) -> String {
    let _ = paper;
    todo!("render::render_html — traverse body, resolve references, emit HTML")
}

/// Render a paper to a print-oriented backend (future work; the layout
/// engine is deliberately unspecified in DESIGN.md §10).
pub fn render_print(paper: &Paper) -> Vec<u8> {
    let _ = paper;
    todo!("render::render_print — layout engine not yet designed")
}
