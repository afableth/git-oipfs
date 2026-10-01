use gix::Repository;
use std::error::Error;
use std::path::Path;

pub fn clone_if_not_exist(dir: &Path, url: &str) -> Result<Repository, Box<dyn Error>> {
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
        let result =
            gix::prepare_clone(url, dir)?.fetch_only(&mut task, &gix::interrupt::IS_INTERRUPTED)?;
        drop(progress);
        result
    };
    return Ok(repo);
}
