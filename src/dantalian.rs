use crate::bangumi::{get_anime_data, search_anime, BgmAnime, EpisodeStatus, EpisodeType};
use crate::nfogen::{Actor, Episode, Generator, TVShow, TVSHOW_NFO_NAME};
use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use walkdir::{DirEntry, WalkDir};

// AnimeData store data for generator nfo files.
#[derive(Debug)]
pub struct AnimeData {
    pub tvshow: TVShow,
    pub episodes: Vec<Episode>,
}

// AnimeNFO for the content of anime nfo files.
#[derive(Debug)]
pub struct AnimeNFO {
    pub tvshow: String,
    pub episodes: Vec<String>,
}

pub struct Dantalian<'a> {
    nfo_generator: Generator<'a>,
}

impl<'a> Dantalian<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Dantalian<'a> {
        Dantalian {
            nfo_generator: Generator::new(),
        }
    }

    pub async fn gen_nfos(&self, ad: &AnimeData) -> Result<AnimeNFO> {
        let tvshow = self.nfo_generator.gen_tvshow_nfo(&ad.tvshow)?;
        let mut episodes: Vec<String> = Vec::new();
        for e in ad.episodes.iter() {
            episodes.push(self.nfo_generator.gen_episode_nfo(&e)?);
        }
        Ok(AnimeNFO { tvshow, episodes })
    }

    pub async fn check_anime(&self, subject_id: u32) -> Result<AnimeData> {
        let bgm_anime = get_anime_data(subject_id)
            .await
            .with_context(|| "get_anime_data")?;
        let data = parse_bgm_anime(bgm_anime);
        Ok(data)
    }

    /// generate nfo files for all tv anime media files.
    /// the files structure should follow this:
    /// media_root/
    ///     ├── ひぐらしのなく頃に 業
    ///     │   ├── ひぐらしのなく頃に 業 01.mp4
    ///     │   ├── ひぐらしのなく頃に 業 02.mp4
    ///     │   ├── ひぐらしのなく頃に 業 03.mp4
    ///     │   └── ひぐらしのなく頃に 業 04.mp4
    ///     ├── 化物語
    ///     │   ├── 化物語 01.mp4
    ///     │   ├── 化物語 02.mp4
    ///     │   └── 化物語 SP5_5.mp4
    ///     └── 进击的巨人 最终季
    ///         ├── 进击的巨人 最终季 60.mp4
    ///         ├── 进击的巨人 最终季 61.mp4
    ///         ├── 进击的巨人 最终季 62.mp4
    ///         └── 进击的巨人 最终季 63.mp4

    pub async fn generate_path(&self, root: &str, force: &HashSet<String>) -> Result<()> {
        for e in WalkDir::new(root).min_depth(1).max_depth(1) {
            let entry = e?;
            if entry.file_type().is_dir() {
                println!("{}", entry.path().display());
                let path = entry.path().to_str();
                let filename = entry.file_name().to_str();
                match (path, filename) {
                    (Some(p), Some(f)) => {
                        println!("Try Anime Name: '{}'", f);
                        self.generate_anime(p, f, force.contains(f)).await?;
                    }
                    _ => {
                        println!("Can't parse this path, skip");
                    }
                }
            }
        }
        Ok(())
    }

    async fn generate_anime(&self, path: &str, anime_name: &str, force: bool) -> Result<()> {
        let job = collect_gen_jobs(path, anime_name, force)?;
        if job.is_empty() {
            return Ok(());
        }
        let subjects = search_anime(&anime_name.to_string()).await?;
        if subjects.is_empty() {
            return Ok(());
        }
        let subject_id = subjects[0].id;
        let anime_data = parse_bgm_anime(get_anime_data(subject_id).await?);
        if job.gen_tvshow {
            print!("Prepare to gen tvshow.nfo for '{}' ... ", &anime_name);
            let file_content = self.nfo_generator.gen_tvshow_nfo(&anime_data.tvshow)?;
            let file_path = Path::new(path).join(TVSHOW_NFO_NAME);
            let mut f = File::create(file_path)?;
            f.write_all(&file_content.into_bytes())?;
            println!("Done!");
        }
        for episode in anime_data.episodes {
            let job_map = match episode.is_sp {
                false => &job.gen_episodes,
                true => &job.gen_sps,
            };
            if let Some(ep_file) = job_map.get(&episode.ep_index) {
                let file_name = match episode.is_sp {
                    false => format!("{} {}.nfo", &anime_name, ep_file),
                    true => format!("{} SP{}.nfo", &anime_name, ep_file),
                };
                print!("Prepare to gen {} ... ", &file_name);
                let file_content = self.nfo_generator.gen_episode_nfo(&episode)?;
                let file_path = Path::new(path).join(file_name);
                let mut f = File::create(file_path)?;
                f.write_all(&file_content.into_bytes())?;
                println!("Done!");
            }
        }
        Ok(())
    }
}

