# phextura standoff — the compatibility profile

**Status: DRAFT profile, v0.1 — 2026-11-11.** Task coordinate
23.7.11/8.7.11/6.7.9. Originating ruling (Will Bickford, 2026-11-11):
*"phextura needs to be standoff compatible. mutable will be the first
editor."*

This document specifies what it means for a phextura file to be
annotated from outside itself: commentary, persona marginalia, warrant
evidence, and proposed corrections living in separate documents,
anchored to the base by durable address plus span selectors with
edition pins, composed at read time, never modifying the base. It
builds on DESIGN.md (esp. §3 warrant edges, §5.2 annotation corpora)
and LAYER2.md (the `px2:` scheme, edition pins, the nine failure
modes). Where this profile touches DESIGN.md or LAYER2.md it does so
as proposals (§8); neither document is amended by this one.

The stance generalizes persona-alignment amendment A10 — read-time
composition is a **view**, not a merge — from persona marginalia to
all annotation. What A10 scoped for one component, this profile makes
the format's general law.

## 0. The first editor, as it verifiably is today

The profile is written against mutable (wbic16/mutable-os, a fork of
argimenes/mutable-os; the repo's own description: "Another version of
Codex OS rebuilt with Astra as a reactive block-tree based system in
SolidJS"). Facts below are from the repository at `main` as read on
2026-11-11; each is cited to its source file so the implementation
phase can re-check them. What could not be verified is listed as an
open question, here and in §9 — the profile invents no editor
capabilities.

**Verified.**

- **Phext is supported, as a projection.** Mutable opens and exports
  `.phext` files from the Workspace menu (`phextDocuments` feature
  flag, enabled by default). Both are explicit manual file actions:
  no autosave, no background conversion, nothing phext-related runs
  while typing. One phext ↔ one Document; one scroll ↔ one TextBlock
  whose `metadata.phextCoordinate` carries the coordinate string; the
  codec round-trips byte for byte. The Document's own format is
  Mutable JSON — phext is a text projection of it. (`PHEXT.md`,
  `src/phext/document.ts`.)
- **Coordinates are the only durable identity across that
  projection.** Import assigns each block a fresh random UUID
  (`crypto.randomUUID()`); block identity does not survive an
  export/re-import cycle. The coordinate in block metadata does.
  Any anchoring scheme for mutable must therefore address
  coordinates, never editor-internal ids. (`src/phext/document.ts`.)
- **Import and export today carry text only.** `phextToDocument`
  constructs bare text blocks — roles, warrant edges, and all
  phextura structure above the scroll level are not interpreted;
  `documentToPhext` collects blocks with string `text` and omits the
  rest. No annotation of any kind crosses the phext boundary today.
  (`src/phext/document.ts`.)
- **Mutable's text engine is already standoff at the intra-document
  level.** Paragraph text is a sequence of Cells; annotations are
  `StandoffAnnotation` records `{id, type, start, end}` with
  inclusive Cell indexes, stored in `payload.standoffProperties` —
  data held apart from the text, never markup inside it. Range
  machinery exists and is disciplined: `TextRanges.snapshot/validate`
  (half-open, *versioned* boundaries carrying content/placement
  identity, in `cell` or `utf16` coordinates),
  `RangeAnnotations.apply` (Cell coordinates required, whole batch
  validated, exact duplicates removed), and text edits move ranges
  through core range-mapping rather than per-effect mutation.
  (`docs/development/CREATING_STANDOFF_EFFECTS.md`.)
- **Cross-block and cross-resource annotation precedents exist.**
  `LinkedAnnotations` lets segments in many blocks share one
  `annotationId` whose definition lives in a registry on an owner
  record; a segment may instead carry an `externalDefinition`
  pointing at a definition in *another resource* (source + targetId +
  version), under the format tag `codex-external-definition-gate`.
  Resolution refuses to guess: "unresolved/ambiguous is never a
  guessed value." In the code as read, external definitions resolve
  only when their version is `unpinned`; a pinned external definition
  is not resolved by that path today. Margin rendering, an annotation
  monitor, and a marginalia feature exist. Deleting an annotation
  marks it deleted rather than removing the record.
  (`src/block-tree/linked-annotations.ts`,
  `docs/development/ANNOTATION_MONITOR_MIGRATION.md`, repo tree.)
- **The live model is a normalized block repository** (content and
  placement records) with intent-level commands, per-view
  projections, and a persistence service using optimistic revision
  checks; durable block history exists but is feature-flagged off by
  default. (docs/architecture/OVERVIEW.md.)

**Not present today (verified absences in the sources read).**

- No concept of one Document annotating *another* Document's phext
  coordinates; annotations live inside the document they annotate.
- No edition identity for phext files: `PhextDocumentMetadata` is
  `{version: 1, sourceFilename}` — no content checksum, no manifest,
  nothing an edition pin could bind to.
- No anchor re-location procedure across phext editions, and no
  offset system in Unicode scalar values (mutable's range systems
  are Cell and UTF-16).

These absences are not defects in mutable; they are the profile's
work list for the first editor, itemized as testable requirements in
§6. Where mutable's existing model constrains the profile, the
constraint is stated where it bites (§2.4, §6).

## 1. Definitions

- **Base corpus** (the base): a phextura file being annotated — a
  paper, a tracker corpus, any file authored under DESIGN.md. The
  base is identified by an L2 work reference and, wherever exactness
  matters, by an edition pin (LAYER2 §1.3, §3).
- **Standoff document**: a phextura file whose scrolls annotate
  another file. It has its own identity, its own edition, its own
  author; it is a first-class corpus, citable and annotatable in its
  turn. DESIGN §5.2's "annotation corpus" is this profile's standoff
  document restricted to review kinds (§3.3).
- **Annotation entry**: one scroll in a standoff document carrying
  one anchor, one kind, a body, and its receipts (§3.2). Entries are
  the atoms of the system: individually addressable, individually
  honourable, never silently editable.
- **Anchor**: the part of an entry that names its target: an L2
  address (work reference + edition pin + object selector), an
  optional span selector, and the survival kit of §2.3 that lets an
  editor re-locate the target — or fail loudly — when the base has
  drifted from the pinned edition.

**The invariant, stated once:** *a standoff document never modifies
its base. Composition happens at read time, as a view. The only path
by which anything in a standoff document changes the base is a new
edition of the base, produced by the base's keeper, receipted in both
files (§5).* Everything below is machinery for keeping that sentence
true under drift, disagreement, and time.

## 2. The anchor model

### 2.1 What can be pointed at

An anchor names, in order of preference:

1. **An object** — a keyed research object in the base (LAYER2
   §1.4–1.5): a theorem, a figure, a claim, a section. The object is
   a scroll (or, for media, the header scroll per one-float-one-
   address), and object identity survives the base's re-bases
   because keys are minted once and never reassigned. This is the
   durable form, and the one tooling should mint by default.
2. **A span within an object** — a character range inside the
   object's scroll (§2.2). Spans are for the comment that is about
   *these words*, not the whole block.
3. **An unkeyed scroll, by coordinate** — the fallback form, valid
   only under an edition pin (§2.2). A coordinate is internal to its
   file and edition (LAYER2 §1.1); pinned, it is exact; unpinned,
   it is meaningless, and a conforming tool must refuse to mint one.
   This preserves DESIGN §5.2's current pointing rule — (manifest
   checksum, coordinate) — as the profile's special case for targets
   that carry no key, rather than discarding it.

