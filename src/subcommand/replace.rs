use std::fs;
use std::fs::ReadDir;
use std::io;
use std::path::Path;

use crate::subcommand::params::ReplaceCmd;

pub fn run(param: ReplaceCmd) -> Result<(), String> {
	let entries = fs::read_dir(param.shared.dir).map_err(|e| String::from(format!("{:?}", e)))?;
	replace(entries, param.src, param.dst.unwrap_or(String::default()), param.rename_dir).map_err(|e| String::from(format!("{:?}", e)))
}

fn replace(entries: ReadDir, src: String, dst: String, rename_dir: bool) -> io::Result<()> {
	for entry in entries {
		if let Ok(entry) = entry {
			let is_dir = if let Ok(ft) = entry.file_type() {
				ft.is_dir()
			} else {
				false
			};
			let rename = if is_dir {
				let entries = fs::read_dir(entry.path())?;
				replace(entries, src.clone(), dst.clone(), rename_dir)?;
				rename_dir
			} else {
				true
			};

			if rename {
				let old = entry.file_name().into_string().unwrap_or(String::default());
				let new = old.replace(src.as_str(), dst.as_str());

				let path = entry.path();
				let dir = match path.parent() {
					Some(v) => v,
					None => return Ok(())
				};
				let _ = fs::rename(entry.path(), dir.join(Path::new(new.as_str())));
			}
		}
	}
	Ok(())
}
