# Rubric — m09-workers

Gate 2 review. The tests prove it works; grade whether the student understands *why*
the concurrency is safe. Score each 0–2; pass = no zeros and ≥ 10/14. Comment
quality and observability are standing criteria.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Really parallel, really correct | `parallel_sum` genuinely spawns threads over chunks (not a disguised `iter().sum()`); correct at 1, 64, and > len workers, and on empty input (no `chunks(0)` panic) |
| 2 | Scope + channel, understood | Uses `thread::scope` so threads borrow `data`, and an `mpsc` channel with a cloned `Sender` per worker; can say why `spawn` alone would force `'static` |
| 3 | Panic isolation | `run_jobs` maps `join`'s `Err` to `Err(message)`; the student can explain that the *process* survived, and where that matters in a server |
| 4 | Order preserved | Results come back in job order (spawn-all-then-join, not join-as-you-spawn) |
| 5 | Process handling | `run_command` captures both stdout and exit code, propagates launch failure with `?`; the student can say how a child process differs from a thread |
| 6 | Comments + the orchestration journal | The journal reflects a real `ndn-fwd` child run — names the timing and cleanup (zombie) gotchas, not a paraphrase; comments capture the borrow/isolation "why" |
| 7 | Machine-strand honesty | Measured where more threads stop helping; can state the context-switch cost, and when to reach for rayon vs hand-rolled threads |

## Reflection prompts (gate 3, pick 2–3)

- `parallel_sum` with 1000 workers on 1000 items is slower than with 4. Where does
  the time go, and what does that tell you about picking a worker count?
- A job panicked and the others survived. If instead a job had entered an infinite
  loop, would `join` still protect you? What tool would you need then?
- A thread and a child process both isolate a crash. Name one thing a child process
  gives you that a thread can't, and one cost you pay for it.
