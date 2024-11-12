//! Various utilities realted to Octatrack data files, but not used during
//! Serialization / Deserialization.

use crate::common::RBoxErr;
use std::{ffi::OsStr, fmt::Error, path::PathBuf};

#[allow(dead_code)]
fn pathbuf_to_fname(path: &PathBuf) -> RBoxErr<String> {
    let name = path
        .clone()
        .file_name()
        .unwrap_or(&OsStr::new("err"))
        .to_str()
        .unwrap_or("err")
        .to_string();

    if name == "err" {
        return Err(Box::new(Error));
    };
    Ok(name)
}
