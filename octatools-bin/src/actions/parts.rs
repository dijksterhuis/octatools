use std::path::PathBuf;

use crate::{OctatoolErrors, RBoxErr};
use serde_octatrack::{banks::Bank, FromPath};

/// Show deserialised representation of Part unsaved state
pub fn show_unsaved_parts(path: &PathBuf, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        return Err(Box::new(OctatoolErrors::CliMissingPatternIndices));
    };
    if *indexes.iter().max().unwrap() > 4 || *indexes.iter().min().unwrap() < 1 {
        return Err(Box::new(OctatoolErrors::CliInvalidPartIndices));
    }

    let b = &Bank::from_path(path).expect("Could not load bank file");

    for index in indexes {
        let x = &b.parts_saved[index - 1];
        println!("{x:#?}");
    }
    Ok(())
}

/// Show deserialised representation of Part's saved state
pub fn show_saved_parts(path: &PathBuf, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        return Err(Box::new(OctatoolErrors::CliMissingPatternIndices));
    };
    if *indexes.iter().max().unwrap() > 4 || *indexes.iter().min().unwrap() < 1 {
        return Err(Box::new(OctatoolErrors::CliInvalidPartIndices));
    }

    let b = &Bank::from_path(path).expect("Could not load bank file");

    for index in indexes {
        let x = &b.parts_saved[index - 1];
        println!("{x:#?}");
    }

    Ok(())
}

mod test {
    use super::*;

    mod unsaved {
        use super::*;

        #[test]
        fn test_show_one_index_ok() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1].to_vec();
            let r = show_unsaved_parts(&bank_fp, idxs);
            assert!(r.is_ok())
        }
        #[test]
        fn test_show_two_index_ok() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 4].to_vec();
            let r = show_unsaved_parts(&bank_fp, idxs);
            assert!(r.is_ok())
        }

        #[test]
        fn test_show_all_index_ok() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 2, 3, 4].to_vec();
            let r = show_unsaved_parts(&bank_fp, idxs);
            assert!(r.is_ok())
        }

        #[test]
        fn test_show_no_index_err() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [].to_vec();
            let r = show_unsaved_parts(&bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_one_oob_index_fail() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [5].to_vec();
            let r = show_unsaved_parts(&bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_two_oob_index_fail() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [6, 24].to_vec();
            let r = show_unsaved_parts(&bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_nx_oob_good_index_fail() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 4, 25, 32].to_vec();
            let r = show_unsaved_parts(&bank_fp, idxs);
            assert!(r.is_err())
        }
    }

    mod saved {
        use super::*;

        #[test]
        fn test_show_one_index_ok() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1].to_vec();
            let r = show_saved_parts(&bank_fp, idxs);
            assert!(r.is_ok())
        }
        #[test]
        fn test_show_two_index_ok() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 4].to_vec();
            let r = show_saved_parts(&bank_fp, idxs);
            assert!(r.is_ok())
        }

        #[test]
        fn test_show_all_index_ok() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 2, 3, 4].to_vec();
            let r = show_saved_parts(&bank_fp, idxs);
            assert!(r.is_ok())
        }

        #[test]
        fn test_show_no_index_err() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [].to_vec();
            let r = show_saved_parts(&bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_one_oob_index_fail() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [5].to_vec();
            let r = show_saved_parts(&bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_two_oob_index_fail() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [6, 24].to_vec();
            let r = show_saved_parts(&bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_nx_oob_good_index_fail() {
            let bank_fp = PathBuf::from("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 4, 25, 32].to_vec();
            let r = show_saved_parts(&bank_fp, idxs);
            assert!(r.is_err())
        }
    }
}
