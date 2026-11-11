//! Embedded media and foreign documents (DESIGN.md §5).
//!
//! libphext scrolls are text; there is no binary payload type, and this
//! module does not pretend otherwise. Text-native media (SVG, CSV, code)
//! embed as literal scroll content. Binary payloads ride as encoded text:
//! a header scroll (this module's [`MediaHeader`]) plus payload scrolls
//! spanning a coordinate [`Range`]. The header's checksum
//! (`libphext::phext::checksum`, xxh3-128) is verified before a payload is
//! materialized at render time, so a damaged or missing payload fails
//! loudly instead of rendering as a silent gap.

use libphext::phext::Range;

/// How a payload's bytes are carried as scroll text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayloadEncoding {
    /// The payload is text already (SVG, CSV, source code): stored raw.
    Literal,
    /// Binary payload carried as base64 text (~33% size cost, stated in
    /// DESIGN.md §5; containment and addressability are what it buys).
    Base64,
}

/// The header of an embedded media item: one scroll in the media
/// division, preceding the payload scrolls it describes.
#[derive(Clone, Debug)]
pub struct MediaHeader {
    /// MIME content type, e.g. `image/png` or `image/svg+xml`.
    pub content_type: String,
    /// How the payload scrolls encode the content.
    pub encoding: PayloadEncoding,
    /// The name the payload had outside the corpus, for provenance.
    pub original_filename: String,
    /// xxh3-128 hex checksum of the *decoded* payload bytes' text form,
    /// as produced by `libphext::phext::checksum`.
    pub checksum: String,
    /// The coordinate range the payload scrolls occupy.
    pub payload: Range,
}

/// Verify a payload against its header checksum before rendering.
pub fn verify(header: &MediaHeader, payload_text: &str) -> bool {
    let _ = (header, payload_text);
    todo!("media::verify — checksum(payload) == header.checksum")
}
