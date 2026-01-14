use crate::parser::prelude::*;
use crate::parser::*;
use crate::parser::primitives::*;

/// Parse point
fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("POINT").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, id) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, x) = number.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, y) = number.parse(input)?;
    let (input, z) = opt(preceded(ws, number)).parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, Point { id, x, y, z }))
}

/// Parse points section
fn parse_points(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ POINT COORDINATES").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_point,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

/// Parse line
fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("LINE").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, id) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, line_type) = alt((
        tag("COLUMN").map(|s: &str| s.to_string()),
        tag("BEAM").map(|s: &str| s.to_string()),
    )).parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, point_i) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, point_j) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, cardinal_point) = digit1.map(|s: &str| s.parse().ok()).parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, Line {
        id,
        line_type,
        point_i,
        point_j,
        cardinal_point,
    }))
}

/// Parse lines section
fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ LINE CONNECTIVITIES").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_line,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

/// Parse area
fn parse_area(input: &str) -> IResult<&str, Area> {
    let (input, _) = tag("AREA").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, id) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, area_type) = alt((
        tag("PANEL").map(|s: &str| s.to_string()),
        tag("FLOOR").map(|s: &str| s.to_string()),
    )).parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, num_joints) = digit1.map(|s: &str| s.parse().unwrap_or(0)).parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, Area {
        id,
        area_type,
        num_joints,
        points: vec![],
    }))
}

/// Parse areas section
fn parse_areas(input: &str) -> IResult<&str, Vec<Area>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ AREA CONNECTIVITIES").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_area,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}