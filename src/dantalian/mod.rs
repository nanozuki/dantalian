use crate::nfogen::{Generator, TVSHOW_NFO_NAME};
use anyhow::{Result, Context, anyhow};
use config::Config;
use job::Job;
use output::out;
use std::collections::HashSet;
use std::path::Path;
use utils::path_str;
use walkdir::WalkDir;
use data::AnimeData;
use crate::bangumi::get_anime_data;
use std::fs::File;
use std::io::Write;
use crate::bangumi::SubjectType::Anime;

mod config;
mod job;
mod output;
mod utils;
mod data;

pub fn dantalian(media_root: &str, forces: &HashSet<String>) -> Result<()> {
    out(0, &format!("Run dantalian for {}", media_root));
    for e in WalkDir::new(media_root).min_depth(1).max_depth(1) {
        let entry = e?;
        if entry.file_type().is_dir() {
            let path = path_str(entry.path())?;
            out(0, &format!("Run in {}...", path));
            match handle_dir(entry.path(), forces.contains(path)) {
                Ok => out(1, "ok!"),
                Err(e) => out(1, &format!("failed: {}", e)),
            }
        }
    }
    Ok(())
}

fn handle_dir(path: &Path, force: bool) -> Result<()> {
    let config = Config::parse(path)?;
    let job = Job::parse(path, &config, force)?;
    if job.is_empty() {
        return Ok(());
    }
    let anime_data = AnimeData::from(
        get_anime_data(job.subject_id)
            .await
            .with_context(|| "get_anime_data")?,
    );
    let generator = Generator::new();
    if job.should_gen_tvshow {
        let file_str = generator.gen_tvshow_nfo(&anime_data.tvshow)?;
        let file_path = Path::new(path).join(TVSHOW_NFO_NAME);
        let mut f = File::create(file_path)?;
        f.write_all(&file_content.into_bytes())?;
    }
    for episode in job.episodes {
        let data = anime_data.
            find_episode(episode.index, episode.is_sp)
            .ok_or(anyhow!("Can't find anime data"))?;
        let file_str = generator.gen_episode_nfo(data)?;
        let mut f = File::create(&episode.filename)?;
        f.write_all(&file_str.into_bytes())?;
    }
    Ok(())
}
