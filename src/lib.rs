use std::io::{self, BufRead};

use byteorder::{ReadBytesExt, BE};
use strum::{AsRefStr, EnumIter, EnumProperty};

#[derive(Debug, EnumIter, EnumProperty, AsRefStr)]
pub enum FileIdentity {
	#[strum(props(Mime = "text/plain", IsBinary = "N"))]
	Text,

	#[strum(
		serialize = "Portable Network Graphics",
		props(Mime = "image/png", IsBinary = "Y")
	)]
	PortableNetworkGraphics { width: u32, height: u32 },

	#[strum(
		serialize = "Java Class File",
		props(Mime = "application/java-vm", IsBinary = "Y")
	)]
	JavaClassFile {
		version_major: u16,
		version_minor: u16,
	},

	#[strum(
		serialize = "Portable Document Format",
		props(Mime = "application/pdf")
	)]
	PortableDocumentFormat,

	#[strum(props(IsBinary = "Y"))]
	Unknown,
}

static PORTABLE_NETWORK_GRAPHICS: [u8; 4] = [0x89, 0x50, 0x4E, 0x47];
static JAVA_CLASS_FILE: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
static PORTABLE_DOCUMENT_FORMAT: [u8; 5] = [0x25, 0x50, 0x44, 0x46, 0x2D];

pub fn identifier<T: BufRead>(data: &mut T) -> io::Result<FileIdentity> {
	let mut buf = [0u8; 512];
	data.read_exact(&mut buf)?;

	if buf[..4] == PORTABLE_NETWORK_GRAPHICS {
		let width = (&buf[16..20]).read_u32::<BE>()?;
		let height = (&buf[20..24]).read_u32::<BE>()?;

		return Ok(FileIdentity::PortableNetworkGraphics { width, height });
	};

	if buf[..4] == JAVA_CLASS_FILE {
		let version_minor = (&buf[4..6]).read_u16::<BE>()?;
		let version_major = (&buf[6..8]).read_u16::<BE>()?;

		return Ok(FileIdentity::JavaClassFile {
			version_major,
			version_minor,
		});
	}

	if buf[..5] == PORTABLE_DOCUMENT_FORMAT {
		return Ok(FileIdentity::PortableDocumentFormat);
	}

	if buf.iter().all(u8::is_ascii) {
		return Ok(FileIdentity::Text);
	};

	Ok(FileIdentity::Unknown)
}
