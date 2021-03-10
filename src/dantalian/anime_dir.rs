use anyhow::{bail, Result};
use walkdir::DirEntry;

use super::config::Config;

struct AnimeDir {
    dir: DirEntry,
    config: Config,
}

impl AnimeDir {
    async fn new(dir: DirEntry) -> Result<AnimeDir> {
        if !dir.file_type().is_dir() {
            bail!("{} is not a directory", dir.path().display());
        }
        Ok(AnimeDir {
            config: Config::parse(&dir.path()).await?,
            dir,
        })
    }

    fn subject_id(&self) -> u32 {
        self.config.subject_id
    }

    fn detect_files(&self) {}

    fn gen_job() {}
}
