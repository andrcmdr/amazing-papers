# Memory management principles in Mojo (and nearby languages)

This note maps **Mojo’s** memory management model to other programming languages that are “close” in *principles*.
“Close” here means *how the language intends you to reason about lifetimes, aliasing, and cleanup* — not that the runtime/implementation is identical.

Two orthogonal axes show up repeatedly:

- **Axis A — Compile-time aliasing control:** ownership and borrowing, linear/uniqueness types, permissions, or regions (all reduce unsafe aliasing and “use-after-free” classes of bugs).
- **Axis B — Deterministic cleanup:** resources are released at a predictable point (usually scope / lifetime end), rather than at an arbitrary later time via tracing GC.

Rust and Mojo sit near the **intersection**: *strong aliasing control + deterministic destruction*.

---

## 1) Mojo’s model at runtime

### Ownership, but no GC/RC
Mojo’s documentation describes an **ownership** model: at any moment there is a single “owner” of a value, and values can be passed around by moving ownership. The rules ensure one owner at a time, and the compiler can use this to provide memory safety without forcing you to manually call `free`.
Mojo also explicitly states it has **no reference counter** and **no garbage collector**.
Sources: Modular docs on value lifecycle and ownership:
- https://docs.modular.com/mojo/manual/lifecycle/
- https://docs.modular.com/mojo/manual/values/

### Deterministic destruction (destructors on lifetime end)
When a value’s lifetime ends, Mojo calls the value’s destructor (`__del__()`), and that destructor is where heap memory or other resources should be released for custom types. The docs describe this as happening “typically” at the point of last use.
Sources:
- https://docs.modular.com/mojo/manual/lifecycle/death/
- https://docs.modular.com/mojo/manual/values/

**What this implies:** Mojo aims to feel “automatic” to the user (no explicit GC tuning), while still offering **predictable teardown** and systems-level control via explicit destructors and (when needed) lower-level memory primitives.

---

## 2) Two principle variants

### (a) Rust-style: ownership + borrowing checked by the compiler (“borrow checker family”)
Key idea: the type system tracks *who owns data* and enforces rules for references so they **cannot outlive** the owned value and so **mutable aliasing** is controlled.

Canonical statement (Rust):
- Ownership makes memory safety guarantees **without a garbage collector**.
- Borrowing uses references so you can access data without taking ownership, and the compiler checks reference validity.
Sources:
- Rust book — Ownership: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
- Rust book — References and borrowing: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
- Rust std — `Drop`: https://doc.rust-lang.org/std/ops/trait.Drop.html

**Languages that fit (a) most directly:**
- **Rust** (the archetype).
- **Mojo** (ownership model; explicitly no GC/RC; destructor on lifetime end).
  Sources: https://docs.modular.com/mojo/manual/lifecycle/ , https://docs.modular.com/mojo/manual/values/ , https://docs.modular.com/mojo/manual/lifecycle/death/
- **Move** (has a borrow checker; formalized in the “Move Borrow Checker” paper).
  Sources:
  - arXiv paper: https://arxiv.org/pdf/2205.05181
  - (Also see ecosystem docs describing borrow-checker rules, e.g. “no dangling references; at most one mutable or many immutable refs”.)

You can think of (a) as “*compile-time lifetime and aliasing discipline* as the primary safety tool.”

### (b) C++-style: deterministic destruction / RAII (“scope-based resource management family”)
Key idea: tie a resource’s lifetime to an object, and rely on destructors (or scope guards) to release it deterministically.

Canonical statement (C++ RAII):
- RAII binds a resource’s lifecycle to object lifetime; resources are acquired during initialization and released in destructors.
Sources:
- https://en.cppreference.com/w/cpp/language/raii.html
- Microsoft docs: https://learn.microsoft.com/en-us/cpp/cpp/object-lifetime-and-resource-management-modern-cpp?view=msvc-170

**Languages that fit (b) strongly:**
- **C++** (origin of RAII).
- **Ada** (controlled types provide user-defined finalization hooks).
  Sources:
  - https://www.adaic.org/resources/add_content/docs/craft/html/ch16.htm
  - https://learn.adacore.com/courses/advanced-ada/parts/resource_management/controlled_types.html
