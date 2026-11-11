# phextura Layer 2 — global research addresses on phext

**Status: DRAFT specification, v0.1 — 2026-11-11.** Task coordinate
44.9.10/11.8.15/8.9.10. This document specifies the Layer 2 address
scheme named by Will Bickford as phextura's key innovation: *"the
first global address scheme for phext — addresses that are durable
between phextura files."* It is implemented entirely on top of phext:
no libphext changes, no new storage format. Where this draft touches
DESIGN.md it does so as proposals (§9); DESIGN.md is not amended by
this document.

Terms: **L1** is phext's native coordinate scheme. **L2** is the
scheme specified here. A **phextura file** is one phext corpus
authored under DESIGN.md (a paper, a proceedings volume, a tracker
corpus, a catalog). An **edition** is one immutable state of a file,
identified per PRINCIPLES.md P1.

## 1. The model

### 1.1 L1 and L2, one sentence each

- **L1:** a phext coordinate names a scroll's position inside one
  file, and means nothing outside that file.
- **L2:** a Layer 2 address names a research object in a named
  phextura file, and resolves in any verified copy of that file,
  wherever the file lives.

The two schemes are different in kind, not in degree. An L1 coordinate
is a *position*; it is internal to its file by construction — every
phextura file roots its body at 1.1.1, so the string `1.1.1/1.1.4/1.2.5`
names a different scroll in every file that contains it. An L2 address
is a *denotation*; it is global by construction — it names the file
first and the object second, and neither part is a phext coordinate.
L2 addresses must therefore never *look* like phext coordinates: the
grammar in §3 is alphabetic and colon-structured precisely so that no
reader, human or program, can mistake one scheme for the other.

### 1.2 What an L2 address denotes

An L2 address denotes a **research object**: a unit that research
discourse itself treats as a thing — a theorem, a definition, a claim,
a figure, a cited work, a person, a concept. Research objects have
three properties scrolls do not:

1. **Role.** A theorem is cited *as* a theorem. The kind is part of
   the denotation, not metadata about it.
2. **Identity across editions.** "Theorem 4.1" of the Onsager paper is
   the same object before and after a restructure moves its scroll;
   its display number may change (P5: numbers are views), its L1
   coordinate may change (P1: re-basing moves units), and the object
   persists. L2 names the object, not its current position.
3. **Identity across files.** The object lives in one file and is
   cited from others. Its address is therefore complete on its own —
   it carries its home file's identity with it — and a citation never
   smuggles a foreign L1 coordinate that would be meaningless (or
   worse, *differently* meaningful) in the citing file.

An L2 address denotes; it does not certify. Resolution proves that a
held copy is the file the address names and that the object exists in
it. It asserts nothing about the object's truth (P11), its authorship
(P10), or its merit. §9 states the limits this imposes.

### 1.3 File identity: work, edition, copy

A global scheme must name three different things and keep them apart:

- A **work** is the file as a continuing thing — the Onsager paper
  across all its editions. Work identity is what a citation usually
  means.
- An **edition** is one exact state of the work: per P1, the pair
  (corpus root, manifest checksum). Edition identity is what
  verification means. The **edition checksum** is the manifest's
  declared corpus checksum (libphext `checksum`, xxh3-128, computed
  over the normalized corpus with the checksum line itself excluded).
- A **copy** is a byte artifact a reader happens to hold. Copies are
  not named; they are *verified* — a copy is admitted as an edition
  when its computed checksum matches the edition checksum it declares
  (§4.2). This is the self-certifying property: the address carries
  enough information to check the copy against the name, so no
  registry's say-so is ever part of resolution. (The name-collision
  failure this can produce is specified in §4.2, step 2.)

A work is named in an address by a **work reference**, in one of three
forms, in decreasing order of external anchoring:

1. **Typed identifier** — an identifier from a system that already
   exists and already endures: `arxiv:2611.08794`, `doi:10.…`. The
   target file declares the identifier in its own metadata (the demo
   corpus already carries `arxiv: 2611.08794` in its metadata
   scroll). L2 borrows these systems' durability; it does not
   recreate it.
2. **Declared name** — a name the file asserts for itself in its
   manifest, under the scheme token `corpus:` (e.g.
   `corpus:research`). Self-asserted, and labelled as such wherever
   it is displayed (P10): two files may declare the same name, and
   §4 makes that collision a loud failure, never a silent choice.
3. **Checksum form** — `sum:xxh3-128:<hex>`, the edition checksum
   itself as the work reference. Name-free and registry-free: the
   address is exactly as durable as the content it names. Its costs
   are priced in §4.5: a new address per edition, no human
   readability, and no discoverability — a holder must already possess
   the file or a catalog entry that maps the checksum to a location.

The recommended citation form pairs a work reference with an edition
pin (§3): the name routes discovery and human reading; the pin makes
the citation exact. Neither half alone is the design's recommendation
— name-only citations float across editions (§5 states when that is
legitimate), pin-only citations are unfindable.

### 1.4 Object identity inside a file: keys

