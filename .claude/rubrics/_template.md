# Rubric template

Copy per exercise as `<exercise-name>.md`. The tutor grades each criterion
0–2 (0 = absent/wrong, 1 = present but weak, 2 = solid) with one concrete,
line-anchored observation per criterion. Verdict: pass requires no zeros and
a total ≥ 70% of maximum; otherwise revise with an exact change list.

| # | Criterion | What "2" looks like |
|---|---|---|
| 1 | Correctness beyond the tests | Handles cases the witnesses imply but don't enumerate; no lurking panic paths |
| 2 | Idiom | Reads like Rust, not translated C/Python; std vocabulary used where it exists |
| 3 | Error design | Failure paths are typed, precise, and impossible to ignore silently |
| 4 | Comments (standing) | Comments say *why*; no narration, no commented-out code, no drift |
| 5 | Logging (standing, where applicable) | Right level, right boundary, structured; silent where silence is correct |
| 6 | Tests the student added | At least one test beyond the provided suite showing they probed their own edges |
| 7 | Simplicity | Nothing present that the spec didn't demand; complexity is spent, not leaked |
