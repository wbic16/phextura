//! phextura — a document model for papers, built on phext.
//!
//! A phextura paper is a phext corpus: plain-text scrolls at deterministic
//! coordinates (see DESIGN.md). Structure is addressing, references are
//! coordinates, and rendering is a single traversal. The document layer
//! executes nothing; all computation lives in this crate, in Rust, over
//! [`libphext::phext`].
//!
//! Status: design skeleton. Types and doc comments state each module's
//! intended role; unimplemented behavior is `todo!()`, never faked.

pub mod bibliography;
pub mod document;
pub mod media;
pub mod reference;
pub mod render;
pub mod style;
