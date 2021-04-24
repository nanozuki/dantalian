use super::config::Config;
use anyhow::{anyhow, Result};
use std::ffi::OsStr;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub struct Job {
    pub subject_id: u32,
    pub should_gen_tvshow: bool,
    pub episodes: Vec<EpisodeJob>,
}

pub struct EpisodeJob {
    pub index: String,
    pub is_sp: bool,
    pub filename: String,
}

const TVSHOW_NFO_NAME: &str = "tvshow.nfo";

impl Job {
    pub fn parse(dir: &Path, config: &Config, force: bool) -> Result<Job> {
        let tv_show_file = dir.join(TVSHOW_NFO_NAME);
        let should_gen_tvshow = force || tv_show_file.exists();
        let mut episodes: Vec<EpisodeJob> = vec![];
        for e in WalkDir::new(dir).min_depth(1).max_depth(1) {
            let entry = e?;
            if entry.file_type().is_file() {
                let ep = Self::check_episode(&entry, config, force)?;
                if let Some(ep_job) = ep {
                    episodes.push(ep_job);
                }
            }
        }
        Ok(Job {
            subject_id: config.subject_id,
            should_gen_tvshow,
            episodes,
        })
    }

    fn check_episode(
        file_entry: &DirEntry,
        config: &Config,
        force: bool,
    ) -> Result<Option<EpisodeJob>> {
        // if this file don't have extension or this is nfo file, skip it.
        let skip = file_entry
            .path()
            .extension()
            .and_then(OsStr::to_str)
            .map_or(true, |ext| ext == "nfo");
        if skip {
            return Ok(None);
        }
        let file_name = match file_entry.file_name().to_str() {
            Some(f) => f,
            None => return Ok(None),
        };
        let nfo_file_path = file_entry.path().with_extension("nfo");
        if (!force) && nfo_file_path.exists() {
            // nfo file of current file already exists, don't need a job
            return Ok(None);
        }
        let caps = config.episode_re.captures(file_name);
        let ep: String = match caps.as_ref().and_then(|c| c.name("ep")) {
            Some(ep_match) => ep_match.as_str().parse()?,
            None => return Ok(None),
        };
        let sp = caps
            .and_then(|c| c.name("sp"))
            .map_or(false, |mat| mat.as_str() != "");
        return Ok(Some(EpisodeJob {
            index: ep,
            is_sp: sp,
            filename: String::from(
                nfo_file_path
                    .to_str()
                    .ok_or(anyhow!("invalid nfo file name"))?,
            ),
        }));
    }

    pub fn is_empty(&self) -> bool {
        (!self.should_gen_tvshow) && (self.episodes.len() == 0)
    }
}
