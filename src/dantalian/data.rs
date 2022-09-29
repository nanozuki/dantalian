use crate::bangumi::{BgmAnime, EpisodeType};
use crate::nfogen::{Actor, Episode, TVShow};
use std::rc::Rc;

// AnimeData store data for generator nfo files.
#[derive(Debug)]
pub struct AnimeData {
    pub tvshow: TVShow,
    pub episodes: Vec<Episode>,
}

impl AnimeData {
    pub fn find_episode(&self, index: &str, is_sp: bool) -> Option<&Episode> {
        self.episodes
            .iter()
            .find(|&ep| ep.ep_index == index && ep.is_sp == is_sp)
    }
}

impl From<BgmAnime> for AnimeData {
    fn from(bgm_data: BgmAnime) -> Self {
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
                poster: bgm_data.subject.images.map(|img| img.large),
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
            if !be.is_empty() {
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
}
