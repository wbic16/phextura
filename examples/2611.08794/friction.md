# phextura demo — friction log

Paper: arXiv:2611.08794, "High-Dimensional Off-Policy Evaluation without Sample
Splitting using an Onsager Correction" (Jain, Braun, Imaizumi), converted from
the arXiv HTML full text into a phextura corpus exactly per
`~/workspace/phextura-design/DESIGN.md`. Every item below names the paper
construct and the design section that bent, broke, or had no answer. Items
F1–F5 are the sharpest; F6–F11 are real but second-order. "Converter" items are
defects of this demo's throwaway conversion harness, recorded here only where
they expose something about the design.

---

## F1 — Displayed mathematics: the design leaves entry OPEN (DESIGN §10); the interim inherits the disease

**Status: INTERIM, flagged as required.** Display equations are stored as
verbatim LaTeX source (LaTeXML's `alttext`) in `@equation` scrolls; inline math
is stored as `$…$` LaTeX source inside prose scrolls. This was the only
mechanical choice available (all 3,096 `<math>` elements in the source carry
`alttext`), and it exposes exactly why §10 cannot stay open:

- The stored math inherits the entire LaTeX macro surface phextura exists to
  escape. Worse, the paper's own preamble macros have *no home in the corpus
  layout*: no collection or scroll type holds paper-level macro definitions,
  so the stored math is not self-contained — `\coloneqq`-class shorthands and
  journal macros dangle semantically even though every coordinate resolves.
- `alttext` is LaTeXML's reconstruction, not the author's source. Fidelity of
  every one of the 637 display equations is bounded by a third-party
  converter; a phextura-native author currently has *no defined way to write
  an equation at all*.
- The demo renderer displays the LaTeX source as text. Human readability of
  mathematics — a headline phextura goal — is precisely what the interim does
  not deliver.
- The inline/display distinction is encoded only by position (`$…$` in prose
  vs. an `@equation` scroll). Nothing in the design states this convention.

## F2 — The role vocabulary (§2) has no home for the theorem-family kinds real papers actually use

The design's block roles are `@theorem @lemma @definition @proof @equation
@figure @caption @quote @code @footnote`. This one paper needs, in addition:
proposition (11 blocks), corollary (4), **assumption (4)**, remark (3), and
supplementary proposition/lemma (6). Assumptions are not decoration — they are
load-bearing logical objects, referenced from theorem statements exactly like
theorems ("under Assumption 2.1"). The converter mapped every missing kind to
its nearest design role and preserved the true kind only in a free-text
`kind:` line — i.e. the design's typed-block guarantee silently degrades into a
string convention the moment it meets a real paper. The design must either
grow the role set (and say who governs it) or declare roles an open vocabulary
over a small normative core.

## F3 — Headings have no defined home, and the address has one section level too few (§2, §3)

The design assigns roles to statements, proofs and equations but never says
where a section's *heading text* lives. The converter stored each heading as a
`#`-/`##`-/`###`-prefixed first block of its slot — a Markdown convention
smuggled into a design that positions itself against markup conventions; a
reader cannot distinguish a heading block from prose that begins with `#`
without out-of-band knowledge. Compounding this, the body address
(group, section, block under a Book) provides exactly one section level, so
LaTeX's §/§§/§§§ depth does not map: this paper's paragraph-level units
(`ltx_paragraph` sections) were flattened into their parent subsection behind
level-3 heading blocks, which changes the citation granularity of everything
beneath them.

## F4 — Float placement and float identity are unspecified (§3, §5) — and the ambiguity bit this demo

The design places media in Collection 3 but never states how the body says
"the figure goes *here*, mid-argument", nor that a float has exactly one
address. The converter invented a placement-marker block (a body scroll whose
entire content is the media scroll's coordinate). The hazard is not
hypothetical: the first working version of this converter briefly gave every
float *two* identities — the marker block's coordinate and the media scroll's
coordinate — and 27 in-text references ("Figure 8 shows…") silently landed on
the marker blocks instead of the figures. It was caught only because the
renderer's link check failed. A design whose central claim is "the reference
*is* the coordinate" must normatively state: one float, one address; placement
is a marker naming that address, never a second addressable block.

## F5 — Bibliography (§4) presumes structured data; the wild is semi-structured

The design's bibliography scroll (author:/title:/year:/venue: field lines)
presumes a structured source. The arXiv HTML bibliography is semi-structured:
the year lives inside the author–year tag span, venues are split across
journal/volume/pages spans, and some entries expose no parseable year or venue
at all. The converter emitted the design's fields where they were extractable
and added a full `text:` fallback line to every entry — a deviation the design
neither sanctions nor forbids. Open questions the design must answer: is a
bibliography scroll fields, text, or both? And authors are plain strings —
there is no author identity (no per-author scroll or link), so cross-paper
deduplication and disambiguation are impossible by construction.

---

## F6 — Equation groups have no unit (§2, §3)

LaTeXML `equationgroup`s (multi-row displays) are semantic units: the paper
refers to them as wholes and numbers rows with letter suffixes. The converter
made each row its own `@equation` scroll and resolved group references to the
first row's coordinate. The letter-suffixed numbering (…a, …b) cannot be
regenerated by a render-time overlay, because overlay numbers derive from
traversal order alone and the corpus records no grouping. The block model
needs either a group construct or a stated rule that groups do not exist.

## F7 — Narrative citations do not survive coordinate substitution (§3)

Parenthetical citations convert cleanly: "(1.1.1/2.1.1/1.1.9; …)". Narrative
citations do not: in "Jain et al. (2026) show…", the source anchor wraps only
"(2026)", so substitution yields prose where a bare coordinate does the
grammatical work of a year. The design treats a citation as a reference; in
narrative position it is also a noun phrase. No rule is given for either form.

## F8 — Inline typographic structure is unaddressed (§2)

`\emph`, `\textbf`, defined terms, and other inline styling are flattened to
plain text throughout the corpus. The design specifies block structure only;
like math (F1), everything *inside* a block is an undefined plain-text
convention. A reader of the corpus cannot recover which phrase the paper
italicised as a term definition.

## F9 — Footnotes: untested by this paper (coverage gap, stated honestly)

The source paper contains **zero** footnotes, so the design's `@footnote`
path went entirely unexercised by this demo. This is a gap in the demo, not
evidence about the design. Tracker Paper 012 (arXiv:2611.11959, 16 footnotes)
is the natural follow-up test; it was rejected as the primary pick because it
has no figures or tables and would have left Collection 3 unexercised instead.

## F10 — Nested addressable blocks are unspecified (§2, §3)

Labeled equations sit inside proofs and theorem statements throughout the
paper. The converter extracted each one to its own `@equation` scroll placed
immediately after its host block, and the host's text carries the coordinate
at the display's position. The design never states whether a proof's interior
is addressable content or opaque text; the alternative converter choice
(leave displays inline as `$$…$$`) produces a materially different corpus with
different reference semantics. One of them must be normative.

## F11 — "Every coordinate in the corpus" is not well-defined without the opacity rule (§5, §7)

Validation (§7's one-pass check) must exclude media payload scrolls from
reference scanning: an SVG payload can contain substrings that match the
coordinate grammar, and §5 declares binary/hoisted payloads opaque. So the
design's validation story is only coherent if
...[truncated 1093 chars]