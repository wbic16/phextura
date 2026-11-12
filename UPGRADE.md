# phextura — the upgrade

**Status: the focus document — 2026-11-11.** Task coordinate
29.6.9/8.7.13/6.6.9. Originating direction (Will Bickford, 2026-11-11):
*"let's focus on the upgrade that feels like latex joined the 21st
century, just like .txt -> .phext".*

This document states phextura's centre of gravity. DESIGN.md states
the model, PRINCIPLES.md derives it, LAYER2.md and STANDOFF.md extend
it — but when any phextura document conflicts with another **on an
author-facing choice**, this one governs. Machinery serves the
feeling, never the reverse.

## 1. The claim

Phextura is to LaTeX what phext is to plain text. Not a replacement
species and not a negation: the same authorial DNA — prose, structure,
mathematics, citation — carried in a medium that has the century's
capabilities built in instead of simulated on top. A phextura paper is
an addressed corpus of plain-text scrolls: its parts have coordinates,
its references resolve instead of rerunning, its annotations pin to
addresses and cannot drift, its media and evidence live inside it as
scrolls, its claims carry their warrants as data, and machines read
the same artifact humans read, without executing anything. The
feeling test, stated plainly: **a LaTeX user should feel at home in
the first five minutes and feel the century in the first hour.** The
project's motto is the same test in two words — *Rigorous Delight*
(PRINCIPLES.md) — and this document is where the motto stops being a
grade and becomes the plan.

## 2. The analogy, worked point by point

The analogy is earned below or not at all. Each row states what
`.txt -> .phext` actually did (grounded in incipit.phext, the root
corpus: thoughts "organized across 6 dimensions" with three left
unused for the reader, 7,594 scrolls at 9-component coordinates), the
`.tex -> .phextura` counterpart, its design citation, and its true
state. State labels mean: **running** — a real corpus or program
exhibits it today; **designed** — specified in the design record, not
yet built as a general tool; **designed-not-built** — specified, with
no running instance; **not-yet-designed** — named as open.

