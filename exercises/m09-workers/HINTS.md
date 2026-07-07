# Hints — m09-workers

Revealed one rung at a time by `./course hint m09-workers`.

## Hint 1 — parallel_sum: scope lets threads borrow

`std::thread::scope(|s| { ... })` runs a block in which `s.spawn(...)` starts threads
that are all joined before `scope` returns — so they may borrow `data`. Size a chunk
(`let chunk = data.len().div_ceil(workers).max(1);` — `max(1)` avoids `chunks(0)`
panicking on empty input), then `for c in data.chunks(chunk)` spawn a thread that
sums `c`.

## Hint 2 — parallel_sum: the channel

Make an `mpsc` channel before the scope: `let (tx, rx) = std::sync::mpsc::channel();`.
Inside each spawned thread, send the chunk's sum: clone the sender first
(`let tx = tx.clone();`) and `move` the clone in, so each thread owns one. After the
`scope` block (every thread has finished and dropped its clone), `drop(tx)` to close
the channel, then `rx.iter().sum()` collects every partial. Order doesn't matter for
a sum.

## Hint 3 — run_jobs: spawn all, then join all

Spawn every job first so they run concurrently, collecting the handles:
`let handles: Vec<_> = jobs.into_iter().map(std::thread::spawn).collect();` — a boxed
`FnOnce() -> u64 + Send` is exactly what `spawn` wants, so no wrapping closure is
needed. Then map each handle through `join`:
`.map(|h| h.join().map_err(panic_message))`. `join()` gives `Ok(v)` normally and
`Err(payload)` if the thread panicked; `map_err(panic_message)` turns the payload
into your `String`.

## Hint 4 — run_command

`std::process::Command::new(program).args(args).output()?` runs the child and waits,
returning an `Output`. `output.status.code()` is the `Option<i32>` exit code;
`String::from_utf8_lossy(&output.stdout).into_owned()` is the captured text. Wrap
them in `Ok((code, stdout))`. The `?` already handles the "couldn't launch" case by
returning the `io::Error`.
