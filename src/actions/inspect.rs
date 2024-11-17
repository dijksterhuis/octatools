//! Functions to inspect/show the deserialised output of a specific data file.

use std::path::PathBuf;

use serde_octatrack::{
    arrangements::{ArrangementFile, ArrangementFileRawBytes},
    banks::{Bank, BankRawBytes},
    common::{FromFileAtPathBuf, RBoxErr},
    projects::Project,
    samples::{SampleAttributes, SampleAttributesRawBytes},
};

/// Show deserialised representation of a Bank for a given bank file at `path`
pub fn show_bank(path: &PathBuf) -> RBoxErr<()> {
    let b = Bank::from_pathbuf(&path)?;
    println!("{b:#?}");
    Ok(())
}

/// Show deserialised representation of all Patterns for a given bank file at `path`
pub fn show_patterns(path: &PathBuf) -> RBoxErr<()> {
    let b = Bank::from_pathbuf(&path)?.patterns;
    println!("{b:#?}");
    Ok(())
}

/// Show deserialised representation of one Pattern for a given bank file at `path`
pub fn show_pattern(path: &PathBuf, index: usize) -> RBoxErr<()> {
    let b = &Bank::from_pathbuf(&path)?.patterns[index];
    println!("{b:#?}");
    Ok(())
}

/// Show deserialised representation of all Parts for a given bank file at `path`
pub fn show_parts(path: &PathBuf) -> RBoxErr<()> {
    let b = Bank::from_pathbuf(&path)?.parts;
    println!("{b:#?}");
    Ok(())
}

/// Show deserialised representation of one part for a given bank file at `path`
pub fn show_part(path: &PathBuf, index: usize) -> RBoxErr<()> {
    let b = &Bank::from_pathbuf(&path)?.parts[index];
    println!("{b:#?}");
    Ok(())
}

/// Show deserialised representation of a Project for a given project file at `path`
pub fn show_project(path: &PathBuf) -> RBoxErr<()> {
    let b = Project::from_pathbuf(&path)?;
    println!("{b:#?}");
    Ok(())
}

/// Show deserialised representation of a Sample Attributes file at `path`
pub fn show_ot_file(path: &PathBuf) -> RBoxErr<()> {
    let b = SampleAttributes::from_pathbuf(&path)?;
    println!("{b:#?}");
    Ok(())
}

/// Show deserialised representation of an Arrangement for a given arrangement file at `path`
pub fn show_arrangement(path: &PathBuf) -> RBoxErr<()> {
    let b = ArrangementFile::from_pathbuf(&path);
    println!("{b:#?}");
    Ok(())
}


fn get_bytes_slice(data: Vec<u8>, start_idx: &Option<usize>, len: &Option<usize>) -> Vec<u8> {
    let start: usize = match start_idx {
        None => {0}
        _ => {start_idx.unwrap()}
    };

    let end: usize = match len {
        None => {data.len() - 1}
        _ => {len.unwrap() + start}
    };

    data[start..end].to_vec()
}


/// Show bytes output as u8 values for an Arrangement file located at `path`
pub fn show_arrangement_bytes(
    path: &PathBuf,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let bytes = get_bytes_slice(
        ArrangementFileRawBytes::from_pathbuf(&path)?.data.to_vec(),
        start_idx,
        len,
    );
    println!("{:#?}", bytes);
    Ok(())
}


/// Show bytes output as u8 values for a Bank file located at `path`
pub fn show_bank_bytes(
    path: &PathBuf,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let bytes = get_bytes_slice(
        BankRawBytes::from_pathbuf(&path)?.data.to_vec(),
        start_idx,
        len,
    );
    println!("{:#?}", bytes);
    Ok(())
}

/// Show bytes output as u8 values for a Sample Attributes file located at `path`
pub fn show_ot_file_bytes(
    path: &PathBuf,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let bytes = get_bytes_slice(
        SampleAttributesRawBytes::from_pathbuf(&path)?.data.to_vec(),
        start_idx,
        len,
    );
    println!("{:#?}", bytes);
    Ok(())
}