- **D** (RAII exists for structs and scoped class variables; scope guards exist).
  Sources:
  - D scope guards: https://tour.dlang.org/tour/en/gems/scope-guards
  - D spec (struct destructors): https://dlang.org/spec/struct.html
  - D glossary RAII entry: https://dlang.org/spec/glossary.html

A key nuance: (b) is about **deterministic cleanup**. It does *not* necessarily enforce “no aliasing” the way Rust/Mojo do.

---

## 3) Broader lists: languages closer to Rust/Mojo vs closer to C++

This section expands the earlier lists with older and research languages.
Again: “closer” means *conceptual similarity* in how lifetimes and memory are reasoned about.

---

## 3A) Languages closer to Rust and Mojo (aliasing/lifetime control as a first-class idea)

### A1) Borrow-checker ownership (direct cousins)
These languages put ownership/borrowing rules at the center.

- **Rust** — ownership + borrowing; deterministic `Drop`.
  Sources: Rust book and `Drop` docs (linked above).
- **Mojo** — ownership model; explicitly no GC/RC; destructor on lifetime end.
  Sources: Modular docs (linked above).
- **Move** — borrow checker for references; formalized.
  Source: https://arxiv.org/pdf/2205.05181

**Rust-adjacent but different enforcement:**
- **Blech** (research/industrial language work discussing borrowing/overlap checking; borrow-checker-like direction).
  Source: https://www.blech-lang.org/docs/language-evolution/borrowing/20_borrowing/

### A2) Single ownership without a classic borrow checker
- **Vale** — advertises “memory-safe single ownership without garbage collection or a borrow checker.”
  Source: https://vale.dev/

(“Without a borrow checker” doesn’t mean “without safety”; it usually means *a different* static and/or runtime mechanism.)

### A3) Region-based memory management (explicit lifetime regions)
Region systems carve the heap into regions and reclaim whole regions at once (or with region-local strategies).

- **Cyclone** — classic work on region-based memory management for a safe C-like language, with a static typing discipline; the design also integrates with stack allocation and a garbage collector.
  Source (paper): https://www.cs.umd.edu/projects/cyclone/papers/cyclone-regions.pdf
- **Tofte–Talpin regions / ML tradition** — region inference for functional programs; classic “Region-Based Memory Management” work.
  Source: https://www.sciencedirect.com/science/article/pii/S0890540196926139
- **MLKit with regions** — a production-grade system with region inference tooling.
  Source (manual): https://elsman.com/pdf/mlkit-4.6.0.pdf
- **Project Verona** — organizes objects into a forest of isolated regions; memory is managed locally per region, and regions can use different memory-management strategies.
  Sources:
  - Microsoft Research: https://www.microsoft.com/en-us/research/publication/reference-capabilities-for-flexible-memory-management/
  - Paper (PDF): https://arxiv.org/pdf/2309.02983

### A4) Linear / uniqueness / permission typing (type-theoretic cousins)
These languages constrain aliasing so the compiler can safely insert deallocation, allow in-place update, or enforce resource protocols.
Some are GC-backed in practice, but the *principle* is similar: **make “who can use this value” explicit in types**.

- **Clean** — highlights uniqueness typing as a major language feature; includes a reference manual chapter on uniqueness typing.
  Sources:
  - https://clean.cs.ru.nl/
  - https://clean.cs.ru.nl/download/html_report/CleanRep.2.2_11.htm
- **Mercury** — “unique modes” (similar to linear types) and the manual explicitly notes the compiler can do “compile-time garbage collection” (automatic deallocation insertion) when it knows there will be no more references.
  Source: https://mercurylang.org/information/doc-release/mercury_ref/Unique-modes.html
- **ATS** — emphasizes using linear types to reduce memory footprint and improve safety.
  Source: https://www.cs.bu.edu/~hwxi/atslangweb/
- **Idris 2** — supports linear resources/types (Quantitative Type Theory in practice).
  Sources:
  - Idris 2 linear resources docs: https://idris2.readthedocs.io/en/latest/app/linear.html
  - ECOOP paper: https://drops.dagstuhl.de/storage/00lipics/lipics-vol194-ecoop2021/LIPIcs.ECOOP.2021.9/LIPIcs.ECOOP.2021.9.pdf
  - Blog explainer: https://www.type-driven.org.uk/edwinb/linearity-and-erasure-in-idris-2.html
