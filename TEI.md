# TEI: the phextura profile for TEI-XML corpora

**Status: DRAFT v0.1** — the profile as implemented by teixml-to-mutable
0.3.0 (the reference converter). This document is the registry of record
for the profile's extension kinds and its normative segmentation table
(per LAYER2 §1.5 as amended). It follows the shape of STANDOFF.md: rules
first, conformance as a checklist, costs stated plainly.

Phextura converts TEI-XML into two phexts — a base corpus (DESIGN.md)
and a standoff corpus (STANDOFF.md) — such that the conversion is
*provably* reversible to the source's text, structure, and properties.
This profile exists because TEI is the largest living corpus format for
historical and literary documents, and because a conversion that cannot
be checked against its source is a claim, not a conversion.

Editorial rulings behind this profile (Will Bickford, 2026-11-14):
one base corpus per input directory; originals hoisted into the base
corpus, never sidecars; comments and processing instructions carried as
standoff entries; attributes as escaped plain header lines; one
standoff entry per element occurrence, at the accepted size cost.

---

## 0. What TEI is, verifiably

- TEI (Text Encoding Initiative) was founded in November 1987 at
  Vassar College to develop guidelines for encoding literary and
  linguistic texts.
- The current specification is TEI P5, first released 2 November 2007.
- The TEI Consortium publishes feature releases at least twice a year,
  plus patch releases every few months (4.9.0 Jan 2025, 4.10.0 Aug
  2025, 4.11.0 Feb 2026, 4.12.0 Jul 2026). TEI is a living format; this
  profile is written against its document model, not a frozen dialect.
- A TEI document is XML: one root element (conventionally `TEI`) in the
  TEI namespace `http://www.tei-c.org/ns/1.0`, holding a `teiHeader`
  (metadata) and a `text` (content: `front`, `body`, `back`).

## 1. The artifact set

Converting one input directory produces four files:

- `<name>.phext` — the base corpus (§2), where `<name>` is the input
  directory's name;
- `<name>.standoff.phext` — the standoff corpus (§4);
- `cavern.phext` — a PhextVFS image (below) holding the pair as its
  files, for Mutable's phext mode; `--no-vfs` opts out;
- `<name>.failures.txt` — always written; one `path<TAB>reason` line
  per file that did not convert (empty when none). A failed file is
  still hoisted into the base as an original with a failure provenance
  line in its metadata scroll. There are no silent skips.

The pair is self-contained: everything the checker (§5) needs is in
the two phexts plus the corpus they name. Nothing is fetched, ever:
external images and DTDs are declared, not retrieved.

Three phext-shaped things are easily confused; the disambiguation
matters more than the count:

- a **raw phext chain** (the converter's stage 0.1 form) is a stream of
  member phexts — not a corpus under DESIGN.md, and not emitted here;
- a **phextura corpus** — this profile's base — is a single phext laid
  out under DESIGN.md, addressed by LAYER2 keys, read as scrolls;
- a **PhextVFS image** (strata, JSON index scrolls, chunked content)
  is Mutable's filesystem serialization — not a corpus. This profile
  emits one as a companion artifact: `cavern.phext` holds the pair as
  the files `/<name>.phext` and `/<name>.standoff.phext` in the
  documents stratum, so a phext-mode Mutable Cavern opens on exactly
  the bytes this profile defines. The image changes nothing about the
  pair: the pair is the profile's artifact, the image is its transport
  into Mutable. Because phext bytes contain dimension break bytes,
  which the VFS text format refuses, the files are stored in the VFS
  binary format (base64 chunks under a format marker); read-back is
  byte-identical to the standalone files.

The corpus is named `corpus:<slug>` in L2 addresses, where `<slug>` is
the slugified directory name (§3 gives the slug rules). Both files are
written in libphext canonical form (`normalize` of `implode`). The
base edition's checksum is the libphext checksum (xxh3-128) of that
form with the manifest's `checksum:` line excluded; the first 16 hex
digits are the edition **pin** every standoff anchor carries. The
standoff carries its own checksum, computed the same way over its own
metadata scroll.

## 2. The base corpus

Corpus root Z = 1.1.1. Series 1 is corpus-level matter. Each TEI
document takes the next Series (2, 3, …) in sorted relative-path
order — including failed documents, whose Series holds only their
hoisted original and failure provenance.

### 2.1 Divisions (per document Series)

DESIGN.md's six Collections, allocated by this profile (per DESIGN §2
as amended by D-1, format profiles may allocate the standard divisions
and declare header vocabularies without minting roles):

