# Panel review — phextura surface-syntax policy

**Date:** 2026-11-11. **Seated by:** Will Bickford ("This design must
pass review by Knuth, Von Neumann, Booch, Turing, Lovelace, and
Bickford.") **Record:** this file, published at the root of
wbic16/phextura@exo.

## The design under review

Phextura's surface-syntax policy, ruled by Will Bickford on
2026-11-11 (live Knuth Test Round 1; recorded in
design-verdicts.md), verbatim substance:

> phextura reserves `\scroll` for locating content in another
> scroll. Otherwise, normal addressing within the existing tex
> lexicon applies.

In full: authors use the TeX lexicon they already know; phextura
adds no new surface syntax for anything TeX already does; the
nine-dimensional phext address exists underneath every block —
machinery-facing, not an author-facing notation; `\scroll` is the
single reserved new word, for the one act TeX cannot perform:
locating content in another scroll. Referee receipt on record:
across all 595 imported LaTeX class templates, `\scroll` is defined
nowhere (2 internal `\trace@scroll` occurrences only). The ruling's
referee left one question open: `\scroll`'s mood — cite, include,
or jump.

## Preamble — what these seats are

Every seat on this panel is a **labelled simulation**: a critique
distilled from the named person's actual published writings and
recorded standards, with sources cited in each review. No seat
confers the person's authority, participation, agreement, or
endorsement. Where a person's record is thin on a point the design
raises, the seat declares that negative space and abstains rather
than improvise; the abstentions are printed with each review and
are part of the record, not gaps in it.

**Process discipline:** the six reviews were run as independent
passes. Each seat formed and wrote its verdict from its own sources
without reading any other seat's verdict. The synthesis at the end
is the only place the seats meet, and it reports their
disagreements as data — nothing was averaged, traded, or
harmonised away between the passes.

**The Booch seat.** Grady Booch is a living person and is *not*
self-proposed to this panel. His seat is a simulation of his
published design standards only, built from public writings; it
must not be read as claiming his participation or endorsement.

**The claim path (Will, 2026-11-11).** A living person's simulated
persona is *claimable by its subject*: Booch can claim his persona
by supplying his own variant — his own distillation of his
standards. A claimed variant supersedes the simulation built from
public writings, and the claiming itself satisfies the rapport
gate, because the gate is self-proposal (the same gate the
Bickford seat passed when Will proposed his own name). This is a
general mechanism for any living person's seat, not a Booch
special case. Until claimed, the seat remains what it is labelled:
a simulation from public record.

**The Bickford seat.** This seat is a simulation applying Will
Bickford's *recorded* standards. It is not Will, its verdict does
not bind him, and it confers nothing: the living acceptor of this
design is Will himself, and this panel's roll-up goes to him as
advice with receipts, not as a decision.

## Roll-up

| Seat | Verdict |
|---|---|
| Knuth | PASS WITH CONDITIONS (K1–K6) |
| Von Neumann | PASS WITH CONDITIONS (V1–V7) |
| Booch | PASS WITH CONDITIONS (B1–B6) |
| Turing | PASS WITH CONDITIONS (T1–T6) |
| Lovelace | PASS WITH CONDITIONS (L1–L5) |
| Bickford (simulated) | PASS WITH CONDITIONS (W1–W7) |

**The design passes review — at all six seats, and conditionally
at all six.** No seat passed it as it stands, and no seat failed
it. The collected conditions are sorted in the synthesis: some are
clarifications the design record can absorb by editing text; the
rest are decisions that return to Will. Nothing in this review was
applied to DESIGN.md, UPGRADE.md, or the ruling itself.

---

## Review 1 — Knuth

KNUTH — VERDICT: PASS WITH CONDITIONS

*Labelled simulation, distilled from the writings cited. It confers no authority and no endorsement.*

### Critique

**1. Does `\scroll` earn its keep? Yes — conditionally.**

The test I have always applied is that TeX provides a small set of primitives, and everything else is built above them by formats and by users. Plain TeX is deliberately not LaTeX; LaTeX is what Lamport built on top of it. [*The TeXbook*, Chs. 20–23 on definitions and format building; the plain TeX description in Appendix B.] A new control sequence is justified only if it does work that cannot be composed from what is already there.

By the receipt supplied, `\scroll` passes the collision part of that test and plausibly passes the necessity part: 595 document-class templates define it nowhere — its only neighbours are internal `\trace@scroll` macros, `@`-namespaced in exactly the way LaTeX reserves internal names so user-level words never collide [*The TeXbook* on `@` as a letter for internal control sequences; LaTeX practice derived from it]. And intra-document `\label`/`\ref`, `\cite`, and `\input` genuinely cannot address a block in another scroll of a 9-dimensional corpus. That is new work, not a synonym.

But a word with no pinned effect has not yet earned its keep. It has earned a *reservation*. See §3.

**2. Reserving exactly one word is the right restraint — provided the restraint has a mechanism behind it.**

Reserving one word rather than a family is the disciplined move, and it is also the project's own acceptance test in other dress: a LaTeX user recognises everything else in five minutes because nothing else has been touched. This is how a lexicon stays learnable: the primitive set stays small, the built lexicon is documented, and users build the rest. [*The TeXbook*, Preface and Ch. 20: TeX's power comes from definitions users write, not from an ever-growing primitive list; *Literate Programming* (1992) on programs being read by people first.]

The danger is not the one reservation. It is the second, third, and fourth, taken ad hoc when implementation needs internal helpers, validator hooks, and editor hooks. Knuth's answer to that pressure was never "reserve more user words in advance" — it was namespace discipline: internal names live where users do not write. The ruling as stated reserves the word but does not say where phextura's *own* internal macros will live. That is a macro-discipline hole, not a reason to enlarge the reservation. [*The TeXbook*, Ch. 20 on definition homes and grouping; the `@` convention above.]

**3. "Locating" is a noun hiding a verb — and in TeX, the verb is the design.**

This is the framing error in the ruling, and framing errors count: my reward practice — $2.56, one hexadecimal dollar, for errors found in my books and programs — has always covered errors in how a problem is posed, not only typos in its solution.

TeX's reference verbs are kept scrupulously distinct, because each has a different expansion and typesetting effect:

- `\input` — *include*: the named file's text is read and processed in place. It produces typeset material. It is transclusion at file granularity.
- `\ref` / `\pageref` — *refer*: nothing of the target is included; a number resolved across passes is typeset in its place, and an unresolved one typesets `??` and warns.
- `\cite` — *cite*: a specialised `\ref` into a bibliography database, again typesetting only a key, resolved on a later pass.

There is no fourth, native TeX verb "jump." Navigation belongs to the viewer and to device-dependent extensions, which TeX from the beginning quarantined behind `\special` — an explicit escape hatch whose contents TeX itself does not interpret. [*The TeXbook*, Ch. 22–23 on `\input` and cross-referencing; Ch. 25 / DVI description on `\special` as the driver extension point.]

"Locating content in another scroll" most plausibly wants to be in the `\ref`/`\cite` family, extended in exactly one dimension: the target is identified by a stable corpus address instead of by a document-local label. That reading fits the rest of the ruling — validators resolve references by scanning the corpus, which is LaTeX's two-pass label resolution grown to corpus scale — and it fits the acceptance test, because a LaTeX user already knows what citing something feels like.

The other two readings change the verdict:

- If `\scroll` means *include*, it is `\input` with a corpus address, and it inherits `\input`'s hard problems: expansion order, provenance of the included block in the output, and recursive inclusion loops.
- If `\scroll` means only *jump*, it typesets nothing, and by TeX's own taxonomy it does not belong in the author-facing lexicon at all — it belongs to the editor/viewer layer, or behind a `\special`-like annotation. Reserving the sole new author word for a non-typesetting act would waste the reservation.

The mood is therefore not a detail to pin later. It decides which family the word belongs to, and whether it belongs.

**4. Plain-text durability: the ruling's strongest point.**

TeX was designed so that the source file is the durable artefact — plain text, readable decades later, with no hidden state required to interpret it. [*The TeXbook*, Preface: TeX documents as files meant to outlive particular systems; *Literate Programming* on the source as literature for human readers.]

