use anyhow::Result;
use dialoguer::{Confirm, theme::ColorfulTheme};
use tracing::{info, warn};

use crate::helpers;

impl super::Host {
    pub fn deploy_nixos_rebuild(&self, user: &str, destination: &str, port: &str) -> Result<bool> {
        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to run nixos-rebuild?")
            .interact()?
        {
            warn!("❗ Skipping deployments via nixos-rebuild");
            return Ok(false);
        }

        info!("🚀 Deploying nix-config via nixos-rebuild");
        let repo = self.get_repo()?;

        let build_on_target = match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to build on target?")
            .interact()?
        {
            true => format!(" --target-host {}@{}", repo.host, user,),
            false => format!(""),
        };

        let command = format!(
            "NIX_SSHOPTS=\"-A -p {}\" nixos-rebuild switch --flake {}#{} --target-host {}@{} --use-substitutes --sudo --ask-sudo-password {}",
            port,
            repo.path.display(),
            repo.host,
            user,
            destination,
            build_on_target
        );
        tracing::info!("🔸 {command}");
        loop {
            match helpers::command::run(&command) {
                Ok(_) => return Ok(true),
                Err(err) => {
                    if !Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to retry?")
                        .interact()?
                    {
                        return Err(err);
                    }
                }
            }
        }
    }
}
