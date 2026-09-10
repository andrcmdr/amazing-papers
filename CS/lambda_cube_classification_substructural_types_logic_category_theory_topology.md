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

## Part II — Substructural Type Systems

### What Are Structural Rules?

In Gentzen's sequent calculus, the **structural rules** govern how hypotheses (assumptions in a typing context Γ) may be manipulated *independently of their logical content*. A type system is **structural** if it admits all of them; it is **substructural** if it restricts one or more.

The classical structural rules are:

| Rule | Formal | Informal meaning | What restriction buys you |
|---|---|---|---|
| **Exchange** (E) | Γ, A, B, Δ ⊢ C  ⟹  Γ, B, A, Δ ⊢ C | The order of assumptions doesn't matter | Dropping it: *positional* / *ordered* reasoning |
| **Weakening** (W) | Γ ⊢ C  ⟹  Γ, A ⊢ C | You may ignore an assumption | Dropping it: every variable *must be used* |
| **Contraction** (C) | Γ, A, A ⊢ C  ⟹  Γ, A ⊢ C | You may use an assumption more than once | Dropping it: every variable used *at most once* |

Two additional structural properties arise in some formulations:

| Rule | Informal meaning |
|---|---|
| **Associativity** (A) | Grouping of context concatenation doesn't matter: (Γ, Δ), Σ ≡ Γ, (Δ, Σ) |
| **Identity / Cut** | An assumption of type A can be used where A is needed; a derivation can be composed with another |

### The Substructural Hierarchy

By systematically dropping combinations of Exchange, Weakening, and Contraction, we get a lattice of substructural logics, each corresponding to a type discipline:

```
                    Unrestricted (Structural)
                    E + W + C
                   ╱           ╲
                  ╱             ╲
          Relevant              Affine
          E + C                 E + W
          (no Weakening)        (no Contraction)
                  ╲             ╱
                   ╲           ╱
                    Linear
                    E only
                    (no W, no C)
                      │
                    Ordered
                    (no E, no W, no C)
```

| System | Exchange | Weakening | Contraction | Intuition |
|---|:---:|:---:|:---:|---|
| **Unrestricted** (normal) | ✓ | ✓ | ✓ | Use variables any number of times, in any order |
| **Relevant** | ✓ | — | ✓ | Every variable must be used *at least once* (but may be duplicated) |
| **Affine** | ✓ | ✓ | — | Every variable used *at most once* (may be discarded) |
| **Linear** | ✓ | — | — | Every variable used *exactly once* |
| **Ordered** | — | — | — | Every variable used exactly once, in order (non-commutative) |

Beyond these five core systems, richer disciplines combine substructural control with additional logical structure:

| Extended system | Based on | Additional feature |
|---|---|---|
| **Separation logic** | Affine / linear | Separating conjunction `∗` for heap reasoning; frame rule |
| **Bunched implications** (BI) | Linear + structural | Two conjunctions: multiplicative (linear) and additive (structural) |
| **Quantitative type theory** (QTT) | Linear | Usage annotations 0, 1, ω on each binding — subsumes linear, affine, and unrestricted |
| **Graded modal types** | Linear | A semiring of usage grades generalising QTT |
| **Uniqueness types** | Affine | Values guaranteed to have a single reference at the point of use |
| **Capability types** | Affine / linear | Track unforgeable tokens of authority |
| **Fractional permissions** | Linear | Permissions as rational numbers: full = write, fractional = read-only |
| **Ownership types** | Affine | Hierarchical containment of object references (OOP) |

---

### Substructural Type Systems in Programming Languages

#### Unrestricted (all structural rules) — the default

Most programming languages live here. Variables can be used zero or more times in any order.

| Language | Paradigm | Notes |
|---|---|---|
| C, C++ (without move) | Imperative / multi | Copy semantics by default |
| Java, C#, Kotlin | OOP | Reference semantics, GC handles lifetimes |
| Python, Ruby, JavaScript | Dynamic | No restrictions on usage |
| Haskell, OCaml, Elm | Functional | Lazy/eager, but fully structural — GC everywhere |
| Scala | FP + OOP | Unrestricted by default |
| Go | Imperative | GC, no ownership model |
| Fortran, COBOL, Pascal | Imperative | Classic unrestricted |

```haskell
-- Unrestricted: 'x' can be used 0, 1, or many times
f x = (x, x)      -- used twice (contraction)
g x = 42           -- used zero times (weakening)
h x = x            -- used once
-- All three are perfectly legal in Haskell
```

```java
// Unrestricted in Java
String s = "hello";
System.out.println(s);  // use 1
System.out.println(s);  // use 2 — contraction is free
// s could also never be used — weakening is free
```

#### Affine types — use at most once (Exchange + Weakening, no Contraction)

An affine type system allows you to *discard* a value (weakening) but not *duplicate* it (no contraction). This is the basis of ownership and move semantics.

**Rust** is the most prominent example. Its ownership system is fundamentally an affine type system embedded in a systems language.

| Language | Paradigm | Mechanism |
|---|---|---|
| **Rust** | Systems / multi | Ownership + move semantics; borrow checker |
| **Swift** (partial) | Multi-paradigm | Copy-on-write, move-only types (Swift 5.9+) |
| **C++** (partial) | Multi-paradigm | Move semantics (`std::move`), `[[nodiscard]]` — opt-in, not enforced by type system |
| **Val / Hylo** | Value-oriented | Mutable value semantics with ownership |
| **Austral** | Systems | Linear/affine types for safe resource management |
| **Koka** | Functional + effects | Effect handlers with owned/borrowed distinction |

