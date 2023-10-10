use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use pfi::FileIdentity;
use strum::EnumProperty;
use tabled::{builder::Builder, settings::Style};

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
	file: PathBuf,
}

fn main() {
	let cli = Cli::parse();

	let mut file = BufReader::new(File::open(cli.file).unwrap());

	let fi = pfi::identifier(&mut file).unwrap();

	let mut table = Builder::new();

	table.push_record(["Type", fi.as_ref()]);

	if let Some(mime) = fi.get_str("Mime") {
		table.push_record(["MIME", mime]);
	}

	if let FileIdentity::PortableNetworkGraphics { width, height } = fi {
		table.push_record(["Width", &width.to_string()]);
		table.push_record(["Height", &height.to_string()]);
	}

	println!(
		"{}",
		table.build().with(Style::modern())
	);
}
