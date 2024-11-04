//! CLI 'actions' functions


use log::{debug, error, info, warn};
use std::collections::HashSet;
use std::path::PathBuf;
use std::{fs::copy, process::exit};

use crate::octatrack::samples::{
    configs::{SampleLoopConfig, SampleTrimConfig},
    get_otsample_nbars_from_wavfiles,
    slices::Slices,
    SampleAttributes,
};
use crate::yaml_io::samplechains::YamlChainConfig;

use crate::octatrack::banks::{Bank, TrackMachineType};
use crate::octatrack::common::FromFileAtPathBuf;
use crate::octatrack::projects::slots::ProjectSampleSlots;
use crate::octatrack::projects::Project;

use crate::octatrack::options::{
    ProjectSampleSlotType, SampleAttributeLoopMode, SampleAttributeTimestrechMode,
    SampleAttributeTrigQuantizationMode,
};

use std::collections::HashMap;

use crate::audio::wavfile::{chain_wavfiles_64_batch, WavFile};
use crate::octatrack::samples::SampleFilePair;

/// Create Octatrack samplechain file-pairs from a loaded yaml config.

pub fn create_samplechains_from_yaml(
    yaml_conf: &YamlChainConfig,
) -> Result<Vec<SampleFilePair>, ()> {
    let mut outchains_files: Vec<SampleFilePair> = vec![];
    let mut outchains_samplechains: Vec<SampleAttributes> = vec![];

    for chain_config in &yaml_conf.chains {
        info!("Creating chain: {}", &chain_config.chain_name);

        debug!(
            "Reading wav files: n={:#?}",
            &chain_config.sample_file_paths.len()
        );
        let mut wavfiles: Vec<WavFile> = Vec::new();
        for wav_file_path in &chain_config.sample_file_paths {
            // TODO: Clone
            let wavfile = WavFile::from_file(wav_file_path.clone()).unwrap();
            wavfiles.push(wavfile);
        }

        debug!("Batching wav files ...");
        // first element is the chained wavfile output
        // second is the individual wav files that made the chain
        let wavfiles_batched: Vec<(WavFile, Vec<WavFile>)> =
            chain_wavfiles_64_batch(&wavfiles).unwrap();

        for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {
            info!("Processing batch: {} / {}", idx + 1, wavfiles_batched.len());

            debug!(
                "Have {:1?} WAV chains from {:2?} samples",
                &wavfiles_batched.len(),
                &wavfiles.len()
            );

            let slices = Slices::from_wavfiles(&vec_wavs, &0).unwrap();

            // let chain = SampleChain::from_yaml_conf(&chain_config).unwrap();
            // chains.insert(chain);

            // TODO -- can use single wavfile here?! would make the funtion more generally applicable.
            let bars = get_otsample_nbars_from_wavfiles(&vec_wavs, &125.0).unwrap();

            let trim_config = SampleTrimConfig {
                start: 0,
                end: single_wav.len,
                length: bars,
            };

            let loop_config = SampleLoopConfig {
                start: 0,
                length: bars,
                mode: chain_config.octatrack_settings.loop_mode,
            };

            let fstem = chain_config.chain_name.clone() + &format!("-{:?}", idx);

            let chain_data = SampleAttributes::new(
                &chain_config.octatrack_settings.bpm,
                &chain_config.octatrack_settings.timestretch_mode,
                &chain_config.octatrack_settings.quantization_mode,
                &chain_config.octatrack_settings.gain,
                &trim_config,
                &loop_config,
                &slices,
            )
            .unwrap();

            let base_outchain_path = yaml_conf.global_settings.out_dir_path.join(fstem);

            let mut ot_outpath = base_outchain_path.clone();
            let mut wav_sliced_outpath = base_outchain_path.clone();

            ot_outpath.set_extension("ot");
            wav_sliced_outpath.set_extension("wav");

            let _chain_res = chain_data.to_file(&ot_outpath);
            let _wav_slice_res = single_wav.to_file(&wav_sliced_outpath);

            info!(
                "Created chain files: audio={:?} ot={:?}",
                wav_sliced_outpath, ot_outpath
            );

            let sample =
                SampleFilePair::from_pathbufs(&wav_sliced_outpath, &Some(ot_outpath)).unwrap();

            outchains_samplechains.push(chain_data);
            outchains_files.push(sample);
        }
        info!("Created sample chain(s): {}", &chain_config.chain_name);
    }
    debug!("SAMPLE CHAINS GENERATED: {:#?}", outchains_samplechains);

    Ok(outchains_files)
}

/// Create Octatrack samplechain file-pairs from a loaded yaml config.

