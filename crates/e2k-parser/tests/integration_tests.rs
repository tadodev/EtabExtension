#[cfg(test)]
mod tests {
    use e2k_parser::E2KModel;
    use super::*;

    const SAMPLE_E2K: &str = r#"$ File test.e2k saved 1/4/2026 10:31:42 PM

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
"#;

    #[test]
    fn test_parse_sample() {
        let result = E2KModel::from_str(SAMPLE_E2K);
        assert!(result.is_ok());

        let model = result.unwrap();
        assert_eq!(model.stories.len(), 2);
        assert_eq!(model.points.len(), 2);
        assert_eq!(model.load_patterns.len(), 1);
    }

    #[test]
    fn test_model_statistics() {
        let model = E2KModel::from_str(SAMPLE_E2K).unwrap();
        let stats = model.get_statistics();

        assert_eq!(stats.num_stories, 2);
        assert_eq!(stats.num_points, 2);
        assert_eq!(stats.total_height, 40.0); // 15 + 25
    }

    #[test]
    fn test_validation() {
        let model = E2KModel::from_str(SAMPLE_E2K).unwrap();
        let result = model.validate();
        assert!(result.is_ok());
    }
}