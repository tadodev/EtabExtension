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