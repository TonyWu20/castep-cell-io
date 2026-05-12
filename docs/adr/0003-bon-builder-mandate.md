# 0003 — bon builder mandate

All non-trivial struct construction in `castep-cell-io` uses `#[derive(bon::Builder)]`. This is a hard rule — no manual constructors, no positional `new()`, no `Default` (except for param groups where all fields are `Option<T>` anyway).

The `bon` crate was chosen over `derive_builder` (the previous choice, migrated from in v0.4+) because:

- **Mandatory field support**: `bon` enforces required fields at compile time via typestate (`IsComplete`). `derive_builder` required runtime panics or `Option<T>` for every field.
- **Fallible builders**: Documents with validation (mutual-exclusion constraints between blocks) need a fallible `build()` that returns `Result`. `bon` supports this with `finish_fn` customisation; `derive_builder` required a separate validation step after construction.
- **Collection helpers**: Block types hold `Vec<Entry>` fields. `bon`'s collection builder methods (`add_*`) are generated automatically from `#[builder(default)]` on `Vec<T>` fields, producing ergonomic push-style APIs without manual `maybe_` wrappers.
- **No proc-macro conflicts**: `bon` derives work alongside serde's `#[derive(Serialize, Deserialize)]` without ordering issues.

This mandate means no struct in `castep-cell-io` is constructible without a builder. Callers always write `Type::builder().field_a(x).field_b(y).build()`, never `Type::new(x, y)`. The consistency eliminates a class of bugs where positional arguments were swapped in manual constructors.

**Considered Options**: Manual `new()` methods (verbose, positional arguments error-prone). `derive_builder` (runtime optionals for all fields, no fallible builds). Pure `Default` + mutation (no compile-time safety). `bon` was the only option that satisfied the mandate at compile time.

**Consequences**: All struct definitions in `castep-cell-io` are annotated with `#[derive(bon::Builder)]`. The builder module structure follows bon's conventions (`{type}_builder::IsComplete` for fallible builders, `maybe_` prefix for `Option<T>` setters). Adding a new field to a struct automatically updates the builder — no constructor edits needed.
