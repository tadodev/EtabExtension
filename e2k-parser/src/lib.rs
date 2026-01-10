pub struct E2KParser {
    content: String,
}

impl E2KParser {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn parse(&self) -> Result<E2KModel, String> {
        match parse_e2k(&self.content) {
            Ok((_, model)) => Ok(model),
            Err(e) => Err(format!("Parse error: {:?}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_units() {
        let input = r#"UNITS  "KIP"  "FT"  "F"  "#;
        let result = parse_units(input);
        assert!(result.is_ok());
        let (_, units) = result.unwrap();
        assert_eq!(units.force, "KIP");
        assert_eq!(units.length, "FT");
        assert_eq!(units.temperature, "F");
    }

    #[test]
    fn test_parse_story() {
        let input = r#"STORY "ROOF"  HEIGHT 15"#;
        let result = parse_story(input);
        assert!(result.is_ok());
        let (_, story) = result.unwrap();
        assert_eq!(story.name, "ROOF");
        assert_eq!(story.height, Some(15.0));
    }

    #[test]
    fn test_parse_point() {
        let input = r#"POINT "25"  0 5.999823E-11"#;
        let result = parse_point(input);
        assert!(result.is_ok());
        let (_, point) = result.unwrap();
        assert_eq!(point.id, "25");
        assert_eq!(point.x, 0.0);
    }

    #[test]
    fn test_parse_load_pattern() {
        let input = r#"LOADPATTERN "Dead"  TYPE  "Dead"  SELFWEIGHT  1"#;
        let result = parse_load_pattern(input);
        assert!(result.is_ok());
        let (_, pattern) = result.unwrap();
        assert_eq!(pattern.name, "Dead");
        assert_eq!(pattern.load_type, "Dead");
        assert_eq!(pattern.self_weight, 1.0);
    }

    #[test]
    fn test_parse_grid() {
        let input = r#"GENGRID "G1"  LABEL "1"  X1 0 Y1 -30 X2 0 Y2 150 VISIBLE "Yes"  BUBBLELOC "End""#;
        let result = parse_grid(input);
        assert!(result.is_ok());
        let (_, grid) = result.unwrap();
        assert_eq!(grid.system, "G1");
        assert_eq!(grid.label, "1");
        assert_eq!(grid.x1, 0.0);
        assert_eq!(grid.y1, -30.0);
    }

    #[test]
    fn test_parse_line() {
        let input = r#"LINE  "C1"  COLUMN  "25"  "25"  1"#;
        let result = parse_line(input);
        assert!(result.is_ok());
        let (_, line) = result.unwrap();
        assert_eq!(line.id, "C1");
        assert_eq!(line.line_type, "COLUMN");
        assert_eq!(line.point_i, "25");
        assert_eq!(line.point_j, "25");
    }

    #[test]
    fn test_parse_material() {
        let input = r#"MATERIAL  "A992Fy50"    TYPE "Steel"    GRADE "Grade 50""#;
        let result = parse_material(input);
        assert!(result.is_ok());
        let (_, material) = result.unwrap();
        assert_eq!(material.name, "A992Fy50");
        assert_eq!(material.material_type, "Steel");
    }

    #[test]
    fn test_parse_rebar_definition() {
        let input = r#"REBARDEFINITION  "#2"  AREA  0.00034722223 DIA  0.020833334"#;
        let result = parse_rebar_definition(input);
        assert!(result.is_ok());
        let (_, rebar) = result.unwrap();
        assert_eq!(rebar.name, "#2");
        assert_eq!(rebar.area, 0.00034722223);
    }

    #[test]
    fn test_parse_frame_section() {
        let input = r#"FRAMESECTION  "C30x48_8ksi"  MATERIAL "8000Psi"  SHAPE "Concrete Rectangular""#;
        let result = parse_frame_section(input);
        assert!(result.is_ok());
        let (_, section) = result.unwrap();
        assert_eq!(section.name, "C30x48_8ksi");
        assert_eq!(section.material, "8000Psi");
        assert_eq!(section.shape, "Concrete Rectangular");
    }

    #[test]
    fn test_parse_diaphragm() {
        let input = r#"DIAPHRAGM "D1"    TYPE SEMIRIGID"#;
        let result = parse_diaphragm(input);
        assert!(result.is_ok());
        let (_, diaphragm) = result.unwrap();
        assert_eq!(diaphragm.name, "D1");
        assert_eq!(diaphragm.diaphragm_type, "SEMIRIGID");
    }

    #[test]
    fn test_parse_load_combination() {
        let input = r#"COMBO "_DL"  TYPE "Linear Add""#;
        let result = parse_load_combination(input);
        assert!(result.is_ok());
        let (_, combo) = result.unwrap();
        assert_eq!(combo.name, "_DL");
        assert_eq!(combo.combo_type, "Linear Add");
    }

    #[test]
    fn test_full_parse() {
        let sample = r#"$ File test.e2k saved 1/4/2026 10:31:42 PM

$ PROGRAM INFORMATION
  PROGRAM  "ETABS"  VERSION "22.7.0"

$ CONTROLS
  UNITS  "KIP"  "FT"  "F"

$ STORIES - IN SEQUENCE FROM TOP
  STORY "ROOF"  HEIGHT 15
  STORY "L01"  HEIGHT 25

$ POINT COORDINATES
  POINT "1"  0 0
  POINT "2"  30 0

$ LOAD PATTERNS
  LOADPATTERN "Dead"  TYPE  "Dead"  SELFWEIGHT  1

$ LOAD CASES
  LOADCASE "Dead"  TYPE  "Linear Static"  INITCOND  "PRESET"

$ END OF MODEL FILE
"#;

        let parser = E2KParser::new(sample.to_string());
        let result = parser.parse();
        assert!(result.is_ok());
        let model = result.unwrap();

        assert!(model.program_info.is_some());
        assert_eq!(model.stories.len(), 2);
        assert_eq!(model.points.len(), 2);
        assert_eq!(model.load_patterns.len(), 1);
    }
}
