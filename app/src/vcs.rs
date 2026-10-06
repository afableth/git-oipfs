use crate::config::VcsConfig;
use gix::{Repository, ObjectId};
use std::error::Error;

pub fn clone_if_not_exist(vcs_config: &VcsConfig) -> Result<Repository, Box<dyn Error>> {
    if vcs_config.clone_path.exists() {
        let repo = gix::open(&vcs_config.clone_path)?;
        return Ok(repo);
    }
    let (repo, _) = gix::prepare_clone_bare(vcs_config.url.clone(), &vcs_config.clone_path)?
        .fetch_only(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;
    return Ok(repo);
}

pub fn get_head_hash(repo: &Repository) -> Result<ObjectId, Box<dyn std::error::Error>> {
    Ok(repo.head_commit()?.id().detach())
}
