use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{
        alpha1, alphanumeric1, char, digit1,
        line_ending, multispace1, space0, space1,
    },
    combinator::{opt, recognize, value},
    multi::many0,
    sequence::{pair},
    IResult,
    Parser,
};

/// whitespace
pub fn ws(input: &str) -> IResult<&str, &str> {
    space0.parse(input)
}

pub fn ws1(input: &str) -> IResult<&str, &str> {
    space1.parse(input)
}

/// quoted string
pub fn quoted_string(input: &str) -> IResult<&str, String> {
    let (input, _) = char('"').parse(input)?;
    let (input, content) = take_until("\"").parse(input)?;
    let (input, _) = char('"').parse(input)?;
    Ok((input, content.to_string()))
}

/// identifier
pub fn identifier(input: &str) -> IResult<&str, String> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_"), tag("-"), tag(".")))),
    ))
        .map(|s: &str| s.to_string())
        .parse(input)
}

/// floating number
pub fn number(input: &str) -> IResult<&str, f64> {
    let (input, sign) = opt(alt((char('+'), char('-')))).parse(input)?;

    let (input, digits) = recognize((
        digit1,
        opt(pair(char('.'), opt(digit1))),
        opt((
            alt((char('e'), char('E'))),
            opt(alt((char('+'), char('-')))),
            digit1,
        )),
    ))
        .parse(input)?;

    let mut s = String::new();
    if let Some(c) = sign {
        s.push(c);
    }
    s.push_str(digits);

    Ok((input, s.parse().unwrap_or(0.0)))
}

/// comment line
pub fn comment_line(input: &str) -> IResult<&str, ()> {
    let (input, _) = char('$').parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;
    Ok((input, ()))
}

/// skip whitespace + comments
pub fn skip_ws_and_comments(input: &str) -> IResult<&str, ()> {
    many0(alt((
        value((), multispace1),
        comment_line,
    )))
        .map(|_| ())
        .parse(input)
}

/// key = value
pub fn parse_key_value(input: &str) -> IResult<&str, (String, String)> {
    let (input, key) = identifier.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, value) = alt((
        quoted_string,
        take_while1(|c: char| !c.is_whitespace() && c != '"')
            .map(|s: &str| s.to_string()),
    ))
        .parse(input)?;

    Ok((input, (key, value)))
}
