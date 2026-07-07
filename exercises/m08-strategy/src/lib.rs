//! m08-strategy — traits, generics, closures: choosing the tool.
//!
//! Read SPEC.md. Run the witness with `./course check m08-strategy`.
//!
//! A forwarding *strategy* decides which faces an Interest goes out on, given the
//! face it arrived on and the FIB's candidate next-hops. It's the perfect place to
//! meet Rust's dispatch choices: one `Strategy` trait, implemented three ways
//! (two named types, one closure), and used two ways — as a generic (`<S:
//! Strategy>`, monomorphized) and as a trait object (`dyn Strategy`, a vtable).
//!
//! You write six small pieces; each one is a different tool. The stubs compile;
//! the tests are red until you fill them.

/// A face identifier.
pub type FaceId = u32;

/// A forwarding strategy: given the incoming face and the candidate next-hops,
/// return the faces to send the Interest out on.
pub trait Strategy {
    fn choose(&self, incoming: FaceId, candidates: &[FaceId]) -> Vec<FaceId>;

    /// A short name for logs. Has a default; the named strategies override it.
    fn name(&self) -> &str {
        "strategy"
    }
}

/// Send to every candidate except the one it arrived on.
pub struct Multicast;

impl Strategy for Multicast {
    fn choose(&self, incoming: FaceId, candidates: &[FaceId]) -> Vec<FaceId> {
        candidates
            .iter()
            .copied()
            .filter(|&f| f != incoming)
            .collect()
    }

    fn name(&self) -> &str {
        "multicast"
    }
}

/// Send to the single best (first) candidate that isn't the incoming face.
pub struct BestRoute;

impl Strategy for BestRoute {
    fn choose(&self, incoming: FaceId, candidates: &[FaceId]) -> Vec<FaceId> {
        candidates
            .iter()
            .copied()
            .find(|&f| f != incoming)
            .into_iter()
            .collect()
    }

    fn name(&self) -> &str {
        "best-route"
    }
}

/// A strategy backed by a closure. A closure is a value you can store; `Fn` is the
/// bound that lets you call it again and again through `&self`.
#[allow(dead_code)] // until you read `label` and `f` in the impl below
pub struct FnStrategy<F> {
    label: &'static str,
    f: F,
}

impl<F> FnStrategy<F>
where
    F: Fn(FaceId, &[FaceId]) -> Vec<FaceId>,
{
    /// (provided)
    pub fn new(label: &'static str, f: F) -> Self {
        Self { label, f }
    }
}

impl<F> Strategy for FnStrategy<F>
where
    F: Fn(FaceId, &[FaceId]) -> Vec<FaceId>,
{
    fn choose(&self, incoming: FaceId, candidates: &[FaceId]) -> Vec<FaceId> {
        (self.f)(incoming, candidates)
    }

    fn name(&self) -> &str {
        self.label
    }
}

/// Dispatch chosen at COMPILE time: `S` is a known type, so this call is
/// monomorphized and inlinable — no vtable. (Compare with `compare_strategies`.)
pub fn forward_static<S: Strategy>(
    strategy: &S,
    incoming: FaceId,
    candidates: &[FaceId],
) -> Vec<FaceId> {
    strategy.choose(incoming, candidates)
}

/// Dispatch chosen at RUN time over a *heterogeneous* set of strategies — the
/// thing generics can't express, because each `Box<dyn Strategy>` may be a
/// different concrete type behind one vtable. Returns each strategy's name and
/// its choice.
pub fn compare_strategies(
    strategies: &[Box<dyn Strategy>],
    incoming: FaceId,
    candidates: &[FaceId],
) -> Vec<(String, Vec<FaceId>)> {
    strategies
        .iter()
        .map(|s| (s.name().to_string(), s.choose(incoming, candidates)))
        .collect()
}

/// The process-wide default strategy, built once on first call and shared
/// thereafter. (Here the value is cheap; the pattern earns its keep when it isn't.)
pub fn default_strategy() -> &'static Multicast {
    static DEFAULT: std::sync::OnceLock<Multicast> = std::sync::OnceLock::new();
    DEFAULT.get_or_init(|| Multicast)
}
