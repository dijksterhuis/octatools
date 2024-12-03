use std::path::{Path, PathBuf};

use crate::{actions::get_bytes_slice, RBoxErr};
use serde_octatrack::{
    arrangements::{ArrangementFile, ArrangementFileRawBytes},
    FromPath,
};

/// Show deserialised representation of an Arrangement for a given arrangement file at `path`
pub fn show_arrangement(path: &PathBuf) -> RBoxErr<()> {
    let b = ArrangementFile::from_path(path).expect("Could not load arrangement file");
    println!("{b:#?}");
    Ok(())
}

/// Show bytes output as u8 values for an Arrangement file located at `path`
pub fn show_arrangement_bytes(
    path: &PathBuf,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let bytes = get_bytes_slice(
        ArrangementFileRawBytes::from_path(path)
            .expect("Could not load arrangement file")
            .data
            .to_vec(),
        start_idx,
        len,
    );
    println!("{:#?}", bytes);
    Ok(())
}

/// Load Arrangement file data from a YAML file
pub fn load_arrangement(_yaml_path: &Path, _outfile: &Path) -> RBoxErr<()> {
    unimplemented!("Need to deal with intermediate struct conversions.")
}

/// Dump Arrangement file data to a YAML file
pub fn dump_arrangement(_path: &Path, _yaml_path: &Path) -> RBoxErr<()> {
    unimplemented!("Need to deal with intermediate struct conversions.")
}
