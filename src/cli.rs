use clap::{Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::path::PathBuf;

const HELP_TEMPLATE: &str = "
{before-help}{name} {version}
{about-section}Author: {author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    arg_required_else_help(true),
    help_template(HELP_TEMPLATE)
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Commands {
    /// Initialize the PARA directories in the current working directory
    Init {
        /// Force the creation of new PARA folders in the current directory.
        /// This location will be used for future PARA tasks.
        #[arg(short, long)]
        force: bool,
        /// Prepend numbers to the PARA folders to maintain order
        #[arg(short, long)]
        numbered: bool,
    },
    /// Create a new folder in one of the PARA folders with a provided name.
    New {
        // TODO make it so each value is a boolean flag (use ArgGroup?)
        #[arg(short='t', long="type", value_enum, default_value_t = Para::Projects)]
        /// Which type of PARA item to create, used to determine where to create the item.
        variant: Para,
        /// Name of the file/folder to create.
        name: PathBuf,
        /// If set create a new file, otherwise create a new directory.
        #[arg(short, long, default_value_t = false)]
        file: bool,
    },
    /// Send the files/folders at the provided paths to the Archives
    Archive { paths: Vec<PathBuf> },
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Para {
    Projects,
    Areas,
    Resources,
    Archives,
}
