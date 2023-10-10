use std::io::{self, BufRead};

pub enum FileIdentity {
	Text,
	PortableNetworkGraphics,
	Unknown,
}

static PORTABLE_NETWORK_GRAPHICS: [u8; 4] = [0x89, 0x50, 0x4E, 0x47];

pub fn identifier<T: BufRead>(data: &mut T) -> io::Result<FileIdentity> {
	let mut buf = [0u8; 512];
	data.read_exact(&mut buf)?;

	if buf[..4] == PORTABLE_NETWORK_GRAPHICS {
		return Ok(FileIdentity::PortableNetworkGraphics);
	};

	if buf.iter().all(u8::is_ascii) {
		return Ok(FileIdentity::Text);
	};

	Ok(FileIdentity::Unknown)
}
