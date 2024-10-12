use std::path::PathBuf;
use serde::de::DeserializeOwned;

pub fn read_file(path_buf: PathBuf) -> std::io::Result<String> {
	if !path_buf.exists() {
		return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
	}

	Ok(std::fs::read(path_buf)?
		.iter()
		.map(|c| *c as char)
		.collect::<String>())
}

pub fn read_toml<T : DeserializeOwned>(path_buf: PathBuf) -> std::io::Result<T> {
	let file = read_file(path_buf)?;
	match toml::from_str::<T>(file.as_str()) {
		Ok(t) => Ok(t),
		Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
	}
}