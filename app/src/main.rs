use gix::Repository;
use std::error::Error;
use std::path::Path;
use std::time::Duration;
mod config;

fn main() {
    let config = config::get_config();
    let repo = clone_if_not_exist(&config.repository_path, &config.repository_url);
    println!("{:?}", repo);
    loop {
        println!("reconciliation!");
        std::thread::sleep(Duration::from_secs(&config.reconciliation_cycle * 60));
    }
}

fn clone_if_not_exist(dir: &Path, url: &str) -> Result<Repository, Box<dyn Error>> {
    if dir.exists() {
        let repo = gix::open(dir)?;
        return Ok(repo);
    }
    let (repo, _) = gix::prepare_clone(url, dir)?
        .fetch_only(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;
    return Ok(repo);
}
