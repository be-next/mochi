use serde::{Deserialize};
use serde_yaml::Mapping;

#[derive(Deserialize, Clone, Debug)]
pub struct ApiYaml {
    pub name: String,
    pub headers: Mapping,
    pub rules: Vec<RuleYaml>
}

#[derive(Deserialize, Clone, Debug)]
pub struct RuleYaml {
    pub matches: String,
    pub status: String,
    pub body: Option<String>
}


#[derive(Deserialize, Clone, Debug)]
pub struct ApiShapeYaml {
    pub name: String,
    pub shape: Vec<String>
}

#[derive(Clone, Debug)]
pub struct SystemFolder {
    pub name: String,
    pub shapes: Vec<ApiShapeYaml>,
    pub apis: Vec<ApiYaml>
}