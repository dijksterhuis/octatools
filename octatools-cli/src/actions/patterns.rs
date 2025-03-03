use std::path::{Path, PathBuf};

use crate::actions::banks::utils::{
    find_sample_slot_refs_in_patterns, get_bank_fname_from_id,
    get_zero_indexed_slots_from_one_indexed, BankSlotReferenceType,
};
use crate::{OctatoolErrors, RBoxErr};
use itertools::Itertools;
use octatools_lib::{
    banks::Bank, projects::options::ProjectSampleSlotType, projects::Project,
    read_type_from_bin_file,
};

pub(crate) fn pattern_index_is_valid(indexes: &[usize]) -> bool {
    let max_elem = *indexes.iter().max().unwrap();
    let min_elem = *indexes.iter().min().unwrap();
    max_elem <= 16 && min_elem > 0
}

/// Show deserialized representation of Pattern state
pub fn show_pattern(path: &Path, indexes: &[usize]) -> RBoxErr<()> {
    if indexes.is_empty() {
        return Err(Box::new(OctatoolErrors::CliMissingPatternIndex));
    };
    if !pattern_index_is_valid(indexes) {
        return Err(Box::new(OctatoolErrors::CliInvalidPatternIndex));
    }

    let b = read_type_from_bin_file::<Bank>(path).expect("Could not load bank file");

    for index in indexes {
        let x = &b.patterns[index - 1];
        println!("{x:#?}");
    }

    Ok(())
}

// TODO: Duplicated from actions/bank.rs!
#[derive(Debug)]
#[allow(dead_code)] // clippy doesn't detect the usage below for some reason
struct SlotUseListItem {
    sample_loaded: bool,
    sample_type: ProjectSampleSlotType,
    slot_id: u8,
    path: Option<PathBuf>,
}

pub fn list_pattern_sample_slot_references(
    project_dirpath: &Path,
    bank_id: usize,
    pattern_id: usize,
    ignore_empty_slots: bool,
) -> RBoxErr<()> {
    let project_fpath = project_dirpath.to_path_buf().join("project.work");

    let bank_fpath = project_dirpath
        .to_path_buf()
        .join(get_bank_fname_from_id(bank_id));

    let src_project =
        read_type_from_bin_file::<Project>(&project_fpath).expect("Failed to read project file.");

    // we will be changing this in place so needs to be mutable
    let bank = read_type_from_bin_file::<Bank>(&bank_fpath).expect("Failed to read bank file.");

    let pattern = bank.patterns[pattern_id - 1].clone();

    find_sample_slot_refs_in_patterns(
        &get_zero_indexed_slots_from_one_indexed(&src_project.slots)?,
        &[pattern],
    )?
    .iter()
    .filter(|x| ignore_empty_slots != (x.reference_type == BankSlotReferenceType::Active))
    .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
    .map(|x| {
        let path = if x.reference_type == BankSlotReferenceType::Active {
            Some(
                &src_project
                    .slots
                    .iter()
                    .find(|s| s.slot_id == x.slot_id)
                    .unwrap()
                    .path,
            )
        } else {
            None
        };

        SlotUseListItem {
            sample_loaded: (x.reference_type == BankSlotReferenceType::Active),
            sample_type: x.sample_type.clone(),
            slot_id: x.slot_id + 1,
            path: path.cloned(),
        }
    })
    .for_each(|x| println!("{:?}", x));

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