- **Collection 1 — Body**: the document's text (§2.2, §2.3).
- **Collection 2 — Bibliography**: one scroll per bibliography entry
  (§2.4).
- **Collection 3 — Media**: one header scroll per external graphic
  (§2.5).
- **Collection 4 — Styles**: unused by this profile.
- **Collection 5 — Supplements**: the hoisted original (Chapter 1) and
  any DOCTYPE internal subset (Chapter 2) (§2.6).
- **Collection 6 — Metadata**: the document metadata scroll (§2.7).
  Series 1's Collection 6 holds the corpus manifest and the object
  table (§3.3).

### 2.2 Books and grouping

Books follow the document's own top-level shape. The **book roots**,
in order, are: the `teiHeader` element; then for the `text` element,
its `front`, `body`, and `back` children in order (or `text` itself
when it has none of those); then any other top-level element, each its
own book. Books are numbered from 1 in that order (so with a header
present, Book 1 is header matter and Book 2 is text matter). Every
element belongs to the book of its nearest book-root ancestor.

Within a book, Body coordinates group the segment stream (§2.3):
segments whose owners share the same direct child of the book root
form a **Chapter** run; within a chapter, segments whose owners share
the same second-level ancestor form a **Section** run; a segment's
**Scroll** number is its position in its section run. A new run starts
whenever the grouping ancestor changes, so the same ancestor appearing
again later opens a new chapter or section.

### 2.3 Segmentation (normative)

The segmentation is the profile's core editorial act, shared verbatim
with the converter's Mutable export so the two outputs can never
disagree about what a document's text segments are.

An element is **block-level** iff its local name is in the normative
block list. A block-level element, and any element containing a
block-level descendant, is a **container**: its direct text forms
segments between its child blocks. Every other element is **inline**:
its text joins the open segment, and the element is recorded as an
inline occurrence with a half-open Unicode-scalar span over the
segment text.

block-elements: ab, additions, adminInfo, analytic, argument, bibl, biblStruct, bindingDesc, body, back, cell, cit, closer, custodialHist, decoDesc, div, div1, div2, div3, div4, div5, div6, div7, docAuthor, docDate, docEdition, docImprint, docTitle, editionStmt, encodingDesc, epigraph, facsimile, fileDesc, front, handDesc, head, history, imprint, item, layoutDesc, lg, l, list, monograph, msContents, msDesc, msItem, msPart, notesStmt, objectDesc, opener, origin, additional, p, physDesc, profileDesc, publicationStmt, quote, recordHist, respStmt, row, series, seriesStmt, sealDesc, sourceDesc, sp, stage, supportDesc, surface, table, text, titlePage, titlePart, titleStmt, zone

A segment **survives** iff its text is non-whitespace or it carries at
least one inline occurrence; whitespace-only segments between blocks
are inter-block formatting and are not represented (the original is
hoisted, so nothing is lost — §5's checker compares under exactly this
normalisation). **One surviving segment is one Body scroll.** Inline
elements never appear in the base (DESIGN §10): they live in the
standoff (§4).

A Body scroll's text is, in order:

1. a role line: `@heading` when the segment's owning element is a
   `head`, otherwise `@prose` — these are DESIGN's structural roles;
   TEI element names are never roles;
2. a `key:` line, when the scroll is a keyed object (§3);
3. `tei-element: <local name>` — the owning element;
4. `tei-namespace: <uri>` when the owning element is not in the TEI
   namespace (the value is `(none)` for no namespace);
5. a blank line;
6. the segment text, verbatim.

Because the header lines are part of the stored scroll, standoff spans
address the full stored text, headers included (§4.2) — a span's
meaning never depends on stripping anything.

### 2.4 Bibliography (Collection 2)

A bibliography entry is minted for every `bibl` or `biblStruct`
element that is not nested inside another `bibl`/`biblStruct` and
whose subtree text is non-empty. Its scroll (Chapter 1, Section 1,
scrolls in document order) is header lines only:

- `key: bibentry:<slug>` (§3.2);
- `author:` — the first descendant `author` (else `editor`) element's
  text, whitespace-collapsed, escaped per §4.4 — when present;
- `title:` — the first descendant `title` element's text, likewise;
- `year:` — from the first descendant `date`: its `@when` year, else
  the first four-digit year in its text — when found;