fn parse_bgm_anime(bgm_data: BgmAnime) -> AnimeData {
    let mut data = AnimeData {
        episodes: Vec::new(),
        tvshow: TVShow {
            uid: bgm_data.subject.id,
            title: bgm_data.subject.name_cn,
            original_title: bgm_data.subject.name,
            rating_value: bgm_data.subject.rating.score,
            rating_votes: bgm_data.subject.rating.total,
            has_sp: false,
            eps_count: bgm_data.subject.eps_count,
            plot: bgm_data.subject.summary,
            poster: bgm_data.subject.images.large,
            genres: vec![],
            tags: vec![],
            premiered: bgm_data.subject.air_date,
            status: None,
            studio: None,
            actors: Rc::from(Vec::new()),
        },
    };

    let mut actors: Vec<Actor> = Vec::new();
    for crt in bgm_data.subject.crt.iter() {
        match &crt.actors {
            Some(crt_actors) => {
                for a in crt_actors.iter() {
                    actors.push(Actor {
                        name: String::from(&crt.name_cn),
                        role: String::from(&a.name),
                        order: actors.len() as u32,
                        thumb: String::from(&crt.images.large),
                    });
                }
            }
            None => {
                actors.push(Actor {
                    name: String::from(&crt.name_cn),
                    role: String::from("N/A"),
                    order: actors.len() as u32,
                    thumb: String::from(&crt.images.large),
                });
            }
        }
    }
    data.tvshow.actors = Rc::from(actors);

    let mut directors: Vec<String> = Vec::new();
    let mut credits: Vec<String> = Vec::new();
    for staff in bgm_data.subject.staff.iter() {
        for job in staff.jobs.iter() {
            if job == "导演" {
                directors.push(String::from(&staff.name));
            }
            if job == "脚本" {
                credits.push(String::from(&staff.name));
            }
        }
    }
    let rc_directors = Rc::from(directors);
    let rc_credits = Rc::from(credits);

    for be in bgm_data.episodes {
        if be.status != EpisodeStatus::NA {
            let is_sp = be.episode_type == EpisodeType::Sp;
            data.tvshow.has_sp = data.tvshow.has_sp || is_sp;
            data.episodes.push(Episode {
                uid: be.id,
                title: be.name_cn,
                original_title: be.name,
                show_title: String::from(&data.tvshow.title),
                rating_value: None,
                rating_votes: None,
                ep_index: format!("{}", be.sort),
                is_sp,
                plot: be.desc,
                directors: Rc::clone(&rc_directors),
                credits: Rc::clone(&rc_credits),
                premiered: String::from(&data.tvshow.premiered),
                status: Some(format!("{:?}", be.status)),
                aired: Some(be.airdate),
                studio: None,
                actors: Rc::clone(&data.tvshow.actors),
            })
        }
    }
    data
}

enum FileType {
    Unknown,
    TVShowNFO,
    EpNFO(String),
    SpNFO(String),
    EpMedia(String, String), // episode nfo : episode in filename
    SpMedia(String, String), // episode nfo : episode in filename
}

struct GenerateJob {
    gen_tvshow: bool,
    gen_episodes: HashMap<String, String>,
    gen_sps: HashMap<String, String>,
}

impl GenerateJob {
    fn is_empty(&self) -> bool {
        (!self.gen_tvshow) && (self.gen_episodes.is_empty()) && (self.gen_sps.is_empty())
    }
}

fn collect_gen_jobs(path: &str, anime_name: &str, force: bool) -> Result<GenerateJob> {
    let mut job = GenerateJob {
        gen_tvshow: true,
        gen_episodes: HashMap::new(),
        gen_sps: HashMap::new(),
    };
    let mut has_episode_nfo: HashSet<String> = HashSet::new();
    let mut has_episode_media: HashMap<String, String> = HashMap::new();
    let mut has_sp_nfo: HashSet<String> = HashSet::new();
    let mut has_sp_media: HashMap<String, String> = HashMap::new();
    for file in WalkDir::new(path).min_depth(1).max_depth(1) {
        let f = file?;
        match check_file(&f, anime_name) {
            FileType::TVShowNFO => {
                if !force {
                    job.gen_tvshow = false;
                }
            }
            FileType::EpNFO(epi) => {
                has_episode_nfo.insert(epi);
            }
            FileType::EpMedia(epi, ep) => {
                has_episode_media.insert(epi, ep);
            }
            FileType::SpNFO(epi) => {
                has_sp_nfo.insert(epi);
            }
            FileType::SpMedia(epi, ep) => {
                has_sp_media.insert(epi, ep);
            }
            _ => {}
        }
    }
    for (ep_index, ep_file) in has_episode_media {
        if (!has_episode_nfo.contains(&ep_index)) || force {
            job.gen_episodes.insert(ep_index, ep_file);
        }
    }
    for (ep_index, ep_file) in has_sp_media {
        if (!has_sp_nfo.contains(&ep_index)) || force {
            job.gen_sps.insert(ep_index, ep_file);
        }
    }
    Ok(job)
}

fn check_file(file: &DirEntry, anime_name: &str) -> FileType {
    if !file.file_type().is_file() {
        return FileType::Unknown;
    }
    let file_name = match file.file_name().to_str() {
        Some(file_name) => file_name,
        None => {
            return FileType::Unknown;
        }
    };
    if file_name == TVSHOW_NFO_NAME {
        return FileType::TVShowNFO;
    }
    // shouldn't be error
    let episode_re =
        Regex::new(format!(r"^(?P<name>{}) (?P<sp>SP)?(?P<ep>[_\d]+)\.", anime_name).as_str())
            .unwrap();
    let cap = match episode_re.captures(&file_name) {
        Some(cap) => cap,
        None => {
            return FileType::Unknown;
        }
    };
    let is_sp = cap.name("sp").map_or(false, |mat| mat.as_str() == "SP");
    let ep_file = match cap.name("ep").map(|mat| mat.as_str()) {
        Some(ep) => ep,
        None => {
            return FileType::Unknown;
        }
    };
    let ep_index = ep_file.replace("_", ".");
    let ep_index = ep_index.trim_start_matches('0');
    let is_nfo = file_name.ends_with(".nfo");
    match (is_nfo, is_sp) {
        (true, true) => FileType::SpNFO(String::from(ep_index)),
        (true, false) => FileType::EpNFO(String::from(ep_index)),
        (false, true) => FileType::SpMedia(String::from(ep_index), String::from(ep_file)),
        (false, false) => FileType::EpMedia(String::from(ep_index), String::from(ep_file)),
    }
}
