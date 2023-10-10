use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
	file: PathBuf,
}

fn main() {
	let cli = Cli::parse();

	let mut file = BufReader::new(File::open(cli.file).unwrap());

	let fi = pfi::identifier(&mut file).unwrap();

	println!("{fi:#?}");
}
