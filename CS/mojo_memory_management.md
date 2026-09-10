# Memory Management in Mojo and Nearby Language Families

## Executive summary

Mojo’s runtime memory model is an **ownership-based, deterministic-destruction** model.

- **No garbage collection and no built-in reference counting.**
- **Single-owner rule with controlled borrowing.** Values have one owner at a time; other code may access a value through references.
- **Deterministic destruction, with an “as soon as possible” policy.** Values are destroyed after last use, not only at the end of a scope.
- **User-defined lifecycle hooks.** Types can define constructors, copy and move constructors, and a destructor that releases resources.

The closest mainstream analogue is **Rust** (ownership and borrowing). The closest mainstream analogue for deterministic destruction without compiler-enforced borrowing is **C++ RAII**.

---

## 1. What Mojo uses at program runtime

### 1.1 Stack and heap

Mojo follows the conventional split of a process’s memory into code and global segments, plus a **stack** and a **heap**:

- The **stack** holds fixed-size local values inside stack frames.
- The **heap** holds dynamically sized or long-lived data. A stack local that “refers to heap data” typically stores a pointer-sized handle, while the underlying payload is heap allocated.

Mojo’s documentation describes this model explicitly and highlights that heap data is the typical source of memory errors when ownership is unclear.

### 1.2 The ownership model

Mojo describes a third approach between tracing garbage collection and fully manual `malloc/free` style management:

- A value has **exactly one owner** at a time.
- Ownership can be **moved** between variables or across function boundaries.
- Non-owning access occurs via **references**, so multiple call sites can use a value without duplicating ownership.

The guiding invariant is that ownership rules prevent the classic error modes:

- use-after-free
- double free
- leaks that come from ambiguous responsibility for freeing

### 1.3 Deterministic destruction and lifecycle methods

Mojo ties resource release to value lifetime through lifecycle methods on `struct` types:

- `__init__()` for construction
- `__init__(copy=)` and `__init__(take=)` for copy and move construction
- `__del__()` for destruction

The destructor is where a type author releases heap memory or other resources owned by the value.

A distinctive detail in Mojo is its destruction timing: the documentation states that Mojo destroys values **as soon as they are no longer used**, using an **ASAP** policy that can run after every sub-expression.

### 1.4 “Automatic” feeling without a garbage collector

Mojo is intended to feel productive like a high-level language while keeping systems-level control.

The developer experience is often described as “memory safe without manual memory management” for typical code, while still allowing manual control for custom types or explicit pointer usage.

---

## 2. A taxonomy of memory-management principles

The languages below can be grouped by which **mechanism decides when heap objects are reclaimed** and which **rules control aliasing**.

### 2.1 Tracing garbage collection

- A runtime collector discovers unreachable objects and reclaims them.
- Pros: minimal lifetime reasoning for developers.
- Cons: runtime overhead, pauses, and less predictable reclamation.

### 2.2 Reference counting

- A runtime counter tracks how many references exist to an object.
- Pros: often deterministic deallocation when count reaches zero.
- Cons: overhead on reference updates; cycles require special handling.

### 2.3 RAII / deterministic destruction

- Resource lifetime is tied to object lifetime.
- Destructors (or scope-exit constructs) perform cleanup at predictable points.
- RAII can manage both memory and non-memory resources (files, locks, sockets).

### 2.4 Ownership and borrowing

- The compiler enforces a single-owner invariant and restricts aliasing.
- Borrowing rules define when references may exist and whether mutation is permitted.
- Goal: memory safety without a tracing GC.

### 2.5 Regions

- Objects are allocated into regions; entire regions are reclaimed at once.
- Pros: very fast allocation and reclamation; predictable costs.
- Cons: lifetimes must fit region structure; escaping values can complicate design.

### 2.6 Linear, affine, and uniqueness typing

- The type system restricts how many times a value can be used or aliased.
- Linear and uniqueness systems can enable safe in-place update and predictable release.
- These systems are often used to enforce resource protocols at compile time.

