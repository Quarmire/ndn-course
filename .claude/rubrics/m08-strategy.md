# Rubric — m08-strategy

Gate 2 review. The tests prove behavior; grade whether the student can *justify the
tool*. Score each 0–2; pass = no zeros and ≥ 10/14. Comment quality and
observability are standing criteria.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | The strategies | `Multicast`/`BestRoute` are clean iterator chains (`filter`/`find` + `collect`), not manual index loops; `BestRoute` returns 0-or-1 via `Option → into_iter` |
| 2 | Closure + Fn | `FnStrategy::choose` calls `(self.f)(...)` through the `Fn` bound; the student can say why it's `Fn` and not `FnMut`/`FnOnce` (repeated calls via `&self`) |
| 3 | Generics used right | `forward_static` is a clean generic; the student can explain monomorphization and when its code-size cost matters |
| 4 | `dyn` used right | `compare_strategies` genuinely relies on trait objects for heterogeneity; the student can name what generics couldn't do here |
| 5 | Object safety understood | Can state why `Strategy` is object-safe and what would break it (a generic method / `impl Trait` return), with the `Face` trait as the counter-example |
| 6 | OnceLock + comments | `default_strategy` uses `get_or_init` correctly; a comment notes the "runs once, shared" property and where it pays off; standing comment quality holds |
| 7 | Property test read | The student engaged with the fuzz test — can state an invariant it checks, and ideally added one of their own (e.g. an idempotence or subset property) |

## Reflection prompts (gate 3, pick 2–3)

- You used generics for `forward_static` and `dyn` for `compare_strategies`. Give a
  concrete third scenario and argue which you'd pick and why.
- `Strategy` is object-safe; the real `Face` trait is not (it returns `impl
  Future`). What does `Face` gain by giving up `Box<dyn Face>`, and how might an
  engine store faces instead?
- The property test found no bug because your code is right — but what invariant, if
  you'd gotten `BestRoute` subtly wrong, would it have caught that a hand-written
  example might have missed?
