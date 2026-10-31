use crate::config::VcsConfig;
use gix::bstr::ByteSlice;
use gix::Repository;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::fs::create_dir_all;
use std::io::Write;
use std::path::PathBuf;

/// HEAD が指すコミットのファイル構成（パス → バイナリデータ）をすべて読み取る
fn read_head_files(repo: &Repository) -> Result<BTreeMap<PathBuf, Vec<u8>>, Box<dyn Error>> {
    let tree = repo.head_commit()?.tree()?;
    let mut files = BTreeMap::new();
    read_tree(&tree, PathBuf::new(), &mut files)?;
    Ok(files)
}

/// ツリーを再帰的に走査してファイルを読み取る
fn read_tree(
    tree: &gix::Tree<'_>,
    prefix: PathBuf,
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    for entry in tree.iter() {
        let entry = entry?;
        let path = match entry.filename().to_str() {
            Ok(name) => prefix.join(name),
            Err(_) => continue, // non-UTF8 filenames are skipped
        };
        if entry.mode().is_blob() {
            let blob = tree.repo.find_blob(entry.oid())?;
            files.insert(path, blob.data.to_vec());
        } else if entry.mode().is_tree() {
            let sub_tree = tree.repo.find_tree(entry.oid())?;
            read_tree(&sub_tree, path, files)?;
        }
        // symlink / submodule are ignored
    }
    Ok(())
}

/// HEAD のファイル構成を指定パスに展開する
pub fn checkout_head_to(repo: &Repository, dest: &std::path::Path) -> Result<(), Box<dyn Error>> {
    // dest 配下の既存コンテンツをクリア
    if dest.exists() {
        for entry in fs::read_dir(dest)? {
            let entry = entry?;
            fs::remove_dir_all(entry.path())?;
        }
    } else {
        create_dir_all(dest)?;
    }

    let files = read_head_files(repo)?;
    for (path, data) in files {
        let file_path = dest.join(path);
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }
        fs::File::create(&file_path)?.write_all(&data)?;
    }
    Ok(())
}

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
