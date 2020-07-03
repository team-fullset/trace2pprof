use std::error::Error;
use std::fs::File;
use std::io::Write;

use flate2::{write::GzEncoder, Compression};
use structopt::StructOpt;

mod assembly;
mod counter;
mod profile;
mod reader;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opt::from_args();

    let trace_file = File::open(opts.input)?;
    let mut trace_file_reader = reader::TraceFileReader::new(trace_file);

    let mut instr_counter = counter::StackInstructionCounter::new();

    while let Some((instr_addr, instr_asm)) = trace_file_reader.read_line()? {
        instr_counter.count(instr_addr, 1);

        if assembly::is_call(instr_asm) {
            instr_counter.push(instr_addr);
        }

        if assembly::is_return(instr_asm) {
            instr_counter.pop();
        }
    }

    let mut profile_builder = profile::Builder::new();

    profile_builder.push_sample_type("instructions", "count");

    for (addrs, count) in instr_counter.iter() {
        profile_builder.push_sample_values(addrs, &[count as i64]);
    }

    let encoded_profile = profile_builder.finish();

    let profile_output_file = File::create("profile.pb.gz")?;
    let mut profile_output_encoder = GzEncoder::new(profile_output_file, Compression::default());

    profile_output_encoder.write_all(&encoded_profile)?;
    profile_output_encoder.finish()?;

    Ok(())
}
