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
        let _ = (incoming, candidates);
        todo!("every candidate that isn't `incoming`, in order — see HINTS")
    }

    fn name(&self) -> &str {
        "multicast"
    }
}

/// Send to the single best (first) candidate that isn't the incoming face.
pub struct BestRoute;

impl Strategy for BestRoute {
    fn choose(&self, incoming: FaceId, candidates: &[FaceId]) -> Vec<FaceId> {
        let _ = (incoming, candidates);
        todo!("the first candidate that isn't `incoming`, or none — see HINTS")
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
        let _ = (incoming, candidates);
        todo!("call the stored closure with the arguments — see HINTS")
    }

    fn name(&self) -> &str {
        todo!("return the label you were constructed with")
    }
}

/// Dispatch chosen at COMPILE time: `S` is a known type, so this call is
/// monomorphized and inlinable — no vtable. (Compare with `compare_strategies`.)
pub fn forward_static<S: Strategy>(
    strategy: &S,
    incoming: FaceId,
    candidates: &[FaceId],
) -> Vec<FaceId> {
    let _ = (strategy, incoming, candidates);
    todo!("just call choose — the lesson is the generic bound, not the body")
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
    let _ = (strategies, incoming, candidates);
    todo!("map each boxed strategy to (its name as a String, its choice) — see HINTS")
}

/// The process-wide default strategy, built once on first call and shared
/// thereafter. (Here the value is cheap; the pattern earns its keep when it isn't.)
pub fn default_strategy() -> &'static Multicast {
    todo!("use a `static OnceLock<Multicast>` and `get_or_init` — see HINTS")
}
