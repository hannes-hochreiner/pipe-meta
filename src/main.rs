use clap::Parser;
use pipe_meta::generate_metadata;
use std::{
    fs::File,
    io::{stdin, stdout, Write},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// name of the output file
    #[arg(short, long)]
    meta_file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let meta = generate_metadata(stdin(), stdout())?;

    File::create(args.meta_file)?.write_all(serde_json::to_string(&meta)?.as_bytes())?;

    Ok(())
}
