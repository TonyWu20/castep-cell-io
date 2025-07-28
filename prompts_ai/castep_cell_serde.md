This is a file in format of `CASTEP`, `.cell`:

```
%BLOCK LATTICE_CART
      10.182880152352300       0.000000000000000       0.000000000000000
       0.000000000000000       5.969867637928440       0.000000000000000
       0.000000000000000       0.000000000000000       4.750940602435009
%ENDBLOCK LATTICE_CART

%BLOCK POSITIONS_FRAC
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
%ENDBLOCK POSITIONS_FRAC

%BLOCK KPOINTS_LIST
   0.0000000000000000    0.2500000000000000    0.3333333333333333       0.333333333333333
   0.0000000000000000    0.2500000000000000    0.0000000000000000       0.333333333333333
   0.0000000000000000    0.2500000000000000   -0.3333333333333333       0.333333333333333
%ENDBLOCK KPOINTS_LIST

%BLOCK SYMMETRY_OPS
       1.000000000000000       0.000000000000000       0.000000000000000
       0.000000000000000       1.000000000000000       0.000000000000000
       0.000000000000000       0.000000000000000       1.000000000000000
       0.000000000000000        0.000000000000000        0.000000000000000
      -1.000000000000000       0.000000000000000       0.000000000000000
       0.000000000000000      -1.000000000000000       0.000000000000000
       0.000000000000000       0.000000000000000      -1.000000000000000
       0.000000000000000        0.000000000000000        0.000000000000000
%ENDBLOCK SYMMETRY_OPS

%BLOCK CELL_CONSTRAINTS
       1       2       3
       4       5       6
%ENDBLOCK CELL_CONSTRAINTS

FIX_COM : false
%BLOCK IONIC_CONSTRAINTS
%ENDBLOCK IONIC_CONSTRAINTS

%BLOCK EXTERNAL_EFIELD
    0.0000000000     0.0000000000     0.0000000000
%ENDBLOCK EXTERNAL_EFIELD

%BLOCK SPECIES_MASS
       O     15.9989995956
      Mg     24.3050003052
      Si     28.0849990845
      Cr     51.9959983826
%ENDBLOCK SPECIES_MASS

%BLOCK SPECIES_POT
       O  O_00PBE_OP.recpot
      Mg  Mg_00PBE_OP.recpot
      Si  Si_00PBE_OP.recpot
      Cr  Cr_00.recpot
%ENDBLOCK SPECIES_POT

%BLOCK SPECIES_LCAO_STATES
       O         2
      Mg         4
      Si         2
      Cr         3
%ENDBLOCK SPECIES_LCAO_STATES

SYMMETRY_GENERATE
```

Let's try to map it into rust's `serde`'s data model.

We have the following primitive types in `.cell`:

1. `string`: map to `string`
2. Integers: map to `i32` is enough
3. floating numbers: `CASTEP` uses double precision, so it should be `f64`
4. `key : value` pair, like `FIX_COM : false`. How about mapping it to a `newtype_struct`
   in `serde`, as `FixCOM(bool)`?
5. `key` without anything trailing, like `SYMMETRY_GENERATE`. In `.cell`, the present of this
   kind of keyword set the value to `true`, and hence it is like a flag. I think mapping it
   to `unit_struct` is suitable, e.g. `struct SymmetryGenerate`, or also a `newtype_struct`
   as `SymmetryGenerate(bool)`. Since in `.cell`, this type of entry is vary rare,
   we can just make it as a `unit_struct` without worrying too much.
6. Blocks. Starts with a marker `%BLOCK`, and then the name of the block, which does not allow
   whitespaces. Then there are lines of text until reaching the end of the block, a line with
   `%ENDBLOCK` then follows the same block name as that in the header line.
   The format of the lines are consistent within the same block, with optional field inside the line.
   However, the format of lines vary according to the concrete blocks. I proposed to compose the `serde`
   data types to correctly express this type of entry in `.cell`.
   - The whole block is a `struct`
   - Lines are `seq` (`Vec<T>` in rust) of a `struct` (the `T` in `Vec<T>`).
   - For each line, we split the values by whitespaces. Each line will be deserialized into a `struct`,
     and the struct fields are filled by these values.
   - Since `.cell` allows some alternative expressions in the lines of blocks, e.g., you can specify atom
     positions as `<Element Symbol> <frac coord x> <frac coord y> <frac coord z>`,
     or also `<Atomic number> <frac coord x> <frac coord y> <frac coord z>`. For these alternatives, we can
     defined it as `enum` for the field of `struct` of this line.
     The keys and block names are not allowed to repeat.

I have implemented a parser to parse the `.cell` into intermediate representations, like this:

```rust
#[derive(Debug, Clone)]
pub enum CellValue<'a> {
    Null,
    Bool(bool),
    Str(&'a str),
    Int(i32),
    Float(f64),
    Array(Vec<CellValue<'a>>),
}

pub enum CellEntry<'a'>{
    KeyValue(&'a str, CellValue<'a>),
    Block(Block<'a>),
    Flag(&'a str),
}

#[derive(Debug, Clone)]
/// The intermediate struct representing the `block` type
/// of `CASTEP` in `.cell` and `.param`
pub struct Block<'a> {
    /// Block name
    pub name: &'a str,
    /// Lines of block
    pub contents: Vec<CellValue<'a>>,
}

impl<'a> Block<'a> {
    pub fn new(name: &'a str, contents: Vec<CellValue<'a>>) -> Self {
        Self { name, contents }
    }
}

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    IterParser, Parser,
    error::Rich,
    extra::{self},
    prelude::*,
    text::{self, ident, newline, whitespace},
};

use crate::{Block, Cell};

pub fn parse_cell_file(input: &str) -> Result<Vec<CellEntry>, Vec<Rich<char>>> {
    choice((block(), keyvalue(), flag()))
        .padded_by(comment().or(newline()).repeated().or_not())
        .repeated()
        .collect::<Vec<CellEntry>>()
        .parse(input)
        .into_result()
}

pub fn cell_primitives<'src>()
-> impl Parser<'src, &'src str, CellValue<'src>, extra::Err<Rich<'src, char>>> {
    let digits = text::digits::<&str, extra::Err<Rich<char>>>(20).to_slice();
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
        .boxed()
        .map(|s: &str| match s.parse::<i32>() {
            Ok(i) => Cell::Int(i),
            Err(_) => s
                .parse::<f64>()
                .map(CellValue::Float)
                .unwrap_or(CellValue::Float(0.0)),
        });
    let boolean = choice((just("true").to(true), just("false").to(false)));
    let word = none_of(" %!#\r\n\n").repeated().at_least(1).to_slice();
    let comment = just('#')
        .or(just('!'))
        .then_ignore(
            any::<&str, extra::Err<Rich<char>>>()
                .and_is(newline().not())
                .repeated(),
        )
        .ignored();

    choice((
        comment.map(|_| CellValue::Null),
        boolean.map(CellValue::Bool),
        number,
        word.map(CellValue::Str),
    ))
}

/// Parser to handle lines inside the block.
/// It returns a `Vec<Cell::Array(Cell)>`, as we have zero to many lines,
/// and each line contains a variety of `Cell` type.
pub fn block_lines<'src>()
-> impl Parser<'src, &'src str, Vec<CellValue<'src>>, extra::Err<Rich<'src, char>>> {
    // Recognize the basic types
    cell_primitives()
        // .. then separated by at least one whitespace
        .separated_by(just(' ').then(whitespace()).to_slice())
        // Since `CASTEP` and `Materials Studio` prefers formatting the data in right-align and with fixed-width
        // style, leading whitespaces are of high likelihood.
        .allow_leading()
        .collect::<Vec<CellValue>>()
        // Turn `Input` to `Parser` for convenience
        .boxed()
        // The final line before "%ENDBLOCK" goes with a trailing newline,
        // the parser will generate an empty `Vec`, so we filter it out
        .filter(|item| !item.is_empty())
        .map(CellValue::Array)
        // Separate the lines
        .separated_by(newline())
        .collect::<Vec<_>>()
}

/// `.cell` format is case-insensitive, `chumsky` does not have built-in
/// caseless parsing support, so we have to handle the case-sensitivity in `%BLOCK`
/// and `%ENDBLOCK` with this workaround.
fn caseless_check_block<'src, 'a>(
    to_check: &'a str,
) -> impl Parser<'src, &'src str, (), extra::Err<Rich<'src, char>>> {
    // Marker of the block identifier
    just::<&'src str, &'src str, extra::Err<Rich<char>>>("%")
        .then(ident())
        .to_slice()
        .validate(move |x: &'src str, e, emitter| {
            if !x.eq_ignore_ascii_case(to_check) {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("{} is not a valid block identifier {}", x, to_check,),
                ))
            }
        })
}

/// Parse the whole block
pub fn block<'src>() -> impl Parser<'src, &'src str, CellEntry<'src>, extra::Err<Rich<'src, char>>> {
    let block_start = caseless_check_block("%block")
        .padded()
        .ignore_then(ident())
        .then_ignore(newline());
    block_start
        .then(block_lines())
        .then(
            caseless_check_block("%endblock")
                .padded()
                .ignore_then(ident()), // .then_ignore(newline().or(end())),
        )
        .validate(|((blk, lines), endblock), e, emitter| {
            if blk != endblock {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!(
                        "{} is inconsistent with parsed endblock name {}",
                        blk, endblock
                    ),
                ))
            }
            Cell::Block(Block::new(blk, lines))
        })
}

/// Parse a `key : value` pair
pub fn keyvalue<'src>() -> impl Parser<'src, &'src str, CellEntry<'src>, extra::Err<Rich<'src, char>>> {
    ident()
        .then_ignore(just(":").padded())
        .then(cell_primitives())
        .map(|(key, value)| Cell::KeyValue(key, value))
}

/// Rare in `.cell` and `.param`, example: `MAKE_SYMMETRY` and `STOP`
pub fn flag<'src>() -> impl Parser<'src, &'src str, CellEntry<'src>, extra::Err<Rich<'src, char>>> {
    ident()
        .then_ignore(newline().or(end()).rewind())
        .map(Cell::Flag)
}

/// Just throw away the comments
pub fn comment<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Rich<'src, char>>> {
    just('#')
        .or(just('!'))
        .padded()
        .then(any().and_is(newline().not()).repeated())
        .ignored()
}

fn rich_error(e: Rich<char>, src: &str) {
    Report::build(ReportKind::Error, ((), e.span().into_range()))
        .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
        .with_message(e.to_string())
        .with_label(
            Label::new(((), e.span().into_range()))
                .with_message(e.reason().to_string())
                .with_color(Color::Red),
        )
        .finish()
        .eprint(Source::from(src))
        .unwrap()
}
```

Please guide me how to implement a `Deserializer` for this format, based on my parser and intermediate data structures, so I can
deserialize the intermediate representations to my defined data structs or enums.