- `text:` — the entry's full descendant text, whitespace-collapsed,
  escaped. This line is normative: §5's checker compares it.

The entry also stands in the Body as the segments its text forms; the
Collection 2 scroll is the citable object, not a second copy of the
reading text (its `text:` line is a field, collapsed by design).

### 2.5 Media (Collection 3) and closure

Every `graphic` element carrying a `url` attribute gets a header
scroll (Chapter 1, Section 1, document order):

```
@figure
key: figure:<slug>
url: <the url, verbatim>
tei-element: graphic
checksum: absent — external, not hoisted
```

The image bytes are an external dependency: the converter never
fetches them. Per DESIGN §5's closure rule, every distinct URL also
appears as an `external: <url>` line in the corpus manifest (§2.8), so
the corpus names everything its rendering would need.

### 2.6 Originals and DOCTYPE (Collection 5)

Chapter 1 holds the source file itself: a header scroll
(`source-path:`, `source-sha256:`, `size:`, `storage:`) and, for a
non-empty file, a content scroll holding the bytes verbatim when they
are text (valid UTF-8, no phext break bytes), else the canonical
libphext binary scroll form (`phext-binary:1:base64`). Byte-identity
of the hoisted original is profile law (§5).

A document's DOCTYPE is metadata, not text: its name and any
public/system identifiers appear as `doctype-name:` /
`doctype-public-id:` / `doctype-system-id:` lines on the document
metadata scroll, and a `doctype-subset: <coordinate>` line points at
the internal subset, hoisted verbatim to Collection 5 Chapter 2 (a
header scroll with `source-path:`, `doctype-name:`, `storage:`, and
the subset as its content scroll, binary form if it is not text). The
external DTD itself is never fetched.

### 2.7 Document metadata (Collection 6, X = 1.1.1)

For a converted document, in order:

- `key: paper:<slug>` (§3.2);
- `title:` — the document title (the chain: teiHeader's
  titleStmt/title, else msIdentifier/idno, else the filename stem),
  whitespace-collapsed, escaped;
- one `ms-<name>:` line per child element of the first `msIdentifier`
  (its text, collapsed, escaped), in document order;
- `source-path:` and `source-sha256:`;
- DOCTYPE lines (§2.6), when present;
- `conversion: converted`.

For a failed document: `source-path:`, `source-sha256:`, and
`conversion: failed — <reason>` only.

### 2.8 The corpus manifest (Series 1, Collection 6, X = 1.1.1)

Lines, in order: `corpus: <slug>`, `kind: tei-corpus`,
`tei-profile: 0.1`, `converter: <name and version>`, `created: <ISO
timestamp>`, `documents: <converted count>`, `failures: <count>`, one
`object-index: <kind> <coordinate>` per kind present (§3.3),
`object-ledger: <coordinate>`, one `external: <url>` per distinct
external URL (sorted), and lastly `checksum: xxh3-128:<hex>` — the
edition checksum of §1, excluded from its own computation.

## 3. Keys and the object table

LAYER2 keys are minted on demand, for the cited surface only. Kinds,
in object-table order:

key-kinds: paper, section, bibentry, figure, tei-element

Slugs follow LAYER2 §2.4's grammar (`[a-z][a-z0-9-]*`): lowercase
ASCII, non-alphanumerics become dashes, a slug that would start with a
digit takes an `id-` prefix. Within a kind, the first object in
document order keeps the bare slug; later collisions take `-2`, `-3`,
… suffixes. An object holds exactly one kind; when several could
apply, precedence is the kind order above (a cited `div` is its
section, not a `tei-element`).

### 3.1 The paper

Each converted document is a `paper`. Its slug derives from its
manuscript identity: the first `msIdentifier`'s `idno` text, slugified;
when there is no usable idno, from the document's relative path,
slugified. Its scroll is the document metadata scroll (§2.7), which
carries its `key:` line.

### 3.2 Sections, bibentries, figures, cited elements

section-elements: div, div1, div2, div3, div4, div5, div6, div7, msItem

- A **section** is a headed element from the section list: one with a
  descendant `head`. Its slug derives from the head's text (collapsed,
  slugified), else the element's `xml:id`, else `s-<series>-<n>`. Its
  scroll is the heading segment's Body scroll — the head's own first
  segment scroll where the head owns one, else the first Body scroll
  in the element's subtree — and that scroll carries the `key:` line.
