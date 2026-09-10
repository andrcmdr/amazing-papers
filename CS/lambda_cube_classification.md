# Programming Languages Through the Lens of the Lambda Cube

## Introduction

In 1991, Henk Barendregt introduced the **Lambda Cube** (λ-cube), a framework that classifies eight systems of typed lambda calculus along three axes of abstraction. Each axis represents a kind of dependency between **terms** (values, expressions — the world of `∗`) and **types** (the world of `◻`):

| Dependency | Notation | Meaning | Common Name |
|---|---|---|---|
| Terms → Terms | `(∗, ∗)` | Functions: values computed from values | **Ordinary computation** |
| Types → Terms | `(◻, ∗)` | Polymorphism: values parameterised by types | **Parametric polymorphism** |
| Terms → Types | `(∗, ◻)` | Dependent types: types computed from values | **Dependent types** |
| Types → Types | `(◻, ◻)` | Type operators: types computed from types | **Type constructors** |

Every typed language supports `(∗, ∗)` — plain functions. The three binary switches `(◻, ∗)`, `(∗, ◻)`, and `(◻, ◻)` then yield **eight vertices** of the cube:

```
                        λΠω (Calculus of Constructions)
                       ╱ |                           |
                     ╱   |                           |
                   λΠω̲   |                          λΠ2
                  ╱  |   |                         ╱  |
                ╱    |   |                       ╱    |
             λω̲     |  λΠ                    λ2      |
               ╲     |  ╱                      ╲     |
                 ╲   |╱                          ╲   |
                  λ→ ─────────────────────────── (origin)
```

| # | System | `(◻,∗)` | `(∗,◻)` | `(◻,◻)` | Informal description |
|---|--------|:---:|:---:|:---:|---|
| 1 | **λ→** | — | — | — | Simply-typed lambda calculus |
| 2 | **λ2** (System F) | ✓ | — | — | Polymorphism |
| 3 | **λω̲** | — | — | ✓ | Type operators |
| 4 | **λΠ** (LF) | — | ✓ | — | Dependent types |
| 5 | **λ2ω̲** (System Fω) | ✓ | — | ✓ | Polymorphism + type operators |
| 6 | **λΠ2** | ✓ | ✓ | — | Polymorphism + dependent types |
| 7 | **λΠω̲** | — | ✓ | ✓ | Dependent types + type operators |
| 8 | **λΠω** (CoC) | ✓ | ✓ | ✓ | All three — Calculus of Constructions |

> **Note on practical languages.** No real-world language maps perfectly onto a single vertex. Languages are engineering artifacts that mix, restrict, or approximate these abstractions. The classification below indicates the *highest vertex a language substantially inhabits* and notes which axes it supports partially.

---

## 1. λ→ — Simply-Typed Lambda Calculus

**Supports:** `(∗, ∗)` only — terms depend on terms.

This is the minimal typed system: every function has a fixed, monomorphic type. There is no way to abstract over types.

### Languages at this level

| Language | Paradigm | Era | Notes |
|---|---|---|---|
| **C** (pre-C11) | Imperative/procedural | 1972 | No generics; `void*` is untyped escape hatch |
| **Pascal** | Procedural | 1970 | Strictly monomorphic |
| **Fortran 77** | Imperative | 1978 | Fixed types, no polymorphism |
| **BASIC** | Imperative | 1964 | Primitive type system |
| **COBOL** | Imperative | 1959 | Data-description oriented, no abstraction |
| **Go** (pre-1.18) | Imperative + structural typing | 2009 | Deliberately omitted generics until 2022 |

### Example: C

```c
// (∗, ∗) — terms depend on terms: a plain function
int square(int x) {
    return x * x;
}

// No polymorphism — we must write separate functions or resort to void*
int max_int(int a, int b) { return a > b ? a : b; }
double max_double(double a, double b) { return a > b ? a : b; }

// The "escape hatch": void* loses all type information
void* identity(void* x) { return x; }
```

### Example: Pascal

