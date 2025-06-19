use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use tempfile::TempDir;
use tracing::info;

use crate::helpers;

pub struct Repo {
    pub path: PathBuf,
    #[allow(dead_code)]
    tmp_dir: Option<TempDir>, // Keep tempdir alive
    pub host: String,
}

impl Repo {
    pub fn get_nix_config(use_path: bool) -> Result<Self> {
        let (repo, tmp_dir) = match use_path {
            true => {
                info!("📂 Get nix-config git repository ");
                let path = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter nix-config path:")
                    .default("nixos".to_string())
                    .allow_empty(false)
                    .show_default(true)
                    .interact_text()?;
                let repo = helpers::git::get_repository_by_path(&path)?;
                (repo, None)
            }
            false => {
                info!("📂 Clone nix-config git repository ");
                let (repo, tmp_dir) = helpers::git::get_repository_by_clone("nix-config")?;
                (repo, Some(tmp_dir))
            }
        };
        let repo_dir = repo.path().to_path_buf();
        let repo_path = repo_dir
            .parent()
            .context("Could not get parent path of cloned git repository")?;
        let host = Self::get_config_host(repo_path)?;
        Ok(Self {
            path: repo_path.to_path_buf(),
            tmp_dir,
            host,
        })
    }

    fn get_config_host(repo_path: &Path) -> Result<String> {
        let hosts =
            serde_json::from_str::<Vec<String>>(&helpers::command::run_with_stdout(&format!(
                " nix eval --json {}#nixosConfigurations --apply builtins.attrNames",
                repo_path.display()
            ))?)?;
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a config host?")
            .items(&hosts)
            .interact()?;
        let host = hosts
            .get(selection)
            .ok_or_else(|| anyhow!("Selected host doesn't be found"))?;
        Ok(host.to_string())
    }
}
