# 0001 — Two-crate architecture

The workspace is split into two crates: `castep-cell-fmt` (the parser/formatter backend) and `castep-cell-io` (the domain types). `castep-cell-io` depends on `castep-cell-fmt`. The reverse dependency is forbidden.

This isolates two sources of churn: the format specification (`.cell`/`.param` syntax evolves independently of CASTEP version support) and the domain vocabulary (keyword enums, constraint rules, unit types evolve with CASTEP releases). A downstream consumer who only needs typed domain objects depends on `castep-cell-io`; a consumer building a new format tool can depend only on `castep-cell-fmt` without pulling in domain types.

**Considered Options**: Single crate (simpler, all code in one place; risk of coupling format-spec changes to domain-type releases). Two-crate won because the CASTEP format itself is stable — it's the set of supported keywords and their validation that changes between CASTEP versions. Two crates let us ship domain-type updates independently of the parser/formatter, and let third-party format tools reuse the parser without the domain types.

**Consequences**: Higher workspace overhead (two Cargo.tomls, separate versioning). Requires clear boundary enforcement — `castep-cell-fmt` must never import from `castep-cell-io`.
