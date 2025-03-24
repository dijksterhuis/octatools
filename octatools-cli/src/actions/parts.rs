use crate::actions::banks::utils::{
    find_sample_slot_refs_in_parts, get_bank_fname_from_id,
    get_zero_indexed_slots_from_one_indexed, BankSlotReferenceType,
};
use crate::{OctatoolErrors, RBoxErr};
use itertools::Itertools;
use octatools_lib::banks::parts::Part;
use octatools_lib::projects::options::ProjectSampleSlotType;
use octatools_lib::projects::Project;
use octatools_lib::{banks::Bank, read_type_from_bin_file};
use std::path::{Path, PathBuf};

fn part_index_is_valid(indexes: &[usize]) -> bool {
    let max_elem = *indexes.iter().max().unwrap();
    let min_elem = *indexes.iter().min().unwrap();
    max_elem <= 4 && min_elem >= 1
}

/// Show deserialized representation of Part unsaved state
pub fn show_unsaved_parts(path: &Path, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        return Err(Box::new(OctatoolErrors::CliMissingPatternIndex));
    };
    if !part_index_is_valid(&indexes) {
        return Err(Box::new(OctatoolErrors::CliInvalidPartIndex));
    }

    let b = read_type_from_bin_file::<Bank>(path).expect("Could not load bank file");

    for index in indexes {
        let x = &b.parts.saved[index - 1];
        println!("{x:#?}");
    }
    Ok(())
}

/// Show deserialized representation of Part's saved state
pub fn show_saved_parts(path: &Path, indexes: Vec<usize>) -> RBoxErr<()> {
    if indexes.is_empty() {
        return Err(Box::new(OctatoolErrors::CliMissingPatternIndex));
    };
    if !part_index_is_valid(&indexes) {
        return Err(Box::new(OctatoolErrors::CliInvalidPartIndex));
    }

    let b = read_type_from_bin_file::<Bank>(path).expect("Could not load bank file");

    for index in indexes {
        let x = &b.parts.saved[index - 1];
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

// TODO: How to consolidate these two functions to avoid the mess that is duplicated code here?

pub fn list_unsaved_part_sample_slot_references(
    project_dirpath: &Path,
    bank_id: usize,
    part_id: usize,
    ignore_empty_slots: bool,
) -> RBoxErr<()> {
    let project_fpath = project_dirpath.to_path_buf().join("project.work");
    let bank_fpath = project_dirpath
        .to_path_buf()
        .join(get_bank_fname_from_id(bank_id));

    let proj =
        read_type_from_bin_file::<Project>(&project_fpath).expect("Failed to read project file.");
    let bank = read_type_from_bin_file::<Bank>(&bank_fpath).expect("Failed to read bank file.");
    let part = bank.parts.unsaved[part_id - 1].clone();

    list_part_slot_refs(&proj, part, ignore_empty_slots)?;

    Ok(())
}

pub fn list_saved_part_sample_slot_references(
    project_dirpath: &Path,
    bank_id: usize,
    part_id: usize,
    ignore_empty_slots: bool,
) -> RBoxErr<()> {
    let project_fpath = project_dirpath.to_path_buf().join("project.work");
    let bank_fpath = project_dirpath
        .to_path_buf()
        .join(get_bank_fname_from_id(bank_id));

    let proj =
        read_type_from_bin_file::<Project>(&project_fpath).expect("Failed to read project file.");
    let bank = read_type_from_bin_file::<Bank>(&bank_fpath).expect("Failed to read bank file.");
    let part = bank.parts.saved[part_id - 1].clone();

    list_part_slot_refs(&proj, part, ignore_empty_slots)?;

    Ok(())
}

fn list_part_slot_refs(project: &Project, part: Part, ignore_empty_slots: bool) -> RBoxErr<()> {
    find_sample_slot_refs_in_parts(
        &get_zero_indexed_slots_from_one_indexed(&project.slots)?,
        &[part],
    )?
    .iter()
    .filter(|x| ignore_empty_slots != (x.reference_type == BankSlotReferenceType::Active))
    .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
    .map(|x| {
        let path = if x.reference_type == BankSlotReferenceType::Active {
            Some(
                &project
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
