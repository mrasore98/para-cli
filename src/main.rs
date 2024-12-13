use std::default::Default;
use std::env::current_dir;
use std::path::PathBuf;
use std::process::exit;

use anyhow::{Context, Result};
use clap::Parser;
use confy;
use env_logger;
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

use para_cli::{
    cli::{Cli, Commands},
    core, ParaPaths,
};

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize)]
struct AppConfig {
    root_dir: PathBuf,
    use_prefix: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            root_dir: current_dir().expect("Could not get current directory"),
            use_prefix: false,
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    trace!("Parsed args: {:?}", args);
    debug!(
        "Parsing config file from {:?}",
        confy::get_configuration_file_path(PACKAGE_NAME, None)?
    );
    let mut cfg: AppConfig = confy::load(PACKAGE_NAME, None)?;
    if let Commands::Init { force, numbered } = args.command {
        let cwd = current_dir().with_context(|| "Could not get current directory")?;
        // Update the config before creating paths if using init in a new directory.
        if cfg.root_dir != cwd {
            if force {
                warn!("Existing config directory does not match current directory.");
                debug!("Config root: {:?}", cfg.root_dir);
                debug!("Current directory: {:?}", cwd);
                info!("Setting new root directory for PARA folders: {:?}", cwd);
                cfg.root_dir = cwd;
            } else {
                error!("Existing config directory does not match current directory!");
                error!("Use `--force` to set this directory as the config directory and create new PARA folders.");
                exit(1);
            }
        }
        cfg.use_prefix = numbered;
        // Don't save modified settings when testing
        if !args.test{
            confy::store(PACKAGE_NAME, None, &cfg)?;
        }
        
    }

    let para_paths = ParaPaths::from_root(&cfg.root_dir, cfg.use_prefix);
    debug!("Using the following PARA paths: {:?}", para_paths);
    match args.command {
        Commands::Init {
            force: _,
            numbered: _,
        } => core::init(&para_paths)?,
        Commands::New {
            variant,
            name,
            file,
        } => core::new(&para_paths, variant, name, file)?,
        Commands::Archive { paths } => core::archive(&para_paths.archives, paths)?,
        Commands::Path { variant } => core::path(variant, &para_paths),
        Commands::Move {
            destination,
            subfolder,
            src,
        } => core::move_(&para_paths, destination, subfolder, src)?,
        Commands::Copy {
            destination,
            subfolder,
            src,
        } => core::copy(&para_paths, destination, subfolder, src)?,
    };
    Ok(())
}