Within a file, an object is named by a **key**: a pair (kind, slug),
e.g. (`theorem`, `uniform-consistency`). Keys obey four rules:

- **Minted once, recorded in the file.** The key is written as a
  `key:` line on the object's own scroll (structure is data, P8) and
  indexed in the file's object table (§4.1). A key that exists only in
  a citing file's prose is not a key; it is a wish.
- **Never reused, never silently renamed.** A deleted object's key is
  tombstoned in the object table's ledger (§4.1), with an optional
  successor pointer. A misspelled key is corrected by minting the
  correct key and recording an **alias** in the ledger; the old key
  resolves through the alias forever. Durability is bought exactly
  here: the file remembers every name it has ever answered to.
- **Minted by tooling, confirmed by the author.** At L2-enablement,
  tooling mints keys in canonical traversal order (P7) from each
  object's declared name — the demo corpus's `kind: theorem 4.1
  (Uniform consistency)` line yields slug `uniform-consistency`. The
  author may choose a different slug *before the edition is
  published*; after publication the permanence rules above apply.
  Minting is append-only thereafter: a new object takes a fresh key;
  existing keys never shift. Insertion in the middle of a paper
  renumbers nothing — the property P5 demands of display numbers,
  applied one layer up to identity itself.
- **Minted on demand, not exhaustively.** A file of 1,154 scrolls
  does not need 1,154 keys. Keys are minted for (a) the paper's
  citable furniture — theorem-family blocks, definitions,
  assumptions, sections, figures, tables, algorithms, bibliography
  entries; (b) any object actually cited from another file or carrying
  warrant edges (§2); (c) any object the author explicitly keys. The
  demo corpus's 637 equations, for instance, keep their L1
  coordinates for intra-file reference and acquire keys lazily — when
  first cited across files, or when the author keys one. This keeps
  object tables proportional to a file's *cited surface*, not its
  size.

### 1.5 Object kinds in v1

Derived from the demo corpus's actual contents (arXiv:2611.08794,
tracker Paper 010: 37 theorem-family blocks, 29 proofs, 637 display
equations, 39 bibliography entries, 9 figures, 6 tables, 2 algorithms,
16 books), from the amended DESIGN (§2 roles, §3 warrant edges), and
from the cross-file uses in §6:

| Kind | Denotes | Derived from |
|---|---|---|
| `theorem` | a theorem block | demo (5 in Book 4 alone) |
| `lemma` | a lemma block | demo (8) |
| `proposition` | a proposition block | demo (11) |
| `corollary` | a corollary block | demo (4) |
| `definition` | a definition block | demo (5) |
| `assumption` | an assumption block — load-bearing, referenced like a theorem | demo (4; see note) |
| `remark` | a remark block | demo (3) |
| `proof` | a proof block, addressable separately from its statement | demo (29) |
| `equation` | one display equation (or one row of a group) | demo (637) |
| `equationgroup` | a multi-row display referenced as a whole | DESIGN §2 |
| `figure` | a figure (its media header scroll, per one-float-one-address) | demo (9) |
| `table` | a table | demo (6) |
| `algorithm` | an algorithm block | demo (2) |
| `section` | a section: a book, or a headed slot within one | demo (16 books + slots) |
| `bibentry` | one bibliography entry | demo (39) |
| `claim` | any claim-bearing scroll carrying ≥1 warrant edge | DESIGN §3 (P11) |
| `paper` | the work itself as an object (its metadata scroll) | §6 composition |
| `author` | a person as denoted by one file's metadata | §6; see §9.3 on identity |
| `concept` | a concept scroll in a non-paper corpus (tracker, catalog) | research.phext Ch. 1 §2 |

Two notes. First, the demo corpus stores Assumption 2.1 as an
`@definition` scroll with a free-text `kind: assumption 2.1` line —
the pre-amendment degradation friction item F2 recorded. Under L2 the
kind is fixed by the object's key record (`assumption:known-logging`,
§7), not recovered from prose: enabling L2 on a corpus is also the
occasion to repair its role data, in the open, as a new edition.
Second, kinds are an **open vocabulary over this core**, mirroring
P8's role discipline: a file declares extension kinds in its manifest;
a resolver treats the kind as an opaque token matched against the
object table, and the core list is what generic tooling must
understand.

## 2. Claims and warrants under L2

A `claim` object is any scroll carrying warrant edges. L2's
contribution to the warrant graph (DESIGN §3, P11) is range: an edge
whose target lives in another file cannot be an L1 coordinate — in the
citing file that coordinate names whatever happens to sit there, which
is precisely the silent-rot failure definition (b) calls worse than no
reference. One kind rule first: an object has exactly one L2 kind —
its primary role kind. A theorem carrying warrant edges is addressed
as a `theorem`; `claim` is the kind of a claim-bearing scroll that has
no stronger role kind (a prose assertion carrying edges). A refutation
targets the object's own address; the edge kind carries the semantics.
Therefore:

