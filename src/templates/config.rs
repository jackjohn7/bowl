use serde::{Deserialize, Serialize};

/// bowl.toml config
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub template: TemplateInfo,
    pub options: Options,
}

impl Config {
    /// Create default config from template name
    pub fn new_default(template_name: String) -> Self {
        Self {
            template: TemplateInfo {
                name: template_name,
                version: "0.0.1".into(),
                source: None,
                description: None,
            },
            options: Options {
                ignore: None,
                readme: default_readme(),
            },
        }
    }
}

/// Configuration of bowl template
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateInfo {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub ignore: Option<Vec<String>>,
    #[serde(default = "default_readme")]
    pub readme: String,
}

pub fn default_readme() -> String {
    "./bowl.md".into()
}
