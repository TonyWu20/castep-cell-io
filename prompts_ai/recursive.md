Please explain the below recursive parser of json for me, focus on how it will recursively parse into object, array and the values of the array.
You should also explain the mapping betweens the parser behaviors and the data types in JSON, and the hierarchy of the data types. Why we don't need another variant to express there are many objects in a JSON file?'

If I want to implement a similar parser for a custom format, what should I pay attention to and how should I structure the pieces of the recursive parser?

```rust
//! This is a parser for JSON.
//! Run it with the following command:
//! cargo run --example json -- examples/sample.json

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use std::{collections::HashMap, env, fs};

#[derive(Clone, Debug)]
pub enum Json {
    Invalid,
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

fn parser<'a>() -> impl Parser<'a, &'a str, Json, extra::Err<Rich<'a, char>>> {
    recursive(|value| {
        let digits = text::digits(10).to_slice();

        let frac = just('.').then(digits);

        let exp = just('e')
            .or(just('E'))
            .then(one_of("+-").or_not())
            .then(digits);

        let number = just('-')
            .or_not()
            .then(text::int(10))
            .then(frac.or_not())
            .then(exp.or_not())
            .to_slice()
            .map(|s: &str| s.parse().unwrap())
            .boxed();

        let escape = just('\\')
            .then(choice((
                just('\\'),
                just('/'),
                just('"'),
                just('b').to('\x08'),
                just('f').to('\x0C'),
                just('n').to('\n'),
                just('r').to('\r'),
                just('t').to('\t'),
                just('u').ignore_then(text::digits(16).exactly(4).to_slice().validate(
                    |digits, e, emitter| {
                        char::from_u32(u32::from_str_radix(digits, 16).unwrap()).unwrap_or_else(
                            || {
                                emitter.emit(Rich::custom(e.span(), "invalid unicode character"));
                                '\u{FFFD}' // unicode replacement character
                            },
                        )
                    },
                )),
            )))
            .ignored()
            .boxed();

        let string = none_of("\\\"")
            .ignored()
            .or(escape)
            .repeated()
            .to_slice()
            .map(ToString::to_string)
            .delimited_by(just('"'), just('"'))
            .boxed();

        let array = value
            .clone()
            .separated_by(just(',').padded().recover_with(skip_then_retry_until(
                any().ignored(),
                one_of(",]").ignored(),
            )))
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(
                just('['),
                just(']')
                    .ignored()
                    .recover_with(via_parser(end()))
                    .recover_with(skip_then_retry_until(any().ignored(), end())),
            )
            .boxed();

        let member = string.clone().then_ignore(just(':').padded()).then(value);
        let object = member
            .clone()
            .separated_by(just(',').padded().recover_with(skip_then_retry_until(
                any().ignored(),
                one_of(",}").ignored(),
            )))
            .collect()
            .padded()
            .delimited_by(
                just('{'),
                just('}')
                    .ignored()
                    .recover_with(via_parser(end()))
                    .recover_with(skip_then_retry_until(any().ignored(), end())),
            )
            .boxed();

        choice((
            just("null").to(Json::Null),
            just("true").to(Json::Bool(true)),
            just("false").to(Json::Bool(false)),
            number.map(Json::Num),
            string.map(Json::Str),
            array.map(Json::Array),
            object.map(Json::Object),
        ))
        .recover_with(via_parser(nested_delimiters(
            '{',
            '}',
            [('[', ']')],
            |_| Json::Invalid,
        )))
        .recover_with(via_parser(nested_delimiters(
            '[',
            ']',
            [('{', '}')],
            |_| Json::Invalid,
        )))
        .recover_with(skip_then_retry_until(
            any().ignored(),
            one_of(",]}").ignored(),
        ))
        .padded()
    })
}

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let (json, errs) = parser().parse(src.trim()).into_output_errors();
    println!("{json:#?}");
    errs.into_iter().for_each(|e| {
        Report::build(ReportKind::Error, ((), e.span().into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(e.to_string())
            .with_label(
                Label::new(((), e.span().into_range()))
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .print(Source::from(&src))
            .unwrap()
    });
}
```

---

I want to parse a formatted text like this:

