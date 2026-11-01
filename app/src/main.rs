mod config;
mod vcs;
use std::path::PathBuf;

use crate::config::VcsConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config()?;
    reconciliation(&config)?;
    println!("reconciliation!");
    Ok(())
}

fn reconciliation(config: &config::AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let repo = vcs::clone_if_not_exist(&config.vcs_config)?;
    vcs::checkout_head_to(&repo, &config.vcs_config.working_directory.join("checkout"))?;
    build(&config)?;
    Ok(())
}

fn build(config: &config::AppConfig) -> Result<std::process::Output, std::io::Error> {
    let build_path = if let Some(r) = &config.vcs_config.build_root.clone() {
        &config.vcs_config.working_directory.join("repo").join(r)
    } else {
        &config.vcs_config.working_directory.join("repo")
    };
    std::env::set_current_dir(build_path)?;
    std::process::Command::new(&config.build_config.program)
        .args(&config.build_config.args)
        .output()
}
