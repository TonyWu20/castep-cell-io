use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    IterParser, Parser,
    error::Rich,
    extra::{self},
    prelude::*,
    text::{self, ident, newline, whitespace},
};

use crate::{Cell, CellValue};

pub fn parse_cell_file<'a>(input: &'a str) -> Result<Vec<Cell<'a>>, Vec<Rich<'a, char>>> {
    choice((block(), keyvalue(), flag()))
        .padded_by(comment().or(newline()).repeated().or_not())
        .repeated()
        .collect::<Vec<Cell<'a>>>()
        .parse(input)
        .into_result()
}

fn cell_primitives<'src>()
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
        .map(|s: &str| {
            s.parse::<u32>()
                .map(CellValue::UInt)
                .or_else(|_| s.parse::<i32>().map(CellValue::Int))
                .or_else(|_| s.parse::<f64>().map(CellValue::Float))
                .unwrap()
        });
    let word = none_of(" %!#\r\n\n").repeated().at_least(1).to_slice();
    let boolean = word.validate(|word: &'src str, e, emitter| {
        if !word.eq_ignore_ascii_case("true") && !word.eq_ignore_ascii_case("false") {
            emitter.emit(Rich::custom(
                e.span(),
                format!("{word} is not a valid boolean value"),
            ))
        }
        word.eq_ignore_ascii_case("true")
    });
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
fn block_lines<'src>()
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
                    format!("{x} is not a valid block identifier {to_check}"),
                ))
            }
        })
}

/// Parse the whole block
fn block<'src>() -> impl Parser<'src, &'src str, Cell<'src>, extra::Err<Rich<'src, char>>> {
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
                    format!("{blk} is inconsistent with parsed endblock name {endblock}"),
                ))
            }
            Cell::Block(blk, lines)
        })
}

/// Parse a `key : value` pair
fn keyvalue<'src>() -> impl Parser<'src, &'src str, Cell<'src>, extra::Err<Rich<'src, char>>> {
    ident()
        .then_ignore(just(":").padded())
        .then(cell_primitives())
        .map(|(key, value)| Cell::KeyValue(key, value))
}

/// Rare in `.cell` and `.param`, example: `MAKE_SYMMETRY` and `STOP`
fn flag<'src>() -> impl Parser<'src, &'src str, Cell<'src>, extra::Err<Rich<'src, char>>> {
    ident()
        .then_ignore(newline().or(end()).rewind())
        .map(Cell::Flag)
}

/// Just throw away the comments
fn comment<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Rich<'src, char>>> {
    just('#')
        .or(just('!'))
        .padded()
        .then(any().and_is(newline().not()).repeated())
        .ignored()
}

pub fn rich_error(e: &Rich<char>, source_name: &str, src: &str) {
    Report::build(ReportKind::Error, (source_name, e.span().into_range()))
        .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
        .with_message(e.to_string())
        .with_label(
            Label::new((source_name, e.span().into_range()))
                .with_message(e.reason().to_string())
                .with_color(Color::Red),
        )
        .finish()
        .print((source_name, Source::from(src)))
        .unwrap()
}

#[cfg(test)]
mod parser_test {
    use std::fs::read_to_string;

    use super::parse_cell_file;
    use ariadne::{Color, Label, Report, ReportKind, Source};
    use chumsky::prelude::*;

    const EXAMPLE: &str = r#"
%BLOCK CELL_CONSTRAINTS
       1       2       3
       4       5       6
%ENDBLOCK CELL_CONSTRAINTS

FIX_COM : false
MAKE_SYMMETRY
"#;
    // const POSITIONS: &str = r#"%BLOCK POSITIONS_FRAC
    const POSITIONS: &str = r#"%BLOCK POSITIONS_FRAC
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
%ENDBLOCK POSITIONS_FRAC"#;

    fn rich_error(e: &Rich<char>, source_name: &str, src: &str) {
        Report::build(ReportKind::Error, (source_name, e.span().into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(e.to_string())
            .with_label(
                Label::new((source_name, e.span().into_range()))
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .print((source_name, Source::from(src)))
            .unwrap()
    }

    #[test]
    fn test_file() {
        let example_file = "Mg2SiO4_Cr_1.cell";
        let example = read_to_string(example_file).unwrap();
        let parsed = parse_cell_file(&example)
            .map_err(|errors| {
                errors
                    .iter()
                    .for_each(|e| rich_error(e, "positions", POSITIONS));
            })
            .unwrap();
        dbg!(parsed);
    }
    #[test]
    fn parser_test_part() {
        let parsed = parse_cell_file(POSITIONS)
            .map_err(|errors| {
                errors
                    .iter()
                    .for_each(|e| rich_error(e, "POSITIONS", POSITIONS));
            })
            .unwrap();
        dbg!(parsed);
    }
}
