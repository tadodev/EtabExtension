pub fn parse_e2k(input: &str) -> IResult<&str, E2KModel> {
    let (input, file_info) = opt(parse_file_info)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, program_info) = opt(parse_program_info)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, controls) = opt(parse_controls)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, stories) = opt(parse_stories)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, grids) = opt(parse_grids)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, diaphragms) = opt(parse_diaphragms)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, materials) = opt(parse_materials)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, rebar_defs) = opt(parse_rebar_definitions)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, frame_sections) = opt(parse_frame_sections)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, concrete_sections) = opt(parse_concrete_sections)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    // Skip tendon sections
    let (input, _) = opt(tuple((
        tag("$ TENDON SECTIONS"),
        take_until("$ "),
    )))(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, shell_props) = opt(parse_shell_props)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    // Skip additional shell prop sections
    let (input, _) = many0(tuple((
        opt(parse_shell_props),
        skip_whitespace_and_comments,
    )))(input)?;

    // Skip link and panel zone sections
    let (input, _) = opt(tuple((
        alt((
            tag("$ LINK PROPERTIES"),
            tag("$ PANEL ZONE PROPERTIES"),
            tag("$ PIER/SPANDREL NAMES"),
        )),
        take_until("$ "),
    )))(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, points) = opt(parse_points)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, lines) = opt(parse_lines)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, areas) = opt(parse_areas)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, groups) = opt(parse_groups)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, point_assigns) = opt(parse_point_assigns)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, line_assigns) = opt(parse_line_assigns)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, area_assigns) = opt(parse_area_assigns)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, load_patterns) = opt(parse_load_patterns)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    // Skip various load sections
    let (input, _) = many0(tuple((
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
        skip_whitespace_and_comments,
    )))(input)?;

    let (input, load_cases) = opt(parse_load_cases)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, load_combos) = opt(parse_load_combinations)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    // Skip design preferences
    let (input, _) = many0(tuple((
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
        skip_whitespace_and_comments,
    )))(input)?;

    let (input, project_info) = opt(parse_project_info)(input)?;

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
        groups: groups.unwrap_or_default(),
        point_assigns: point_assigns.unwrap_or_default(),
        line_assigns: line_assigns.unwrap_or_default(),
        area_assigns: area_assigns.unwrap_or_default(),
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

fn take_until_or_end(pattern: &'static str) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input: &str| {
        if let Ok((rest, content)) = take_until::<_, _, nom::error::Error<_>>(pattern)(input) {
            Ok((rest, content))
        } else {
            Ok(("", input))
        }
    }
}