This ruling honours that, and the older design wording it supersedes must be reconciled to say so explicitly: the 9-D address is machinery underneath, editors derive hyperlinks from written addresses, validators derive resolution by scanning — nothing essential lives only in an editor's database or in binary metadata. A paper whose `\scroll` arguments are literal text in the source degrades gracefully: strip the phextura layer away and a reader still has a LaTeX document and human-readable pointers to elsewhere. That is the correct direction, and it should be stated as an invariant, not left as a consequence.

**5. The error-shaped hole: a dangling `\scroll`.**

Error messages are part of the design, written for the user who is already unhappy. TeX stops, explains in plain terms what it was doing, what it found, and what might fix it, and it offers recovery rather than silent omission. [*The TeXbook*, Ch. 27, "Recovery from Errors," and Appendix I on help messages.] LaTeX's undefined-reference behaviour is the model already in the user's hands: visible `??` in the output, a specific warning in the log, and a summary that undefined references exist.

The ruling says nothing about a `\scroll` whose target scroll does not exist, has moved, or exists but holds no block of the expected role. Silence here — omitting the reference, or worse, omitting included content — would be the one unforgivable outcome, because in a corpus whose entire claim is addressability, a quietly broken address is a false statement typeset as if true. The specification must define this case before it defines the success case, in the unhappy user's terms: what mark appears in the output, what the log says (coordinate sought, scroll and line where it was sought from), and whether a build with dangling scroll-references may call itself successful.

### Conditions

Each condition is checkable against the design record.

**K1 — Pin the mood, in TeX's verb terms.** The design record shall state, in one sentence, which of *refer/cite*, *include*, or *jump* `\scroll` performs, what (if anything) it typesets, and on which pass it is resolved. **This condition must be satisfied before the verdict stands.** If the pinned mood is *jump-only*, this verdict does not stand — the word should be reassigned to the editor/viewer layer and the reservation reconsidered.

**K2 — One mood only.** `\scroll` shall have exactly one primary effect. Any second effect (e.g. citing that also transcludes a preview) shall be a separate, named mechanism, not a mode flag or context-dependent behaviour of `\scroll`. Checkable: the record's definition of `\scroll` contains no disjunction of effects.

**K3 — Dangling-target behaviour is specified and testable.** The record shall specify, for a nonexistent target: (a) the visible placeholder typeset in the output, (b) the log/warning text including the full coordinate sought and the citing location, and (c) the build's success/failure status. **This condition depends on K1** — the placeholder and status differ for a failed citation versus a failed inclusion — and must be pinned with it before the verdict stands. A test fixture containing one dangling `\scroll` shall produce exactly the specified output, placeholder, and log line.

**K4 — Argument is literal plain text.** The record shall give a grammar for `\scroll`'s argument (the 9-D coordinate form), state that it appears literally in the source file, and state that no editor state, database, or generated file is required to interpret it. Checkable: a phextura paper stripped of all tooling is still a readable LaTeX source whose `\scroll` arguments a human can parse unaided.

**K5 — Internal names have a home that is not the user lexicon.** The record shall state the namespace (on the `@` model or an explicitly named equivalent) in which all phextura-internal control sequences live, and shall commit that `\scroll` remains the only reserved user-level word. Checkable by scanning the phextura implementation and template corpus: zero user-level reserved words other than `\scroll`, all internal definitions in the stated namespace.

