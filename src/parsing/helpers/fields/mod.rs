use winnow::{token::take_till, ModalResult, Parser};

use super::{effective_line, KeywordType};

/// When it is "keyword : value"
pub fn field_name<'s>(input: &mut &'s str) -> ModalResult<KeywordType<'s>> {
    take_till(0.., |c| c == ' ' || c == ':')
        .map(|s: &str| KeywordType::Field(s.trim()))
        .parse_next(input)
}

pub fn get_field_data(input: &mut &str) -> ModalResult<String> {
    effective_line
        .map(|s| s.replace(':', "").trim_start().to_string())
        .parse_next(input)
}

#[cfg(test)]
mod test {
    use winnow::{
        error::{AddContext, ContextError, ErrMode, StrContext},
        stream::Stream,
    };

    use super::{field_name, get_field_data};

    #[test]
    fn test_field() {
        let mut tests = [
            "KPOINT_MP_GRID 3 4 5\r\n",
            "FIX_ALL_COM : true\r\n",
            "QUANTIZATION_AXIS: 0.0 0.0 1.0",
        ];
        tests.iter_mut().for_each(|s| {
            let name = field_name(s).unwrap();
            let context = ContextError::<StrContext>::new().add_context(
                s,
                &s.checkpoint(),
                StrContext::Label("Data"),
            );
            let data = get_field_data(s).map_err(|_| ErrMode::Backtrack(context));
            println!("{name:?}");
            dbg!(data.ok());
        });
    }
}
