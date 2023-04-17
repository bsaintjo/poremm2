mod align;
mod index;

use align::AlignArgs;
use clap::Parser;
use index::IndexArgs;

#[derive(Parser)]
enum Command {
    Index(IndexArgs),
    Align(AlignArgs),
}

fn main() -> eyre::Result<()> {
    env_logger::init();
    let cmd = Command::parse();
    match cmd {
        Command::Index(idx_args) => {
            log::info!("Indexing genome...");
            idx_args.run()?;
        }
        Command::Align(aln_args) => {
            log::info!("Aligning reads...");
            aln_args.run()?;
        }
    }
    Ok(())
}
