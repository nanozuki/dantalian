use anyhow::Result;
use clap::Parser;
use dantalian::bangumi;
use dantalian::dantalian::{dantalian, dantalian_movie};
use dantalian::{info, logger::Logger};
use log::set_logger;
use options::{BgmCmd, BgmSubCmd, Opts, SubCmd};
use std::collections::HashSet;
use std::iter::FromIterator;

mod options;

#[tokio::main]
async fn main() -> Result<()> {
    let Opts {
        access_token,
        verbose,
        subcmd,
        force_all,
        force,
        source,
        movie_source,
    } = Opts::parse();
    if let Some(access_token) = access_token {
        bangumi::set_access_token(access_token);
    }
    match verbose {
        true => set_logger(Logger::init(log::LevelFilter::Trace)).unwrap(),
        false => set_logger(Logger::init(log::LevelFilter::Info)).unwrap(),
    }
    match subcmd {
        None => {
            let force: HashSet<String> = HashSet::from_iter(force);
            let force_all = force_all;
            let is_force = |path| force_all || force.contains(&path);
            for source in source {
                dantalian(&source, &is_force).await?;
            }
            for movie_source in movie_source {
                dantalian_movie(&movie_source, &is_force).await?;
            }
            Ok(())
        }
        Some(subcmd) => match subcmd {
            SubCmd::Bgm(sub_opts) => bgm_cmd(sub_opts).await,
        },
    }
}

async fn bgm_cmd(opts: BgmCmd) -> Result<()> {
    match opts.subcmd {
        BgmSubCmd::Search(search_opts) => {
            let keyword = search_opts.keyword.join(" ");
            let res = bangumi::search_anime(&keyword).await?;
            info!("found {} result(s):\n", res.data.len());
            for item in res.data {
                info!("{:>1}", item);
            }
            Ok(())
        }
        BgmSubCmd::Get(get_opts) => {
            let subject = bangumi::get_subject(get_opts.id).await?;
            info!("{}", &subject);
            if !get_opts.no_persons {
                let persons = bangumi::get_subject_persons(get_opts.id).await?;
                info!("{}", persons);
            }
            if !get_opts.no_characters {
                let characters = bangumi::get_subject_characters(get_opts.id).await?;
                info!("{}", characters);
            }
            Ok(())
        }
        BgmSubCmd::GetEp(get_opts) => {
            let res = bangumi::get_subject_episodes(get_opts.id).await?;
            info!("{}", &res);
            Ok(())
        }
    }
}
