use crate::bangumi::get_anime_data;
use crate::nfogen::{Generator, TVSHOW_NFO_NAME};
use crate::{error, info};
use anyhow::{anyhow, Context, Result};
use config::Config;
use data::AnimeData;
use job::Job;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

mod config;
mod data;
mod job;
mod utils;

pub async fn dantalian<F: FnMut(String) -> bool>(source: &Path, mut is_force: F) -> Result<()> {
    info!("Run dantalian for {}", source.to_string_lossy());
    for e in WalkDir::new(source).min_depth(1).max_depth(1) {
        let entry = e?;
        if entry.file_type().is_dir() {
            let path = entry.path().to_string_lossy().to_string();
            info!(ind: 1, "Check {} ...", path);
            match handle_dir(entry.path(), is_force(path)).await {
                Ok(_) => info!(ind: 2, "Completed!"),
                Err(e) => error!(ind: 2, "Failed: {}", e),
            };
        }
    }
    Ok(())
}

async fn handle_dir(path: &Path, force: bool) -> Result<()> {
    let config = Config::parse(path).await?;
    let job = Job::parse(path, &config, force)?;
    if job.is_empty() {
        info!(ind: 3, "No file should be generate, skip.");
        return Ok(());
    }
    let bgm_data = get_anime_data(job.subject_id)
        .await
        .with_context(|| "get_anime_data")?;
    info!(ind: 3,
        "Fetch anime data for: [{}] {} / {}",
        &bgm_data.subject.id,
        &bgm_data.subject.name,
        &bgm_data.subject.name_cn
    );
    let anime_data = AnimeData::from(bgm_data);
    let generator = Generator::new();
    if job.should_gen_tvshow {
        info!(ind: 4, "Generate {} ...", TVSHOW_NFO_NAME);
        let file_str = generator.gen_tvshow_nfo(&anime_data.tvshow)?;
        let file_path = Path::new(path).join(TVSHOW_NFO_NAME);
        let mut f = File::create(file_path)?;
        f.write_all(&file_str.into_bytes())?;
    }
    for episode in job.episodes {
        info!(ind: 4, "Generate {} ...", &episode.filename);
        let data = anime_data
            .find_episode(&episode.index, episode.is_sp)
            .ok_or_else(|| anyhow!("Can't find ep {}, is_sp {}", episode.index, episode.is_sp))?;
        let file_str = generator.gen_episode_nfo(data)?;
        let mut f = File::create(&episode.filename)?;
        f.write_all(&file_str.into_bytes())?;
    }
    Ok(())
}
