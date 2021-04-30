use crate::bangumi::get_anime_data;
use crate::logger::indent;
use crate::nfogen::{Generator, TVSHOW_NFO_NAME};
use anyhow::{anyhow, Context, Result};
use config::Config;
use data::AnimeData;
use job::Job;
use log::info;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use utils::path_str;
use walkdir::WalkDir;

mod config;
mod data;
mod job;
mod utils;

pub async fn dantalian(media_root: &str, forces: &HashSet<String>) -> Result<()> {
    info!("Run dantalian for {}", media_root);
    for e in WalkDir::new(media_root).min_depth(1).max_depth(1) {
        let entry = e?;
        if entry.file_type().is_dir() {
            let path = path_str(entry.path())?;
            info!("{}Check {} ...", indent(1), path);
            match handle_dir(entry.path(), forces.contains(path)).await {
                Ok(_) => info!("{}Completed!", indent(2)),
                Err(e) => info!("{}Failed: {}", indent(2), e),
            };
        }
    }
    Ok(())
}

async fn handle_dir(path: &Path, force: bool) -> Result<()> {
    let config = Config::parse(path).await?;
    let job = Job::parse(path, &config, force)?;
    if job.is_empty() {
        info!("{}No file should be generate, skip.", indent(3));
        return Ok(());
    }
    let bgm_data = get_anime_data(job.subject_id).await.with_context(|| "get_anime_data")?;
    info!("{}Fetch anime data for: [{}] {} / {}",
          indent(3),
          &bgm_data.subject.id,
          &bgm_data.subject.name,
          &bgm_data.subject.name_cn);
    let anime_data = AnimeData::from(bgm_data);
    let generator = Generator::new();
    if job.should_gen_tvshow {
        info!("{}Generate {} ...", indent(4), TVSHOW_NFO_NAME);
        let file_str = generator.gen_tvshow_nfo(&anime_data.tvshow)?;
        let file_path = Path::new(path).join(TVSHOW_NFO_NAME);
        let mut f = File::create(file_path)?;
        f.write_all(&file_str.into_bytes())?;
    }
    for episode in job.episodes {
        info!("{}Generate {} ...", indent(4), &episode.filename);
        let data = anime_data
            .find_episode(&episode.index, episode.is_sp)
            .ok_or_else(|| anyhow!("Can't find ep {}, is_sp {}", episode.index, episode.is_sp))?;
        let file_str = generator.gen_episode_nfo(data)?;
        let mut f = File::create(&episode.filename)?;
        f.write_all(&file_str.into_bytes())?;
    }
    Ok(())
}
