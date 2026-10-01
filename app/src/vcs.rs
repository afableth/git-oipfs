use gix::Repository;
use std::error::Error;
use crate::config::VcsConfig;

pub fn clone_if_not_exist(vcs_config: &VcsConfig) -> Result<Repository, Box<dyn Error>> {
    if vcs_config.clone_path.exists() {
        let repo = gix::open(&vcs_config.clone_path)?;
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
            gix::prepare_clone(vcs_config.url.clone(), vcs_config.clone_path.clone())?.fetch_only(&mut task, &gix::interrupt::IS_INTERRUPTED)?;
        drop(progress);
        result
    };
    return Ok(repo);
}