- A warrant edge's target grammar is extended: a target is an L1
  coordinate (intra-file) or an **L2 address** (cross-file). The
  `citation` kind already requires the target edition's checksum to
  be recorded (DESIGN §3); an L2 target with an edition pin carries
  that checksum in the address itself.
- A claim in file B may thus carry `warrant: citation
  px2:arxiv:2611.08794#theorem:uniform-consistency`, and a refutation
  in file C targets B's claim by *its* L2 address. The warrant graph
  becomes a graph over the literature, not over one file — with every
  edge's validation rules unchanged (resolution + checksum agreement;
  checkers certify, addresses only denote).

## 3. The grammar — DRAFT

The grammar is **DRAFT**: the shape (scheme prefix, work reference,
fragment-style object part, edition pin) is the proposal; token details
are Will's to ratify. Design constraints, each forced: plain text,
human-writable, machine-parseable without an expansion engine (P4);
recognition decidable at a glance — the `px2:` prefix means an L2
address can never be confused with a version number, a section
numeral, or an L1 coordinate, which discharges P2's reference-grammar
obligation for cross-file references by construction; and no character
that needs escaping in prose.

```ebnf
address     = scheme work-ref [ pin ] [ "#" object-ref ] ;
scheme      = "px2:" ;
work-ref    = typed-id | declared-name | checksum-form ;
typed-id    = ("arxiv" | "doi") ":" id-text ;        (* id-text runs to pin, "#", or end;
                                                        a DOI's "/" is safe — see parse order *)
declared-name = "corpus" ":" slug ;
checksum-form = "sum" ":" "xxh3-128" ":" 32HEXDIG ;
pin         = "@" "e" ":" 16*HEXDIG ;                (* prefix of the edition checksum *)
object-ref  = kind ":" key ;
kind        = "theorem" | "lemma" | "proposition" | "corollary"
            | "definition" | "assumption" | "remark" | "proof"
            | "equation" | "equationgroup" | "figure" | "table"
            | "algorithm" | "section" | "bibentry" | "claim"
            | "paper" | "author" | "concept" | ext-kind ;
key         = slug ;
slug        = ALPHA *( ALPHA | DIGIT | "-" ) ;       (* lowercase ASCII *)
```

**Parse order** (normative, because a DOI contains `/` and may
contain other punctuation): (1) require and strip the `px2:` prefix;
(2) split at the **first** `#` — the tail, if present, is the
object-ref; (3) in the head, if it begins `sum:` it is a checksum
form and is complete; otherwise split the pin at the **last** `@e:`;
the remainder is the work-ref, matched *literally* against manifest
declarations (§4). An address with no `#` denotes the work itself.

**Forms, worked on the demo corpus** (arXiv:2611.08794; §7 collects
the underlying L1 facts):

