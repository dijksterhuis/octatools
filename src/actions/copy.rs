//! CLI actions related to copying Octatrack entities

use log::{debug, error, info, warn};
use std::error::Error;
use std::path::PathBuf;

use serde_octatrack::{
    banks::Bank,
    common::{FromFileAtPathBuf, RBoxErr, ToFileAtPathBuf},
    projects::{slots::ProjectSampleSlots, Project},
};

/// Transfer a bank from one project to another project

pub fn transfer_bank(
    source_bank_file_path: PathBuf,
    dest_bank_file_path: PathBuf,
    merge_duplicate_sample_slots: bool,
) -> Result<(), ()> {
    // === take sample slots and copy them to new slots in new project ===
    // ===================================================================
    //
    // 1. read old project
    // 2. get sample slots
    // 3. read new project
    // 4. find space in new project sample slots
    // 5. read src bank data
    //  * machine assigned sample slots
    //  * trig sample lock assigned sample slots
    // 6. edit read bank data sample slot usage
    // 7. edit read bank data sample slots
    //  *  machine assignment
    //  *  trig smaple lock assignment
    // 8. create backup files
    //  * new project
    //  * new bank file
    // 9. copy samples to new project folder
    //  * todo: add a .txt log file detailing copied files?
    // 10. add samples to project sample slots
    // 11. write over project file
    // 11. write new bank data over old bank

    // 1. read old project

    let src_proj_path = source_bank_file_path
        .parent()
        .unwrap()
        .to_path_buf()
        .join("project.work");

    let src_dirpath = &src_proj_path.parent().unwrap().to_path_buf();
    let src_project = Project::from_pathbuf(src_proj_path).unwrap();

    // 2. get sample slots
    let src_sample_slots: Vec<ProjectSampleSlots> = src_project
        .slots
        .into_iter()
        .filter(|x| x.slot_id < 128) // no recording buffers
        .collect();

    // 5. read src bank data
    //  * machine assigned sample slots
    //  * trig sample lock assigned sample slots
    let src_bank_data = Bank::from_pathbuf(source_bank_file_path).unwrap();

    let x = [0, 15];

    for i in x {
        println!("-------------");

        println!(
            "PTRN AT - TMASKS {:#?}",
            src_bank_data.patterns[i].audio_track_trigs[0].trig_masks
        );
        println!(
            "PTRN AT - SCALE {:#?}",
            src_bank_data.patterns[i].audio_track_trigs[0].scale_per_track_mode
        );
        println!(
            "PTRN AT - SWING {:#?}",
            src_bank_data.patterns[i].audio_track_trigs[0].swing_amount
        );
        println!(
            "PTRN AT - PTRN SETTINGS {:#?}",
            src_bank_data.patterns[i].audio_track_trigs[0].pattern_settings
        );
        println!(
            "PTRN AT - UKN1 {:#?}",
            src_bank_data.patterns[i].audio_track_trigs[0].unknown_1
        );
        println!(
            "PTRN AT - UKN4 {:#?}",
            src_bank_data.patterns[i].audio_track_trigs[0].unknown_4
        );
        println!(
            "PTRN AT - UKN5 {:#?}",
            src_bank_data.patterns[i].audio_track_trigs[0].unknown_5
        );

        println!("-------------");

        println!("PTRN - SCALE {:#?}", src_bank_data.patterns[i].scale);
        println!(
            "PTRN - CHAINBEHAV {:#?}",
            src_bank_data.patterns[i].chain_behaviour
        );
        println!("PTRN - UKN2 {:#?}", src_bank_data.patterns[i].unknown);
        println!("PTRN - TEMPO1 {:#?}", src_bank_data.patterns[i].tempo_1);
        println!("PTRN - TEMPO2 {:#?}", src_bank_data.patterns[i].tempo_2);
    }

    let _ = src_bank_data.to_pathbuf(dest_bank_file_path);

    Ok(())
}
