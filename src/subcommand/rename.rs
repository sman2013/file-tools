use std::fs;
use std::fs::ReadDir;
use std::io;
use std::path::Path;

use crate::subcommand::params::RenameCmd;

pub fn run(param: RenameCmd) -> Result<(), String> {
    let entries = fs::read_dir(param.shared.dir.clone()).map_err(|e| String::from(format!("{:?}", e)))?;
    rename(entries, param).map_err(|e| String::from(format!("{:?}", e)))
}

fn rename(entries: ReadDir, param: RenameCmd) -> Result<(), String> {
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(meta) =  entry.metadata() {

            }
        }
    }
    Ok(())
}