- *Work only:* `px2:arxiv:2611.08794`
- *Intra-paper object* (an L2 address used inside its own file —
  legal, and the re-base-proof way to cite one's own furniture):
  `px2:arxiv:2611.08794#figure:risk-estimation-selection`
- *Cross-paper object:* `px2:arxiv:2611.08794#theorem:uniform-consistency`
- *Cross-corpus object* (a paper citing the tracker; §6):
  `px2:corpus:research#concept:onsager-correction`
- *Pinned (exact edition):*
  `px2:arxiv:2611.08794@e:cec31e46e2ff1f1c#theorem:uniform-consistency`
  — the pin value shown is a stand-in (see §7's honesty note); an
  L2-enabled edition's pin is the prefix of its manifest checksum.
- *Self-certifying (no name at all):*
  `px2:sum:xxh3-128:<edition checksum>#theorem:uniform-consistency`

Display is a view (P5 applied to addresses): a renderer shows a
resolved citation as the target's human label — "Theorem 4.1 of
Jain–Braun–Imaizumi" — computed from the target file's metadata and
traversal at render time. The stored text is always the address.

## 4. Resolution

Resolution answers: *given this address and the copies I hold, which
scroll, in which verified edition, does it denote — or exactly how
does it fail?* The data resolution needs is of two kinds, and P3's
spirit — no silent dependence, everything checksummed — assigns each
its home:

- **Intra-file data lives in the file.** The object table (§4.1) is
  scrolls in the file's own Collection 6, declared in its manifest.
  A file is self-describing: any verified copy resolves its own
  addresses with no outside help.
- **Cross-file data is the target file itself.** A citing file does
  not — cannot — contain the cited file's table. What the citing file
  contains is only the address. Resolution happens where the copies
  are: in the reader's (or the composing tool's) library, against
  verified copies. **There is no registry in the resolution path.**
  Registries and catalogs exist (§4.4) but they answer *discovery*
  (where can a copy be fetched?), never *resolution* (what does this
  address denote in the copy I hold?). The moment a registry could
  answer the second question, addresses would denote whatever the
  registry says today — the rot and authority failures §9 excludes.

### 4.1 The object table

Each L2-enabled file carries an **object table** in Collection 6,
declared in the manifest, in two parts:

- **The index** (derived data): one line per keyed object —
  `theorem:uniform-consistency -> 1.1.1/1.1.4/1.2.5` — mapping key to
  the object's L1 coordinate *in this edition*. The index is
  rebuildable from the corpus (keys ride on the object scrolls as
  `key:` lines, §1.4) and is regenerated, never hand-edited, on every
  re-base: when `offset` moves a subspace, the moved objects' `key:`
  lines move with them and the index is recomputed. Sharding: one
  index scroll per kind for large files.
- **The ledger** (authored data, append-only): minting records,
  aliases (`alias: theorem:uniform-consistancy ->
  theorem:uniform-consistency`, from the edition where the spelling
  was fixed), and tombstones (`retired: equation:scratch-bound, in
  edition <checksum>, successor: equation:value-of-policy` or
  `successor: none`). The ledger is the part of the table that is
  *history*, and history is never derived or rewritten (P10).

Size, honestly: Θ(O) lines for O keyed objects, tens of bytes each.
Enabling L2 on the demo corpus keys its furniture — 37 theorem-family
blocks, 29 proofs, 39 bibentries, 17 media objects, 16 books and
their headed sections, a handful of equations — roughly 200 objects,
≈12 KB of index on a 1,421,714-byte file (under 1%). The 637
unequally-cited equations stay unkeyed until cited (§1.4), so the
table tracks the cited surface, not the scroll count.

### 4.2 The algorithm

Given address A and a local library L (the set of phextura files the
resolver holds — a directory, a composed reading set, a venue's
archive):

1. **Parse.** Per §3's parse order. Failure → `MALFORMED`.
2. **Locate candidate copies.** Match A's work-ref against each
   file's manifest declarations (typed identifiers, declared name) —
   or, for the checksum form, against each file's declared edition
   checksum. A **library index** (work-ref → copies, with each copy's
   edition checksum) is built once per library state; building it
   reads manifests only. Zero candidates → `UNRESOLVED-WORK`.
   Candidates declaring the same self-asserted `corpus:` name but
   different works (distinguishable by incompatible metadata and
   disjoint object tables) with no pin and no checksum form to
   separate them → `AMBIGUOUS-NAME`.
3. **Select the edition.** If A carries a pin: keep copies whose
   edition checksum matches the pin prefix. None → `EDITION-MISMATCH`
   (the receipt lists the editions actually held — the failure tells
   the reader what would satisfy it). A prefix matching two held
   editions → `AMBIGUOUS-EDITION`. If A carries no pin: if all
   candidates share one edition checksum, proceed; if several
   editions are held → `AMBIGUOUS-EDITION`, listing them — the
   resolver does not silently pick "latest," because latest is a
   policy, and policies belong to venues and readers (P11's division
   of labor), stated where they are applied.
4. **Verify the copy.** Recompute the located copy's checksum over
   its normalized corpus and compare with the edition checksum its
   manifest declares (and with A's pin / checksum form, where
   present). Mismatch → `INTEGRITY`. This step is the self-certifying
   core: after it, the resolver *knows* the copy is the edition the
   address names, without trusting the source it came from. Paid once
   per file per library state, cached against the copy's bytes.
5. **Resolve the object.** Load the edition's object table; follow
   aliases in the ledger; look up (kind, key). Absent →
   `UNRESOLVED-OBJECT`. Present only as a tombstone →
   `OBJECT-RETIRED` (the receipt carries the tombstone's successor
   pointer or `none`). Present → the L1 coordinate for this edition.
6. **Fetch and check kind.** `fetch` the coordinate; confirm the
   scroll's role matches A's kind (an `assumption` address landing on
   a `@prose` scroll means table or corpus corruption). Mismatch →
   `KIND-MISMATCH`. Return the **resolution receipt**: (work-ref,
   edition checksum in full, L1 coordinate, scroll) — the receipt,
   not the address, is where L1 coordinates legitimately appear
   (§6.3).

### 4.3 Failure modes

Every failure is loud, located at the citing site (P2), and distinct
— a tool must be able to tell an author *which* repair applies:

| Failure | Meaning | Repair it names |
|---|---|---|
| `MALFORMED` | the string is not an L2 address | fix the string |
| `UNRESOLVED-WORK` | no held copy declares the work-ref | fetch a copy (discovery, §4.4) |
| `AMBIGUOUS-NAME` | a self-asserted name is claimed by two works | pin, or use the checksum form |
| `EDITION-MISMATCH` | pin names an edition not held | fetch that edition, or re-pin knowingly |
| `AMBIGUOUS-EDITION` | several editions held, no pin | pin, or apply a stated local policy |
| `INTEGRITY` | a copy's bytes do not match the checksum it declares | discard the copy; it is not that edition |
| `UNRESOLVED-OBJECT` | the key is not in this edition's table | the object was never keyed, or the key is wrong |
| `OBJECT-RETIRED` | the key is tombstoned | follow the successor pointer, or cite knowing it is gone |
| `KIND-MISMATCH` | the table points at a scroll of the wrong role | the target file is corrupt; report upstream |

### 4.4 Discovery is data, not infrastructure

A reader holding an address but no copy needs a *location*, and L2's
answer is deliberately boring: a **catalog** is an ordinary phextura
(or phext) corpus whose scrolls map work references and edition
checksums to fetch locations (URLs, repository paths, DOIs of record).
Catalogs are data — authored, checksummed, disputable, forkable —
never a resolution authority: a catalog can send you to a copy, and
step 4.2.4 decides whether the copy is what the address names. A lying
catalog wastes a fetch; it cannot forge a resolution. arXiv and
Crossref already serve as catalogs for the typed identifiers L2
borrows; nothing in this scheme requires building a new one before
the first citation is written.

### 4.5 Cost (P12 format)

Spec-local variables, extending the P12 palette as P12 permits when
named: B, S as in PRINCIPLES.md (bytes, scrolls of the *target*
file); R = L2 references resolved; D = distinct (file, object)
targets; W = files in the library; O = keyed objects in the target
file; E = editions traversed in a mapping chain (§5).

- **Address parse:** Θ(|A|) time, Θ(|A|) space; |A| ≈ 45–60
  characters in the named forms.
- **Library index build:** Θ(M) time and space, M = total manifest
  bytes across W files (manifests only, not corpora). Rebuilt on
  library change; incremental maintenance is an implementation
  detail, not claimed here.
- **Copy verification:** Θ(B) time, Θ(B) space (current tooling loads
  whole files), once per file per library state — the checksum pass.
  This is the honest price of self-certification: resolution is cheap
  per address because verification is paid per file.
- **Single resolution, warm library** (index built, copy verified,
  table loaded): Θ(1) expected time (hash lookups assumed), Θ(1)
  space beyond the returned scroll.
- **Single resolution, cold:** Θ(B + O) time — verification plus
  object-table load (the table is part of the file, Θ(O) lines) —
  then as warm.
- **Batch:** resolving R references into D distinct files costs
  Θ(Σ B over the D files + Σ O over the D files + R) expected time:
  each touched file is verified and tabled once, each reference is a
  lookup. Compare L1 validation (Θ(B + S + D) expected, P2): composing
  files multiplies the *verification* term by the number of files
  touched, and nothing else grows. Composition is linear in what it
  touches.
- **Checksum-form addresses** skip step 2's matching subtlety (the
  checksum *is* the match) but pay §1.3's price: a fresh address per
  edition, and discovery impossible from the address alone.

