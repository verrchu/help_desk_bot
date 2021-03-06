mod db;
pub use db::DbConfig;

mod feedback;
pub use feedback::FeedbackConfig;

mod logs;
pub use logs::LogsConfig;

mod http;
pub use self::http::HttpConfig;

use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub feedback: FeedbackConfig,
    pub db: DbConfig,
    pub http: HttpConfig,
    #[serde(default)]
    pub logs: LogsConfig,
}

impl Config {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let file = File::open(path).map_err(anyhow::Error::from)?;
        serde_yaml::from_reader(file).map_err(anyhow::Error::from)
    }
}
