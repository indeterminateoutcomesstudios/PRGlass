use config::{Config, ConfigError, File};


#[derive(Debug, Deserialize)]
pub struct Settings {
    pub baserepo: String,
    pub targetrepo: String,
    pub githubtoken: String,
    pub repoloc: String,
    pub mainbranch: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("Config.toml")).unwrap();
        s.try_into()
    }
}