## 5. Edition drift: the P1 promise, kept

P1 as amended promises: any operation that re-bases addresses creates
a new edition and emits a **mapping scroll** (old coordinate → new
coordinate for every moved unit), so a citation to edition *k*
resolves mechanically into edition *k+1*. L1 citations depend on that
promise entirely. L2 citations mostly do not — and the difference is
the layer's reason to exist:

- **An unpinned L2 citation does not notice a re-base at all.** The
  work-ref and the key are unchanged; the new edition's object table
  maps the key to the object's new coordinate. The address string a
  citer wrote is byte-identical across the restructure.
- **A pinned L2 citation resolves forward through the chain.** Each
  edition's manifest lists its predecessor's edition checksum and the
  coordinate of the mapping scroll the re-base emitted. A resolver
  holding edition *k+n* against a pin for *k*: read the object key's
  coordinate in edition *k*'s table (if that edition is held) or —
  the normal case — start from the key in the held edition, since the
  key, unlike the coordinate, is edition-invariant. The mapping chain
  is needed only to transport *L1-pinned* receipts and to audit that
  the object at the key in edition *k+n* is the object that held the
  key in edition *k* — an audit the ledger (§4.1) makes mechanical:
  keys are minted once and never reassigned, so sameness of key
  across editions *is* sameness of object, by the minting rules.
  Chain traversal cost, when an audit is demanded: Θ(E) table reads,
  E = editions in the chain.
- **What a re-base must emit, normatively:** the new edition's
  regenerated index (§4.1), ledger entries for any keys minted or
  retired in the restructure (retirements only — a pure re-base
  retires nothing), the P1 mapping scroll for its L1 citations, and a
  manifest predecessor line. A re-base that moves an object without
  regenerating the index fails step 4.2.6's kind check or 4.2.5's
  lookup, loudly — the failure modes are the enforcement.

**Worked example.** The demo corpus's main theorem — Theorem 4.1,
*Uniform consistency* — sits at L1 `1.1.1/1.1.4/1.2.5` in its first
edition, keyed `theorem:uniform-consistency`. Its L2 address is
`px2:arxiv:2611.08794#theorem:uniform-consistency`. Now the
hypothetical re-base: a restructure inserts a new book before Book 4
and `offset` moves Book 4 (*Theoretical Guarantees*) to Book 5; the
theorem's scroll lands at, say, `1.1.1/1.1.5/1.3.5`. Every L1 citation
to `1.1.1/1.1.4/1.2.5` anywhere in the wild now depends on the P1
mapping scroll. The L2 address **does not change** — not one
character. A citing file holding the new edition resolves it through
the new object table to `1.1.1/1.1.5/1.3.5`; a citing file holding
only the old edition resolves it to `1.1.1/1.1.4/1.2.5`; both
resolutions are correct, each receipt names its edition, and no citer
did anything. Theorem 4.2 (*oracle inequality*,
`px2:arxiv:2611.08794#theorem:oracle-inequality`) cites Theorem 4.1 in
its statement ("Under the assumptions … of Theorem ⟨address⟩") — if
that citation is written in L2, the paper's most load-bearing internal
reference is exactly as durable as its external ones.

