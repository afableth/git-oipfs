mod config;
mod vcs;

fn main() {
    let config = config::get_config().expect("failed to load config");
    reconciliation(&config).expect("failed");
    println!("reconciliation!");
}

fn reconciliation(config: &config::AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let repo = vcs::clone_if_not_exist(&config.vcs_config)?;
    let hash1 = vcs::get_head_hash(&repo)?;
    let hash = vcs::get_head_hash(&repo)?;
    println!("{}", hash);
    Ok(())
}