```pascal
{ Monomorphic — a swap procedure only for integers }
procedure SwapInt(var a, b: Integer);
var tmp: Integer;
begin
  tmp := a; a := b; b := tmp;
end;

{ Must be duplicated for every type }
procedure SwapReal(var a, b: Real);
var tmp: Real;
begin
  tmp := a; a := b; b := tmp;
end;
```

### OOP angle: early object systems

Early Smalltalk (1972–1980) and Objective-C use dynamic dispatch but have no *static* type-level polymorphism — the type system itself is essentially monomorphic (or absent). Method lookup is a runtime mechanism, not a type-system feature.

```objectivec
// Objective-C — message passing, but no generic types
- (id)identity:(id)x {
    return x;  // 'id' is the universal untyped pointer
}
```

---

## 2. λ2 — System F (Polymorphism)

**Supports:** `(∗, ∗)` + `(◻, ∗)` — terms depend on terms, and terms depend on types.

This is the axis of **parametric polymorphism**: a single function definition can work uniformly over all types. The caller chooses the type; the function body is agnostic.

### Languages at this level

| Language | Paradigm | Era | Notes |
|---|---|---|---|
| **Haskell 98** (without extensions) | Functional | 1990 | Hindley–Milner ≈ predicative fragment of System F |
| **ML / OCaml** (core) | Functional | 1973/1996 | Let-polymorphism (rank-1) |
| **F#** (core) | Functional + OOP | 2005 | ML-family polymorphism on .NET |
| **Java** (5+, with erasure) | OOP | 1995 | Generics added 2004; erased at runtime |
| **C#** (2.0+) | OOP | 2000 | Reified generics |
| **Swift** | Multi-paradigm | 2014 | Generics with protocol constraints |
| **Kotlin** | Multi-paradigm | 2011 | JVM generics with declaration-site variance |
| **Dart** (2.0+) | OOP | 2011 | Sound generic type system |
| **Go** (1.18+) | Imperative + structural | 2009 | Type parameters added 2022 |
| **Elm** | Functional | 2012 | Hindley–Milner, no higher-kinded types |

### Example: Haskell (Hindley–Milner polymorphism)

```haskell
-- (◻, ∗) — a term (value) parameterised by a type variable 'a'
-- 'id' works for any type: Int, String, [Bool], ...
id :: a -> a
id x = x

-- Polymorphic list map
map :: (a -> b) -> [a] -> [b]
map _ []     = []
map f (x:xs) = f x : map f xs

-- Usage: the type variable is instantiated by the compiler
-- map (+1) [1,2,3]        — here a = Int, b = Int
-- map show [True, False]  — here a = Bool, b = String
```

### Example: Java generics (OOP approach to polymorphism)

```java
// (◻, ∗) — the class is parameterised by a type T
public class Box<T> {
    private T value;
    
    public Box(T value) { this.value = value; }
    public T getValue() { return value; }
    
    // A polymorphic method — <U> is a type parameter on the method
    public <U> Box<U> map(Function<T, U> f) {
        return new Box<>(f.apply(value));
    }
}

// Usage
Box<Integer> intBox = new Box<>(42);
Box<String> strBox = intBox.map(Object::toString);
```

### Example: Go 1.18+ (late addition of generics)

```go
// Before 1.18: monomorphic, must use interface{} (≈ void*)
// After 1.18: (◻, ∗) — type parameters
func Map[T any, U any](xs []T, f func(T) U) []U {
    result := make([]U, len(xs))
    for i, x := range xs {
        result[i] = f(x)
    }
    return result
}
```

### Example: Swift (generics + protocol constraints)

```swift
// (◻, ∗) — generic function with a constraint (bounded polymorphism)
func largest<T: Comparable>(_ a: T, _ b: T) -> T {
    return a > b ? a : b
}

// OOP angle: protocols as type-class-like constraints
protocol Summable {
    static func +(lhs: Self, rhs: Self) -> Self
}

func sum<T: Summable>(_ items: [T]) -> T {
    items.reduce(items[0], +)
}
```

### Functional vs. OOP polymorphism

The same `(◻, ∗)` axis manifests differently across paradigms:

