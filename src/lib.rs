use std::path::{Path, PathBuf};

pub mod cli;

#[derive(Debug)]
pub struct ParaPaths {
    pub root: PathBuf,
    pub projects: PathBuf,
    pub areas: PathBuf,
    pub resources: PathBuf,
    pub archives: PathBuf,
}

impl ParaPaths {
    pub fn from_root<P: AsRef<Path>>(path: P, prefix: bool) -> Self {
        let root_path = PathBuf::from(path.as_ref());
        let base_paths = vec!["Projects", "Areas", "Resources", "Archives"];
        let paths: Vec<String> = if prefix {
            base_paths
                .iter()
                .enumerate()
                .map(|(idx, p)| format!("{}_{}", idx, p))
                .collect()
        } else {
            base_paths.iter().map(|p| p.to_string()).collect()
        };
        Self {
            root: root_path.clone(),
            projects: root_path.clone().join(&paths[0]),
            areas: root_path.clone().join(&paths[1]),
            resources: root_path.clone().join(&paths[2]),
            archives: root_path.clone().join(&paths[3]),
        }
    }
}

pub mod commands {
    use super::{cli::Para, ParaPaths};
    use anyhow::{anyhow, Context, Result};
    use fs_extra;
    use indicatif::ProgressBar;
    use log::{debug, warn};
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Handles the "init" command
    ///
    /// Creates new directories for Projects, Areas, Resources, and Archives.
    ///
    /// If the directories already exist, continues trying to create the other directories.
    pub fn handle_init(para_paths: &ParaPaths) -> Result<()> {
        let paths_to_create = [
            &para_paths.projects,
            &para_paths.areas,
            &para_paths.resources,
            &para_paths.archives,
        ];
        for path in paths_to_create.iter() {
            if path.exists() && path.is_dir() {
                warn!("Directory at {:?} already exists! Skipping...", path);
                continue;
            }
            fs::create_dir(path).with_context(|| format!("Could not create {:?}", path))?;
            debug!("Created new directory: {:?}", path)
        }
        Ok(())
    }

    /// Handles the "new" command
    pub fn handle_new(
        para_paths: &ParaPaths,
        variant: Para,
        name: PathBuf,
        file: bool,
    ) -> Result<()> {
        // Get PARA path according to provided PARA variant
        let base_path = match variant {
            Para::Projects => &para_paths.projects,
            Para::Areas => &para_paths.areas,
            Para::Resources => &para_paths.resources,
            Para::Archives => &para_paths.archives,
        };
        if !base_path.exists() {
            return Err(anyhow!("Path {:?} does not exist. Try running `para init` first to create the PARA folders.", base_path));
        }
        // Create the new file/folder in the appropriate location
        let new_path = base_path.join(name);
        if file {
            fs::File::create(new_path)?;
        } else {
            fs::create_dir_all(new_path)?;
        }
        Ok(())
    }

    /// Handles the "archive" command
    pub fn handle_archive<P: AsRef<Path> + std::fmt::Debug>(
        archive_dir: P,
        paths: Vec<PathBuf>,
    ) -> Result<()> {
        // Confirm archive exists
        if !archive_dir.as_ref().exists() {
            return Err(anyhow!(
                "Could not find archive directory at {:?}",
                archive_dir
            ));
        }
        let options = fs_extra::dir::CopyOptions::new();
        let prog_bar = ProgressBar::new(100);
        let handle = |process_info: fs_extra::TransitProcess| {
            let perc = (process_info.copied_bytes / process_info.total_bytes) * 100;
            prog_bar.set_position(perc);
            fs_extra::dir::TransitProcessResult::ContinueOrAbort
        };
        // Move items to archive
        fs_extra::move_items_with_progress(&paths, &archive_dir, &options, handle)?;
        prog_bar.finish_and_clear();
        Ok(())
    }
}
