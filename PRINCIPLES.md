# phextura from first principles

*Pass 1 of the design loop, 2026-11-11. This document derives what phextura
should be from what a paper is. The previous design (DESIGN.md) was built by
reformulating LaTeX3 — negating a list of warts. Negation inherits the shape
of the thing negated. This pass starts lower: definitions first, principles
derived from the definitions alone, phextura's commitments derived from the
principles. Analogy is banned as a justification throughout: no principle
below rests on "LaTeX does X" or "systems usually Y." Where a derivation
fails, the item is marked OPEN rather than decorated.*

## Pass 3 reconciliation (2026-11-11)

This document was attacked in Pass 2 by a Knuth-register critic
(`critic.md`) and revised in place. What changed, in summary: definition
(c) was rewritten from the sources (its demerits formula was wrong —
see critic §5, H1); P1, P2, P3, P7, P8, P9, P10, P12, P13 survive
amended; P11's status enum is replaced by a warrant graph; **P6 and
P14 die as stated** and are replaced, respectively, by an OPEN
requirement and a benchmark obligation; the costs in the commitments
table are recomputed in the P12 format; the "weave" claim is amended to
a provenance graph with stated conditions for earning the stronger
name. One rule is adopted from the critic, verbatim in effect: **every
claim about an existing system carries a citation at checkable grain,
or is struck.** Pass 1 imposed that discipline on personas and not on
itself; Pass 3 applies it at home.

## The grading standard: Rigorous Delight

Will's motto for the project (2026-11-11): **"Phextura: Rigorous Delight."**
This loop adopts it as its explicit standard, and every principle and design
element below is graded against both halves:

- **Rigorous** — derivable from the definitions, checkable by a program or a
  reader, and costed: its price in time, space, or author-minutes is stated,
  not waved at.
- **Delightful** — a pleasure to use and to read: the common operations cost
  the author almost nothing, and the artifact rewards the reader who opens
  it.

Rigor alone is homework. Delight alone is a toy. A principle that satisfies
only one half does not pass this loop.

## Definitions

### (a) What a paper is

A paper exists to change what a reader is justified in believing, in a way
that does not depend on trusting the author. Strip everything else away and
a paper is:

> **A fixed, citable, verifiable unit of argument: claims, the evidence for
> them, and the means to check both — addressed so that another person can
> point at any part of it.**

Each word carries weight:

- **Fixed.** Citation is a promise that the cited thing will still be there,
  unchanged, when the reader arrives. A unit that silently mutates under its
  citations is not fixed, and arguments built on it are not checkable.
- **Citable.** Pointing must work at finer grain than the whole: a theorem,
  a figure, a single equation — not just "the paper, somewhere."
- **Verifiable.** Checking must be possible without the author's testimony.
  If verifying a claim requires asking the author what they meant or which
  file produced the figure, the unit has failed as a paper and succeeded
  only as a claim *about* a paper.
- **Argument.** Not a container of text: an ordered structure of claims and
  warrants, where each claim's support — proved here, cited, measured,
  assumed — is part of what is asserted.

Note what the definition excludes: certification. Refereeing, acceptance,
prizes are social events *around* artifacts. They matter enormously and
they are not in the definition, because a rejected paper is still a paper
and an accepted one is not thereby more fixed, citable, or verifiable.
This exclusion is load-bearing in the Persona Panel evaluation below.

### (b) What a reference is

A reference is a pointer from one part of the discourse to another, with
three obligations:

1. **Resolvability.** A reader (human or program) can traverse it and
   arrive at the target.
2. **Integrity.** What the reader arrives at is what the author meant — or
   the mismatch is detectable.
3. **Locality of failure.** If the pointer breaks, the break is discovered
   at the pointer, with its location known — not silently, downstream, in
   a reader's confusion.

A reference that can silently rot is worse than no reference: it launders
false confidence. "See Section 4" in a document whose sections were
renumbered is a lie the format tells on the author's behalf.

### (c) What typesetting is

Typesetting is **the mapping of an abstract document structure to a
two-dimensional visual field, as an optimization under constraints.**

