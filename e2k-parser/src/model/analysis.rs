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