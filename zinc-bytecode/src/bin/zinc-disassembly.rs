//!
//! The bytecode viewer binary.
//!

use std::path::PathBuf;

use structopt::StructOpt;

use std::fs::File;
use std::io::Read;
use zinc_bytecode::{dispatch_instruction, DecodingError, Instruction, InstructionInfo};

#[derive(Debug, StructOpt)]
#[structopt(name = "zinc", about = "The Zinc bytecode viewer")]
struct Arguments {
    #[structopt(
        name = "INPUT",
        parse(from_os_str),
        help = "Specifies the *.zn input file name"
    )]
    input: PathBuf,
}

fn main() -> Result<(), DecodingError> {
    init_logger();

    let args: Arguments = Arguments::from_args();

    let mut file = File::open(&args.input).expect("failed to open file");
    let mut input: Vec<u8> = Vec::new();
    file.read_to_end(&mut input).expect("failed to read file");

    let instructions = zinc_bytecode::decode_all_instructions(input.as_slice())?;
    for instruction in instructions.iter() {
        println!(
            "{}",
            dispatch_instruction!(instruction => instruction.to_assembly())
        );
    }

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
}
