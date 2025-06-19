use anyhow::Result;
use dialoguer::{Input, theme::ColorfulTheme};
use tracing::{info, warn};

mod helpers;
mod local;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("🚀 Welcome to nix-deploy !");
    info!("🔸 A tool to deploy nixos configuration");
    warn!("🔸 SSH access must be available");
    warn!("🔸 Root privileges must be available");

    let mut local = local::Host::new()?;

    if helpers::ask_confirmation("Do you want to use nix config locally?")? {
        local.set_nix_config(true)?;
    } else {
        local.set_nix_config(false)?;
    }

    let user = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter ssh user:")
        .default("nixos".to_string())
        .allow_empty(false)
        .show_default(true)
        .interact_text()?;
    let destination = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter ssh destination:")
        .default("127.0.0.1".to_string())
        .allow_empty(false)
        .show_default(true)
        .interact_text()?;
    let port = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter ssh port (1-65535):")
        .default("22".to_string())
        .allow_empty(false)
        .show_default(true)
        .validate_with(|input: &String| -> Result<(), &str> {
            input
                .parse::<u16>()
                .map_err(|_| "Please enter a valid number between 1 and 65535")
                .and_then(|n| {
                    if (1..=65535).contains(&n) {
                        Ok(())
                    } else {
                        Err("Port must be between 1 and 65535")
                    }
                })
        })
        .interact_text()?;

    local.deploy_nixos_rebuild(&user, &destination, &port)?;

    Ok(info!("🚀 Reboot your remote host and enjoy !"))
}