The exemplar, stated from the sources (Knuth & Plass, "Breaking
Paragraphs into Lines," *Software—Practice and Experience* 11 (1981),
1119–1184; *The TeXbook*, ch. 14): total-fit line breaking does not set
line after line greedily. A paragraph is a sequence of **boxes** (rigid:
width, height, depth), **glue** (a quadruple: natural size, stretch,
shrink, and an *order of infinity* ∈ {normal, fil, fill, filll} — only
glue of the highest order present participates in adjustment), and
**penalties** (integers in [−10000, +10000]; ≥ +10000 forbids a break,
≤ −10000 forces one; hyphenation's default penalty is 50). For a
candidate line, the adjustment ratio r is computed against the stretch
or shrink at the highest order present; **badness** is b ≈ 100·|r|³,
capped at inf_bad = 10000, and sorts the line into a fitness class
(tight, decent, loose, very loose). A breakpoint is **feasible** iff
reachable through lines of badness ≤ a threshold — **pretolerance**
(default 100; first pass, no hyphenation), then **tolerance** (default
200; hyphenation on), then an emergency pass if no feasible sequence
exists. Per-line **demerits**, with λ = line penalty (default 10),
b = badness, p = the breakpoint's penalty, are:

- p ≥ 0: d = (λ + b)² + p²
- −∞ < p < 0: d = (λ + b)² − p²
- p = −∞: d = (λ + b)²

— plus additive double-hyphen demerits (consecutive flagged lines),
adjacent demerits (fitness classes differing by more than one), and
final-hyphen demerits. Penalties live at breakpoints; demerits live in
the objective; the two layers are not interchangeable. A dynamic
program over the feasible-breakpoint graph then finds the sequence of
minimum total demerits: a global optimum *of the stated model over the
feasible set*. Scope honesty: this totality holds for paragraphs.
TeX's page breaking is a greedy best-break-so-far procedure against a
page goal, and float placement has no such optimisation — the exemplar
proves a solver exists for one sub-problem, not for documents entire.

The generalizable content: the author of a document is really specifying
**constraints and intent** (this may break, this must stay with that, this
is worth a penalty), and the typesetter is a **solver**. Any paper system
inherits this whether it admits it or not: the only question is whether
the constraint vocabulary exists as mathematics — quantities a solver can
price — or degenerates into manual spacing and prayer. (Pass 1's first
draft of this section misstated the demerits formula from memory. The
critic caught it; the correction stands above as the loop's own first
exercise in P2 applied to prose.)

### (d) What reading is

Reading happens twice over the same artifact, by two different readers:

- **The human reader decodes sequentially.** The eye takes fixations in
  order; working memory holds a sliding window of a few items; coherence is
  built locally and prediction is constant. For this reader, canonical
  order, local coherence, and stable visual landmarks are cognitive
  necessities, not aesthetics.
- **The machine reader traverses a graph.** A program does not read down;
  it fetches by address, follows edges, computes over structure — the
  reference graph, the citation graph, the derivation graph of figures
  from code and data.

A paper system serves both or it serves neither fully: if the machine reads
a different artifact than the human (a shadow structure recovered by
executing the file), then claims verified by the machine are not claims
about the paper the human read.

### (e) What authorship costs

The minutes of writing a paper divide into:

1. **The work itself** — thinking, drafting, proving, measuring. This
   should dominate, and a tool has no business taxing it.
2. **Bookkeeping** — numbering theorems and figures, keeping references
   consistent, formatting bibliographies, managing the file set.
3. **Verification labor** — checking that figure 3 is the output of the
   current code on the current data; chasing provenance.
4. **Tool-fighting** — escaping regimes, preamble tuning, inscrutable
   errors, rerun-until-stable rituals.
5. **Revision mechanics** — inserting one result and repairing forty
   pointers; locating referee objections in the text; replying point by
   point.

A paper system is good exactly insofar as it deletes (2)–(5) without
constraining (1). Every proposed feature is charged against this ledger:
it must state which line it reduces, and by how much, or it is a cost
masquerading as a capability.

## Derived principles

Each principle is stated, then derived in a line or two from the
definitions. Costs are stated where the definition forces them.

**P1 — Identity is (edition, address), not name.** *(amended in Pass 3)*
From (a)-citable and (b): pointing at a part requires an identifier whose
target survives editing. An author-invented string survives only by the
author's vigilance; a structural address survives insertion into unused
coordinates — but integer coordinates are not dense, so re-basing
operations (`offset`, `expand`, `contract`) can move a unit, and external
citations cannot be rewritten by any tool. Therefore identity is the pair
**(edition, coordinate)**: an edition is the corpus root plus its manifest
checksum (P10); a coordinate is identity *within* an immutable edition;
and any operation that re-bases addresses creates a new edition and must
emit a **mapping scroll** (old coordinate → new coordinate for every moved
unit), so a citation to edition *k* remains mechanically resolvable into
edition *k+1*. Names — titles, running heads — are display text, never
keys.

**P2 — References fail loudly, or not at all.** *(amended in Pass 3)*
From (b)'s three obligations: resolution must be mechanically checkable
over the whole artifact (else resolvability is a hope), and failure must
be reported at the pointing site (else locality fails). Validation
requires a stated **reference grammar**: coordinates collide with
ordinary dotted numerals (version numbers, section-like strings), so
what counts as a written reference must be decidable, or the validator
produces false dangling errors or silent misses — both violate (b).
Cost, in the P12 format (variables: B = corpus bytes, S = scrolls,
R = written references, D = distinct targets): Θ(B) to scan and
enumerate references, Θ(S) to build the resolution index, Θ(D) expected
to resolve distinct targets against it — **Θ(B + S + D) expected time,
Θ(B + S + R) space**, performed as roughly three traversals (explode,
navmap construction, resolution), *not* one pass; without the index the
cost is Θ(B + R·S) and the design must say so. Any system whose
references can dangle silently violates (b) by definition, whatever
else it does well.

**P3 — The artifact is closed, relative to its manifest.** *(amended in
Pass 3)*
From (a)-verifiable: checking must not require unversioned state outside
the artifact. Absolute closure is unachievable — fonts live in the OS;
so do the renderer and the reader's programs — and an unachievable
principle is unfalsifiable, since any leak can be reclassified as "not
needed." Therefore: the artifact is closed *iff* every dependency of
rendering and checking is either (i) a scroll in the corpus, or (ii) a
line in the Collection 6 manifest naming the external dependency and the
checksum or version that satisfies it. Anything neither in the corpus
nor in the manifest is a closure violation, mechanically detectable.
The price is stated, not hidden: hoisting toolchains and media at
4⌈P/3⌉ expansion (P12 table) makes a closed artifact potentially large,
and that size is a cost of the principle, charged openly.

**P4 — Nothing in the artifact executes.**
From (a) and (d): the machine must read what the human reads. If structure
is revealed only by running the author's code, the read artifact is a
function of a runtime — not fixed, not inspectable, not the same object
for both readers. Therefore the artifact is data. Programs that process
it — renderers, checkers, critics — live outside it or ride in it as
declared, opaque, unexecuted payload (P10 governs their provenance); the
artifact itself runs nothing. This is the principle with the sharpest
teeth, and it is also the one that makes P2's one-pass validation
possible: data can be checked in a pass; behavior cannot.

**P5 — Display numbers are a view; identity never is.**
From (e)-bookkeeping: a number ("Theorem 3.2") is a function of how many
peers precede a block in reading order. If numbers are stored, every
insertion taxes the author with repairs (P2 failures follow). Therefore
numbers are computed at render time from traversal order and never stored
in the artifact. Identity (P1) is what persists; numbers are weather.

**P6 — DEMOTED in Pass 3: layout intent requires a priced constraint
vocabulary. (OPEN requirement, not a principle.)**
Pass 1 stated this as "presentation is parametric, and layout is the
solving of it," generalizing "Knuth's own METAFONT move." The critic
convicted it on three counts, all sustained: (i) no vocabulary was
supplied — no quantities, domains, units, or objective — so the
principle could judge no design; (ii) no solver was named, and the
feasibility question (what happens when constraints are unsatisfiable —
the question TeX answers with pretolerance/tolerance/emergency passes)
was never asked; (iii) the METAFONT correspondence was a metaphor:
METAFONT solves *linear* equations over known / unknown-independent /
unknown-dependent variables by elimination, and its parameters are plain
inputs assigned before solving (*The METAFONTbook*; "Metafont,"
§Variables and equations) — phextura exhibits parameter *assignment*,
which every stylesheet since stylesheets also exhibits, and no document
equations at all. P6 also conflated its two exemplars: penalties and
costs are TeX's discrete optimisation, a different mathematical
character from METAFONT's equation solving. What survives, stated at
its true size: **style is a parameter assignment stored as addressed
scrolls (Collection 4); one corpus therefore renders under many
assignments without editing; and if a print backend is built, paragraph
breaking adopts total-fit optimisation (definition (c)) as-is.** The
requirement that remains — a priced block-level constraint vocabulary
with an objective, a feasibility/failure regime, and a named solver
whose cost is stated — is OPEN (see the OPEN list, which now carries
the critic's seven-item minimal specification). One discharge, owed to
P4: the renderer, an external program, plays the interpreter's role;
the corpus is data throughout. There is no execution in the artifact,
borrowed patron or not.

**P7 — One artifact, two readings.** *(amended in Pass 3)*
From (d): the corpus must define both a canonical sequential order (the
eye's path) and an addressable graph (the program's path) over the *same*
units. The canonical order is a **defined traversal**, not an assumed
one: the divisions in a stated order (Body, then Bibliography, …),
coordinate order within each division, and referenced non-body units
(figures, footnotes) rendered at their reference sites — reference sites
being data in the body scrolls. So defined, the reading order is a named
function of the corpus, and non-drift is checkable: a test can compare
the traversal's output against the graph unit-for-unit.

**P8 — Structure is data, not side effect.** *(amended in Pass 3)*
From (d)-machine and (a): the role of a unit — theorem, proof, caption,
datum, code — must be readable off the artifact without interpreting it.
A block's role is declared in the block: a **primary role plus optional
secondary role atoms** (a figure that is also evidence; code that is
also a supplement), because one-word-one-role forces authors to choose
the display role and lose the structural one. The role vocabulary is
versioned in the manifest, and an untagged block is not role-less: its
role is explicitly `prose`. (If roles were recoverable only by rendering
or executing, P4 and P7 fall with it.)

**P9 — Revision is algebra, not surgery.** *(amended in Pass 3; the word
"algebra" is on probation)*
From (e)-revision and (a)-fixed: the cost of a change must be proportional
to the change, not to the document. Therefore the corpus supports
operations — insert, delete, move a subspace, re-base addresses — whose
effects are local and predictable, and pointers (P1) keep their meaning
across them. But an algebra is elements, operations, and **laws**, and
Pass 1 stated none. The design therefore owes, as checkable equations
with side conditions, at least: the domain on which `remove ∘ insert`
is the identity; whether `offset(a) ∘ offset(b) = offset(a+b)`;
whether `merge` is associative, commutative, idempotent; and where
`subtract` is and is not a left inverse of `merge`. Until those are
stated and verified against libphext, "algebra" is an adjective. The
cost is restated honestly: the *logical* effect of an edit is
span-local; the *mechanical* cost in the current one-file storage model
is Θ(B) rewrite per edit, and Θ(B + R) when a re-base rewrites
references; span-proportional mechanical cost, if required, is a
storage-layer design obligation, not an assumption. A system where
inserting a lemma is O(document) of *author labor* fails (e) regardless
— that half was always about the author, and it stands.

**P10 — Provenance rides inside.** *(amended in Pass 3)*
From (a)-verifiable and (a)-fixed: whatever a checker needs to trust a
unit — who wrote it, when, from what sources, rendered or produced by
which exact program at which checksum — is part of the artifact,
checksummed with it. What checksums buy is **tamper-evidence**: a change
made after an honestly computed checksum is detectable. They do not buy
**authenticity**: provenance false when written, then honestly
checksummed, is internally consistent and simply untrue; "who wrote it"
requires a signature by a key whose fingerprint is anchored *outside*
the corpus — or the claim is recorded as self-asserted and labelled as
such. Provenance for generated units must record, in addition to
program identity and input checksums, the **output checksum at
generation time**, so substitution of the output is detectable even
when the edge is honest. Manifest cost, printed per P12: Θ(B + H) time
to build; Θ(S) entries at tens of bytes each — on fine-grained corpora
of many small scrolls, a measurable percentage of total size, not
absorbed into the word "linear." This principle governs *generated and
simulated content* with special force: text produced by a program must
be marked as such, with the program and its inputs identified, inside
the artifact that carries it.

**P11 — A claim carries its warrant as data: a warrant graph, not a
status field.** *(mechanism replaced in Pass 3; the obligation is
Pass 1's, the graph is the critic's amendment)*
From (a)-argument: an argument's steps differ in warrant — proved here,
proved by citation, measured, assumed for the sake of argument,
conjectured, entertained as fiction inside a contradiction. A reader
checking the argument must be able to recover each claim's warrant from
the artifact, because the warrant is part of what the claim asserts.
Typography can *display* status; it cannot *be* the status, because
display is a view (P5) and views are lossy. So much Pass 1 derived, and
it stands — it is not an import from anyone's research habits; it is
what "argument" means, made checkable.

Pass 1's mechanism — one closed status enum per claim — does not stand,
for three reasons the critic established. (1) The categories are not
one kind of thing: `assumed`/`conjectured` report the author's *stance*;
`empirical` reports a *history*; `cited` a *relation* to another
artifact; `proved-here` a *mathematical property* no author can declare
into existence. One field flattens four ontologies. (2) Existence is
not entailment: a pointer-to-proof that merely *resolves* validates
that a scroll exists, not that an argument holds — `proved-here`
pointing at the word "obvious" passes, and a self-awarded "verified"
is precisely the false-confidence laundering definition (b) calls
worse than none. (3) One status per claim fails ordinary papers:
conditional proofs, dual independent warrants, probabilistic proofs,
computer-assisted proofs, claims cited to later-retracted work,
stipulative definitions (warrant-free by kind).

Therefore: each claim-bearing unit carries zero or more **warrant
edges** — records of the form *(kind, target coordinate(s),
asserted-by, asserted-at, basis)* — where kind ∈ {`formal-proof`,
`prose-proof`, `citation`, `measurement`, `assumption`, `conjecture`,
`fiction-device`, `stipulation`, `refutation`}, under mechanical
validation rules:

- `formal-proof`: the target is a proof in a declared checkable format
  and the declared checker **runs and accepts it against this claim's
  statement scroll**. Until the phext-native format exists, no scroll
  may carry this kind except via a hoisted checker payload that
  actually checks (today: Lean's kernel). Cost: the checker's, stated
  per P12 — never fetch's.
- `prose-proof`: target resolves and is role-tagged `@proof`; display
  is "proof (prose) at ⟨coordinate⟩," never a bare "proved."
- `measurement`: targets are a data scroll and a code-or-method scroll,
  both checksummed (the produced-by edge of the weave section, doing
  double duty); validation checks resolution and checksum agreement.
- `citation`: target resolves in its corpus **and** the edge records
  the target edition's checksum (P1), so upstream retraction registers
  as an edition change instead of being silently inherited.
- `assumption`, `conjecture`, `fiction-device`, `stipulation`: no
  target required; author stances, displayed as stances.

Displayed status ("Theorem — proved") is a **view** computed from the
graph under a stated policy — P5's doctrine applied to P11. The author
assigns edges; **checkers assign validation results**; venues and
readers assign display policy. No party both asserts and certifies.
Transitions *add* edges (a `refutation` edge points at its basis);
edges are never silently edited, so the history of what was believed,
when, and why it changed is itself data under P10. Whether `proved` as
an asserted outcome belongs in the artifact: it does not — an edge is a
*record of* a basis, and records are data (a) legitimately covers;
asserting the outcome was process smuggled in, and Pass 3 smuggles it
back out.

**P12 — Every mechanism states its cost, in a fixed format.** *(amended
in Pass 3)*
From (a)-verifiable: verification that cannot be afforded will not be
done, so a verifiable system must be able to state the cost of checking
in terms of the artifact's size. Pass 3 fixes the format the critic
demanded: every cost statement names (1) its size variables, from the
palette {B bytes, S scrolls, R references, D distinct targets, N body
blocks, P payload bytes}; (2) time *and* space; (3) worst case, or
expected case **with the assumption that buys it** (e.g. "expected,
under a hash index"); (4) the algorithm whose cost is stated. A cost
that cannot be put in this form is printed **UNPRICED** — a more honest
word than a guessed big-O. Under this format: validation is
Θ(B + S + D) expected (P2); HTML traversal/rendering is Θ(B + D)
expected; display numbering is Θ(N) by a single traversal with running
counters; optimised layout is UNPRICED (see P6-demoted); pipeline
re-execution is UNPRICED and *unbounded* (the code is arbitrary — its
cost is the pipeline's own, never folded into any "linear" sentence);
Persona Panel inference, if ever built, is Θ(A · T) tokens per opening
for A personas over a T-token corpus — a recurring cost attached to the
act of opening, and a reason opening must never be silently automatic.
A principle without a cost is a wish; a system without stated costs
cannot be graded rigorous under the motto. Pass 1's own table failed
this format in four rows; the corrected table is below, and the failure
is recorded rather than defended — the correction is the principle
working as designed.

**P13 — Corpora compose.** *(amended in Pass 3)*
From (a)-citable across artifacts: papers cite papers; proceedings are
papers composed; a library is many corpora under one address space.
Therefore composition is an operation on corpora — merge, extract,
re-base — with the property that addresses inside a part keep their
meaning, or are offset by a stated, uniform amount, when parts combine.
Pass 3 adds the discipline the collision case forces: the standalone
root (Z = 1.1.1) is a collision attractor — every standalone paper
starts there — so roots are allocated distinctly, or the smaller corpus
is re-based on merge, emitting its P1 mapping scroll; an external
citation always names (edition checksum, coordinate), so composition
never invalidates an external pointer — it adds an edition. Cost:
Θ(B_A + B_B + R_rewritten) time, Θ(B_A + B_B) space. Identity through
composition follows the (edition, coordinate) pair — one notion of
"same scroll," held by P1, P10, and P13 alike.

**P14 — REPLACED in Pass 3: delight is measured, not asymptotic.**
Pass 1 stated: common operations "must cost the author O(1) actions."
The critic's interrogation stands unanswered: an "action" was never
defined (keystroke? command? decision?); finding a coordinate is a
search whose cost was moved off-ledger; and strictly, an address of
unbounded integer components has length Θ(log of the largest
component) — ≈17–25 characters in practice, good and printable, but
not O(1). A claim that cannot be false is not a cost statement, and
the motto forbids uncosted cost claims above all. What survives of the
intent, in falsifiable form: **phextura maintains a benchmark suite**
— for each common operation (write a paragraph; cite a bibliography
entry; reference a theorem; insert a block mid-document; reply to one
objection; read one scroll raw), a cost specified in (keystrokes,
editor commands, decisions requiring information not on screen),
measured on the demo corpus, with a target value and a stated
measurement method. Delight is then improvable per failure instead of
asserted per document. The suite is an obligation this document hands
to the demo phase; until it exists and is run, all delight claims in
phextura documents — including the motto grades the critic assigned —
are provisional. A system can satisfy P1–P13 and still fail here
(imagine perfect addressing at forty keystrokes per reference). Under
the motto it has then failed outright: rigor at unmeasured author cost
is homework with good provenance.

## phextura's commitments, derived

The phext primitives were not chosen by taste; each is the mechanism a
principle forces. (The full libphext API mapping already lives in
DESIGN.md §8 and is reused, not redone.)

| Principle | phext mechanism | Stated cost (P12 format) |
|---|---|---|
| P1 (edition, address) | `Coordinate` — nine components, structural; edition = root + manifest checksum; mapping scrolls on re-base | Address string ≈17–25 chars typical; length Θ(log of largest component) strictly |
| P2 loud failure | `navmap` enumerates every written reference; each distinct target `fetch`ed against an index | Θ(B + S + D) expected time (hash index assumed), Θ(B + S + R) space; ≈3 traversals, not one pass |
| P3 closure (manifest-relative) | Media/supplements as scrolls in-corpus; binary hoisted as base64; externals (fonts, renderer) as manifest lines with version/checksum | Hoisting: H = 4⌈P/3⌉ exactly (4.00× at P = 1, → 4/3 asymptotically), plus header/manifest overhead; integrity Θ(H) |
| P4 no execution | Scrolls are plain text; programs are external, or hoisted as declared opaque payloads the reader's program may choose to run | Reading a paper costs a parse, never a sandbox |
| P5 view numbers | Defined traversal (P7) generates "Theorem 3.2" overlays via one pass with running counters per (role, section) class | Θ(N) time per render; zero author actions per insertion |
| P6 layout vocabulary | **OPEN** — style scrolls (Collection 4) hold parameter assignments today; no constraint vocabulary or solver exists | UNPRICED |
| P7 two readings | One corpus: the defined traversal for the eye, address graph (`navmap`, `index`) for the program | Same bytes; no shadow structure to maintain |
| P8 structure as data | Role line: primary tag + secondary atoms (`@figure +evidence`); vocabulary versioned in the manifest; untagged = `@prose` | One line per block, paid once by the author |
| P9 revision operations | `insert` / `replace` / `remove` / `range_replace` / `offset` / `expand` / `contract`; laws owed, see P9 | Logical effect span-local; mechanical Θ(B) per edit in one-file storage, Θ(B + R) on re-base |
| P10 provenance | Collection 6 manifest (contents, checksums, toolchain pins, signature or self-asserted label); provenance lines inside scrolls | Θ(B + H) build time; Θ(S) manifest entries (tens of bytes each — material on fine-grained corpora) |
| P11 warrant graph | Warrant edges as lines on claim scrolls; validation per edge kind (checker runs for `formal-proof`; resolution + checksum for `measurement`/`citation`) | One edge per warrant; `formal-proof` validation costs the checker's run — stated per checker, never as fetch |
| P13 composition | `merge` / `subtract` / `offset`; root allocation + mapping scrolls; external citations name (edition, coordinate) | Θ(B_A + B_B + R_rewritten) time, Θ(B_A + B_B) space |
| P14 measured delight | Benchmark suite over the demo corpus (keystrokes / editor commands / off-screen decisions per common operation) | Suite UNPRICED until defined and run — all delight claims provisional until then |

### What the derivation produces that reformulation could not

The warts-driven design reached P1–P10 and P13–P14 in substance, because
negating LaTeX's failures happens to trace them. Three things it did
**not** reach, because they are not the negation of any wart:

1. **P11, warrant as data.** LaTeX never had claim warrant to lose —
   warrant rides in typography and prose convention there, so a
   wart-hunt never misses it. But (a) defines a paper as *argument*, and
   an argument whose steps' warrants are recoverable only from styling
   fails its own definition. phextura must therefore carry, on every
   claim-bearing scroll, its warrant edges (P11 as amended: a graph,
   not an enum — Pass 1's enum is convicted in the P11 entry above).
   A related discipline exists in Will's research tracker (every claim
   carries True/False/Unknown/Fiction under the Minimum Viable Oracle
   rules); note, per the critic's H10, that MVO tracks *status of
   claim* while P11 tracks *warrant of claim* — different species.
   phextura's obligation is derived here from the definition of
   argument, not borrowed; the overlap of the two disciplines is a
   fact to examine, not a premise to lean on.

2. **The weave test for code and evidence — amended: a provenance
   graph today, a reproducibility weave only under conditions.**
   DESIGN.md §5 says a paper "can carry its own evidence." Carrying is
   not weaving. WEB's construction guarantees (Knuth, "Literate
   Programming," *The Computer Journal*, 1984): **G1** the code read
   and the code compiled are the same tokens from one source; **G2**
   narrative and compilation orders both derived mechanically; **G3**
   total cross-reference of chunks and identifiers by construction;
   **G4** no place for code to hide. Produced-by edges alone achieve
   none of these fully: they are assertions joining two independently
   editable objects (G1 fails — an edge detects code *changed since
   the edge was written*, not that the output *is* that code's output);
   the run order over a pipeline DAG is unspecified (G2 fails);
   cross-reference holds at scroll grain, not identifier grain (G3
   partial); and nothing yet *validates* that an evidence scroll has
   an edge (G4 fails). What P3+P10 force today is a **provenance
   graph**. It earns the name *reproducibility weave* only when:
   (i) produced-by edges are **mandatory and validated** for every
   evidence-tagged scroll, recording code coordinate + checksum, input
   coordinates + checksums, an environment declaration (the §5.1
   hoisted toolchain is the natural anchor), and the output checksum
   at generation time; (ii) the run-traversal is a specified
   topological traversal of the produced-by DAG with cycle rejection;
   (iii) verification is stratified — edge validation (cheap, linear,
   always on) vs reproduction (re-execute, compare output checksums;
   expensive, opt-in, priced per pipeline, UNPRICED in general). Under
   (i)–(iii) phextura would have something WEB never attempted. Until
   then the honest word is provenance graph, and the design's §5 is
   amended to say so.

3. **P6's residue.** The reformulated design keeps Knuth's
   line-breaking "as a renderer option" and stops; Pass 1 claimed to
   go further and, per its own critic, advanced only in vocabulary.
   What the derivation genuinely adds beyond the old design is the
   *requirement stated as a requirement*: a priced constraint
   vocabulary must exist before any layout claim is made (OPEN list),
   and — from the critic's §3.2, item 6 — the consequence that when a
   solver does exist, its identity and version become part of the
   rendering definition and belong in the manifest (P10), since two
   solvers reaching different optima otherwise produce two papers from
   one corpus.

### The provisional-Lean position, tested

§5.1 of the design hoists Lean's checker as a provisional payload,
pending a phext-native proof format. Against the principles: the payload
itself is legitimate (P4 permits declared opaque payloads; P10 requires
their declaration; extraction is the reader-side program's decision).
But the *destination* is forced, not merely preferred: proofs carried as
compiled binary fail P7 (the human reader cannot read them) and P8
(their structure is not data the corpus can address — steps are not
scrolls, so prior theorems cannot be cited *by coordinate into the
proof*). Two precisions the critic forced, both cutting against this
document's first draft: Lean's *kernel* is in fact a small trusted
checker core — the large surface is the elaborator and toolchain, which
need not be trusted for proof *checking* — so the honest TCB comparison
is kernel-to-kernel, and on that measure the bootstrap is better, and
the native format's required victory harder, than Pass 1 said. And
Metamath (verifier ≈300 lines, substitution as sole rule; Megill, *The
Metamath Proof Language*) proves a small checker is possible for a
format optimised for its checker; its proofs are famously not
prose-readable, so it is **no** existence proof for phextura's actual
target — small checker *and* readable proofs *and* step-addressability,
a conjunction that remains an open research problem, not a citation.
A native format — proof steps as text scrolls, theorem citations as
coordinates, a small checker — is what the principles converge on.
**Verdict of this pass: the provisional position survives derivation.**
Lean is the bootstrap; the principles name the destination independently
of anyone's roadmap.

### Equation entry: requirements only

The notation itself is OPEN and belongs to a session with Will. What the
definitions force, any candidate notation must satisfy:

- **R1 (from (e), P14):** entry cost is linear in the mathematics, not in
  ceremony — no escaping regime, no macro vocabulary to memorize.
- **R2 (from P4, P8):** the notation is data: readable raw by a human,
  parseable by a program, its meaning never dependent on expansion state.
- **R3 (from P1, P2):** an equation is a block like any other —
  addressable, referenceable, its references validated in the same pass.
- **R4 (from (d)):** one source, two readings — the notation must encode
  enough structure for the machine reader (what is bound, what applies
  to what) while retaining presentational intent for the human reader.
  A notation encoding only the picture starves the machine; only the
  semantics, the eye.

## Candidate component: the Persona Panel

Will's input (2026-11-11 00:48 CST), verbatim: *"Embed personas for
Knuth, Newton, Einstein, etc. - people who can tear your paper to shreds
in real-time as your MCP agent opens it."* Working name: **the Persona
Panel**. Each persona is a small corpus of scrolls encoding that reader's
standards, obsessions, and characteristic failure-hunts, distilled from
their actual writings. When an MCP agent opens a paper, the panel wakes:
each persona reads the corpus and writes critique as *addressed
marginalia* — annotation scrolls pointing at the exact scrolls they
attack — and author replies are scrolls in the same space. This section
evaluates the panel against the principles like everything else. It
decomposes into two components with different fates.

### Component 1: addressed marginalia with provenance — DERIVES

Refereeing is excluded from the definition of a paper (see (a)): it is
process, not artifact. But the *record* of checking — objections,
replies, revisions — is data about the argument, and the moment such a
record exists the principles dictate its form:

- Objections must point by address (P2), or they rot the first time the
  paper is revised — the referee's "page 7, line 3" is exactly a silently
  rotting reference.
- Authorship of every annotation rides inside it (P10): who or what
  wrote this, when, against which checksum of the target scroll.
- The paper itself stays **fixed** ((a)): marginalia therefore cannot be
  written into the paper's corpus. They form a **sibling annotation
  corpus**, keyed to the paper's root coordinate and checksum, composed
  with the paper at read time by `merge`/`offset` (P13) and removable by
  `subtract`. Staleness is mechanical: if a target scroll's checksum has
  moved since the annotation was written, the annotation flags itself
  stale instead of pointing at prose that no longer says that.
- The payoff lands on (e)(5): point-by-point reply to addressed
  objections is scroll-against-scroll, not archaeology.

So marginalia-as-corpus is not a feature idea; it is what review records
must look like in any system satisfying P2/P9/P10/P13. **Derived.**

### Component 2: the personas themselves — DOES NOT DERIVE

Nothing in the definitions requires the checker to wear a face. A
faceless engine running the same standards satisfies every principle
identically. What the persona adds is an *interface* to a standards
corpus — and interfaces are judged by P14 and the motto, where the honest
argument is empirical, not derivational: a standard embodied as a
memorable reader ("Newton will demand the phenomenon that forces this;
Einstein will ask what you *mean* by that word, operationally; Knuth
will find the unstated assumption and the missing base case") may be
remembered at the moment of writing in a way checklist item 7 is not.
That is a claim about human memory, testable only in use.

The architectural trigger, at least, is fully specifiable (an
implementer could build this tomorrow): **opening is traversal.** An MCP
host opens a paper by (1) fetching the Collection 6 manifest; (2)
enumerating the body subspace via `get_subspace_coordinates`; (3)
reading the manifest's panel declaration — which persona corpora, at
which coordinates and checksums, the paper (or the venue, or the reader)
has engaged; (4) spawning one reader-agent per persona, each reading
through the same fetch/traversal interface and writing its marginalia
into the annotation corpus. The personas ride as declared payloads and
definitions (P4/P10): the paper executes nothing; the host's decision to
wake the panel is the same class of reader-side decision as extracting a
toolchain (§5.1).

The labelling requirements are not decoration; they are P10 and P11
applied to generated text, and they are absolute:

1. Persona output is **labelled as simulation everywhere it appears** —
   in the annotation scroll, in any rendering, in any export. A persona
   encodes standards; it confers no authority; a persona's words are
   never the person's words and never an endorsement.
2. A persona is only as good as its **source distillation**: the persona
   corpus must cite its sources **at checkable grain** (for Knuth:
   *Mathematical Writing*; *The TeXbook*, ch. 14; Knuth–Plass 1981 —
   work and chapter, not "the TeXbook's own standards") — and every
   shredding must cite *both* the attacked scroll *and* the persona
   scroll's derivation line from that primary source. A citation chain
   that terminates at the persona scroll proves only that the persona
   agrees with itself (critic, §6.2). An uncited shred is vibes, and
   vibes fail P2.
3. Persona claims carry epistemic status (P11): `simulation`,
   permanently distinct from referee verdicts and from the paper's own
   claim statuses.

**Motto test, by name.** Rigorous? Only under the citation discipline
above — standards with sources, attacks with addresses. Delightful?
Being torn to shreds by Einstein *before* submission, with receipts,
while every wound is still fixable — that is the rare feature that is
more delightful the more rigorous it is. Toy? The instant a persona
performs a famous voice instead of enforcing a sourced standard, yes —
and the format's defence is that the toy version cannot satisfy the
citation requirements, so it cannot wear the label.

**Disposition:** the marginalia mechanism is derived (Component 1) and
is recorded above as part of phextura's commitments; the critic grades
it Rigorous 5/5, Delightful 4/5 and judges this document too modest
about it. The Persona Panel as a shipped component — simulated persons
waking on open — does not derive from the definition of a paper, and
under this loop's rules it goes to the **proposals for Will, unapplied**.
The critic concurs in the disposition and corrects this document's one
overclaim: Pass 2 of this loop is a hand-run Knuth persona over this
very document — a prototype of the *standards essay*, aimed by a human
at a named target, sources fetched by hand, run exactly once. It
prototypes nothing about the wake-on-open *system*. What generalizing
it costs, stated plainly and completed per the critic: one scholarly
distillation corpus per persona (the expensive part, and the part that
determines whether the whole thing is rigorous or theatre); **one
fidelity benchmark per persona** — a corpus of papers with known,
catalogued defects against which the persona's recall and precision
are measured before it is unleashed, without which instrument and
impression are indistinguishable; a runner protocol (the
open/traversal semantics above); labelling enforcement in every
renderer; staleness machinery; **per-open inference cost Θ(A · T)
tokens, forever** (P12 table); egress — the paper's full text leaves
for model providers when personas run remotely, a confidentiality fact
closure must reckon with; disagreement arbitration (contradictory
marginalia need a conflict display policy, or the author drowns);
persona versioning under the same edition discipline as everything
else (P1/P13); and the standing temptation — multiplied by every
persona added — to let character substitute for standards. Panel as
specified, critic's motto grades: Rigorous 2/5, Delightful 4/5. Under
the motto, 2-and-4 does not ship; it goes to proposals.

## What only the demo can test

The end-to-end demo running in parallel is the first contact between
these principles and a real paper. It can test: P5 (insert a theorem;
does the overlay renumber with zero author actions?), P9 (restructure a
real chapter; is the logical cost proportional?), P2 at real scale, P7
(does the defined traversal *read* as the paper reads?), P11 (do a real
paper's claims carry warrant edges without distortion — conditional
proofs, dual warrants, a retracted citation are the test cases the
critic named), and the P14 benchmark suite's first measurements. It
cannot test the layout vocabulary (no solver exists) or P3 at
adversarial scale. Principles the demo falsifies get revised here, not
defended.

## OPEN — honestly not derived

- **The layout constraint vocabulary.** The requirement is derived
  (P6-demoted); the vocabulary is not yet mathematics. The critic's
  minimal specification (critic.md §3.2) is the starting contract:
  (1) measurable block geometry in stated units, rigid vs flexible
  dimensions declared; (2) inter-block glue quadruples per adjacency
  class, with the need for orders of infinity *decided by derivation*,
  not omitted by inattention; (3) named penalty classes (keep-with-next,
  keep-together, break desirability, widow/orphan analogues,
  figure-displacement cost) each with domain, default, and interaction
  rule; (4) a single stated objective (a demerit formula with every
  term named and defaulted) or an explicitly lexicographic one;
  (5) feasibility thresholds and a specified degradation when no
  feasible layout exists; (6) the solver, its complexity, and its
  tie-breaking — with the solver's identity pinned in the manifest,
  since different optima are different papers; (7) scope honesty:
  line breaking (solved; adopt Knuth–Plass), page breaking (greedy
  classically), and float placement (no classical optimum) are three
  problems, not one. This is the largest single gap between the
  principles and the design. (Critic's exercise rating: [25].)
- **The warrant graph's display policy** (P11). The edge kinds and
  validation rules are specified; the policy that computes a displayed
  status from the graph — so a reader distinguishes formal proof, prose
  proof, measurement, and stance at a glance without trusting the
  author — is not yet designed, and wants testing on a real paper with
  a conditional proof and a retracted citation. (Critic's rating: [45],
  research-grade.)
- **The revision laws** (P9). The equations the corpus operations must
  satisfy are listed as obligations in P9; verifying them against
  libphext's actual operations is owed work, not done work.
- **The delight benchmark suite** (P14). Obligations and units are
  specified; the suite itself — tasks, targets, method — is to be
  defined against the demo corpus and run. Until then every delight
  claim in the phextura documents is provisional.
- **Equation notation.** Requirements R1–R4 derived; syntax deliberately
  unsettled — it belongs to a session with Will.
- **Concurrent authorship.** The definitions describe the fixed artifact;
  drafts are pre-papers, and no collaboration protocol is derivable from
  them. Declined, not forgotten.
