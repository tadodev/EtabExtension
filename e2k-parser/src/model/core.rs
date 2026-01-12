use crate::model::geometry::*;
use crate::model::structural::*;
use crate::model::loading::*;
use crate::model::analysis::*;
use crate::model::design::*;


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