---

## 3. Languages closer to Rust and Mojo

This section emphasizes languages whose memory story is driven primarily by **compile-time lifetime and aliasing control**, typically aiming for **memory safety without a tracing GC**.

### 3.1 Ownership and borrowing (Rust-style)

**Rust**

- Ownership plus borrowing rules enforced at compile time.
- Deterministic destruction when owners go out of scope or values are dropped.

**Mojo**

- Ownership-based memory management with deterministic destruction.
- No built-in garbage collector and no built-in reference counter.
- Destruction after last use via an ASAP policy.

**Move**

- Designed for safe programming with digital assets.
- Uses a borrow-checking discipline for references, with verification at load time in typical deployments.

### 3.2 Single ownership without a classic borrow checker (Rust-adjacent)

**Vale**

- Markets itself as memory-safe **single ownership** without garbage collection.
- Positions itself as achieving safety without Rust’s style of borrow checker.

This group is conceptually close to Mojo’s “one owner” framing, but may differ substantially in ergonomics and formal guarantees.

### 3.3 Region-based systems

**Cyclone**

- A type-safe dialect of C.
- Uses **region-based memory management** integrated with stack allocation.
- Research lineage focused on safety while retaining C-like control.

**Standard ML with regions (MLKit with Regions)**

- Region inference for SML.
- Replaces GC in many cases by allocating into regions and reclaiming regions.

**Project Verona (Reggio / isolated regions)**

- Uses hierarchical isolated regions.
- Supports choosing a memory management strategy per region, while controlling aliasing with a capability system.

**Mercury (research on RBMM)**

- Research work exists on region-based memory management for Mercury programs.
- Included here as part of the region-based lineage, even though it is not a mainstream default memory mode.

### 3.4 Linear, affine, and uniqueness-type systems

These languages are close in *principle* to ownership-based safety, but often look different in everyday programming.

**ATS**

- Uses linear types to reduce memory footprint and enforce safe resource usage.
- Commonly used to express precise invariants about low-level data.

**Clean**

- Uniqueness typing enables single-threaded use of objects and can support destructive update in a pure setting.

**Cogent**

- A restricted functional systems language.
- Uniqueness types are used to guarantee memory safety **without** relying on a trusted runtime or garbage collector.

**Idris 2**

- Supports linear resource usage protocols through quantitative typing.
- Often used to encode and enforce correct resource handling.

**Linear Haskell**

- Integrates linear function arrows into Haskell.
- Enables safe mutable data behind pure interfaces and protocol enforcement, while still living in a GC ecosystem.

**Futhark**

- Uses uniqueness types to support in-place updates in a functional array language.

**Alms**

- A practical language featuring affine types.
- Demonstrates resource-aware abstractions enabled by restricted aliasing.

### 3.5 Permission and typestate approaches (adjacent)

**Vault**

- Research work on practical linear types for imperative programming.
- Tracks lifetimes and object state transitions to enforce protocols.

**Mezzo**

- Permission-based alias control.
- Notably, Mezzo is designed to require a garbage collector, so it is adjacent in aliasing theory but not in runtime memory management.

---

## 4. Languages closer to C++ memory-management principles

This section emphasizes languages where the primary idiom is **deterministic destruction** and **explicit control**, but where global borrow-checker style alias rules are not the defining feature.

### 4.1 RAII and deterministic destructors

**C++**

- RAII ties resource release to object lifetime.
- Destructors run deterministically on scope exit for stack objects.

**Ada (controlled types)**

- Controlled and limited controlled types provide explicit initialization, adjustment, and finalization hooks.
- Enables RAII-like patterns for resource management.

**D**

- Supports RAII patterns and scope-based cleanup.
- Also has a tracing GC in common configurations, but can be used in styles that avoid GC for critical code.

### 4.2 Deterministic destruction driven by reference counting

These languages are often “RAII-like” for resources, but their memory reclamation is based on reference counting.

**Swift**

