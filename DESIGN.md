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
(Chapter.Section.Scroll) read as (section, subsection, block): the body
address provides two sectioning levels below the chapter. A third level
(subsubsection, paragraph-sections in LaTeX terms) is carried by heading
blocks (below), which are addressable as blocks — citation granularity is
preserved at block level even though the third level is not a coordinate
component. That is a stated limitation of the layout, recorded, not hidden.

The block — one scroll — is the smallest authored unit: a paragraph, an
equation, a theorem, a proof. Its role is declared by a first-line role
line, plain text: a primary role plus optional secondary role atoms:

```
@theorem
Let G be a finite group and p a prime dividing |G|...
```

```
@figure +evidence
…
```

An untagged block is not role-less: its role is `prose`, explicitly. The
normative core vocabulary is `@theorem`, `@lemma`, `@proposition`,
`@corollary`, `@assumption`, `@definition`, `@proof`, `@equation`,
`@equationgroup`, `@figure`, `@caption`, `@quote`, `@code`, `@footnote`,
`@heading`, `@remark`, `@prose`. Roles are an **open vocabulary over this
normative core**: a paper may declare extension roles in its Collection 6
manifest (which also records the vocabulary version), and renderers must
know the core; a role outside both is a lint error, never a silent
free-text fallback. (Field evidence: the first demo paper needed
proposition, corollary, assumption — load-bearing, referenced like
theorems — and remark, none of which the original closed list had.)

- Format profiles (TEI.md is the first instance) may allocate the
  standard divisions and declare header-line vocabularies for
  source-format facts, without minting roles; a profile's header
  lines are data in the scroll, read by the same discipline as
  `key:` lines.

A **heading** is a block with role `@heading`; its text is the heading
text. Headings are data, never markup prefixes smuggled into prose.

An **equation group** (a multi-row display referenced as a whole) is an
`@equationgroup` header scroll followed by its row `@equation` scrolls.
References to the group name the header; references to a row name the
row. The render overlay numbers rows with letter suffixes by position
within the group — grouping is recorded in the corpus, so the numbering
is regenerable from traversal, like every other display number (§3).

A claim-bearing block may carry **warrant edges** as lines after its
role line; the edge grammar and its validation rules are in §3.

This design defines block structure only. Inline typographic structure
(emphasis, defined terms, and the like) is plain text by stated
convention; an inline convention is an open question (§10), not an
unstated assumption.

The role line is data in the scroll, so a renderer, a linter, and a
human all read the same fact the same way.

Addresses are deterministic: given the outline, any address can be computed
by hand. Unused components sit at 1. Nothing is ever addressed by a string
the author invented and a program merely hopes matches.

## 3. References are coordinates

To refer to a theorem, an equation, a section, or a figure, the author
writes its coordinate in the text. There is no `\label`, no `\ref`, and no
rerun-until-it-settles:

- In the phext editor, an address written in text is automatically a
  hyperlink to that scroll (verified behavior of Phext Edit). The reference
  and the link are the same object.
- A written reference is recognised by the coordinate grammar (libphext
  `Coordinate: TryFrom`, which in 0.5.0 properly returns
  `Err(PhextParseError)`). What counts as a written reference must be
  *decidable*: coordinate-shaped strings occur in ordinary prose (version
  numbers, section-like numerals), and the grammar's collision surface is
  the validator's stated problem — a reference that is missed silently,
  or flagged falsely, is a reference that rots silently, one step removed.
- Validation scans the corpus's **non-opaque scrolls** (the §5 opacity
  rule: hoisted payload scrolls and media payloads are excluded — a
  payload may contain coordinate-shaped substrings that are not
  references). Cost, stated: Θ(B + S + D) expected time (B = corpus
  bytes, S = scrolls, D = distinct targets; the expected case assumes a
  hash index), Θ(B + S + R) space (R = written references), performed
  as roughly three traversals — enumerate, index, resolve. Without the
  index the cost is Θ(B + R·S); the design does not claim
  otherwise. Any coordinate that does not resolve is reported with the
  coordinate of the scroll that contains it. A dangling reference is a
  lint error at a known location, not a `??` discovered in a PDF.
