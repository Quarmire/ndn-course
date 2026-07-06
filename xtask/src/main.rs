//! The `course` CLI — the student's interface to the course machinery.
//!
//! Subcommands: doctor · start · next · check · hint · submit · progress
//! Gate 1 (mechanical) lives here; gates 2–3 (rubric review, reflection)
//! happen with the tutor (see CLAUDE.md) after `submit` goes green.

use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask sits one level below the course root")
        .to_path_buf()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    let arg = args.get(1).cloned();
    let code = match cmd {
        "doctor" => cmd_doctor(),
        "start" => cmd_start(),
        "next" => cmd_next(),
        "check" => cmd_check(arg),
        "submit" => cmd_submit(arg),
        "hint" => cmd_hint(arg),
        "progress" => cmd_progress(),
        _ => {
            print_help();
            0
        }
    };
    std::process::exit(code);
}

fn print_help() {
    println!("course — the ndn-course CLI\n");
    println!("  ./course doctor            environment + pins diagnostic");
    println!("  ./course start             doctor, then point at the next exercise");
    println!("  ./course next              show the next unfinished exercise");
    println!("  ./course check [name]      gate 1: tests + clippy + fmt for one exercise");
    println!("  ./course hint [name]       reveal the next hint on the ladder");
    println!("  ./course submit [name]     run gate 1; on green, hand off to the tutor");
    println!("  ./course progress          summary of where you are");
}

// ---------------------------------------------------------------- exercises

fn exercises() -> Vec<String> {
    let dir = root().join("exercises");
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() && p.join("Cargo.toml").exists() {
                if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                    out.push(name.to_string());
                }
            }
        }
    }
    out.sort();
    out
}

fn next_exercise() -> Option<String> {
    let p = load_progress();
    exercises()
        .into_iter()
        .find(|e| p["exercises"][e.as_str()]["status"].as_str() != Some("done"))
}

fn resolve(arg: Option<String>) -> Option<String> {
    match arg {
        Some(name) => {
            if exercises().iter().any(|e| e == &name) {
                Some(name)
            } else {
                println!("no exercise named `{name}` under exercises/");
                None
            }
        }
        None => {
            let n = next_exercise();
            if n.is_none() {
                println!("all exercises done — nothing to resolve");
            }
            n
        }
    }
}

// ----------------------------------------------------------------- progress

fn progress_path() -> PathBuf {
    root().join("student").join("progress.json")
}

fn load_progress() -> Value {
    fs::read_to_string(progress_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| json!({ "version": 1, "exercises": {} }))
}

fn save_progress(v: &Value) {
    let _ = fs::create_dir_all(root().join("student"));
    if let Ok(s) = serde_json::to_string_pretty(v) {
        let _ = fs::write(progress_path(), s + "\n");
    }
}

// ------------------------------------------------------------------ helpers

