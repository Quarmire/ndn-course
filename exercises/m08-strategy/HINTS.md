# Hints — m08-strategy

Revealed one rung at a time by `./course hint m08-strategy`.

## Hint 1 — the two named strategies are iterator one-liners

`Multicast::choose`: keep the candidates that aren't the incoming face, in order —
`candidates.iter().copied().filter(|&f| f != incoming).collect()`. `BestRoute::choose`:
the *first* eligible one, as a `Vec` of zero or one — `candidates.iter().copied()
.find(|&f| f != incoming)` gives an `Option<FaceId>`; `.into_iter().collect()` turns
`Some(x)` into `vec![x]` and `None` into `vec![]`.

## Hint 2 — calling the stored closure

Inside `FnStrategy::choose`, the closure is the field `self.f`. Call it like a
function, but wrap the field access in parentheses so Rust doesn't think you mean a
method: `(self.f)(incoming, candidates)`. `name` just returns `self.label`. The
`Fn` bound on the impl is what makes `self.f(...)` legal through `&self`.

## Hint 3 — static vs dynamic

`forward_static` is deliberately trivial: `strategy.choose(incoming, candidates)`.
The teaching is entirely in the `<S: Strategy>` on the signature — one monomorphized
copy per `S`. `compare_strategies` iterates the boxed strategies and, for each,
collects a pair: `strategies.iter().map(|s| (s.name().to_string(), s.choose(incoming,
candidates))).collect()`. Each `s` is a `&Box<dyn Strategy>`; method calls auto-deref
through the box and dispatch via its vtable.

## Hint 4 — OnceLock

Declare the storage as a function-local `static` and initialize it once:

```rust
use std::sync::OnceLock;
pub fn default_strategy() -> &'static Multicast {
    static DEFAULT: OnceLock<Multicast> = OnceLock::new();
    DEFAULT.get_or_init(|| Multicast)
}
```

`get_or_init` runs its closure only on the first call and returns a `&'static`
reference (the static lives for the whole program). The `|| Multicast` is an
`FnOnce` — it needs to run at most once, and that's exactly what `OnceLock`
guarantees.
