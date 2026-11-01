mod config;
mod vcs;

use std::ffi::OsString;

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

fn build(config: &config::AppConfig, checkout: &std::path::Path) -> Result<(), BuildError> {
    let build_path = match &config.vcs_config.build_root {
        Some(r) => checkout.join(r),
        None => checkout.to_path_buf(),
    };
    std::env::set_current_dir(&build_path)?;
    let output = std::process::Command::new(&config.build_config.program)
        .args(&config.build_config.args)
        .output()?;
    if !output.status.success() {
        return Err(BuildError::NonZeroExit {
            program: config.build_config.program.clone(),
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    Ok(())
}

#[derive(Debug)]
enum BuildError {
    Io(std::io::Error),
    NonZeroExit {
        program: OsString,
        status: std::process::ExitStatus,
        stderr: String,
    },
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::Io(err) => write!(f, "{err}"),
            BuildError::NonZeroExit {
                program,
                status,
                stderr,
            } => {
                write!(f, "{program:?} failed with {status}")?;
                if !stderr.is_empty() {
                    write!(f, "\nstderr:\n{stderr}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for BuildError {}

impl From<std::io::Error> for BuildError {
    fn from(err: std::io::Error) -> Self {
        BuildError::Io(err)
    }
}
