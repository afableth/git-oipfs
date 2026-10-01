use std::path::PathBuf;

pub struct AppConfig {
    pub reconciliation_cycle: u64,
    pub repository_url: String,
    pub repository_path: PathBuf,
}

pub fn get_config() -> AppConfig {
    AppConfig {
        reconciliation_cycle: 15,
        repository_url: String::from("https://github.com/poske57/portfolio.git"),
        repository_path: PathBuf::from("./repo"),
    }
}
