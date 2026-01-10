// ============================================================================
// Basic Parsers
// ============================================================================

fn ws(input: &str) -> IResult<&str, &str> {
    space0(input)
}

fn ws1(input: &str) -> IResult<&str, &str> {
    space1(input)
}

fn quoted_string(input: &str) -> IResult<&str, String> {
    let (input, _) = char('"')(input)?;
    let (input, content) = take_until("\"")(input)?;
    let (input, _) = char('"')(input)?;
    Ok((input, content.to_string()))
}

fn identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"), tag("-"), tag("."))))
        )),
        |s: &str| s.to_string()
    )(input)
}

fn number(input: &str) -> IResult<&str, f64> {
    let (input, sign) = opt(alt((char('+'), char('-'))))(input)?;
    let (input, digits) = recognize(
        tuple((
            digit1,
            opt(pair(char('.'), opt(digit1))),
            opt(tuple((
                alt((char('e'), char('E'))),
                opt(alt((char('+'), char('-')))),
                digit1
            )))
        ))
    )(input)?;

    let mut num_str = String::new();
    if let Some(s) = sign {
        num_str.push(s);
    }
    num_str.push_str(digits);

    let value = num_str.parse::<f64>().unwrap_or(0.0);
    Ok((input, value))
}

fn comment_line(input: &str) -> IResult<&str, ()> {
    let (input, _) = char('$')(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, ()))
}

fn skip_whitespace_and_comments(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(alt((
        value((), multispace1),
        comment_line,
    )))(input)?;
    Ok((input, ()))
}

fn parse_key_value(input: &str) -> IResult<&str, (String, String)> {
    let (input, key) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, value) = alt((
        quoted_string,
        map(
            take_while1(|c: char| !c.is_whitespace() && c != '"'),
            |s: &str| s.to_string()
        )
    ))(input)?;
    Ok((input, (key, value)))
}

// ============================================================================
// Section Parsers
// ============================================================================

fn parse_file_info(input: &str) -> IResult<&str, FileInfo> {
    let (input, _) = tag("$ File ")(input)?;
    let (input, path) = take_until(" saved")(input)?;
    let (input, _) = tag(" saved ")(input)?;
    let (input, date) = take_until("\n")(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, FileInfo {
        path: path.to_string(),
        saved_date: date.trim().to_string(),
    }))
}

fn parse_program_info(input: &str) -> IResult<&str, ProgramInfo> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ PROGRAM INFORMATION")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("PROGRAM")(input)?;
    let (input, _) = ws(input)?;
    let (input, program) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("VERSION")(input)?;
    let (input, _) = ws(input)?;
    let (input, version) = quoted_string(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, ProgramInfo { program, version }))
}

fn parse_units(input: &str) -> IResult<&str, Units> {
    let (input, _) = tag("UNITS")(input)?;
    let (input, _) = ws(input)?;
    let (input, force) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, length) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, temperature) = quoted_string(input)?;

    Ok((input, Units { force, length, temperature }))
}

fn parse_controls(input: &str) -> IResult<&str, Controls> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ CONTROLS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, units) = parse_units(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;
    let (input, _) = opt(line_ending)(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    // Parse optional lines
    let (input, _) = many0(terminated(
        tuple((identifier, take_while(|c| c != '\n' && c != '\r'))),
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, Controls {
        units,
        title1: None,
        title2: None,
        preference: None,
        rllf: None,
    }))
}

fn parse_story(input: &str) -> IResult<&str, Story> {
    let (input, _) = tag("STORY")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = ws(input)?;

    let (input, (height, elevation)) = alt((
        map(
            preceded(tuple((tag("HEIGHT"), ws)), number),
            |h| (Some(h), None)
        ),
        map(
            preceded(tuple((tag("ELEV"), ws)), number),
            |e| (None, Some(e))
        ),
    ))(input)?;

    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, Story { name, height, elevation }))
}