- **Linear Haskell** — practical linearity integrated into Haskell (still typically GC-backed, but linear types give strong protocol and aliasing guarantees).
  Source: https://arxiv.org/pdf/1710.09756
- **Cogent** — uniqueness types designed to eliminate the need for a trusted runtime or garbage collector while guaranteeing memory safety; compiles to C.
  Sources:
  - Docs: https://cogent.readthedocs.io/
  - Paper PDF: https://people.eng.unimelb.edu.au/rizkallahc/publications/cogent-jfp.pdf
- **Futhark** — uniqueness types used to enable safe in-place updates; the paper explicitly compares to Clean and mentions the relationship to Rust ownership ideas.
  Source: https://futhark-lang.org/publications/pldi17.pdf
- **Vault** (research) — “practical linear types” and typestate-style tracking for enforcing protocols and lifetimes.
  Source: https://www.microsoft.com/en-us/research/wp-content/uploads/2002/05/pldi02.pdf
- **Mezzo** (research) — permission-based reasoning about aliasing/ownership, but the paper states it **requires a garbage collector** (compiled via OCaml runtime in prototype).
  Source: https://gallium.inria.fr/~fpottier/publis/mezzo-icfp2013-long.pdf

---

## 3B) Languages closer to C++ (deterministic teardown + explicit memory control)

### B1) RAII / destructors as the primary “resource safety” mechanism
- **C++** — RAII and destructors, classic scope-based resource management.
  Sources:
  - https://en.cppreference.com/w/cpp/language/raii.html
  - https://learn.microsoft.com/en-us/cpp/cpp/object-lifetime-and-resource-management-modern-cpp?view=msvc-170
- **Ada** — controlled types and finalization.
  Sources:
  - https://www.adaic.org/resources/add_content/docs/craft/html/ch16.htm
  - https://learn.adacore.com/courses/advanced-ada/parts/resource_management/controlled_types.html
- **D** — RAII and deterministic destructors for structs; scope guards provide structured cleanup.
  Sources:
  - https://dlang.org/spec/struct.html
  - https://dlang.org/spec/glossary.html
  - https://tour.dlang.org/tour/en/gems/scope-guards

### B2) Deterministic destruction driven by reference counting (ARC/RC)
These feel RAII-like in that destruction happens promptly when references go away, but the mechanism is **reference counting** (and cycles are a well-known edge case).

- **Swift** — Automatic Reference Counting (ARC) frees memory when an instance is no longer needed.
  Source: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/automaticreferencecounting/
- **Nim (ARC/ORC modes)** — runtime “does not use classical GC algorithms anymore” and is “based on destructors and move semantics”; ARC/ORC are reference counting strategies.
  Sources:
  - Destructors & move semantics: https://nim-lang.org/docs/destructors.html
  - ARC/ORC intro: https://nim-lang.org/blog/2020/10/15/introduction-to-arc-orc-in-nim.html
- **Vala** — RC-based memory management; docs explicitly say destructors are deterministic and suitable for RAII; reference cycles may require weak references.
  Sources:
  - Deterministic destructors: https://docs.vala.dev/tutorials/programming-language/main/03-00-object-oriented-programming/03-03-destruction.html
  - RC and cycles: https://wiki.gnome.org/Projects/Vala/ValaForJavaProgrammers

### B3) Manual allocator languages (explicit allocation/free, allocator-passing idioms)
These languages are “C++-ish” in that you explicitly manage heap allocations, often through an allocator abstraction rather than raw `malloc/free`.

- **Zig** — explicit allocator-based allocation; standard library is designed around passing allocators (no hidden allocations).
  Sources:
  - Zig guide allocators: https://zig.guide/standard-library/allocators/
  - (Another overview) https://pedropark99.github.io/zig-book/Chapters/01-memory.html
- **Odin** — explicitly a manual memory management language with strong support for custom allocators (including allocator propagation via `context`).
  Source: https://odin-lang.org/docs/overview/

