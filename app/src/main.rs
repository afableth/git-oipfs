mod config;
mod vcs;

fn main() {
    let config = config::get_config().expect("failed to load config");
    reconciliation(&config).expect("failed");
    println!("reconciliation!");
}

fn reconciliation(config: &config::AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let repo = vcs::clone_if_not_exist(&config.vcs_config)?;
    Ok(())
}

fn build(config: config::BuildConfig) -> Result<std::process::Output, std::io::Error> {
    std::process::Command::new(config.program)
        .args(config.args)
        .output()
}