- A **bibentry** is a bibliography entry (§2.4). Its slug is
  `<author surname>-<year>` when both are known (surname: the part
  before a comma, else the last name token), else the author, else the
  title's first five words, else `bib-<series>-<n>`.
- A **figure** is a `graphic` with a `url` (§2.5); its slug derives
  from the URL's filename stem.
- A **tei-element** (extension kind, registered by this document) is
  minted only on cite: an element whose `xml:id` is the target of an
  intra-document fragment reference (§4.5) and which holds no other
  kind. Uncited `xml:id`s are not keyed — the table stays the cited
  surface, not a dump of every identifier.

### 3.3 The object table (Series 1, Collection 6)

One index scroll per kind present, at Chapters 2, 3, … in the kind
order of §3, addressed from the manifest. An index scroll's lines are
`<kind>:<slug> -> <coordinate>`, sorted by slug; the table is derived
state and must rebuild byte-for-byte from the corpus's `key:` lines
and minting ledger — that rebuild is a validation rule (§6). The
ledger, at the chapter after the last index, records every mint in
order as `minted: <kind>:<slug> -> <coordinate>` under a
`ledger: key minting record` first line. Cited-element keys live only
in the table: no `key:` line for them exists in the base, because
LAYER2 §5 keeps citation machinery out of base scrolls — their scroll
is a Body scroll owned by another element, and their standoff entries
(§4) carry the spans.

## 4. The standoff corpus

The standoff is a phextura corpus under STANDOFF.md §3.1, pointing at
the base:

- Collection 1 holds the entries: X Chapter = the document's Series,
  Section 1, Scroll = the entry's ordinal within its document.
  Entries are keyed objects of kind `annotation` (`e-1`, `e-2`, … in
  corpus minting order: documents in Series order, entries in document
  order); the standoff's Collection 6 carries one index shard per
  document (`annotation:e-<n> -> <coordinate>` lines, in order).
- Collection 5 holds the events ledger: one `converted:` / `failed:`
  line per document under an `events: <total entries>` first line.
- Collection 6 (Series 1) holds the metadata scroll: `standoff-of:
  px2:corpus:<slug>`, `standoff-profile: 0.1`, `converter:`,
  `created:`, `entries: <total>`, and its own `checksum:` line.

### 4.1 Entry kinds

Per STANDOFF §3.3 as amended (converter profiles may declare
structural entry kinds), this profile declares:

- `tei-element` — one entry per element occurrence, all elements,
  block-level included (the D5 rule; grouping occurrences is not
  permitted by this profile);
- `tei-comment` — one entry per XML comment;
- `tei-pi` — one entry per processing instruction.

### 4.2 Anchors

An entry's `target:` is a full L2 address of the pinned base edition:
`px2:corpus:<slug>@e:<pin>#<selector>`, where `<pin>` is the base
edition checksum's first 16 hex digits (§1). Two selector forms:

- **Key form** `#<kind>:<slug>` — used for exactly one case: a section
  occurrence (§3.2). The section's object scroll is a Body scroll
  (its heading scroll), and the entry's survival kit then describes
  that scroll whole: the anchor denotes the object. Every other
  occurrence anchors by coordinate, including bibentry, figure, and
  cited-element occurrences (their home spans are the precise
  addresses; their keys resolve through the object table and
  `refers-to:` lines);
- **Coordinate form** `#at:<coordinate>@<start>..<end>` — a half-open
  span of Unicode scalar values over the target scroll's full stored
  text, header lines included. `start == end` is a point anchor. A
  span is never byte-based and never relative to a stripped body.

Home assignment — the span an occurrence anchors to:

- an inline occurrence: its span in its segment's scroll;
- a block-level element owning segments: its first segment's scroll,
  whole;
- any other element: a point at the text start of the first Body
  scroll in its subtree; failing that, the next Body scroll in the
  document (text start); failing that, the previous Body scroll (text
  end); failing that (a document with no Body scrolls), point 0 of
  its metadata scroll;
- a comment or PI inside a surviving segment: a point at its position
  in that segment; in a dropped segment or outside any segment: the
  text end of the previous surviving segment, else the text start of
  the next; a prolog comment/PI: point 0 of the metadata scroll; an
  epilog one: the text end of the last Body scroll, else metadata
  point 0.

### 4.3 The survival kit