| Functional style | OOP style |
|---|---|
| Type variables in signatures: `a -> a` | Type parameters on classes: `class Box<T>` |
| Type classes (Haskell) / traits (Rust) constrain variables | Interfaces / abstract classes constrain parameters |
| Polymorphism is *parametric* — uniform behavior | Often mixes parametric + subtype polymorphism |
| Instantiation inferred | Often requires explicit `<Type>` annotations |

---

## 3. λω̲ — Type Operators

**Supports:** `(∗, ∗)` + `(◻, ◻)` — terms depend on terms, and types depend on types.

This axis introduces **type-level functions**: types that are computed from other types. The canonical example is a *higher-kinded type* — a type constructor like `List` that takes a type and returns a type: `List : Type → Type`.

### Languages at this level (type operators without polymorphism are rare)

Pure `λω̲` without polymorphism is unusual in practice; most languages that support `(◻, ◻)` also support `(◻, ∗)`, placing them in System Fω (section 5). However, some systems have type-level computation without full parametric polymorphism:

| Language | Paradigm | Notes |
|---|---|---|
| **C** (with `typedef` + macros) | Procedural | Preprocessor performs text-level "type operators" — not truly in the type system |
| **C++** (templates, pre-concepts) | Multi-paradigm | Template metaprogramming is Turing-complete type-level computation, but pre-concepts it lacks proper bounded polymorphism |

### Example: C++ template metaprogramming as type operators

```cpp
// (◻, ◻) — a type computed from a type
// 'Pointer' is a type-level function: Type → Type
template<typename T>
using Pointer = T*;

// A more complex type operator: compile-time list of types
template<typename... Ts>
struct TypeList {};

// Type-level function: prepend a type to a type list
template<typename T, typename List>
struct Prepend;

template<typename T, typename... Ts>
struct Prepend<T, TypeList<Ts...>> {
    using type = TypeList<T, Ts...>;
};

// Usage: purely at the type level
using MyList = Prepend<int, TypeList<double, char>>::type;
// MyList = TypeList<int, double, char>
```

---

## 4. λΠ — Dependent Types (LF)

**Supports:** `(∗, ∗)` + `(∗, ◻)` — terms depend on terms, and types depend on terms.

This is the axis of **dependent types** in their pure form: a *type* can mention and be computed from a *value*. The classic example is `Vector n a` — a list whose type encodes its length.

### Languages at this level

Full dependent types without polymorphism are rare in isolation; research systems tend to include `(◻, ∗)` too. But some languages provide *limited* dependent typing:

| Language | Paradigm | Notes |
|---|---|---|
| **LF / Twelf** | Logic framework | Pure dependent types for encoding logics |
| **Dependent ML** (DML) | Functional | Xi's extension of ML with limited index types |

### Approximations in mainstream languages

Several mainstream languages approximate `(∗, ◻)` through restricted mechanisms:

```typescript
// TypeScript — literal types and conditional types approximate (∗, ◻)
// The return type depends on the *value* of the input (via literal narrowing)
function getLength(s: "hello"): 5;
function getLength(s: "hi"): 2;
function getLength(s: string): number;
function getLength(s: string): number {
    return s.length;
}

// Conditional types — type depends on structure of another type
type IsString<T> = T extends string ? "yes" : "no";
// IsString<"hello"> = "yes"
// IsString<42>      = "no"
```

```python
# Python — typing.Literal (PEP 586) provides a tiny slice of (∗, ◻)
from typing import Literal, overload

@overload
def open_file(mode: Literal["r"]) -> str: ...
@overload
def open_file(mode: Literal["rb"]) -> bytes: ...
def open_file(mode: str):
    ...
```

---

## 5. λ2ω̲ — System Fω (Polymorphism + Type Operators)

**Supports:** `(∗, ∗)` + `(◻, ∗)` + `(◻, ◻)` — polymorphism and type-level computation, but types cannot depend on runtime values.

This is the **sweet spot for most modern statically-typed languages**. You get generics (polymorphism) and higher-kinded types or type-level functions (type operators), but no true dependent types.

### Languages at this level

