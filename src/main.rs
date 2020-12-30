use clap::{crate_authors, crate_description, crate_version, Clap};
mod bangumi;
use anyhow::Result;
use bangumi::Bangumi;
use tokio;

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
                let bgm = Bangumi {};
                println!("search anime {}", search_opts.keyword);
                let _ = bgm.search_subject(search_opts.keyword).await?;
                // tokio::join!(fut);
                // futures::future::join(fut).await;
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
}

#[derive(Clap)]
#[clap(about = "search keyword")]
struct BgmSearchOpt {
    #[clap(short, long, required = true)]
    keyword: String,
}