| # | .txt → .phext | .tex → .phextura | State |
|---|---|---|---|
| 1 | The text stays plain text. Every scroll's content is text any editor can open; incipit's Scroll of LIFE: once opened, a scroll is "a normal (unbounded) plane of text" — the lattice is the addressing, never a cage on the content. | A phextura block's content stays plain text, readable raw; the corpus is data, executed by nothing (DESIGN §1, §2; PRINCIPLES P4). | **Running** — the demo corpus, the research tracker, and the templates corpus are plain-text phext files, read raw daily. |
| 2 | Every part gains an address: one file's worth of text becomes scrolls at coordinates in a 9-dimension lattice (Library.Shelf.Series / Collection.Volume.Book / Chapter.Section.Scroll). | Every part of a paper gains an address: chapters, sections, blocks, theorems, equations, figures, bibliography entries are scrolls at coordinates (DESIGN §2). | **Designed, demonstrated** — the demo paper (arXiv:2611.08794) sits in 1,154 addressed scrolls. |
| 3 | Files and folders become one lattice: a phext holds many scrolls, grouped by its coordinate dimensions instead of a directory tree. | The paper becomes one corpus with six divisions — Body, Bibliography, Media, Styles, Supplements, Metadata (DESIGN §2) — instead of a directory of `.tex`, `.bib`, figures, and side files. | **Designed, running** — all three corpora above use the layout; the tracker lives in it. |
| 4 | Find-in-file (a linear scan) becomes navigation: fetch by address, derived maps (`textmap`, `navmap`, `index`) computed from the corpus. | Contents, lists of figures, the index, and the reference graph are traversals of the corpus, not side files a program regenerates (DESIGN §7, §8). Dangling references are exactly the `navmap` targets that fail `fetch`. | **Designed, demonstrated** — the demo validator resolved 780 of 780 written references, 0 dangling. |
| 5 | The file stops being the unit: phexts compose — `merge`, `subtract`, `offset` are operations on whole corpora. | Papers compose two ways: **binding** (merge into proceedings, P13) and **citing** (files refer to each other's objects by global L2 address while staying separate files — "composition is a relation among files, not a fusion of files," LAYER2 §6). | Binding: **designed** (libphext operations). Citing: **designed-not-built** — LAYER2 is a draft spec; no corpus is L2-enabled yet (LAYER2 §9.5). |
| 6 | Nothing about text itself changes: same characters, same act of writing; phext adds addressing *around* text, never a new kind of text. | Nothing about writing itself changes: the author still writes prose, structure, and mathematics as text; an untagged block is `@prose` explicitly (DESIGN §2). | **Designed** — with one honest exception: mathematics *entry* is the exception row (§3). |
| 7 | Any program can read a .txt without running it; a phext inherits this — the corpus is parsed, never executed. | Any program can read and validate a paper without executing it: structure is data (role lines), references are decidable text, validation is a stated number of traversals (DESIGN §7; PRINCIPLES P4, P8). | **Designed, demonstrated** — validators run green on the demo corpus and on the living tracker corpus in both its forms. |
| 8 | A text file's identity was its path; copies drifted silently. Phext adds content checksums (`checksum`, xxh3-128) so a copy can verify itself. | A paper's identity is the pair (edition, address): the edition is the corpus root plus its manifest checksum, copies self-verify against it, and a re-base emits a mapping scroll so old citations resolve forward (PRINCIPLES P1; LAYER2 §4.2, §5). | Checksums: **running** (payload verification in the demo; every templates source scroll hash-verified at import). Edition machinery: **designed-not-built**. |

**Where the analogy stops — stated, not smoothed over.** Three
places the parallel does not reach, and this document claims none of
them. (i) Plain text never promised typesetting, so phext never
needed a layout engine; phextura does, and has none yet — the layout
constraint vocabulary is an open requirement and every layout claim
is UNPRICED (PRINCIPLES P6, demoted; DESIGN §6, §10). The printed
page is the one place "joined the 21st century" is not yet true.
(ii) A .txt and a .phext are read by the *same* editors; a phextura
paper's raw scrolls are readable anywhere, but its *rendered* reading
— traversal, display numbers as views, resolved cross-file labels —
needs phextura-aware tooling that is mostly not built (the demo
renderer was a throwaway harness, not phextura). (iii) Phext needed
one address scheme; phextura needs two layers (L1 positions inside a
file, L2 denotations between files, LAYER2 §1.1) because research
objects outlive their positions. The analogy's cleanest row
(addresses) is therefore really two mechanisms, one of them unbuilt.

## 3. Continuity: what a LaTeX user keeps

The upgrade is worthless if it costs a LaTeX user their world. What
they keep, concretely:

- **Their classes are already here.** The CTAN class topic was
  imported whole into one phextura corpus — 449 packages enumerated,
  **595 templates** from 398 packages (51 gaps, each named with its
  reason), every source scroll preserved verbatim with its package,
  version, and license — at `templates/templates.phextura` in this
  repository (wbic16/phextura@exo, commit `012053f`). A template's
  header speaks the user's vocabulary: the literal
  `\documentclass[...]{...}` use line, documentclass options, the
  sectioning ladder, the environments provided. In phextura a paper
  names a style scroll from Collection 4 (DESIGN §6); the corpus is
  where those styles live, cited and preserved instead of found by
  filename search.
- **Their mental model maps one-to-one.** The templates corpus's home
  scroll (1.1.1/1.1.1/1.1.1) states the mapping, and this document
  adopts it: document → one corpus of block scrolls; class →
  Collection 4 style; preamble → Collection 6 metadata and style
  choices at stated addresses; title/author/abstract → Collection 6;
  sections → the paper's own coordinates; figures and tables →
  Collection 3, one float at one address, caption travelling with it;
  bibliography → Collection 2, one entry per scroll, the short key
  the author types resolving to a durable address underneath.
- **Their learning path is the familiarity baseline.** *Learn LaTeX
  in 30 minutes* (Overleaf; cited in the templates corpus home
  scroll) walks: first document → `\documentclass` → preamble →
  title/author/abstract → sectioning ladder → figures with captions,
  labels, references → lists → mathematics → tables → packages via
  CTAN. If a phextura surface contradicts what minute 30 of that
  tutorial taught, that is a design smell — unless the difference is
  the point and says so (§6).
- **Mathematics is the one deliberate deferral, stated in one
  line:** equation entry syntax is not yet designed — the notation
  waits until the Bickford Sentron core machine design settles,
  because phextura refuses to fake familiarity in the one place the
  upgrade must be real (DESIGN §10; PRINCIPLES, R1–R4 are the
  requirements any candidate must meet). Until then, stored math is
  verbatim LaTeX (friction F1) — familiar, and labelled interim.

## 4. The century: what the upgrade adds

Each addition is a pain a LaTeX user already has (documented in the
project's own record — the demo friction log F1–F11, the warts
analysis behind DESIGN §9), the phextura mechanism that answers it,
and its true state.

1. **Rerun-until-it-settles, and `??` in the PDF.** Labels are
   strings, counters live in side files, and a reference is a hope
   until the third pass (warts W4, W5, W27). → References *are*
   coordinates; validation scans the corpus's non-opaque scrolls
   and reports any dangling reference as a lint error at the scroll
   that contains it (DESIGN §3, §7). Display numbers are a
   render-time view, never stored, so inserting a theorem renumbers
   nothing by hand (PRINCIPLES P5). **State: designed; demonstrated**
   — 780/780 references resolved in the demo; the same validation
   runs green on the living tracker corpus after every paper.
2. **Marginalia that drift.** `\marginpar` and its kin are floats:
   the note goes where the page allows, not where its sentence is,
   and a referee's "page 7, line 3" rots at the first revision. →
   Annotation is *standoff*: a separate document whose entries pin
   to the base by L2 address plus span selector, carrying an
   edition pin, a hash of the anchored text, and its quoted context,
   so an editor re-locates the anchor after drift — or fails loudly
   (`ANCHOR-LOST`, rendered detached, never silently re-attached).
   Composition is a read-time view; the base is never modified
   (STANDOFF §§1–2, §4–§5). **State: designed-not-built** — the
   profile is drafted (v0.1); the first editor (mutable) has the
   intra-document standoff machinery but none of the cross-document
   conformance items yet (STANDOFF §0, §6).
3. **Macro soup with no home.** A paper's own preamble macros dangle
   semantically: no collection holds them, so stored mathematics is
   not self-contained even when every coordinate resolves (F1). →
   Macros do not exist in phextura: repeated *content* is a scroll
   other scrolls reference; repeated *computation* is code in the
   renderer (DESIGN §6). **State: designed, with an open half** —
   the negative (no expansion in the artifact, P4) is settled; the
   positive requirement F1 forced — a declared corpus home for a
   paper's own definitions, so stored math is self-contained math —
   is recorded in DESIGN §10 and rides with the deferred math
   design (§3).
4. **Floats that split in two.** A converter that gives a figure two
   identities loses references silently — the demo's first converter
   did exactly this, and 27 in-text references landed on placement
   markers instead of figures (F4). → One float, one address: the
   float is its media scroll; placement in the argument is a
   *marker* naming that address, never a second citable identity
   (DESIGN §3). **State: designed** — normative since the demo bit;
   the rule, not the tooling, is what exists today.
5. **A claim's evidence living in a different universe.** The
   dataset is a path, the code is a repository, the figure is an
   output nobody can re-derive — the paper points out of itself at
   things that may not be there at render time (warts W18, W19). →
   Evidence lives *inside* the corpus: media and supplements are
   scrolls, binary hoisted as base64 under a header whose checksum
   fails before anything renders silently missing; evidence scrolls
   carry **produced-by edges** (code coordinate + checksum, input
   checksums, environment, output checksum at generation time), and
   the paper is closed relative to its manifest — anything neither
   in the corpus nor in the manifest is a mechanically detectable
   violation (DESIGN §5; PRINCIPLES P3, P10). Claims likewise carry
   their warrant as edges on the claim scroll itself (DESIGN §3;
   P11). **State: designed; halves demonstrated** — hoisting and
   checksum verification run today (demo payloads; the templates
   corpus is a single 27.8 MB file holding 964 verified source
   scrolls); the full provenance graph is specified, and earning
   the stronger name *reproducibility weave* awaits its three
   conditions (PRINCIPLES, commitments §2).
6. **Correction by new PDF, with no memory.** A revised paper is a
   fresh artifact; what changed, what was believed before, and which
   objection a fix answers live in email, if anywhere. → Identity
   is (edition, address): a re-base creates a new edition and emits
   a mapping scroll, so citations resolve forward mechanically
   (P1; LAYER2 §5); warrant edges are *added, never silently
   edited* — a `refutation` edge supersedes by addition, so the
   history of what was believed, when, and why it changed is itself
   corpus data (DESIGN §3); a standoff note an author honours enters
   the base only as a new edition, receipted in both files
   (STANDOFF §5). **State: designed-not-built** for the edition
   machinery on papers; the warrant-history-by-addition half is
   living practice in the tracker corpus.

## 5. What this focus deprioritizes

The following stay necessary and move *behind* the author-facing
surface — behind import and conversion of real papers, reference
validation an author can feel, the standoff view in a real editor,
and the benchmark suite that measures the first hour (PRINCIPLES
P14: the suite belongs to the focus, because it is how the feeling
gets *measured* rather than asserted):

- **Registry-extraction internals** — deriving the research
  tracker's registry from warrant edges instead of carrying it as
  replaced lists (the shadow's standing R13 debt). Real debt,
  invisible in an author's first hour.
- **Warrant invalidation mechanics** — how the add-only warrant
  graph represents a standing's later invalidation (the L8 gap:
  the tracker's False→Unknown transition). The graph's honesty
  depends on it; no author's first hour does.
- Machinery of the same class, named so the class is recognizable:
  mapping-scroll *emission tooling*, the catalog format as a
  specification of its own (LAYER2 §10), the revision-laws
  verification owed under P9.

And the anti-goal, stated once: **phextura never wins by feeling
foreign-clever.** Where a choice trades familiarity against power,
familiarity wins ties. A capability an author cannot find from the
surfaces they already know might as well not exist; a divergence
that cannot state its reason in the user's own terms is a defect,
not a feature.

## 6. The acceptance test

Every future author-facing design decision — a syntax, a layout
rule, a tool behaviour, a default — must answer three questions **on
the record** before it is adopted:

1. **Would a LaTeX user recognize this in five minutes?** (Measured
   against the §3 baseline: the class/preamble/section/environment/
   citation model and the 30-minute path.)
2. **Does it repair a pain they already have?** (Named, from the
   record — a friction item, a wart, a stated open requirement —
   not an invented one.)
3. **If it diverges, is the reason one they'd thank us for?**
   (Stated plainly, in their vocabulary, at the point of divergence
   — the way the templates corpus home scroll states its three.)

A decision that fails (1) needs (2) and (3) at full strength. A
decision that fails all three is not phextura work, however rigorous
it is: rigor alone is homework, and this project's centre of gravity
is the upgrade a LaTeX user can *feel* — at home in five minutes,
in the 21st century within the hour.
