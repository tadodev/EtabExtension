
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectInfo {
    pub company_name: String,
    pub model_name: String,
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
