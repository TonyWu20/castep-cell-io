use winnow::{
    ascii::{line_ending, space1, till_line_ending, Caseless},
    combinator::{alt, eof, preceded, repeat_till, terminated},
    token::take_until,
    ModalResult, Parser,
};

use super::{effective_line, KeywordType};

/// Go to block name
/// essential for matching the next data is block type or field type.
pub fn strip_to_block_name<'s>(input: &mut &'s str) -> ModalResult<KeywordType<'s>> {
    preceded(terminated(Caseless("%block"), space1), effective_line)
        .map(KeywordType::Block)
        .parse_next(input)
}
/// Get contents in block
fn contents_in_block<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    take_until(0.., '%').parse_next(input)
}

/// Move out of the block ending line
fn end_of_block<'s>(input: &mut &'s str) -> ModalResult<(&'s str, &'s str)> {
    // Only move out of this line.
    // The remaining blank lines are handled by higher level parsers.
    (till_line_ending, alt((line_ending, eof))).parse_next(input)
}

/// When the block name is identified, the remaining `input`
/// is directly the lines for data and the ending line of the block
/// Returns the lines and throw away the ending to move out of the
/// block in `input`
pub fn get_block_data(input: &mut &str) -> ModalResult<String> {
    terminated(contents_in_block, end_of_block)
        .map(|s: &str| {
            if s.is_empty() {
                s.to_string()
            } else {
                repeat_till(0.., effective_line, eof)
                    .map(|(s, _): (Vec<&str>, &str)| s.join("\n"))
                    .parse(s)
                    .unwrap()
            }
        })
        .parse_next(input)
}

#[cfg(test)]
mod test {

    use crate::parsing::helpers::block::strip_to_block_name;

    use super::{effective_line, get_block_data};

    #[test]
    fn test_effective_line() {
        let mut input = ["  C 000 #000 000  #00\n", "  C 0 0 0\n"];
        input.iter_mut().for_each(|s| {
            let s = effective_line(s);
            println!("{:?}", s)
        });
        let mut input = "  C  0.0756034347004260  0.0756034355668187  0.5000000004346841
  C  0.1496332166229109  0.1496332194727908  0.5000000000710555
  C  0.2145289813410105  0.2145289823390266  0.4999999994942101 ##
  #C  0.4243965644332829 -0.0000000008663758  0.5000000004346841
  C  0.3503667805273500 -0.0000000028498315  0.5000000000710555
  V  0.3934837276229430  0.6065302751523840  0.5001896946183580 SPIN=  2.0000000000
%ENDBLOCK POSITIONS_FRAC";
        let data = get_block_data(&mut input);
        println!("{:?}", data);
        let mut keyword = "%BLOCK POSITIONS_FRAC\r\n # Fractional COOR\n";
        println!("{:#?}", strip_to_block_name(&mut keyword));
        let mut input = "%BLOCK POSITIONS_FRAC
C  0.0756034347004260  0.0756034355668187  0.5000000004346841
  C  0.1496332166229109  0.1496332194727908  0.5000000000710555
  C  0.2145289813410105  0.2145289823390266  0.4999999994942101 ##
  #C  0.4243965644332829 -0.0000000008663758  0.5000000004346841
  C  0.3503667805273500 -0.0000000028498315  0.5000000000710555
  V  0.3934837276229430  0.6065302751523840  0.5001896946183580 SPIN=  2.0000000000
%ENDBLOCK POSITIONS_FRAC\r\n
\r\n";
        let keyword = strip_to_block_name(&mut input).unwrap();
        let data = get_block_data(&mut input).unwrap();
        println!("{:?}", keyword);
        println!("{}", data);
        println!("{}", input);
    }
}
