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
