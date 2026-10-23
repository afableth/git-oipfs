use gix::Url;
use std::path::PathBuf;

pub struct AppConfig {
    pub vcs_config: VcsConfig,
    pub build_config: BuildConfig,
    pub reconciliation_cycle: u64,
}

pub struct VcsConfig {
    pub url: gix::Url,
    pub clone_path: PathBuf,
    pub build_root: Option<PathBuf>,
}

impl VcsConfig {
    pub fn build_path(&self) -> PathBuf {
        if let Some(r) = self.build_root.clone() {
            self.clone_path.join(r)
        } else {
            self.clone_path.clone()
        }
    }
}

pub struct BuildConfig {
    pub command: String,
}

pub fn get_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    Ok(AppConfig {
        vcs_config: VcsConfig {
            url: Url::try_from("https://github.com/poske57/portfolio.git")?,
            clone_path: PathBuf::from("./repo"),
            build_root: None,
        },
        build_config: BuildConfig {
            command: String::from("nix run nixpkgs#hugo build"),
        },
        reconciliation_cycle: 15,
    })
}
