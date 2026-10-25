use crate::config::VcsConfig;
use gix::Repository;
use std::error::Error;
use std::fs::create_dir_all;

pub fn clone_if_not_exist(vcs_config: &VcsConfig) -> Result<Repository, Box<dyn Error>> {
    if vcs_config.clone_path.exists() {
        let repo = gix::open(&vcs_config.clone_path)?;
        return Ok(repo);
    }
    create_dir_all(&vcs_config.clone_path)?;
    let (repo, _) = gix::prepare_clone_bare(vcs_config.url.clone(), &vcs_config.clone_path)?
        .fetch_only(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;
    return Ok(repo);
}
