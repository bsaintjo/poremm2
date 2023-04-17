use std::path::PathBuf;

use clap::Args;
use minimap2::Aligner;
use rust_htslib::bam::{Header, Read};

#[derive(Args)]
pub struct AlignArgs {
    genome_index: String,
    input_bam: PathBuf,
    output_bam: PathBuf,
}

impl AlignArgs {
    pub fn run(&self) -> eyre::Result<()> {
        let mut reader = rust_htslib::bam::Reader::from_path(&self.input_bam)?;
        let header = Header::from_template(reader.header());
        let mut writer = rust_htslib::bam::Writer::from_path(
            &self.output_bam,
            &header,
            rust_htslib::bam::Format::Bam,
        )?;

        let aligner = Aligner::builder()
            .map_ont()
            .with_index(&self.genome_index, None)
            .map_err(|_| eyre::eyre!("Failed to build aligner"))?;
        for res in reader.records() {
            let mut record = res?;
            let seq = record.seq().as_bytes();
            let mapping = aligner
                .map(&seq, false, false, None, None)
                .map_err(|_| eyre::eyre!("Mapping error"))?;
            for alnmnt in mapping.into_iter() {
                record.set_mapq(alnmnt.mapq as u8);
                writer.write(&record)?;
            }
        }
        todo!()
    }
}
