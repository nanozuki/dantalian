use anyhow::Result;
use clap::Clap;
use dantalian::bangumi;
use dantalian::dantalian::dantalian;
use dantalian::{info, logger::Logger};
use log::set_logger;
use options::{BgmCmd, BgmSubCmd, Opts, SubCmd};
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
            for root in opts.root {
                dantalian(&root, &force).await?;
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