- Uses ARC (automatic reference counting) and `deinit` for cleanup.

**Objective-C (ARC mode)**

- Similar ARC-based lifetime model (historically a precursor to Swift’s approach).

**Nim (ARC or ORC runtimes)**

- ARC and ORC configurations avoid classical tracing GC.
- The model is explicitly described as being based on destructors and move semantics.

**Vala**

- Reference counting based memory management.
- Documentation explicitly notes deterministic destructors that can implement RAII.

### 4.3 Manual allocation with allocator-centric design

This family shares C and C++’s “explicit allocation” feel, often with stronger tooling for allocator plumbing and safer defaults.

**Zig**

- Manual memory management with explicit allocators.
- No garbage collector; allocation and free are explicit.

**Odin**

- Manual memory management language with strong support for custom allocators.

**Jai**

- Manual memory management.
- Uses `defer` as a structured scope-exit mechanism to reduce resource leaks.

### 4.4 GC languages that still offer deterministic *resource* cleanup

These languages typically use tracing GC for memory, but provide constructs to deterministically release external resources.

**C#**

- `using` with `IDisposable` provides deterministic cleanup for unmanaged resources.

**Java**

- `try-with-resources` ensures `AutoCloseable` resources are closed at the end of the statement.

These constructs are closer to RAII for **non-memory resources** than to ownership-based reclamation of heap objects.

---

## 5. Compact comparison table

| Mechanism family | Representative languages | Heap reclamation trigger | Deterministic deallocation | Primary safety lever |
|---|---|---:|---:|---|
| Ownership + borrowing | Rust, Mojo, Move | Static rules decide last-use ownership and drops | Yes | Compile-time aliasing control |
| RAII destructors | C++, Ada, D (in RAII styles) | Scope exit and object lifetime | Yes | Deterministic destructors + conventions |
| Reference counting | Swift, Vala, Nim ARC/ORC | Refcount reaches zero | Often yes | Runtime refcounts + cycle avoidance |
| Regions | Cyclone, MLKit Regions, Verona | Region end or policy | Yes (per region) | Region partitioning + capability rules |
| Linear or uniqueness types | ATS, Clean, Cogent, Idris 2, Futhark | Type rules constrain aliasing and consumption | Often yes | Substructural typing |
| Tracing GC + deterministic resource cleanup | C#, Java | Runtime GC; explicit `Dispose/close` | Resources: yes, Memory: no | Explicit cleanup APIs |

---

## 6. How the two “closeness” clusters differ

### 6.1 Rust and Mojo cluster

- Memory safety is primarily obtained by **compile-time rules about aliasing and lifetime**.
- Deterministic destruction is a core part of the model.
- The strongest similarity to Mojo is found in Rust, and in borrow-checking languages like Move.

### 6.2 C++ cluster

- Deterministic destruction is achieved via **destructors and RAII patterns**, but the compiler usually does not enforce a global “one mutable reference at a time” discipline.
- Memory safety is typically achieved by coding patterns, libraries, tooling, and restricted subsets rather than by a borrow checker.
- Zig, Odin, and Jai often feel “C-like” in allocation responsibility, while improving ergonomics around allocator passing and scope-exit cleanup.

---

## 7. Further reading (primary sources and classic papers)

- Mojo manual: value ownership and value lifecycle (Modular docs)
- Rust: ownership and borrowing (Rust Book)
- The Move Borrow Checker (Blackshear, Mitchell, Nowacki, Qadeer)
- Region-Based Memory Management in Cyclone (Grossman et al.)
- Region-Based Memory Management (Tofte and Talpin) and MLKit with Regions
- Reference Capabilities for Flexible Memory Management (Project Verona / Reggio)
- Adoption and Focus: Practical Linear Types for Imperative Programming (Vault)
- Clean language report and uniqueness typing
- Cogent: Uniqueness Types and Certifying Compilation
- Linear Haskell: practical linearity in a higher-order polymorphic language
- Futhark: uniqueness types for in-place updates