### B4) GC languages with deterministic *resource* cleanup constructs (not deterministic memory)
These aren’t “C++-like” for memory, but they provide a deterministic *resource* scope mechanism that is often described as RAII-adjacent.

- **C#** — `using` ensures an `IDisposable` is disposed when control leaves the block (even on exceptions).
  Source: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/statements/using
- **Java** — try-with-resources ensures each resource is closed at the end of the statement.
  Source: https://docs.oracle.com/javase/tutorial/essential/exceptions/tryResourceClose.html

---

## 4) Quick “cheat-sheet” tables

### 4.1 Rust/Mojo-like family (compile-time lifetime/aliasing discipline)

| Language | What’s “Rust/Mojo-like” about it | GC/RC required? | Notes |
|---|---|---:|---|
| Rust | Ownership + borrowing; compiler checks references; deterministic `Drop` | No | Archetype of borrow-checker ownership |
| Mojo | Ownership; no GC/RC; destructor on lifetime end | No | Systems-focused ownership model |
| Move | Borrow checker for safe references | No (by design) | Strongly inspired by Rust-style alias rules |
| Vale | Single ownership without borrow checker; memory safe | No (advertised) | Uses different safety mechanisms |
| Cyclone | Region-based memory management + static typing discipline | Mixed | Regions plus integration with GC/stack |
| Verona | Region hierarchy; per-region memory mgmt strategies | Mixed/Pluggable | Explicit region abstraction for concurrency + memory |
| Clean / Mercury / ATS / Idris2 / Linear Haskell / Cogent / Futhark / Vault / Mezzo | Linear/uniqueness/permission ideas controlling aliasing | Varies | Principle similarity: use types to control who can use a value and when |

### 4.2 C++-like family (deterministic teardown and explicit allocation)

| Language | What’s “C++-like” about it | Memory mechanism | Notes |
|---|---|---|---|
| C++ | RAII + destructors; manual allocation available | Manual + RAII | Classic scope-based cleanup |
| Ada | Controlled types + finalization | Deterministic finalization hooks | “RAII-like” for resources |
| D | Struct destructors + scope guards | Mixed (GC exists; structs deterministic) | Scope guards offer structured cleanup |
| Swift | Deterministic deinit via ARC | Reference counting | Cycles require care |
| Nim | ARC/ORC + destructors and move semantics | Reference counting | Designed to avoid “classical GC” in ARC/ORC modes |
| Vala | Reference counting, deterministic destructors | Reference counting | Cycles require weak references |
| Zig / Odin | Explicit allocators / manual memory | Manual | “No allocations behind your back” style patterns |
| C# / Java | Deterministic *resource* scopes | GC for memory | `using` / try-with-resources are RAII-adjacent constructs |

---

## 5) How to read “closeness” in practice

If you want a *very* compact heuristic:

- If you care most about **preventing dangling references and unsafe aliasing by construction**, look to: **Rust, Mojo, Move**, and the **linear/uniqueness/permission** line of languages (some GC-backed).
- If you care most about **predictable cleanup timing** (files, locks, buffers) with relatively “free-form” aliasing, look to: **C++/Ada/D**, plus **ARC/RC** languages (Swift/Nim/Vala) that offer deterministic destruction but with RC trade-offs.
- If you care most about **explicit control over allocation strategy** (arena/bump/per-thread allocators, etc.), look to: **Zig, Odin**, and also RAII-heavy subsets of C++/Rust that lean on custom allocators.

---

## References (accessed 2026-04-06)

Mojo
1. Modular docs — Intro to value lifecycle: https://docs.modular.com/mojo/manual/lifecycle/
2. Modular docs — Intro to value ownership: https://docs.modular.com/mojo/manual/values/
3. Modular docs — Value destruction: https://docs.modular.com/mojo/manual/lifecycle/death/

Rust
4. Rust book — Understanding Ownership: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
5. Rust book — References and Borrowing: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
6. Rust std — `Drop`: https://doc.rust-lang.org/std/ops/trait.Drop.html

Move
7. *The Move Borrow Checker* (paper): https://arxiv.org/pdf/2205.05181

