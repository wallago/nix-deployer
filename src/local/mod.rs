use anyhow::{Result, anyhow};

use crate::local::git::Repo;

mod deploy;
mod git;

pub struct Host {
    repo: Option<Repo>,
}

impl Host {
    pub fn new() -> Result<Self> {
        Ok(Self { repo: None })
    }

    pub fn set_nix_config(&mut self, use_path: bool) -> Result<()> {
        self.repo = Some(Repo::get_nix_config(use_path)?);
        Ok(())
    }

    pub fn get_repo(&self) -> Result<&Repo> {
        Ok(self
            .repo
            .as_ref()
            .ok_or_else(|| anyhow!("Git repo not seems to be cloned"))?)
    }
}