| Language | Paradigm | Era | Notes |
|---|---|---|---|
| **Haskell** (with extensions) | Functional | 1990+ | `TypeFamilies`, `DataKinds`, `GADTs` — pushes toward λΠω but the core is Fω |
| **Scala** (2 & 3) | FP + OOP | 2004 | Higher-kinded types, match types (Scala 3) |
| **Rust** | Systems / multi-paradigm | 2010 | Traits with associated types, const generics (partial `(∗,◻)`) |
| **OCaml** (with modules) | Functional | 1996 | Functors = type operators over module signatures |
| **C++** (modern, with concepts) | Multi-paradigm | 1979+ | Templates + concepts ≈ bounded polymorphism + type operators |
| **TypeScript** | Multi-paradigm | 2012 | Generic types + mapped/conditional types = powerful (◻,◻) |
| **Kotlin** (advanced) | Multi-paradigm | 2011 | Reified generics, star-projections |
| **PureScript** | Functional | 2013 | Full higher-kinded polymorphism |

### Example: Haskell — higher-kinded types and type families

```haskell
-- (◻, ∗) — polymorphism: 'f' is a type variable of kind * -> *
class Functor f where
    fmap :: (a -> b) -> f a -> f b

-- (◻, ◻) — a type-level function (type family)
type family Element (container :: *) :: * where
    Element [a]        = a
    Element (Set a)    = a
    Element ByteString = Word8

-- Combining both: a polymorphic function using a type family
class Container c where
    type Elem c :: *
    empty  :: c
    insert :: Elem c -> c -> c

instance Container [Int] where
    type Elem [Int] = Int
    empty  = []
    insert = (:)
```

### Example: Scala 3 — higher-kinded types and match types

```scala
// (◻, ∗) — polymorphism with a higher-kinded type parameter
trait Functor[F[_]]:
  def map[A, B](fa: F[A])(f: A => B): F[B]

given Functor[List] with
  def map[A, B](fa: List[A])(f: A => B): List[B] = fa.map(f)

// (◻, ◻) — match types: type-level pattern matching
type Elem[X] = X match
  case List[a]   => a
  case Option[a] => a
  case String    => Char

// Elem[List[Int]] = Int
// Elem[String]    = Char

// OOP angle: path-dependent types (a Scala specialty)
class Graph:
  class Node:
    def connectTo(other: Node): Unit = ???  // 'Node' is *this* graph's Node

val g1 = new Graph
val g2 = new Graph
// g1.Node and g2.Node are different types — the type depends on the object path
```

### Example: Rust — traits, associated types, and const generics

```rust
// (◻, ∗) — generic function (polymorphism)
fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// (◻, ◻) — associated types as type operators
trait Container {
    type Item;                       // a type computed from the implementing type
    fn first(&self) -> Option<&Self::Item>;
}

impl Container for Vec<i32> {
    type Item = i32;
    fn first(&self) -> Option<&i32> { self.get(0) }
}

// Partial (∗, ◻) — const generics: types parameterised by values
// (This is a limited form of dependent typing)
struct Matrix<const ROWS: usize, const COLS: usize> {
    data: [[f64; COLS]; ROWS],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    fn transpose(&self) -> Matrix<C, R> {
        let mut result = Matrix { data: [[0.0; R]; C] };
        for i in 0..R {
            for j in 0..C {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}
// Matrix<3, 4>.transpose() returns Matrix<4, 3> — checked at compile time
```

### Example: C++ — concepts + templates

```cpp
// (◻, ∗) — bounded polymorphism via concepts (C++20)
template<typename T>
concept Numeric = requires(T a, T b) {
    { a + b } -> std::same_as<T>;
    { a * b } -> std::same_as<T>;
};

template<Numeric T>
T dot(const std::vector<T>& a, const std::vector<T>& b) {
    T sum{};
    for (size_t i = 0; i < a.size(); ++i)
        sum = sum + a[i] * b[i];
    return sum;
}

// (◻, ◻) — type-level computation
template<typename T>
struct AddPointer { using type = T*; };

template<typename T>
struct AddPointer<T*> { using type = T*; };   // idempotent

// AddPointer<int>::type   = int*
// AddPointer<int*>::type  = int*
```

