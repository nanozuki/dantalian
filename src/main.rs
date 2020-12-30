use clap::{crate_authors, crate_description, crate_version, Clap};

#[derive(Clap)]
#[clap(author=crate_authors!(), version=crate_version!(), about=crate_description!())]
struct Opts {
    #[clap(long, required = true)]
    roots: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    for root in opts.roots {
        println!("root: {}", root)
    }
}