Anchors target **non-opaque scrolls only** (DESIGN §5's opacity
rule): a hoisted payload can be anchored as a whole, via its header
scroll; a span *inside* base64 is not a span.

### 2.2 Anchor syntax

The anchor's address part is an L2 address (LAYER2 §3 grammar)
carrying two profile extensions, both confined to the fragment:

```ebnf
anchor      = px2-address [ span ] ;
object-ref  = kind ":" key | "at" ":" coordinate ;
                (* extends LAYER2 §3: the "at" selector names an
                   unkeyed scroll by its L1 coordinate and REQUIRES
                   a pin in the address head *)
span        = "@" start ".." end ;
start, end  = DIGIT+ ;
                (* Unicode scalar offsets into the target scroll's
                   full stored text — role line and warrant lines
                   included — half-open [start, end); start ≤ end;
                   start = end is a point anchor *)
coordinate  = (* an L1 coordinate, LAYER2 §3's coordinate grammar *)
```

Parse notes: the pin remains in the head (`@e:…`, split per LAYER2's
parse order, before the `#`); a key slug cannot contain `@`, so the
first `@` in the fragment unambiguously opens the span. Hash and
offset conventions are over the scroll's **full stored text**,
because the scroll is the unit `fetch` returns: any convention that
excluded the role line would make every hash depend on a
preprocessing step two implementations could perform differently.

Worked forms (demo corpus facts per LAYER2 §7; pin value a stand-in,
as there):