### Example: TypeScript — mapped and conditional types

```typescript
// (◻, ∗) — generic function
function identity<T>(x: T): T { return x; }

// (◻, ◻) — mapped type: a type operator over object types
type Readonly<T> = {
    readonly [K in keyof T]: T[K];
};

// Conditional type: type-level 'if'
type Flatten<T> = T extends Array<infer U> ? U : T;
// Flatten<string[]>  = string
// Flatten<number>    = number

// Recursive conditional type
type DeepReadonly<T> = {
    readonly [K in keyof T]: T[K] extends object ? DeepReadonly<T[K]> : T[K];
};
```

### OOP approach: how object-oriented languages encode type operators

In class-based OOP, the equivalent of a type operator is often an **abstract type member** or a **generic class used as a type constructor**:

```scala
// Scala — abstract type members as type operators
trait Collection:
  type Elem                         // abstract: will be defined by subtypes
  type Self <: Collection           // F-bounded: the type refers to itself
  def map(f: Elem => Elem): Self

class IntList extends Collection:
  type Elem = Int
  type Self = IntList
  def map(f: Int => Int): IntList = ???
```

```csharp
// C# — generic interfaces as type operators
// IEnumerable<T> is essentially a type function: Type → Type
public interface IEnumerable<T> {
    IEnumerator<T> GetEnumerator();
}

// LINQ uses this for polymorphic, type-operator-driven pipelines
var names = people
    .Where(p => p.Age > 18)          // IEnumerable<Person> → IEnumerable<Person>
    .Select(p => p.Name);            // IEnumerable<Person> → IEnumerable<string>
```

---

## 6–7. λΠ2 and λΠω̲ — Dependent Types with Polymorphism or Type Operators

These intermediate vertices are mostly inhabited by **research languages** and **proof assistants** that have dependent types combined with one other feature:

| System | Languages | Notes |
|---|---|---|
| **λΠ2** (dependent types + polymorphism) | Cayenne, early versions of Epigram | Polymorphism where types can also depend on values |
| **λΠω̲** (dependent types + type operators) | Some formulations of Martin-Löf type theory | Types depend on values, and type-level functions exist |

In practice, once a language commits to dependent types, it almost always ends up at the full λΠω corner (section 8).

### Approximations in production languages

Several production languages reach *toward* these vertices without fully inhabiting them:

```rust
// Rust's const generics — a limited form of λΠ2
// The type depends on a *value* (the const parameter), and it's also polymorphic in T
struct Array<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Array<T, N> {
    fn new() -> Self {
        Array { data: [T::default(); N] }
    }
}

// Usage: Array<f64, 3> and Array<f64, 4> are different types
let v3: Array<f64, 3> = Array::new();
let v4: Array<f64, 4> = Array::new();
// v3 = v4;  // compile error — different types!
```

```haskell
-- Haskell with DataKinds + GADTs — simulating dependent types
{-# LANGUAGE DataKinds, GADTs, KindSignatures, TypeFamilies #-}

data Nat = Zero | Succ Nat   -- promoted to the type level by DataKinds

data Vec (n :: Nat) (a :: *) where
    VNil  :: Vec 'Zero a
    VCons :: a -> Vec n a -> Vec ('Succ n) a

-- The type of 'head' guarantees non-emptiness
vhead :: Vec ('Succ n) a -> a
vhead (VCons x _) = x

-- Type-safe append: lengths add up at the type level
type family Add (m :: Nat) (n :: Nat) :: Nat where
    Add 'Zero     n = n
    Add ('Succ m) n = 'Succ (Add m n)

vappend :: Vec m a -> Vec n a -> Vec (Add m n) a
vappend VNil         ys = ys
vappend (VCons x xs) ys = VCons x (vappend xs ys)
```

```scala
// Scala 3 — literal types and inline provide a taste of (∗, ◻)
import compiletime.ops.int.*

type Plus[A <: Int, B <: Int] = A + B   // type-level arithmetic

// The return type is computed from the *value* of the type parameters
inline def checkedAdd[A <: Int, B <: Int](
    a: ValueOf[A], b: ValueOf[B]
): ValueOf[Plus[A, B]] =
    new ValueOf(a.value + b.value)
```

