# NDN/NDF Applied Rust Course — Module-by-Module Syllabus

**Status:** v0.2 draft for review · **Substrate:** ndn-workspace (pinned tags) + NDF refounding (ndf-core et al. as they land) · **Tutor:** LLM agent via course plugin
**v0.2 delta:** dedicated debugging module (M5) + the debugging covenant; dedicated threads & processes module (M9); comment etiquette, clean code, and logging as taught-and-graded craft; public landing page (§4.6).

---

## 1. Course identity

Take a developer who knows Rust syntax through The Book ch. 6 — and has *heard of* the rest without being able to wield it — to the point where they can (a) contribute production-quality code to ndn-workspace and NDF, (b) hold their own on any large Rust codebase, (c) make and defend architectural decisions with a written plan and a methodical path to implementation, and (d) diagnose and fix the bugs they will inevitably meet, in code deliberately written to make that diagnosis easy.

**Non-goals:** teaching Rust in the abstract (everything is anchored to real code); covering every repo (depth over breadth; electives cover the spokes); replacing The Book (it is linked as reference, never assigned as a standalone read).

## 2. Student profile and gap map

Baseline: Rust Book ch. 1–6 concepts (ownership basics, structs, enums, `match`). Has *conceptual familiarity* with the rest but weak applied muscle. Named weak areas and where the course attacks each:

