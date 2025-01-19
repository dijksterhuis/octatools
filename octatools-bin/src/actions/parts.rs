use crate::{OctatoolErrors, RBoxErr};
use serde_octatrack::{banks::Bank, read_type_from_bin_file};
use std::path::Path;

fn part_index_is_valid(indexes: &[usize]) -> bool {
    let max_elem = *indexes.iter().max().unwrap();
    let min_elem = *indexes.iter().min().unwrap();
    max_elem <= 4 && min_elem >= 1
}

/// Show deserialized representation of Part unsaved state
pub fn show_unsaved_parts(path: &Path, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        return Err(Box::new(OctatoolErrors::CliMissingPatternIndices));
    };
    if !part_index_is_valid(&indexes) {
        return Err(Box::new(OctatoolErrors::CliInvalidPartIndices));
    }

    let b = read_type_from_bin_file::<Bank>(path).expect("Could not load bank file");

    for index in indexes {
        let x = &b.parts_saved[index - 1];
        println!("{x:#?}");
    }
    Ok(())
}

/// Show deserialized representation of Part's saved state
pub fn show_saved_parts(path: &Path, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        return Err(Box::new(OctatoolErrors::CliMissingPatternIndices));
    };
    if !part_index_is_valid(&indexes) {
        return Err(Box::new(OctatoolErrors::CliInvalidPartIndices));
    }

    let b = read_type_from_bin_file::<Bank>(path).expect("Could not load bank file");

    for index in indexes {
        let x = &b.parts_saved[index - 1];
        println!("{x:#?}");
    }

    Ok(())
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;

    mod unsaved {
        use super::*;

        #[test]
        fn test_show_one_index_ok() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1].to_vec();
            let r = show_unsaved_parts(bank_fp, idxs);
            assert!(r.is_ok())
        }
        #[test]
        fn test_show_two_index_ok() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 4].to_vec();
            let r = show_unsaved_parts(bank_fp, idxs);
            assert!(r.is_ok())
        }

        #[test]
        fn test_show_all_index_ok() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 2, 3, 4].to_vec();
            let r = show_unsaved_parts(bank_fp, idxs);
            assert!(r.is_ok())
        }

        #[test]
        fn test_show_no_index_err() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [].to_vec();
            let r = show_unsaved_parts(bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_one_oob_index_fail() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [5].to_vec();
            let r = show_unsaved_parts(bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_two_oob_index_fail() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [6, 24].to_vec();
            let r = show_unsaved_parts(bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_nx_oob_good_index_fail() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 4, 25, 32].to_vec();
            let r = show_unsaved_parts(bank_fp, idxs);
            assert!(r.is_err())
        }
    }

    mod saved {
        use super::*;

        #[test]
        fn test_show_one_index_ok() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1].to_vec();
            let r = show_saved_parts(bank_fp, idxs);
            assert!(r.is_ok())
        }
        #[test]
        fn test_show_two_index_ok() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 4].to_vec();
            let r = show_saved_parts(bank_fp, idxs);
            assert!(r.is_ok())
        }

        #[test]
        fn test_show_all_index_ok() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 2, 3, 4].to_vec();
            let r = show_saved_parts(bank_fp, idxs);
            assert!(r.is_ok())
        }

        #[test]
        fn test_show_no_index_err() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [].to_vec();
            let r = show_saved_parts(bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_one_oob_index_fail() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [5].to_vec();
            let r = show_saved_parts(bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_two_oob_index_fail() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [6, 24].to_vec();
            let r = show_saved_parts(bank_fp, idxs);
            assert!(r.is_err())
        }

        #[test]
        fn test_show_nx_oob_good_index_fail() {
            let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
            let idxs: Vec<usize> = [1, 4, 25, 32].to_vec();
            let r = show_saved_parts(bank_fp, idxs);
            assert!(r.is_err())
        }
    }
}
