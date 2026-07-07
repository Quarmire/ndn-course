# Hints — m14-verdict

Revealed one rung at a time by `./course hint m14-verdict`.

## Hint 1 — verdict is two comparisons and an else

The band is `[estimate - margin, estimate + margin]`. Ask, in order:

```rust
if c.estimate - c.margin >= threshold {
    SpatialVerdict::Covered           // the whole band is at/above the threshold
} else if c.estimate + c.margin < threshold {
    SpatialVerdict::NotCovered        // the whole band is below it
} else {
    SpatialVerdict::Unresolved        // the threshold is somewhere inside the band
}
```

Order matters only in that the `else` catches the straddle. Use `>=` for the covered
edge (a band whose bottom just touches the threshold is covered).

## Hint 2 — don't compare floats for equality

You only ever need `>=` and `<` here, never `==` on an `f64` — good, because float
equality is a trap. The three zones are decided entirely by inequalities.

## Hint 3 — authorize is a match that preserves the numbers

Reuse `verdict`, then translate:

```rust
match verdict(c, threshold) {
    SpatialVerdict::Covered    => Verdict::Authorized,
    SpatialVerdict::NotCovered => Verdict::Refused,
    SpatialVerdict::Unresolved => Verdict::Unresolved {
        estimate: c.estimate,
        margin: c.margin,
        threshold,
    },
}
```

The `Unresolved` arm must carry `c.estimate`, `c.margin`, and `threshold` — that's
the "here's exactly what's missing" the caller needs. If you're tempted to write
`SpatialVerdict::Unresolved => Verdict::Refused` "to be safe," stop: that collapse is
the exact bug this module is about.

## Hint 4 — the decision note

For the journal note, keep the house form tight: one-line **principle** (a litmus a
reader can apply), a short **why**, what it **replaces/rejects**, and an **authority
tag** (ruling / recommendation / advisory). Let the four-verdict thesis do the
arguing: if `Unresolved` collapses to `Deny`, what totality property is lost, and who
should own the choice the collapse silently makes?
