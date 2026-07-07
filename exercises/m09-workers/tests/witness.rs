//! Witness suite for m09-workers. Threads must produce the sequential answer,
//! panics must stay contained and ordered, and the child process's output and exit
//! code must come back.

use m09_workers::{parallel_sum, run_command, run_jobs};

// ------------------------------------------------------------- parallel_sum

#[test]
fn parallel_sum_matches_sequential_at_every_worker_count() {
    let data: Vec<u64> = (0..1000).collect();
    let expected: u64 = data.iter().sum(); // 499500
    for workers in [1usize, 2, 3, 8, 64, 1000] {
        assert_eq!(parallel_sum(&data, workers), expected, "workers={workers}");
    }
}

#[test]
fn parallel_sum_handles_small_and_empty_inputs() {
    assert_eq!(parallel_sum(&[10, 20, 30], 8), 60); // fewer items than workers
    assert_eq!(parallel_sum(&[42], 1), 42);
    assert_eq!(parallel_sum(&[], 4), 0);
}

// ------------------------------------------------------------------ run_jobs

#[test]
fn run_jobs_isolates_panics_and_preserves_order() {
    let jobs: Vec<Box<dyn FnOnce() -> u64 + Send>> = vec![
        Box::new(|| 1),
        Box::new(|| panic!("boom")),
        Box::new(|| 20 + 22),
    ];
    let results = run_jobs(jobs);

    assert_eq!(results.len(), 3);
    assert_eq!(results[0], Ok(1));
    assert!(
        results[1].is_err(),
        "the panicking job must come back as Err"
    );
    assert!(
        results[1].as_ref().unwrap_err().contains("boom"),
        "the panic message should carry through, got {:?}",
        results[1]
    );
    assert_eq!(results[2], Ok(42));
}

#[test]
fn run_jobs_runs_all_when_none_panic() {
    let jobs: Vec<Box<dyn FnOnce() -> u64 + Send>> = (0..16u64)
        .map(|i| Box::new(move || i * i) as Box<_>)
        .collect();
    let results = run_jobs(jobs);
    let squares: Vec<Result<u64, String>> = (0..16u64).map(|i| Ok(i * i)).collect();
    assert_eq!(results, squares);
}

// --------------------------------------------------------------- run_command

#[test]
fn run_command_captures_stdout_and_exit_code() {
    // `/bin/sh -c 'printf hello'` writes exactly "hello" and exits 0.
    let (code, out) = run_command("/bin/sh", &["-c", "printf hello"]).expect("sh should launch");
    assert_eq!(code, Some(0));
    assert_eq!(out, "hello");

    // A non-zero exit code comes back too.
    let (code, _out) = run_command("/bin/sh", &["-c", "exit 3"]).expect("sh should launch");
    assert_eq!(code, Some(3));
}

#[test]
fn run_command_errors_when_the_program_is_missing() {
    let result = run_command("this-program-does-not-exist-9f3a2b", &[]);
    assert!(
        result.is_err(),
        "launching a missing program should be an Err"
    );
}