## 6. Composition: the point of the layer

Composition is what L2 is *for*. The failure it removes is concrete:
today, the only way for file B to cite a theorem in file A is to write
A's L1 coordinate into B's text — a string that, read inside B,
denotes one of B's own scrolls or nothing. The citation is meaningless
in the file that carries it and correct only by a convention no tool
can check. L2 makes the citation carry its home with it.

**The worked example — three files.**

- **A** = the demo corpus: Jain, Braun & Imaizumi, arXiv:2611.08794
  (real; §7 gives its facts).
- **B** = a follow-up paper, *Onsager corrections for fitted-value
  comparisons* — **hypothetical**, constructed for this example; it
  declares `corpus:onsager-fitted-values`. Its main theorem's scroll
  reads, in part:

  ```
  @theorem
  key: fitted-value-transfer
  warrant: citation px2:arxiv:2611.08794#theorem:uniform-consistency
  The uniform consistency of
  px2:arxiv:2611.08794#theorem:uniform-consistency extends to
  fitted-value comparisons: …
  ```

  B also reuses A's experiment figure in its discussion, citing
  `px2:arxiv:2611.08794#figure:risk-estimation-selection` — the
  figure itself is not copied into B; the citation resolves into A
  at reading time, or fails loudly if the reader holds no copy of A.
- **C** = research.phext, the tracker corpus, declaring
  `corpus:research`. Its concept scroll *The Onsager correction*
  (L1 `1.1.1/1.1.1/1.2.113` in C) gains the line
  `source-theorem: px2:arxiv:2611.08794#theorem:uniform-consistency`
  — the reverse edge, recording which object in A the concept was
  distilled from. Symmetrically, A cites the concept where its
  criterion is introduced: `px2:corpus:research#concept:onsager-correction`.
  (A's metadata scroll already records the relationship in prose —
  "tracker concepts 1.2.110-1.2.116" — an L1-shaped note that means
  nothing outside C. The L2 line is the same fact as data.)

**A composed reading/validation pass** over the library {A, B, C}
does, in order: (1) build the library index from the three manifests
(§4.5, Θ(M)); (2) verify each file once (Θ(B) per file — the only
step whose cost scales with corpus size); (3) enumerate every L2
reference in every file — decidable by the `px2:` prefix — and
resolve each per §4.2, reporting failures at the citing scroll with
their failure class; (4) validate each file's L1 references as today
(DESIGN §7). The composed reference graph is the union of the three
`navmap`s and the cross-file edges; in this example the cross-file
edges are B→A (theorem citation, warrant citation, figure citation),
A→C (concept citation), C→A (source-theorem). (5) Reading: each file
traverses by its own P7 order; a cross-file reference renders as a
link whose label — "Theorem 4.1 (Jain–Braun–Imaizumi 2026)" — is
computed from the *target* file's traversal and metadata at render
time and stored nowhere. At no point is anything merged, offset, or
re-based: A, B, and C each keep their own root at 1.1.1, their own
internal coordinates, and their own editions. **Composition is a
relation among files, not a fusion of files.**

**P13, revisited.** The design loop amended P13 (corpora compose)
with an intra-file answer to the collision attractor at Z = 1.1.1:
roots are allocated distinctly, or the smaller corpus is re-based on
merge. That discipline answers the question "how do two corpora
become *one* corpus?" — and it is the right answer *for that
question*. L2 reveals that composition was two operations wearing one
name:

- **Binding** — making one artifact out of several (a proceedings
  volume, an anthology file). `merge`/`offset`, root-allocation
  discipline, mapping scrolls: P13 as amended, unchanged.
- **Citing** — files referring to each other's objects while
  remaining separate files. L2: no root discipline, no re-basing, no
  merge, because global identity was never the coordinate.

Before L2, citing was only possible by binding (merge first, then use
coordinates) — which is why composition looked like a storage
operation. Under L2 the default composition is citing, and binding is
the special case a venue performs when it must ship one file. §10
proposes the P13 amendment in these terms.

## 7. Worked-example fact base (the demo corpus, checked)

Every L2 example in this document resolves against these facts,
extracted from `demo/2611.08794.phext` (1,421,714 bytes, 1,154
scrolls) and `research.phext`. The `key:` lines and object table do
not yet exist in the corpus — it predates this spec — so the keys
below are the ones §1.4's tooling would mint at L2-enablement, shown
so every example is checkable against the real file.