```rust
// Affine types in Rust: a value can be moved (used once) or dropped (weakening)

fn consume(s: String) {
    println!("{}", s);
    // s is consumed (dropped) at end of scope
}

fn main() {
    let s = String::from("hello");
    consume(s);         // s is *moved* — used exactly once
    // println!("{}", s);  // COMPILE ERROR: s has been moved (no contraction)

    let t = String::from("world");
    // t is never used — that's okay (weakening is allowed)
    // It will be dropped at end of scope
    
    // Borrowing relaxes affinity temporarily:
    let u = String::from("borrowed");
    let r1 = &u;       // shared borrow — multiple readers allowed
    let r2 = &u;       // another shared borrow
    println!("{} {}", r1, r2);  // both used, u not moved
}
```

```rust
// Rust's ownership as affine resource management
use std::fs::File;
use std::io::Write;

fn write_and_close(mut file: File) {  // takes ownership (affine move)
    file.write_all(b"data").unwrap();
    // file is closed (dropped) here — resource management via affinity
}

fn main() {
    let f = File::create("out.txt").unwrap();
    write_and_close(f);    // f moved
    // f.write(b"more");   // COMPILE ERROR: f was moved
}
```

```swift
// Swift 5.9+ — move-only (noncopyable) types
struct FileHandle: ~Copyable {
    let fd: Int32
    
    consuming func close() {
        // After calling close(), the value is consumed
    }
    
    deinit {
        // Automatic cleanup if dropped without explicit close
    }
}

func process(_ handle: consuming FileHandle) {
    // handle is consumed here — cannot be used after this function
    handle.close()
}
```

#### Linear types — use exactly once (Exchange only, no Weakening, no Contraction)

Linear types are stricter than affine: you *must* use every value exactly once. You cannot silently discard a resource — you must explicitly consume it. This guarantees that every allocated resource is properly handled.

| Language | Paradigm | Mechanism |
|---|---|---|
| **Linear Haskell** (GHC 9.0+) | Functional | `%1 ->` (linear arrow) via `-XLinearTypes` |
| **Idris 2** | Dependently-typed | Quantitative type theory with multiplicity 0, 1, ω |
| **Clean** | Functional | Uniqueness types (related to linear types) |
| **ATS** | Systems | Linear types + dependent types |
| **Mercury** | Logic/functional | Uniqueness and determinism modes |
| **Granule** | Research | Full graded modal type system |
| **Austral** | Systems | Linear types as the default |

```haskell
-- Linear Haskell (GHC 9.0+)
{-# LANGUAGE LinearTypes #-}

-- The linear arrow  %1 ->  means: the argument must be used exactly once
dup :: a %1 -> (a, a)    -- ILLEGAL: would need to use 'a' twice

-- A legal linear function
swap :: (a, b) %1 -> (b, a)
swap (x, y) = (y, x)       -- each component used exactly once ✓

-- Linear file I/O: the token must be threaded through
openFile  :: FilePath -> IOL (Handle %1)
readLine  :: Handle %1 -> IOL (Handle %1, String)
closeFile :: Handle %1 -> IOL ()

-- Usage: the handle *must* be closed — you can't drop it
processFile :: FilePath -> IOL String
processFile path = do
    h <- openFile path
    (h', line) <- readLine h     -- h consumed, h' is new token
    closeFile h'                  -- h' consumed — resource freed ✓
    pure line
    -- Forgetting to call closeFile would be a type error
```

```idris
-- Idris 2 — Quantitative Type Theory (QTT)
-- Usage annotations: 0 = erased, 1 = linear, Unrestricted = unlimited

-- A linear function: the argument of type a is used exactly once
linearId : (1 x : a) -> a
linearId x = x

-- Erased argument: exists only at compile time, cannot be used at runtime
length : (0 n : Nat) -> Vect n a -> Nat
length {n} _ = n     -- n is erased, only its type-level info is used

-- A protocol enforced by linearity
data DoorState = Opened | Closed

data Door : DoorState -> Type where
  MkDoor : (1 _ : ()) -> Door s

openDoor  : (1 _ : Door Closed) -> Door Opened
closeDoor : (1 _ : Door Opened) -> Door Closed
knockDoor : (1 _ : Door Closed) -> Door Closed

-- The linear usage ensures the door is always in a valid state
-- and its handle is never duplicated or leaked
```

```
-- ATS: linear types for manual memory management without a GC
// A linear view: a proof that memory at location l holds a value of type T
viewdef T @ l = T @ l   // @ is the "at" viewtype

// A function that takes a linear proof of memory ownership
fn {a:t@ype} ptr_get
  {l:addr} (pf: a @ l | p: ptr l): (a @ l | a)
  // Takes proof of ownership, returns it along with the value
  // The proof must be returned — it cannot be dropped (no weakening)
```

#### Ordered types — no Exchange, no Weakening, no Contraction

Ordered (non-commutative) type systems are the most restrictive: values must be used exactly once and in the exact order they appear in the context. These are mostly found in research settings, modelling things like stack-based protocols or non-commutative resources.

| Language / System | Notes |
|---|---|
| **Ordered linear logic** (research) | Lambek calculus; models natural language syntax |
| **Stack-based languages** (conceptually) | Forth, Factor — operand order matters, values consumed from stack |
| **Session types** (ordered fragment) | Communication protocols where message order is enforced |

```factor
! Factor (stack-based, concatenative) — operationally ordered
! Values are consumed from the stack in order; no implicit reordering
5 3 -    ! pushes 5, pushes 3, subtracts: result is 2
! You cannot access 5 after 3 has been pushed without explicit stack shuffling
! This is *operationally* ordered, though Factor doesn't have a formal ordered type system
```

#### Separation logic — substructural reasoning about the heap

Separation logic extends Hoare logic with a **separating conjunction** `P ∗ Q` meaning "P holds for one part of the heap and Q holds for a *disjoint* part." The **frame rule** allows local reasoning: if a function only touches its own footprint, the rest of the heap is automatically preserved.