| Weak area | Primary modules | Method |
|---|---|---|
| Reader/writer patterns | M3 | Build a mini-TLV reader/writer from scratch, then study `ndn-tlv`'s `TlvReader`/`TlvWriter` as the professional version of what they just built |
| What to make `pub` / API design | M6, M12 | Design a public API, get it reviewed against the Rust API Guidelines; visibility strategy (`pub(crate)`, re-exports, sealed modules) |
| Structuring and growing a workspace | M1, M12 | ndn-workspace's scope buckets (`spec`/`extension`/`tooling`/`draft`) and dependency-direction rule taught as a living case study; student adds a crate under those rules |
| Lifetimes/ownership beyond trivial examples | M4, M7 | Refactor katas (clone-ectomy, owned→borrowed), signature-locked exercises, zero-copy parsing with real lifetime-carrying structs |
| Debugging skills and mindset | M5, then every module | A dedicated module installs the tools and the covenant (§below); the bug journal makes diagnosis a graded, lifelong habit |
| Comment etiquette · clean code · logging | M5, M11 | Taught together as one discipline — code that explains itself at rest (comments, names) and at runtime (logs, spans); a standing review criterion from M5 onward |
| Threads and processes | M9 | OS-level mastery *before* async: `std::thread`, scoped threads, channels, rayon, `std::process::Command`, daemons and IPC — with `ndn-fwd` itself as the running specimen |
| When to use `Arc`, `Rc`, `OnceCell`/`OnceLock`, `FnOnce`-family closures | M8, M10 | A decision tree built in two passes, each choice backed by a measured or asm-inspected consequence, then located in the real codebase (PIT's `DashMap`, `bytes::Bytes` refcounting) |
| Writing macros | M13 | `macro_rules!` for a genuine use case (TLV type-number tables, test-corpus builders); "when *not* to macro" is half the module |
| Memory concepts | M4, M10 + machine strand | Stack vs heap measured, not described; allocation cost benchmarks; cache lines and false sharing labs |
| Bit-level manipulation | M2, M3 | varint encode/decode by hand; endianness; TLV wire format as the running motivation |
| Hardware / internals / assembly | Machine strand, every module | See §3 — a persistent strand, not a one-off module |

**Pedagogical spine 1 — the toy → real → constraint loop.** Every major concept appears at least three times: (1) the student builds a toy version, (2) the student finds and reads the production version in the workspace, (3) the student uses it under constraint (refactor, extend, or fix). This is the deliberate answer to "knows the concept, can't use the muscle" — recognition is not recall, and recall is not fluency.

**Pedagogical spine 2 — the debugging covenant.** Two commitments, held simultaneously and both graded: *write the best code you can* (prevention: types that make illegal states unrepresentable, assertions, clippy, tests, honest comments) **and** *assume you will debug it anyway* (diagnosis: readable logs, hypothesis-driven investigation, tooling fluency). Debugging is a *when*, not an *if* — so prevention is never an excuse to skip diagnosability, and diagnosability is never an excuse to write sloppy code. The **bug journal** (started in M5) is the covenant's ledger: every real bug gets an entry — symptom, hypothesis trail, fix, lesson — and journal quality is reviewed at every checkpoint from M5 on.

## 3. The four strands

Every module braids four strands rather than teaching them in separate blocks:

1. **Rust** — the language topic of the module, always motivated by code the student is about to touch.
2. **Domain (NDN/NDF)** — Interests/Data, names, trust, Blocks. Interleaved so domain confusion is never misdiagnosed as Rust confusion.
3. **Machine** — the "under the hood" segment: how hardware and the compiler support the code just written. Compiler Explorer (godbolt.org) and `cargo-show-asm` sessions, memory-hierarchy labs, measured benchmarks. The goal is *derivation over memorization*: a student who knows why monomorphization produces the asm it does can re-derive the generics-vs-dyn tradeoff instead of reciting it.
4. **Craft** — the professional habits: testing, **debugging discipline and the bug journal**, **comment etiquette and clean code** (comments say *why*, names carry meaning, functions stay small), **logging best practice** (levels, structure, boundaries — culminating in `tracing` spans), honesty culture (`EXPECTED_FAILURES`-style known-issues files), commit hygiene, design docs, decision notes.

**Machine-strand resource backbone** (assigned in slices, never whole): *Putting the "You" in CPU* (cpu.land) early; *Computer Systems: A Programmer's Perspective* (CS:APP) selected chapters for memory hierarchy, machine-level representation, and **processes**; Ulrich Drepper's *What Every Programmer Should Know About Memory* (selected sections); Mara Bos, *Rust Atomics and Locks* (free online) for M10; Compiler Explorer as a standing lab tool from M2 onward. NAND2Tetris is offered as an elective for students who want the full stack from logic gates up.

## 4. Delivery architecture

### 4.1 The repo and the minutes-to-running contract

A single public repo (working name `ndn-course`) that anyone can find, clone, and be *running packets* from within minutes. This is enforced, not aspired to:

- **README top = three commands.** `git clone …` → `./setup.sh` → `course start`. Nothing above the fold but those and a one-paragraph pitch.
- **`setup.sh` / `course doctor`** — checks rustup, installs the pinned toolchain (`rust-toolchain.toml`), shallow-clones the pinned ndn-workspace tags as siblings, builds (or fetches prebuilt) `ndn-fwd` + `ndn-tools`, and runs a green/red diagnostic. Prebuilt binaries offered so a slow laptop still reaches "first packet" fast; the full build happens in the background or on demand.
- **Time-to-first-green CI.** A scheduled job runs the quick start in a fresh container and fails if setup-to-doctor-green exceeds the budget. The onboarding promise is a tested invariant, exactly like any other witness.

### 4.2 Content surface (not mdBook-first)

Course text is authored as markdown/MDX + assets in the repo (the durable source of truth), but **rendered through a visually rich static framework** — Astro Starlight or equivalent — rather than mdBook. That buys: embedded Compiler Explorer panes, inline quizzes that hand off to the tutor, proper diagrams and callouts, per-module progress indicators, and a non-monotonic visual identity. mdBook is demoted to at most an optional export target for offline reading.

A tasteful dogfooding option: a **Dioxus-built course site** (the ecosystem already uses Dioxus for `ndn-dashboard`). Recommended path: ship v1 on Starlight for speed; list "port the course site to Dioxus" as a curated Capstone-B ticket — a student-built course surface is a fitting graduation artifact.

### 4.3 Exercises and the `course` CLI

- **`exercises/`** — a cargo workspace, one crate per exercise, pinned by path/tag to the workspace snapshots. Three exercise species matching the difficulty gradient: **fill-in-blank** (code with holes + failing tests), **template-assisted** (scaffold + spec), **from-scratch** (spec + hidden witness suite only). Three more species for muscle-building: **refactor katas** (working-but-bad code to transform under constraints), **signature-locked** exercises (target function signatures given; make them compile and pass — the single best drill for lifetime fluency), and **planted-bug hunts** (vendored code seeded with real bug classes; find, hypothesize, fix, journal).
- **`course` CLI** (an xtask): `next`, `check` (test/clippy/fmt), `hint` (escalating, Socratic), `submit` (mechanical gate → LLM rubric review), `progress`, `doctor`.

### 4.4 Tutor, grading, memory

- **Tutor**: a `.claude/` course plugin — persona, Socratic hint policy (hints escalate; answers never precede submission), per-assignment rubrics.
- **Three grading gates** on every graded artifact: (1) *mechanical* — provided tests + clippy + fmt, written in the workspace's own audit-witness style (exits 1 broken, 0 fixed); (2) *rubric review* — the LLM evaluates idiom, design, docs, **comment quality, and log quality** against the assignment rubric; (3) *reflection* — the student explains choices; the tutor probes with follow-ups. From-scratch assessments and capstones add a viva.
- **Memory**: a structured progress manifest (JSON) + free-form journal (including the bug journal), committed in the student's fork — local-first and git-versioned by construction; PMB-compatible as an optional enhancement later.
- **AI-use policy**: the tutor is always available for review and hints; generation is off-limits for from-scratch assessments until after first submission. Late course (M12+) flips this deliberately — working *with* an agent on a large codebase becomes a graded skill.

### 4.5 Pinning and content rot

Each course release pins to named tags of every workspace repo (ndn-rs's pinnable v0.1.0 tag — the refounding's own "prerequisite zero" — is likewise prerequisite zero here). A scheduled content-rot CI job rebuilds all exercises against the pins and produces a delta report whenever the course rebases onto newer tags. NDF modules track the refounding: concepts (vault/spec) are stable teaching material now; `ndf-core` code exercises activate as the new repos land, with the frozen legacy `ndf-rs` conformance vectors as the grading oracle either way.

### 4.6 The front door: a public landing page

A single page whose job is to show a prospective student exactly what they will get — the outcomes, the journey, the artifacts they'll build, and the three-command quick start — before they've read a single module. It doubles as the index of the course site and must obey the same minutes-to-running promise it advertises. A working prototype (`course-landing.html`) ships alongside this syllabus; its content is drawn from §§1, 5, and 7, so syllabus revisions flow to the page.

## 5. The gradient at a glance

| Phase | Modules | Rust load | Dominant exercise species | Checkpoint identity |
|---|---|---|---|---|
| 0 — Orientation | M0–M1 | ch. 1–7 refresh | observation, scavenger hunt | "I can run it and find things" |
| 1 — Bytes, memory, bugs | M2–M5 | ch. 8–9 + ownership muscle + debugging | from-scratch (small), fill-in-blank, katas, bug hunts | "I can build, test, and debug a codec" |
| 2 — Types that carry meaning | M6–M8 | ch. 10, 13 in anger | template-assisted, signature-locked, fuzzing | "I can design an API and prove a parser" |
| 3 — Threads, processes, async | M9–M12 | ch. 15–16 + OS + async (beyond The Book) | template → from-scratch, witness authoring | "I can add a real component" (junior-contributor checkpoint) |
| 4 — Architecture & NDF | M13–M15 | consolidation + macros | design docs, oracle-graded implementation | "I can decide and defend" |
| Capstones | A, B | everything | proposal-gated projects | "I am a contributor" |

Rough pacing at ~10 h/week: modules run 1–2 weeks each; capstones 3–5 weeks each; ~7 months end to end. Self-paced; the gates, not the calendar, are the authority.

---

## 6. Modules

Each module lists its four strands, exercises (typed), readings (repo / vault / external — external readings are curated launch points, not required cover-to-cover), and its gate.

### Phase 0 — Orientation

#### M0 · Zero to running
**Goal:** environment green and first packets observed in one sitting.
**Rust:** none written; toolchain, cargo, `rust-toolchain.toml`, what `target/` is.
**Domain:** first contact — start `ndn-fwd` from the example TOML; `ndn-ping`, `ndn-peek`, `ndn-put` against it; watch the logs; optionally open the dashboard.
**Machine:** what a process is; memory layout (text/data/stack/heap) inspected live on the running forwarder; what's inside a binary (`file`, a glance at section headers).
**Craft:** how the course works — journal, progress manifest, the personal known-issues file (the honesty habit, day one).
**Exercises:** observation lab with guided questions ("what happened when the second peek hit the cache?"); journal entry 1.
**Readings:** course quick start; cpu.land (opening chapters); workspace README.
**Gate:** `course doctor` green; observation questions answered.

#### M1 · Reading a big codebase
**Goal:** navigate a ~60-crate, multi-repo workspace without drowning; acquire the NDN mental model.
**Rust:** modules and paths, `use`/re-exports (visibility from the *reader's* side), cargo workspace anatomy, sibling-repo path deps.
**Domain:** Interests and Data; names, not hosts. Trace one Interest conceptually through the pipeline (`FaceCheck → TlvDecode → CsLookup → PitCheck → Strategy → Dispatch`) with the code open.
**Machine (light):** what `cargo build` actually does; compilation units; incremental artifacts.
**Craft:** tests-as-documentation; `cargo doc --open`; grep discipline; drawing the crate dependency graph; **reading logs as a navigation tool** (RUST_LOG filters on the running forwarder — logging taught from the consumer's side first).
**Exercises:** scavenger hunt (find where a PIT entry is created; where Data gets cached and why `ctx.verified` gates it; answers as file:line + one-paragraph explanation, tutor-graded); annotate the pipeline diagram in your own words.
**Readings:** `ndn-rs/ARCHITECTURE.md`; vault `why-ndn`; matklad, *Large Rust Workspaces*.
**Gate:** scavenger hunt complete; a one-page "how a packet moves" explainer.

### Phase 1 — Bytes, memory, and bugs

#### M2 · Bits, bytes, numbers
**Goal:** bit-level manipulation stops being scary.
**Rust:** integer types and casting, bit ops, shifts and masks, endianness, arrays vs slices, `u8` buffers by hand.
**Domain:** why wire formats exist; TLV as a contract; the varint (`varu64`) as NDN's length encoding.
**Machine:** two's complement; registers; **first Compiler Explorer session** — watch your own varint code compile; shifts vs multiplies.
**Craft:** property tests as specification (the tests are provided this time; the student reads them as a spec before writing code).
**Exercises:** from-scratch (small): implement `varu64` encode/decode against provided property tests; then diff your version against `ndn-tlv`'s and write three observations.
**Readings:** Sean Anderson, *Bit Twiddling Hacks* (as a reference shelf, not a read); cpu.land continued; Rust Book ch. 3 linked for gaps.
**Gate:** property tests green; a short asm reflection ("one thing the compiler did that surprised me").

#### M3 · The reader/writer pattern: mini-TLV
**Goal:** the course's first real crate — a working TLV codec — and the reader/writer pattern installed as muscle memory.
**Rust:** cursor-over-`&[u8]` reader structs (position tracking, bounds discipline), writer into `Vec<u8>`/`&mut [u8]`; error handling *design* — error enums, `thiserror`, `Result` plumbing, when panicking is correct; first pass at "what to make `pub`."
**Domain:** NDN TLV specifics (type/length/value, nesting); the odd/even critical-bit convention as a forward-compatibility design lesson.
**Machine:** a slice is a pointer + length; what bounds checks cost and when the compiler elides them (godbolt evidence); iterator loop vs index loop asm.
**Craft:** unit-test habits; rustdoc examples that compile; first-pass comment etiquette (doc comments on the public surface — refined into a graded criterion in M5).
**Exercises:** (1) from-scratch `mini-tlv` against a written spec + hidden witness suite; (2) fill-in-blank holes in a vendored `TlvReader` subset; (3) compare-and-critique: your design vs `ndn-tlv`'s — what did they do differently and *why might that be*.
**Readings:** `std::io::Read`/`Write` docs; BurntSushi on error handling (or Rust Book ch. 9 as reference); `ndn-tlv` source.
**Gate:** witness suite green; critique reviewed by tutor.

#### M4 · Ownership and memory beyond trivial
**Goal:** the ownership/borrowing muscle, built through repetition under constraint — and the memory model those rules protect.
**Rust:** refactor katas (clone-ectomy: remove every `clone` from deliberately clone-heavy code, justifying each survivor); `Box`, `Vec` internals, `String` vs `&str`, `Drop`; lifetimes that matter — a struct holding `&[u8]`; signature-locked drills.
**Domain:** why `ndn-tlv`/`ndn-packet` are `no_std`; what `alloc` means; embedded foreshadowing.
**Machine:** stack vs heap *measured* — a criterion benchmark of allocate-per-call vs buffer reuse; cache lines; an optional heap-profiler lab (dhat/heaptrack).
**Craft:** benchmarks as evidence; "justify the clone" comments as a code-review norm.
**Exercises:** clone-ectomy kata; signature-locked lifetime drills (make the given signatures compile); build a borrowed `TlvView<'a>` over your mini-tlv.
**Readings:** *Learn Rust With Entirely Too Many Linked Lists* (selected); fasterthanlime on ownership/lifetimes; CS:APP memory-hierarchy chapter (selected sections); Rust Book ch. 10.3 as reference.
**Gate:** katas pass with zero unjustified clones; `TlvView` compiles with the intended signatures and passes tests.

#### M5 · Debugging, and code that debugs itself
**Goal:** the debugging covenant installed — tools, mindset, and the prevention habits that make bugs rare *and* shallow. This module also formalizes comment etiquette, clean code, and logging as one discipline: code that explains itself at rest (names, comments) and at runtime (logs).
**Rust — prevention half:** making illegal states unrepresentable (newtypes, enums over booleans, typestate taste); `assert!`/`debug_assert!` and stating invariants; `#[must_use]`; exhaustive `match` as a bug net; clippy as a teacher (read the lint's *why*; never silence without a comment); `Debug` impls worth reading; `dbg!` used well and removed after.
**Rust — diagnosis half:** reading a backtrace (`RUST_BACKTRACE`); `rust-gdb`/`rust-lldb` (or CodeLLDB) on a real crash — breakpoints, frames, variable inspection; `git bisect` as a search algorithm; test-case minimization; a first `miri` run on planted UB (foreshadows unsafe, later); the scientific method of debugging — hypothesis → experiment → conclusion, written down.
**Clean code & comments:** comments say *why*, never narrate *what*; doc comments vs inline comments and the job of each; comment smells (commented-out code, apologies, drift from the code below); naming as the first documentation; small functions with one job. The workspace's own conventions — honest notices, `EXPECTED_FAILURES.md` — studied as exemplars of truthful commentary at project scale.
**Logging:** the level ladder (error/warn/info/debug/trace) and what belongs on each rung; log state transitions, decisions, and boundary inputs — never secrets, never per-packet spam on hot paths; structured fields over string soup; `log` + `env_logger` now, `tracing` spans completed in M11. Lab: add three *good* log lines to your mini-tlv and defend each.
**Machine:** what a panic actually is (unwind vs abort); what a debugger does (ptrace, symbols, a DWARF one-pager) — the debugger stops being magic.
**Domain:** RUST_LOG spelunking on `ndn-fwd`; find the log line that proves a cache hit.
**Exercises:** **planted-bug hunt** — a vendored crate seeded with distinct bug classes (logic slip, off-by-one, misuse of a borrow, a panic-hiding `unwrap`); each fix requires a written hypothesis log; **git bisect lab** on a prepared history; **retrofit kata** — take a working but silent, uncommented module to review quality (comments + logs + one assertion), graded on restraint as much as coverage; **bug journal initialized** — from this module until the end of the course, every real bug gets an entry.
**Readings:** David Agans, *Debugging: The 9 Indispensable Rules* (selected); a curated comment-etiquette essay; `tracing`/`log` docs; a Julia Evans debugging zine.
**Gate:** all planted bugs found with hypothesis logs; retrofit kata passes rubric review; bug journal has its first entries. From here on, comment quality and log quality are standing rubric criteria in gate 2 of every module.

### Phase 2 — Types that carry meaning

#### M6 · Names and API design
**Goal:** design a public API on purpose, not by accident.
**Rust:** newtypes; `Ord`/`Eq`/`Hash` — derived and manual; `From`/`TryFrom`; `Display`/`Debug`; visibility strategy in full (`pub`, `pub(crate)`, re-export patterns, sealed modules); semver thinking.
**Domain:** `Name`/`NameComponent` and NDN canonical ordering (`ndn-foundation-types`); a preview of NDF's self-certifying TLV-type-3 name root.
**Machine:** what `derive` expands to — first `cargo expand` session (macro foreshadowing for M13).
**Craft:** the Rust API Guidelines as a checklist; rustdoc quality bar.
**Exercises:** template-assisted `Name` subset with canonical `Ord`, property-tested against `ndn-foundation-types` as oracle; **API-review exercise** — the tutor reviews your public surface against the API Guidelines; you defend or amend each finding.
**Readings:** Rust API Guidelines; matklad on module structure and visibility; `std::collections::BTreeMap` docs (Ord contract).
**Gate:** oracle-parity tests green; API review resolved with a written disposition per finding.

#### M7 · Zero-copy parsing: lifetimes in anger
**Goal:** lifetimes as a tool you reach for, demonstrated by a real performance win.
**Rust:** lifetime-carrying structs and impls; `Cow`; `bytes::Bytes` (refcounted buffers — the segue into M8/M10's smart-pointer question); lazy decoding.
**Domain:** how `ndn-packet` lazily decodes Interest/Data without copying; `MetaInfo`.
**Machine:** why Rust can optimize borrows aggressively (aliasing guarantees); measure the copies you removed.
**Craft:** benchmark-driven refactoring; writing a perf note a reviewer can trust.
**Exercises:** build a zero-copy packet field extractor; refactor an owned parser to borrowed and produce a criterion comparison report.
**Readings:** `bytes` crate docs; a curated zero-copy parsing article; *Rust for Rustaceans* lifetimes chapter (reference).
**Gate:** extractor passes; benchmark report shows and explains the delta.

#### M8 · Traits, generics, closures: choosing the tool
**Goal:** dispatch decisions made from understanding, not superstition — and the parser proven by force.
**Rust:** generics vs trait objects (monomorphization vs vtable, *with asm evidence*); object safety — the workspace's own non-object-safe `Face` trait (RPITIT) as the case study; `Fn`/`FnMut`/`FnOnce` closures; iterator adapters; `OnceCell`/`OnceLock`/`LazyLock` and lazy-init patterns; decision tree v1: `Box` vs `&dyn` vs generic vs `Rc`/`Arc` (completed in M10).
**Domain:** the `Strategy` trait and `PipelineStage` — real polymorphism seams in the engine.
**Machine:** vtable layout; inlining across monomorphized calls (godbolt side-by-side).
**Craft:** property-based testing philosophy; **first fuzzing lab** — parsers are the ideal introduction; fuzzer findings feed the bug journal.
**Exercises:** implement a small `Strategy`-like trait with both a generic and a `dyn` path, inspect and compare the asm; proptest + `cargo-fuzz` your mini-tlv (and, for sport, the vendored subset) — file and fix anything found, journaled.
**Readings:** Jon Gjengset, *Crust of Rust* (dispatch and closures episodes); proptest book; cargo-fuzz book.
**Gate:** fuzzer runs clean for the assigned budget or findings are filed, fixed, and journaled; a one-page dispatch write-up ("when I'd pick which, and why the asm says so").

### Phase 3 — Threads, processes, and async systems

#### M9 · Processes and threads
**Goal:** OS-level mastery — what a process and a thread actually *are*, and fluency with `std::thread` and `std::process` before any async sugar exists to hide them.
**Rust:** `std::thread::spawn` and `JoinHandle`; `move` closures (the closure muscle, again, now with stakes); scoped threads (`thread::scope`); `std::sync::mpsc` channels; thread panics and their propagation; `std::process::Command` — spawning, piping stdin/stdout, exit codes, timeouts, clean termination; rayon for data parallelism (a thread pool without the ceremony).
**Domain:** **`ndn-fwd` as the running specimen** — a daemon's anatomy: process lifecycle, config, the management Unix socket; `ndn-ipc`'s `ForwarderClient` (async and blocking variants) as the IPC case study; the `ndn-repo` daemon as a second specimen.
**Machine:** process anatomy — address space, file descriptors, fork/exec conceptually; the demonstration that threads share memory and processes don't (two threads mutating one static vs two processes failing to); context-switch cost measured; a signals one-pager (what Ctrl-C actually delivers); `/proc` spelunking on the live forwarder.
**Craft:** graceful shutdown as a *design requirement*, not an afterthought; a threads-vs-processes-vs-async(preview) decision note in house format.
**Exercises:** build a multi-threaded work distributor with scoped threads + channels (parallel TLV validation over a corpus is the suggested payload); **process-orchestration lab** — spawn `ndn-fwd` as a child from Rust, drive it with `ndn-ping` through piped I/O, assert on its log output, terminate it cleanly and verify the exit; rayon-ify a hot loop and measure the speedup curve against thread count.
**Readings:** CS:APP processes chapter (selected); Julia Evans on processes and signals; rayon docs; Rust Book ch. 16 as reference.
**Gate:** distributor passes tests including clean shutdown under panic; orchestration lab green; decision note reviewed.

#### M10 · Sharing state: smart pointers and concurrency
**Goal:** the `Rc`/`Arc`/`Mutex`/`RwLock`/atomics decision tree completed and grounded in hardware — with the races now demonstrable using M9's real threads.
**Rust:** `Rc`+`RefCell` vs `Arc`+`Mutex`/`RwLock`; `Send`/`Sync` intuition (and reading the compiler's refusals as information); interior mutability; `DashMap`; atomics 101; decision tree v2 (final form, one page, student-authored).
**Domain:** the engine's real choices as case studies — PIT as sharded `DashMap` (no global lock on the hot path), FIB as `NameTrie` with per-node `RwLock`, the `DeadNonceList`.
**Machine:** cache coherence; **false-sharing lab** (padded vs unpadded counters, measured); memory ordering gently (`Relaxed` vs `SeqCst` — enough to read code, not to write lock-free structures).
**Craft:** concurrency tests; making races impossible by construction where you can; debugging a race is a bug-journal centerpiece entry.
**Exercises:** build a toy sharded PIT against a provided concurrent test harness; take a deliberately racy program, understand *why* it doesn't compile, fix it two different ways and compare; annotate a real `ndn-store` type's `Send`/`Sync` story.
**Readings:** Mara Bos, *Rust Atomics and Locks* (ch. 1–3 + memory-ordering chapter skim); *Crust of Rust* Arc/Mutex episode.
**Gate:** sharded PIT passes the harness; decision tree v2 submitted and defended.

#### M11 · Async Rust for real
**Goal:** async demystified as compiled state machines; fluency with tokio's working set; the logging arc completed with `tracing`.
**Rust:** the `Future` mental model; tasks vs M9's threads — when each, and why the engine uses both; `spawn` vs `block_on`; `mpsc`/`oneshot`; `select`; cancellation; `Pin` only as far as needed to read real code.
**Domain:** the engine's task topology (`face_task → pipeline_runner → expiry_task`); `ndn-runtime`'s `Spawn`/`Sleep`/`Now` abstraction as a portability lesson (tokio native, wasm in the browser — same engine).
**Machine:** what `.await` compiles to (peek at the generated state machine); an epoll one-pager; tasks are cheap, threads are not — measured against M9's numbers.
**Craft:** **logging best practice, completed** — from log lines (M5) to structured `tracing` spans: span hierarchies, fields, when a span earns its existence; reading span trees as the async debugger's first tool.
**Exercises:** template-assisted in-memory `Face` pair over channels; an async echo producer/consumer across it; instrument with `tracing` and verify the span topology matches your mental model.
**Readings:** the Tokio tutorial (canonical); Async Book chapters as reference; one curated "how async actually works" deep-dive.
**Gate:** face pair passes the provided integration tests; span tree + explanation.

#### M12 · Growing the system: your first real component
**Goal:** the junior-contributor checkpoint — add a component under the workspace's own growth discipline.
**Rust:** feature flags; integration tests and `examples/`; adding a crate to a workspace properly.
**Domain:** implement a new face behind the `Transport` + `LinkService` split, following the repo's own `implementing-a-face` guide — e.g., a "lossy loopback" face for testing, or a Unix-datagram variant — wired into `ndn-fwd` config behind a feature flag. (Alternative track: a new `PipelineStage`.)
**Machine:** the syscall boundary; an optional `strace` session on your face.
**Craft:** **workspace growth as taught doctrine** — the scope buckets (`spec`/`extension`/`tooling`/`draft`), the dependency-direction rule, and where your crate belongs and why; commit hygiene; a witness test in the audit style (exits 1 before, 0 after); an honest `EXPECTED_FAILURES`-style note for what your face doesn't do; logs and comments at review quality (the M5 criteria, now under real-component load).
**Exercises:** template-assisted → from-scratch face implementation; witness authoring; a short PR-style narrative as if submitting upstream — including the debugging story: at least one journaled bug from this build, told as a reviewer would want to hear it.
**Readings:** `docs/wiki/src/guides/implementing-a-face.md`; the ARCHITECTURE.md scope-policy section; a curated article on structuring Rust projects as they grow.
**Gate:** witness green; tutor review passes at "would not embarrass you in a real PR" bar.

### Phase 4 — Architecture and NDF

#### M13 · Macros and codegen
**Goal:** macros written when warranted — and recognized when not.
**Rust:** `macro_rules!` (patterns, repetition, hygiene basics); the decision ladder *function → generic → trait → build.rs → macro*; reading a derive macro (not writing one); `cargo expand` as the x-ray.
**Domain:** TLV type-number tables and test-corpus builders — genuine, non-contrived macro use cases from this ecosystem.
**Machine:** where macros run in the compilation pipeline (tokens → AST); compile-time cost awareness.
**Craft:** the justification note: every macro ships with a paragraph on why nothing simpler sufficed; macros and debuggability in tension (expanded code is where the backtrace points).
**Exercises:** write a `macro_rules!` table generator for a TLV type registry + a corpus-builder macro; critique one existing macro found in the workspace.
**Readings:** *The Little Book of Rust Macros*; `std` macro sources (browse).
**Gate:** macros pass tests; justification notes accepted.

#### M14 · NDF: architecture as a first-class skill
**Goal:** the big-picture muscle — read a design corpus, take a position, write it down in a form others can ratify or refute.
**Rust:** deliberate consolidation — no new syntax; everything applied.
**Domain:** the vault's six-step spine (thesis → Block → kinds/manifests → security composition → wire → mediators); the Block primitive; the decisions ledger (D-1…D-47) and tensions register as *the genre of professional architectural writing*; the refounding's verdict litmus ("moving/storing bytes → ndn-workspace; meaning of bytes → NDF") as a reusable architectural razor.
**Craft:** ADR/decision-note writing in the house D-format; design docs before code as a norm, not a ceremony.
**Exercises:** (1) **tension exercise** — take a real pending question or proposed tension from the register, write a decision note in house format, defend it in Socratic review; (2) implement a slice of `ndf-core` — the Block header codec or chain/fork detection — **graded against the frozen conformance-corpus vectors** (the same oracle that guards the refounding grades the student).
**Readings:** vault spine notes; the disposition map; two curated ADR-culture articles (any language — the genre transfers).
**Gate:** decision note at ratifiable quality; corpus vectors green.

#### M15 · Trust and the verifier
**Goal:** security machinery understood well enough to extend without hand-rolling crypto.
**Rust:** error taxonomies at scale (typed `DenyReason` vocabularies); crypto hygiene — use the primitives, never reimplement them; constant-time comparison as a rule, not an optimization.
**Domain:** signing and verification end to end — `KeyChain`, the `TrustContext` keyring and its longest-prefix dispatch, NDF's two-signature model; implement two to three steps of the AC.12 conforming verifier against corpus vectors.
**Machine:** what a signature verify costs (benchmark Ed25519), and how that cost motivates the bounded-amortization rule.
**Craft:** threat thinking — a short "how would I attack my own module" write-up.
**Exercises:** verifier-step implementation (oracle-graded); a keyring dispatch exercise ("which context validates this packet, and why not the others").
**Readings:** vault `ac-12-conforming-verifier` and `security-composition`; the keyring section of ARCHITECTURE.md; one curated "cryptographic right answers"-style article.
**Gate:** verifier steps pass corpus; threat write-up reviewed.

## 7. Capstones

Both capstones are **proposal-gated**: a one-page design doc plus at least two decision notes in house format, reviewed and approved *before* implementation begins. This is where "make big architectural decisions confidently with a plan and implement methodically" is assessed, not just encouraged. Both end in a viva. Both capstone rubrics carry the covenant criteria: diagnosable code (comments, logs, spans) and a bug-journal chapter told honestly.

### Capstone A · The application (3–5 weeks)
Build a real application on the `ndn-app` Node API, deployed against `ndn-fwd` via the docker-compose testbed. Menu (or propose your own): a tap-to-share variant; LAN chat over SVS sync; sensor telemetry with a dashboard panel; a named-data toy for whatever hardware the student owns.
**Must include:** tests (at least one integration witness), `tracing` instrumentation, and a README whose quick start obeys the course's own minutes-to-running rule — the student dogfoods the onboarding standard.
**Grading:** the three gates + capstone rubric + viva + a live demo.

### Capstone B · The contribution (3–5 weeks)
Land a PR-quality contribution to ndn-workspace or the NDF refounding, drawn from a **curated ticket list** maintained with the instructor (sized examples: the N-5 docs-contract ask; witness-coverage gaps; a dashboard slice; a `ndf-core` piece from the disposition map; the Dioxus course-site port). Full contributor flow: issue → design note → review gate → implement → witness tests → PR-quality submission → review rounds until mergeable-quality.
**Outcome:** the student doesn't simulate being a contributor; they *are* one.

## 8. Electives (post-M12, any order)

- **Embedded** — `no_std` for real on `ndn-embedded`; memory strand goes deepest here.
- **Browser/WASM** — the engine in the browser (`ndn-wasm`, the Dioxus demo); the `ndn-runtime` abstraction pays off.
- **FFI** — `ndn-boltffi` Kotlin/Swift seams; what crosses a language boundary and what can't.
- **Simulation** — `ndn-sim` topologies; encode an NDF conformance scenario (the N-8 ask as a stretch).
- **Dashboard/Dioxus** — UI in Rust; feeds the course-site port ticket.
- **Performance** — criterion + flamegraphs + `perf`; make one hot path measurably faster and prove it.
- **Debugging, advanced** — sanitizers, `miri` in depth, core-dump analysis, deterministic-replay tools; for students who want diagnosis as a specialty.
- **NAND2Tetris** — for students who want the machine strand all the way down.

## 9. Open questions for course v0.3

1. **Cohort features** — the self-hosted server and richer interactivity re-enter here if cohorts materialize; v1 is deliberately single-player + tutor.
2. **NDF module activation** — M14/M15 code exercises depend on `ndf-core` landing; until then the conceptual halves run against the vault + frozen legacy vectors. Revisit at each course release.
3. **Editor integration** — a thin LSP-adjacent or extension layer only after content is proven with ≥3 real students.
4. **Assessment calibration** — the first cohort's submissions calibrate rubric strictness; budget a revision pass.
5. **Landing page → site integration** — the prototype landing page becomes the Starlight index; keep its content generated from this syllabus so the two never drift.
