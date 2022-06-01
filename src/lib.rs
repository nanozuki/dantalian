#![allow(clippy::upper_case_acronyms)]
pub mod bangumi;
pub mod dantalian;
#[macro_use]
pub mod logger;
pub mod nfogen;
pub mod options;

#[cfg(feature = "bin")]
pub use bin::run;

#[cfg(feature = "bin")]
mod bin {
    use crate::dantalian::{dantalian, dantalian_movie};
    use crate::logger::Logger;
    use crate::options::{BgmCmd, BgmSubCmd, Opts, SubCmd};
    use crate::bangumi;
    use anyhow::Result;
    use log::set_logger;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    pub async fn run(opts: Opts) -> Result<()> {
        match opts.verbose {
            true => set_logger(Logger::init(log::LevelFilter::Trace)).unwrap(),
            false => set_logger(Logger::init(log::LevelFilter::Info)).unwrap(),
        }
        match opts.subcmd {
            SubCmd::Gen(gen_opts) => {
                let force: HashSet<String> = HashSet::from_iter(gen_opts.force);
                let force_all = gen_opts.force_all;
                let is_force = |path| force_all || force.contains(&path);
                for source in gen_opts.source {
                    dantalian(&source, &is_force).await?;
                }
                for movie_source in gen_opts.movie_source {
                    dantalian_movie(&movie_source, &is_force).await?;
                }
                Ok(())
            }
            SubCmd::Bgm(bgm_opts) => bgm_cmd(bgm_opts).await,
        }
    }

    async fn bgm_cmd(opts: BgmCmd) -> Result<()> {
        match opts.subcmd {
            BgmSubCmd::Search(search_opts) => {
                let keyword = &search_opts.keyword.join(" ");
                let res = bangumi::search_anime(keyword).await?;
                info!("found {} result(s):\n", &res.results);
                for item in res.list.iter() {
                    info!("{:>1}", item);
                }
                Ok(())
            }
            BgmSubCmd::Get(get_opts) => {
                let subject = bangumi::get_subject_info(get_opts.id).await?;
                info!("{}", &subject);
                Ok(())
            }
            BgmSubCmd::GetEp(get_opts) => {
                let res = bangumi::get_subject_episodes(get_opts.id).await?;
                info!("{}", &res);
                Ok(())
            }
        }
    }
}