| L2 address (object part) | L1 coordinate, edition 1 | Corpus evidence |
|---|---|---|
| `#theorem:uniform-consistency` | `1.1.1/1.1.4/1.2.5` | `@theorem`, "kind: theorem 4.1 (Uniform consistency)" |
| `#theorem:oracle-inequality` | `1.1.1/1.1.4/1.2.8` | `@theorem`, "kind: theorem 4.2 (Arm-wise iterate selection and oracle inequality)" |
| `#assumption:known-logging` | `1.1.1/1.1.2/1.4.3` | stored as `@definition`, "kind: assumption 2.1 (Known context-independent logging…)" — the F2 degradation; the key record fixes the kind |
| `#equation:value-of-policy` | `1.1.1/1.1.2/1.2.4` | `@equation`, the 𝒱(ν) ≔ E[…] definition — keyed on demand (§1.4) |
| `#figure:risk-estimation-selection` | header `1.1.1/3.1.1/1.1.2` | `@figure` header naming payload `1.1.1/3.1.1/1.1.3`, checksum xxh3-128:aacd1b97…; one float, one address — the key resolves to the header |
| `#bibentry:arlot-celisse-2010` | `1.1.1/2.1.1/1.1.1` | entry carries DOI 10.1214/09-SS054; as a phextura file that work's own address would be `px2:doi:10.1214/09-SS054`, recorded on the entry in a `work:` line (§10) |
| `#section:theoretical-guarantees` | `1.1.1/1.1.4/1.1.1` | Book 4's heading scroll |
| `#paper:onsager-ope` | `1.1.1/6.1.1/1.1.1` | the metadata scroll (title, authors, `arxiv: 2611.08794`) |
| `#author:jain-s` (also `braun-g`, `imaizumi-m`) | `1.1.1/6.1.1/1.1.1` | denotations minted from the metadata `authors:` line; see §9.3 |
| `px2:corpus:research#concept:onsager-correction` | research.phext `1.1.1/1.1.1/1.2.113` | tracker concept "The Onsager correction — subtracting a model's memory of its own data" |

**Honesty notes.** (1) The demo manifest is prose: it carries no
computed checksum line, no `key:` lines, no object table. The pin
shown in §3, `cec31e46e2ff1f1c`, is the first 16 hexadecimal
characters of the demo *file's* SHA-256
(`cec31e46e2ff1f1c5f3a…`), used as a stand-in so the example is
concrete and checkable; in an L2-enabled edition the pin is the
prefix of the manifest's xxh3-128 corpus checksum (§1.3), and the
stand-in must not be mistaken for one. (2) File B in §6 is
hypothetical and labelled so wherever it appears. (3) The demo corpus
contains no warrant edges (it predates the amended DESIGN §3); §2's
cross-file warrant is shown on B, where it belongs.

## 8. The verdict on PROPOSE (2)

The design loop held, unapplied, **PROPOSE (2)**: *external citations
name (edition checksum, coordinate)* — exactness achieved by pinning
every outside citation to an edition and a position inside it.

**Verdict: L2 subsumes PROPOSE (2), and supersedes it as the
external citation standard.** The proposal's instinct — an external
citation must be exact and checkable — is preserved entire, in two
places: the **pin** carries the edition half (`@e:<checksum
prefix>`), and the **resolution receipt** (§4.2.6) reports the
(edition checksum, L1 coordinate) pair after resolution, which is
where the pair legitimately lives: as a *report about* a resolution,
not as the address itself. As an address the pair fails Will's own
correction — a coordinate is internal to its file, meaningless in the
citing file's text — and it names a position where the citer means an
object (§5 shows the two diverging at the first re-base).

An external citation string under this scheme is one of:

- floating: `px2:arxiv:2611.08794#theorem:uniform-consistency`
- pinned: `px2:arxiv:2611.08794@e:cec31e46e2ff1f1c#theorem:uniform-consistency`
  (pin value a stand-in, §7)
- name-free: `px2:sum:xxh3-128:<edition checksum>#theorem:uniform-consistency`

Recommendation to Will: record PROPOSE (2) as *superseded by Layer
2*, not adopted and not rejected — its requirement (exactness) is met
by the pin and the receipt, its mechanism (coordinates abroad) is
retired.

## 9. Honest limits

**9.1 What L2 must not become: a name system that rots.** L2 has
exactly two rot surfaces, and the design treats them differently
because they are different. *Object keys cannot rot within a
lineage*: minted once, never reused, tombstoned rather than deleted,
aliased rather than renamed, all recorded in the file's own ledger —
the anti-rot mechanism is that the file remembers every name it has
ever answered to, and carries the memory inside itself (§4.1). *Work
names can rot*: typed identifiers inherit their systems' maintenance
(arXiv's and Crossref's durability is theirs, borrowed honestly);
declared `corpus:` names can be abandoned or collided — mitigated by
pins and the checksum form, and collisions fail loudly
(`AMBIGUOUS-NAME`) instead of resolving by someone's say-so. What no
address scheme can promise is that a *copy survives*: if no verified
copy of a file exists anywhere, its addresses denote objects no one
can fetch, and `UNRESOLVED-WORK` is the honest report. Durability of
denotation is not durability of archives, and L2 does not claim the
second while delivering the first.

