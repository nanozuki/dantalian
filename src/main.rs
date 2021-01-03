use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, Clap};
use dantalian::bangumi;
use dantalian::dantalian::Dantalian;
use std::collections::HashSet;

#[derive(Clap)]
#[clap(author=crate_authors!(), version=crate_version!(), about=crate_description!())]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCmd::Bgm(sub_opts) => match sub_opts.subcmd {
            BgmSubCmd::Search(search_opts) => {
                println!("search anime {}", search_opts.keyword);
                let _ = bangumi::search_anime(&search_opts.keyword).await?;
                Ok(())
            }
            BgmSubCmd::Get(get_opts) => {
                println!("get subject {}", get_opts.id);
                let _ = bangumi::get_subject_info(get_opts.id).await?;
                Ok(())
            }
            BgmSubCmd::GetEp(get_opts) => {
                println!("get subject {}", get_opts.id);
                let _ = bangumi::get_subject_episodes(get_opts.id).await?;
                Ok(())
            }
        },
        SubCmd::Gen(gen_opts) => {
            let mut force: HashSet<String> = HashSet::new();
            for f in gen_opts.force {
                force.insert(f);
            }
            for root in gen_opts.root {
                println!("root: {}, rescan: {:#?}", &root, &force);
                let d = Dantalian::new();
                d.generate_path(&root, &force).await?;
            }
            Ok(())
        }
        SubCmd::Check(check_opts) => {
            let d = Dantalian::new();
            let data = d.check_anime(check_opts.subject).await?;
            println!("get anime data:\n{:#?}", &data);
            let nfos = d.gen_nfos(&data).await?;
            println!("gen tvshow file:\n{}", &nfos.tvshow);
            for e in nfos.episodes.iter() {
                println!("gen episode file:\n{}", &e);
            }
            Ok(())
        }
    }
}

#[derive(Clap)]
enum SubCmd {
    #[clap()]
    Gen(GenCmd),
    #[clap()]
    Check(CheckCmd),
    #[clap()]
    Bgm(BgmCmd),
}

#[derive(Clap)]
#[clap(about = "gen nfo files for spci")]
struct GenCmd {
    #[clap(long, about = "path root of anime media files", required = true)]
    root: Vec<String>,
    #[clap(long, about = "anime names which you want to rescan", required = false)]
    force: Vec<String>,
}

#[derive(Clap)]
#[clap(about = "gen nfo files for spci")]
struct CheckCmd {
    #[clap(short, long, required = true)]
    subject: u32,
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
    keyword: String,
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
