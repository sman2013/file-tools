use std::collections::HashMap;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;
use std::time::{SystemTime};

use crate::subcommand::params::RenameCmd;

pub const ORDER_MODE_CREATED: u8 = 0;
pub const ORDER_MODE_MODIFIED: u8 = 1;
pub const ORDER_MODE_ACCESSED: u8 = 2;

/// 0 1 2 3
const SEQUENCE_MODE_DECIMAL: u8 = 0;
/// (0) (1) (3)
const SEQUENCE_MODE_DECIMAL_X: u8 = 1;
/// [1] [2] [3]
const SEQUENCE_MODE_DECIMAL_Z: u8 = 2;
/// {0} {1} {2}
const SEQUENCE_MODE_DECIMAL_D: u8 = 3;
/// a b c
const SEQUENCE_MODE_ABC: u8 = 4;
/// 一 二 三
const SEQUENCE_MODE_CHINESE: u8 = 5;

/// 1*** 2***
const SEQUENCE_POSITION_HEAD: u8 = 0;
/// ***1.*** ***2.***
const SEQUENCE_POSITION_MIDDLE: u8 = 1;
/// ***.***1 ***.***2
const SEQUENCE_POSITION_TAIL: u8 = 2;

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
    for (i, key) in keys.into_iter().enumerate() {
        let entry = match files.get(&key) {
            Some(f) => f,
            None => continue
        };
        let i = i as u16;
        // todo
        let name = if param.remain {
            match param.sequence_position {
                SEQUENCE_POSITION_HEAD => {
                    let name = entry.file_name().into_string().unwrap_or_default();
                    match param.sequence_mode {
                        SEQUENCE_MODE_DECIMAL => {
                            format!("{}{}{}", i + param.sequence_start, param.connector, name)
                        }
                        SEQUENCE_MODE_DECIMAL_X => {
                            format!("({}){}{}", i + param.sequence_start, param.connector, name)
                        }
                        SEQUENCE_MODE_DECIMAL_Z => {
                            format!("[{}]{}{}", i + param.sequence_start, param.connector, name)
                        }
                        SEQUENCE_MODE_DECIMAL_D => {
                            format!("{{{}}}{}{}", i + param.sequence_start, param.connector, name)
                        }
                        // SEQUENCE_MODE_ABC => {
                        //     // todo
                        // }
                        // SEQUENCE_MODE_CHINESE => {
                        //     // todo
                        // }
                        _ => Default::default()
                    }
                }
                SEQUENCE_POSITION_MIDDLE => {
                    let old = entry.file_name().into_string().unwrap_or_default();
                    match old.rfind(".") {
                        Some(index) => {
                            let (n, e) = old.split_at(index);
                            match param.sequence_mode {
                                SEQUENCE_MODE_DECIMAL => {
                                    format!("{}{}{}{}", n, param.connector, i + param.sequence_start, e)
                                }
                                SEQUENCE_MODE_DECIMAL_X => {
                                    format!("{}{}({}){}", n, param.connector, i + param.sequence_start, e)
                                }
                                SEQUENCE_MODE_DECIMAL_Z => {
                                    format!("{}{}[{}]{}", n, param.connector, i + param.sequence_start, e)
                                }
                                SEQUENCE_MODE_DECIMAL_D => {
                                    format!("{}{}{{{}}}{}", n, param.connector, i + param.sequence_start, e)
                                }
                                // SEQUENCE_MODE_ABC => {
                                //     // todo
                                // }
                                // SEQUENCE_MODE_CHINESE => {
                                //     // todo
                                // }
                                _ => Default::default()
                            }
                        }
                        _ => return Ok(())
                    }
                }
                SEQUENCE_POSITION_TAIL => {
                    let name = entry.file_name().into_string().unwrap_or_default();
                    match param.sequence_mode {
                        SEQUENCE_MODE_DECIMAL => {
                            format!("{}{}{}", name, param.connector, i + param.sequence_start)
                        }
                        SEQUENCE_MODE_DECIMAL_X => {
                            format!("{}{}({})", name, param.connector, i + param.sequence_start)
                        }
                        SEQUENCE_MODE_DECIMAL_Z => {
                            format!("{}{}[{}]", name, param.connector, i + param.sequence_start)
                        }
                        SEQUENCE_MODE_DECIMAL_D => {
                            format!("{}{}{{{}}}", name, param.connector, i + param.sequence_start)
                        }
                        // SEQUENCE_MODE_ABC => {
                        //     // todo
                        // }
                        // SEQUENCE_MODE_CHINESE => {
                        //     // todo
                        // }
                        _ => Default::default()
                    }
                }
                _ => Default::default()
            }
        } else {
            match param.sequence_position {
                SEQUENCE_POSITION_HEAD => {
                    match param.sequence_mode {
                        SEQUENCE_MODE_DECIMAL => {
                            format!("{}{}{}", i + param.sequence_start, param.connector, param.shard_str.clone())
                        }
                        SEQUENCE_MODE_DECIMAL_X => {
                            format!("({}){}{}", i + param.sequence_start, param.connector, param.shard_str.clone())
                        }
                        SEQUENCE_MODE_DECIMAL_Z => {
                            format!("[{}]{}{}", i + param.sequence_start, param.connector, param.shard_str.clone())
                        }
                        SEQUENCE_MODE_DECIMAL_D => {
                            format!("{{{}}}{}{}", i + param.sequence_start, param.connector, param.shard_str.clone())
                        }
                        // SEQUENCE_MODE_ABC => {
                        //     // todo
                        // }
                        // SEQUENCE_MODE_CHINESE => {
                        //     // todo
                        // }
                        _ => Default::default()
                    }
                }
                SEQUENCE_POSITION_TAIL => {
                    match param.sequence_mode {
                        SEQUENCE_MODE_DECIMAL => {
                            format!("{}{}{}", param.shard_str.clone(), param.connector, i + param.sequence_start)
                        }
                        SEQUENCE_MODE_DECIMAL_X => {
                            format!("{}{}({})", param.shard_str.clone(), param.connector, i + param.sequence_start)
                        }
                        SEQUENCE_MODE_DECIMAL_Z => {
                            format!("{}{}[{}]", param.shard_str.clone(), param.connector, i + param.sequence_start)
                        }
                        SEQUENCE_MODE_DECIMAL_D => {
                            format!("{}{}{{{}}}", param.shard_str.clone(), param.connector, i + param.sequence_start)
                        }
                        // SEQUENCE_MODE_ABC => {
                        //     // todo
                        // }
                        // SEQUENCE_MODE_CHINESE => {
                        //     // todo
                        // }
                        _ => Default::default()
                    }
                }
                _ => panic!("when `remain` is false, `sequence_position` can only be 0: head, 1: tail")
            }
        };

        let path = entry.path();
        let dir = match path.parent() {
            Some(v) => v,
            None => return Ok(())
        };
        let _ = fs::rename(entry.path(), dir.join(Path::new(name.as_str())));
    }
    Ok(())
}