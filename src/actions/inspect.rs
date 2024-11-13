use std::path::PathBuf;

use serde_octatrack::{
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
    let b = SampleAttributes::from_file(path.to_str().unwrap())?;
    println!("{b:#?}");
    Ok(())
}
