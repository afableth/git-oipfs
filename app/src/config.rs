use std::path::PathBuf;

pub struct AppConfig {
    pub vcs_config: VcsConfig,
    pub reconciliation_cycle: u64,
}

pub struct VcsConfig {
    pub url: String,
    pub clone_path: PathBuf,
}

pub fn get_config() -> AppConfig {
    AppConfig {
        vcs_config: VcsConfig {
            url: String::from("https://github.com/poske57/portfolio.git"),
            clone_path: PathBuf::from("./repo"),
        },
        reconciliation_cycle: 15,
    }
}
