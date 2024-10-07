/*
Read Octatrack `project.work` data files to find out whether a sample is
currently used in project sample slots. 

If so -- care need be taken when moving files around.

Possibly set this up to write TEMPLATE project files (no overwriting existing projects!).
e.g. fill static sample slots 001 through 032 with drum sample chains, 064-128 with 
field recordings etc.

Possibly set it up to handle MOVING sample paths during a sync. if an old sample 
on-machine path in a project and we are moving the sample location, change the path
in the project file.

TODO: what about project.strd ??! which one of work/strd is the "active" 
un-saved/un-synced data?
*/
use std::{
    fs::File,
    collections::HashMap,
    io::prelude::*,
    path::PathBuf,
};
use itertools::Itertools;
use bincode::ErrorKind;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OctatrackProject {
    // has to be a vec because the length of the 
    // file depends on how many samples are added?
    pub data: Vec<String>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OctatrackProjectMetadata {
    filetype: String,
    // this is just VERSION...
    // i've called it project verison until i can work out what "version" means
    project_version: String,
    os_version: String,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OctatrackProjectSettings {
    write_protected: bool,
    tempo: u32, // this is multiplied by 24 in file!
    pattern_tempo_enabled: bool,
    midi_clock_send: bool,
    midi_clock_receive: bool,
    midi_transport_send: bool,
    midi_transport_receive: bool,
    midi_progchange_send: bool,
    midi_progchange_send_channel: i8, // -1 is "not enabled"
    midi_progchange_receive: bool,
    midi_progchange_receive_channel: i8, // -1 is "not enabled"
    midi_trig_channel_one: u8,
    midi_trig_channel_two: u8,
    midi_trig_channel_three: u8,
    midi_trig_channel_four: u8,
    midi_trig_channel_five: u8,
    midi_trig_channel_six: u8,
    midi_trig_channel_seven: u8,
    midi_trig_channel_eight: u8,
    midi_auto_channel: u8,
    midi_soft_thru: bool,
    midi_audio_track_cc_in: u8,
    midi_audio_track_cc_out: u8,
    midi_audio_track_note_in: u8,
    midi_audio_track_note_out: u8,
    midi_midi_track_cc_in: u8,
    pattern_change_chain_behaviour: u8,  // bool?
    pattern_change_auto_slice_tracks: u8,  // bool?
    pattern_change_auto_trigger_lfos: u8,  // bool?
    load_24bit_flex: u8,  // bool?
    dynamic_recorders: bool,
    record_24bit: u8,  // bool?
    reserved_recorder_count: u8,
    reserved_recorder_length: u32,
    input_delay_compensation: u8, //?
    gate_ab: u8, // 127 is default so i assume this is u8? midpoint?
    gate_cd: u8, // 127 is default so i assume this is u8? midpoint?
    gain_ab: u8, // 64 is default so i assume this is u8? why would this be 64 default?
    gain_cd: u8, // 64 is default so i assume this is u8? why would this be 64 default?
    dir_ab: u8, // no idea what params max mins are here
    dir_cd: u8, // no idea what params max mins are here
    phones_mix: u8, // no idea what params max mins are here
    main_to_cue: u8, // no idea what params max mins are here
    master_track: bool,  // pretty sure this is the track 8 master setting?
    cue_studio_mode: bool,  // whether cue channels are on output or not pretty sure
    main_level: u8, // no idea what params max mins are here
    cue_level: u8, // no idea what params max mins are here
    metronome_time_signature: u8, // i'm guessing 3 is actually 4/4? 0-indexed
    metronome_time_denominator: u8, // i'm guessing 2 is actually 4/4? 0-indexed
    metronome_preroll: bool,
    metronome_cue_volume: u8,  // default is 32
    metronome_main_volume: u8,  // default is 0
    metronome_pitch: u8,  // default is 12
    metronome_tonal: bool,  // default is 1, i think this is a toggle?
    metronome_enabled: bool,  // default is 0, this must be a toggle?
    // helpfully these are all just called TRIG_MODE_MIDI
    // but there's 8 o them so they must refer to the channels somehow
    // all default to 0, think it's a toggle, but could be an enum or something
    trig_mode_midi_ch1: bool, // I THINK THIS IS A TOGGLE?
    trig_mode_midi_ch2: bool, // I THINK THIS IS A TOGGLE?
    trig_mode_midi_ch3: bool, // I THINK THIS IS A TOGGLE?
    trig_mode_midi_ch4: bool, // I THINK THIS IS A TOGGLE?
    trig_mode_midi_ch5: bool, // I THINK THIS IS A TOGGLE?
    trig_mode_midi_ch6: bool, // I THINK THIS IS A TOGGLE?
    trig_mode_midi_ch7: bool, // I THINK THIS IS A TOGGLE?
    trig_mode_midi_ch8: bool, // I THINK THIS IS A TOGGLE?
}


// [STATES]\r\nBANK=0\r\nPATTERN=0\r\nARRANGEMENT=0\r\nARRANGEMENT_MODE=0\r\nPART=0\r\nTRACK=0\r\nTRACK_OTHERMODE=0\r\nSCENE_A_MUTE=0\r\nSCENE_B_MUTE=0\r\nTRACK_CUE_MASK=0\r\nTRACK_MUTE_MASK=0\r\nTRACK_SOLO_MASK=0\r\nMIDI_TRACK_MUTE_MASK=0\r\nMIDI_TRACK_SOLO_MASK=0\r\nMIDI_MODE=0\r\n[/STATES]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OctatrackProjectStates {
    bank: u8,
    pattern: u8,
    arrangement: u8,
    arrangement_mode: u8, // dunno if this is a toggle or an enum
    part: u8, // dunno if this is a toggle or an enum
    track: u8, // dunno if this is a toggle or an enum
    track_othermode: u8, // WTFF is this?
    scene_a_mute: bool, // pretty sure this is a toggle for whether the selected A scene is on/off
    scene_b_mute: bool, // pretty sure this is a toggle for whether the selected B scene is on/off
    track_cue_mask: u8, // no idea what this is
    track_mute_mask: u8, // no idea what this is
    track_solo_mask: u8, // no idea what this is
    midi_track_mute_mask: u8, // no idea what this is
    midi_track_solo_mask: u8, // no idea what this is
    midi_mode: u8, // no idea what this is
}


// [SAMPLE]\r\nTYPE=FLEX\r\nSLOT=001\r\nPATH=../AUDIO/flex.wav\r\nTRIM_BARSx100=173\r\nTSMODE=2\r\nLOOPMODE=1\r\nGAIN=48\r\nTRIGQUANTIZATION=-1\r\n[/SAMPLE]

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OctatrackProjectSample {
    sample_type: String,  // needs to be an enum --> STATIC vs. FLEX
    slot_id: String, // string in data (001) but can be up to 136 for flex slots (8 recorder +  128 flex slots)
    path: String, // RELATIVE Path to the file on the card, relative from the project dir. 
    // trim_bars: String, // current bar trim (float)... this is x100 on the machine
    timestrech_mode: u8, // TSMODE is the key name! default is 2... convert to SampleTimestretchModes enum?
    loop_mode: u8, //  convert to SampleLoopModes enum?
    trig_quantization_mode: i8, //  convert to SampleTrigQuantizationModes enum?
    gain: u8,  // 48 is default as per ot_io OT file read/write
    // bpm: u16,  // NOTE: BPM is not actually set in for sample slots. it is set for recording buffers.
}


impl OctatrackProject {

    fn get_samples_from_string_data(data: &String) -> Result<Vec<OctatrackProjectSample>, ()> {

        let n_samples = data
            .split("[SAMPLE]")
            .into_iter()
            .collect_vec()
            .len() - 1
            ;

        let mut data_window: Vec<&str> = data
            .split("[/SAMPLE]")
            .into_iter()
            .collect()
            ;

        data_window = data_window[1..(data_window.len() - 1)].to_vec();

        let samples: Vec<Vec<Vec<&str>>> = data_window
            .into_iter()
            .map(
                |sample: &str| sample
                    .strip_prefix("\r\n\r\n[SAMPLE]\r\n")
                    .unwrap()
                    .strip_suffix("\r\n")
                    .unwrap()
                    .split("\r\n")
                    .into_iter()
                    .map(|x: &str| x.split("=").into_iter().collect_vec())
                    .filter(|x: &Vec<&str>| x.len() == 2)
                    .collect_vec()

            )
            .collect()
        ;

        let mut sample_structs: Vec<OctatrackProjectSample> = Vec::new();
        for sample in samples {
            let mut hmap: HashMap<String, String> = HashMap::new();
            for key_value_pair in sample {
                hmap.insert(
                    key_value_pair[0].to_string().to_lowercase(),
                    key_value_pair[1].to_string(),
                );
            }

            let sample_struct = OctatrackProjectSample {
                sample_type: hmap.get("type").unwrap().clone(),
                slot_id: hmap.get("slot").unwrap().clone(),
                path: hmap.get("path").unwrap().clone(),
                // trim_bars: hmap.get("trim_barsx100").unwrap().clone(),
                timestrech_mode: hmap.get("tsmode").unwrap().clone().parse::<u8>().unwrap(),
                loop_mode: hmap.get("loopmode").unwrap().clone().parse::<u8>().unwrap(),
                trig_quantization_mode: hmap.get("trigquantization").unwrap().clone().parse::<i8>().unwrap(),
                gain: hmap.get("gain").unwrap().clone().parse::<u8>().unwrap(),
                // bpm: 2880,  // setting as default value until i can work out how to deal with recording buffer vs. sample slot behaviour
            };

            sample_structs.push(sample_struct);
        }

        Ok(sample_structs)

    }

    fn get_section_from_string_data(data: &String, section: &str) -> Result<String, ()> {

        let start_search_pattern = format!("[{}]", section);
        let end_search_pattern = format!("[/{}]", section);

        let start_idx: usize = data.find(&start_search_pattern).unwrap();
        let start_idx_shifted: usize = start_idx + section.len() + 2;
        let end_idx: usize = data.find(&end_search_pattern).unwrap();

        let section: String = data[start_idx_shifted..end_idx].to_string();

        Ok(section)

    }

    // TODO: This currently breaks reading the trig_mode_midi_* fields 
    // because they're all labeled with the same key (TRIG_MODE_MIDI)
    // :/
    fn split_fields_to_hashmap(section: &String) -> Result<HashMap<String, String>, ()> {

        let mut hmap: HashMap<String, String> = HashMap::new();
        for split_s in section.split("\r\n") {
            // new line splits returns empty fields :/
            if split_s != "" {
                let key_pair_string = split_s.to_string();
                let key_pair_split: Vec<&str> = key_pair_string.split("=").into_iter().collect();

                hmap.insert(
                    key_pair_split[0].to_string().to_ascii_lowercase(), 
                    key_pair_split[1].to_string(),
                );

            }
        }

        Ok(hmap)
    }

    pub fn to_file(&self, file_path: &PathBuf) -> ! {
        todo!()
    }

    /// Read some `.ot` file into a new struct
    pub fn from_file(path: &str) -> Result<String, Box<ErrorKind>> {

        let mut infile: File = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;
        // println!("BYTES: {:?}", bytes);

        let s: String = String::from_utf8(bytes).unwrap();

        let meta = OctatrackProject
            ::get_section_from_string_data(&s, "META")
            .unwrap()
            ;
        let meta_split = OctatrackProject
            ::split_fields_to_hashmap(&meta)
            .unwrap()
            ;

        let states = OctatrackProject
            ::get_section_from_string_data(&s, "STATES")
            .unwrap()
            ;
        let states_split = OctatrackProject
            ::split_fields_to_hashmap(&states)
            .unwrap()
            ;

        let settings = OctatrackProject
            ::get_section_from_string_data(&s, "SETTINGS")
            .unwrap()
            ;
        let settings_split = OctatrackProject
            ::split_fields_to_hashmap(&settings)
            .unwrap()
            ;

        let samples_split = OctatrackProject
            ::get_samples_from_string_data(&s)
            .unwrap()
            ;

        println!("META: {:?}", meta_split);
        println!("STATES: {:?}", states_split);
        println!("SETTINGS: {:?}", settings_split);

        println!("SAMPLES: {:?}", &samples_split);

        Ok(s)
    }

}



#[cfg(test)]
mod test_make_work_thing {
    use super::*;

    #[test]
    fn test_make_thing_decode() {
        let read_project = OctatrackProject
            ::from_file("/media/dijksterhuis/OT-LIVE1/DEV-OTsm/FLEX-ONESTRTEND/project.work")
            .unwrap()
            ;

        // println!("{:?}", read_project);

        assert!(false);

    }
}