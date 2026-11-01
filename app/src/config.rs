use gix::Url;
use std::ffi::OsString;
use std::path::PathBuf;

pub struct AppConfig {
    pub vcs_config: VcsConfig,
    pub build_config: BuildConfig,
    pub reconciliation_cycle: u64,
}

pub struct VcsConfig {
    pub url: gix::Url,
    pub working_directory: PathBuf,
    pub build_root: Option<PathBuf>,
}

pub struct BuildConfig {
    pub program: OsString,
    pub args: Vec<OsString>,
    pub publish_dir: PathBuf,
}

pub fn get_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    Ok(AppConfig {
        vcs_config: VcsConfig {
            url: Url::try_from("https://github.com/afableth/portfolio.git")?,
            working_directory: PathBuf::from("./oipfs/"),
            build_root: None,
        },
        build_config: BuildConfig {
            program: OsString::from("hugo"),
            args: vec![OsString::from("build")],
            publish_dir: PathBuf::from("./oipfs/publish"),
        },
        reconciliation_cycle: 15,
    })
}
