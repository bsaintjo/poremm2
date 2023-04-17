use clap::Args;
use minimap2::Aligner;

#[derive(Args)]
pub struct IndexArgs {
    /// Path to genome fasta file
    genome: String,
    /// Path to index file output
    index: String,
}

impl IndexArgs {
    pub fn run(&self) -> eyre::Result<()> {
        Aligner::builder()
            .map_ont()
            .with_index(&self.genome, Some(&self.index))
            .map_err(|_| eyre::eyre!("test"))?;
        Ok(())
    }
}
