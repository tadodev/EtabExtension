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