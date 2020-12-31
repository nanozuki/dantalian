use super::{Actor, EPISODE_TEMPLATE, Episode, TVSHOW_TEMPLATE, TVShow};
use crate::bangumi::{get_subject_episode, get_subject_info, EpisodeType};
use anyhow::{Context, Result};
use tinytemplate::TinyTemplate;

pub struct Generator<'a> {
    tt: TinyTemplate<'a>,
}

impl<'a> Generator<'a> {
    pub fn new() -> Generator<'a> {
        let mut g = Generator {
            tt: TinyTemplate::new(),
        };
        g.tt.add_template("tvshow", TVSHOW_TEMPLATE).unwrap();
        g.tt.add_template("episode", EPISODE_TEMPLATE).unwrap();
        g
    }

    pub fn gen_tvshow_nfo(&self, show: &TVShow) -> Result<String> {
        let rendered = self
            .tt
            .render("tvshow", show)
            .with_context(|| "render tvshow")?;
        println!("generated tvshow nfo file:\n{}", &rendered);
        Ok(rendered)
    }

    pub fn gen_episode_nfo(&self, episode: &Episode) -> Result<String> {
        let rendered = self
            .tt
            .render("episode", episode)
            .with_context(|| "render episode")?;
        println!("generated episode nfo file:\n{}", &rendered);
        Ok(rendered)
    }

    pub async fn gen_anime_nfos(&self, anime_subject_id: u32) -> Result<AnimeNFOs> {
        let bgm_subject = get_subject_info(anime_subject_id).await?;
        let bgm_episodes = get_subject_episode(anime_subject_id).await?;
        let mut actors: Vec<Actor> = Vec::new();
        let mut i = 0;
        let mut has_sp = false;
        for crt in bgm_subject.crt {
            for a in crt.actors {
                actors.push(Actor{
                    name: &a.name,
                    role: &crt.name,
                    order: i,
                    thumb: &crt.images.large,
                });
                i+=1;
            }
        }
        let episodes: Vec<Episode> = bgm_episodes.into_iter().map(|be| {
            let is_sp = be.episode_type == EpisodeType::Sp;
            if is_sp {
                has_sp = true;
            }
            Episode{
                uid: be.id,
                title: be.name_cn,
                original_title: be.name,
                show_title: bgm_subject.name,
                rating_value: None,
                rating_votes: None,
                ep_index: be.sort,
                is_sp,
                plog: be.desc,
                director: vec![], // TODO
                credits: vec![], // TODO
                premiered: bgm_subject.air_date,
                status: Some(format!("{:?}", be.status)),
                aired: Some(be.airdate),
                studio: None,
                actor: &actors,
            }
        }).collect();
        let tvshow = TVShow{
            uid: bgm_subject.id,
            title: bgm_subject.name_cn,
            original_title: bgm_subject.name,
            rating_value: bgm_subject.rating.score,
            rating_votes: bgm_subject.rating.total,
            has_sp,
            eps_count: bgm_subject.eps_count,
            plot: bgm_subject.summary,
            director: vec![], // TODO
            credits: vec![], // TODO
            poster: bgm_subject.images.large,
            genre: vec![],
            tag: vec![],
            premiered: bgm_subject.air_date,
            status: None,
            studio: None,
            actor: &actors,
        };
        let mut nfos = AnimeNFOs{
            tvshow: self.gen_tvshow_nfo(&tvshow)?,
            episodes: Vec::new(),
        };
        for e in episodes {
            let nfo = self.gen_episode_nfo(&e)?;
            nfos.episodes.push(nfo);
        }
        Ok(nfos)
    }
}

pub struct AnimeNFOs {
    tvshow: String,
    episodes: Vec<String>,
}
