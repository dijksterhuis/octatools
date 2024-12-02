use std::path::PathBuf;

use crate::common::RBoxErr;
use serde_octatrack::{banks::Bank, FromPath};

/// Show deserialised representation of Part unsaved state
pub fn show_unsaved_parts(path: &PathBuf, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        panic!("No Part numbers specified!");
    };
    if *indexes.iter().max().unwrap() > 4 || *indexes.iter().min().unwrap() < 1 {
        panic!("Invalid Part numbers specified! Only 1-4 are allowed.");
    }

    let b = &Bank::from_path(path).expect("Could not load bank file");

    for index in indexes {
        if !(1..=4).contains(&index) {
            panic!("Octatrack Parts are indexed from 1 to 4: partNumber={index:#?}");
        }
        let x = &b.parts_saved[index - 1];
        println!("{x:#?}");
    }
    Ok(())
}

/// Show deserialised representation of Part's saved state
pub fn show_saved_parts(path: &PathBuf, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        panic!("No Part numbers specified!");
    };
    if *indexes.iter().max().unwrap() > 4 || *indexes.iter().min().unwrap() < 1 {
        panic!("Invalid Part numbers specified! Only 1, 2, 3, 4 are allowed");
    }

    let b = &Bank::from_path(path).expect("Could not load bank file");

    for index in indexes {
        if !(1..=4).contains(&index) {
            panic!("Octatrack Parts are indexed from 1 to 4: partNumber={index:#?}");
        }
        let x = &b.parts_saved[index - 1];
        println!("{x:#?}");
    }

    Ok(())
}
