//! Functions to inspect/show the deserialised output of a specific data file.

use std::path::PathBuf;

use serde_octatrack::{
    arrangements::{ArrangementFile, ArrangementFileRawBytes},
    banks::Bank,
    common::{FromFileAtPathBuf, RBoxErr},
    projects::Project,
    samples::SampleAttributes,
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
    println!("ARRANGE: {b:#?}");
    Ok(())
}

/// Show bytes output as u8 values for an Arrangement file located at `path`
pub fn show_arrangement_bytes(
    path: &PathBuf,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let b = ArrangementFileRawBytes::from_pathbuf(&path)?.data.to_vec();
    let start: usize = if start_idx.is_none() {
        0
    } else {
        start_idx.unwrap()
    };
    let end: usize = if len.is_none() {
        b.len() - 1
    } else {
        len.unwrap() + start
    };

    let b = &b[start..end];
    println!("ARRANGE: {b:#?}");
    Ok(())
}