- Display numbers (“Theorem 3.2”, “Figure 5”,
  “§4”) are a **render-time overlay**, computed by a single
  traversal with running counters per (role, section) class —
  Θ(N) in body blocks. They are never stored, so inserting a theorem
  renumbers nothing by hand and breaks nothing: numbers are views;
  coordinates are identity. This deletes LaTeX's entire
  label/counter/rerun complex (warts W4, W5, W27).

**One float, one address.** A figure or table has exactly one address
— its media scroll (Collection 3). Placement in the argument is a
*marker*: a body scroll whose content is the float's coordinate, stating
“render it here.” References to the float name the float's own
address, never the marker's; a marker is not a citable identity for the
float. (A reference that can land on two addresses is not a reference.
Field evidence: an early demo converter gave every float two identities
and 27 in-text references silently landed on placement markers.)

**Nested displays are extracted.** Displayed content inside a host block
(an equation inside a proof) that is itself referenced gets its own
scroll, placed immediately after its host; the host's text carries the
coordinate at the display's position. Unreferenced displays may remain
inline in the host. A proof's referenced interior is therefore
addressable content, never opaque text.

**Citations have two positions.** A parenthetical citation is the
bibliography entry's coordinate alone; the renderer generates its label.
A narrative citation (“Jain et al. (2026) show…”) is prose:
the author writes the noun phrase in the text and places the entry's
coordinate immediately after it as the reference; a renderer may fold
the coordinate into a link on the phrase. A bare coordinate is never
made to do the grammatical work of a noun phrase.

### Warrant edges

A claim's warrant is data (see PRINCIPLES.md, P11). A claim-bearing
block carries zero or more warrant edges as lines after its role line:

```
warrant: <kind> <target-coordinate(s)> [asserted-by …] [basis …]
```

Kinds: `formal-proof`, `prose-proof`, `citation`, `measurement`,
`assumption`, `conjecture`, `fiction-device`, `stipulation`,
`refutation`. Validation is per kind, mechanically:

- `formal-proof`: the declared checker **runs and accepts** the proof
  against this claim's statement scroll. Existence of a proof scroll is
  not validation. (Until the native proof format exists, this kind is
  available only via a hoisted checker payload that actually checks
  — §5.1.)
- `prose-proof`: target resolves and is role-tagged `@proof`; displayed
  as “proof (prose) at ⟨coordinate⟩,” never as a bare
  “proved.”
- `measurement`: targets are a data scroll and a code-or-method scroll,
  both checksummed; validation checks resolution and checksum agreement.
- `citation`: target resolves, and the edge records the target
  edition's checksum — an upstream retraction then registers as an
  edition change instead of being silently inherited.
- `assumption`, `conjecture`, `fiction-device`, `stipulation`: author
  stances, no target required, displayed as stances.

Displayed status (“Theorem — proved”) is a view computed
from the edges under a stated policy. The author assigns edges; checkers
assign validation results; venues and readers assign display policy. No
party both asserts and certifies. Edges are added, never silently
edited: a `refutation` edge supersedes by addition, so the history of
what was believed, when, and why it changed is itself corpus data.

## 4. Bibliography without a pipeline

A bibliography entry is one scroll in Collection 2. An entry is
**both** structured fields and text: field lines where structured data
exists, and a `text:` line — always present — carrying the entry
exactly as published:

```
author: A. Grothendieck
title: Revêtements étales et groupe fondamental (SGA 1)
year: 1971
text: A. Grothendieck, Revêtements étales et groupe fondamental (SGA 1), 1971
```

Real bibliographies are semi-structured: years hide inside author–year
tags, venues split across spans, some entries expose no parseable year at
all. The `text:` line is therefore normative, not a fallback: fields are
an index over the entry, the text is the entry. A citation in body text
is the entry's coordinate (§3, including the narrative-position rule).
The rendered bibliography is a traversal of the cited entries — in
citation order or sorted, a renderer choice. There is no `.bib` file, no
`.bbl`, no external program, and no citation-style programming language:
a citation style is a style scroll (Collection 4) declaring field order
and punctuation.

