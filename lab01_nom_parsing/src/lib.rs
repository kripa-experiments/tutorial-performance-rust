use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

/// Parses a parenthesized Lisp-style string and returns a vector of words.
/// Example: "(action arg1 arg2)" -> ["action", "arg1", "arg2"]
pub fn parse_lisp_string(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        tuple((tag("("), multispace0)),
        separated_list0(multispace1, alphanumeric1),
        tuple((multispace0, tag(")"))),
    )(input)
}
