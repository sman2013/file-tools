use std::fs;
use std::fs::ReadDir;
use std::io;
use std::path::Path;

use crate::subcommand::params::InsertCmd;

pub fn run(param: InsertCmd) -> Result<(), String> {
    let entries = fs::read_dir(param.shared.dir).map_err(|e| String::from(format!("{:?}", e)))?;
    insert(entries, param.position, param.str, param.rename_dir).map_err(|e| String::from(format!("{:?}", e)))
}

fn insert(entries: ReadDir, position: String, str: String, rename_dir: bool) -> io::Result<()> {
    match position.to_lowercase().as_str() {
        "head" | "middle" | "tail" => {}
        _ => { return Ok(()); }
    }
    for entry in entries {
        if let Ok(entry) = entry {
            let is_dir = if let Ok(ft) = entry.file_type() {
                ft.is_dir()
            } else {
                false
            };
            let rename = if is_dir {
                let entries = fs::read_dir(entry.path())?;
                insert(entries, position.clone(), str.clone(), rename_dir)?;
                rename_dir
            } else {
                true
            };
            if rename {
                let old = entry.file_name().into_string().unwrap_or(String::default());
                let new = match position.to_lowercase().as_str() {
                    "head" => {
                        format!("{}{}", str, old)
                    }
                    "middle" => {
                        match old.rfind(".") {
                            Some(i) => {
                                let (n, e) = old.split_at(i);
                                format!("{}{}{}", n, str, e)
                            }
                            _ => { old }
                        }
                    }
                    "tail" => {
                        format!("{}{}", old, str)
                    }
                    _ => { old }
                };

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