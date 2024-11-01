use std::path::PathBuf;
use serde::de::DeserializeOwned;
use crate::utils::io::read_file::read_file;

pub fn read_toml<T : DeserializeOwned>(path_buf: PathBuf) -> std::io::Result<T> {
	let file = read_file(path_buf)?;
	match toml::from_str::<T>(file.as_str()) {
		Ok(t) => Ok(t),
		Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
	}
}