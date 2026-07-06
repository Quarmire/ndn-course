# Hints — m06-name-api

Revealed one rung at a time by `./course hint m06-name-api`.

## Hint 1 — NameComponent::cmp is three tiers, chained

`Ordering` has exactly the combinator for this: `a.cmp(&b).then_with(|| ...)`.
The first tier that isn't `Equal` wins; ties fall through. Tier 1 is
`self.typ.cmp(&other.typ)`. Tier 2 is the value *lengths*
(`self.value.len().cmp(&other.value.len())`). Tier 3 is the value *bytes*
(`self.value.cmp(&other.value)` — a `Vec<u8>` already compares lexicographically).
Length before content is the part that isn't a derive.

## Hint 2 — Name::cmp is one call

A `Vec<T>` where `T: Ord` is itself `Ord`, compared lexicographically. So the whole
method is `self.components.cmp(&other.components)` — the standard library walks the
two vectors component-by-component, using the `NameComponent::cmp` you just wrote,
and handles the shorter-name-loses-on-a-prefix case for you.

## Hint 3 — has_prefix, Display

`has_prefix`: if `prefix.len() > self.len()`, it can't be a prefix — return false.
Otherwise `self.components.iter().zip(prefix.components())` and check `all` pairs
equal. For `Display`: if there are no components, `write!(f, "/")`; otherwise loop
and `write!(f, "/{}", String::from_utf8_lossy(&c.value))?` for each — that yields
`/ndn/course` and, for the root, `/`.

## Hint 4 — From<&str>

`uri.split('/')` gives you the segments, including empty strings for the leading
slash and any doubled/trailing ones. `.filter(|s| !s.is_empty())` drops those. Map
each surviving segment to `NameComponent::generic(s.as_bytes().to_vec())` and feed
the iterator to `Name::from_components`. `"/ndn/course"` → two generic components;
`"/"` and `""` → the root name.