---

## 8. λΠω — The Calculus of Constructions (Full Lambda Cube)

**Supports:** all four dependencies — `(∗,∗)`, `(◻,∗)`, `(∗,◻)`, `(◻,◻)`.

This is the apex of the cube: terms can depend on terms and types; types can depend on terms and types. Every other system is a subsystem of this one. Languages here can express propositions as types and proofs as programs (the Curry–Howard correspondence in its full generality).

### Languages at this level

| Language | Paradigm | Era | Notes |
|---|---|---|---|
| **Coq** (Rocq) | Proof assistant / functional | 1989 | Based on the Calculus of Inductive Constructions |
| **Agda** | Dependently-typed functional | 2007 | Full dependent types with pattern matching |
| **Lean** (4) | Proof assistant / functional | 2013 | Dependent types + powerful tactic framework |
| **Idris** (1 & 2) | Dependently-typed, general-purpose | 2011 | Designed as a *programming* language, not just a prover |
| **F*  (F-star)** | Dependently-typed + effects | 2011 | Combines dependent types with refinement types and effects |
| **ATS** | Systems programming | 2004 | Dependent types + linear types for low-level programming |
| **Pie** | Educational | 2018 | Minimal dependently-typed language from "The Little Typer" |

### Example: Agda — full dependent types

```agda
-- All four dependencies in action

-- (∗, ∗) — ordinary function
double : ℕ → ℕ
double n = n + n

-- (◻, ∗) — polymorphism: a term parameterised by a type
id : (A : Set) → A → A
id A x = x

-- (∗, ◻) — dependent type: the type Vec A n depends on the value n
data Vec (A : Set) : ℕ → Set where
  []  : Vec A zero
  _∷_ : {n : ℕ} → A → Vec A n → Vec A (suc n)

-- Type-safe head: the type *guarantees* the vector is non-empty
head : {A : Set} {n : ℕ} → Vec A (suc n) → A
head (x ∷ _) = x

-- (◻, ◻) — type operator: a type-level function
Pair : Set → Set → Set
Pair A B = A × B

-- Combining all: a dependently-typed, polymorphic append
-- whose return type is computed from runtime values
_++_ : {A : Set} {m n : ℕ} → Vec A m → Vec A n → Vec A (m + n)
[]       ++ ys = ys
(x ∷ xs) ++ ys = x ∷ (xs ++ ys)

-- Proof as a program: n + 0 ≡ n
+-identity : (n : ℕ) → n + 0 ≡ n
+-identity zero    = refl
+-identity (suc n) = cong suc (+-identity n)
```

### Example: Idris 2 — dependently-typed general-purpose programming

```idris
-- A type-safe printf: the format string (a value) determines the function's type

-- (∗, ◻) — the type is computed from the format string value
data Format = FInt Format | FStr Format | FLit Char Format | FEnd

-- Parse a format string into a Format descriptor
parseFormat : List Char -> Format
parseFormat ('%' :: 'd' :: rest) = FInt (parseFormat rest)
parseFormat ('%' :: 's' :: rest) = FStr (parseFormat rest)
parseFormat (c :: rest)          = FLit c (parseFormat rest)
parseFormat []                   = FEnd

-- Compute the function type from the Format
PrintfType : Format -> Type
PrintfType (FInt rest) = Int -> PrintfType rest
PrintfType (FStr rest) = String -> PrintfType rest
PrintfType (FLit _ rest) = PrintfType rest
PrintfType FEnd = String

-- The implementation
printf : (fmt : String) -> PrintfType (parseFormat (unpack fmt))
-- ... (implementation omitted for brevity)

-- Usage:
-- printf "%s is %d years old" : String -> Int -> String
```

### Example: Lean 4 — theorem proving and programming

