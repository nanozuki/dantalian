use clap::{crate_authors, crate_description, crate_version, Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author=crate_authors!(), version=crate_version!(), about=crate_description!())]
pub struct Opts {
    #[clap(short, long, about = "enable verbose")]
    pub verbose: bool,
    #[clap(short, long, about = "source folders", required = false, value_hint=ValueHint::DirPath)]
    pub source: Vec<PathBuf>,
    #[clap(short, long, about = "source folders", required = false, value_hint=ValueHint::DirPath)]
    pub movie_source: Vec<PathBuf>,
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

#[derive(Parser)]
pub enum SubCmd {
    #[clap()]
    Bgm(BgmCmd),
}

#[derive(Parser)]
#[clap(about = "cli tools for bangumi apis")]
pub struct BgmCmd {
    #[clap(subcommand)]
    pub subcmd: BgmSubCmd,
}

#[derive(Parser)]
pub enum BgmSubCmd {
    Search(BgmSearchOpt),
    Get(BgmGetSubjectOpt),
    GetEp(BgmGetSubjectEpsOpt),
}

#[derive(Parser)]
#[clap(about = "search keyword")]
pub struct BgmSearchOpt {
    #[clap(about = "search keyword")]
    pub keyword: Vec<String>,
}

#[derive(Parser)]
#[clap(about = "get subject")]
pub struct BgmGetSubjectOpt {
    #[clap(about = "subject id")]
    pub id: u32,
}

#[derive(Parser)]
#[clap(about = "get subject episodes")]
pub struct BgmGetSubjectEpsOpt {
    #[clap(about = "subject id")]
    pub id: u32,
}
