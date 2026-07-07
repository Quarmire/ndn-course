# Rubric — m13-macros

Gate 2 review. The tests prove the macros generate the right code; grade whether the
student wields macros *judiciously* and understands what they expand to. Score each
0–2; pass = no zeros and ≥ 10/14. Comment quality and observability are standing.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Registry lookups | `name_of`/`value_of` are driven by `$( ... )*` over the pairs — no hand-written arm per type, no second copy of the table |
| 2 | Uses fragments correctly | `$value` stays in expression position, `stringify!($name)` produces the name string; no attempt to use `:expr` where a pattern is required |
| 3 | tlv! builder | Collects the value bytes, computes the length at expansion, emits `[type, length, ...value]`; the empty-value and trailing-comma cases work |
| 4 | Reads the expansion | The student used (or can describe) `cargo expand` to see the generated code, and can read a macro error as pointing at the expansion |
| 5 | Decision ladder | Can argue why a *macro* (not a function/generic/build.rs) is the right tool for the registry — it generates items a function can't — and where they'd stop short of one |
| 6 | Derive = proc-macro | Understands that `#[derive(...)]` is a procedural macro (a separate crate over `TokenStream`), a different mechanism from `macro_rules!` |
| 7 | Critique + comments | The journal critiques a real ndn-rs `macro_rules!` (what it generates, whether it earns its keep); comments explain the *why* where a maintainer would look |

## Reflection prompts (gate 3, pick 2–3)

- Your registry could also be a `const` array of `(&str, u64)` plus two functions
  that scan it. What does the macro version buy, and what does it cost a reader?
- Macros are hygienic — a name you introduce won't clash with the caller's. Sketch a
  bug that hygiene quietly prevents.
- `tlv!` computes the length at expansion. When would you *want* that at compile time
  instead (a `const`), and what would change about the macro?
