//! Module containing code related to running commands

pub mod arrangements;
pub mod banks;
pub mod drive;
pub mod parts;
pub mod patterns;
pub mod projects;
pub mod samples;

use crate::RBoxErr;
use serde_octatrack::{FromYamlFile, ToPath};
use std::path::Path;

fn get_bytes_slice(data: Vec<u8>, start_idx: &Option<usize>, len: &Option<usize>) -> Vec<u8> {
    let start: usize = match start_idx {
        None => 0,
        _ => start_idx.unwrap(),
    };

    let end: usize = match len {
        None => data.len() - 1,
        _ => len.unwrap() + start,
    };

    data[start..end].to_vec()
}

/// Load binary file data from a YAML file
fn load_from_yaml<T: FromYamlFile + ToPath>(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    let b = T::from_yaml(yaml_path).expect("Could not load YAML file");
    b.to_path(outfile).expect("Could not write data to file");
    Ok(())
}
