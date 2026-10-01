use gix::Repository;
use std::error::Error;
use std::path::Path;
use std::time::Duration;
mod config;

fn main() {
    let config = config::get_config();
    let repo = clone_if_not_exist(&config.repository_path, &config.repository_url);
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
    let (repo, _) = {
        let tree = prodash::tree::Root::new();
        let progress = prodash::render::line::render(
            std::io::stdout(),
            std::sync::Arc::downgrade(&tree),
            prodash::render::line::Options::default()
                .auto_configure(prodash::render::line::StreamKind::Stdout),
        );
        let mut task = tree.add_child("clone");
        let result = gix::prepare_clone(url, dir)?
            .fetch_only(&mut task, &gix::interrupt::IS_INTERRUPTED)?;
        drop(progress);
        result
    };
    return Ok(repo);
}