**9.2 Not a second global namespace.** Section 6 of the warts
analysis names the disease: one global namespace, collisions managed
socially. L2's only global artifacts are this grammar and the kind
core. There is no registry in the resolution path (§4); work names
are per-file declarations, not allocations; catalogs are data. The
scheme recreates the wart only if some name authority becomes
load-bearing for resolution — the structural defence is that
resolution never consults one: names route discovery, checksums
decide identity.

**9.3 Not an authority layer.** An address denotes; resolution
verifies existence and integrity of a copy. Nothing in a successful
resolution certifies truth (P11's warrant validation is a separate,
stronger act), authorship (a `corpus:` name and an `author:` object
are self-assertions until anchored outside the file, P10), or
identity of persons: `author:jain-s` in file A and `author:jain-s` in
file B are two denotations that *may* be asserted the same person —
by a registry corpus carrying that claim as data with its own
provenance. That is the design loop's PROPOSE (3); **L2 does not
decide it.** L2 gives the question addresses to be asked in, and
refuses to let sharing a slug silently answer it.

**9.4 The authoring cost, priced as honestly as P14 allows.** The
author pays: (a) *slug confirmation at enablement* — tooling proposes
every key; the author confirms or corrects. On the demo corpus that
is ~200 decisions, once per file, most of them one glance each —
stated as a count, not a time: the time is **UNPRICED** until the
P14 benchmark suite exists. (b) *Citation length* — an L2 address
runs ≈45–60 characters against ≈19 for an L1 coordinate. The
mitigation is a tooling obligation, stated as a requirement: an
editor claiming L2 support must offer completion from the target
file's object table (the author selects an object; the editor writes
the address) and must display the resolved label inline, or the
author pays the layer's full cost in keystrokes and the layer fails
P14 in practice. (c) *Holding the target* — citing across files
requires a local copy of the target; that is an off-screen cost of
exactly the kind P14 counts. The benchmark suite gains a task —
"cite a theorem in another file," target set when the suite is
defined — and until it is run, every delight claim for L2, including
any in this document's §6, is provisional.

**9.5 Also not claimed.** No discovery guarantee (§4.4 is a
mechanism, not a promise). No story for two *drafts* citing each
other before either is published — addresses to unpublished works
resolve only where copies are shared, which is stated here rather
than solved. No concurrency semantics (DESIGN §10's exclusion
stands). And no claim that the tracker, or any existing corpus, is
L2-enabled today: §7's keys are what enablement would mint, and
enablement itself is an edition-producing act each corpus's keeper
performs deliberately.

## 10. Integration — proposed amendments to DESIGN.md (for Will)

Nothing below is applied. Each item names the DESIGN section, the
amendment it would need, and its size.

1. **§2 (corpus layout).** Add the `key:` line to the block structure
   (after the role line, beside the warrant lines), and name the
   object table — index scrolls and the append-only ledger — among
   Collection 6's contents, declared in the manifest. *Two paragraphs
   plus the manifest contents list.*
2. **§3 (references).** Add a subsection, "Layer 2 addresses":
   the grammar by reference to this document; the decidability note
   (the `px2:` prefix); warrant-edge targets extended to admit L2
   addresses; the external-citation standard recorded as L2, with
   PROPOSE (2) marked superseded (§8). *One subsection.*
3. **§4 (bibliography).** A bibentry scroll may carry a `work:` line
   giving the cited work's L2 address when that work exists as a
   phextura file — the bibliography becomes a graph of works, not of
   strings. *One paragraph.*
4. **§5.2 (marginalia).** Annotation corpora currently point at
   (edition checksum, coordinate); under L2 they point at L2
   addresses — object-stable across the reviewed paper's re-bases —
   pinned when the annotation is about an exact edition. *Replace
   the pointing rule (one paragraph).*
5. **§7 (rendering).** The validation pass gains the cross-file step
   (§4.2) whenever a library is present, and resolved cross-file
   citations display target-computed labels as views. *One validation
   bullet, one display note.*
6. **§9 (contrast table).** The references row gains the L2 line:
   cross-file citation by global address, versus LaTeX's
   bibliography-string or nothing. *Editorial, one row.*
7. **§10 (open questions).** Add: slug governance at the
   publication boundary (the exact act that freezes a key); the
   catalog format as a specification of its own; the P14 benchmark
   task for cross-file citation (§9.4). *Three bullets.*
8. **PRINCIPLES.md P13 (noted, not a DESIGN section).** Amend
   "corpora compose" to name the two operations — binding (merge;
   root discipline; P13 as amended stands) and citing (L2; no root
   discipline) — and record that P1's mapping-scroll promise remains
   normative for L1 citations and receipts, while L2 citations are
   carried by keys and tables instead. *One paragraph in P13.*

— End of draft. —
