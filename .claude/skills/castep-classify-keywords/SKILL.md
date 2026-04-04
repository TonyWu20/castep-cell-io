---
name: castep-classify-keywords
description: Interactive form to classify missing CASTEP keywords into Rust data types
model: haiku
---

# CASTEP Keyword Classifier

Reads TODOS.md, presents an interactive form for classifying each missing keyword into the appropriate Rust data type, then updates TODOS.md with your decisions.

## Instructions

When invoked:
1. Read TODOS.md to get missing keywords
2. For each keyword, present a simple choice form asking which Rust type to use
3. Update TODOS.md with the classifications

## Rust Type Options

- **Pattern 1: Simple Keyword Enum** - Enum with named variants (e.g., Task, ElectronicMinimizer)
- **Pattern 1b: Enum with Data Variants** - Enum variants containing data (e.g., Option<T>, Result<T, E>)
- **Pattern 2: Simple Scalar Type** - Newtype wrapping single value (e.g., Charge, Comment)
- **Pattern 3: Composite Key-Value Type** - Struct with multiple fields (e.g., CutOffEnergy with value + unit)
- **Pattern 4: Block Type** - Two-level Entry + Container structs (e.g., PositionsFrac)
- **Pattern 5: Unit Type** - Unit enum (e.g., EnergyUnit, LengthUnit)

## Workflow

1. Parse TODOS.md to extract missing keywords with their type, default, and URL
2. For each keyword, fetch full description from the URL
3. Use AskUserQuestion showing keyword name, type, default, and full description
4. Present the 6 Rust type pattern options
5. Collect all responses and regenerate TODOS.md with classifications