| Language / Tool | Application |
|---|---|
| **Rust** (borrow checker) | The borrow checker enforces separation of mutable references — essentially a lightweight separation logic |
| **Dafny** | Verification language with implicit dynamic frames (related to separation logic) |
| **Viper** | Verification infrastructure using separation logic with fractional permissions |
| **VeriFast** | C/Java verifier based on separation logic |
| **Iris** (Coq library) | Higher-order concurrent separation logic framework |
| **Steel** (F*) | Effectful programming with concurrent separation logic |
| **Mezzo** | Research language with separation and ownership |

```rust
// Rust's borrow checker as separation logic in action
fn main() {
    let mut v = vec![1, 2, 3];

    let r1 = &v[0];      // shared borrow of v[0]
    // let r2 = &mut v;   // COMPILE ERROR: cannot borrow v mutably
                          // while a shared borrow exists
                          // This enforces SEPARATION: the mutable and immutable
                          // views cannot coexist

    println!("{}", r1);   // r1's lifetime ends here
    
    let r2 = &mut v;      // now a mutable borrow is allowed
    r2.push(4);           // exclusive access — safe mutation
}

// Conceptually: &v and &mut v correspond to fractional permissions
//   &v   = read permission (can be shared: P ∗ P is fine for reads)
//   &mut v = full write permission (exclusive: no sharing)
```

```dafny
// Dafny — implicit dynamic frames (separation logic variant)
class Cell {
    var value: int

    method Increment()
        modifies this           // declares the "footprint" — separation!
        ensures value == old(value) + 1
    {
        value := value + 1;
    }
}

method Swap(a: Cell, b: Cell)
    requires a != b             // separation condition: disjoint cells
    modifies a, b
    ensures a.value == old(b.value) && b.value == old(a.value)
{
    var tmp := a.value;
    a.value := b.value;
    b.value := tmp;
}
```

#### Uniqueness types — a different angle on linearity

Uniqueness types (pioneered by Clean) guarantee that a value has **a single reference** at the point of use. Unlike linear types (which track how many times a binding is *used*), uniqueness types track how many *references* exist. If a value is unique, it can be updated in place — enabling purely functional languages to perform destructive updates safely.

| Language | Mechanism |
|---|---|
| **Clean** | `*` annotation: `*World -> *World` means the world is unique |
| **Idris 2** | Combines uniqueness with QTT multiplicities |
| **SAC** (Single Assignment C) | With-loop and uniqueness for array updates |

```clean
// Clean — uniqueness types
// *File means the file handle is unique — exactly one reference exists
readAndClose :: *File -> (String, *File)
readAndClose file
    # (line, file) = freadline file   // destructive read — safe because unique
    # file = fclose file              // file consumed
    = (line, file)

// *World threading for I/O — similar to Haskell's IO monad but via uniqueness
Start :: *World -> *World
Start world
    # (console, world) = stdio world
    # console = fwrites "Hello\n" console
    # (_, world) = fclose console world
    = world
```

---

### Comprehensive Language–Substructure Classification

| Language | Paradigm | Base discipline | E | W | C | Substructural features |
|---|---|---|:---:|:---:|:---:|---|
| **C** | Procedural | Unrestricted | ✓ | ✓ | ✓ | None; manual memory management |
| **C++** (modern) | Multi | Unrestricted + affine opt-in | ✓ | ✓ | ~| `std::move`, `unique_ptr` (affine-like) |
| **Java** | OOP | Unrestricted | ✓ | ✓ | ✓ | GC; `AutoCloseable` is a convention, not a type-level guarantee |
| **C#** | OOP | Unrestricted | ✓ | ✓ | ✓ | `IDisposable`, `using` — convention-based |
| **Python** | Dynamic | Unrestricted | ✓ | ✓ | ✓ | Context managers (`with`) — runtime only |
| **Go** | Imperative | Unrestricted | ✓ | ✓ | ✓ | GC; `defer` for cleanup — not type-enforced |
| **Haskell** (base) | Functional | Unrestricted | ✓ | ✓ | ✓ | GC; purity handles most resource issues |
| **Haskell** (LinearTypes) | Functional | Linear (opt-in) | ✓ | — | — | `%1 ->` linear arrows; `%m ->` multiplicity polymorphism |
| **Rust** | Systems | **Affine** | ✓ | ✓ | — | Ownership, move semantics, borrow checker, lifetimes |
| **Swift** | Multi | Unrestricted + affine opt-in | ✓ | ✓ | ~| `~Copyable` (noncopyable types), consuming parameters |
| **Scala** | FP + OOP | Unrestricted | ✓ | ✓ | ✓ | No substructural types; relies on GC |
| **OCaml** | Functional | Unrestricted + modal (5.0) | ✓ | ✓ | ✓ | OCaml 5 has modes for locality and uniqueness |
| **Kotlin** | Multi | Unrestricted | ✓ | ✓ | ✓ | Coroutine-structured concurrency (convention) |
| **TypeScript** | Multi | Unrestricted | ✓ | ✓ | ✓ | No substructural features |
| **Idris 2** | Dep. typed | **QTT** (0, 1, ω) | ✓ | per-binding | per-binding | Full quantitative type theory |
| **Clean** | Functional | **Uniqueness** | ✓ | — | — | `*` uniqueness annotations |
| **ATS** | Systems | **Linear** | ✓ | — | — | Linear views and viewtypes |
| **Austral** | Systems | **Linear** (default) | ✓ | — | — | Linear types as the default for resource types |
| **Granule** | Research | **Graded modal** | ✓ | graded | graded | Full graded linear type system with semiring coefficients |
| **F*** | Verification | **Separation** | ✓ | — | — | Steel library: concurrent separation logic |
| **Mezzo** | Research | **Separation** | ✓ | — | — | Ownership + adoption/abandon |
| **Coq** | Proof assistant | Unrestricted | ✓ | ✓ | ✓ | Iris library adds separation logic |

