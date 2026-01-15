use crate::error::{Result};


// ============================================================================
// Data Structures
// ============================================================================
#[derive(Debug, Clone, PartialEq)]
pub struct E2KModel {
    pub file_info: Option<FileInfo>,
    pub program_info: Option<ProgramInfo>,
    pub controls: Option<Controls>,
    pub stories: Vec<Story>,
    pub grids: Vec<Grid>,
    pub diaphragms: Vec<Diaphragm>,
    pub materials: Vec<Material>,
    pub rebar_definitions: Vec<RebarDefinition>,
    pub frame_sections: Vec<FrameSection>,
    pub concrete_sections: Vec<ConcreteSection>,
    pub tendon_sections: Vec<TendonSection>,
    pub shell_props: Vec<ShellProp>,
    pub link_props: Vec<LinkProp>,
    pub panel_zones: Vec<PanelZone>,
    pub pier_names: Vec<String>,
    pub spandrel_names: Vec<String>,
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub areas: Vec<Area>,
    pub groups: Vec<Group>,
    pub point_assigns: Vec<PointAssign>,
    pub line_assigns: Vec<LineAssign>,
    pub area_assigns: Vec<AreaAssign>,
    pub load_patterns: Vec<LoadPattern>,
    pub point_loads: Vec<PointLoad>,
    pub line_loads: Vec<LineLoad>,
    pub shell_uniform_load_sets: Vec<ShellUniformLoadSet>,
    pub area_loads: Vec<AreaLoad>,
    pub analysis_options: Option<AnalysisOptions>,
    pub mass_source: Option<MassSource>,
    pub functions: Vec<Function>,
    pub load_cases: Vec<LoadCase>,
    pub load_combinations: Vec<LoadCombination>,
    pub design_preferences: DesignPreferences,
    pub project_info: Option<ProjectInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileInfo {
    pub path: String,
    pub saved_date: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramInfo {
    pub program: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Controls {
    pub units: Units,
    pub title1: Option<String>,
    pub title2: Option<String>,
    pub preference: Option<Preference>,
    pub rllf: Option<RLLFMethod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Units {
    pub force: String,
    pub length: String,
    pub temperature: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Preference {
    pub merge_tolerance: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RLLFMethod {
    pub method: String,
    pub use_default_min: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Story {
    pub name: String,
    pub height: Option<f64>,
    pub elevation: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub system: String,
    pub label: String,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub visible: Option<String>,
    pub bubble_loc: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diaphragm {
    pub name: String,
    pub diaphragm_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub name: String,
    pub material_type: String,
    pub grade: Option<String>,
    pub weight_per_volume: Option<f64>,
    pub sym_type: Option<String>,
    pub e: Option<f64>,
    pub u: Option<f64>,
    pub a: Option<f64>,
    pub fy: Option<f64>,
    pub fu: Option<f64>,
    pub fc: Option<f64>,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RebarDefinition {
    pub name: String,
    pub area: f64,
    pub diameter: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameSection {
    pub name: String,
    pub material: String,
    pub shape: String,
    pub dimensions: Vec<(String, f64)>,
    pub modifiers: Vec<(String, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConcreteSection {
    pub name: String,
    pub long_bar_material: String,
    pub confine_bar_material: String,
    pub section_type: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TendonSection {
    pub name: String,
    pub material: String,
    pub strand_area: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShellProp {
    pub name: String,
    pub prop_type: String,
    pub material: String,
    pub modeling_type: Option<String>,
    pub thickness: Option<f64>,
    pub slab_type: Option<String>,
    pub wall_thickness: Option<f64>,
    pub modifiers: Vec<(String, f64)>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkProp {
    pub name: String,
    pub link_type: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PanelZone {
    pub name: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub id: String,
    pub line_type: String,
    pub point_i: String,
    pub point_j: String,
    pub cardinal_point: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub id: String,
    pub area_type: String,
    pub num_joints: i32,
    pub points: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub name: String,
    pub members: Vec<GroupMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupMember {
    pub member_type: String,
    pub id: String,
    pub story: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointAssign {
    pub point: String,
    pub story: String,
    pub restraint: Option<String>,
    pub user_joint: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineAssign {
    pub line: String,
    pub story: String,
    pub section: String,
    pub angle: Option<f64>,
    pub cardinal_point: Option<i32>,
    pub release: Option<String>,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaAssign {
    pub area: String,
    pub story: String,
    pub section: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadPattern {
    pub name: String,
    pub load_type: String,
    pub self_weight: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointLoad {
    pub point: String,
    pub story: String,
    pub load_pattern: String,
    pub load_type: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineLoad {
    pub line: String,
    pub story: String,
    pub load_pattern: String,
    pub load_type: String,
    pub direction: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShellUniformLoadSet {
    pub name: String,
    pub load_pattern: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaLoad {
    pub area: String,
    pub story: String,
    pub load_type: String,
    pub load_set: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisOptions {
    pub active_dof: String,
    pub model_hinges_in_links: String,
    pub p_delta: Option<PDelta>,
    pub auto_mesh_options: Option<AutoMeshOptions>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PDelta {
    pub method: String,
    pub tolerance: f64,
    pub loads: Vec<(String, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AutoMeshOptions {
    pub mesh_type: String,
    pub localized_floor_meshing: String,
    pub floor_mesh_merge_joints: String,
    pub floor_mesh_max_size: f64,
    pub wall_mesh_max_size: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MassSource {
    pub name: String,
    pub include_elements: String,
    pub include_loads: String,
    pub is_default: String,
    pub loads: Vec<(String, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub func_type: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadCase {
    pub name: String,
    pub case_type: String,
    pub init_cond: String,
    pub load_patterns: Vec<(String, f64)>,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadCombination {
    pub name: String,
    pub combo_type: String,
    pub cases: Vec<(String, f64)>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DesignPreferences {
    pub general: Option<GeneralDesignPreference>,
    pub steel: Option<SteelDesignPreference>,
    pub concrete: Option<ConcreteDesignPreference>,
    pub composite: Option<CompositeDesignPreference>,
    pub wall: Option<WallDesignPreference>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneralDesignPreference {
    pub structural_system: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SteelDesignPreference {
    pub code: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConcreteDesignPreference {
    pub code: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeDesignPreference {
    pub code: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WallDesignPreference {
    pub code: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectInfo {
    pub company_name: String,
    pub model_name: String,
}


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

    /// Validate model integrity
    pub fn validate(&self) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();

        // Check for required sections
        if self.stories.is_empty() {
            report.add_error("Model has no stories defined".to_string());
        }

        if self.points.is_empty() {
            report.add_warning("Model has no points defined".to_string());
        }

        // Validate point references in lines
        for line in &self.lines {
            if !self.points.iter().any(|p| p.id == line.point_i) {
                report.add_error(format!(
                    "Line '{}' references undefined point '{}'",
                    line.id, line.point_i
                ));
            }
            if !self.points.iter().any(|p| p.id == line.point_j) {
                report.add_error(format!(
                    "Line '{}' references undefined point '{}'",
                    line.id, line.point_j
                ));
            }
        }

        // Validate material references in frame sections
        for section in &self.frame_sections {
            if !self.materials.iter().any(|m| m.name == section.material) {
                report.add_error(format!(
                    "Frame section '{}' references undefined material '{}'",
                    section.name, section.material
                ));
            }
        }

        if report.has_errors() {
            Err(crate::E2kError::validation(format!(
                "Validation failed with {} errors",
                report.error_count()
            )))
        } else {
            Ok(report)
        }
    }
}

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
        writeln!(f, "Model Statistics:")?;
        writeln!(f, "  Stories: {}", self.num_stories)?;
        writeln!(f, "  Total Height: {:.2} ft", self.total_height)?;
        writeln!(f, "  Points: {}", self.num_points)?;
        writeln!(f, "  Lines: {} (Columns: {}, Beams: {})",
                 self.num_lines, self.num_columns, self.num_beams)?;
        writeln!(f, "  Areas: {} (Walls: {}, Floors: {})",
                 self.num_areas, self.num_walls, self.num_floors)?;
        writeln!(f, "  Materials: {}", self.num_materials)?;
        writeln!(f, "  Load Patterns: {}", self.num_load_patterns)?;
        writeln!(f, "  Load Cases: {}", self.num_load_cases)?;
        writeln!(f, "  Load Combinations: {}", self.num_load_combinations)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}
