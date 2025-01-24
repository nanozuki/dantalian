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
        let BgmAnime {
            subject,
            episodes,
            persons,
            characters,
        } = bgm_data;
        let mut data = AnimeData {
            episodes: Vec::new(),
            tvshow: TVShow {
                uid: subject.id,
                title: subject.name_cn,
                original_title: subject.name,
                rating_value: subject.rating.score,
                rating_votes: subject.rating.total,
                has_sp: false,
                eps_count: subject.total_episodes,
                plot: subject.summary,
                poster: subject.images.map(|img| img.large),
                genres: vec![],
                tags: vec![],
                premiered: subject.date,
                status: None,
                studio: None,
                actors: Rc::from(Vec::new()),
            },
        };

        let mut actors: Vec<Actor> = Vec::new();
        for character in characters {
            if character.actors.is_empty() {
                actors.push(Actor {
                    name: character.name,
                    role: String::from("N/A"),
                    order: actors.len() as u32,
                    thumb: character.images.map_or(String::from(""), |ci| ci.large),
                });
            } else {
                for actor in character.actors {
                    actors.push(Actor {
                        name: character.name.clone(),
                        role: actor.name,
                        order: actors.len() as u32,
                        thumb: character
                            .images
                            .as_ref()
                            .map_or(String::from(""), |ci| ci.large.clone()),
                    });
                }
            }
        }
        data.tvshow.actors = Rc::from(actors);

        let mut credits: Vec<String> = Vec::new();
        let mut directors: Vec<String> = Vec::new();
        // staff
        for person in persons {
            if person.relation == "导演" {
                directors.push(person.name);
            } else if person.relation == "脚本" {
                credits.push(person.name);
            }
        }
        let rc_directors = Rc::from(directors);
        let rc_credits = Rc::from(credits);
        let episode_len = episodes.len();

        for be in episodes {
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
                    // New bangumi api has no status.
                    status: None,
                    aired: Some(be.airdate),
                    studio: None,
                    actors: Rc::clone(&data.tvshow.actors),
                });
            } else if episode_len == 1 && be.episode_type == EpisodeType::Honpen && be.ep == Some(1)
            {
                data.episodes.push(Episode {
                    uid: be.id,
                    title: data.tvshow.title.clone(),
                    original_title: data.tvshow.original_title.clone(),
                    show_title: String::from(&data.tvshow.title),
                    rating_value: None,
                    rating_votes: None,
                    ep_index: format!("{}", be.sort),
                    is_sp: false,
                    plot: be.desc,
                    directors: Rc::clone(&rc_directors),
                    credits: Rc::clone(&rc_credits),
                    premiered: String::from(&data.tvshow.premiered),
                    // New bangumi api has no status.
                    status: None,
                    aired: Some(be.airdate),
                    studio: None,
                    actors: Rc::clone(&data.tvshow.actors),
                });
            }
        }
        data
    }
}