---

## Part III — Mapping the Lambda Cube to Substructural Type Systems

### Orthogonality of the two classifications

The lambda cube and the substructural hierarchy classify type systems along **orthogonal dimensions**:

- The **lambda cube** answers: *what can abstract over what?* (terms↔types dependency structure)
- **Substructural types** answer: *how many times and in what order can a resource be used?* (structural rules on the typing context)

These two dimensions are *independent* — in principle, any vertex of the lambda cube can be combined with any substructural discipline:

```
                 Substructural axis
                 (resource control)
                 
    Ordered ──── Linear ──── Affine ──── Unrestricted
        │            │           │             │
        │    ┌───────┼───────────┼─────────────┼──────────┐
        │    │  λ→   │           │             │          │
        │    │       │    ATS    │    Rust      │  C       │  ← Lambda cube
        │    │  λ2   │   (lin+  │   (affine+   │  Java    │     axis
        │    │       │    dep)  │    poly+     │  ML      │   (abstraction)
        │    │  Fω   │          │    tyop)     │  Haskell │
        │    │       │          │             │  Scala   │
        │    │  CoC  │   Idris2 │             │  Agda    │
        │    │       │  (QTT+   │             │  Coq     │
        │    │       │   dep)   │             │  Lean    │
        │    └───────┼───────────┼─────────────┼──────────┘
        │            │           │             │
```

### The product space: Cube × Substructure

We can describe any type system as a point in the product **Cube × Substructure**:

| Language | Lambda cube vertex | Substructural discipline | Coordinates |
|---|---|---|---|
| C | λ→ | Unrestricted | (→, EWCA) |
| Pascal | λ→ | Unrestricted | (→, EWCA) |
| Java (5+) | λ2 | Unrestricted | (2, EWCA) |
| Haskell 98 | λ2 | Unrestricted | (2, EWCA) |
| Go 1.18+ | λ2 | Unrestricted | (2, EWCA) |
| OCaml (core) | λ2 | Unrestricted | (2, EWCA) |
| C# | λ2 | Unrestricted | (2, EWCA) |
| Elm | λ2 | Unrestricted | (2, EWCA) |
| Haskell + extensions | Fω (→ CoC) | Unrestricted | (Fω, EWCA) |
| Haskell + LinearTypes | Fω (→ CoC) | Linear (opt-in) | (Fω, E) |
| Scala 3 | Fω | Unrestricted | (Fω, EWCA) |
| **Rust** | **Fω** | **Affine** | **(Fω, EW)** |
| C++ (modern) | Fω | Unrestricted + affine opt-in | (Fω, EWCA / EW) |
| TypeScript | Fω | Unrestricted | (Fω, EWCA) |
| Swift | λ2–Fω | Unrestricted + affine opt-in | (2–Fω, EWCA / EW) |
| Clean | λ2 | Uniqueness (≈ linear) | (2, E) |
| ATS | λΠω | Linear | (Πω, E) |
| **Idris 2** | **λΠω** | **QTT (0, 1, ω)** | **(Πω, graded)** |
| Agda | λΠω | Unrestricted | (Πω, EWCA) |
| Coq | λΠω | Unrestricted (+Iris) | (Πω, EWCA / sep) |
| Lean 4 | λΠω | Unrestricted | (Πω, EWCA) |
| F* | λΠω | Separation (Steel) | (Πω, sep) |
| Granule | λ2 | Graded modal | (2, graded) |
| Austral | λ2 | Linear | (2, E) |

### How substructural control interacts with each axis of the cube

#### Polymorphism `(◻, ∗)` and substructure

When you introduce polymorphism, a natural question arises: **does a polymorphic function `∀a. a → a` promise to be linear in its argument?** In an unrestricted system, no — the polymorphic identity `id x = x` could also be written as `const () x = ()` (discarding `x`). In a linear system, the type `a ⊸ a` (linear arrow) guarantees that the argument is used exactly once.

This interaction produces **multiplicity polymorphism**:

```haskell
-- GHC's multiplicity-polymorphic identity
-- 'p' is a multiplicity variable: can be 1 (linear) or Many (unrestricted)
id :: forall (p :: Multiplicity) a. a %p -> a
id x = x

-- When instantiated at p=1: id :: a %1 -> a   (linear)
-- When instantiated at p=Many: id :: a -> a    (unrestricted)
```

```idris
-- Idris 2: the multiplicity is part of the Π-type
-- (0 a : Type) means 'a' is erased; (1 x : a) means 'x' is linear
the : (0 a : Type) -> (1 x : a) -> a
the _ x = x
```

#### Dependent types `(∗, ◻)` and substructure

The combination of dependent types and substructural control is where the most interesting interactions occur. If a *type* depends on a *value*, and that value is linearly typed, then the type itself is entangled with a resource:

```idris
-- Idris 2: a dependent type indexed by a linear state
data DoorState = Opened | Closed

-- The type of the door depends on its state (a term),
-- and the door handle is linear (used exactly once)
data Door : DoorState -> Type where
  MkDoor : (1 tag : ()) -> Door s

-- Type-safe state transitions with linear resource control
openDoor  : (1 _ : Door Closed) -> Door Opened
closeDoor : (1 _ : Door Opened) -> Door Closed

-- The combination guarantees:
-- 1. State correctness (dependent types: ∗ → ◻)
-- 2. No handle leaks (linearity: no weakening)
-- 3. No aliasing (linearity: no contraction)
```

This combination — `(∗, ◻)` dependent types with linear resources — is the theoretical foundation of **session types** for safe concurrent communication:

