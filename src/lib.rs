use std::fmt;
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

impl fmt::Display for ParaPaths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Root:\t\t{:?}\nProjects:\t{:?}\nAreas:\t\t{:?}\nResources:\t{:?}\nArchives:\t{:?}",
            self.root, self.projects, self.areas, self.resources, self.archives
        )
    }
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

    pub fn get_path(&self, path_type: cli::Para) -> &PathBuf {
        match path_type {
            cli::Para::Projects => &self.projects,
            cli::Para::Areas => &self.areas,
            cli::Para::Resources => &self.resources,
            cli::Para::Archives => &self.archives,
        }
    }
}

pub mod core {
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
    pub fn init(para_paths: &ParaPaths) -> Result<()> {
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
    pub fn new(para_paths: &ParaPaths, variant: Para, name: PathBuf, file: bool) -> Result<()> {
        // Get PARA path according to provided PARA variant
        let base_path = para_paths.get_path(variant);
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
    pub fn archive<P: AsRef<Path> + std::fmt::Debug>(
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

    pub fn path(arg: Option<Para>, para_paths: &ParaPaths) -> () {
        match arg {
            None => {
                println!("{}", para_paths);
            }
            Some(para) => {
                let path = para_paths.get_path(para);
                println!("{:?}", path);
            }
        }
    }

    pub fn move_(
        para_paths: &ParaPaths,
        dest: Para,
        subdir: Option<PathBuf>,
        src: Vec<PathBuf>,
    ) -> Result<()> {
        let base_path = para_paths.get_path(dest);
        let dest_path = match subdir {
            Some(sub_path) => base_path.join(sub_path),
            None => base_path.to_owned(),
        };
        
        if !dest_path.exists() {
            fs_extra::dir::create_all(&dest_path, false)?;
        }

        let options = fs_extra::dir::CopyOptions::new();
        fs_extra::move_items(&src, dest_path, &options)?;

        Ok(())
    }

    pub fn copy(
        para_paths: &ParaPaths,
        dest: Para,
        subdir: Option<PathBuf>,
        src: Vec<PathBuf>,
    ) -> Result<()> {
        let base_path = para_paths.get_path(dest);
        let dest_path = match subdir {
            Some(sub_path) => base_path.join(sub_path),
            None => base_path.to_owned(),
        };

        let options = fs_extra::dir::CopyOptions::new();
        fs_extra::copy_items(&src, dest_path, &options)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    
    fn setup_empty_test_dir() -> (ParaPaths, assert_fs::TempDir) {
        let temp = assert_fs::TempDir::new().expect("Could not create temp dir");
        let para_paths = ParaPaths::from_root(temp.path(), false);
        (para_paths, temp)
    }
    
    fn setup_test_dir_with_para() -> (ParaPaths, assert_fs::TempDir) {
        let (para_paths, temp) = setup_empty_test_dir();
        // Create empty para folders
        for para_path in vec!["Projects", "Areas", "Resources", "Archives"] {
            temp.child(para_path).create_dir_all().unwrap();
        }
        (para_paths, temp)
    }
    
    #[test]
    fn test_init() -> Result<()>{
        let (para_paths, temp) = setup_empty_test_dir();
        assert_eq!(core::init(&para_paths)?, ());
        for dir in vec!["Projects", "Areas", "Resources", "Archives"] {
            temp.child(dir).assert(predicate::path::exists());
        }
        temp.close()?;
        Ok(())
    }
    
    #[test]
    fn test_new_dir() -> Result<()>{
        let (para_paths, temp) = setup_test_dir_with_para();
        let new_path = PathBuf::from("nested/dir");
        core::new(&para_paths, cli::Para::Areas, new_path, false)?;
        temp.child("Areas/nested/dir").assert(predicate::path::exists());
        temp.child("Areas/nested/dir").assert(predicate::path::is_dir());
        temp.close()?;
        Ok(())
    }
    
    #[test]
    fn test_new_file() -> Result<()> {
        let (para_paths, temp) = setup_test_dir_with_para();
        let new_file = PathBuf::from("new_file");
        core::new(&para_paths, cli::Para::Resources, new_file, true)?;
        temp.child("Resources/new_file").assert(predicate::path::exists());
        temp.child("Resources/new_file").assert(predicate::path::is_file());
        temp.close()?;
        Ok(())
    }
    
    #[test]
    fn test_archive() -> Result<()> {
        let (para_paths, temp) = setup_test_dir_with_para();
        temp.child("Projects/test_archive").create_dir_all()?;
        for i in 0..3 {
            temp.child(format!("Projects/test_archive/file_{}.txt", i)).touch()?;
        }
        core::archive(&para_paths.archives, vec![PathBuf::from(
            &temp.child("Projects/test_archive").path())])?;
        temp.child("Archives/test_archive").assert(predicate::path::exists());
        for i in 0..3 {
            temp.child(format!("Archives/test_archive/file_{}.txt", i))
                .assert(predicate::path::exists());
        }
        temp.close()?;
        Ok(())
    }
    
    #[test]
    fn test_move() -> Result<()> {
        let (para_paths, temp) = setup_test_dir_with_para();
        let temp2 = assert_fs::TempDir::new()?;
        // Create files in temp2, outside PARA folders
        temp2.child("file1.txt").touch()?;
        temp2.child("dir1").create_dir_all()?;
        // Attempt to move files from temp2 to Projects/test_move
        core::move_(
            &para_paths,
            cli::Para::Projects,
            Some(PathBuf::from("test_move")),
            vec![
                temp2.child("file1.txt").path().to_path_buf(),
                temp2.child("dir1").path().to_path_buf()
            ])?;
        // Confirm move
        assert!(temp.child("Projects/test_move/file1.txt").exists());
        assert!(temp.child("Projects/test_move/dir1").exists());
        assert!(!temp2.child("file1.txt").exists());
        assert!(!temp2.child("dir1").exists());
        
        temp.close()?;
        temp2.close()?;
        Ok(())
    }
}