- Object: `px2:arxiv:2611.08794@e:cec31e46e2ff1f1c#theorem:uniform-consistency`
- Span: `px2:arxiv:2611.08794@e:cec31e46e2ff1f1c#theorem:uniform-consistency@412..517`
- Unkeyed scroll: `px2:arxiv:2611.08794@e:cec31e46e2ff1f1c#at:1.1.1/1.1.4/1.2.9`

**Offset-system constraint from the first editor (§0):** mutable's
native range systems are Cell indexes and UTF-16 code units; phext
scrolls are text, and the profile's scalars are neither. A conforming
editor converts at the boundary — its native ranges to scalar
offsets when minting an anchor, scalar offsets back when resolving —
and the conversion must be exact, including for astral-plane
characters (one Cell can hold multiple code units; string slicing is
not a conversion). This is real implementation work in mutable, not
a configuration option, and §6 prices it as such.

### 2.3 The survival kit

An address alone names where the target *was*. The kit is what lets
an editor find it again — or prove it gone. Every entry carries, as
lines in its scroll (§3.2):

- **The edition pin** (`@e:` in the target address): the edition the
  anchor was minted against.
- **The offset pair** (in the span selector; for object anchors, the
  implicit whole-scroll span).
- **A hash of the anchored text**: `target-hash: xxh3-128:<hex>` —
  libphext `checksum` over the anchored span's UTF-8 bytes (for an
  object anchor, over the scroll's full stored text). Distinct from
  the corpus checksum: this one covers exactly what the entry is
  about.
