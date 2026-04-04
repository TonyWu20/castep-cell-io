---
name: Architecture decisions
description: Key decisions and patterns discovered during orchestration
type: project
---

# Architecture Decisions

## CResult and Error type
`CResult<T>` = `Result<T, Error>` from `castep_cell_serde::error`.
`Error` enum has variants: Message(String), Empty, UnexpectedType(String,String), KeyNotFound(String), TryFromInt(TryFromIntError).
During Phase 3, serde trait impls on Error (serde::ser::Error and serde::de::Error) will be removed.

## Cell and CellValue IR
Defined in `castep_cell_serde/src/lib.rs`. Must NOT be moved — they are the shared IR.
Parser produces `Vec<Cell<'a>>` with lifetime tied to the input `&str`.

## ToCellValue / ToCell / ToCellFile traits
These serialization traits stay in `castep_cell_serde/src/lib.rs` throughout the rewrite.
They are NOT removed by the plan — only serde is removed.

## format.rs vs ser.rs
`format.rs` is an infallible direct formatter (returns `String`, not `CResult<String>`).
It temporarily exports as `to_string_direct` to avoid conflict with `ser.rs`'s `to_string`.
In Phase 3, `to_string_direct` → `to_string` after `ser.rs` is deleted.

## Ordering constraints
- T-001 and T-002 must precede T-003 (format.rs uses query helpers)
- T-001, T-002, T-003 must all complete before T-004 (integration tests need all three)
- T-005 must precede T-008 (Pattern D blocks use unit enum FromCellValue impls)
- T-005, T-006 must precede T-007 (param types may use unit enums as values)
- T-005 through T-009 must all precede T-010 (tests validate the migrated types)
- Phase 2 (T-005..T-010) must complete before Phase 3 (T-011..T-022)
- T-015 through T-019 must precede T-020 (use path changes depend on crate rename)
