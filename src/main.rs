use std::path::PathBuf;

use clap::Parser;
use mutsuki_service_config::{ConfigOverrides, ServiceConfig};

#[derive(Parser)]
#[command(name = "mutsuki-cli")]
#[command(about = "Terminal control client for MutsukiServiceHost")]
struct Cli {
    #[arg(long)]
    profile: Option<String>,
    #[arg(long)]
    config: Option<PathBuf>,
    #[arg(long)]
    home: Option<PathBuf>,
    #[arg(long, env = "MUTSUKI_CONTROL_TOKEN")]
    token: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = ServiceConfig::load(ConfigOverrides {
        profile: cli.profile,
        config_file: cli.config,
        home_dir: cli.home,
        control_token: cli.token,
    })?;
    mutsuki_cli_host::run(config).await
}
