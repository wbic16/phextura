# phextura — design

Phextura reformulates the LaTeX document model in terms of phext. It keeps
what was mathematics and discards what was 1978. This document states the
model, maps every capability of libphext-rs (0.5.0) to a named role, and
contrasts the result with expl3/l3kernel, the cleanest implementation of
LaTeX3 (verdict and receipts: the project's research notes).

## 1. The premise

A LaTeX document is a one-dimensional character stream that a macro expander
executes. Every structural feature — sections, references, bibliography,
figures — is simulated on top of that stream with strings, counters, side
files, and repeated passes (the full derivation is the project's warts
analysis: 27 warts, five of them load-bearing).

A phextura document is a **phext corpus**: a set of text scrolls at
deterministic addresses in a nine-dimensional coordinate lattice
(Library.Shelf.Series / Collection.Volume.Book / Chapter.Section.Scroll).
Structure is not simulated; it is the addressing. A paper is data. Programs
that read, validate, and render it are ordinary Rust over libphext, and they
read the paper without executing it.

Two properties are the point of the design:

- **Extremely fast to write papers in.** No preamble ritual, no escaping
  regime, no compile-to-see-structure loop, no rerun-until-the-labels-settle.
  You write prose in scrolls, you address the parts, you cite by address.
- **Extremely readable for humans and machines.** A scroll is plain text a
  person can read raw. The same scroll is trivially machine-readable,
  because its identity and role are its coordinate and a one-word tag, not
  the side effects of executing it.

## 2. The corpus layout

A standalone paper lives at Z = 1.1.1 (Library.Shelf.Series). In a merged
proceedings, each paper takes its own Series under a shared Library.Shelf;
composition is libphext `merge` (§6).

Within the paper, the Y coordinate's Collection component names the division:

| Collection | Division | Contents |
|---|---|---|
| 1 | Body | Chapters (Book), sections, and content blocks |
| 2 | Bibliography | One reference entry per scroll |
| 3 | Media | Figures, images, audio, video, embedded foreign files |
| 4 | Styles | Style scrolls (the phextura replacement for classes/packages) |
| 5 | Supplements | Embedded documents: data papers, code, extended proofs, toolchains (§5.1) |
| 6 | Metadata | Title, authors, abstract, provenance, checksum manifest |

Body addressing: Y.Book is the chapter. Within a chapter, X addresses
(Chapter.Section.Scroll) read as (group, section, block); the group component
is 1 in the common case and exists for books long enough to group sections.
The block — one scroll — is the smallest authored unit: a paragraph, an
equation, a theorem, a proof. Its role is declared by an optional first-line
tag, plain text:

```
@theorem
Let G be a finite group and p a prime dividing |G|...
```

Untagged blocks are prose paragraphs. Defined roles: `@theorem`, `@lemma`,
`@definition`, `@proof`, `@equation`, `@figure`, `@caption`, `@quote`,
`@code`, `@footnote`. The tag is data in the scroll, so a renderer, a linter,
and a human all read the same fact the same way.

Addresses are deterministic: given the outline, any address can be computed
by hand. Unused components sit at 1. Nothing is ever addressed by a string
the author invented and a program merely hopes matches.

## 3. References are coordinates

To refer to a theorem, an equation, a section, or a figure, the author
writes its coordinate in the text. There is no `\label`, no `\ref`, and no
second pass:

- In the phext editor, an address written in text is automatically a
  hyperlink to that scroll (verified behavior of Phext Edit). The reference
  and the link are the same object.
- Validation is one pass over the corpus: parse each written coordinate
  (libphext `Coordinate: TryFrom`, which in 0.5.0 properly returns
  `Err(PhextParseError)`), `fetch` it, and report any coordinate that does
  not resolve — with the coordinate of the scroll that contains the bad
  reference. A dangling reference is a lint error at a known location, not a
  `??` discovered in a PDF.
- Display numbers ("Theorem 3.2", "Figure 5", "§4") are a **render-time
  overlay**, computed from traversal order. They are never stored, so
  inserting a theorem renumbers nothing by hand and breaks nothing: numbers
  are views; coordinates are identity. This deletes LaTeX's entire
  label/counter/rerun complex (warts W4, W5, W27).

## 4. Bibliography without a pipeline

A bibliography entry is one scroll in Collection 2, fields as plain lines:

```
author: A. Grothendieck
title: Revêtements étales et groupe fondamental (SGA 1)
year: 1971
```

A citation in body text is the entry's coordinate. The rendered bibliography
is a traversal of the cited entries — in citation order or sorted, a
renderer choice. There is no `.bib` file, no `.bbl`, no external program, and
no citation-style programming language: a citation style is a style scroll
(Collection 4) declaring field order and punctuation. Duplicate detection
across entries uses libphext `soundex_v1` on author names as a first-pass
fuzzy aid (it is a phonetic hash; phextura uses it for candidate grouping
only, never as proof of identity).

## 5. Media and embedded documents live inside the paper

Phext is a text format, and its binary support is canonical, not bolted
on: binary content is **hoisted** — encoded as base64 text inside ordinary
scrolls. A hoisted payload is opaque by design. The format stores and
addresses it; what the bytes *mean*, and whether they are decoded at all,
is deferred to the program rendering or processing the paper. Embedding
therefore comes in two tiers:

- **Text-native media embed literally.** SVG figures, vector descriptions,
  CSV/TSV data, source code, and any text format are scroll content as-is,
  in Collection 3 (media) or Collection 5 (supplements). A whole phextura
  paper embeds as a supplement with no encoding at all — it is already text
  in the same lattice.
- **Binary content is hoisted.** A PNG, a video, an audio clip, or a
  foreign document (e.g. a PDF) is stored as a header scroll plus payload
  scrolls: the header declares content type, encoding (`base64`),
  original filename, and the payload's libphext `checksum` (xxh3-128). A
  large payload spans a coordinate `Range` of scrolls, addressed and
  replaceable as one unit (`range_replace`). Base64 encodes three bytes in
  four characters, so a hoisted payload occupies about 4/3 of its binary
  size — a neutral property of the encoding. What hoisting provides is
  containment: the paper is one artifact, nothing is referenced by
  filesystem path, nothing can be silently missing at render time (the
  header checksum fails first), and the payload stays opaque until a
  processing program chooses to decode it.

This deletes `\includegraphics`, `\input`-by-path, driver-specific
`\special`s, and the "the paper is a directory" failure mode (W18, W19).

A paper can therefore carry its own evidence: the dataset behind Figure 3,
the code that produced it, and the supplementary proof are scrolls in the
same corpus, citable by the same coordinates as the theorems.

### 5.1 Self-extracting papers

Hoisting has a consequence worth stating as its own design point: a
phextura paper can encode literally anything — including a compiler and a
copy of libphext itself. A corpus may therefore carry its own toolchain:
the renderer (or the specific version of it the paper was written
against), a compiler, and libphext, all as hoisted payload scrolls in
Collection 5, declared in the Collection 6 manifest (`manifest`) alongside
the paper's other contents.

- **Extraction is the reader-side program's decision.** Payloads are
  opaque; nothing in the format executes, extracts, or even decodes them.
  A processing program decides whether a declared toolchain payload is
  trustworthy and whether to extract and run it — the same trust decision
  a reader already makes about any renderer, made explicit. Because every
  payload is declared in the manifest with its checksum, nothing extracts
  by surprise.
- **Version rot dies.** A paper read in twenty years does not depend on a
  future toolchain still behaving like today's: it pins its own,
  bit-for-bit, inside itself.
- **The recursion closes.** A paper *about* phextura can carry phextura —
  the paper, its subject, and the machinery to read it, in one artifact.

## 6. Styles are addressed scrolls, not a global namespace

A style scroll (Collection 4) declares presentation facts as plain lines:
typeface names, sizes, spacing, page geometry, theorem numbering scheme,
citation format, float placement policy. The metadata scroll (Collection 6)
declares which style coordinates govern which subspaces of the paper.

Consequences, each the inverse of a wart:

- **Namespacing is structural.** Two style scrolls that both define
  "theorem" sit at different coordinates; coordinates do not collide, so
  styles cannot collide (W10). There is no load order (W11) — composition
  is the explicit declaration in metadata.
- **Styles are citable and versionable.** A paper can name the exact style
  scroll (coordinate + checksum) it was rendered with; a journal can publish
  its style as a corpus others merge.
- **Macros do not exist.** Repeated *content* is a scroll other scrolls
  reference; repeated *computation* is Rust in the renderer. Nothing in the
  document layer expands (W7, W8, W25).

## 7. Rendering is a traversal

Rendering a phextura paper is a single pass:

1. **Load:** `explode` the `.phext` file into scrolls.
2. **Validate:** every coordinate written in text `fetch`es; every media
   header's `checksum` matches its payload; every cited bibliography scroll
   exists. Failures are reported by coordinate.
3. **Traverse:** enumerate the body subspace (`get_subspace_coordinates`)
   and walk it in coordinate order (`next_scroll`), resolving each reference
   by `fetch` as it is met — references are resolved by construction, so
   there is nothing to rerun.
4. **Present:** apply the governing style scrolls; emit the backend format.

Backends: HTML is the native first target (a coordinate becomes a URL via
`Coordinate::to_urlencoded`). A print/PDF backend is a layout engine behind
the same traversal. **What phextura keeps from Knuth, by choice:** the
line-breaking mathematics (total-fit paragraph optimization) and
box-and-glue layout are good algorithms, not 1978 accidents; a PDF backend
may implement them. What it does not keep is the stream they were trapped
in: layout consumes addressed blocks, not an expansion.

Derived matter — table of contents, list of figures, index — is computed
from the same traversal (`textmap` for the content map, `navmap` for the
reference graph, `index` for the coordinate index). The reference graph has
a second use: `navmap` targets that fail `fetch` are exactly the dangling
references, so the graph *is* the validator.

## 8. Every libphext capability, and its role in phextura

"Leverages all functionality in libphext-rs" is a requirement. The 0.5.0
public API (79 entries: 1 module, 6 structs, 26 constants, 27 free
functions, 18 methods, 1 trait impl — enumerated in the research notes)
maps as follows:

| libphext capability | Role in phextura |
|---|---|
| `Coordinate` + `ZCoordinate`/`YCoordinate`/`XCoordinate` | The address type of everything: papers, divisions, blocks, entries, media |
| `Coordinate::new`, `default_coordinate` | Corpus construction; the paper root (1.1.1/1.1.1/1.1.1) |
| `Coordinate::validate_coordinate` | Address validation in the linter and at reference resolution |
| `Coordinate::to_string` | Printing an address in text — the written form of a reference |
| `Coordinate::to_urlencoded` | Turning a reference into a web link in the HTML backend |
| `Coordinate` `*_break` methods + the 22 break constants (`SCROLL_BREAK`…`LIBRARY_BREAK`, `_U8` twins) + `ADDRESS_*` constants | The on-disk serialization format phextura files are written in; the writer layer |
| `COORDINATE_MINIMUM` | The rule that addressing starts at 1 — no zero/negative components to special-case |
| `Coordinate: TryFrom` + `PhextParseError` | Parsing addresses written in prose into typed references; parse failures are lint errors |
| `fetch` | Reference resolution; reading any unit of the paper |
| `insert`, `replace`, `remove` | Editing blocks without renumbering anything else |
| `range_replace` + `Range` | Multi-scroll units: long proofs, chunked media payloads, replaced spans |
| `offset` | Re-basing a subspace when a chapter moves or papers merge |
| `PositionedScroll` | The renderer's work item; also the anchor type for review comments |
| `explode` / `implode` | Load a `.phext` file into scrolls / write the corpus back to one file |
| `merge` | Proceedings and anthologies: many papers, one corpus; importing a style corpus |
| `subtract` | Extracting a supplement or a single chapter as its own artifact |
| `normalize` | Canonical form before checksumming, diffing, or submission |
| `expand` / `contract` | Reserving address room in a growing draft; compacting a finished one |
| `phokenize` | The import path: tokenizing a plain-text draft into scroll units |
| `next_scroll` | Sequential traversal cursor for rendering and reading order |
| `get_subspace_coordinates` | Enumerating a division: all of a chapter, the bibliography, the media set |
| `textmap` | The content map behind table-of-contents generation |
| `navmap` | The reference graph: link checking, citation graphs, "what cites this" |
| `index` | The coordinate index backing fast fetch and the printed index |
| `manifest` | The corpus manifest in metadata: what documents/media the paper contains |
| `create_summary` | First-pass summaries of a subspace (abstract drafting aid, supplement summaries) |
| `checksum` (xxh3-128) | Integrity: media payload verification, style pinning, exact-version citation |
| `soundex_v1` | Fuzzy author-name grouping for bibliography duplicate candidates |
| `is_phext_break`, `check_for_cowbell`, `MORE_COWBELL` | Well-formedness gates: is this file a valid phext corpus at all |

## 9. Contrast with the cleanest LaTeX3 (expl3/l3kernel)

| | expl3 / l3kernel | phextura |
|---|---|---|
| Document model | 1-D character stream (unchanged from 1978) | Addressed corpus of scrolls |
| Programming layer | A disciplined library *inside* macro expansion | Ordinary Rust over libphext; the document executes nothing |
| Data structures | `seq`/`prop`/`tl` implemented as macro encodings | The corpus itself; divisions are subspaces |
| References | String labels, `.aux`, multi-pass, can dangle | Coordinates, one pass, validated by fetch |
| Bibliography | External `.bib`/`.bbl` pipeline | Entry scrolls in the corpus |
| Media | External files by path | Scrolls inside the corpus (§5) |
| Namespaces | Naming grammar by convention (`\__module_`) | Coordinate regions by construction |
| Styles/packages | Global macros, load order | Addressed style scrolls, explicit composition |
| Errors | Expansion-stack context | Coordinate + scroll that contains the fault |
| Kept from the old world | — | Knuth's line-breaking/box model, as a renderer option |

What phextura newly makes possible, that neither TeX nor LaTeX3 can express:
a paper that *contains* its data, code, media, and supplements as addressed
parts of itself; references that are the same object as their links;
renumbering-free restructuring (insert, offset, merge, subtract are corpus
operations, not editing rituals); a paper that carries its own toolchain
and is self-extracting (§5.1); and a document a machine can fully read —
structure, references, provenance — without executing a single instruction
of the author's.

## 10. Non-goals and open questions, stated plainly

- Phextura does not replace TeX's typesetting quality on day one; the PDF
  backend's layout engine is future work, and the HTML backend is the
  honest first target.
- Phextura does not define what a processing program does with a hoisted
  payload — render it, extract it, execute it, or leave it opaque. That
  decision belongs to the program (and its operator), not to the format
  and not to this design; §5.1 declares the pattern, not a policy.
- Collaborative editing semantics (concurrent writers on one corpus) are
  out of scope for this design; `merge`/`subtract` give the algebra, not
  the protocol.
- Equation *entry* syntax inside a scroll (a math notation) is deliberately
  unspecified here: an equation scroll holds notation as text, and choosing
  that notation is a separate design that must not be allowed to grow into
  a new macro language by accident.
