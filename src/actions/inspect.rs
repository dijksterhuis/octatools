use std::path::PathBuf;

use serde_octatrack::{
    arrangements::{ArrangementFile, ArrangementFileRawBytes},
    banks::Bank,
    common::{FromFileAtPathBuf, RBoxErr},
    projects::Project,
    samples::SampleAttributes,
};

pub fn show_bank(path: &PathBuf) -> RBoxErr<()> {
    let b = Bank::from_pathbuf(&path)?;
    println!("{b:#?}");
    Ok(())
}

pub fn show_patterns(path: &PathBuf) -> RBoxErr<()> {
    let b = Bank::from_pathbuf(&path)?.patterns;
    println!("{b:#?}");
    Ok(())
}

pub fn show_pattern(path: &PathBuf, index: usize) -> RBoxErr<()> {
    let b = &Bank::from_pathbuf(&path)?.patterns[index];
    println!("{b:#?}");
    Ok(())
}

pub fn show_parts(path: &PathBuf) -> RBoxErr<()> {
    let b = Bank::from_pathbuf(&path)?.parts;
    println!("{b:#?}");
    Ok(())
}

pub fn show_part(path: &PathBuf, index: usize) -> RBoxErr<()> {
    let b = &Bank::from_pathbuf(&path)?.parts[index];
    println!("{b:#?}");
    Ok(())
}

pub fn show_project(path: &PathBuf) -> RBoxErr<()> {
    let b = Project::from_pathbuf(&path)?;
    println!("{b:#?}");
    Ok(())
}

pub fn show_ot_file(path: &PathBuf) -> RBoxErr<()> {
    let b = SampleAttributes::from_pathbuf(&path)?;
    println!("{b:#?}");
    Ok(())
}

pub fn show_arrangement(path: &PathBuf) -> RBoxErr<()> {
    let b = ArrangementFile::from_pathbuf(&path);
    println!("ARRANGE: {b:#?}");
    Ok(())
}

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
