use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use flate2::{write::GzEncoder, Compression};
use structopt::StructOpt;

mod profile;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opt::from_args();

    let trace_file = File::open(opts.input)?;
    let trace_file_reader = BufReader::new(trace_file);

    let mut symbol_counts = BTreeMap::<u64, u64>::new();

    for line in trace_file_reader.lines() {
        let instr_addr = u64::from_str_radix(line?.split(":").next().unwrap(), 16)?;

        symbol_counts
            .entry(instr_addr)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut profile_builder = profile::Builder::new();

    profile_builder.push_sample_type("instructions", "count");

    for (addr, count) in symbol_counts.iter() {
        profile_builder.push_sample_values(*addr, &[*count as i64]);
    }

    let encoded_profile = profile_builder.finish();

    let profile_output_file = File::create("profile.pb.gz")?;
    let mut profile_output_encoder = GzEncoder::new(profile_output_file, Compression::default());

    profile_output_encoder.write(&encoded_profile)?;
    profile_output_encoder.finish()?;

    Ok(())
}