```
  O   0.1635419733526620    0.0317792047151180    0.2751746346719976
  O   0.3354045184454477    0.9672373612661035    0.7746824750061752
  O   0.8364477955763916    0.5313805688511324    0.7241701610821136
  O   0.6653182432350798    0.4679952694597609    0.2187534484550325
  O   0.0539200916667086    0.7500362471704362    0.7220278753304680
  O   0.4453965167140691    0.2440160309838545    0.2128379221037726
  O   0.0918663259131503    0.2498358217910210    0.7673845460556011
  O   0.4059051594219431    0.7518455464779391    0.2698918585108838
  O  -0.1635419733526620   -0.0317792047151180   -0.2751746346719977
  O  -0.3354045184454477   -0.9672373612661035   -0.7746824750061753
  O  -0.8364477955763918   -0.5313805688511324   -0.7241701610821137
  O  -0.6653182432350799   -0.4679952694597609   -0.2187534484550325
  O  -0.0539200916667086   -0.7500362471704362   -0.7220278753304681
  O  -0.4453965167140692   -0.2440160309838544   -0.2128379221037725
  O  -0.0918663259131503   -0.2498358217910209   -0.7673845460556012
  O  -0.4059051594219432   -0.7518455464779391   -0.2698918585108838
 Mg   0.2227159254607965    0.7504470916631593    0.4926066816971608
 Mg   0.2757366027392655    0.2484214504543120    0.9892634813029080
 Mg  -0.2227159254607965   -0.7504470916631593   -0.4926066816971609
 Mg  -0.2757366027392656   -0.2484214504543120   -0.9892634813029079
 Mg   0.0000000000000000    0.0000000000000000    0.0000000000000000
 Mg   0.5000000000000000    0.0000000000000000    0.5000000000000000
 Mg   0.0000000000000000    0.5000000000000000    0.0000000000000000
 Si   0.0942200147741907    0.2501820112892181    0.4262683890462294
 Si   0.4056313552411618    0.7503866208268292    0.9270250152159749
 Si  -0.0942200147741907   -0.2501820112892181   -0.4262683890462295
 Si  -0.4056313552411619   -0.7503866208268292   -0.9270250152159748
 Cr   0.5000000000000000    0.5000000000000000    0.5000000000000000
```

With the following data type definitions:

```rust
#[derive(Debug, Clone)]
pub enum Cell {
    Null,
    Bool(bool),
    Str(String),
    Int(i32),
    Float(f64),
    Array(Vec<Cell>),
}
```

I want to parse it as `Cell::Array(Vec<Cell::Array>)`
Please analyze the AST of the format, and tell me how to parse it with `chumsky`'s left-recursion.
   Compiling castep-cell-io v0.3.0 (/home/tony/programming/castep-cell-io)
warning: unused import: `collections::HashMap`
  --> src/lib.rs:21:15
   |
21 |     use std::{collections::HashMap, fs::read_to_string};
   |               ^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `parser::parse_cell_tokens`
  --> src/lib.rs:29:9
   |
29 |     use parser::parse_cell_tokens;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `Parser`, `error::Rich`, `extra`, `newline`, `prelude::*`, `self`, and `whitespace`
 --> src/parser_test/json_like.rs:4:5
  |
4 |     Parser,
  |     ^^^^^^
5 |     error::Rich,
  |     ^^^^^^^^^^^
6 |     extra,
  |     ^^^^^
7 |     prelude::*,
  |     ^^^^^^^^^^
8 |     text::{self, newline, whitespace},
  |            ^^^^  ^^^^^^^  ^^^^^^^^^^

warning: unused imports: `combinator::SeparatedBy` and `ident`
  --> src/parser_test/json_like.rs:56:9
   |
56 |         combinator::SeparatedBy,
   |         ^^^^^^^^^^^^^^^^^^^^^^^
57 |         prelude::*,
58 |         text::{ident, newline, whitespace},
   |                ^^^^^

warning: unused variable: `line`
   --> src/parser_test/json_like.rs:125:17
    |
125 |             let line = integer
    |                 ^^^^ help: if this is intentional, prefix it with an underscore: `_line`
    |
    = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `example`
  --> src/lib.rs:97:13
   |
97 |         let example = read_to_string(example_file).unwrap();
   |             ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_example`

warning: `castep-cell-io` (lib test) generated 6 warnings (run `cargo fix --lib -p castep-cell-io --tests` to apply 4 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.19s
     Running unittests src/lib.rs (target/debug/deps/castep_cell_io-0265730ec12dc0a4)

running 1 test
[src/lib.rs:136:9] output = None
Error: found 'O' expected something else
   ╭─[ positions:1:3 ]
   │
 1 │   O   0.1635419733526620    0.0317792047151180    0.2751746346719976
   │   │ 
   │   ╰─ found 'O' expected something else
───╯
test parser_test::test_recur ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

