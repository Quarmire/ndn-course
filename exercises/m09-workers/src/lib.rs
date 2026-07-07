//! m09-workers — processes and threads: doing many things at once, on purpose.
//!
//! Read SPEC.md. Run the witness with `./course check m09-workers`.
//!
//! Before async hides it, you meet concurrency where the operating system does the
//! work: real OS threads, channels between them, panics that stay contained, and
//! whole child processes. Three functions, three tools:
//!
//!   * `parallel_sum` — scoped threads + a channel of partial results,
//!   * `run_jobs`     — one thread per job, with panics isolated by `join`,
//!   * `run_command`  — a child *process*, its stdout and exit code captured.
//!
//! The stubs compile; the tests are red until you fill them in.

/// Extract a human-readable message from a panic payload — what `JoinHandle::join`
/// hands back in its `Err`. Panics usually carry a `&str` or a `String`. (provided)
#[allow(dead_code)]
fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}

/// Sum `data` using up to `workers` threads. Each worker sums one chunk and sends
/// its partial total back over a channel; the main thread adds the partials up.
/// Assume `workers >= 1`. The result must equal `data.iter().sum()`.
pub fn parallel_sum(data: &[u64], workers: usize) -> u64 {
    use std::sync::mpsc;
    use std::thread;
    let chunk = data.len().div_ceil(workers).max(1);
    let (tx, rx) = mpsc::channel();
    thread::scope(|s| {
        for c in data.chunks(chunk) {
            let tx = tx.clone();
            s.spawn(move || {
                let partial: u64 = c.iter().sum();
                tx.send(partial).unwrap();
            });
        }
    });
    drop(tx);
    rx.iter().sum()
}

/// Run each job on its own thread. A job that PANICS must not take down the others
/// or the process: return `Ok(value)` for a job that returned normally, and
/// `Err(message)` for one that panicked. Results stay in the jobs' original order.
pub fn run_jobs(jobs: Vec<Box<dyn FnOnce() -> u64 + Send>>) -> Vec<Result<u64, String>> {
    use std::thread;
    let handles: Vec<_> = jobs.into_iter().map(thread::spawn).collect();
    handles
        .into_iter()
        .map(|h| h.join().map_err(panic_message))
        .collect()
}

/// Run an external `program` with `args` to completion, capturing its stdout and
/// exit code. `Ok((exit_code, stdout))`; `Err` if the program couldn't be launched
/// at all (e.g. it doesn't exist).
pub fn run_command(program: &str, args: &[&str]) -> std::io::Result<(Option<i32>, String)> {
    let output = std::process::Command::new(program).args(args).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    Ok((output.status.code(), stdout))
}