```lean
-- Dependent types: Vector indexed by length
inductive Vec (α : Type) : Nat → Type where
  | nil  : Vec α 0
  | cons : α → Vec α n → Vec α (n + 1)

-- The type prevents calling head on an empty vector
def Vec.head : Vec α (n + 1) → α
  | .cons a _ => a

-- Proof that list reversal preserves length
theorem reverse_length (xs : List α) : (xs.reverse).length = xs.length := by
  induction xs with
  | nil => simp
  | cons x xs ih => simp [List.reverse_cons, ih]
```

### Example: ATS — dependent types for systems programming

```ats
(* ATS: dependent types meeting C-level performance *)

(* A type-safe array access: the index is bounded by the array size *)
fun {a:t@ype} array_get
  {n:int} {i:nat | i < n}
  (arr: array(a, n), i: int(i)): a

(* Matrix multiplication where dimensions must match *)
fun {a:t@ype} matmul
  {m,n,p:nat}
  (A: matrix(a, m, n), B: matrix(a, n, p)): matrix(a, m, p)
```

---

## Comparative Summary

### The Cube at a Glance

```
                     Dependent Types (∗, ◻)
                          ▲
                          │
    λΠ                    │                λΠω  (CoC)
    LF, Twelf             │                Coq, Agda, Lean, Idris
                          │
                          │
    λ→                    │                λω̲
    C, Pascal,            │                (rare in isolation)
    Fortran               │
                          │
         ─────────────────┼──────────────────▶  Type Operators (◻, ◻)
                          │
    λ2 (System F)         │                λ2ω̲  (System Fω)
    Haskell 98, ML,       │                Haskell+, Scala, Rust,
    Java generics,        │                C++, TypeScript,
    Go 1.18+              │                OCaml modules
                          │
                          │
    λΠ2                   │                λΠω̲
    (rare)                │                (rare)
                          │
                          ▼
                  Polymorphism (◻, ∗)
```

### Language Classification Table

| Language | Paradigm | `(◻,∗)` Poly | `(∗,◻)` Dep | `(◻,◻)` TyOp | Vertex | Notes |
|---|---|:---:|:---:|:---:|---|---|
| C | Procedural | — | — | — | λ→ | `void*` for ad-hoc polymorphism |
| Pascal | Procedural | — | — | — | λ→ | |
| Fortran 77 | Imperative | — | — | — | λ→ | |
| COBOL | Imperative | — | — | — | λ→ | |
| Go (pre-1.18) | Imperative | — | — | — | λ→ | `interface{}` for ad-hoc |
| Go (1.18+) | Imperative | ✓ | — | — | λ2 | Rank-1 generics only |
| Java (5+) | OOP | ✓ | — | — | λ2 | Erased generics |
| C# (2.0+) | OOP | ✓ | — | — | λ2 | Reified generics |
| Swift | Multi | ✓ | — | — | λ2 | Protocol-constrained generics |
| Kotlin | Multi | ✓ | — | — | λ2 | Declaration-site variance |
| Elm | Functional | ✓ | — | — | λ2 | Hindley–Milner |
| Haskell 98 | Functional | ✓ | — | — | λ2 | Let-polymorphism |
| ML / OCaml (core) | Functional | ✓ | — | — | λ2 | |
| Dart | OOP | ✓ | — | — | λ2 | |
| Haskell + exts | Functional | ✓ | ≈ | ✓ | Fω → CoC | `DataKinds`, `TypeFamilies`, singletons |
| Scala 2/3 | FP + OOP | ✓ | ≈ | ✓ | Fω | Match types, path-dependent types |
| Rust | Systems | ✓ | ≈ | ✓ | Fω | Const generics, associated types |
| C++ (modern) | Multi | ✓ | ≈ | ✓ | Fω | Templates + concepts |
| TypeScript | Multi | ✓ | ≈ | ✓ | Fω | Conditional + mapped types |
| OCaml (modules) | Functional | ✓ | — | ✓ | Fω | Module functors |
| PureScript | Functional | ✓ | — | ✓ | Fω | Row polymorphism, HKT |
| F# | FP + OOP | ✓ | — | ≈ | λ2–Fω | Limited type operators |
| Coq / Rocq | Proof assistant | ✓ | ✓ | ✓ | λΠω | CIC |
| Agda | Dependently-typed FP | ✓ | ✓ | ✓ | λΠω | |
| Lean 4 | Proof + programming | ✓ | ✓ | ✓ | λΠω | |
| Idris 2 | Dependently-typed GP | ✓ | ✓ | ✓ | λΠω | Quantitative type theory |
| F* | Verification | ✓ | ✓ | ✓ | λΠω | Refinement + effects |
| ATS | Systems | ✓ | ✓ | ✓ | λΠω | Dependent + linear types |

