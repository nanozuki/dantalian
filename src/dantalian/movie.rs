use crate::bangumi::get_subject;
use crate::dantalian::Config;
use crate::nfogen::nfo::{Movie, MOVIE_NFO_NAME};
use crate::nfogen::Generator;
use crate::{error, info};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

pub async fn dantalian_movie<F: Fn(String) -> bool>(source: &Path, is_force: &F) -> Result<()> {
    info!("Run dantalian for {}", source.to_string_lossy());
    let generator = Generator::new();
    for e in WalkDir::new(source).min_depth(1).max_depth(1) {
        let entry = e?;
        if entry.file_type().is_dir() {
            let path = entry.path().to_string_lossy().to_string();
            info!(ind: 1, "Check {} ...", path);
            match handle_dir(entry.path(), is_force(path), &generator).await {
                Ok(_) => info!(ind: 2, "Completed!"),
                Err(e) => error!(ind: 2, "Failed: {}\n{}", e, e.root_cause()),
            };
        }
    }
    Ok(())
}

async fn handle_dir<'a>(path: &Path, force: bool, generator: &'a Generator<'a>) -> Result<()> {
    let config = Config::parse(path).await?;
    let subject_id = config.subject_id;
    let movie_nfo_path = path.join(MOVIE_NFO_NAME);
    if !force && movie_nfo_path.exists() {
        // do not need generate
        return Ok(());
    }
    let movie_data = get_subject(subject_id)
        .await
        .with_context(|| "get_movie_info")?;
    let movie = Movie::from(movie_data);
    let movie_nfo = generator.gen_movie_nfo(&movie)?;
    let mut nfo_file = File::create(movie_nfo_path)?;
    nfo_file.write_all(movie_nfo.as_bytes())?;
    Ok(())
}
