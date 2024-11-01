use std::path::PathBuf;

pub fn read_file(path_buf: PathBuf) -> std::io::Result<String> {
	if !path_buf.exists() {
		return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
	}

	Ok(std::fs::read(path_buf)?
		.iter()
		.map(|c| *c as char)
		.collect::<String>())
}
