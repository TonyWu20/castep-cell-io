# 0002 — chumsky for parsing

CASTEP `.cell` and `.param` files are line-oriented text with simple, non-recursive syntax: key-value pairs, block delimiters (`%block`/`%endblock`), and flags. This maps well to parser combinators — linear sequences of token patterns where each line's structure is independent of context.

`chumsky` was chosen over the alternatives because:

- **vs `nom`/`winnow`**: These are better suited for binary formats and streaming parsers with complex state machines. They expose a low-level "stream of bytes" API. The CASTEP format is entirely line-based — chumsky's combinator chain reads line-by-line with labelled errors, which produces significantly more readable code for this format.
- **vs hand-rolled**: A hand-rolled parser would work but would duplicate error-recovery and diagnostic infrastructure that chumsky provides out of the box (`chardump`, `.labeled()`, `.recover_with()`).
- **vs `pest`**: PEG grammars are declarative but opaque — harder to debug, harder to add conditional logic (e.g., parsing a block header gives context for how to parse its body rows).

The main risk is ecosystem maturity: `chumsky` is younger than `nom`. However, at version 0.10 with an established user base and active maintenance, it covers our use case without exposing us to churn.

**Consequences**: Format parsing code is in combinators, not imperative loops. New format features require chaining new combinators, not adding match arms in a loop. The `ariadne` crate is used alongside chumsky for rich error diagnostics (spans, underlines, hints).