Authors are plain strings in this design. Duplicate detection across
entries uses libphext `soundex_v1` on author names as a first-pass fuzzy
aid (it is a phonetic hash; phextura uses it for candidate grouping
only, never as proof of identity). Cross-paper author identity —
deduplication and disambiguation of the same person across papers —
is **not** solved by this design and is not claimed: it would require a
shared author registry, which is a separate component (see the Pass 3
proposals).

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
same corpus, citable by the same coordinates as the theorems. Carrying is
the floor, not the achievement. What the corpus records is a
**provenance graph**: an evidence scroll (role `@figure +evidence`, a
dataset, a table) carries a **produced-by edge** naming the coordinate
and checksum of the code scroll that produced it, the coordinates and
checksums of its inputs, an environment declaration (the §5.1
manifest's toolchain entries are the anchor), and the output's own
checksum recorded at generation time. Produced-by edges are mandatory
for evidence-tagged scrolls and are validated like references (§3):
an evidence scroll without a valid edge fails validation. That buys
tamper-evidence and audit. It does not by itself buy reproduction:
re-running the pipeline and comparing output checksums is a stronger,
opt-in act, priced per pipeline, never folded into the cost of
validation. (The name for the stronger property — a
*reproducibility weave*, with the run-traversal specified as a
topological traversal of the produced-by graph — is defined in
PRINCIPLES.md; this design claims the provenance graph.)

**Closure is relative to the manifest.** The paper is closed iff every
dependency of rendering and checking is either a scroll in the corpus or
a line in the Collection 6 manifest naming the external dependency with
the checksum or version that satisfies it. Fonts and the renderer
itself are honest manifest entries, not silent exceptions. Anything
neither in the corpus nor in the manifest is a closure violation,
mechanically detectable.

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

Self-extraction extends to proof. A paper may also hoist a proof checker
as a payload scroll and carry its proofs as addressed scrolls in the
corpus, so the paper is its own verified artifact rather than pointing at
a formalization living in a separate repository; prior theorems are cited
by coordinate exactly as bibliography entries are (§4).

The checker that would be hoisted today is Lean's, and its status in this
design is **provisional**. The Lean payload is a bootstrap: adopted
because it is proven and at hand — libphext itself has been formally
verified with Lean/Aeneas tooling in the author's own proving program, so
the first self-verifying phextura papers have a real checker-and-proof
story to carry from day one — and explicitly pending replacement by a
cleaner, more phext-native proof format. The payload is the bootstrap,
not the destination.

The native format, when it comes, is defined by what it must be:

- proofs as plain-text scrolls, readable like the rest of the corpus;
- statements, steps, and prior theorems addressed by coordinate — no
  global proof namespace (the same wart as §6, one system deeper);
- verification by a small reader-side checker whose full behavior can be
  specified briefly and re-implemented independently, so no single
  implementation is the format. Metamath (verifier ≈300 lines,
  substitution as sole rule) is the standing existence proof that a
  small checker is possible for a format optimised for its checker;
  its proofs are famously not prose-readable, so it is no existence
  proof for this design's actual target — small checker *and*
  readable proofs *and* step-addressability. That conjunction is the
  open problem, stated at its true difficulty. Two related precisions:
  the trusted-base comparison for any checker is kernel-to-kernel
  (Lean's kernel is small; its elaborator and toolchain are the large
  surface and need not be trusted for checking), and a manifest
  checksum gives tamper-evidence, not authorship — authorship
  requires a signature anchored outside the corpus, or is recorded as
  self-asserted.

Stated honestly: phextura absorbs the prover as cargo and dissolves the
paper/proof boundary; it does not replace the checker's kernel or the
proof language — the checker remains itself, carried rather than competed
with.

### 5.2 Review and marginalia are corpora

Review is not a process around the paper; its product is a corpus. An
annotation corpus is a sibling phext whose scrolls point at the reviewed
paper's coordinates (edition-qualified: the paper's manifest checksum
plus the coordinate), so every comment is navigable from remark to the
exact scroll it addresses, and author replies are scrolls in the same
space, pointing back. Marginalia ride the same provenance discipline as
everything else (§5, PRINCIPLES.md P10): who wrote an annotation,
when, and — for machine-generated annotation — which program produced
it, are recorded in the annotation corpus itself.

Machine-generated review is **labelled as simulation everywhere it
appears**: a generated critique encodes the standards it was distilled
from, confers no authority, and is never presented as any person's
actual words or endorsement. (Simulated reviewer *personas* that wake
when a paper is opened were evaluated in the design loop and are a
proposal for Will, not part of this design — see PRINCIPLES.md.)

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

