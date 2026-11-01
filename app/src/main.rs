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
    let checkout = config.vcs_config.working_directory.join("checkout");
    vcs::checkout_head_to(&repo, &checkout)?;
    build(&config, &checkout)?;
    Ok(())
}

fn build(
    config: &config::AppConfig,
    checkout: &std::path::Path,
) -> Result<std::process::Output, std::io::Error> {
    let build_path = match &config.vcs_config.build_root {
        Some(r) => checkout.join(r),
        None => checkout.to_path_buf(),
    };
    std::env::set_current_dir(&build_path)?;
    std::process::Command::new(&config.build_config.program)
        .args(&config.build_config.args)
        .output()
}