Every entry carries STANDOFF §2.3's kit, computed over its anchor
span: `target-hash: xxh3-128:<hex>` over the span's UTF-8 bytes,
`target-text:` (the span verbatim, escaped per §4.4), and
`target-before:` / `target-after:` (up to 80 scalars of context on
each side, omitted only at the scroll's boundaries). Re-location
follows STANDOFF §2.4: against the pinned edition every anchor must
resolve EXACT; against a drifted edition, key anchors re-base through
the object table, coordinate anchors transport only through a mapping
scroll (base Collection 6, Chapter 8, `<old addr> -> <new addr>`
lines), and a span that cannot be verified is detached with its
quoted text — never heuristically re-attached.

### 4.4 Entry scrolls and the attr- grammar

An entry scroll is, in order: `@annotation`; `kind:`; `key:
annotation:e-<n>`; `target:`; the kit lines; for element entries,
`tei-element: <local name>` and (when not in the TEI namespace)
`tei-namespace: <uri>` (`(none)` for no namespace); for PI entries,
`pi-target: <target>`; the attribute lines; the reference lines
(§4.5); `author: teixml-to-mutable`; `produced-by: <converter and
version>`; `created: <ISO timestamp>`; a blank line; and the body —
for elements, the element's full descendant text verbatim; for
comments, the comment text verbatim; for PIs, the PI data verbatim.

A header line is `name: value` with the name running to the first
`: `. Attributes ride as `attr-<name>: <value>` lines in document
order. Namespace declarations (`xmlns`, `xmlns:*`) are not attributes:
the namespace rides on `tei-namespace:`. A namespaced attribute name
renders as `attr-{<namespace-uri>}<local name>`, the prefix resolved
against the `xmlns` declarations in scope (`xml` is predefined); an
undeclared prefix renders as written. Values are plain text with
backslash escaping — the full escape set is: `\\` for backslash,
`\n`, `\r`, `\t`, and `\ ` for a leading or trailing space. One
physical line always carries exactly one value; unescaping is strict
(an unknown escape is an error, never a guess).

### 4.5 References

pointing-attributes: ref, target, corresp, decls, sameAs, copyOf, next, prev, ana

Attributes from the pointing list are recorded verbatim as `attr-`
lines, and additionally, per whitespace-separated token:

- a `#fragment` token naming an `xml:id` **in the same document**
  (XML fragment semantics; keys are corpus-global but resolution is
  intra-document) earns a `refers-to: px2:corpus:<slug>@e:<pin>#<kind>:<slug>`
  line naming the target's key (§3.2);
- a `#fragment` token naming nothing in the document earns a
  `refers-to-unresolved: <token>` line — loud, counted, never guessed;
- any other token earns a `refers-to-external: <token>` line,
  verbatim, never fetched.

`msIdentifier` is metadata, not linkage; `locus` and friends are
ordinary elements. Nothing in the base is written or altered by
reference resolution — the base never knows the standoff exists.

## 5. Reconstruction is profile law

A pair conforms only if the source document model can be
reconstructed from the pair and checked equal, for every converted
file:

1. **Originals**: every hoisted original (converted and failed files
   alike) is byte-identical to its source, SHA-256 checked.
2. **Text**: the Body scroll bodies, in coordinate order, are exactly
   the source's kept segment texts under §2.3's segmentation.
3. **Structure**: element entries, in order, equal the source's
   element occurrences in pre-order — local name, namespace, attribute
   set (rendered per §4.4), and body (full descendant text).
4. **Comments and PIs**: the `tei-comment` / `tei-pi` entries, in
   order, equal the source's comments and PIs in document order
   (prolog, in-tree, epilog), text and PI target/data verbatim.
5. **DOCTYPE**: the metadata lines and hoisted subset equal the
   source's declaration.
6. **References**: each entry's `refers-to:` addresses resolve through
   the object table to exactly the occurrences the source's pointing
   attributes name; unresolved and external counts match the source.
7. **Bibliography and media**: Collection 2 `text:` lines equal the
   candidate entries' collapsed texts; Collection 3 URLs equal the
   source's `graphic` URLs.

A pair that cannot pass this check is not a conversion of its corpus.
The reference converter ships the checker (`verify-phextura
--corpus`); §7 reports its results on the reference corpus.

## 6. Conformance checklist

- [present] Converter emitting this profile's artifact set
  (teixml-to-mutable 0.3.0, `phextura` subcommand).
- [present] DESIGN §7 validation over emitted pairs: coordinates and
  scroll grammar, manifest and standoff checksums recompute, object
  table rebuilds byte-for-byte, Collection 3 closure against the
  manifest, no resolvable bare coordinate in Body prose, entry
  grammar and pin discipline, standoff index shards rebuild.
- [present] Reconstruction checker (§5) and re-location harness
  (§4.3), including drifted-edition probes (edited spans detach as
  ANCHOR-LOST; ambiguous spans detach as AMBIGUOUS; nothing
  re-attaches silently).
- [present] Hosting: the pair is two phext files; SQ serves their
  scrolls unchanged — every scroll SQ can address, byte-for-byte
  (its bundled libphext caps coordinate components at 999, so scrolls
  past that cap are unreachable in SQ v0.6.0; see §7).
- [present] The PhextVFS companion image: emitted by default
  (`--no-vfs` opts out); Mutable's own PhextVFS reader opens it with
  L9 checksum verification passing, the pair reads back out of it
  byte-identical, and Mutable's phext-mode install flow accepts it
  as a Cavern.
- [to build] Mutable-side composition: a Mutable phext-mode document
  carries base scroll text with coordinates in its block metadata,
  but composing base+standoff into annotated views inside Mutable
  (STANDOFF §6 items M1/M3/M5/M6) is Mutable work, not done; today
  standoff entries arrive in Mutable as entry text only.

## 7. Costs and honest limits

Measured on the reference corpus (the Wellcome Collection TEI corpus:
1,886 files, 28.8 MB of XML, 1,875 converted / 11 failed):

- Base corpus: 58.05 MB (58,053,699 bytes, 142,619 scrolls);
  standoff corpus: 341.30 MB (341,302,423 bytes, 386,175 scrolls);
  384,298 standoff entries (374,947 element, 7,094 comment,
  2,257 PI). The PhextVFS companion image is 533.21 MB
  (533,212,798 bytes, 8,144 scrolls) — the pair's bytes at base64's
  4/3, plus its index and checksum scrolls.

The standoff is the expensive half by design: one entry per element
occurrence with a full survival kit and the element's text as body.
That is the ruled price of never losing an anchor; a profile amendment
that groups occurrences would need to re-prove §5.

Limits, stated plainly:

- The segmentation's block list (§2.3) is a curated profile choice,
  not the whole of TEI; elements outside it are inline. The list is
  normative here so the choice is checkable, not because it is final.
- References resolve intra-document only; cross-document linking in a
  corpus is future profile work.
- The entry body of an element is its text; element *boundaries*
  inside a segment are the inline spans, and containers are points —
  XML tree shape is recoverable from entry order, kinds, and bodies
  (§5.3), not from the base alone.
- Nothing external is fetched: images, DTDs, and external references
  are declared, and a consumer that needs the bytes must fetch them
  itself.
- SQ v0.6.0 bundles libphext 0.3.3, whose coordinate components cap
  at 999: on the reference corpus it cannot address 28,723 of the
  base's 142,619 scrolls or 154,376 of the standoff's 386,175 (every
  scroll it can address is served byte-identical). This is a reader
  limit, not a corpus property — current libphext reads every
  scroll, and this profile's own validation runs over the full maps.

## 8. Amendments this profile carries

- **S1** (STANDOFF.md §3.3): converter profiles may declare structural
  entry kinds in their profile document; §4.1's kinds are the first
  instance.
- **D-1** (DESIGN.md §2): format profiles may allocate the standard
  Collections and declare header vocabularies for their scrolls
  without minting roles; §2.1 and §2.7's lines are the first instance.
- **L-1** (LAYER2.md §1.5): extension kinds registered by a profile
  document at the repository root are registered in the vocabulary;
  §3's `tei-element` kind is the first instance.

## 9. Dependencies on unbuilt machinery

- Mutable composed views (STANDOFF §6 M1/M3/M5/M6) are [to build] in
  Mutable; this profile claims nothing about them.
- Mutable's knowledge-store annotation budget (10,000 properties per
  resource) binds representations that carry properties *inside* a
  document; this profile's entries are scrolls in a separate corpus,
  not resource properties, and the budget does not bind here. Whether
  a future Mutable composition re-introduces it is Mutable's question.
- Edition transport beyond one re-base (mapping chains across several
  editions) follows STANDOFF §2.4 unchanged; this profile adds only
  the mapping scroll's home (§4.3).

*TEI.md — DRAFT v0.1. Amendments follow the STANDOFF §8 precedent:
numbered, appended, never silently rewritten.*
