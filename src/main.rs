use anyhow::Result;
use dialoguer::{Input, theme::ColorfulTheme};
use tracing::{info, warn};

mod config;
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

    let mut cfg = config::DeployConfig::load()?;

    let user = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter ssh user:")
        .default(if cfg.ssh_user.is_empty() {
            "nixos".into()
        } else {
            cfg.ssh_user.clone()
        })
        .allow_empty(false)
        .show_default(true)
        .interact_text()?;
    let destination = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter ssh destination:")
        .default(if cfg.ssh_destination.is_empty() {
            "127.0.0.1".into()
        } else {
            cfg.ssh_user.clone()
        })
        .allow_empty(false)
        .show_default(true)
        .interact_text()?;
    let port = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter ssh port (1-65535):")
        .default(if cfg.ssh_port.is_empty() {
            "22".into()
        } else {
            cfg.ssh_port.clone()
        })
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

    cfg.ssh_user = user.clone();
    cfg.ssh_destination = destination.clone();
    cfg.ssh_port = port.clone();
    cfg.save()?;

    local.deploy_nixos_rebuild(&user, &destination, &port)?;
    Ok(info!("🚀 Reboot your remote host and enjoy !"))
}
