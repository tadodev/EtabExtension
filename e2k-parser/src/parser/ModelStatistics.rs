#[derive(Debug, Clone, PartialEq)]
pub struct ModelStatistics {
    pub num_stories: usize,
    pub num_points: usize,
    pub num_lines: usize,
    pub num_areas: usize,
    pub num_columns: usize,
    pub num_beams: usize,
    pub num_walls: usize,
    pub num_floors: usize,
    pub num_materials: usize,
    pub num_load_patterns: usize,
    pub num_load_cases: usize,
    pub num_load_combinations: usize,
    pub total_height: f64,
}

impl std::fmt::Display for ModelStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Model Statistics:\n")?;
        write!(f, "  Stories: {}\n", self.num_stories)?;
        write!(f, "  Total Height: {:.2} ft\n", self.total_height)?;
        write!(f, "  Points: {}\n", self.num_points)?;
        write!(f, "  Lines: {} (Columns: {}, Beams: {})\n",
               self.num_lines, self.num_columns, self.num_beams)?;
        write!(f, "  Areas: {} (Walls: {}, Floors: {})\n",
               self.num_areas, self.num_walls, self.num_floors)?;
        write!(f, "  Materials: {}\n", self.num_materials)?;
        write!(f, "  Load Patterns: {}\n", self.num_load_patterns)?;
        write!(f, "  Load Cases: {}\n", self.num_load_cases)?;
        write!(f, "  Load Combinations: {}\n", self.num_load_combinations)?;
        Ok(())
    }
}