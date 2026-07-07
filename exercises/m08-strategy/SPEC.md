# m08-strategy — traits, generics, closures: choosing the tool

**Module:** M8 · Traits, generics, closures · **Species:** template-assisted
**You write:** `Multicast::choose`, `BestRoute::choose`, `FnStrategy`'s impl, `forward_static`, `compare_strategies`, `default_strategy`.

One `Strategy` trait, and every dispatch tool Rust gives you, each with a real
reason to exist. The lesson isn't the strategies — they're a few lines each — it's
*which tool for which job*, and being able to defend the choice.

## The dispatch decision

- **Generic (`fn f<S: Strategy>(s: &S)`)** — the type is known at compile time. The
  compiler stamps out a specialized copy per `S` (*monomorphization*) and can inline
  the call. Fast, zero indirection; but the collection can only hold one concrete
  type, and code size grows per instantiation. This is `forward_static`.
- **Trait object (`&dyn Strategy`, `Box<dyn Strategy>`)** — the type is known only
  at run time. The call goes through a *vtable* (a pointer to the right `choose`).
  One pointer of indirection, no inlining across it — but you can hold *different*
  concrete strategies in one `Vec`, decide at run time, and keep code size flat.
  This is `compare_strategies`, and it's the thing generics simply cannot do.

Rule of thumb: reach for generics by default (they're free); reach for `dyn` when
you need heterogeneity or a run-time choice. **Machine strand:** paste
`forward_static::<Multicast>` and a `dyn` call into `cargo-show-asm` or the
[Compiler Explorer](https://godbolt.org) and see the vtable load appear — derived,
not memorized.

## Object safety (why `dyn Strategy` even compiles)

You can only make `dyn Strategy` because `Strategy` is *object-safe*: its methods
take `&self`, use no generic type parameters, and return `Sized` values — so a
vtable entry can exist for each. Add a generic method or return `impl Trait` and the
trait stops being object-safe and `Box<dyn Strategy>` stops compiling. The real
workspace has exactly such a trait: `Face` (in ndn-rs) returns futures via
return-position `impl Trait` and is deliberately **not** object-safe — so the engine
holds faces a different way. Read `../ndn-rs`'s `Face` and note what it gives up
(easy `Box<dyn Face>`) to gain what (borrowing, no boxed futures).

## Closures and the `Fn` family

`FnStrategy` stores a closure and calls it from `choose`. A closure is just a value;
the trait bound says how you may call it: **`Fn`** (borrows, callable many times —
what you need here), **`FnMut`** (mutably borrows, callable many times), **`FnOnce`**
(consumes, callable once). You want `Fn` because `choose(&self, …)` is called
repeatedly through a shared reference.

## Initialize-once (`OnceLock`)

`default_strategy` builds its value the first time it's asked and shares it forever
after, via a `static OnceLock<_>` and `get_or_init(|| …)` — the closure runs at most
once. (`LazyLock` is the same idea as a static you can use directly; `OnceCell` is
the single-threaded cousin.) The value here is cheap, but the pattern pays off when
first-time construction is expensive and you don't want to do it eagerly at startup.

## The fuzz/property lab

The witness includes a *property test*: it throws two thousand random
`(incoming, candidates)` pairs at your strategies and checks invariants — Multicast
never returns the incoming face and only returns candidates; BestRoute returns at
most one, and it's the first eligible. This is the fuzzing mindset — assert
*properties* over generated inputs, not just hand-picked examples. Read the
[proptest](https://proptest-rs.github.io/proptest/) and
[cargo-fuzz](https://rust-fuzz.github.io/book/) books; the seeded LCG here is the
same idea without the toolchain.

## Done means

`./course check m08-strategy` green: behavior tests, the heterogeneous `dyn` test,
and the property test, plus clippy `-D warnings` and fmt. Then `./course submit`.

## Rules of engagement

No `unsafe`. Don't add a generic method to `Strategy` (it would break object safety
and `compare_strategies`) — that constraint is part of the lesson. Afterward, read
the real `Strategy` and `PipelineStage` traits in ndn-rs and note one place they
choose `dyn` and one where they choose generics.
