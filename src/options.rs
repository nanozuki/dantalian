use clap::{crate_authors, crate_description, crate_version, Clap};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(author=crate_authors!(), version=crate_version!(), about=crate_description!())]
pub struct Opts {
    #[clap(short, long, about = "enable verbose")]
    pub verbose: bool,
    #[clap(short, long, about = "source folders", required = false)]
    pub source: Vec<PathBuf>,
    #[clap(
        long,
        about = "dir names which you want to force re-generate",
        required = false
    )]
    pub force: Vec<String>,
    #[clap(long, about = "force re-generate all anime")]
    pub force_all: bool,
    #[clap(subcommand)]
    pub subcmd: Option<SubCmd>,
}

#[derive(Clap)]
pub enum SubCmd {
    #[clap()]
    Bgm(BgmCmd),
}

#[derive(Clap)]
#[clap(about = "cli tools for bangumi apis")]
pub struct BgmCmd {
    #[clap(subcommand)]
    pub subcmd: BgmSubCmd,
}

#[derive(Clap)]
pub enum BgmSubCmd {
    Search(BgmSearchOpt),
    Get(BgmGetSubjectOpt),
    GetEp(BgmGetSubjectEpsOpt),
}

#[derive(Clap)]
#[clap(about = "search keyword")]
pub struct BgmSearchOpt {
    #[clap(about = "search keyword")]
    pub keyword: Vec<String>,
}

#[derive(Clap)]
#[clap(about = "get subject")]
pub struct BgmGetSubjectOpt {
    #[clap(about = "subject id")]
    pub id: u32,
}

#[derive(Clap)]
#[clap(about = "get subject episodes")]
pub struct BgmGetSubjectEpsOpt {
    #[clap(about = "subject id")]
    pub id: u32,
}