Stated plainly, what a style scroll is: a **parameter assignment** —
named quantities given values, stored as addressed data. This design
defines **no layout constraint vocabulary and no solver**: there are no
document-level equations, no unknowns, and no optimisation over blocks.
Paragraph breaking in a future PDF backend adopts total-fit optimisation
(§7); page breaking and float placement have no such classical
optimum and are not claimed to. A priced constraint vocabulary for
block layout is an open requirement (§10, and PRINCIPLES.md for its
minimal specification). When a backend does solve any layout problem,
the solver's identity and version are part of the rendering definition
and are recorded in the manifest — two solvers reaching different
optima otherwise produce two papers from one corpus.

## 7. Rendering is a traversal

Rendering a phextura paper is a defined traversal, in bounded passes
whose costs are stated (§3 gives the validation cost; nothing here
claims to be a single pass):

1. **Load:** `explode` the `.phext` file into scrolls.
2. **Validate:** every coordinate written in a non-opaque scroll
   `fetch`es; every media header's `checksum` matches its payload; every
   cited bibliography scroll exists; every evidence scroll's
   produced-by edge is valid; `formal-proof` warrant edges are checked
   by running their declared checker. Failures are reported by
   coordinate.
3. **Traverse:** the canonical reading order is *defined*, not assumed:
   the Body division in coordinate order, with referenced non-body
   units (figures, footnotes, media) rendered at their reference sites;
   the remaining divisions in Collection order for validation and
   derived matter. Enumerate the body subspace
   (`get_subspace_coordinates`) and walk it (`next_scroll`), resolving
   each reference by `fetch` as it is met — references are resolved
   by construction, so there is nothing to rerun. HTML rendering is
   Θ(B + D) expected.
4. **Present:** apply the governing style scrolls; emit the backend
   format. Layout beyond the traversal is UNPRICED in this design: no
   block-layout solver is defined (§6, §10).

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
| References | String labels, `.aux`, multi-pass, can dangle | Coordinates, mechanically validated at stated cost; dangling = lint error at the pointing scroll |
| Bibliography | External `.bib`/`.bbl` pipeline | Entry scrolls in the corpus |
| Media | External files by path | Scrolls inside the corpus (§5) |
| Claim warrant | Typography and convention | Warrant edges as corpus data; display computed by policy (§3) |
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
  a new macro language by accident. One requirement is already forced by
  field evidence: whatever notation is chosen, a paper's own definitions
  (its preamble macros, in LaTeX terms) need a declared home in the corpus
  — interim practice stores verbatim LaTeX source whose paper-level
  macros dangle semantically even when every coordinate resolves. Stored
  math must be self-contained math.
- The layout constraint vocabulary (§6) is an open requirement: the
  minimal specification — measurable block geometry, inter-block glue,
  named penalty classes, a single stated objective, feasibility
  thresholds with specified degradation, a named solver pinned in the
  manifest, and scope honesty across line/page/float breaking — is
  in PRINCIPLES.md. Until it exists, every layout claim in this design
  is UNPRICED.
- The warrant graph's **display policy** (§3) — how a reader
  distinguishes formal proof, prose proof, measurement, and stance at a
  glance without trusting the author — is specified as an obligation,
  not yet as a design; it wants testing on a paper with a conditional
  proof and a retracted citation.
- Delight is an obligation with a measuring instrument still to build:
  a benchmark suite over a real corpus (keystrokes, editor commands,
  off-screen decisions per common operation, with targets and method).
  Until it is run, delight claims for this design are provisional.
- An inline typographic convention (emphasis, defined terms) inside a
  block is undefined; block content is plain text by stated convention
  (§2). Settling it is a notation design, like math entry, and is
  not to be smuggled in as markup by a converter.
- The `@footnote` path is unexercised by the first demo (its paper had
  no footnotes). The follow-up test is named: tracker Paper 012
  (arXiv:2611.11959, 16 footnotes).
- A phext-native proof format is an explicit open research problem: format
  TBD. §5.1 states its requirements — plain-text proof scrolls,
  coordinate-addressed statements, steps, and prior theorems, and a small
  reader-side checker that can be specified briefly and re-implemented
  independently. Lean payloads are the provisional bootstrap until that
  format exists, not the destination.