Borrowing research-ish
8. Blech borrowing docs: https://www.blech-lang.org/docs/language-evolution/borrowing/20_borrowing/

Regions
9. Cyclone regions paper: https://www.cs.umd.edu/projects/cyclone/papers/cyclone-regions.pdf
10. Tofte & Talpin — Region-Based Memory Management: https://www.sciencedirect.com/science/article/pii/S0890540196926139
11. MLKit regions manual: https://elsman.com/pdf/mlkit-4.6.0.pdf
12. Verona reference capabilities: https://www.microsoft.com/en-us/research/publication/reference-capabilities-for-flexible-memory-management/
13. Verona paper (PDF): https://arxiv.org/pdf/2309.02983

Linear / uniqueness / permissions
14. Clean wiki: https://clean.cs.ru.nl/
15. Clean 2.2 reference manual, uniqueness typing chapter: https://clean.cs.ru.nl/download/html_report/CleanRep.2.2_11.htm
16. Mercury — Unique modes: https://mercurylang.org/information/doc-release/mercury_ref/Unique-modes.html
17. ATS overview: https://www.cs.bu.edu/~hwxi/atslangweb/
18. Idris 2 linear resources docs: https://idris2.readthedocs.io/en/latest/app/linear.html
19. Idris 2 paper (ECOOP 2021): https://drops.dagstuhl.de/storage/00lipics/lipics-vol194-ecoop2021/LIPIcs.ECOOP.2021.9/LIPIcs.ECOOP.2021.9.pdf
20. Idris 2 blog: https://www.type-driven.org.uk/edwinb/linearity-and-erasure-in-idris-2.html
21. Linear Haskell paper: https://arxiv.org/pdf/1710.09756
22. Cogent docs: https://cogent.readthedocs.io/
23. Cogent paper (PDF): https://people.eng.unimelb.edu.au/rizkallahc/publications/cogent-jfp.pdf
24. Futhark paper (PLDI 2017): https://futhark-lang.org/publications/pldi17.pdf
25. Vault paper (PLDI 2002): https://www.microsoft.com/en-us/research/wp-content/uploads/2002/05/pldi02.pdf
26. Mezzo paper (ICFP 2013): https://gallium.inria.fr/~fpottier/publis/mezzo-icfp2013-long.pdf
27. Vale site: https://vale.dev/

RAII / deterministic cleanup
28. C++ RAII (cppreference): https://en.cppreference.com/w/cpp/language/raii.html
29. Microsoft RAII overview: https://learn.microsoft.com/en-us/cpp/cpp/object-lifetime-and-resource-management-modern-cpp?view=msvc-170
30. Ada controlled types chapter: https://www.adaic.org/resources/add_content/docs/craft/html/ch16.htm
31. AdaCore controlled types tutorial: https://learn.adacore.com/courses/advanced-ada/parts/resource_management/controlled_types.html
32. D scope guards: https://tour.dlang.org/tour/en/gems/scope-guards
33. D struct destructors spec: https://dlang.org/spec/struct.html
34. D RAII glossary entry: https://dlang.org/spec/glossary.html

ARC/RC determinism
35. Swift ARC docs: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/automaticreferencecounting/
36. Nim destructors + move semantics (ARC/ORC): https://nim-lang.org/docs/destructors.html
37. Nim ARC/ORC blog: https://nim-lang.org/blog/2020/10/15/introduction-to-arc-orc-in-nim.html
38. Vala destruction (deterministic due to RC): https://docs.vala.dev/tutorials/programming-language/main/03-00-object-oriented-programming/03-03-destruction.html
39. Vala memory management (RC + cycles): https://wiki.gnome.org/Projects/Vala/ValaForJavaProgrammers

Manual allocator systems languages
40. Zig allocators guide: https://zig.guide/standard-library/allocators/
41. Zig book memory chapter: https://pedropark99.github.io/zig-book/Chapters/01-memory.html
42. Odin overview (manual memory + allocators): https://odin-lang.org/docs/overview/

Deterministic resource scopes in GC languages
43. C# `using`: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/statements/using
44. Java try-with-resources: https://docs.oracle.com/javase/tutorial/essential/exceptions/tryResourceClose.html
