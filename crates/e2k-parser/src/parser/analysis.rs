use crate::parser::prelude::*;
use crate::parser::*;
use crate::parser::primitives::*;

/// Parse load pattern
fn parse_load_pattern(input: &str) -> IResult<&str, LoadPattern> {
    let (input, _) = tag("LOADPATTERN").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("TYPE").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, load_type) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("SELFWEIGHT").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, self_weight) = number.parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, LoadPattern {
        name,
        load_type,
        self_weight,
    }))
}

/// Parse load patterns section
fn parse_load_patterns(input: &str) -> IResult<&str, Vec<LoadPattern>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ LOAD PATTERNS").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    let (input, patterns) = many0(alt((
        terminated(
            parse_load_pattern,
            (opt(line_ending), skip_ws_and_comments)
        ),
        terminated(
            (identifier, take_while(|c| c != '\n' && c != '\r')),
            (opt(line_ending), skip_ws_and_comments)
        ).map(|_| LoadPattern {
            name: String::new(),
            load_type: String::new(),
            self_weight: 0.0,
        })
    ))).parse(input)?;

    Ok((input, patterns.into_iter().filter(|p| !p.name.is_empty()).collect()))
}

/// Parse load case
fn parse_load_case(input: &str) -> IResult<&str, LoadCase> {
    let (input, _) = tag("LOADCASE").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("TYPE").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, case_type) = quoted_string.parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, LoadCase {
        name,
        case_type,
        init_cond: String::new(),
        load_patterns: vec![],
        properties: vec![],
    }))
}

/// Parse load cases section
fn parse_load_cases(input: &str) -> IResult<&str, Vec<LoadCase>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ LOAD CASES").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many0(
        terminated(
            parse_load_case,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}