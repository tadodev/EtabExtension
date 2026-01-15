use crate::{types::*, error::{Result, E2kError}};
use nom::Parser;

use crate::parser::prelude::*;
use crate::parser::primitives::*;

mod prelude;
mod primitives;
mod geometry;
mod loading;
mod analysis;
mod structural;

// Re-export for internal use
use structural::*;
use geometry::*;
use analysis::*;

/// Helper to take until pattern or end of input
fn take_until_or_end(pattern: &'static str) -> impl Fn(&str) -> nom::IResult<&str, &str> {
    use nom::bytes::complete::take_until;
    move |input: &str| {
        if let Ok((rest, content)) = take_until::<_, _, nom::error::Error<_>>(pattern).parse(input) {
            Ok((rest, content))
        } else {
            Ok(("", input))
        }
    }
}

/// Main E2K parser - orchestrates all section parsers
pub fn parse_e2k(content: &str) -> Result<E2KModel> {
    match parse_e2k_internal(content) {
        Ok((_, model)) => Ok(model),
        Err(e) => Err(E2kError::parsing_general(format!("Failed to parse E2K file: {:?}", e))),
    }
}

/// Internal parser implementation
fn parse_e2k_internal(input: &str) -> nom::IResult<&str, E2KModel> {
    let (input, file_info) = opt(parse_file_info).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, program_info) = opt(parse_program_info).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, controls) = opt(parse_controls).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, stories) = opt(parse_stories).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, grids) = opt(parse_grids).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, diaphragms) = opt(parse_diaphragms).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, materials) = opt(parse_materials).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, rebar_defs) = opt(parse_rebar_definitions).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, frame_sections) = opt(parse_frame_sections).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, concrete_sections) = opt(parse_concrete_sections).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    // Skip tendon sections
    let (input, _) = opt((
        tag("$ TENDON SECTIONS"),
        take_until_or_end("$ "),
    )).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    let (input, shell_props) = opt(parse_shell_props).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    // Skip additional shell prop sections
    let (input, _) = many0((
        opt(parse_shell_props),
        skip_ws_and_comments,
    )).parse(input)?;

    // Skip link and panel zone sections
    let (input, _) = opt((
        alt((
            tag("$ LINK PROPERTIES"),
            tag("$ PANEL ZONE PROPERTIES"),
            tag("$ PIER/SPANDREL NAMES"),
        )),
        take_until_or_end("$ "),
    )).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    let (input, points) = opt(parse_points).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, lines) = opt(parse_lines).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, areas) = opt(parse_areas).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    // Skip groups, assignments sections
    let (input, _) = many0((
        opt(alt((
            tag("$ GROUPS"),
            tag("$ POINT ASSIGNS"),
            tag("$ LINE ASSIGNS"),
            tag("$ AREA ASSIGNS"),
        ))),
        take_until_or_end("$ "),
        skip_ws_and_comments,
    )).parse(input)?;

    let (input, load_patterns) = opt(parse_load_patterns).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    // Skip various load sections
    let (input, _) = many0((
        opt(alt((
            tag("$ POINT OBJECT LOADS"),
            tag("$ FRAME OBJECT LOADS"),
            tag("$ SHELL UNIFORM LOAD SETS"),
            tag("$ SHELL OBJECT LOADS"),
            tag("$ ANALYSIS OPTIONS"),
            tag("$ MASS SOURCE"),
            tag("$ FUNCTIONS"),
            tag("$ GENERALIZED DISPLACEMENTS"),
        ))),
        take_until_or_end("$ "),
        skip_ws_and_comments,
    )).parse(input)?;

    let (input, load_cases) = opt(parse_load_cases).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, load_combos) = opt(parse_load_combinations).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    // Skip design preferences
    let (input, _) = many0((
        opt(alt((
            tag("$ GENERAL DESIGN PREFERENCES"),
            tag("$ STEEL DESIGN PREFERENCES"),
            tag("$ CONCRETE DESIGN PREFERENCES"),
            tag("$ COMPOSITE DESIGN PREFERENCES"),
            tag("$ COMPOSITE COLUMN DESIGN PREFERENCES"),
            tag("$ WALL DESIGN PREFERENCES"),
            tag("$ CONCRETE SLAB DESIGN PREFERENCES"),
            tag("$ TABLE SETS"),
        ))),
        take_until_or_end("$ "),
        skip_ws_and_comments,
    )).parse(input)?;

    let (input, project_info) = opt(parse_project_info).parse(input)?;

    Ok((input, E2KModel {
        file_info,
        program_info,
        controls,
        stories: stories.unwrap_or_default(),
        grids: grids.unwrap_or_default(),
        diaphragms: diaphragms.unwrap_or_default(),
        materials: materials.unwrap_or_default(),
        rebar_definitions: rebar_defs.unwrap_or_default(),
        frame_sections: frame_sections.unwrap_or_default(),
        concrete_sections: concrete_sections.unwrap_or_default(),
        tendon_sections: vec![],
        shell_props: shell_props.unwrap_or_default(),
        link_props: vec![],
        panel_zones: vec![],
        pier_names: vec![],
        spandrel_names: vec![],
        points: points.unwrap_or_default(),
        lines: lines.unwrap_or_default(),
        areas: areas.unwrap_or_default(),
        groups: vec![],
        point_assigns: vec![],
        line_assigns: vec![],
        area_assigns: vec![],
        load_patterns: load_patterns.unwrap_or_default(),
        point_loads: vec![],
        line_loads: vec![],
        shell_uniform_load_sets: vec![],
        area_loads: vec![],
        analysis_options: None,
        mass_source: None,
        functions: vec![],
        load_cases: load_cases.unwrap_or_default(),
        load_combinations: load_combos.unwrap_or_default(),
        design_preferences: DesignPreferences::default(),
        project_info,
    }))
}

/// Parse load combination
fn parse_load_combinations(input: &str) -> nom::IResult<&str, Vec<LoadCombination>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ LOAD COMBINATIONS").parse(input)?;
    let (input, _) = nom::character::complete::line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many0(
        nom::sequence::terminated(
            parse_load_combination,
            (opt(nom::character::complete::line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

fn parse_load_combination(input: &str) -> nom::IResult<&str, LoadCombination> {
    use nom::bytes::complete::take_while;

    let (input, _) = tag("COMBO").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("TYPE").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, combo_type) = quoted_string.parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, LoadCombination {
        name,
        combo_type,
        cases: vec![],
    }))
}

fn parse_project_info(input: &str) -> nom::IResult<&str, ProjectInfo> {
    use nom::bytes::complete::take_while;
    use nom::character::complete::line_ending;

    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ PROJECT INFORMATION").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("PROJECTINFO").parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, ProjectInfo {
        company_name: String::new(),
        model_name: String::new(),
    }))
}