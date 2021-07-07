use anyhow::Result;
use clap::Clap;
use dantalian::dantalian::{dantalian, generate_config};
use dantalian::{bangumi, info, logger::Logger};
use log::set_logger;
use options::{BgmCmd, BgmSubCmd, GenConfigCmd, Opts, SubCmd};
use std::collections::HashSet;

mod options;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.verbose {
        true => set_logger(Logger::init(log::LevelFilter::Trace)).unwrap(),
        false => set_logger(Logger::init(log::LevelFilter::Info)).unwrap(),
    }
    match opts.subcmd {
        None => {
            let mut force: HashSet<String> = HashSet::new();
            for f in opts.force {
                force.insert(f);
            }
            for source in opts.source {
                dantalian(&source, &force).await?;
            }
            Ok(())
        }
        Some(subcmd) => match subcmd {
            SubCmd::Bgm(sub_opts) => bgm_cmd(sub_opts).await,
            SubCmd::GenConfig(gen_opts) => {
                let GenConfigCmd { keyword, path } = gen_opts;
                generate_config(keyword, &path).await
            }
        },
    }
}

async fn bgm_cmd(opts: BgmCmd) -> Result<()> {
    match opts.subcmd {
        BgmSubCmd::Search(search_opts) => {
            let keyword = &search_opts.keyword.join(" ");
            let res = bangumi::search_anime(keyword).await?;
            info!("found {} result(s):\n", &res.results);
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
