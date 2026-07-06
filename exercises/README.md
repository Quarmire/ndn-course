# exercises/

One crate per exercise, named `mNN-slug` so lexical order is course order.
Every exercise is self-describing:

| File | Role |
|---|---|
| `SPEC.md` | The contract you build against. Read it fully before writing code. |
| `src/lib.rs` | Your code. Stubs compile (`todo!()`); the tests define done. |
| `tests/` | The witness suite — gate 1. Some exercises also have hidden witnesses run at review time. |
| `HINTS.md` | The Socratic ladder. Revealed one rung at a time via `./course hint`. Don't open it directly — climbing costs nothing but is recorded honestly. |

## Species

- **from-scratch** — spec + witnesses only (this is `m02-varint`)
- **fill-in-blank** — real code with holes and failing tests
- **template-assisted** — a scaffold plus a spec
- **refactor kata** — working-but-bad code to transform under constraints
- **signature-locked** — target signatures given; make them compile and pass
- **planted-bug hunt** — seeded bugs; each fix requires a written hypothesis log

## Workflow

```
./course next          # what's up
$EDITOR exercises/<n>  # build
./course check <n>     # gate 1: tests + clippy -D warnings + fmt
./course submit <n>    # gate 1 green → hands off to the tutor for gates 2–3
```
