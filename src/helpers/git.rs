use anyhow::{Context, Result, bail};
use git2::Repository;
use tempfile::{TempDir, tempdir};

pub fn get_repository_by_clone(name: &str) -> Result<(Repository, TempDir)> {
    let tmp_dir = tempdir().context("Failed to create temp directory")?;
    let config_path = tmp_dir.path().join(name);
    let repo = Repository::clone(&format!("https://github.com/wallago/{name}"), &config_path)
        .context("Failed to clone repository")?;
    if repo.is_bare() {
        bail!("Cloned repository is a bare")
    }
    if repo.is_shallow() {
        bail!("Cloned repository is a shallow")
    }
    Ok((repo, tmp_dir))
}

pub fn get_repository_by_path(path: &str) -> Result<Repository> {
    let repo = Repository::discover(path).context("Failed to discover repository")?;
    if repo.is_bare() {
        bail!("Cloned repository is a bare")
    }
    if repo.is_shallow() {
        bail!("Cloned repository is a shallow")
    }
    Ok(repo)
}
