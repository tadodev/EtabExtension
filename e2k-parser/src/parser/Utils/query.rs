// ============================================================================
// Helper Functions for Extended Parsing
// ============================================================================

impl E2KModel {
    /// Get all columns from the model
    pub fn get_columns(&self) -> Vec<&Line> {
        self.lines.iter()
            .filter(|line| line.line_type == "COLUMN")
            .collect()
    }

    /// Get all beams from the model
    pub fn get_beams(&self) -> Vec<&Line> {
        self.lines.iter()
            .filter(|line| line.line_type == "BEAM")
            .collect()
    }

    /// Get all walls from the model
    pub fn get_walls(&self) -> Vec<&Area> {
        self.areas.iter()
            .filter(|area| area.area_type == "PANEL")
            .collect()
    }

    /// Get all floors/slabs from the model
    pub fn get_floors(&self) -> Vec<&Area> {
        self.areas.iter()
            .filter(|area| area.area_type == "FLOOR")
            .collect()
    }

    /// Get story by name
    pub fn get_story(&self, name: &str) -> Option<&Story> {
        self.stories.iter().find(|s| s.name == name)
    }

    /// Get point by id
    pub fn get_point(&self, id: &str) -> Option<&Point> {
        self.points.iter().find(|p| p.id == id)
    }

    /// Get material by name
    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials.iter().find(|m| m.name == name)
    }

    /// Get frame section by name
    pub fn get_frame_section(&self, name: &str) -> Option<&FrameSection> {
        self.frame_sections.iter().find(|s| s.name == name)
    }

    /// Get shell property by name
    pub fn get_shell_prop(&self, name: &str) -> Option<&ShellProp> {
        self.shell_props.iter().find(|s| s.name == name)
    }

    /// Get load pattern by name
    pub fn get_load_pattern(&self, name: &str) -> Option<&LoadPattern> {
        self.load_patterns.iter().find(|p| p.name == name)
    }

    /// Get load case by name
    pub fn get_load_case(&self, name: &str) -> Option<&LoadCase> {
        self.load_cases.iter().find(|c| c.name == name)
    }

    /// Get load combination by name
    pub fn get_load_combination(&self, name: &str) -> Option<&LoadCombination> {
        self.load_combinations.iter().find(|c| c.name == name)
    }

    /// Calculate total building height
    pub fn total_height(&self) -> f64 {
        self.stories.iter()
            .filter_map(|s| s.height)
            .sum()
    }

    /// Get number of stories
    pub fn story_count(&self) -> usize {
        self.stories.len()
    }

    /// Get building statistics
    pub fn get_statistics(&self) -> ModelStatistics {
        ModelStatistics {
            num_stories: self.stories.len(),
            num_points: self.points.len(),
            num_lines: self.lines.len(),
            num_areas: self.areas.len(),
            num_columns: self.get_columns().len(),
            num_beams: self.get_beams().len(),
            num_walls: self.get_walls().len(),
            num_floors: self.get_floors().len(),
            num_materials: self.materials.len(),
            num_load_patterns: self.load_patterns.len(),
            num_load_cases: self.load_cases.len(),
            num_load_combinations: self.load_combinations.len(),
            total_height: self.total_height(),
        }
    }
}