fn parse_stories(input: &str) -> IResult<&str, Vec<Story>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ STORIES")(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, stories) = many1(terminated(
        parse_story,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, stories))
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (input, _) = tag("GENGRID")(input)?;
    let (input, _) = ws(input)?;
    let (input, system) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("LABEL")(input)?;
    let (input, _) = ws(input)?;
    let (input, label) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("X1")(input)?;
    let (input, _) = ws(input)?;
    let (input, x1) = number(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("Y1")(input)?;
    let (input, _) = ws(input)?;
    let (input, y1) = number(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("X2")(input)?;
    let (input, _) = ws(input)?;
    let (input, x2) = number(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("Y2")(input)?;
    let (input, _) = ws(input)?;
    let (input, y2) = number(input)?;

    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

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

fn parse_grids(input: &str) -> IResult<&str, Vec<Grid>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ GRIDS")(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, _) = opt(tuple((
        tag("GRIDSYSTEM"),
        take_until("\n"),
        line_ending,
        skip_whitespace_and_comments
    )))(input)?;

    let (input, grids) = many1(terminated(
        parse_grid,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, grids))
}

fn parse_diaphragm(input: &str) -> IResult<&str, Diaphragm> {
    let (input, _) = tag("DIAPHRAGM")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("TYPE")(input)?;
    let (input, _) = ws(input)?;
    let (input, diaphragm_type) = identifier(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, Diaphragm { name, diaphragm_type }))
}

fn parse_diaphragms(input: &str) -> IResult<&str, Vec<Diaphragm>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ DIAPHRAGM NAMES")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, diaphragms) = many1(terminated(
        parse_diaphragm,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, diaphragms))
}

fn parse_material(input: &str) -> IResult<&str, Material> {
    let (input, _) = tag("MATERIAL")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = ws(input)?;

    let (input, material_type) = opt(preceded(
        tuple((tag("TYPE"), ws)),
        quoted_string
    ))(input)?;

    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

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

fn parse_materials(input: &str) -> IResult<&str, Vec<Material>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ MATERIAL PROPERTIES")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, materials) = many1(terminated(
        parse_material,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, materials))
}

fn parse_rebar_definition(input: &str) -> IResult<&str, RebarDefinition> {
    let (input, _) = tag("REBARDEFINITION")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("AREA")(input)?;
    let (input, _) = ws(input)?;
    let (input, area) = number(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("DIA")(input)?;
    let (input, _) = ws(input)?;
    let (input, diameter) = number(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, RebarDefinition { name, area, diameter }))
}

fn parse_rebar_definitions(input: &str) -> IResult<&str, Vec<RebarDefinition>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ REBAR DEFINITIONS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, rebars) = many1(terminated(
        parse_rebar_definition,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, rebars))
}

fn parse_frame_section(input: &str) -> IResult<&str, FrameSection> {
    let (input, _) = tag("FRAMESECTION")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("MATERIAL")(input)?;
    let (input, _) = ws(input)?;
    let (input, material) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("SHAPE")(input)?;
    let (input, _) = ws(input)?;
    let (input, shape) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, FrameSection {
        name,
        material,
        shape,
        dimensions: vec![],
        modifiers: vec![],
    }))
}

fn parse_frame_sections(input: &str) -> IResult<&str, Vec<FrameSection>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ FRAME SECTIONS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, sections) = many1(terminated(
        parse_frame_section,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, sections))
}

fn parse_concrete_section(input: &str) -> IResult<&str, ConcreteSection> {
    let (input, _) = tag("CONCRETESECTION")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, ConcreteSection {
        name,
        long_bar_material: String::new(),
        confine_bar_material: String::new(),
        section_type: String::new(),
        properties: vec![],
    }))
}

fn parse_concrete_sections(input: &str) -> IResult<&str, Vec<ConcreteSection>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ CONCRETE SECTIONS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, sections) = many1(terminated(
        parse_concrete_section,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, sections))
}

fn parse_tendon_section(input: &str) -> IResult<&str, TendonSection> {
    let (input, _) = tag("TENDONSECTION")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, TendonSection {
        name,
        material: String::new(),
        strand_area: 0.0,
    }))
}

