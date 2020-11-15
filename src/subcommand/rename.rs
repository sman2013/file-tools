use std::collections::HashMap;
use std::fs;
use std::fs::ReadDir;
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::subcommand::params::RenameCmd;

pub const ORDER_MODE_CREATED: u8 = 0;
pub const ORDER_MODE_MODIFIED: u8 = 1;
pub const ORDER_MODE_ACCESSED: u8 = 2;

pub fn run(param: RenameCmd) -> Result<(), String> {
    let entries = fs::read_dir(param.shared.dir.clone()).map_err(|e| String::from(format!("{:?}", e)))?;
    rename(entries, &param).map_err(|e| String::from(format!("{:?}", e)))
}

fn rename(entries: ReadDir, param: &RenameCmd) -> Result<(), String> {
    let mut files = HashMap::new();
    let mut keys: Vec<u64> = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    let entries = fs::read_dir(entry.path()).map_err(|e| format!("{:?}", e))?;
                    let _ = rename(entries, param);
                } else {
                    match param.order_mode {
                        ORDER_MODE_CREATED => {
                            let created = meta.created().map_err(|e| format!("{:?}", e))?;
                            let duration = created.duration_since(SystemTime::UNIX_EPOCH).map_err(|e| format!("{:?}", e))?.as_secs();
                            files.insert(duration, entry);
                            keys.push(duration);
                        }
                        ORDER_MODE_MODIFIED => {
                            let modified = meta.modified().map_err(|e| format!("{:?}", e))?;
                            let duration = modified.duration_since(SystemTime::UNIX_EPOCH).map_err(|e| format!("{:?}", e))?.as_secs();
                            files.insert(duration, entry);
                            keys.push(duration);
                        }
                        ORDER_MODE_ACCESSED => {
                            let accessed = meta.accessed().map_err(|e| format!("{:?}", e))?;
                            let duration = accessed.duration_since(SystemTime::UNIX_EPOCH).map_err(|e| format!("{:?}", e))?.as_secs();
                            files.insert(duration, entry);
                            keys.push(duration);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    keys.sort_by(|a, b| a.cmp(b));
    for key in keys {
        let file = match files.get(&key) {
            Some(f) => f,
            None => continue
        };
        // todo
    }
    Ok(())
}