use crate::bangumi::{get_anime_data, BgmAnime, EpisodeStatus, EpisodeType};
use crate::nfogen::{Actor, Episode, Generator, TVShow};
use anyhow::{Context, Result};
use std::rc::Rc;

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
        for a in crt.actors.iter() {
            actors.push(Actor {
                name: String::from(&a.name),
                role: String::from(&crt.name_cn),
                order: actors.len() as u32,
                thumb: String::from(&crt.images.large),
            });
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
                ep_index: be.sort,
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
