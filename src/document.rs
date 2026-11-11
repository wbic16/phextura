//! The paper itself: a phext corpus plus its division layout.
//!
//! Layout (DESIGN.md §2): the paper lives at a root `Coordinate`
//! (Z = Library.Shelf.Series). Within it, the Y Collection component names
//! the division: 1 body, 2 bibliography, 3 media, 4 styles, 5 supplements,
//! 6 metadata. Body blocks are scrolls whose optional first-line tag
//! (`@theorem`, `@proof`, `@equation`, …) declares their role.

use libphext::phext::Coordinate;

/// Y-Collection values naming a phextura division.
pub mod collection {
    /// Chapters, sections, and content blocks.
    pub const BODY: usize = 1;
    /// One bibliography entry per scroll.
    pub const BIBLIOGRAPHY: usize = 2;
    /// Figures and embedded media payloads.
    pub const MEDIA: usize = 3;
    /// Style scrolls.
    pub const STYLES: usize = 4;
    /// Embedded documents: data, code, extended proofs.
    pub const SUPPLEMENTS: usize = 5;
    /// Title, authors, abstract, provenance, checksum manifest.
    pub const METADATA: usize = 6;
}

/// A phextura paper: the corpus text and the coordinate of its root.
pub struct Paper {
    /// The whole corpus, in imploded phext form (one `.phext` artifact).
    pub corpus: String,
    /// The paper's root coordinate; divisions hang below it.
    pub root: Coordinate,
}

impl Paper {
    /// Create an empty paper rooted at `root`.
    ///
    /// Intended behavior: seed the metadata subspace and an empty body.
    pub fn new(root: Coordinate) -> Paper {
        Paper {
            corpus: String::new(),
            root,
        }
    }

    /// Load a paper from imploded phext text (`libphext::phext::explode`
    /// is the intended mechanism for working with its scrolls).
    pub fn load(corpus: &str) -> Paper {
        let _ = corpus;
        todo!("document::Paper::load — parse and root-check a corpus")
    }

    /// Fetch one scroll of the paper by coordinate
    /// (`libphext::phext::fetch` is the intended mechanism).
    pub fn fetch(&self, coord: Coordinate) -> String {
        let _ = coord;
        todo!("document::Paper::fetch — resolve a coordinate in the corpus")
    }
}