```
-- Conceptual session type (as in Idris 2 or GV)
-- A protocol where the TYPE of the next message depends on the VALUE sent
data Protocol : Type where
  Send    : (a : Type) -> (a -> Protocol) -> Protocol  -- dependent on value sent
  Receive : (a : Type) -> (a -> Protocol) -> Protocol
  Done    : Protocol

-- A channel carrying this protocol is LINEAR:
-- you must follow the protocol exactly, using the channel exactly once per step
```

#### Type operators `(◻, ◻)` and substructure

Type operators interact with substructure through **graded modalities**. A graded type operator `□_r A` annotates a type with a *grade* `r` from a semiring, indicating how the value may be used:

```
-- Conceptual (Granule-style):
-- □_0 A    = A at grade 0: erased, cannot be used at runtime
-- □_1 A    = A at grade 1: used exactly once (linear)
-- □_ω A    = A at grade ω: used without restriction
-- □_(2) A  = A at grade 2: used exactly twice

-- The grading is itself a type-level operation (◻, ◻):
-- □ : Semiring → Type → Type
```

```granule
-- Granule: a research language with graded modal types
-- The grade (usage count) is a type-level annotation

id : a [1] -> a         -- use the boxed value exactly once
id [x] = x

dup : a [2] -> (a, a)   -- use the boxed value exactly twice
dup [x] = (x, x)

drop : a [0] -> ()       -- use the boxed value zero times
drop [x] = ()
```

---

## Part IV — Category Theory, Algebraic Topology, and Type Systems

This chapter traces the deep mathematical correspondences between type systems (lambda cube + substructural), category theory, and algebraic topology. These are not mere analogies — they are *formal equivalences* established by decades of research.

### The Curry–Howard–Lambek Correspondence

The starting point is the three-way correspondence first observed by Curry and Howard and extended by Lambek:

| Logic | Type theory | Category theory |
|---|---|---|
| Propositions | Types | Objects |
| Proofs | Terms (programs) | Morphisms |
| Implication A → B | Function type A → B | Exponential object B^A |
| Conjunction A ∧ B | Product type (A, B) | Categorical product A × B |
| Disjunction A ∨ B | Sum type A + B | Coproduct A + B |
| Truth ⊤ | Unit type () | Terminal object 1 |
| Falsity ⊥ | Empty type Void | Initial object 0 |
| Universal ∀x.P(x) | Dependent product Π(x:A).B(x) | Right adjoint to pullback |
| Existential ∃x.P(x) | Dependent sum Σ(x:A).B(x) | Left adjoint to pullback |
| Cut elimination | Computation (β-reduction) | Composition of morphisms |

### The Lambda Cube in Category Theory

Each vertex of the lambda cube corresponds to a specific categorical structure:

#### λ→ : Cartesian Closed Categories (CCCs)

The simply-typed lambda calculus corresponds to the internal language of a **Cartesian closed category** — a category with finite products and exponential objects.

```
A CCC has:
  - A terminal object 1              ↔  Unit type
  - Binary products  A × B           ↔  Pair types (A, B)
  - Exponentials     B^A             ↔  Function types A → B

The key equations:
  Hom(A × B, C) ≅ Hom(A, C^B)      ↔  Currying: (A, B) → C  ≅  A → (B → C)
```

| Programming concept | CCC concept |
|---|---|
| Function application | Evaluation morphism `eval : B^A × A → B` |
| Lambda abstraction | Currying: transposing `A × B → C` to `A → C^B` |
| Tuple construction | Product pairing `⟨f, g⟩ : C → A × B` |
| Pattern matching | Product projections `π₁ : A × B → A` |

```haskell
-- Haskell is the canonical "CCC language"
-- Currying is the CCC adjunction:
curry   :: ((a, b) -> c) -> (a -> b -> c)
uncurry :: (a -> b -> c) -> ((a, b) -> c)

-- These form a natural isomorphism:
-- curry . uncurry = id
-- uncurry . curry = id
```

#### λ2 (System F): Polymorphism as natural transformations