- **Quoted context**: `target-text:` — the anchored span verbatim
  (for an object anchor, the scroll's first line); plus
  `target-before:` / `target-after:` — up to 80 scalars of the text
  immediately preceding and following the span, omitted at scroll
  boundaries.

The quote is not decoration. It is the part of the anchor that
survives the anchor's death: when re-location fails, the reader still
sees *what words* the entry addressed (§2.4, step 6). It is also a
disclosure: minting an anchor quotes base text into another file,
and §7 prices that honestly.

### 2.4 Re-location: when the base has drifted

An editor holding base edition *k+n* and an anchor pinned to edition
*k* runs, per anchor:

1. **Same edition.** If the held copy verifies (LAYER2 §4.2 step 4)
   as the pinned edition, resolve directly: object table lookup (or
   the `at:` coordinate), offsets as recorded, hash check over the
   span. Outcome `EXACT`.
2. **Key first.** If the edition differs and the anchor names an
   object, look the key up in the held edition's object table —
   keys are edition-invariant, so this finds the object's current
   scroll without the mapping chain. For an `at:` anchor (no key),
   transport the coordinate through the editions' mapping scrolls
   (LAYER2 §5), Θ(E) table reads for E editions in the chain.
3. **Verify at the recorded offsets.** Hash the text now at
   [start, end) in the found scroll. A match is outcome
   `RELOCATED-VERIFIED`: the object moved; the words did not.
4. **Search the scroll.** On mismatch, search the found scroll for
   the quoted span text. Exactly one occurrence → outcome
   `RELOCATED-MOVED`, new offsets recorded in the resolution
   receipt. Several occurrences → disambiguate with
   `target-before`/`target-after`; a unique context match wins.
5. **Fail loudly.** No occurrence, no unique context match, the key
   tombstoned, or the scroll gone: the anchor does not re-locate,
   and the editor must not guess — mutable's own house rule for
   external definitions ("unresolved/ambiguous is never a guessed
   value") is adopted here as profile law.
6. **Render the failure, don't bury it.** A failed entry renders in
   a detached list attached to the view (never silently dropped,
   never re-attached by heuristic), showing its kind, its author,
   its failure class, and its quoted target text. The comment
   outlives its anchor *as a comment about quoted words*; what it
   loses is placement, not existence.

Failure classes: the address half inherits LAYER2 §4.3's nine
(`MALFORMED` … `KIND-MISMATCH`) unchanged. The span half adds two,
in the same spirit — loud, located at the entry, each naming its
repair:

| Failure | Meaning | Repair it names |
|---|---|---|
| `ANCHOR-LOST` (reason: `span-absent` / `span-edited` / `key-retired` / `scroll-absent`) | the target was found as an object, or not at all, but the anchored words cannot be recovered in the held edition | re-anchor by hand against the new edition, or let the entry stand detached |
| `ANCHOR-AMBIGUOUS` | the quoted span, with its context, matches more than one position | the entry's author narrows the quote or widens the context |

Receipts: every re-location produces a resolution receipt in
LAYER2 §4.2 step 6's form, extended with (outcome, old offsets, new
offsets). Costs: step 3 is Θ(span); step 4 is Θ(target scroll
bytes); a whole stack against a drifted base is Θ(sum of distinct
target scroll bytes + entries) — re-location never requires reading
the whole base.

## 3. The standoff document format

### 3.1 A standoff document is a phextura file

No new container. A standoff document is authored under DESIGN.md
with the profile's allocation of the standard divisions, declared in
its manifest:

| Collection | Contents in a standoff document |
|---|---|
| 1 (Body) | Annotation entries — one scroll per entry |
| 2 (Bibliography) | Works the entries cite (ordinary bibentries) |
| 3 (Media) | Media carried by entries (rare; hoisted as usual) |
| 5 (Supplements) | The **events ledger** (§3.4): honouring, withdrawal, supersession — append-only |
| 6 (Metadata) | Identity and the `standoff-of` declaration (below) |

Collection 6 metadata must carry:

```
standoff-of: px2:<base work reference>
standoff-profile: 0.1
authors: …
```

A standoff document is L2-enabled like any file: it has its own
work reference, its own editions, its own object table. Entries are
its keyed objects, under the extension kind `annotation` (LAYER2
§1.5's open vocabulary: declared in the manifest, opaque to
resolvers that don't know it, understood by anything claiming this
profile). Entry keys are minted once and never reused, exactly as
LAYER2 §1.4 requires of all keys — an entry's address
(`px2:corpus:onsager-review#annotation:uniform-consistency-gap`) is
how replies, receipts, and honouring records name it forever.

### 3.2 Entry structure

One entry, one scroll in Collection 1, role line `@annotation`,
header lines following DESIGN's plain-line discipline:

```
@annotation
kind: commentary
target: px2:arxiv:2611.08794@e:cec31e46e2ff1f1c#theorem:uniform-consistency@412..517
target-hash: xxh3-128:0123456789abcdef0123456789abcdef
target-text: "…the anchored words, verbatim…"
target-before: "…up to 80 scalars before…"
target-after: "…up to 80 scalars after…"
author: A. Reviewer
created: 2026-11-11T09:15:00-06:00

The body of the comment: ordinary prose, as many paragraphs as the
entry needs. References inside the body are ordinary phextura
references — coordinates intra-file, px2 addresses cross-file.
```

Header discipline:

- `kind:` — one of §3.3's kinds. Required.
- `target:` — one anchor (§2). Required, except for entries whose
  target is the base as a whole, which use the base's `#paper:`
  object.
- `target-hash:`, `target-text:` — required (the survival kit,
  §2.3); `target-before:`/`target-after:` required unless the span
  touches a scroll boundary.
- `author:` — required; a name, or the author's L2 `author:`
  address where one exists (identity caveats are LAYER2 §9.3's,
  unchanged: a name here is a self-assertion until anchored
  elsewhere).
- `produced-by:` — required for machine-generated entries: the
  program, its version, and, for persona wakes, the wake's instance
  identifier (persona-alignment A7's per-wake receipts ride in this
  line and its companions). Human entries omit it; its absence is
  the human claim, and §5.2's labelling rule — machine review is
  labelled simulation everywhere it appears — is carried by the
  `simulation: true` line persona entries must include.
- `in-reply-to:` — optional; the L2 address of another entry (in
  this or any standoff document). This is DESIGN §5.2's "author
  replies are scrolls in the same space, pointing back," formalized.
- `created:` — required; ISO timestamp. (The phextura corpus has no
  other clock; timestamps are claims by the entry's author, and
  provenance discipline treats them as such.)

Entries are immutable once their containing edition is published:
a correction to an entry is a new entry (with `in-reply-to:` or a
supersession event in the ledger, §3.4), never an edit — the same
add-never-silently-edit rule DESIGN §3 sets for warrant edges,
applied to the annotation layer itself.

### 3.3 Entry kinds, and what they subsume

| Kind | Carries | Relation to what exists |
|---|---|---|
| `commentary` | Human marginalia: remarks, questions, objections | **Subsumes** DESIGN §5.2's annotation corpus for human review |
| `persona-note` | Machine-generated review in a distilled persona's register | **Subsumes** §5.2's machine review; a persona corpus *is* a standoff document whose entries are all of this kind, with A7 receipts and the sealed lane (§4.4) |
| `warrant-evidence` | A proposed warrant edge for a claim in the base (below) | **Extends** the warrant graph from outside, without writing to it |
| `correction-proposal` | A defect report with proposed replacement text | New; the input to the acceptance path (§5) |
| `reply` | A response to another entry | **Formalizes** §5.2's reply scrolls |

**Warrant evidence, precisely.** An entry of kind
`warrant-evidence` targets a claim-bearing scroll in the base and
carries, in its header, one or more lines in DESIGN §3's edge
grammar, prefixed to mark their status:

```
proposes-warrant: citation px2:doi:10.1214/09-SS054 …
proposes-warrant: refutation px2:corpus:…#claim:…
```

A proposed edge is **not** a base edge. It does not enter the base's
warrant graph, it is not validated as one, and a view that renders
the base's warrant status must either omit proposed edges or render
them visibly distinct (the venue's display policy decides; the
profile forbids rendering them indistinguishably). P11's division of
labor is preserved and extended: the author asserts base edges,
checkers validate them, venues set display policy — and outsiders
may *propose*, in the open, with their names on the proposal. The
only promotion path from proposed to base edge is acceptance (§5).
A refutation from outside likewise begins as a proposal; the base
gains a `refutation` edge only when its keeper honours one.

### 3.4 The events ledger

An entry's lifecycle is data, kept in the standoff document's
Collection 5 as an append-only ledger, in the style of LAYER2
§4.1's object ledger:

```
honoured: annotation:uniform-consistency-gap in px2:arxiv:2611.08794@e:<new edition prefix> by <keeper> <date>
honoured-as-modified: annotation:… in px2:… by <keeper> <date>
declined: annotation:… by <keeper> <date> — <reason, or a reply entry's address>
withdrawn: annotation:… by <its author> <date>
superseded: annotation:… by annotation:<successor key> <date>
```

An entry's current state — open, honoured, declined, withdrawn,
superseded — is a **view** computed from the entry plus the ledger,
never a field edited in place. Recording an event is an edit to the
standoff document, hence a new edition of it; the ledger's
append-only discipline is what makes that edition's diff exactly the
new events.

## 4. Layering and composition semantics

### 4.1 The reading set

A composed view is a function of exactly three inputs: **one base
edition**, **a declared stack** of standoff documents (each at a
stated edition), and **a display policy**. Given the same three,
any conforming editor produces the same placement of entries against
anchors (rendering style remains the editor's and the venue's).
Composition never transits: a stack names its documents explicitly,
and second-order documents (§4.5) are included only by being named —
there is no closure to compute and no cycle to fear.

### 4.2 Ordering

- **Within the view**, entries are placed by their anchors: base
  traversal order (DESIGN §7), then span start. The stack's internal
  order confers **no authority and resolves no disagreement**; it is
  a tie-break for margin real estate only (which margin note prints
  above which), and a view must be able to disclose it.
- **Across documents**, no entry outranks another by origin. The
  base author's own notes, a persona's shredding, and a venue's
  overlay are three voices in one margin; the view labels each with
  its document and author, and stops there.

### 4.3 Conflicts are data

Two entries over one anchor may assert incompatible things — a
persona objects, the author replies, a second persona objects to
the reply. The composition rule is the Volumen rule (MR-M2), now
general: **the view surfaces the disagreement; no layer order, sync
policy, or timestamp may silently settle it; last-writer-wins is
forbidden at every level of the system** — across documents, and
within one (entries are immutable, §3.2; the ledger is append-only,
§3.4). Where a venue's display policy *scores* a paper partly by
its chosen panel (Will's ruling, 2026-11-11: a paper that chooses a
wet noodle is scored lower), that scoring reads the stack as data;
it still does not edit it.

What an editor **must** render or account for:

- every entry in the stack whose anchor resolves (at its anchor) or
  fails (in the detached list, §2.4 step 6) — nothing in the stack
  may vanish silently;
- the identity of each layer (document, author, edition) and, for
  machine entries, the simulation label;
- any layer the user's display choices are currently hiding, named
  as hidden (a view may collapse a layer; it may not pretend the
  layer is absent).

What an editor **may** hide or fold: whole named layers, honoured
entries (shown folded by default, §5), and detached entries behind
a count — provided each is disclosed as above.

### 4.4 The sealed lane

A standoff document may carry material its keeper holds apart —
calibration records for a living persona subject are the standing
example (A10). Under this profile the lane is structural: sealed
material lives outside the standoff document's entry space (in the
keeper's own corpus), referenced by pointer only, and no conforming
composition ingests it. A persona's marginalia never include its
sealed lane, and a paper's author reading the marginalia cannot
reach it through the view.

### 4.5 Second-order standoff

Anchors target any phextura file, including standoff documents:
review of the review, a keeper's replies collected as their own
document over the reviewer's, a venue's overlay on a persona's
marginalia. Nothing in the format changes — `standoff-of` names
whatever file is annotated. This is also how a keeper declines at
length when they cannot write in the reviewer's ledger (§3.4): the
decline-with-reasons lives in the keeper's reply document, and the
ledger entry, if any, points at it.

## 5. The acceptance path

Standoff is not a comment ghetto: what it proposes can become part
of the work — by exactly one route.

1. **Only the base's keeper accepts.** Acceptance is an act of the
   base's authorship, never of the annotator, the editor, or a sync
   process. An editor conforming to this profile offers no "apply
   to base" operation that bypasses the keeper producing a new
   edition.
2. **Acceptance produces a new edition of the base.** The keeper
   incorporates what is honoured — the correction-proposal's text
   applied (or adapted), the warrant-evidence's proposed edge added
   as a base edge with its validation run per DESIGN §3, the
   commentary's point answered by revision. The new edition is an
   ordinary edition: new manifest checksum, P1 mapping scroll if
   anything re-based.
3. **The receipts are bilateral.** The new edition's Collection 6
   provenance records, per honoured entry:
   `honours: px2:<standoff address>#annotation:<key>`. The standoff
   document's ledger records the mirror line:
   `honoured: annotation:<key> in px2:<base>@e:<new edition> by
   <keeper> <date>` (§3.4). Either side alone is a claim; together
   they are checkable: the edition diff (P1 mapping plus text
   comparison between editions) shows *what changed*, the receipts
   name *which annotation it honours*.
4. **Partial acceptance is first-class.** A keeper who takes a
   correction's substance but not its wording records
   `honoured-as-modified`; the diff between editions is the record
   of the modification, and no further machinery is claimed.
5. **The annotation is honoured, not deleted.** After acceptance the
   entry remains in its standoff document, addressable forever, its
   state view now reading *honoured*. Views fold honoured entries
   by default; the record — who saw the defect, when, and what the
   keeper did about it — is provenance (P10), and provenance is not
   pruned.

## 6. Conformance checklist — the first editor (mutable)

Each requirement is testable; each is tagged with its state against
§0's verified facts: **[present]** the machinery exists in mutable
today, **[partial]** the machinery exists but the profile's use of
it is new, **[to build]** nothing in the sources read provides it.
Nothing below assumes an untagged capability.

- **M1 — Separate documents, untouched base. [to build, over present parts]**
  Opening a base `.phext` and a standoff `.phext` yields two
  Documents. Annotating, in any amount, leaves the base file's bytes
  unchanged (test: checksum the base file before and after an
  annotation session; identical). Mutable's import/export already
  treats each phext as its own Document; the composition of two into
  one reading view is the new work.
- **M2 — Coordinate identity only. [present]**
  Anchors reference coordinates from block `metadata.phextCoordinate`
  and L2 keys — never mutable block UUIDs, which are re-minted at
  every import (§0). Test: export both files, re-import, compose;
  every entry still places.
- **M3 — Edition identity. [to build]**
  At open, the editor computes the base file's corpus checksum and
  reads its manifest checksum, storing both beside
  `phext: {version, sourceFilename}` (whose current shape has no
  checksum field — §0). Test: the stored checksum equals an
  independent libphext checksum of the file.
- **M4 — Mint anchors without touching the base. [partial]**
  A text selection inside a base scroll mints an entry in the
  standoff Document: anchor per §2.2, survival kit per §2.3. The
  selection → scalar-offset conversion (from Cell/UTF-16 ranges;
  §2.2's constraint) must round-trip exactly, including astral-plane
  characters — test with a scroll containing emoji and combining
  marks, verifying hash equality after a save/reload cycle.
  Mutable's `TextRanges` snapshot/validate discipline is the model;
  the scalar coordinate system is new code.
- **M5 — Resolve and verify. [to build]**
  On composing a view, every anchor is resolved per LAYER2 §4.2 and
  §2.4 of this profile; the span hash is verified before placement.
  A hash mismatch at the recorded offsets triggers re-location, not
  placement. Test: hand-corrupt one scroll in a copy of the base;
  the affected entries land in the detached list, none renders at
  the old offsets.
- **M6 — Report anchor rot. [to build]**
  Opening a stack against a base edition other than the pinned one
  produces a rot report before rendering: counts by outcome
  (`EXACT`, `RELOCATED-VERIFIED`, `RELOCATED-MOVED`, `ANCHOR-LOST`
  by reason, `ANCHOR-AMBIGUOUS`), listed per entry. Mutable's
  annotation monitor is the obvious surface; the report itself is
  new.
- **M7 — Render the composition honestly. [partial]**
  Margin rendering, the annotation monitor, and the marginalia
  feature provide the surfaces. The profile's additions are policy,
  and they are requirements: detached entries visible with quoted
  text and failure class (§2.4 step 6); hidden layers disclosed by
  name; machine entries labelled as simulation (§4.3).
- **M8 — Never guess. [present, as house rule]**
  Mutable's linked-annotation resolver already refuses unresolved
  or ambiguous external definitions rather than guessing a value.
  The profile adopts the rule for anchors (M5/M6) and asks the
  implementation to keep the two paths consistent — one
  no-guessing discipline, visibly the same in code review.
- **M9 — Export a standoff document. [partial]**
  A standoff Document exports through Export Phext as a valid
  phextura file in §3.1's layout. Caveat, from §0: today's export
  collects text blocks and preserves coordinates, so entries
  authored as one text block per entry with header lines as text
  should round-trip — but this must be *verified*, not assumed:
  test that an exported standoff file, re-imported, reproduces
  every entry's header lines byte-for-byte. If export proves lossy
  for entry structure, that is an implementation finding to fix in
  the exporter, not a profile change.
- **M10 — Honour, don't apply. [to build]**
  The editor provides no operation that writes an annotation into
  the base (§5 step 1). What it may provide: an export of the
  honouring receipts (the `honours:` lines) for the keeper to place
  in the new edition's manifest, and the ledger lines for the
  standoff document. Test: with a standoff document open, no menu,
  command, or drag operation mutates the base Document's repository
  state.

**Open questions for the implementation phase** (unverifiable from
the sources read; recorded, not resolved): (a) whether mutable's
`externalDefinition` gate should gain a pinned-version path so
linked definitions and standoff anchors share one resolver, or
anchors get a separate one (§0 notes the gate resolves only
unpinned externals today); (b) whether a Document can hold a live
reference to another Document's repository state, or composition
must run over two independent imports (the OVERVIEW's projection
model suggests the latter, but only the code's authors can say what
it costs); (c) whether durable block history (feature-flagged off)
should be the carrier for rot reports across sessions.

## 7. Costs and honest limits

- **Anchor maintenance under heavy editing.** Pins make old anchors
  exact and increasingly stale; re-location is one search per moved
  anchor (§2.4), and a heavily rewritten base will produce
  `ANCHOR-LOST` in bulk. That is the design being honest, not
  failing: the profile forbids fuzzy re-attachment, so recall is
  traded for the guarantee that a placed annotation is *on its
  words*. Key-based object anchors survive restructuring far better
  than spans; tooling should prefer them and mint spans only where
  the comment is truly about the words.
- **Very large scrolls.** A span search is Θ(scroll bytes) per
  anchor; phextura's fine-grained scroll norm keeps that small in
  practice, but nothing in the profile bounds a scroll's size, and a
  monolithic scroll (a whole chapter in one scroll, say) makes its
  anchors pay chapter-sized searches. The survival kit's bounded
  context (≤80 scalars a side) keeps entries proportional to their
  spans, not their targets.
- **What standoff cannot do.** Anything requiring base mutation:
  fix the typo, add the edge, renumber the section. All of it
  routes through §5 — a keeper, a new edition, receipts. A workflow
  that needs annotation to *be* editing needs a different tool, and
  the profile says so rather than growing an apply-path that would
  dissolve the invariant.
- **The quote is a disclosure.** `target-text:` copies base words
  into the standoff file. For a published base that is nothing; for
  an unpublished draft under review it is a real, if small,
  exposure, and the kit's minimal form (hash + first line +
  bounded context for object anchors) exists so the exposure is a
  choice, made per profile configuration, with eyes open.
- **Interaction with the tracker's R13 gap.** The tracker-as-
  phextura shadow still carries its §4-derived registry lists as
  wholesale replaces from a conversion (friction R13: the
  edge-derived registry extractor does not exist yet). Every such
  replace is edition churn concentrated in derived scrolls, and
  anchors pinned into those scrolls rot by construction — they
  point at text whose *generation*, not whose meaning, changed.
  Until R13 is fixed, conforming tooling **should warn** when an
  anchor targets a scroll in a subspace the base's manifest
  declares derived (the object-table index is already declared
  derived under LAYER2 §4.1; registry lists should be declared the
  same way), and annotators should anchor to the authored scrolls
  the derivation reads, not the derived lists. The profile does not
  fix R13; it refuses to let R13's churn masquerade as anchor rot
  in the base's authored text.
- **Real-time sync (QEXO).** Will has ruled that QEXO will support
  writing and syncing phextura in real time. Under sync, §2.4's
  re-location stops being an open-time audit and becomes a
  continuous predicate evaluated as editions land — the persona
  analysis (§4 of persona-volumen-alignment.md) reaches the same
  conclusion for marginalia. The profile's procedure is written to
  be run per-anchor, per-edition, so it composes with sync; the
  transport, wake policy, and its inference costs are QEXO's, and
  DESIGN §10's exclusion of concurrency semantics from the format
  stands.

## 8. Conflicts and proposed amendments — not applied

Nothing below is in force. Each item names its target, the change,
and its size. Adoption is Will's, in the usual way.

**DESIGN.md**

1. **§5.2 — replace the pointing rule with this profile's anchor
   model.** Annotation corpora currently point at (manifest
   checksum, coordinate); under the profile, keyed targets use L2
   object addresses (durable across re-bases), and the
   (checksum, coordinate) form survives as the pinned `at:`
   selector for unkeyed scrolls. §5.2's "annotation corpus" is
   renamed by reference to *standoff document*, and §5.2 gains a
   pointer to this profile for kinds, layering, and acceptance.
   This item **supersedes LAYER2 §10 item 4's proposal**, which it
   refines: L2 pointing becomes the primary form rather than the
   only form, because unkeyed scrolls exist and §5.2's form is the
   honest way to point at them. *One section reworked.*
2. **§3 — proposed warrant edges.** Add a paragraph: edges may be
   proposed from standoff documents (`proposes-warrant:` lines in
   `warrant-evidence` entries); proposed edges are not base edges;
   display must distinguish them; acceptance (§5 of this profile)
   is the only promotion path. *One paragraph.*
3. **§2 — standoff division allocation.** Note that standoff
   documents allocate the standard collections per this profile
   §3.1 (entries in 1, events ledger in 5, `standoff-of` in 6). No
   layout change. *One bullet.*
4. **§10 — scope note.** The exclusion of collaborative editing
   semantics stands for concurrent *base* editing; annotation-side
   composition is now specified (this profile). *One bullet.*

**LAYER2.md**

5. **§3 — fragment grammar extensions.** Add the span suffix
   (`@start..end`, scalar offsets, half-open) and the `at:`
   coordinate selector (pin required) to the object-ref grammar;
   add `annotation` to the extension kinds named in §1.5's note as
   this profile's declared kind. *One grammar block, one sentence.*
6. **§4.3 — anchor failure classes.** Add `ANCHOR-LOST` (with
   reason tokens) and `ANCHOR-AMBIGUOUS` beside the nine, or a
   cross-reference to this profile §2.4's table. *Two rows or one
   pointer.*
7. **§5 — mapping scrolls serve re-location.** Note that the P1
   mapping chain is also the transport for `at:` anchors under
   drift (this profile §2.4 step 2); no semantic change. *One
   sentence.*

**PRINCIPLES.md**

8. **P13 — name the third relation.** LAYER2 §10 item 8 proposes
   splitting composition into binding and citing; this profile adds
   *annotating* — files standing off other files, composing at read
   time as views, changing neither party — and proposes P13 name
   all three, with a sentence carrying A10's rule in general form:
   read-time composition of annotation layers is a view, not a
   merge. *One paragraph.*

**No conflicts found** between this profile and: P1 (the profile
consumes edition identity and mapping scrolls as specified), P10
(receipts are provenance in both directions), P11 (the warrant
graph's ownership is narrowed, not widened), P12 (costs stated in
its format), or the L2 failure philosophy (extended, not
contradicted — §2.4).

## 9. Dependencies on unbuilt machinery — flagged, not papered over

- **The phextura validator does not exist as code.** DESIGN §7's
  validation pass (reference resolution, checksum verification,
  warrant checking) is a specification; anchor verification (M5)
  inherits that gap entire.
- **No L2 resolver exists.** LAYER2 is DRAFT v0.1; no corpus —
  including the demo corpus and the tracker — is L2-enabled today:
  no `key:` lines, no object tables, no manifest checksums in the
  L2 sense. Every anchor in this profile is therefore spec-level
  until L2-enablement tooling lands, and the first standoff
  documents will be minted against bases whose object tables are
  created in the same act. The `at:` selector (§2.2) is the bridge:
  it needs only a pin and a coordinate, both available today.
- **The mutable side is unbuilt** per §6's tags: edition checksums
  in phext metadata (M3), scalar-offset conversion (M4), stack
  composition (M1/M7), rot reporting (M6), and the honour-receipt
  export (M10) are implementation work, sized by §0's facts but not
  estimated here — the delight/benchmark discipline (P14) prices
  editor work, and this profile does not pre-empt it.
- **QEXO real-time sync is announced, not built** (§7). The profile
  is correct without it; it does not depend on it.

— End of draft. —