**K6 — If, and only if, K1 pins the mood to *include*: loop and provenance rules.** The record shall specify detection of direct and indirect inclusion cycles, the error produced (per K3's standard), and how included material's source coordinate is preserved in the output or log. Checkable with a two-scroll mutually-including fixture. Not applicable under a *refer/cite* pinning.

### Negative space

On the following points Knuth's published record does not speak, and I abstain rather than improvise:

- **Hypertext navigation as a reading act.** TeX and METAFONT are typesetting systems; the writings treat `\special` as a quarantine for device matters, but offer no designed theory of jumps, links, or reader navigation. No verdict on jump-as-experience.
- **Transclusion as a literary/citation doctrine.** `\input` is a file mechanism, not a theory of quoting-by-inclusion, credit, or versioning of included material. Nothing in the record governs which mood is *right* for scholarship — only that the moods must not be conflated.
- **Standoff annotation and corpus-scale (L2) addressing.** Notes living outside a base text, pinned by address and span, and cross-file resolution across a distributed corpus, are outside anything TeX, METAFONT, or the literate-programming writings design for. No standard of his applies to their correctness, only — via §4 — to their plain-text legibility.
- **Real-time sync, collaborative editing, and editor behaviour** generally, including automatic hyperlinking as a user experience. The writings address author, program, and reader of a finished text; they do not address live co-presence.
- **The 9-dimensional address scheme itself** — its dimensionality, ordering, or fitness as an addressing system. That is outside the surface-syntax scope of this review and outside his published subject matter alike.

---

## Review 2 — Von Neumann

VON NEUMANN — VERDICT: PASS WITH CONDITIONS

*Labelled simulation only: the following confers no authority and no endorsement. It applies standards drawn from von Neumann's published work to the policy as stated.*

### I. The reservation is not yet a mechanism

In *Theory of Games and Economic Behavior* (von Neumann and Morgenstern, 1944), a game is not described until its rules, the players' available strategies, and the consequences of each strategy are stated. A rule that is announced but whose violation is not defined, detected, or made impossible is not part of the game; it is a hope about how the game will be played.

Judged by that standard, "exactly one reserved word, everything else inherited" is currently an incomplete mechanism. It specifies the desired outcome — one word today — but not:

- the complete set of admissible primitive words, relative to a pinned version of the TeX lexicon;
- whether a user- or package-defined macro counts as a "new surface word";
- who or what rejects an unreserved primitive;
- the procedure by which a second word could ever be legitimately admitted.

The stability question therefore has a precise answer. The rule is stable **only if deviation is made unplayable** — e.g. a validator that refuses an unregistered primitive — in preference to merely punishable after the fact. Structural enforcement changes the strategy set itself. Social enforcement — a registry, a gatekeeper, convention — only changes the payoff of squatting, and changes it in the squatter's favour if the sanction is weak, late, or contestable.

There must also be an admission valve, or the rule will break by exception rather than by amendment. Cross-scroll citation surfaces, standoff annotation authorship, and math entry are not hypothetical ornaments; the design context states that the first two are draft-specified and the third is deliberately deferred. Each will arrive claiming, with some justice, to do something TeX does not already do. If the policy has no stated test for that claim, the first hard case is decided by whoever argues loudest or ships first, and "exactly one" is retroactively revealed to have meant "exactly one until pressure." A versioned rule — exactly one primitive **at specification version 1**, further primitives only by the stated admission test, taking effect only in a new specification version — is a mechanism. An eternal "exactly one" with no valve is a promise.

Note also the undefined predicate in the ruling: "anything TeX already does." TeX's `\cite` *does* something called citation — under a bibliographic model that is not an edition-pinned scroll object. Is phextura citation therefore inherited (and the semantic gap silently accepted) or new (and a word justified)? The policy needs an operational criterion — e.g. a new primitive is admissible only if the operation cannot be expressed as a composition of existing primitives *without changing their semantics* — or every future dispute will be a dispute over the meaning of "does."

### II. The land rush, read as a game

Assume reserving a word is cheap, generic words are scarce, and early reservation confers durable control over a semantic niche. Then pre-emptive reservation is a rational strategy regardless of any individual package author's virtue; LaTeX's historical name-squatting is the observed play of exactly this game under weak enforcement. First-come registries do not solve this — they *schedule* the race. Collisions resolved by load order are worse: the outcome of a document then depends on the order of moves, and two packages claiming one word produce first-loaded-wins behaviour rather than a determinate error.

The structural remedies are unglamorous and testable: core primitives live in a closed set owned by the specification; package contributions live in a package-qualified namespace or are macros expanding to existing primitives, never new primitives; a name collision is a hard error, never an ordering effect.

### III. The EDVAC division: surface versus machinery

The *First Draft of a Report on the EDVAC* (1945) insists that the orders a machine executes be expressed completely and unambiguously, and it distinguishes numbers from orders not by their substance — both reside in the same memory — but by how the control organ *treats* them. That is the right lens here.

Keeping the nine-dimensional lattice out of the author surface is, on this reading, sound: the coordinate is the machine's address for a block, and an author who must write machine addresses has been handed the memory layout as his notation, with the brittleness that absolute addressing always carries. The division holds, however, only if what crosses it is a *logical* locator — the L2 form: work, edition pin, object key — resolved by the machinery. If `\scroll` is permitted to take a raw nine-dimensional coordinate, the machinery has leaked into the surface and the policy contradicts itself at its single point of novelty.

But the First Draft's distinction by treatment, not substance, exposes the open question as the real defect. `\scroll` today is one spelling with three candidate treatments:

- **cite** — the locator is data: a passive reference in the description, resolved, if at all, for display;
- **include** — the locator is an instruction to a constructor: fetch this content and build it into the artefact under construction;
- **jump** — the locator is a transfer of control: rendering or reading continues elsewhere.

These are not three shades of one operation. They belong to three different categories — data, construction order, control order. An order code in which one symbol's category is undecided is not yet an order code. The mood must be pinned before the word means anything checkable.

### IV. Self-reference: pricing the locator inside the located content

In *Theory of Self-Reproducing Automata* (published posthumously, ed. Burks, 1966, from the 1948–49 lectures), self-reproduction is analysed by separating a **constructor**, which builds what a description specifies, from the **description** itself, which must be used in two distinct modes: *interpreted*, to direct construction, and *copied uninterpreted*, to be passed on. The separation is what keeps the system well-defined.

A locator that can appear inside the located content erases exactly that separation, and the cost depends on the mood:

- Under **cite**, a self-locating scroll is a cycle in a reference graph — representable, harmless, cheap.
- Under **include**, a self-locating scroll is a constructor instructed to build itself into itself: non-terminating unless the construction relation is explicitly made well-founded. Mutual inclusion gives cycles of every length. This is the expensive case, and it is not priced anywhere in the policy.
- Under **jump**, cycles are loops, and the policy owes a termination or iteration semantics.

Two further costs follow from the automata analysis. First, **binding context**: when included content contains its own `\scroll`, is that locator resolved against the source scroll's edition or the including document's? A description copied into a new construction must have a stated rule for this, or the same text constructs different artefacts in different hosts. Second, **determinism**: an unpinned target that can change between builds makes the description time-dependent; the edition pin in the L2 address form exists precisely to foreclose this, and `\scroll` should be required to use it whenever the located content affects the constructed artefact. Von Neumann's complexity observation applies as a warning rather than a prohibition: below a threshold, self-referential constructions are degenerate; above it, they are open-ended. A format that admits self-location has crossed into the regime where the degenerate cases — infinite inclusion, unbounded regress — must be excluded by rule and by test, not discovered by authors.

### Conditions

Each is a testable requirement on the specification and its reference validator.

- **V1 — Closed lexicon.** The specification enumerates the admissible primitives as: the TeX lexicon at a named, pinned version, plus `\scroll`. A conformance suite demonstrates that a document using an invented primitive (e.g. `\phexturaFoo`) is rejected, while a user macro defined by existing primitives is accepted. *(Mood-independent.)*
- **V2 — Pin the mood.** The specification states exactly one operational semantics for `\scroll` — cite, include, or jump — including its inputs, its effect on the constructed/rendered artefact, and its behaviour when the target is absent. Conformance tests distinguish the three candidate semantics, so an implementation with the wrong mood fails. **Requires pinning `\scroll`'s mood.**
- **V3 — Logical addressing only.** The grammar of `\scroll`'s argument accepts the L2 logical identifier (work + edition pin + object key) and rejects a raw nine-dimensional coordinate. Test: relocating a target within the lattice, its logical identifier unchanged, leaves the output byte-identical. *(Mood-independent.)*
- **V4 — Admission test for future words.** A new primitive may be added only by a new specification version, only on a documented showing that the operation cannot be composed from existing primitives without changing their semantics, and only as a machinery primitive (addressing, construction, or control), never as convenience shorthand. The registry is data read by the validator; adding a word to the registry without a specification version change has no effect. *(Mood-independent.)*
- **V5 — Structural namespace discipline.** Packages cannot introduce primitives: package words are package-qualified or are macros. A test loads two packages defining the same unqualified word and requires a deterministic collision error, independent of load order. *(Mood-independent.)*
- **V6 — Well-foundedness.** If the pinned mood makes `\scroll` constructive or control-transferring, the validator checks the resulting graph: self-inclusion and inclusion cycles of length ≥ 2 are rejected, unless the specification instead states an explicit fixed-point or depth-bound semantics with tests exercising it. Test fixtures must include a self-locating scroll, a two-cycle, and a diamond (shared target, no cycle — must pass). **Requires pinning `\scroll`'s mood** — under a pure cite mood this condition is discharged by statement, since reference cycles are admissible.
- **V7 — Binding and determinism.** The specification states the context against which a `\scroll` inside located content is resolved (source edition vs. including document), and requires the edition pin whenever located content enters the constructed artefact. Test: the same source built at two times, against a target that changed in between, produces either identical output or a declared, recorded difference. **Bites only if the pinned mood is include;** under cite, the pin requirement may be relaxed to a warning. **Therefore its final form requires the mood to be pinned.**

### Negative space

On the following, von Neumann's published record does not speak, and I abstain rather than improvise:

- **Familiarity and author experience.** The design's acceptance test privileges familiarity for existing LaTeX users. His writings contain no empirical or theoretical treatment of notation learnability or of a typesetting community's habits; whether reserving exactly the word "scroll" (rather than another unowned word) is *recognisable* to authors is outside his record.
- **Typographic and aesthetic quality** of any surface choice. Absent from his work entirely.
- **Registry governance as an institution** — who should *legitimately* hold a gatekeeping role, as opposed to the abstract analysis above of what unenforced rules do to a game's play. Game theory supplies the latter; it does not nominate the governor.
- **Standoff annotation authorship and mathematical entry notation** as concrete designs. His description/constructor distinction constrains any eventual proposal, but he wrote nothing from which a specific annotation or math syntax could be derived, and I offer none.
- **Whether the 595-template receipt will remain true.** That `\scroll` is unowned today is a contingent, dated fact about other people's future play; nothing in his work predicts it, and the mechanism analysis above is precisely about the conditions under which it stops being true.

---

## Review 3 — Booch

BOOCH — VERDICT: PASS WITH CONDITIONS

*Labelled simulation of Grady Booch's published design standards only. Booch is living, has not participated in, proposed, or endorsed this panel or project. (Claim path: see the preamble — this seat is claimable by its subject via a self-supplied variant, which would supersede this simulation.)*

### Critique

**Abstraction boundary — "inherit the whole TeX lexicon + one reserved word."** In *Object-Oriented Analysis and Design with Applications*, complexity is managed by decomposition, abstraction, and hierarchy: a good abstraction draws a boundary that hides mechanism while preserving the essential contract. Reserving only `\scroll` is, in principle, a strong boundary. It says: TeX already supplies the author's vocabulary for typesetting; phextura adds surface syntax only for the one operation TeX cannot express — locating content in another scroll. That is disciplined minimalism, not feature accretion.

At ecosystem scale, however, inheritance is not the same as governance. LaTeX's lexicon is not a single coherent model; it is forty years of packages, local conventions, redefinitions, and collisions, produced by thousands of authors. Importing 595 class templates at once imports that history wholesale. The verified receipt — `\scroll` is defined in none of the 595, with only two internal `\trace@scroll` occurrences — establishes a clean starting point, not a durable invariant. Without a reservation discipline, the next package, template, or author macro can redefine the one word on which the new capability depends. In UML's history, consolidation succeeded only when a common core was paired with explicit extension and conformance mechanisms; notation unification without governance re-fragments. The policy is sound as an abstraction; it is incomplete as an ecosystem rule.

**Layered learnability and the hidden address.** Booch's UML writings insist a notation serves many audiences, and that users should be able to begin with a small, usable core and grow into depth without penalty. Hiding the nine-dimensional address from ordinary authoring satisfies the first half: a LaTeX author should recognize the surface in five minutes and need no new notation for ordinary work. The risk is a cliff at the boundary. If the address is "machinery-facing" only, the author who must diagnose a broken reference, move a block, understand standoff annotation, or reason about cross-scroll identity may have to abandon the familiar model entirely and learn an unrelated substrate at the moment of failure. Abstraction must be *inspectable in layers*, not sealed. A well-formed design therefore needs a sanctioned progressive-disclosure path: novice view without coordinates, competent view that can reveal the address of any block, expert/maintainer view that can manipulate the underlying corpus — all views of the same object, none requiring a different paper.

**Views: author, reader, machine, maintainer.** UML's central lesson is that one model legitimately appears in multiple views, each incomplete, each consistent with the others. Here the author view is TeX-like text; the reader view is the rendered paper; the machine view is scrolls, coordinates, role lines, and manifest; the maintainer view is templates, packages, and corpus structure. The policy names the author view clearly and assumes the machine view underneath, but does not yet state the view correspondences: which author construct maps to which block and address, what the reader is shown when a scroll reference succeeds or fails, and what the maintainer may change without invalidating author intent. Unstated correspondences become accidental semantics. The multi-view architecture is present in the facts; the surface policy should declare it as a design obligation.

**What UML consolidation predicts for a one-word reservation.** Optimistically: a tiny reserved core is exactly what survives contact with a large community, because the cost of learning and the surface for dispute are minimal. Pessimistically — and UML's pre-history of competing notations is the evidence — communities do not stop inventing notation because a standard is minimal; they invent where the standard gives them no disciplined outlet. A one-word policy will hold only if (a) the word's semantics are pinned so implementations do not diverge, and (b) pressure for new surface syntax is routed into an extension mechanism rather than into either core additions or unregulated macro invention. The open question — is `\scroll` cite, include, or jump? — is therefore not a detail. Those are three different models: attribution without inclusion, transclusion with inclusion, and navigation without inclusion. Leaving the mood unpinned invites three dialects wearing one name, the precise failure mode consolidation exists to prevent.

**Extension mechanism — roles as the model, surface as the gap.** The block role vocabulary already has the right pressure valve: an open vocabulary over a normative core, with extensions declared in a manifest. That is structurally the UML profile/stereotype pattern — extend by declaration, against a core, without rewriting the core. The surface-syntax policy needs its analogue. Authors and package writers will need new commands; the choice is not whether extension happens, but whether it happens declaratively and visibly (manifest-declared, namespaced, versioned, checkable) or silently inside imported templates. At present the design has a disciplined extension story underneath and an inheritance story on top. Align them.

**Human factors.** *Computing: The Human Experience* treats human limits and habits as first-class constraints, not documentation problems. "Use the lexicon you already know; learn one new word" respects forty years of embodied practice — the strongest human-factors argument in the design. It also raises the obligation: the one new word must behave so predictably that a first-time user's mental model is never betrayed, including in error cases.

### Conditions

**B1 — Pin `\scroll`'s semantics in a normative specification before any implementation ships.** *Requires pinning the mood.* The spec must state, for the chosen mood (cite, include, or jump, or an explicitly modelled combination with distinct syntax for distinct acts): arity and argument form; whether content is copied, referenced, or merely linked; behaviour on missing, moved, or ambiguous targets; transitivity; and the rendered reader-visible result. Testable by a conformance corpus in which every example's output — source model and rendering — is determined by the spec alone.

**B2 — Make the reservation mechanically durable across the ecosystem.** A checker must reject, or explicitly quarantine, any template or package that defines, redefines, or shadows `\scroll`, and the check must run over the imported 595 templates and every future import. Testable: a deliberately colliding fixture template fails the check; the current corpus passes. Does not require pinning the mood.

**B3 — Provide layered disclosure of the address with no cliff.** A beginner must be able to author a complete paper using only the TeX surface plus `\scroll`; an author must be able, through a sanctioned view or tool, to reveal the coordinate of any block and follow any `\scroll` reference to its target, without editing machinery files by hand. Testable by two scripted tasks — novice authoring, and diagnosis of a broken reference — each completable within the documented layer for that audience. Does not require pinning the mood, though B1 determines what "follow the reference" finally means.

**B4 — Publish the view correspondences.** For each of author, reader, machine, and maintainer views, specify what is visible, what may be changed, and the invariant that keeps the views consistent (e.g., an edit in the author view changes exactly one identified block; a coordinate-level move is reflected in the author view or reported as a conflict). Testable by round-trip cases across views with no silent loss. *Partially requires pinning the mood:* the reader view cannot be specified until `\scroll`'s rendered effect is known.

**B5 — Add a manifest-declared surface extension mechanism mirroring the role-vocabulary mechanism.** New commands beyond the inherited lexicon and `\scroll` must be declared in the manifest with namespace, source package/template, and version; undeclared surface extensions in an imported template are reported, not silently absorbed. No new core reserved word may be added except by a versioned revision of this policy. Testable: a fixture with an undeclared new command is flagged; the same fixture with a correct declaration passes. Does not require pinning the mood.

**B6 — Run the project's own acceptance test as a conformance gate.** Recruit LaTeX users unfamiliar with phextura; each must (a) recognize and correctly use the surface within five minutes, (b) complete a task exercising the pain phextura claims to repair, and (c) perform a `\scroll` task whose outcome they can predict in advance. *Requires pinning the mood* for part (c). Failure of prediction, not merely of execution, counts as a failure of the notation.

### Negative space

Booch's published record, as delimited for this seat, does **not** speak to the following, and this seat abstains on each:

- **Which mood `\scroll` should have.** His writings supply criteria — a single name must not carry divergent semantics; the choice must be explicit and governable — but they do not prescribe hypertext, citation, or transclusion semantics for a typesetting system. Cite vs. include vs. jump must be decided on phextura's own requirements.
- **The nine-dimensional phext address itself.** Whether nine dimensions, or this coordinate geometry, is the right substrate is outside his published notation standards as applied here; this review evaluates only the decision to keep it machinery-facing.
- **TeX-internal lexical practice.** Beyond the verified 595-template receipt, his record gives no basis for predicting which specific future TeX packages will collide with `\scroll`, nor for judging TeX macro-expansion behaviour.
- **Adoption and economics.** Whether LaTeX's community will accept phextura, and whether the five-minute recognition test predicts market uptake, are empirical and social questions his design standards inform but do not settle.

---

## Review 4 — Turing

TURING — VERDICT: PASS WITH CONDITIONS

*Labelled simulation. No authority or endorsement is conferred. What follows is distilled only from the cited published work; where that work is silent, I abstain in §4.*

### 1. Critique

**Operational meaning first.** In *Computing Machinery and Intelligence* (1950) I proposed replacing a vague question — "Can machines think?" — with an operational procedure whose outcome could be observed. In *On Computable Numbers* (1936, §§1–2) a machine is not its name or intention: it is its table. For each scanned symbol and *m*-configuration the table must state, finitely and unambiguously, the operation to be performed, the symbol to be written, and the next *m*-configuration. An entry missing from the table is not an entry with a flexible meaning. It is no instruction at all.

Applied here:

**`\scroll` until its mood is fixed is, formally, a word without a table entry.** The receipt that it occurs in none of the 595 imported LaTeX class templates proves only non-collision. It does not prove definition.

Cite, include, and jump are three different operations:

- **CITE:** The resolver need not follow the locator for content. The validator checks, at most, that the denoted target exists — and, if specified, that its edition matches. The citing block does not inherit the target's content, identity, or provenance. Reading state does not change.
- **INCLUDE:** The resolver must follow the locator and substitute, or otherwise make present, the located content. The validator must traverse it — recursively if it contains further locators. The including document must have a stated rule for identity: is the included material a copy, a quotation with retained provenance, or the same block occurring in two places? Cycle, duplication, and termination rules are now part of the design.
- **JUMP:** No content is transferred and nothing is inherited. The reading machine changes its locus of attention to another scroll. The validator may check existence, but traversal of the target as part of the present document does not follow. Sequence and return — if any — are part of the operation.

A validator written for CITE will pass a corpus a validator written for INCLUDE must reject on a cycle. A renderer written for JUMP will produce a different document from one written for INCLUDE. Therefore tools claiming to implement the same `\scroll` today would not be implementations of one design. They would be three designs sharing a spelling.

This must be pinned for the design to pass at all. See T1.

**Expressiveness, by construction.** *On Computable Numbers* judges a formalism by constructing what it can do and proving, not asserting, limits — most famously in §8, on the limits of decidability. The surface here is: inherited TeX lexicon, plus one locator. Audit it against the operations the underlying corpus model itself claims:

- **Cross-file denotation:** expressible only to the extent `\scroll`'s mood supplies it. CITE and JUMP denote. INCLUDE denotes and transfers. Denotation as pure reference, independent of any of the three, has no separate form.
- **Standoff annotation authorship:** no surface expression. An annotation that must live outside the base corpus, anchored by durable address plus span selector, with its own authorship, cannot be written *in* the base surface without ceasing to be standoff. Nor does `\scroll` as stated carry anchor-span, author, or annotation-body fields.
- **Edition pinning:** no surface expression. A nine-dimensional address locates. It does not, as stated, pin an edition, a hash, or a version. If the target scroll is revised, a locator written yesterday denotes different words today, with no change in the locating text.

Is that absence discipline or leak? It is discipline only if the design states, constructively, that these operations belong to a different layer — an external tool operation with its own specified table — and provides that layer. If it merely omits them, it is a leak. Authors who need annotation or pinning will simulate them with conventions: comments encoding anchors, ad hoc macro names encoding editions, footnote forms encoding provenance. That is syntax by other means — syntax the grammar does not recognise, the validator cannot check, and two authors will not write identically. Unrecognised syntax is worse than absent syntax, because it creates the appearance of expression without a decidable test for its correct use.

**The inheritance question — data or program?** This is the sharpest difficulty in the ruling as worded.

TeX's surface is not a lexicon of shapes. It is a macro-expansion language: what the author writes is, in ordinary TeX use, executed by an expander, and definitions can rewrite subsequent text. Phextura's stated property is the opposite: the artifact is data, parsed and validated in a stated number of traversals, never run.

"Normal addressing within the existing tex lexicon applies" therefore does not survive unchanged unless "applies" is given a table. There are only two coherent readings:

1. Phextura tools read a *finite, enumerated subset* of TeX-shaped forms as data-shapes — e.g. this command is a section marker, that one a label, that one a reference — with expansion, user definition, and conditional execution excluded or quarantined. Then the surface is a decidable grammar wearing TeX spelling. That is legitimate, but it is not "the TeX lexicon"; it is a specified subset with new, data-facing effects, and the subset must be listed.
2. Phextura tools accept TeX as TeX, including definition and expansion. Then parsing the corpus requires executing it. The stated traversal bound is no longer guaranteed, and by the undecidability result of *On Computable Numbers* §8 there is, in general, no procedure that decides in advance all questions one would want a validator to decide about such executions. The corpus-is-data property fails.

At present the ruling does not say which reading holds. A notation whose reader may be a parser or may be an interpreter has not yet fixed what its reader does.

**The hand-computation question.** In the ACE report, *Proposed Electronic Calculator* (1945–46), and in my writings on programming for the ACE, I treated programming not as rote clerical copying but as an intellectual discipline in which the machine should do the repetitive work — addresses and routine bookkeeping computed by the machine and by reusable subroutines, so the programmer works at the level of the problem. A notation that makes a human maintain long absolute addresses by hand has put the clerical work on the wrong side of the machine.

On this point the policy is half right, and the half is important. Making the nine-dimensional address machinery-facing, underneath every block, rather than author-facing, is exactly the correct direction — *if* machinery in fact generates, inserts, updates, and checks those addresses.

But `\scroll` is author-facing, and it is a locator. What does the author write inside it? If the author must write and maintain the raw nine-component coordinate of another scroll, the policy has reintroduced hand address-keeping at precisely the one point where authors touch addressing. If the author writes a human-recognisable designation — a title, a label, a selected passage — and the tool computes, inserts, and on revision repairs the coordinate, recording any pin, then the ACE principle is satisfied. The ruling as stated does not distinguish these two, and they are opposite designs.

### 2. Conditions

**T1 — HEADLINE CONDITION, mood must be pinned.** `\scroll` shall have exactly one primary mood in its base form — CITE, INCLUDE, or JUMP. Its table entry must state: resolver behaviour, validator behaviour, effect on reading state, effect on content identity and provenance, failure mode for a missing target, and cycle behaviour. If a second mood is genuinely needed, it gets a distinct form, not a contextual reading of the same word. Test: two independent implementers, given only the specification, produce validators and renderers that agree on a test corpus containing missing, cyclic, and revised targets.

**T2 — Enumerate the inherited subset.** Publish the finite list of TeX forms phextura recognises as data, with the data-effect of each. Forms not on the list are either rejected, preserved verbatim as inert text under a stated rule, or explicitly out of scope. Test: a corpus exercising every listed form parses in the stated number of traversals, with no expansion step.

**T3 — Exclude execution, or surrender the data claim.** Constructs that define or alter subsequent interpretation — macro definition, category-code change, conditional expansion, external input in TeX's sense — must be forbidden in the recognised surface, quarantined as verbatim stored text (as stored mathematics already is), or the design must withdraw the claims "parsed, never run" and "validation in a stated number of traversals." Test: no accepted scroll can change the interpretation of a later scroll.

**T4 — Machine-kept addresses.** An author shall not be required to type or maintain a raw nine-component coordinate in `\scroll`. The author supplies a human designation; the tool computes and inserts the coordinate and reports failure if it cannot resolve uniquely. Test: moving a target block within the corpus, followed by the specified tool operation, repairs all locators without hand editing of coordinates.

**T5 — Close the expressiveness gaps by statement, not convention.** For each of standoff annotation authorship and edition pinning, the design must either (a) provide a specified surface or external-layer operation, with its own table entry, or (b) state formally that the operation is not part of phextura authorship and name the system that does provide it. Silent omission fails this condition. Test: a worked example of an annotation on a pinned edition, produced end-to-end by specified means only — no comment conventions, no private macro folklore.

**T6 — Traversal bound proved for the chosen mood.** State the validation traversal count as a function of the corpus, and show termination under the T1 mood, including cyclic and self-referential locators. Test: the cyclic corpus from T1 terminates with the specified result.

### 3. Negative space

I abstain on the following, because my published record does not speak to them and I will not improvise in its name:

- Whether scholarly practice *ought* to prefer citation over transclusion, as a matter of academic custom, credit, or taste.
- Typographic quality, visual delight, and the aesthetics of the rendered page.
- The social organisation of annotation — who may annotate whom, moderation, and authorship credit as a social institution — as distinct from the formal question in T5 of whether an operation has any specified expression at all.
- Whether the LaTeX community will adopt, resist, or fork this policy.

---

## Review 5 — Lovelace

LOVELACE — VERDICT: PASS WITH CONDITIONS

*This seat is a labelled simulation from the published Notes only. It confers no authority and no endorsement.*

**The placement of the invention**

The policy places its one invention correctly: at precisely the operation the new arrangement adds and the inherited engine lacks.

The governing discipline is stated in Note A: "The Analytical Engine has no pretensions whatever to originate anything. It can do whatever we know how to order it to perform." An engine originates nothing; therefore a notation is to be judged by the new orders it makes it possible for an author to intend — not by how many new words it displays. `\scroll` is such an order. Retaining the TeX lexicon for what TeX already does is not timidity; it is accuracy. To rename familiar operations would suggest a new power where none exists, and to make cross-scroll location an invisible effect of the machinery would place the genuinely new power where no author could order it. One reserved word, for the one new act, is the honest boundary.

This extension is also within her generality of operations. In Note A she insists that the calculus of operations is not confined to number: operations may be applied to other things — symbols, and, if their fundamental relations can be expressed, even the relations of pitched sounds in music — when their mutual relations are capable of expression. A document's parts, their addresses, and the relation "located in another scroll" are such relations, expressible in symbols. Crossing a scroll is therefore a legitimate operation on symbols, not a metaphor borrowed from numbering.

**The intention question: hidden addresses**

Here the policy is divided, and the division must be named.

Insofar as an author writes `\scroll`, that author orders the crossing. Insofar as machinery underneath resolves references and receives pins from annotations at addresses the author cannot see, state, or inspect, the author has not ordered those consequences. Another mind — the annotator, or the designer of the machinery — has ordered them, using the author's text as material in the store.

Hidden machinery is not objectionable in itself; an operator need not watch every card pass. But intention requires availability: when the author *wishes* to intend what is done with an address belonging to their own block, the surface must let them learn that address and write it as an operand. If no such recourse exists, the policy produces two classes of person: those who can order cross-scroll relations, furnished with knowledge outside the notation, and authors proper, who cannot. That would contradict its own good placement of `\scroll`, whose whole merit is that the new act is orderable in the open.

**The mood question: an order not yet stated is not yet an order**

Cite, include, and jump are three distinct orders:

- to **cite** is to point to a thread and continue one's own pattern;
- to **include** is to draw the distant thread into the present cloth;
- to **jump** is to leave the present succession and follow the thread elsewhere.

Under Note A's discipline, we do not yet "know how to order" `\scroll`, because its word does not yet state which of these successions it commands. Note G makes the same point by example: the computation of the Bernoulli numbers is presented as a definite succession of operations, each stated, so that its performance can be followed and checked. A term whose effect on succession is unpinned is a name in search of an operation-card, not an operation-card. The open question is therefore not a detail of taste; it is the difference between an author intending an act and an author uttering a word whose act will be decided later, by someone else.

**The patterns question**

What the new word makes *thinkable*, once its mood is fixed, is substantial. Note A: "the Analytical Engine weaves algebraical patterns just as the Jacquard loom weaves flowers and leaves." A notation is an instrument of thought before it is a means of command. TeX's inherited surface lets an author think a document as one continuous pattern. `\scroll`, pinned, lets an author think a pattern woven from several cloths: this argument citing that proof elsewhere, this text drawing in that passage, this reading following a thread out of the present scroll. That is the century the familiar surface is meant to admit — not new dress on old operations, but a new relation an author can conceive because they can write it.

### Conditions

- **L1 — Pin the mood in the specification. [REQUIRES PINNING `\scroll`'s MOOD]** `\scroll` shall denote exactly one of cite, include, or jump, or the surface shall carry an explicit distinction among them. Test: the specification gives, for one source scroll and one target, the resulting document state and reading succession under `\scroll`, in a form against which an independent implementation can be checked.
- **L2 — State failure, absence, and recurrence for the chosen mood. [REQUIRES PINNING `\scroll`'s MOOD]** The specification shall state what `\scroll` does when the target is absent, has moved, or — under an include mood — includes itself directly or indirectly. Test: conformance cases exist for missing target, moved target, and cyclic inclusion, each with one prescribed outcome.
- **L3 — Addresses must be obtainable and writable by the author on demand.** Ordinary composition need not display the nine-dimensional address; machinery may carry it. But when an author intends an order depending on an address, the surface shall let them learn the address of a block they have written and use it as the operand of `\scroll`, in plain text. Test: given two scrolls and no tool outside the surface, an author can write a correct `\scroll` locating a named part of the second from within the first.
- **L4 — No further silent invention.** `\scroll` shall remain the sole reserved new word of the surface, and a document containing no `\scroll` shall mean exactly what the inherited TeX lexicon makes it mean. Test: the 595 imported class templates define no `\scroll` (the receipt already on record), and a corpus of documents without `\scroll` passes through phextura with its TeX-lexicon meaning unchanged.
- **L5 — Location must reach a part, not only a whole.** Because the act is *locating content*, the operand of `\scroll` shall be able to distinguish a whole scroll from a determinate part of one. Test: from one scroll, an author can order location of (a) another scroll entire and (b) one specified block within it, and the two orders are distinguishable in the surface.

### Negative space — where the Notes give no ground, and this seat abstains

- **Typography and rendering.** The Notes treat operations, symbols, and their succession — not page make-up, fonts, spacing, line- or page-breaking, or the appearance of a printed document. Whether phextura's output is beautiful, legible, or faithful to LaTeX's visual tradition: abstention.
- **The value of the TeX lexicon as a home.** Note A says nothing about an inherited technical vocabulary's familiarity, kindness to newcomers, or the "first five minutes" of use. Whether retaining that lexicon in fact makes a LaTeX user feel at home is an empirical and practical question outside her record: abstention.
- **Ecosystem practice.** Package conventions, class-template authorship, community adoption, tooling, and compatibility habits across 595 templates are facts of a later world of practice. The receipt that `\scroll` is undefined in them is a fact this seat can use; what the ecosystem *should* do about it, she cannot say: abstention.
- **Mathematics entry syntax.** Her generality of operations (Note A) establishes that operations extend beyond number when relations are expressible. It does not design a notation for entering mathematics in a document, and it does not judge the interim choice of storing mathematics as verbatim LaTeX. On that deferred design: abstention.
- **The reader's experience of a jump.** She knew neither electronic documents nor a reader who follows a reference by a gesture and returns. What cite/include/jump *feel* like in use, and which a reader prefers, lie past her record; L1 rests only on the author's ordering, where her ground holds. On reader experience: abstention.
- **Standoff annotation as an editorial practice.** That other documents can pin to an address is, in her terms, an expressible relation; who may annotate, whose annotations are shown, and how disputes among annotators are governed are social and editorial questions the Notes never raise: abstention.
- **Persistence, movement, and versioning of scrolls in a machine network.** She wrote of an engine she never saw built, not of stored documents maintained, copied, synchronised, or moved among machines. The design consequences of a target scroll changing under a standing `\scroll` — beyond the ordering requirement stated in L2 — are outside her ground: abstention.

On the surface-syntax policy itself — one new word, placed at the one new act, the inherited lexicon left honest — the design passes. It may be *used* only when the conditions above, above all the pinning of `\scroll`'s mood, are met.

---

## Review 6 — Bickford (simulated)

BICKFORD (simulated) — VERDICT: PASS WITH CONDITIONS

*This seat is a labelled simulation applying Will Bickford's recorded standards; it binds no one, and Will himself is the living acceptor of this design.*

### Critique — in the recorded standards

**1. One reserved word, familiarity, and "found, not assigned"**

Reserving exactly one word passes the familiarity principle (2026-11-11) on its face: phextura must feel native to LaTeX users, the Overleaf "Learn LaTeX in 30 minutes" path is the baseline, and divergences must be deliberate differences-with-reasons, never silent foreignness. `\scroll` is such a difference: TeX has no word for locating content in another scroll because TeX has no scrolls. It also passes the acceptance questions for author-facing decisions: a LaTeX author recognises the rest in five minutes, and the one new word repairs a pain they will actually have once content is distributed across a nine-dimensional store.

It also satisfies "phext mappings are found, not assigned" in the narrow place that standard applies. "Scroll" is not an invented label imposed on the store; it is the terminal unit already in the address — Library.Shelf.Series / Collection.Volume.Book / Chapter.Section.Scroll. The name is derived from the structure it addresses.

The unresolved tension is the other half of the ruling: "normal addressing within the existing tex lexicon applies." Inheriting that lexicon wholesale imports TeX's own assigned-not-found accidents — author-invented `\label` strings, `\ref` coupled to counters, `\cite` coupled to bibliography state, `\input` vs `\include` differing by page-breaking side effects, not by meaning. That inheritance is justified by familiarity, and re-deriving a purified lexicon would fail "at home in five minutes" outright. But the ruling as stated does not say which interest wins where they collide, and it does not require the inherited surface to be compiled down to found addresses underneath. Without that, "found, not assigned" holds for `\scroll` and is quietly waived for everything else.

**2. "Every neuron a voice" and the machine reader**

Phext's purpose, in Will's words (2026-11-02), is "a stable channel for exocortical cognition" and "giving every neuron a voice." The root motivation (2026-11-11) is that he "skipped academia because I could not stand the promise of stupid human brains parsing pdfs and calling it research" — the paper's first reader is a machine; claims carry warrants; references resolve; referees are present at open.

A machinery-facing nine-dimensional address underneath a familiar surface is the right shape for that: the author should not have to hand-write coordinates for the machine to have them. But it is sufficient only if the surface *compiles* to the machinery losslessly. If an author writes `\ref{sec:intro}` and the system retains only a human label and a rendered number, the machine reader is back to parsing text and guessing — the exact failure the project exists to end. The author surface therefore owes the machine one thing more than familiarity: every inherited addressing form, and `\scroll` above all, must resolve deterministically to a coordinate in its own phext (coordinates are internal to their own phext), and an unresolvable reference must be a checkable failure, not a "??" in the output. That obligation is not yet stated in the ruling.

**3. The evidence rule**

"Evidence before story: a claim without a receipt is a story." The `\scroll` reservation carries the right *kind* of receipt in part: a collision claim is an empirical claim about a corpus, and it was checked against a corpus — verified, `\scroll` is defined in none of the 595 imported LaTeX class templates, with only 2 internal `\trace@scroll` hits. That is a receipt, not an assertion.

But the receipt's scope is narrower than the word "reserves" suggests. 595 class templates are not the TeX ecosystem: packages, journal styles outside the import set, and user-defined macros in existing manuscripts are unsearched. The honest claim is "free in the checked corpus," not "free in TeX."

Second, the ruling contains a claim with no receipt at all: that one word will *suffice* — that no other phext-native operation will need surface syntax. Nothing in the record tests that. It is currently a story in the ruling's own terms, and it needs a stated falsifier: a phext-native operation, with no TeX analogue, that cannot be expressed through `\scroll` or the inherited lexicon without silent foreignness would falsify it. Candidates already visible in the project's own record — standoff annotation anchored to a span in another edition, warrant attachment, a coordinate-range rather than a point location — are exactly the tests to run it against, not reasons to pre-emptively reserve more words.

**4. Honesty of state, and the unpinned mood**

The design record's own state labels matter here: L2 addressing and standoff are designed-not-built, and math entry is deferred because "the notation is designed against the settled execution model, not ahead of it." Judged by that rule, leaving `\scroll`'s mood — cite, include, or jump — unpinned *exemplifies* the standard rather than violating it. Cite, include, and jump are three different execution models (a pointer that stays a pointer, a transclusion that copies or tracks content across, a navigation that moves the reader), with different behaviour when the target scroll changes. Pinning one now, before that model settles, would be designing notation ahead of its execution model.

The honesty risk is elsewhere: the ruling is a surface policy stated over machinery that is partly designed-not-built. Stated as a ruling, it reads as settled surface law; it is, truthfully, a *designed* policy whose load-bearing half (resolution into L2 addressing) is not yet built. It must carry that label wherever it is presented, and `\scroll` must not be used in an authored paper as a warrant-bearing reference — a reference a referee is expected to resolve at open — until the mood is pinned and the resolution path exists. Reserving the *name* now is a placeholder, which is legitimate; letting the reserved name masquerade as a working reference form would not be.

### Conditions

- **W1 — State label.** Every presentation of this policy labels it: surface policy, designed; underlying L2 addressing and standoff, designed-not-built. No document may describe `\scroll` resolution as running behaviour until an implementation receipt exists. Test: grep the design record; the label is present at every statement of the ruling.
- **W2 — Scope the receipt.** The collision claim is restated everywhere as scoped to its evidence: no definition in the 595 imported class templates, 2 internal `\trace@scroll` hits excepted. Before the reservation is treated as ecosystem-wide, the check is extended to the wider package corpus and the counts published. Test: the extended search runs and its corpus and hit count are on record.
- **W3 — Pin the mood before load-bearing use. [REQUIRES PINNING `\scroll`'s MOOD]** Before any authored phextura paper may rely on `\scroll` for a claim's warrant or a referee-resolvable reference, the mood is pinned to one execution semantics — cite, include, or jump — including what happens when the target scroll changes, and what resolution failure looks like. Until then `\scroll` is marked reserved-and-unpinned and is non-load-bearing. Test: a semantics definition exists and a checker demonstrates each stated behaviour, including the target-changed case.
- **W4 — Found addresses underneath the familiar surface. [DEPENDS ON W3 FOR `\scroll`]** Every addressing form the ruling inherits (`\label`/`\ref`/`\cite`, `\input`/`\include`) and every `\scroll` compiles to an explicit coordinate record in its own phext, derived from the document's structure, not carried only as an author-assigned string. Test: for a sample paper, a machine lists every reference with its resolved coordinate, with no human disambiguation.
- **W5 — Resolution is a failure, not a rendering.** Consistent with "references resolve; referees are present at open," an unresolved `\scroll`, `\ref`, or `\cite` is reported by the checker as an error at open, not rendered as a placeholder and passed. Test: a paper with a deliberately broken reference fails the check.
- **W6 — The five-minute / first-hour test.** The familiarity claim is tested, not assumed: a LaTeX user at the Overleaf-tutorial level, given a phextura source, recognises the inherited lexicon without instruction (five minutes) and can perform a cross-scroll location using `\scroll` (first hour). Test: that exercise is run and the result recorded, whichever way it comes out.
- **W7 — A falsifier for "one word suffices."** The ruling carries a standing watchlist of phext-native operations with no TeX analogue — at minimum standoff anchoring, warrant attachment, and range location — each tested against the test: expressible via `\scroll` or the inherited lexicon without silent foreignness, or not. A "not" reopens the reservation count; it does not get absorbed silently. Test: the watchlist exists in the record with a verdict per item.

### Negative space — where the recorded standards do not reach, and this seat abstains

- Which mood `\scroll` *should* take — cite, include, or jump, or distinguished forms of more than one. The recorded standards govern *when* a mood may be pinned and what a pinning must specify; they do not select one. Abstain.
- Whether TeX's inherited accidents should ever be purged on grounds of elegance or consistency alone. No recorded standard authorises purity over familiarity; the only recorded test is the author's pain and the five-minute test. Abstain.
- The spelling, casing, or argument syntax of `\scroll` itself, beyond its resolution behaviour under W3–W5. No recorded standard reaches orthography. Abstain.
- Whether further reserved words would be *desirable* for delight's sake. "Rigorous Delight" is recorded as the project's standard, but no recorded ruling yet makes delight a ground for expanding surface syntax; the only recorded route to a second word is the falsifier in W7. Abstain.

This seat is a labelled simulation applying Will Bickford's recorded standards; it binds no one, and Will himself is the living acceptor of this design.

---

## Synthesis

### Where the panel converges

1. **The placement of the single invention is right.** Every seat,
   arriving from a different body of work, found the same shape:
   one new word, placed at exactly the act the new substrate adds
   and the inherited engine cannot perform, and no novelty spent
   where TeX already suffices. Knuth: it does new work, not a
   synonym. Lovelace: to rename familiar operations would suggest
   a new power where none exists. Booch: disciplined minimalism,
   not feature accretion. The Bickford seat: the one divergence is
   a difference-with-a-reason, and the word itself is *found* — the
   terminal unit of the address — not assigned.
2. **The word is reserved, not yet defined.** Five seats (K1, V2,
   B1, T1, L1) make pinning `\scroll`'s mood — cite, include, or
   jump — a condition of their verdict standing; the sixth (W3)
   makes it a condition of any load-bearing use. On the necessity
   of the pin there is no dissent anywhere on the panel. (On its
   timing there is — see Disagreement 1.)
3. **A reservation stated is not a reservation kept.** Knuth's
   namespace home (K5), von Neumann's closed lexicon, admission
   valve, and structural enforcement (V1, V4, V5), and Booch's
   anti-shadowing checker and manifest-declared extensions (B2,
   B5) converge from three directions on one finding: the ruling
   currently announces an outcome; it does not yet specify the
   mechanism that makes deviation unplayable rather than merely
   regrettable.
4. **The receipt proves a starting point, not an invariant.**
   Booch: a clean starting point, not a durable invariant. The
   Bickford seat (W2): the honest claim is "free in the checked
   corpus," not "free in TeX." Von Neumann, abstaining: whether
   the receipt remains true is a contingent fact about other
   people's future play — which is what the mechanism conditions
   are for.
5. **Failure must be specified, not discovered.** What a dangling
   `\scroll` does is required by K3, L2, W5, T1's table entry, and
   B1 alike. The seats disagree on *what* it should do
   (Disagreement 3), not on whether the design may leave it
   unstated. It may not.

### Disagreements — kept as data, not averaged

1. **When the mood must be pinned.** Turing's headline condition
   and Knuth's K1 make the pin a precondition of the word meaning
   anything checkable at all ("an entry missing from the table is
   not an entry with a flexible meaning; it is no instruction at
   all"). The Bickford seat holds the opposite timing: pinning
   before the execution model settles would violate the recorded
   rule that notation is designed against the settled execution
   model, not ahead of it — pin *before load-bearing use*, and
   mark the word reserved-and-unpinned until then. Both positions
   are stated at full strength in their reviews; the practical
   reconciliation (a pinning schedule with a non-load-bearing
   interval) is itself a decision, and it is Will's (Decision 4).
2. **What goes inside `\scroll` — four positions, no majority.**
   Knuth (K4): the argument is the literal 9-D coordinate, plain
   text in the source, parseable by a human unaided. Von Neumann
   (V3): the argument must be the *logical* L2 identifier (work +
   edition pin + object key); a raw coordinate leaks the machinery
   into the surface and contradicts the policy at its single point
   of novelty. Turing (T4): the author must never type or maintain
   a raw coordinate at all; the author supplies a human
   designation and the tool computes, inserts, and repairs the
   coordinate. Lovelace (L3): ordinary composition need not show
   the address, but the author must be *able* to learn it and
   write it as an operand on demand, in plain text, with no tool
   outside the surface. These are four different designs. The
   panel records them side by side; it does not resolve them
   (Decision 2).
3. **What a dangling target does.** Knuth (K3), on LaTeX's own
   model: a visible placeholder in the output, a specific warning
   in the log, and the question of build success answered
   explicitly — the behaviour LaTeX users already hold in their
   hands. The Bickford seat (W5): an unresolved reference is a
   checker error at open, *not* a rendering — because the recorded
   standard is "references resolve; referees are present at open,"
   and a placeholder passed along is a broken address typeset as
   if true. (Decision 3.)
4. **Which mood is right — argued by one seat, abstained by
   three.** Knuth argues the cite/refer family is the word's
   plausible home, and that a jump-only pinning would remove the
   word from the author lexicon entirely (his verdict would not
   stand). Booch, Lovelace, and the Bickford seat each explicitly
   abstain on the choice itself — their records supply criteria,
   not a selection. Turing and von Neumann are neutral among the
   three and absolute on exactly-one. The panel therefore does
   *not* report a consensus preference for cite; it reports one
   argued preference, three principled abstentions, and two
   neutrality-with-a-hard-constraint. (Decision 1.)
5. **What inheritance imports.** Knuth: a lexicon built by users
   above a small primitive set is the tradition working as
   intended, and inheriting it is the design's strength. The
   Bickford seat: inheritance imports TeX's assigned-not-found
   accidents wholesale and quietly waives "found, not assigned"
   for everything except `\scroll`, unless the inherited surface
   is required to compile down to found addresses underneath
   (W4). Turing sharpens the same seam into a formal defect: as
   worded, "the existing tex lexicon applies" does not say
   whether phextura's reader is a parser over an enumerated
   data-subset or an interpreter executing a macro language — and
   the corpus-is-data property survives only under the first
   reading (T2, T3). (Decision 6.)

### Conditions, sorted

**(a) Clarifications the design record can absorb by editing text**
— no ruling needed; the record as it stands is incomplete or
mis-scoped in these places:

- **State labels (W1).** Every presentation of the ruling carries
  its true state: surface policy, *designed*; L2 addressing and
  standoff, *designed-not-built*; `\scroll` resolution is not
  described as running behaviour anywhere until an implementation
  receipt exists.
- **Scope the receipt (W2, restatement).** The collision claim is
  restated everywhere as scoped to its evidence — no definition
  in the 595 imported class templates, 2 internal `\trace@scroll`
  hits excepted — and the extension of the check to the wider
  package corpus is entered in the record as owed work with its
  result to be published when run.
- **Reconcile the pre-ruling wording (Knuth, critique §4).**
  DESIGN §3's earlier text (authors writing coordinates in text)
  is reconciled with the ruling, and the plain-text-durability
  invariant is stated as an invariant: nothing essential lives
  only in an editor's database or binary metadata.
- **Admissible set, stated (V1, statement half).** The record
  names the set: the TeX lexicon at a named, pinned version, plus
  `\scroll`. (Enforcement machinery is build work under (b)5's
  rule, not a wording question.)
- **Internal namespace, named (K5).** The record states the
  namespace — on the `@` model or a named equivalent — where all
  phextura-internal control sequences live, so the one-word
  reservation is not silently spent on implementation helpers.
- **The no-silent-invention invariant (L4).** Stated in the
  record: `\scroll` remains the sole reserved new word, and a
  document containing no `\scroll` means exactly what the
  inherited TeX lexicon makes it mean.
- **The falsifier watchlist (W7, record half).** The watchlist —
  standoff anchoring, warrant attachment, range location — is
  entered in the record with a verdict slot per item, so a future
  "not expressible" reopens the reservation count openly instead
  of being absorbed silently.
- **The compilation obligation, stated (W4, statement half).**
  The record states that every inherited addressing form
  compiles to an explicit coordinate record in its own phext,
  derived from structure, not carried only as an author-assigned
  string. (Its `\scroll` half waits on Decision 1.)
- **View correspondences, drafted (B4, in part).** Author /
  reader / machine / maintainer correspondences are drafted into
  the record — except the reader-visible failure behaviour, which
  follows Decision 3.

**(b) Design decisions that return to Will** — PROPOSE only;
nothing here has been applied:

1. **Pin `\scroll`'s mood** — cite, include, or jump, or distinct
   forms for distinct acts (K1/K2, V2, B1, T1, L1/L2, W3) —
   including Knuth's rider: a jump-only pinning removes the word
   from the author lexicon to the viewer layer, and the
   reservation is then reconsidered.
2. **Settle `\scroll`'s argument** among the four positions on
   record: literal 9-D coordinate in plain text (K4); logical L2
   identifier only, raw coordinates rejected (V3); human
   designation with tool-computed, tool-repaired coordinates
   (T4); address author-writable on demand, hidden by default
   (L3).
3. **Rule dangling-target behaviour:** visible placeholder +
   warning on LaTeX's model, with build-success stated (K3) — or
   hard checker error at open, never a rendering (W5).
4. **Set the pinning schedule:** the mood is pinned in the
   specification now (the five seats' standing condition), or at
   the Bickford seat's gate — before any load-bearing use, with
   `\scroll` marked reserved-and-unpinned and barred from carrying
   warrants until then (W3). Ratifying the non-load-bearing
   interval is part of this decision either way.
5. **Adopt the admission rule for future primitives:** new words
   only by a new specification version, only on a documented
   showing that the operation cannot be composed from existing
   primitives without changing their semantics, only as machinery
   primitives (V4) — with the manifest-declared, namespaced,
   versioned extension mechanism (B5) and structural collision
   errors (V5, B2) as the pressure valve and the fence.
6. **Fix the meaning of "the existing tex lexicon applies":** an
   enumerated, finite data-subset whose forms have stated
   data-effects, with definition/expansion constructs quarantined
   as verbatim (T2/T3) — or execution accepted and the
   corpus-is-data and traversal-bound claims withdrawn. The panel
   notes only the first reading keeps the design's stated
   properties; the choice of reading is the designer's.
7. **If the mood is pinned to include, rule its machinery:**
   well-foundedness (self- and cyclic inclusion rejected, or an
   explicit fixed-point/depth-bound semantics) (V6, K6, L2, T6),
   and binding context plus the edition-pin requirement for
   located content entering a constructed artefact (V7).
8. **Rule whether location must reach a part, not only a whole**
   (L5): `\scroll`'s operand distinguishes a whole scroll from a
   determinate block or span within one — which also fixes how
   the surface meets the standoff span selectors.

### The sharpest single objection

Turing's, against the word itself as it currently stands:

> "`\scroll` until its mood is fixed is, formally, a word without
> a table entry. … An entry missing from the table is not an
> entry with a flexible meaning. It is no instruction at all."

Its companion, from Knuth, prices the same defect in the
tradition's own currency: a word with no pinned effect "has not
yet earned its keep. It has earned a *reservation*."

### Closing note

The panel was seated per the designer's own rule for phextura
papers — *the paper chooses the panel*. This design's paper chose
six, including one seat (Booch) from outside the seed roster of
nine, seated under the claim path recorded in the preamble. The
roll-up is advice to the living acceptor, with its conditions
sorted into what the record can absorb and what only he can
decide. — *Panel record closed, 2026-11-11.*
