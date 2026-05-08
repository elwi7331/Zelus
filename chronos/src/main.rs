mod c_structures;
mod schedule;
mod xmv_structures;

use c_structures::{C, CAnnotations};
use clap::Parser;
use schedule::Schedule;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use xmv_structures::{IntegerType, TimingConstraints, Xmv};

/// Schedule Parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to source file
    path: PathBuf,

    /// Enable if using bitvectors in Kratos, default false
    #[arg(long, default_value_t = false)]
    use_bitvectors: bool,
}

fn read_file(path: PathBuf) -> std::io::Result<String> {
    let mut buf = String::new();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() {
    let args = Args::parse();
    let file = read_file(args.path).expect("Could not schedule read input file");

    let nuxmv_int_type = if args.use_bitvectors {
        IntegerType::BitVec
    } else {
        IntegerType::XmvInt
    };

    let mut schedule = Schedule::try_from(file.as_str()).expect("Could not parse schedule");
    schedule.validate().expect("Invalid schedule");
    schedule.pad_jobs();
    schedule.validate().expect("Invalid schedule after padding");

    let timing_constraints = TimingConstraints::new(&schedule, nuxmv_int_type);
    let c_annotations = CAnnotations::from(&schedule);
    println!(
        "### In the C file:\n{}\n\n### After Compilation to xmv:\n{}",
        c_annotations.as_code(),
        timing_constraints.as_xmv()
    );
}