fn parse_shell_prop(input: &str) -> IResult<&str, ShellProp> {
    let (input, _) = tag("SHELLPROP")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

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

fn parse_shell_props(input: &str) -> IResult<&str, Vec<ShellProp>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = alt((
        tag("$ SLAB PROPERTIES"),
        tag("$ DECK PROPERTIES"),
        tag("$ WALL PROPERTIES"),
    ))(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, props) = many1(terminated(
        parse_shell_prop,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, props))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("POINT")(input)?;
    let (input, _) = ws(input)?;
    let (input, id) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, x) = number(input)?;
    let (input, _) = ws(input)?;
    let (input, y) = number(input)?;
    let (input, z) = opt(preceded(ws, number))(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, Point { id, x, y, z }))
}

fn parse_points(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ POINT COORDINATES")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, points) = many1(terminated(
        parse_point,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, points))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("LINE")(input)?;
    let (input, _) = ws(input)?;
    let (input, id) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, line_type) = alt((
        map(tag("COLUMN"), |s: &str| s.to_string()),
        map(tag("BEAM"), |s: &str| s.to_string()),
    ))(input)?;
    let (input, _) = ws(input)?;
    let (input, point_i) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, point_j) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, cardinal_point) = map(digit1, |s: &str| s.parse().ok())(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, Line {
        id,
        line_type,
        point_i,
        point_j,
        cardinal_point,
    }))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ LINE CONNECTIVITIES")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, lines) = many1(terminated(
        parse_line,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, lines))
}

fn parse_area(input: &str) -> IResult<&str, Area> {
    let (input, _) = tag("AREA")(input)?;
    let (input, _) = ws(input)?;
    let (input, id) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, area_type) = alt((
        map(tag("PANEL"), |s: &str| s.to_string()),
        map(tag("FLOOR"), |s: &str| s.to_string()),
    ))(input)?;
    let (input, _) = ws(input)?;
    let (input, num_joints) = map(digit1, |s: &str| s.parse().unwrap_or(0))(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, Area {
        id,
        area_type,
        num_joints,
        points: vec![],
    }))
}

fn parse_areas(input: &str) -> IResult<&str, Vec<Area>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ AREA CONNECTIVITIES")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, areas) = many1(terminated(
        parse_area,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, areas))
}

fn parse_group_name(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("GROUP")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;
    Ok((input, name))
}

fn parse_groups(input: &str) -> IResult<&str, Vec<Group>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ GROUPS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, names) = many1(terminated(
        parse_group_name,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, names.into_iter().map(|name| Group {
        name,
        members: vec![],
    }).collect()))
}

fn parse_point_assign(input: &str) -> IResult<&str, PointAssign> {
    let (input, _) = tag("POINTASSIGN")(input)?;
    let (input, _) = ws(input)?;
    let (input, point) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, story) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, PointAssign {
        point,
        story,
        restraint: None,
        user_joint: None,
    }))
}

fn parse_point_assigns(input: &str) -> IResult<&str, Vec<PointAssign>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ POINT ASSIGNS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, assigns) = many1(terminated(
        parse_point_assign,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, assigns))
}

fn parse_line_assign(input: &str) -> IResult<&str, LineAssign> {
    let (input, _) = tag("LINEASSIGN")(input)?;
    let (input, _) = ws(input)?;
    let (input, line) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, story) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("SECTION")(input)?;
    let (input, _) = ws(input)?;
    let (input, section) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, LineAssign {
        line,
        story,
        section,
        angle: None,
        cardinal_point: None,
        release: None,
        properties: vec![],
    }))
}

fn parse_line_assigns(input: &str) -> IResult<&str, Vec<LineAssign>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ LINE ASSIGNS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, assigns) = many1(terminated(
        parse_line_assign,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, assigns))
}

fn parse_area_assign(input: &str) -> IResult<&str, AreaAssign> {
    let (input, _) = tag("AREAASSIGN")(input)?;
    let (input, _) = ws(input)?;
    let (input, area) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, story) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("SECTION")(input)?;
    let (input, _) = ws(input)?;
    let (input, section) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, AreaAssign {
        area,
        story,
        section,
        properties: vec![],
    }))
}

