mod config;
mod vcs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config()?;
    reconciliation(&config)?;
    println!("reconciliation!");
    Ok(())
}

fn reconciliation(config: &config::AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let repo = vcs::clone_if_not_exist(&config.vcs_config)?;
    build(&config)?;
    Ok(())
}

fn build(config: &config::AppConfig) -> Result<std::process::Output, std::io::Error> {
    std::env::set_current_dir(&config.vcs_config.build_path())?;
    std::process::Command::new(&config.build_config.program)
        .args(&config.build_config.args)
        .output()
}
