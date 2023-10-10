use std::io::BufRead;

pub enum FileIdentity {
	Text,
	PortableNetworkGraphics,
	Unknown,
}

pub fn identifier<T: BufRead>(data: T) -> FileIdentity { todo!() }
