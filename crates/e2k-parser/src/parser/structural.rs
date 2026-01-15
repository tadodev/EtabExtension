use crate::parser::prelude::*;
use crate::parser::*;
// ============================================================================
// Section Parsers
// ============================================================================

/// Parse file info header
pub fn parse_file_info(input: &str) -> IResult<&str, FileInfo> {
    let (input, _) = tag("$ File ").parse(input)?;
    let (input, path) = take_until(" saved").parse(input)?;
    let (input, _) = tag(" saved ").parse(input)?;
    let (input, date) = take_until("\n").parse(input)?;
    let (input, _) = line_ending.parse(input)?;

    Ok((input, FileInfo {
        path: path.to_string(),
        saved_date: date.trim().to_string(),
    }))
}

/// Parse program information section
pub fn parse_program_info(input: &str) -> IResult<&str, ProgramInfo> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ PROGRAM INFORMATION").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("PROGRAM").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, program) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("VERSION").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, version) = quoted_string.parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    Ok((input, ProgramInfo { program, version }))
}

/// Parse units
pub fn parse_units(input: &str) -> IResult<&str, Units> {
    let (input, _) = tag("UNITS").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, force) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, length) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, temperature) = quoted_string.parse(input)?;

    Ok((input, Units { force, length, temperature }))
}

/// Parse controls section
pub fn parse_controls(input: &str) -> IResult<&str, Controls> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ CONTROLS").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, units) = parse_units.parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    // Skip remaining control lines
    let (input, _) = many0(
        terminated(
            (identifier, take_while(|c| c != '\n' && c != '\r')),
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)?;

    Ok((input, Controls {
        units,
        title1: None,
        title2: None,
        preference: None,
        rllf: None,
    }))
}

/// Parse single story
pub fn parse_story(input: &str) -> IResult<&str, Story> {
    let (input, _) = tag("STORY").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;

    let (input, (height, elevation)) = alt((
        preceded(
            (tag("HEIGHT"), ws),
            number
        ).map(|h| (Some(h), None)),
        preceded(
            (tag("ELEV"), ws),
            number
        ).map(|e| (None, Some(e))),
    )).parse(input)?;

    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, Story { name, height, elevation }))
}

/// Parse stories section
pub fn parse_stories(input: &str) -> IResult<&str, Vec<Story>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ STORIES").parse(input)?;
    let (input, _) = take_until("\n").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_story,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

/// Parse single grid line
pub fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (input, _) = tag("GENGRID").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, system) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("LABEL").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, label) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("X1").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, x1) = number.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("Y1").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, y1) = number.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("X2").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, x2) = number.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("Y2").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, y2) = number.parse(input)?;

    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, Grid {
        system,
        label,
        x1,
        y1,
        x2,
        y2,
        visible: None,
        bubble_loc: None,
    }))
}

/// Parse grids section
pub fn parse_grids(input: &str) -> IResult<&str, Vec<Grid>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ GRIDS").parse(input)?;
    let (input, _) = take_until("\n").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    let (input, _) = opt(
        (
            tag("GRIDSYSTEM"),
            take_until("\n"),
            line_ending,
            skip_ws_and_comments
        )
    ).parse(input)?;

    many1(
        terminated(
            parse_grid,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

/// Parse diaphragm
pub fn parse_diaphragm(input: &str) -> IResult<&str, Diaphragm> {
    let (input, _) = tag("DIAPHRAGM").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("TYPE").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, diaphragm_type) = identifier.parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, Diaphragm { name, diaphragm_type }))
}

/// Parse diaphragms section
pub fn parse_diaphragms(input: &str) -> IResult<&str, Vec<Diaphragm>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ DIAPHRAGM NAMES").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_diaphragm,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

/// Parse material
pub fn parse_material(input: &str) -> IResult<&str, Material> {
    let (input, _) = tag("MATERIAL").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;

    let (input, material_type) = opt(
        preceded(
            (tag("TYPE"), ws),
            quoted_string
        )
    ).parse(input)?;

    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, Material {
        name,
        material_type: material_type.unwrap_or_default(),
        grade: None,
        weight_per_volume: None,
        sym_type: None,
        e: None,
        u: None,
        a: None,
        fy: None,
        fu: None,
        fc: None,
        properties: vec![],
    }))
}

/// Parse materials section
pub fn parse_materials(input: &str) -> IResult<&str, Vec<Material>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ MATERIAL PROPERTIES").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_material,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

/// Parse rebar definition
pub fn parse_rebar_definition(input: &str) -> IResult<&str, RebarDefinition> {
    let (input, _) = tag("REBARDEFINITION").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("AREA").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, area) = number.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("DIA").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, diameter) = number.parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, RebarDefinition { name, area, diameter }))
}

/// Parse rebar definitions section
pub fn parse_rebar_definitions(input: &str) -> IResult<&str, Vec<RebarDefinition>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ REBAR DEFINITIONS").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_rebar_definition,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

/// Parse frame section
pub fn parse_frame_section(input: &str) -> IResult<&str, FrameSection> {
    let (input, _) = tag("FRAMESECTION").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("MATERIAL").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, material) = quoted_string.parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, _) = tag("SHAPE").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, shape) = quoted_string.parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, FrameSection {
        name,
        material,
        shape,
        dimensions: vec![],
        modifiers: vec![],
    }))
}

/// Parse frame sections
pub fn parse_frame_sections(input: &str) -> IResult<&str, Vec<FrameSection>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ FRAME SECTIONS").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_frame_section,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

/// Parse concrete section
pub fn parse_concrete_section(input: &str) -> IResult<&str, ConcreteSection> {
    let (input, _) = tag("CONCRETESECTION").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, ConcreteSection {
        name,
        long_bar_material: String::new(),
        confine_bar_material: String::new(),
        section_type: String::new(),
        properties: vec![],
    }))
}

/// Parse concrete sections
pub fn parse_concrete_sections(input: &str) -> IResult<&str, Vec<ConcreteSection>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = tag("$ CONCRETE SECTIONS").parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_concrete_section,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}

/// Parse shell property
pub fn parse_shell_prop(input: &str) -> IResult<&str, ShellProp> {
    let (input, _) = tag("SHELLPROP").parse(input)?;
    let (input, _) = ws.parse(input)?;
    let (input, name) = quoted_string.parse(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r').parse(input)?;

    Ok((input, ShellProp {
        name,
        prop_type: String::new(),
        material: String::new(),
        modeling_type: None,
        thickness: None,
        slab_type: None,
        wall_thickness: None,
        modifiers: vec![],
        notes: None,
    }))
}

/// Parse shell properties section
pub fn parse_shell_props(input: &str) -> IResult<&str, Vec<ShellProp>> {
    let (input, _) = skip_ws_and_comments.parse(input)?;
    let (input, _) = alt((
        tag("$ SLAB PROPERTIES"),
        tag("$ DECK PROPERTIES"),
        tag("$ WALL PROPERTIES"),
    )).parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, _) = skip_ws_and_comments.parse(input)?;

    many1(
        terminated(
            parse_shell_prop,
            (opt(line_ending), skip_ws_and_comments)
        )
    ).parse(input)
}