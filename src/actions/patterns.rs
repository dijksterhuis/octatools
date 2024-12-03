use std::path::PathBuf;

use crate::RBoxErr;
use serde_octatrack::{banks::Bank, FromPath};

/// Show deserialised representation of Pattern state
pub fn show_pattern(path: &PathBuf, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        panic!("No Pattern numbers specified!");
    };
    if *indexes.iter().max().unwrap() > 16 || *indexes.iter().min().unwrap() < 1 {
        panic!("Invalid Pattern numbers specified! Only 1-16 are allowed.");
    }

    let b = &Bank::from_path(path).expect("Could not load bank file");

    for index in indexes {
        if !(1..=16).contains(&index) {
            panic!("Octatrack Patterns are indexed from 1 to 16");
        }
        let x = &b.patterns[index - 1];
        println!("{x:#?}");
    }

    Ok(())
}
