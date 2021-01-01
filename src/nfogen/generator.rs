use super::{Episode, TVShow, EPISODE_TEMPLATE, TVSHOW_TEMPLATE};
use anyhow::{Context, Result};
use tinytemplate::TinyTemplate;

pub struct Generator<'a> {
    tt: TinyTemplate<'a>,
}

impl<'a> Generator<'a> {
    #[allow(clippy::new_without_default)]
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
}
