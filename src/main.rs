use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

use addr2line::object::{ElfFile, Object};
use flate2::{write::GzEncoder, Compression};
use gumdrop::Options;

mod profile;
mod symbols;

use symbols::SymbolTable;

#[derive(Debug, Options)]
struct Opts {
    help: bool,
    #[options(required, no_short, long = "object-file")]
    object_file: String,
    #[options(required, no_short, long = "trace-file")]
    trace_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse_args_default_or_exit();

    let mut object_file = File::open(opts.object_file)?;

    let mut object_contents = Vec::new();
    object_file.read_to_end(&mut object_contents)?;

    let elf_file = ElfFile::parse(&object_contents)?;

    let symbol_table = SymbolTable::new(&elf_file);

    let symbol_map = elf_file.symbol_map();

    let trace_file = File::open(opts.trace_file)?;
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

    let mut inserted_functions = HashSet::<u64>::new();

    let context = addr2line::Context::new(&elf_file)?;

    for (addr, count) in symbol_counts.iter() {
        profile_builder.push_sample(profile::Sample {
            location_id: *addr,
            value: *count as i64,
        });

        let mut location = profile::Location {
            id: *addr,
            address: *addr,
            lines: vec![],
        };

        let mut frames = context.find_frames(*addr)?;
        while let Some(frame) = frames.next()? {
            if let Some(frame_location) = frame.location {
                if let Some(symbol) = symbol_map.get(*addr) {
                    let sym_addr = symbol.address();
                    if sym_addr != 0 {
                        if !inserted_functions.contains(&sym_addr) {
                            profile_builder.push_function(profile::Function {
                                id: sym_addr,
                                name: "",
                                system_name: symbol.name().unwrap_or(""),
                                filename: frame_location
                                    .file
                                    .as_ref()
                                    .map(|s| s.as_ref())
                                    .unwrap_or(""),
                                start_line: 0,
                            });
                            inserted_functions.insert(sym_addr);
                        }

                        location.lines.push(profile::Line {
                            function_id: sym_addr,
                            line: frame_location.line.unwrap_or(0) as i64,
                        });
                    }
                }
            }
        }

        if location.lines.is_empty() {
            // fall back to simple symbol search
            if let Some(symbol_idx) = symbol_table.lookup_symbol_index(*addr) {
                let symbol = elf_file.symbol_by_index(symbol_idx).unwrap();
                let sym_addr = symbol.address();
                if sym_addr != 0 {
                    if !inserted_functions.contains(&sym_addr) {
                        profile_builder.push_function(profile::Function {
                            id: sym_addr,
                            name: "",
                            system_name: symbol.name().unwrap_or(""),
                            filename: "",
                            start_line: 0,
                        });
                        inserted_functions.insert(sym_addr);
                    }

                    location.lines.push(profile::Line {
                        function_id: sym_addr,
                        line: 0,
                    });
                }
            }
        }

        profile_builder.push_location(location);
    }

    let encoded_profile = profile_builder.finish();

    let profile_output_file = File::create("profile.pb.gz")?;
    let mut profile_output_encoder = GzEncoder::new(profile_output_file, Compression::default());

    profile_output_encoder.write(&encoded_profile)?;
    profile_output_encoder.finish()?;

    Ok(())
}