pub fn create_samplechain_from_pathbufs(
    wav_fps: Vec<PathBuf>,
    outdir_path: PathBuf,
    outchain_name: String,
) -> Result<(), ()> {
    let wavfiles: Vec<WavFile> = wav_fps
        .into_iter()
        .map(|fp: PathBuf| WavFile::from_file(fp).unwrap())
        .collect();

    let wavfiles_batched: Vec<(WavFile, Vec<WavFile>)> =
        chain_wavfiles_64_batch(&wavfiles).unwrap();

    for (idx, (single_wav, vec_wavs)) in wavfiles_batched.iter().enumerate() {
        let slices = Slices::from_wavfiles(&vec_wavs, &0).unwrap();

        // TODO -- can use single wavfile here?! would make the funtion more generally applicable.
        let bars = get_otsample_nbars_from_wavfiles(&vec_wavs, &125.0).unwrap();

        let trim_config = SampleTrimConfig {
            start: 0,
            end: single_wav.len,
            length: bars,
        };

        let loop_config = SampleLoopConfig {
            start: 0,
            length: bars,
            mode: SampleAttributeLoopMode::default(),
        };

        let chain_data = SampleAttributes::new(
            &120.0,
            &SampleAttributeTimestrechMode::default(),
            &SampleAttributeTrigQuantizationMode::default(),
            &0.0,
            &trim_config,
            &loop_config,
            &slices,
        )
        .unwrap();

        let base_outchain_path = outdir_path.join(&outchain_name);

        let mut wav_sliced_outpath = base_outchain_path;
        wav_sliced_outpath.set_extension("wav");
        let _wav_slice_res = single_wav.to_file(&wav_sliced_outpath);
        info!("Created chain audio file: {wav_sliced_outpath:#?}");

        let mut ot_outpath = wav_sliced_outpath;
        ot_outpath.set_extension("ot");
        let _chain_res = chain_data.to_file(&ot_outpath);
        info!("Created chain attributes file: {ot_outpath:#?}");
    }

    Ok(())
}

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

    // 3. read new project
    let dst_proj_path = dest_bank_file_path
        .parent()
        .unwrap()
        .to_path_buf()
        .join("project.work");

    let dst_dirpath = &dest_bank_file_path.parent().unwrap().to_path_buf();
    let dest_project = Project::from_pathbuf(dst_proj_path).unwrap();

    // 4. find space in new project sample slots

    let mut base_vec: Vec<u8> = vec![];
    for i in 1..=128 {
        base_vec.push(i)
    }
    let mut dest_free_static_sample_slots_ids = base_vec.clone();
    let mut dest_free_flex_sample_slots_ids: Vec<u8> = base_vec.clone();

    println!("DEST SLOT USAGE: {:#?}", dest_project.slots);

    for slot in dest_project.slots {
        match slot.sample_type {
            ProjectSampleSlotType::Static => {
                dest_free_static_sample_slots_ids.retain(|x| *x != slot.slot_id as u8);
            }
            ProjectSampleSlotType::Flex => {
                dest_free_flex_sample_slots_ids.retain(|x| *x != slot.slot_id as u8);
            }
            _ => {}
        }
    }

    // for i in 0..127_u8 {
    //     if !&dest_sample_slot_ids.contains(&i) {
    //         dest_free_sample_slots_ids.push(i);
    //     }
    // }

    // not enough sample slots -- clean up slot allocations please.
    if src_sample_slots.len()
        > (dest_free_static_sample_slots_ids.len() + dest_free_flex_sample_slots_ids.len())
    {
        panic!(
            "Not enough spare sample slots in destination project! srcSlotCount={:#?} destSlotCount={:#?}",
            src_sample_slots.len(),
            dest_free_static_sample_slots_ids.len() + dest_free_flex_sample_slots_ids.len(),
        );
    }

    // 5. read src bank data
    //  * machine assigned sample slots
    //  * trig sample lock assigned sample slots
    let src_bank_data = Bank::from_pathbuf(source_bank_file_path).unwrap();

    let mut active_static_slots: HashSet<u8> = HashSet::new();
    let mut active_flex_slots: HashSet<u8> = HashSet::new();

    for pattern in src_bank_data.patterns {
        for (_idx, track_trigs) in pattern.track_trigs.into_iter().enumerate() {
            for trig in track_trigs.trigs {
                if trig.sample_slot < 128 {
                    // When tracks have a Trig Sample Lock the sample lock does not
                    // care about flex / static. The sample locked trig will trigger
                    // whatever sample is in the sample slot indicated by the trig lock.
                    //
                    // so we have to assume that BOTH flex & static sample slots can be
                    // used by trig sample locks

                    active_static_slots.insert(trig.sample_slot);
                    active_flex_slots.insert(trig.sample_slot);
                }
            }
        }
    }

    if active_static_slots.len() > 0 {
        warn!(
            "Detected Trig sample locks. Assuming both Flex and Static slots can be used (Part switching while Pattern playing)."
        );
    }

    for part in src_bank_data.parts {
        for audio_track in part.audio_tracks {
            match audio_track.machine {
                TrackMachineType::StaticMachine { sample_slot } => {
                    active_static_slots.insert(sample_slot);
                }
                TrackMachineType::FlexMachine { sample_slot } => {
                    active_flex_slots.insert(sample_slot);
                }
                _ => {}
            }
        }
    }

    println!("SOURCE BANK DATA: {:#?}", src_bank_data.parts);

    println!(
        "SOURCE STATIC SAMPLE SLOTS IN USE: {:#?}",
        active_static_slots
    );
    println!("SOURCE FLEX SAMPLE SLOTS IN USE: {:#?}", active_flex_slots);
    println!(
        "DEST STATIC SAMPLE SLOTS FREE: {:#?}",
        dest_free_static_sample_slots_ids
    );
    println!(
        "DEST FLEX SAMPLE SLOTS FREE: {:#?}",
        dest_free_flex_sample_slots_ids
    );

    // exit(1);

    // 6. edit read bank data sample slot usage
    // this is just a creating a mapping from old to new.

    let mut source_to_dest_static_slot_map: HashMap<u8, u8> = HashMap::new();
    let mut source_to_dest_flex_slot_map: HashMap<u8, u8> = HashMap::new();

    // reverse so we can just use pop instead of needing to import VecDeque::pop_rev()
    dest_free_static_sample_slots_ids.reverse();
    dest_free_flex_sample_slots_ids.reverse();

    for active_static_slot in active_static_slots {
        let dest_slot_id = dest_free_static_sample_slots_ids.pop().unwrap();
        source_to_dest_static_slot_map.insert(active_static_slot, dest_slot_id);
    }

    for active_flex_slot in active_flex_slots {
        let dest_slot_id = dest_free_flex_sample_slots_ids.pop().unwrap();
        source_to_dest_flex_slot_map.insert(active_flex_slot, dest_slot_id);
    }

    // first, change the struct data so we've got everything correct.

    // TODO: Does this actually **mutate** the bank data
    // or does it just mutate the iterator output in for scope?

    for (k, v) in source_to_dest_static_slot_map.iter() {
        for pattern in src_bank_data.patterns {
            for track_trigs in pattern.track_trigs {
                for mut trig in track_trigs.trigs {
                    if trig.sample_slot == *k {
                        trig.sample_slot = v.clone();
                    }
                }
            }
        }
        for part in src_bank_data.parts {
            for mut audio_track in part.audio_tracks {
                match audio_track.machine {
                    TrackMachineType::StaticMachine { sample_slot } => {
                        audio_track.machine = TrackMachineType::StaticMachine {
                            sample_slot: source_to_dest_static_slot_map
                                .get(&sample_slot)
                                .unwrap()
                                .clone(),
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    for (k, v) in source_to_dest_flex_slot_map.iter() {
        for pattern in src_bank_data.patterns {
            for track_trigs in pattern.track_trigs {
                for mut trig in track_trigs.trigs {
                    if trig.sample_slot == *k {
                        trig.sample_slot = v.clone();
                    }
                }
            }
        }
        for part in src_bank_data.parts {
            for mut audio_track in part.audio_tracks {
                match audio_track.machine {
                    TrackMachineType::FlexMachine { sample_slot } => {
                        audio_track.machine = TrackMachineType::FlexMachine {
                            sample_slot: source_to_dest_flex_slot_map
                                .get(&sample_slot)
                                .unwrap()
                                .clone(),
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    println!("CHANGED SOURCE BANK DATA: {:#?}", src_bank_data.parts);

    // now change the actual bank bytes data

    // NOTE: this would be a lot easier with bank files Serde mapped out fully,
    //       but that's a massive undertaking I'm not super keen on today.
    //       So, we're going with messy, but works, in the first instance!

    // 7. edit read bank data sample slots
    //  *  machine assignment
    //  *  trig smaple lock assignment

    // let sample_slots = todo!();
    // let trig_sample_locks = todo!();
    // let sample_slots_active_machines = todo!();

    // todo!();

    // // Copy bank file
    // copy(src_dirpath, dst_dirpath);

    // // TODO: move them

    Ok(())
}