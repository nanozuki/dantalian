use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, Clap};
use dantalian::bangumi;
use dantalian::dantalian::dantalian;
use dantalian::logger::Logger;
use log::{info, set_logger};
use std::collections::HashSet;

#[derive(Clap)]
#[clap(author=crate_authors!(), version=crate_version!(), about=crate_description!())]
struct Opts {
    #[clap(short, long, about = "enable verbose")]
    verbose: bool,
    #[clap(long, about = "path root of anime media files", required = false)]
    root: Vec<String>,
    #[clap(
        long,
        about = "dir names which you want to force re-generate",
        required = false
    )]
    force: Vec<String>,
    #[clap(subcommand)]
    subcmd: Option<SubCmd>,
}

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

#[derive(Clap)]
enum SubCmd {
    #[clap()]
    Bgm(BgmCmd),
}

#[derive(Clap)]
#[clap(about = "cli tools for bangumi apis")]
struct BgmCmd {
    #[clap(subcommand)]
    subcmd: BgmSubCmd,
}

#[derive(Clap)]
enum BgmSubCmd {
    Search(BgmSearchOpt),
    Get(BgmGetSubjectOpt),
    GetEp(BgmGetSubjectEpsOpt),
}

#[derive(Clap)]
#[clap(about = "search keyword")]
struct BgmSearchOpt {
    #[clap(about = "search keyword")]
    keyword: Vec<String>,
}

#[derive(Clap)]
#[clap(about = "get subject")]
struct BgmGetSubjectOpt {
    #[clap(about = "subject id")]
    id: u32,
}

#[derive(Clap)]
#[clap(about = "get subject episodes")]
struct BgmGetSubjectEpsOpt {
    #[clap(about = "subject id")]
    id: u32,
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
