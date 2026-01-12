// ============================================================================
// Basic Parsers
// ============================================================================
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{
        alpha1, alphanumeric1, char, digit1, line_ending, multispace1, space0, space1,
    },
    combinator::{map, opt, recognize, value},
    multi::many0,
    IResult,
    Parser,
};

// --- Whitespace helpers ---

fn ws(input: &str) -> IResult<&str, &str> {
    space0(input)
}

fn ws1(input: &str) -> IResult<&str, &str> {
    space1(input)
}

// --- Quoted string ---

fn quoted_string(input: &str) -> IResult<&str, String> {
    let (input, _) = char('"')(input)?;
    let (input, content) = take_until("\"")(input)?;
    let (input, _) = char('"')(input)?;
    Ok((input, content.to_string()))
}

// --- Identifier ---
// Accepts: A-Z a-z _ as first char
// Then: A-Z a-z 0-9 _ - .

fn identifier(input: &str) -> IResult<&str, String> {
    let (input, id) = recognize((
        alt((alpha1, tag("_"))),
        many0(alt((
            alphanumeric1,
            tag("_"),
            tag("-"),
            tag("."),
        ))),
    )).parse(input)?;
    Ok((input, id.to_string()))
}

// --- Number parser ---
// Supports: 123, -45.6, +3.14e-2 etc.

fn number(input: &str) -> IResult<&str, f64> {
    let (input, sign) = opt(alt((char('+'), char('-')))).parse(input)?;

    let (input, digits) = recognize((
        digit1,
        opt((char('.'), opt(digit1))),
        opt((
            alt((char('e'), char('E'))),
            opt(alt((char('+'), char('-')))),
            digit1,
        )),
    )).parse(input)?;

    let mut num_str = String::new();
    if let Some(s) = sign {
        num_str.push(s);
    }
    num_str.push_str(digits);

    let value = num_str.parse::<f64>().unwrap_or(0.0);
    Ok((input, value))
}

// --- Comment line ---
// Lines starting with $ until newline

fn comment_line(input: &str) -> IResult<&str, ()> {
    let (input, _) = char('$')(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;
    let (input, _) = opt(line_ending).parse(input)?;
    Ok((input, ()))
}

// --- Skip whitespace & comments ---

fn skip_whitespace_and_comments(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(alt((
        value((), multispace1),
        comment_line,
    ))).parse(input)?;
    Ok((input, ()))
}

// --- Key = Value parser ---
// Example:  NAME "My Model"
// Example:  UNITS kN_m

fn parse_key_value(input: &str) -> IResult<&str, (String, String)> {
    let (input, key) = identifier(input)?;
    let (input, _) = ws(input)?;

    let (input, value) = alt((
        quoted_string,
        map(
            take_while1(|c: char| !c.is_whitespace() && c != '"'),
            |s: &str| s.to_string(),
        ),
    )).parse(input)?;

    Ok((input, (key, value)))
}