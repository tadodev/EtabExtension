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