fn parse_area_assigns(input: &str) -> IResult<&str, Vec<AreaAssign>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ AREA ASSIGNS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, assigns) = many1(terminated(
        parse_area_assign,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, assigns))
}

fn parse_load_pattern(input: &str) -> IResult<&str, LoadPattern> {
    let (input, _) = tag("LOADPATTERN")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("TYPE")(input)?;
    let (input, _) = ws(input)?;
    let (input, load_type) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("SELFWEIGHT")(input)?;
    let (input, _) = ws(input)?;
    let (input, self_weight) = number(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, LoadPattern {
        name,
        load_type,
        self_weight,
    }))
}

fn parse_load_patterns(input: &str) -> IResult<&str, Vec<LoadPattern>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ LOAD PATTERNS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, patterns) = many0(alt((
        terminated(
            parse_load_pattern,
            tuple((opt(line_ending), skip_whitespace_and_comments))
        ),
        map(
            terminated(
                tuple((identifier, take_while(|c| c != '\n' && c != '\r'))),
                tuple((opt(line_ending), skip_whitespace_and_comments))
            ),
            |_| LoadPattern {
                name: String::new(),
                load_type: String::new(),
                self_weight: 0.0,
            }
        )
    )))(input)?;

    Ok((input, patterns.into_iter().filter(|p| !p.name.is_empty()).collect()))
}

fn parse_line_load(input: &str) -> IResult<&str, LineLoad> {
    let (input, _) = tag("LINELOAD")(input)?;
    let (input, _) = ws(input)?;
    let (input, line) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, story) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, LineLoad {
        line,
        story,
        load_pattern: String::new(),
        load_type: String::new(),
        direction: String::new(),
        values: vec![],
    }))
}

fn parse_shell_uniform_load_set(input: &str) -> IResult<&str, ShellUniformLoadSet> {
    let (input, _) = tag("SHELLUNIFORMLOADSET")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("LOADPAT")(input)?;
    let (input, _) = ws(input)?;
    let (input, load_pattern) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("VALUE")(input)?;
    let (input, _) = ws(input)?;
    let (input, value) = number(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, ShellUniformLoadSet {
        name,
        load_pattern,
        value,
    }))
}

fn parse_area_load(input: &str) -> IResult<&str, AreaLoad> {
    let (input, _) = tag("AREALOAD")(input)?;
    let (input, _) = ws(input)?;
    let (input, area) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, story) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, AreaLoad {
        area,
        story,
        load_type: String::new(),
        load_set: String::new(),
    }))
}

fn parse_load_case(input: &str) -> IResult<&str, LoadCase> {
    let (input, _) = tag("LOADCASE")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("TYPE")(input)?;
    let (input, _) = ws(input)?;
    let (input, case_type) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, LoadCase {
        name,
        case_type,
        init_cond: String::new(),
        load_patterns: vec![],
        properties: vec![],
    }))
}

fn parse_load_cases(input: &str) -> IResult<&str, Vec<LoadCase>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ LOAD CASES")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, cases) = many0(terminated(
        parse_load_case,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, cases))
}

fn parse_load_combination(input: &str) -> IResult<&str, LoadCombination> {
    let (input, _) = tag("COMBO")(input)?;
    let (input, _) = ws(input)?;
    let (input, name) = quoted_string(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("TYPE")(input)?;
    let (input, _) = ws(input)?;
    let (input, combo_type) = quoted_string(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, LoadCombination {
        name,
        combo_type,
        cases: vec![],
    }))
}

fn parse_load_combinations(input: &str) -> IResult<&str, Vec<LoadCombination>> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ LOAD COMBINATIONS")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;

    let (input, combos) = many0(terminated(
        parse_load_combination,
        tuple((opt(line_ending), skip_whitespace_and_comments))
    ))(input)?;

    Ok((input, combos))
}

fn parse_project_info(input: &str) -> IResult<&str, ProjectInfo> {
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("$ PROJECT INFORMATION")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = skip_whitespace_and_comments(input)?;
    let (input, _) = tag("PROJECTINFO")(input)?;
    let (input, _) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((input, ProjectInfo {
        company_name: String::new(),
        model_name: String::new(),
    }))
}
