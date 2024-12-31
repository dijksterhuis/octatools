use std::path::PathBuf;

use crate::{OctatoolErrors, RBoxErr};
use serde_octatrack::{banks::Bank, FromPath};

/// Show deserialised representation of Pattern state
pub fn show_pattern(path: &PathBuf, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        return Err(Box::new(OctatoolErrors::CliMissingPatternIndices));
    };
    if *indexes.iter().max().unwrap() > 16 || *indexes.iter().min().unwrap() < 1 {
        return Err(Box::new(OctatoolErrors::CliInvalidPatternIndices));
    }

    let b = &Bank::from_path(path).expect("Could not load bank file");

    for index in indexes {
        let x = &b.patterns[index - 1];
        println!("{x:#?}");
    }

    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_show_one_index_ok() {
        let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
        let idxs: Vec<usize> = [1].to_vec();
        let r = show_pattern(&bank_fp, idxs);
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_two_index_ok() {
        let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
        let idxs: Vec<usize> = [1, 16].to_vec();
        let r = show_pattern(&bank_fp, idxs);
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_all_index_ok() {
        let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
        let idxs: Vec<usize> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].to_vec();
        let r = show_pattern(&bank_fp, idxs);
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_no_index_err() {
        let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
        let idxs: Vec<usize> = [].to_vec();
        let r = show_pattern(&bank_fp, idxs);
        assert!(r.is_err())
    }

    #[test]
    fn test_show_one_oob_index_fail() {
        let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
        let idxs: Vec<usize> = [17].to_vec();
        let r = show_pattern(&bank_fp, idxs);
        assert!(r.is_err())
    }

    #[test]
    fn test_show_two_oob_index_fail() {
        let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
        let idxs: Vec<usize> = [17, 24].to_vec();
        let r = show_pattern(&bank_fp, idxs);
        assert!(r.is_err())
    }

    #[test]
    fn test_show_nx_oob_good_index_fail() {
        let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
        let idxs: Vec<usize> = [1, 4, 25, 32].to_vec();
        let r = show_pattern(&bank_fp, idxs);
        assert!(r.is_err())
    }
}
