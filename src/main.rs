use anyhow::Result;
use clap::Parser;

mod options;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = dantalian::options::Opts::parse();
    dantalian::run(opts).await
}