In System F, a polymorphic function `∀a. F(a) → G(a)` is a **natural transformation** between the functors `F` and `G`. The parametricity theorem (Wadler's "theorems for free") is precisely the naturality condition.

```
A natural transformation  η : F ⟹ G  satisfies:
  For every morphism f : A → B:
    G(f) ∘ η_A  =  η_B ∘ F(f)

In Haskell:
  A polymorphic function  η :: forall a. F a -> G a  satisfies:
    fmap f . η  =  η . fmap f

This is exactly the "free theorem" for η.
```

```haskell
-- Example: reverse :: forall a. [a] -> [a]
-- is a natural transformation from the List functor to itself

-- The free theorem says:
--   map f . reverse  =  reverse . map f
-- i.e. reversing commutes with mapping — guaranteed by parametricity

-- head :: forall a. [a] -> a
-- is a natural transformation from List to Id
-- Free theorem: f . head = head . map f  (for non-empty lists)
```

The categorical model of System F is a **polymorphic fibration** or an **internal language of a topos** with sufficient universe structure.

#### Fω: Higher-kinded types as 2-categories

Type operators (kind `Type → Type`) introduce a second level of structure. The categorical semantics moves to **enriched** or **2-categorical** settings:

```
Level 0:  Values (terms)           ↔  Morphisms
Level 1:  Types                    ↔  Objects
Level 2:  Type constructors        ↔  Functors (morphisms between objects in a 2-category)
Level 3:  Higher-kinded abstraction ↔  Natural transformations between functors
```

```haskell
-- Functor in Haskell IS a functor in category theory
class Functor f where           -- f : Type → Type is a functor
    fmap :: (a -> b) -> f a -> f b  -- the morphism-mapping part

-- A natural transformation between functors
type Nat f g = forall a. f a -> g a

-- Example: maybeToList :: forall a. Maybe a -> [a]
-- is a natural transformation  Maybe ⟹ List

-- The Yoneda lemma in Haskell:
-- forall b. (a -> b) -> f b  ≅  f a
-- This is a deep categorical identity, directly expressible as a type
```

```scala
// Scala: higher-kinded types make the categorical structure explicit
trait Functor[F[_]] {
  def map[A, B](fa: F[A])(f: A => B): F[B]
}

// Natural transformation as a trait
trait ~>[F[_], G[_]] {
  def apply[A](fa: F[A]): G[A]
}

// The Yoneda embedding
trait Yoneda[F[_], A] {
  def run[B](f: A => B): F[B]
}
// Yoneda[F, A] ≅ F[A]  — the Yoneda lemma
```

#### λΠω (CoC): Locally Cartesian Closed Categories and ∞-Topoi

The Calculus of Constructions (and dependent type theory more generally) corresponds to the internal language of a **locally Cartesian closed category** (LCCC). An LCCC is a category where every slice category C/A is itself Cartesian closed.

```
Dependent types in categorical terms:

  Context extension    Γ, x:A          ↔  Morphism  p : E → Γ   (a bundle over Γ)
  Dependent product    Π(x:A).B(x)     ↔  Right adjoint Π_p to pullback p*
  Dependent sum        Σ(x:A).B(x)     ↔  Left adjoint Σ_p to pullback p*

  The adjunction chain:   Σ_p  ⊣  p*  ⊣  Π_p
  
  This is the categorical core of dependent type theory.
```

The **univalence axiom** (Voevodsky, 2006) — central to Homotopy Type Theory — states that the identity type `A ≡ B` is equivalent to the type of equivalences `A ≃ B`. Categorically, this means types form an **∞-groupoid**, not just a set.

### Substructural Type Systems in Category Theory

The categorical semantics of substructural types departs from Cartesian closed categories. The key insight: Cartesian categories have *diagonal* and *projection* morphisms for free, which correspond to contraction and weakening. Removing these structural rules means moving to non-Cartesian monoidal categories.

| Type system | Structural rules | Categorical model |
|---|---|---|
| **Unrestricted** (structural) | E + W + C | Cartesian closed category (CCC) |
| **Relevant** | E + C (no W) | Relevance monoidal category (no projections, but diagonal exists) |
| **Affine** | E + W (no C) | Symmetric monoidal closed category with projections (no diagonal) |
| **Linear** | E only | Symmetric monoidal closed category (SMCC) — **not** Cartesian |
| **Ordered** | none | Monoidal closed category (non-symmetric) — **not** even symmetric |

The crucial distinction:

```
Cartesian monoidal category:
  Diagonal    Δ : A → A ⊗ A    exists naturally     ↔  Contraction (duplication)
  Projection  ! : A → I         exists naturally     ↔  Weakening (discarding)
  Symmetry    σ : A ⊗ B → B ⊗ A exists naturally    ↔  Exchange

Symmetric monoidal closed category (LINEAR):
  NO Δ (no natural diagonal)                         ↔  No contraction
  NO ! (no natural projection to unit)               ↔  No weakening
  Symmetry σ still exists                            ↔  Exchange still holds

  Instead of Cartesian product ×, we have tensor product ⊗
  Instead of exponential B^A, we have linear function space A ⊸ B
  
  The key adjunction:
    Hom(A ⊗ B, C) ≅ Hom(A, B ⊸ C)     ↔  Linear currying
```

#### Linear logic's categorical structure

Girard's linear logic (1987) has a rich categorical interpretation. The key connectives map to:

| Linear logic | Type theory | Category theory |
|---|---|---|
| A ⊗ B (tensor) | Linear pair: both must be used | Tensor product in SMCC |
| A ⊸ B (linear implication) | Linear function: use argument exactly once | Internal hom in SMCC |
| A & B (with / additive conjunction) | Choice: use one of A or B | Categorical product (Cartesian) |
| A ⊕ B (plus / additive disjunction) | Tagged union | Coproduct |
| !A (of course / exponential) | Unlimited use of A | Comonad (the ! modality) |
| ?A (why not) | Unused supply of A | Monad (dual of !) |

The **exponential modality** `!` is the bridge between linear and unrestricted worlds:

```
! : SMCC → CCC
!A represents "as many copies of A as you want"

In categorical terms, ! is a comonad satisfying:
  !A → A           (dereliction: use once)
  !A → !A ⊗ !A     (contraction: duplicate)
  !A → I            (weakening: discard)

These are exactly the structural rules, repackaged as explicit operations.
```

```haskell
-- Haskell encoding of linear logic's ! modality
-- In Linear Haskell, the ! comonad is called 'Ur' (unrestricted)
data Ur a where
  Ur :: a -> Ur a   -- wraps a value as unrestricted

-- A linear function can use Ur a unrestrictedly:
linearFn :: Ur Int %1 -> (Int, Int)
linearFn (Ur x) = (x, x)    -- x extracted from Ur can be used multiple times

-- Without Ur, this would be illegal:
-- illegalDup :: Int %1 -> (Int, Int)
-- illegalDup x = (x, x)    -- ERROR: x used twice in linear context
```

#### The resource interpretation

The categorical model illuminates *why* substructural types correspond to resource management:

```
In a CCC (unrestricted):
  A value of type A is like INFORMATION — can be copied and discarded freely
  
In a SMCC (linear):
  A value of type A is like a PHYSICAL RESOURCE — must be consumed exactly once
  The tensor product A ⊗ B means "having both A and B simultaneously"
  The linear function A ⊸ B means "consuming A to produce B"
  
In Rust's affine system:
  A value is like a RESOURCE WITH A DESTRUCTOR
  Moving  = consuming the resource (use once)
  Dropping = invoking the destructor (weakening is allowed — resources may be discarded)
  No copying = no contraction (resources can't be cloned without explicit .clone())
  &T borrows = the ! modality restricted to a lifetime scope
```

### Algebraic Topology and Homotopy Type Theory

The deepest connection between type systems and mathematics comes from **Homotopy Type Theory** (HoTT), which interprets types as *spaces* and terms as *points* in those spaces, with the identity type corresponding to *paths*.

#### The homotopical interpretation

| Type theory | Algebraic topology |
|---|---|
| Type A | Space (homotopy type / ∞-groupoid) |
| Term a : A | Point in the space A |
| Identity type a =_A b | Path space: space of paths from a to b |
| Higher identity p =_{a=b} q | Homotopy between paths (2-path) |
| Dependent type B(x) over A | Fibration p : E → B |
| Dependent product Π(x:A).B(x) | Space of sections of a fibration |
| Dependent sum Σ(x:A).B(x) | Total space of a fibration |
| Function type A → B | Mapping space Map(A, B) |
| Coproduct A + B | Disjoint union of spaces |
| Unit type 1 | Contractible space (a point) |
| Empty type 0 | Empty space |
| n-Type (truncation level n) | n-truncated space (π_k = 0 for k > n) |
| Set (0-type) | Discrete space (set) |
| Proposition ((-1)-type) | Truth value (empty or contractible) |
| Groupoid (1-type) | 1-groupoid (classical groupoid) |
| Universe U | Classifying space (moduli space of small types) |
| Univalence: (A ≃ B) ≃ (A =_U B) | The universe is a classifying space for fibrations |

#### The truncation hierarchy and the n-types

```
n-truncation level    Type theory              Topology                  Example
─────────────────────────────────────────────────────────────────────────────
    -2                Contractible type        Contractible space        Unit
    -1                Proposition (Bool-like)  (-1)-connected: ∅ or •    Bool, Prop
     0                Set (discrete)           Discrete space            Nat, String
     1                Groupoid                 1-type (π_k=0, k>1)      Type of groups
     2                2-Groupoid               2-type                   Type of categories
     ...              ...                      ...                      ...
     ∞                General type             General ∞-groupoid       Universe U
```

```agda
-- Agda (with --cubical): HoTT constructions

-- The circle S¹ as a higher inductive type
data S¹ : Type where
  base : S¹
  loop : base ≡ base   -- a PATH from base to itself (π₁(S¹) ≠ 0)

-- The fundamental group of S¹ is ℤ:
-- π₁(S¹) ≅ ℤ
-- This is a THEOREM provable within HoTT

-- Transport: a path in the base space lifts to a map between fibers
-- This is the type-theoretic version of the path lifting property
transport : {A : Type} {B : A → Type} {x y : A} → x ≡ y → B x → B y
```

```lean
-- Lean 4: Quotient types correspond to attaching cells
-- (a mild form of higher inductive types)

-- A quotient collapses paths: identifying points = adding paths between them
def ZMod (n : Nat) := Fin n   -- ℤ/nℤ as a quotient of ℤ

-- The quotient construction corresponds topologically to:
-- taking a space and gluing points together
-- i.e. forming a CW complex by attaching 1-cells
```

#### Cohomology and type theory

The cohomology of a space can be defined internally in HoTT:

```
Eilenberg–MacLane space K(G, n):
  - A type with πₙ = G and πₖ = 0 for k ≠ n
  - Ordinary cohomology: Hⁿ(X; G) ≅ ‖X → K(G, n)‖₀
  
  In Agda/HoTT: cohomology groups are DEFINABLE as types.
  The cohomology ring structure comes from the smash product.
```

#### The substructural dimension in topology

The linear / substructural dimension also has a topological interpretation, though it is less well-established than the HoTT correspondence:

| Substructural concept | Topological analogue |
|---|---|
| Linear type A | Cobordism: a "one-use manifold" connecting boundaries |
| Tensor product A ⊗ B | Disjoint union of cobordisms (monoidal, not Cartesian) |
| Linear function A ⊸ B | Cobordism from boundary A to boundary B |
| ! modality (exponential) | Free commutative comonoid: allows arbitrary branching |
| Affine type | Cobordism with a "cap" (can be capped off / discarded) |
| Ordered type | Directed path / non-reversible process |
| Separation logic `∗` | Spatial tensor: disjoint regions of a manifold |

The connection deepens through **topological quantum field theories** (TQFTs):

```
A (1+1)-dimensional TQFT is a symmetric monoidal functor:
  Z : Cob₂ → Vect
  
where Cob₂ is the category of:
  - Objects: 1-manifolds (circles)
  - Morphisms: 2-dimensional cobordisms (surfaces connecting circles)
  
This is fundamentally LINEAR:
  - A circle (type) can be "used" only by connecting it via a surface
  - The pair-of-pants cobordism gives multiplication (not free duplication!)
  - The disk cobordism gives unit/counit (but is not freely available)
  
In linear type theory terms:
  A TQFT witnesses the passage from a linear to a traced monoidal structure.
```

### Grand Synthesis: The Three-Dimensional Classification

Combining all three dimensions — the lambda cube, substructural rules, and categorical/topological models — we arrive at a three-dimensional classification space:

```
Axis 1: Lambda Cube (abstraction power)
  λ→  →  λ2  →  Fω  →  CoC
  
Axis 2: Substructural discipline (resource control)
  Ordered  →  Linear  →  Affine  →  Unrestricted
  
Axis 3: Categorical / topological semantics
  SMCC  →  CCC  →  LCCC  →  ∞-Topos
```

| Language | Cube | Substructure | Categorical semantics |
|---|---|---|---|
| **C** | λ→ | Unrestricted | CCC (trivially — products + exponentials) |
| **Java** | λ2 | Unrestricted | CCC + parametric functors (polymorphism) |
| **Haskell** | Fω | Unrestricted | CCC + enriched structure (higher-kinded) |
| **Rust** | Fω | Affine | SMCC with weakening + functorial polymorphism |
| **Linear Haskell** | Fω | Linear (opt-in) | SMCC ↔ CCC bridge via ! comonad |
| **Scala 3** | Fω | Unrestricted | CCC + path-dependent objects ≈ slices |
| **Idris 2** | CoC | QTT (graded) | Graded LCCC (parametrised monoidal structure) |
| **Agda (cubical)** | CoC | Unrestricted | ∞-Topos (Kan complexes) |
| **ATS** | CoC | Linear | Linearly-typed LCCC |
| **Coq** | CoC | Unrestricted | LCCC / Topos (CIC) |
| **Coq + Iris** | CoC | Separation | Step-indexed separation algebra over LCCC |

### Summary of correspondences

```
┌─────────────────────┬─────────────────────┬──────────────────────┬──────────────────────┐
│  Type Theory        │  Logic              │  Category Theory     │  Topology            │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  λ→ (simple types)  │  Propositional      │  CCC                │  Discrete spaces     │
│                     │  intuitionistic     │                      │                      │
│                     │  logic              │                      │                      │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  λ2 (System F)      │  Second-order       │  Polymorphic         │  Parametric          │
│                     │  propositional      │  fibrations          │  families of spaces  │
│                     │  logic              │                      │                      │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  Fω                 │  Higher-order       │  2-categories,       │  Fibred categories   │
│                     │  propositional      │  enriched categories │  of spaces           │
│                     │  logic              │                      │                      │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  CoC / MLTT         │  Higher-order       │  LCCC / Topos        │  ∞-Groupoids,        │
│                     │  predicate logic    │  (presheaf models)   │  Kan complexes       │
│                     │                     │                      │                      │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  CoC + Univalence   │  HoTT              │  (∞,1)-Topos         │  Homotopy types,     │
│                     │                     │  (Lurie)             │  CW complexes        │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  Linear types       │  Linear logic       │  SMCC (not           │  Cobordisms,         │
│                     │  (Girard)           │  Cartesian)          │  TQFTs               │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  Affine types       │  Affine logic       │  SMCC with           │  Cobordisms with     │
│                     │                     │  weakening           │  caps                │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  Ordered types      │  Non-commutative    │  Monoidal closed     │  Directed spaces,    │
│                     │  (Lambek calculus)  │  (non-symmetric)     │  directed homotopy   │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  Separation logic   │  Bunched            │  BI-hyperdoctrine    │  Sheaves over        │
│                     │  implications (BI)  │  (Day convolution)   │  heap models         │
├─────────────────────┼─────────────────────┼──────────────────────┼──────────────────────┤
│  Graded types (QTT) │  Graded linear      │  Graded (para-       │  Graded modalities   │
│                     │  logic              │  metric) comonad     │  as fibred structure │
└─────────────────────┴─────────────────────┴──────────────────────┴──────────────────────┘
```

---

## Further Reading

### Lambda Cube and Type Systems
- Barendregt, H. (1991). *Introduction to Generalized Type Systems*. Journal of Functional Programming.
- Pierce, B. C. (2002). *Types and Programming Languages*. MIT Press.
- Norell, U. (2007). *Towards a practical programming language based on dependent type theory* (Agda).
- Brady, E. (2013). *Idris, a general-purpose dependently typed programming language*.
- de Moura, L. et al. (2015). *The Lean Theorem Prover*.
- Friedman, D. P. & Christiansen, D. T. (2018). *The Little Typer*. MIT Press.

### Substructural Type Systems
- Girard, J.-Y. (1987). *Linear Logic*. Theoretical Computer Science.
- Walker, D. (2005). *Substructural Type Systems*. In Advanced Topics in Types and Programming Languages.
- Wadler, P. (1990). *Linear Types Can Change the World!*
- Bernardy, J.-P. et al. (2018). *Linear Haskell: practical linearity in a higher-order polymorphic language*. POPL.
- Atkey, R. (2018). *Syntax and Semantics of Quantitative Type Theory*. LICS.
- Tov, J. A. & Pucella, R. (2011). *Practical Affine Types*. POPL.
- Reynolds, J. C. (2002). *Separation Logic: A Logic for Shared Mutable Data Structures*. LICS.
- O'Hearn, P. & Pym, D. (1999). *The Logic of Bunched Implications*. Bulletin of Symbolic Logic.

### Category Theory and Type Theory
- Lambek, J. & Scott, P. J. (1986). *Introduction to Higher-Order Categorical Logic*. Cambridge University Press.
- Jacobs, B. (1999). *Categorical Logic and Type Theory*. Elsevier.
- Abramsky, S. & Tzevelekos, N. (2011). *Introduction to Categories and Categorical Logic*.
- Melliès, P.-A. (2009). *Categorical Semantics of Linear Logic*. Panoramas et Synthèses.
- Shulman, M. (2008). *Set Theory for Category Theory*. arXiv.

### Homotopy Type Theory and Algebraic Topology
- The Univalent Foundations Program. (2013). *Homotopy Type Theory: Univalent Foundations of Mathematics*. Institute for Advanced Study.
- Voevodsky, V. (2006). *A very short note on homotopy λ-calculus*. Unpublished.
- Lurie, J. (2009). *Higher Topos Theory*. Princeton University Press.
- Rijke, E. (2022). *Introduction to Homotopy Type Theory*. arXiv.
- Riehl, E. & Shulman, M. (2017). *A type theory for synthetic ∞-categories*. Higher Structures.

### Intersection: Linear Logic, Categories, and Topology
- Baez, J. C. & Stay, M. (2011). *Physics, Topology, Logic and Computation: A Rosetta Stone*. In New Structures for Physics, Springer.
- Atiyah, M. (1988). *Topological quantum field theories*. Publications Mathématiques de l'IHÉS.
- Mellies, P.-A. & Zeilberger, N. (2015). *Functors are Type Refinement Systems*. POPL.
