use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, Clap};
use dantalian::bangumi;

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
                let _ = bangumi::search_anime(search_opts.keyword).await?;
                Ok(())
            }
            BgmSubCmd::Get(get_opts) => {
                println!("get subject {}", get_opts.id);
                let _ = bangumi::get_subject_info(get_opts.id).await?;
                Ok(())
            }
            BgmSubCmd::GetEp(get_opts) => {
                println!("get subject {}", get_opts.id);
                let _ = bangumi::get_subject_episode(get_opts.id).await?;
                Ok(())
            }
        },
        SubCmd::Gen(gen_opts) => {
            for root in gen_opts.roots {
                println!("root: {}", root)
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
    Bgm(BgmCmd),
}

#[derive(Clap)]
#[clap(about = "gen nfo files for spci")]
struct GenCmd {
    #[clap(long, required = true)]
    roots: Vec<String>,
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
    GetEp(BgmGetSubjectOpt),
}

#[derive(Clap)]
#[clap(about = "search keyword")]
struct BgmSearchOpt {
    #[clap(short, long, required = true)]
    keyword: String,
}

#[derive(Clap)]
#[clap(about = "search keyword")]
struct BgmGetSubjectOpt {
    #[clap(short, long, required = true)]
    id: u32,
}
