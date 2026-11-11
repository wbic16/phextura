# phextura

**Phextura: Rigorous Delight.**

A modern replacement for LaTeX, designed to utilize phext from the ground up.

## The name

The old name was an equation: **La**mport + **TeX** = **LaTeX** — Leslie
Lamport's document system on top of Donald Knuth's TeX. *TeX* itself comes
from the Greek τέχνη (*techne*): art, craft, skill.

The new name is **phextura**: *phext* + *textura*. Textura — also called
textualis — was the formal book hand of the medieval scriptorium: the dense,
regular, woven letterforms a scribe used when a text had to last. Both words
reach back to Latin *texere*, "to weave" — the same root that gives us
*text*. A phextura paper is a woven thing: many scrolls, one corpus, every
part addressed.

## What it is

A LaTeX paper is a one-dimensional character stream executed by a macro
expander, assembled at build time from a directory of files — chapters by
path, figures by path, bibliography by an external program, references by
strings that a second pass hopes to resolve.

A phextura paper is a **phext corpus**: plain-text scrolls at deterministic
coordinates in a nine-dimensional lattice. Sections, theorems, equations,
bibliography entries, figures, media, datasets, code, and whole embedded
documents are scrolls inside the one artifact. A cross-reference is a
coordinate — the same object as its hyperlink. Styles are addressed scrolls,
not a global macro namespace. Rendering is a single traversal; nothing is
rerun until it settles, because references resolve by construction.

Two features are the point:

- **Extremely fast to write papers in.** No preamble, no escaping regime, no
  label-and-rerun ritual. Write prose, address the parts, cite by address.
- **Extremely readable for humans and machines.** Every scroll is plain text
  a person can read raw; its identity and role are its coordinate and a
  one-word tag, so a program reads the same paper the human does — without
  executing it.

Phextura is built on [libphext](https://crates.io/crates/libphext) (the
`libphext` Rust crate, v0.5.0) and uses its full public API — addressing,
subspace operations, corpus algebra (`merge`/`subtract`/`offset`), maps,
manifest, and checksums — each with a named role in the design.

## Read more

- [DESIGN.md](DESIGN.md) — the document model, the corpus layout, how
  references, bibliography, media embedding, and styles work, and the
  explicit contrast with expl3/l3kernel.
- [PRINCIPLES.md](PRINCIPLES.md) — what phextura should be, derived from
  first principles (what a paper, a reference, typesetting, reading, and
  authorship *are*), stress-tested by a Knuth-register critic, and used
  to judge the design. The motto above is its grading standard: every
  principle and every design element must be both rigorous (derivable,
  checkable, costed) and delightful — rigor alone is homework, delight
  alone is a toy.

## Status

Design + compiling skeleton. The crate's modules are honest stubs: types and
doc comments state each module's intended role; unimplemented behavior is
`todo!()`, never faked.