fn run_in_root(program: &str, args: &[&str]) -> bool {
    println!("→ {program} {}", args.join(" "));
    Command::new(program)
        .args(args)
        .current_dir(root())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn version_of(program: &str) -> Option<String> {
    Command::new(program)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

// ------------------------------------------------------------------- doctor

fn cmd_doctor() -> i32 {
    println!("ndn-course doctor\n");
    let mut hard_fail = false;
    let mut warnings = 0u32;

    // toolchain
    for prog in ["rustc", "cargo"] {
        match version_of(prog) {
            Some(v) => println!("  ✓ {v}"),
            None => {
                println!("  ✗ {prog} not found — install Rust via https://rustup.rs");
                hard_fail = true;
            }
        }
    }

    // pinned sibling repos (parse pins.toml: [repos.NAME] … tag = "…")
    let pins = fs::read_to_string(root().join("pins.toml")).unwrap_or_default();
    let parent = root().parent().map(Path::to_path_buf);
    let mut name = String::new();
    for raw in pins.lines() {
        let line = raw.trim();
        if let Some(rest) = line.strip_prefix("[repos.") {
            name = rest.trim_end_matches(']').to_string();
        } else if line.starts_with("tag") && !name.is_empty() {
            let tag = line
                .splitn(2, '=')
                .nth(1)
                .unwrap_or("")
                .trim()
                .split('#')
                .next()
                .unwrap_or("")
                .trim()
                .trim_matches('"')
                .to_string();
            let exists = parent
                .as_ref()
                .map(|p| p.join(&name).is_dir())
                .unwrap_or(false);
            match (exists, tag == "UNPINNED") {
                (true, true) => {
                    println!("  ~ ../{name} present, but pin not yet cut (UNPINNED)");
                    warnings += 1;
                }
                (true, false) => println!("  ✓ ../{name} present (course pins {tag})"),
                (false, true) => {
                    println!("  ~ ../{name} missing and pin not yet cut — needed for forwarder labs");
                    warnings += 1;
                }
                (false, false) => {
                    println!("  ~ ../{name} missing — run setup.sh to clone it at {tag}");
                    warnings += 1;
                }
            }
            name.clear();
        }
    }

    // exercises + student state
    let n = exercises().len();
    if n == 0 {
        println!("  ✗ no exercises found under exercises/");
        hard_fail = true;
    } else {
        println!("  ✓ {n} exercise(s) available");
    }
    if progress_path().exists() {
        println!("  ✓ student/progress.json present");
    } else {
        println!("  ~ student/progress.json will be created on first use");
        warnings += 1;
    }

    println!();
    if hard_fail {
        println!("doctor: RED — fix the ✗ items above");
        1
    } else if warnings > 0 {
        println!("doctor: GREEN with {warnings} warning(s) — you can start");
        0
    } else {
        println!("doctor: GREEN");
        0
    }
}

// ---------------------------------------------------------------- start/next

fn cmd_start() -> i32 {
    let code = cmd_doctor();
    if code != 0 {
        return code;
    }
    println!();
    println!("Welcome. The loop is: read SPEC.md → build → ./course check → ./course submit");
    println!("→ tutor review (open Claude Code here and ask for a review) → reflection → next.");
    println!("Stuck? ./course hint — the ladder exists to be climbed.\n");
    cmd_next()
}

fn cmd_next() -> i32 {
    match next_exercise() {
        Some(e) => {
            println!("next exercise: {e}");
            println!("  spec:  exercises/{e}/SPEC.md");
            println!("  code:  exercises/{e}/src/lib.rs");
            println!("  check: ./course check {e}");
            0
        }
        None => {
            println!("all exercises complete — go build the capstone.");
            0
        }
    }
}

// -------------------------------------------------------------- check/submit

fn run_gate1(pkg: &str) -> bool {
    println!("== gate 1 (mechanical) for {pkg} ==");
    let mut ok = true;
    ok &= run_in_root("cargo", &["test", "-p", pkg]);
    ok &= run_in_root(
        "cargo",
        &["clippy", "-p", pkg, "--all-targets", "--", "-D", "warnings"],
    );
    ok &= run_in_root("cargo", &["fmt", "-p", pkg, "--", "--check"]);
    ok
}

fn cmd_check(arg: Option<String>) -> i32 {
    let Some(pkg) = resolve(arg) else { return 1 };
    if run_gate1(&pkg) {
        println!("\ngate 1: GREEN for {pkg}");
        0
    } else {
        println!("\ngate 1: RED for {pkg} — read the first failure top-down, then hypothesize");
        1
    }
}

fn cmd_submit(arg: Option<String>) -> i32 {
    let Some(pkg) = resolve(arg) else { return 1 };
    if !run_gate1(&pkg) {
        println!("\nsubmit blocked: gate 1 is red. Fix, then re-run ./course submit {pkg}");
        return 1;
    }
    let mut p = load_progress();
    p["exercises"][pkg.as_str()]["status"] = json!("awaiting_review");
    p["exercises"][pkg.as_str()]["mechanical_pass"] = json!(true);
    save_progress(&p);
    println!("\ngate 1: GREEN — recorded in student/progress.json");
    println!("gate 2: open Claude Code in this repo and say:");
    println!("    review my submission for {pkg} against .claude/rubrics/{pkg}.md");
    println!("gate 3: the tutor will follow with reflection questions.");
    0
}

// --------------------------------------------------------------------- hint

fn cmd_hint(arg: Option<String>) -> i32 {
    let Some(pkg) = resolve(arg) else { return 1 };
    let path = root().join("exercises").join(&pkg).join("HINTS.md");
    let Ok(text) = fs::read_to_string(&path) else {
        println!("no HINTS.md for {pkg}");
        return 1;
    };
    let sections: Vec<&str> = text.split("\n## ").skip(1).collect();
    let mut p = load_progress();
    let used = p["exercises"][pkg.as_str()]["hints_used"]
        .as_u64()
        .unwrap_or(0) as usize;
    if used >= sections.len() {
        println!("no more hints for {pkg} ({} used). Talk to the tutor — explaining is always allowed.", sections.len());
        return 0;
    }
    println!("## {}", sections[used].trim_end());
    p["exercises"][pkg.as_str()]["hints_used"] = json!((used + 1) as u64);
    save_progress(&p);
    println!("\n(hint {}/{} recorded)", used + 1, sections.len());
    0
}

// ----------------------------------------------------------------- progress

fn cmd_progress() -> i32 {
    let p = load_progress();
    println!("progress\n");
    for e in exercises() {
        let st = p["exercises"][e.as_str()]["status"]
            .as_str()
            .unwrap_or("not started");
        let hints = p["exercises"][e.as_str()]["hints_used"]
            .as_u64()
            .unwrap_or(0);
        let review = p["exercises"][e.as_str()]["review"].as_str().unwrap_or("—");
        println!("  {e:<24} {st:<16} hints: {hints}  review: {review}");
    }
    0
}
