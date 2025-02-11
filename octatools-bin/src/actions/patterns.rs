use std::path::Path;

use crate::{OctatoolErrors, RBoxErr};
use octatools_lib::{banks::Bank, read_type_from_bin_file};

fn pattern_index_is_valid(indexes: &[usize]) -> bool {
    let max_elem = *indexes.iter().max().unwrap();
    let min_elem = *indexes.iter().min().unwrap();
    max_elem <= 16 && min_elem > 0
}

/// Show deserialized representation of Pattern state
pub fn show_pattern(path: &Path, indexes: &[usize]) -> RBoxErr<()> {
    if indexes.is_empty() {
        return Err(Box::new(OctatoolErrors::CliMissingPatternIndices));
    };
    if !pattern_index_is_valid(indexes) {
        return Err(Box::new(OctatoolErrors::CliInvalidPatternIndices));
    }

    let b = read_type_from_bin_file::<Bank>(path).expect("Could not load bank file");

    for index in indexes {
        let x = &b.patterns[index - 1];
        println!("{x:#?}");
    }

    Ok(())
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn test_show_one_index_ok() {
        let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
        let idxs: [usize; 1] = [1];
        let r = show_pattern(bank_fp, &idxs);
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_two_index_ok() {
        let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
        let idxs: [usize; 2] = [1, 16];
        let r = show_pattern(bank_fp, &idxs);
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_all_index_ok() {
        let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
        let idxs: [usize; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let r = show_pattern(bank_fp, &idxs);
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_no_index_err() {
        let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
        let idxs: [_; 0] = [];
        let r = show_pattern(bank_fp, &idxs);
        assert!(r.is_err())
    }

    #[test]
    fn test_show_one_oob_index_fail() {
        let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
        let idxs: [usize; 1] = [17];
        let r = show_pattern(bank_fp, &idxs);
        assert!(r.is_err())
    }

    #[test]
    fn test_show_two_oob_index_fail() {
        let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
        let idxs: [usize; 2] = [17, 24];
        let r = show_pattern(bank_fp, &idxs);
        assert!(r.is_err())
    }

    #[test]
    fn test_show_nx_oob_good_index_fail() {
        let bank_fp = Path::new("../data/tests/blank-project/bank01.work");
        let idxs: [usize; 4] = [1, 4, 25, 32];
        let r = show_pattern(bank_fp, &idxs);
        assert!(r.is_err())
    }
}