*Legend: ✓ = full support, ≈ = partial/approximated, — = absent*

---

## How OOP and FP Express the Same Axes Differently

The lambda cube is rooted in lambda calculus, but its axes are *structural* — they describe *what can depend on what*, not a particular syntax. Object-oriented and functional languages reach the same vertices through very different mechanisms:

### Polymorphism `(◻, ∗)`

| Mechanism | Tradition | Example |
|---|---|---|
| Parametric polymorphism (`∀a. a → a`) | FP | Haskell, ML |
| Generic classes (`class Box<T>`) | OOP | Java, C#, Kotlin |
| Generic interfaces + protocols | OOP hybrid | Swift, Rust traits |
| Subtype polymorphism (`Animal a = new Dog()`) | OOP | Java, C# (not in the lambda cube per se — it's a separate axis) |
| Duck typing / structural typing | Dynamic/structural | Go interfaces, TypeScript |

### Type operators `(◻, ◻)`

| Mechanism | Tradition | Example |
|---|---|---|
| Type families / type-level functions | FP | Haskell `type family`, Scala `match type` |
| Associated types in traits | FP + OOP | Rust, Swift, Scala |
| Template metaprogramming | Systems | C++ |
| Mapped / conditional types | Hybrid | TypeScript |
| Module functors | FP (ML family) | OCaml, SML |
| Abstract type members | OOP | Scala `type Elem` |

### Dependent types `(∗, ◻)`

| Mechanism | Tradition | Example |
|---|---|---|
| Full Π-types | FP / proof assistants | Agda, Idris, Lean, Coq |
| Const generics / value parameters | Systems | Rust `[T; N]`, C++ `template<int N>` |
| Literal types + conditional types | Hybrid | TypeScript |
| Promoted data constructors | FP | Haskell `DataKinds` |
| Singleton pattern | FP (encoding) | Haskell singletons library |
| Refinement types | Verification | F*, Liquid Haskell |

---

## Conclusion

The lambda cube provides a remarkably clean lens for understanding the design space of type systems, even though real languages are messy, multi-paradigm artifacts that rarely sit at a single vertex. The historical trajectory is clear: programming languages have been climbing the cube for decades.

- **1950s–1970s:** Most languages sit at **λ→** — monomorphic, procedural.
- **1970s–1990s:** ML, Haskell, and Java generics bring **λ2** (polymorphism) into the mainstream.
- **2000s–2020s:** Scala, Rust, TypeScript, and extended Haskell push into **System Fω** with increasingly powerful type-level programming.
- **2010s–present:** Idris, Lean 4, and F* make the **full lambda cube (λΠω)** viable for general-purpose programming, not just theorem proving.

The frontier today lies in making the upper corners of the cube — dependent types — ergonomic enough for everyday software engineering, while maintaining decidable type checking and good error messages. Languages like Rust (const generics), TypeScript (conditional types), and Scala 3 (match types) represent pragmatic compromises: they give programmers a *taste* of dependent types within a System Fω framework, without requiring a Ph.D. in type theory to use them.

---

## Further Reading

- Barendregt, H. (1991). *Introduction to Generalized Type Systems*. Journal of Functional Programming.
- Pierce, B. C. (2002). *Types and Programming Languages*. MIT Press.
- Norell, U. (2007). *Towards a practical programming language based on dependent type theory* (Agda).
- Brady, E. (2013). *Idris, a general-purpose dependently typed programming language*.
- de Moura, L. et al. (2015). *The Lean Theorem Prover*.
- Friedman, D. P. & Christiansen, D. T. (2018). *The Little Typer*. MIT Press.
