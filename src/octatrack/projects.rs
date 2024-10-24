//! Read Octatrack `project.work` data files to find out whether a sample is
//! currently used in project sample slots. 

/*
Possibly set this up to write TEMPLATE project files (no overwriting existing projects!).
e.g. fill static sample slots 001 through 032 with drum sample chains, 064-128 with 
field recordings etc.

Possibly set it up to handle MOVING sample paths during a sync. if an old sample 
on-machine path in a project and we are moving the sample location, change the path
in the project file.

TODO: what about project.strd ??! which one of work/strd is the "active" 
un-saved/un-synced data?

TODO: Rename `from_project_ascii` methods to one of
- `from_project_file`
- `from_file`
- `from_string` / `from_str`
*/

use std::ffi::OsStr;
use std::thread::AccessError;
use std::{
    collections::HashMap,
    path::PathBuf,
    str::FromStr,
    convert::TryFrom,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::octatrack::common::OptionEnumValueConvert;

use crate::octatrack::options::{
    ProjectSampleSlotType,
    SampleAttributeLoopMode,
    SampleAttributeTimestrechMode,
    SampleAttributeTrigQuantizationMode,
};

use crate::octatrack::samples::OctatrackSampleFilePair;

// TODO: Move to contants.rs
/// ASCII data section headings within an Octatrack `project.*` file

// TODO: Move to options
enum ProjectRawFileSection {
    Meta,
    States,
    Settings,
    Samples
}

impl OptionEnumValueConvert for ProjectRawFileSection {

    type T = ProjectRawFileSection;
    type V = String;

    fn from_value(v: Self::V) -> Result<Self::T, ()> {
        match v.to_ascii_uppercase().as_str() {
            "META" => Ok(Self::Meta),
            "STATES" => Ok(Self::States),
            "SETTINGS" => Ok(Self::Settings),
            "SAMPLES" => Ok(Self::Samples),
            _ => Err(())
        }
    }

    // TODO: This should never error, so doesn't need a Result here!
    fn value(&self) -> Result<Self::V, ()> {
        match self {
            Self::Meta => Ok("META".to_string()),
            Self::States => Ok("STATES".to_string()),
            Self::Settings => Ok("SETTINGS".to_string()),
            Self::Samples => Ok("SAMPLES".to_string()),
        }
    }
}

impl ProjectRawFileSection {
    pub fn start_string(self: &Self) -> String {
        format!("[{}]", self.value().unwrap())
    }
    pub fn end_string(self: &Self) -> String {
        format!("[/{}]", self.value().unwrap())
    }
}


// TODO: Validate section is a valid option?

/// Extract ASCII string project data for a specified section as a HashMap of k-v pairs. 

fn string_to_hashmap(data: &String, section: &ProjectRawFileSection) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {

    let start_idx: usize = data.find(&section.start_string()).unwrap();
    let start_idx_shifted: usize = start_idx + &section.start_string().len();
    let end_idx: usize = data.find(&section.end_string()).unwrap();

    let section: String = data[start_idx_shifted..end_idx].to_string();

    let mut hmap: HashMap<String, String> = HashMap::new();
    let mut trig_mode_midi_field_idx = 1;

    for split_s in section.split("\r\n") {

        // new line splits returns empty fields :/

        if split_s != "" {
            let key_pair_string = split_s.to_string();
            let mut key_pair_split: Vec<&str> = key_pair_string
                .split("=")
                .into_iter()
                .collect();

            // there are 8x TRIG_MODE_MIDI key value pairs in project settings data
            // but the keys do not have track number indicators. i assume they're 
            // stored in order of the midi track number, and each subsequent one we 
            // read is the next track.
            let key_renamed: String = format!("trig_mode_midi_track_{}", &trig_mode_midi_field_idx);
            if key_pair_split[0] == "TRIG_MODE_MIDI" {
                key_pair_split[0] = key_renamed.as_str();
                trig_mode_midi_field_idx += 1;
            }

            hmap.insert(
                key_pair_split[0].to_string().to_ascii_lowercase(), 
                key_pair_split[1].to_string(),
            );

        }
    }

    Ok(hmap)

}


/// Trait to enable extracting a section of raw Octatrack Project file ASCII data
trait ParseHashMapValueAs {

    fn parse_hashmap_value<T: std::str::FromStr>(hmap: &HashMap<String, String>, key: &str) -> Result<T, Box<dyn std::error::Error>>     
        where <T as std::str::FromStr>::Err: std::fmt::Debug {
        Ok(
            hmap
                .get(key)
                .unwrap()
                .parse::<T>()
                .unwrap()
        )
    }

    // special case as boolean values are actually stored as 0 / 1 in the project data
    fn parse_hashmap_value_bool(hmap: &HashMap<String, String>, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut res = false;
        if Self::parse_hashmap_value::<u8>(&hmap, &key)? == 1 {res = true};
        Ok(res)
}

}

pub trait FromProjectStringData {
    type T;
    fn from_string(data: &String) -> Result<Self::T, Box<dyn std::error::Error>>;
}


/// Project metadata read from a parsed Octatrack Project file

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectMetadata {

    /// Type of file (always a 'project').
    /// Example ASCII data: `TYPE=OCTATRACK DPS-1 PROJECT`
    filetype: String,

    /// Unknown.
    /// Example ASCII data: `VERSION=19`
    project_version: String,

    /// Version of the Octatrack OS (that the project was created with?).
    /// Example ASCII data: `OS_VERSION=R0177     1.40B`
    os_version: String,
}

impl ParseHashMapValueAs for ProjectMetadata {}

impl FromProjectStringData for ProjectMetadata {

    type T = Self;

    /// Extract `OctatrackProjectMetadata` fields from the project file's ASCII data 

    fn from_string(data: &String) -> Result<Self, Box<dyn std::error::Error>> {

        let hmap = string_to_hashmap(
            &data,
            &ProjectRawFileSection::Meta,
        )?;

        let filetype_default = "OCTATRACK DPS-1 PROJECT".to_string();
        let project_version_default = "19".to_string();
        let os_version_default = "R0177     1.40B".to_string();

        let filetype = hmap.get("type").unwrap_or(&filetype_default);
        let project_version = hmap.get("project_version").unwrap_or(&project_version_default);
        let os_version = hmap.get("os_version").unwrap_or(&os_version_default);
    
        Ok(
            ProjectMetadata {
                filetype: filetype.clone(),
                project_version: project_version.clone(),
                os_version: os_version.clone(),
            }
        )
    }

}

// TODO: This needs splitting up into subsection Structs, loaded by this main struct.

/// Project settings read from a parsed Octatrack Project file

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectSettings {

    /// Whether the project can be written to (probably is currently being read/written when `true`)
    write_protected: bool,

    /// BPM of the current project tempo setting.
    /// **NOTE 1**: This can be ignored by using the `pattern_tempo_enabled`.
    /// **NOTE 2**: Is multiplied by 24 on device.
    tempo: u32,
    
    /// Whether to use the current pattern's tempo or project tempo.
    /// - Pattern Tempo: `true`
    /// - Project Tempo: `false`
    pattern_tempo_enabled: bool,

    // ----------- PROJECT > CONTROL > AUDIO ----------- //

    /// `TRACK 8` setting in `PROJECT` -> `CONTROL` -> `AUDIO` UI menu. 
    /// Whether Track 8 is a master audio track or not:
    /// - **NORMAL**: `false`
    /// - **MASTER**: `true`
    master_track: bool,  // pretty sure this is the track 8 master setting?

    /// `CUE CFG` setting in `PROJECT` -> `CONTROL` -> `AUDIO` UI menu. 
    /// Behaviour for audio routing to CUE outputs. 
    /// - **NORMAL** -> **CUE+TRACK** button combo sends audio to CUE out.
    /// - **STUDIO** -> Individual track volume controls for CUE out (unable to **CUE+TRACK**).
    cue_studio_mode: bool,

    // ---------- MIDI SYNC MENU ----------- //

    /// `CLOCK SEND` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `SYNC` UI menu.
    /// Whether MIDI clock sending is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    midi_clock_send: bool,

    /// `CLOCK RECV` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `SYNC` UI menu.
    /// Whether MIDI clock receiving is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    midi_clock_receive: bool,

    /// `TRANS SEND` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `SYNC` UI menu.
    /// Whether MIDI transport sending is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    midi_transport_send: bool,

    /// `TRANS RECV` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `SYNC` UI menu.
    /// Whether MIDI transport receiving is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    midi_transport_receive: bool,

    /// `PROG CH SEND` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `SYNC` UI menu.
    /// Whether MIDI Program Change sending is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    midi_progchange_send: bool,

    /// `CHANNEL` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `SYNC` UI menu for `PROG CH SEND`.
    /// Channel to send MIDI Program Change messages on. (-1, or between 1 - 16).
    /// **NOTE**: should be set to `-1` when `midi_progchange_send` is disabled.
    /// See manual section 8.7.2 SYNC.
    midi_progchange_send_channel: i8,

    /// `PROG CH RECEIVE` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `SYNC` UI menu.
    /// Whether MIDI Program Change receiveing is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    midi_progchange_receive: bool,

    /// `CHANNEL` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `SYNC` UI menu for `PROG CH RECEIVE`.
    /// Channel to receive MIDI Program Change messages on (-1 or between 1 - 16).
    /// **NOTE**: should be set to `-1` when `midi_progchange_receive` is disabled.
    /// See manual section 8.7.2 SYNC.
    midi_progchange_receive_channel: i8,

    // ---------- MIDI CHANNELS MENU ----------- //

    /// `TRIG CH 1` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 1 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    midi_trig_ch1: u8,

    /// `TRIG CH 2` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 2 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    midi_trig_ch2: u8,

    /// `TRIG CH 3` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 3 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    midi_trig_ch3: u8,

    /// `TRIG CH 4` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 4 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    midi_trig_ch4: u8,

    /// `TRIG CH 5` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 5 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    midi_trig_ch5: u8,

    /// `TRIG CH 6` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 6 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    midi_trig_ch6: u8,

    /// `TRIG CH 7` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 7 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    midi_trig_ch7: u8,

    /// `TRIG CH 8` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 8 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    midi_trig_ch8: u8,

    /// `AUTO CH` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// Auto MIDI Channel (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    midi_auto_channel: u8,

    /// Unknown: Whether MIDI 'Thru' is enabled/disabled?
    midi_soft_thru: bool,

    // ---------- MIDI CONTROL MENU ----------- //

    /// `AUDIO CC IN` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CONTROL` UI menu.
    /// Whether audio tracks respond to MIDI CC IN messages.
    /// See manual section 8.7.1 CONTROL.
    midi_audio_track_cc_in: bool,

    /// `AUDIO CC OUT` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CONTROL` UI menu.
    /// Whether audio tracks send MIDI CC OUT messages. Three options:
    /// - `INT`: No messages sent, knobs only affect Octatrack settings.
    /// - `EXT`: Sends CC OUT messages but they don't alter any Octatrack settings.
    /// - `INT+EXT`: Simulataneously affects Octratack settings and sends CC OUT messages. 
    /// See manual section 8.7.1 CONTROL.
    midi_audio_track_cc_out: u8,

    /// `AUDIO NOTE IN` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CONTROL` UI menu.
    /// Whether to receive MIDI NOTE IN messages on Audio tracks and how the audio tracks 
    /// respond to those MIDI NOTE IN messages.
    /// - **OFF**: midi note has no effet.
    /// - **STANDARD**: standard note mapping (default).
    /// - **FOLLOW TM**: Track's current trig mode affects audio tracks (track/chromatic/slots).
    /// - **MAP/TRACK**: Uses MIDI MAP configuration on a per track basis (track/chromatic/slots
    /// disconnected from user trig mode of track).
    midi_audio_track_note_in: u8,

    /// `AUDIO NOTE OUT` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CONTROL` UI menu.
    /// Whether audio tracks send MIDI NOTE OUT messages. Three options:
    /// - `INT`: No messages sent, knobs only affect Octatrack settings.
    /// - `EXT`: Sends NOTE OUT messages but they don't alter any Octatrack settings.
    /// - `INT+EXT`: Simulataneously affects Octratack settings and sends NOTE OUT messages. 
    /// See manual section 8.7.1 CONTROL.
    midi_audio_track_note_out: u8,

    /// Unknown. MIDI channel to MIDI Track CC In messages n (1 - 16) ?
    midi_midi_track_cc_in: u8,

    // ---------- SEQUENCER MENU ----------- //

    /// `CHAIN AFTER` setting in `PROJECT` -> `CONTROL` -> `SEQUENCER` UI menu.
    /// When chained patterns start playing once the pattern is chosen.
    /// This is the global project level setting, but can be overidden for each pattern.
    /// Default setting is "PATTERN LENGTH". 
    /// See Manual section 8.6.3. SEQUENCER.
    pattern_change_chain_behaviour: u8,  // bool?

    /// `SILENCE TRACKS` setting in `PROJECT` -> `CONTROL` -> `SEQUENCER` UI menu.
    /// Silence tracks when switching to a new pattern.
    /// See Manual section 8.6.3. SEQUENCER.
    pattern_change_auto_silence_tracks: bool,

    /// `LFO AUTO CHANGE` setting in `PROJECT` -> `CONTROL` -> `SEQUENCER` UI menu.
    /// Whether to retrigger LFOs when swtiching to a new pattern
    /// See Manual section 8.6.3. SEQUENCER.
    pattern_change_auto_trig_lfos: bool,

    // ---------- MEMORY MENU ----------- //

    /// Whether samples can be loaded in 24-bit depth (16 bit depth samples are always oaded as 16 bit).
    /// Setting this to false loads all samples as 16 bit depth.
    /// See Manual section 8.6.5. MEMORY.
    load_24bit_flex: bool,

    /// Disabled forces all recorders to use track recorder memory (16 seconds per track). 
    /// When enabled, track recorders can use free Flex RAM memory.
    /// See Manual section 8.6.5. MEMORY.
    dynamic_recorders: bool,

    /// Whether to record in 24 bit depth (`true`) or 16 bit depth (`false`).
    /// See Manual section 8.6.5. MEMORY.
    record_24bit: bool,

    /// How many active track recorders are available in a project. Controls whether TR1 through to TR8 are enabled / disabled.
    /// See Manual section 8.6.5. MEMORY.
    reserved_recorder_count: u8,

    /// How many 'sequencer steps' should be reserved for track recorders in RAM.
    /// See Manual section 8.6.5. MEMORY.
    reserved_recorder_length: u32,

    /// See Manual section 8.6.2. INPUT.
    /// Adds a delay to incoming external audio signals. Controlled by the DIR setting on the MIXER page.
    input_delay_compensation: bool,

    // ---------- MIXER UI ----------- //

    /// dB level of noise gate for the AB external audio inputs.
    /// See Manual section 8.8 MIXER MENU
    gate_ab: u8, // 127 is default so i assume this is u8? midpoint?

    /// dB level of noise gate for the CD external audio inputs.
    /// See Manual section 8.8 MIXER MENU
    gate_cd: u8, // 127 is default so i assume this is u8? midpoint?

    /// Controls the incoming gain of external audio signal through AB inputs. -64 to +63 range.
    /// See Manual section 8.8 MIXER MENU
    gain_ab: u8, // 64 is default

    /// Controls the incoming gain of external audio signal through CD inputs. -64 to +63 range.
    /// See Manual section 8.8 MIXER MENU
    gain_cd: u8, // 64 is default

    /// Routes audio from AB inputs directly to mixer outputs. 0 to 127 range.
    /// See Manual section 8.8 MIXER MENU
    dir_ab: u8,

    /// Routes audio from CD inputs directly to mixer outputs. 0 to 127 range.
    /// See Manual section 8.8 MIXER MENU
    dir_cd: u8,

    /// How much to mix the master / cue outputs on the headphones output. 0 to 127 range with 64 the default (equal mix)
    /// See Manual section 8.8 MIXER MENU
    phones_mix: u8, // 64 is default, so 0 -> 127 with midpoint = 0 middle mix

    /// Unknown.
    /// See Manual section 8.8 MIXER MENU
    main_to_cue: u8,

    /// Final gain / output level of the main outputs. -64 to 63 range. 0 is default.
    /// See Manual section 8.8 MIXER MENU
    main_level: u8,

    /// Final gain / output level of the cue outputs. -64 to 63 range. 0 is default.
    /// See Manual section 8.8 MIXER MENU
    cue_level: u8, // no idea what params max mins are here

    // ---------- METRONOME MENU ----------- //

    /// `TIME SIG. NUMER` setting in `PROJECT` -> `CONTROL` -> `METRONOME` UI menu.
    /// Controls the numerator for time signature (the 3 in 3/4).
    /// See Manual section 8.6.6 METRONOME
    metronome_time_signature: u8, // i'm guessing 3 is actually 4/4? 0-indexed

    /// `TIME SIG. DENOM` setting in `PROJECT` -> `CONTROL` -> `METRONOME` UI menu.
    /// Controls the numerator for time signature (the 3 in 3/4).
    /// See Manual section 8.6.6 METRONOME
    metronome_time_denominator: u8, // i'm guessing 2 is actually 4/4? 0-indexed

    /// `PREROLL` setting in `PROJECT` -> `CONTROL` -> `METRONOME` UI menu.
    /// How many bars to prerolls with the metronome before playing a pattern.
    /// See Manual section 8.6.6 METRONOME
    metronome_preroll: u8,  // what is the maximum for this?

    /// How loud to play the metronome on CUE outputs. Default is 32.
    /// See Manual section 8.6.6 METRONOME
    metronome_cue_volume: u8,

    /// How loud to play the metronome on MAIN outputs. Default is 0.
    /// See Manual section 8.6.6 METRONOME
    metronome_main_volume: u8,

    /// Pitch of the metronome clicks. Default is 12.
    /// See Manual section 8.6.6 METRONOME
    metronome_pitch: u8,

    /// Whether the metronome click has tonal characteristics or not. Default is `true` (enabled).
    /// See Manual section 8.6.6 METRONOME
    metronome_tonal: bool,

    /// Whether the metronome is active. Default is `false`.
    /// See Manual section 8.6.6 METRONOME
    metronome_enabled: bool,

    // helpfully these are all just called TRIG_MODE_MIDI
    // but there's 8 of them so they must refer to the channels somehow
    // all default to 0.
    // refers to whether MDIDI track is on TRACK or CHRMOATIC trig mode.
    // should be u8 as *possible* for more trig modes to be added (although unlikely)

    /// Trig mode for MIDI Track 1
    trig_mode_midi_track_1: u8,

    /// Trig mode for MIDI Track 2
    trig_mode_midi_track_2: u8,

    /// Trig mode for MIDI Track 3
    trig_mode_midi_track_3: u8,

    /// Trig mode for MIDI Track 4
    trig_mode_midi_track_4: u8,

    /// Trig mode for MIDI Track 5
    trig_mode_midi_track_5: u8,

    /// Trig mode for MIDI Track 6
    trig_mode_midi_track_6: u8,

    /// Trig mode for MIDI Track 7
    trig_mode_midi_track_7: u8,

    /// Trig mode for MIDI Track 8
    trig_mode_midi_track_8: u8,
}

impl ParseHashMapValueAs for ProjectSettings {}

impl FromProjectStringData for ProjectSettings {

    type T = Self;

    /// Load project 'state' data from the raw project ASCII file.
    fn from_string(s: &String) -> Result<Self, Box<dyn std::error::Error>> {

        let hmap = string_to_hashmap(&s, &ProjectRawFileSection::Settings)?;

        Ok(
            Self {
                write_protected: Self::parse_hashmap_value_bool(&hmap, "writeprotected")?,
                tempo: Self::parse_hashmap_value::<u32>(&hmap, "tempox24")? / 24,
                pattern_tempo_enabled: Self::parse_hashmap_value_bool(&hmap, "pattern_tempo_enabled")?,
                master_track: Self::parse_hashmap_value_bool(&hmap, "master_track")?,
                cue_studio_mode: Self::parse_hashmap_value_bool(&hmap, "cue_studio_mode")?,
                midi_clock_send: Self::parse_hashmap_value_bool(&hmap, "midi_clock_send")?,
                midi_clock_receive: Self::parse_hashmap_value_bool(&hmap, "midi_clock_receive")?,
                midi_transport_send: Self::parse_hashmap_value_bool(&hmap, "midi_transport_send")?,
                midi_transport_receive: Self::parse_hashmap_value_bool(&hmap, "midi_transport_receive")?,
                midi_progchange_send: Self::parse_hashmap_value_bool(&hmap, "midi_program_change_send")?,
                midi_progchange_send_channel: Self::parse_hashmap_value::<i8>(&hmap, "midi_program_change_send_ch")?,
                midi_progchange_receive: Self::parse_hashmap_value_bool(&hmap, "midi_program_change_receive")?,
                midi_progchange_receive_channel: Self::parse_hashmap_value::<i8>(&hmap, "midi_program_change_receive_ch")?,
                midi_trig_ch1: Self::parse_hashmap_value::<u8>(&hmap, "midi_trig_ch1")?,
                midi_trig_ch2: Self::parse_hashmap_value::<u8>(&hmap, "midi_trig_ch2")?,
                midi_trig_ch3: Self::parse_hashmap_value::<u8>(&hmap, "midi_trig_ch3")?,
                midi_trig_ch4: Self::parse_hashmap_value::<u8>(&hmap, "midi_trig_ch4")?,
                midi_trig_ch5: Self::parse_hashmap_value::<u8>(&hmap, "midi_trig_ch5")?,
                midi_trig_ch6: Self::parse_hashmap_value::<u8>(&hmap, "midi_trig_ch6")?,
                midi_trig_ch7: Self::parse_hashmap_value::<u8>(&hmap, "midi_trig_ch7")?,
                midi_trig_ch8: Self::parse_hashmap_value::<u8>(&hmap, "midi_trig_ch8")?,
                midi_auto_channel: Self::parse_hashmap_value::<u8>(&hmap, "midi_auto_channel")?,
                midi_soft_thru: Self::parse_hashmap_value_bool(&hmap, "midi_soft_thru")?,
                midi_audio_track_cc_in: Self::parse_hashmap_value_bool(&hmap, "midi_audio_trk_cc_in")?,
                midi_audio_track_cc_out: Self::parse_hashmap_value::<u8>(&hmap, "midi_audio_trk_cc_out")?,
                midi_audio_track_note_in: Self::parse_hashmap_value::<u8>(&hmap, "midi_audio_trk_note_in")?,
                midi_audio_track_note_out: Self::parse_hashmap_value::<u8>(&hmap, "midi_audio_trk_note_out")?,
                midi_midi_track_cc_in: Self::parse_hashmap_value::<u8>(&hmap, "midi_midi_trk_cc_in")?,
                pattern_change_chain_behaviour: Self::parse_hashmap_value::<u8>(&hmap, "pattern_change_chain_behavior")?,
                pattern_change_auto_silence_tracks: Self::parse_hashmap_value_bool(&hmap, "pattern_change_auto_trig_lfos")?,
                pattern_change_auto_trig_lfos: Self::parse_hashmap_value_bool(&hmap, "pattern_change_auto_trig_lfos")?,
                load_24bit_flex: Self::parse_hashmap_value_bool(&hmap, "load_24bit_flex")?,
                dynamic_recorders: Self::parse_hashmap_value_bool(&hmap, "dynamic_recorders")?,
                record_24bit: Self::parse_hashmap_value_bool(&hmap, "record_24bit")?,
                reserved_recorder_count: Self::parse_hashmap_value::<u8>(&hmap, "reserved_recorder_count")?,
                reserved_recorder_length: Self::parse_hashmap_value::<u32>(&hmap, "reserved_recorder_length")?,
                input_delay_compensation: Self::parse_hashmap_value_bool(&hmap, "input_delay_compensation")?,
                gate_ab: Self::parse_hashmap_value::<u8>(&hmap, "gate_ab")?,
                gate_cd: Self::parse_hashmap_value::<u8>(&hmap, "gate_cd")?,
                gain_ab: Self::parse_hashmap_value::<u8>(&hmap, "gain_ab")?,
                gain_cd: Self::parse_hashmap_value::<u8>(&hmap, "gate_cd")?,
                dir_ab: Self::parse_hashmap_value::<u8>(&hmap, "dir_ab")?,
                dir_cd: Self::parse_hashmap_value::<u8>(&hmap, "gate_cd")?,
                phones_mix: Self::parse_hashmap_value::<u8>(&hmap, "phones_mix")?,
                main_to_cue: Self::parse_hashmap_value::<u8>(&hmap, "main_to_cue")?,
                main_level: Self::parse_hashmap_value::<u8>(&hmap, "main_level")?,
                cue_level: Self::parse_hashmap_value::<u8>(&hmap, "cue_level")?,
                metronome_time_signature: Self::parse_hashmap_value::<u8>(&hmap, "metronome_time_signature")?,
                metronome_time_denominator: Self::parse_hashmap_value::<u8>(&hmap, "metronome_time_signature_denominator")?,
                metronome_preroll: Self::parse_hashmap_value::<u8>(&hmap, "metronome_preroll")?,
                metronome_cue_volume: Self::parse_hashmap_value::<u8>(&hmap, "metronome_cue_volume")?,
                metronome_main_volume: Self::parse_hashmap_value::<u8>(&hmap, "metronome_main_volume")?,
                metronome_pitch: Self::parse_hashmap_value::<u8>(&hmap, "metronome_pitch")?,
                metronome_tonal: Self::parse_hashmap_value_bool(&hmap, "metronome_tonal")?,
                metronome_enabled: Self::parse_hashmap_value_bool(&hmap, "metronome_enabled")?,
                trig_mode_midi_track_1: Self::parse_hashmap_value::<u8>(&hmap, "trig_mode_midi_track_1")?,
                trig_mode_midi_track_2: Self::parse_hashmap_value::<u8>(&hmap, "trig_mode_midi_track_2")?,
                trig_mode_midi_track_3: Self::parse_hashmap_value::<u8>(&hmap, "trig_mode_midi_track_3")?,
                trig_mode_midi_track_4: Self::parse_hashmap_value::<u8>(&hmap, "trig_mode_midi_track_4")?,
                trig_mode_midi_track_5: Self::parse_hashmap_value::<u8>(&hmap, "trig_mode_midi_track_5")?,
                trig_mode_midi_track_6: Self::parse_hashmap_value::<u8>(&hmap, "trig_mode_midi_track_6")?,
                trig_mode_midi_track_7: Self::parse_hashmap_value::<u8>(&hmap, "trig_mode_midi_track_7")?,
                trig_mode_midi_track_8: Self::parse_hashmap_value::<u8>(&hmap, "trig_mode_midi_track_8")?,
            }
        )
    }
}


// [STATES]\r\nBANK=0\r\nPATTERN=0\r\nARRANGEMENT=0\r\nARRANGEMENT_MODE=0\r\nPART=0\r\nTRACK=0\r\nTRACK_OTHERMODE=0\r\nSCENE_A_MUTE=0\r\nSCENE_B_MUTE=0\r\nTRACK_CUE_MASK=0\r\nTRACK_MUTE_MASK=0\r\nTRACK_SOLO_MASK=0\r\nMIDI_TRACK_MUTE_MASK=0\r\nMIDI_TRACK_SOLO_MASK=0\r\nMIDI_MODE=0\r\n[/STATES]

/// Project state from a parsed Octatrack `project.*` file.
/// This is the current 'UX focus' state, i.e. what parts, patterns, tracks are currently selected etc.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectStates {

    /// Current active bank
    bank: u8,

    /// Current active pattern within a bank
    pattern: u8,
    arrangement: u8,
    arrangement_mode: u8, // dunno if this is a toggle or an enum

    /// Current active part for the active pattern within a bank
    part: u8,

    /// Current active track
    track: u8,

    // Unknown
    track_othermode: u8, // WTFF is this?

    /// Whether Scene A is disabled/enabled
    // TODO: Should 0 be enabled?
    scene_a_mute: bool,

    /// Whether Scene A is disabled/enabled
    // TODO: Should 0 be enabled?
    scene_b_mute: bool,

    // Unknown
    track_cue_mask: u8,

    // Unknown
    track_mute_mask: u8,

    // Unknown
    track_solo_mask: u8,

    // Unknown
    midi_track_mute_mask: u8,

    // Unknown
    midi_track_solo_mask: u8,

    // Unknown
    midi_mode: u8,
}

impl ParseHashMapValueAs for ProjectStates {}

impl FromProjectStringData for ProjectStates {

    type T = Self;

    /// Load project 'state' data from the raw project ASCII file.
    fn from_string(s: &String) -> Result<Self, Box<dyn std::error::Error>> {

        let hmap = string_to_hashmap(&s, &ProjectRawFileSection::States)?;

        Ok(
            Self {
                bank: Self::parse_hashmap_value::<u8>(&hmap, "bank")?,
                pattern: Self::parse_hashmap_value::<u8>(&hmap, "pattern")?,
                arrangement: Self::parse_hashmap_value::<u8>(&hmap, "arrangement")?,
                arrangement_mode: Self::parse_hashmap_value::<u8>(&hmap, "arrangement_mode")?,
                part: Self::parse_hashmap_value::<u8>(&hmap, "part")?,
                track: Self::parse_hashmap_value::<u8>(&hmap, "track")?,
                track_othermode: Self::parse_hashmap_value::<u8>(&hmap, "track_othermode")?,
                scene_a_mute: Self::parse_hashmap_value_bool(&hmap, "scene_a_mute")?,
                scene_b_mute: Self::parse_hashmap_value_bool(&hmap, "scene_b_mute")?,
                track_cue_mask: Self::parse_hashmap_value::<u8>(&hmap, "track_cue_mask")?,
                track_mute_mask: Self::parse_hashmap_value::<u8>(&hmap, "track_mute_mask")?,
                track_solo_mask: Self::parse_hashmap_value::<u8>(&hmap, "track_solo_mask")?,
                midi_track_mute_mask: Self::parse_hashmap_value::<u8>(&hmap, "midi_track_mute_mask")?,
                midi_track_solo_mask: Self::parse_hashmap_value::<u8>(&hmap, "midi_track_solo_mask")?,
                midi_mode: Self::parse_hashmap_value::<u8>(&hmap, "midi_mode")?,
            }
        )
    }
}

// [SAMPLE]\r\nTYPE=FLEX\r\nSLOT=001\r\nPATH=../AUDIO/flex.wav\r\nTRIM_BARSx100=173\r\nTSMODE=2\r\nLOOPMODE=1\r\nGAIN=48\r\nTRIGQUANTIZATION=-1\r\n[/SAMPLE]

/// Project sample read from a parsed Octatrack Project file.
/// This only loads data from the project file.
/// Samples not added to a project sample lsit for sstatic/flex machines will not be loaded.
/// **NOTE**: any fields matching those in an Octatrack sample attributes file 
/// may not have been writtten to an attributes file yet. 
/// (these are project files loaded into memory when switching to the project)/

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectSampleActive {

    // TODO: Should recording buffers be treated as a separate slot type?
    /// Type of sample: STATIC or FLEX
    sample_type: ProjectSampleSlotType,

    /// String ID Number of the slot the sample is assigned to e.g. 001, 002, 003...
    /// Maximum of 128 entries for STATIC sample slots, but can be up to 136 for flex 
    /// slots as there are 8 recorders + 128 flex slots.
    slot_id: u16,

    /// Relative path to the file on the card from the project directory.
    path: PathBuf,

    /// The sample's file pair (audio file and optional attributes file).
    file_pair: Option<OctatrackSampleFilePair>,

    // TODO: This is optional -- not used for recording buffer 'flex' tracks
    /// Current bar trim (float). This is multiplied by 100 on the machine.
    trim_bars: f32,

    /// Current `SampleTimestrechModes` setting for the specific slot. Example: `TSMODE=2`
    timestrech_mode: SampleAttributeTimestrechMode,

    /// Current `SampleLoopModes` setting for the specific slot.
    loop_mode: SampleAttributeLoopMode,

    // TODO: This is optional -- not used for recording buffer 'flex' tracks
    /// Current `SampleTrigQuantizationModes` setting for this specific slot.
    trig_quantization_mode: SampleAttributeTrigQuantizationMode,

    // TODO: Need to scale this to -24.0 dB <= x <= 24.0 dB
    /// Sample gain. 48 is default as per sample attributes file. maximum 96, minimum 0.
    gain: u8,

    // TODO: Need to scale this down by 24.
    /// BPM of the sample in this slot.
    bpm: u16,
}

// cannot use FromProjectStringData because it expects a lone Self result, rather than a Vec. 
impl ProjectSampleActive {

    fn from_hashmap(hmap: &HashMap<String, String>) -> Result<Self, Box<dyn std::error::Error>> {

        let sample_type = ProjectSampleSlotType
            ::from_value(hmap.get("type").unwrap().clone())
            .unwrap()
        ;

        let slot_id = hmap
            .get("slot")
            .unwrap()
            .clone()
            .parse::<u16>()
            .unwrap()
        ;

        let path = PathBuf
            ::from_str(
                hmap
                    .get("path")
                    .unwrap()
                )
            .unwrap()
        ;

        // TODO: Will never find the respective OT file as
        // the ^ path is alwys relative to project dir on CF card

        let mut file_pair = None;
        if path.file_name() != PathBuf::from("").file_name() {
            file_pair = Some(
                OctatrackSampleFilePair
                    ::from_audio_pathbuf(&path)
                    .unwrap()
                )
            ;
        }

        let trim_bars = hmap
            .get("trim_barsx100")
            .unwrap_or(&"0.0".to_string())
            .clone()
            .parse::<f32>()
            .unwrap()
            / 100.0
        ;

        let loop_mode = SampleAttributeLoopMode
            ::from_value(
                hmap
                    .get("loopmode")
                    .unwrap()
                    .clone()
                    .parse::<u32>()
                    .unwrap()
                )
            .unwrap()
        ;

        let timestrech_mode = SampleAttributeTimestrechMode
            ::from_value(
                hmap
                    .get("tsmode")
                    .unwrap()
                    .clone()
                    .parse::<u32>()
                    .unwrap()
                )
            .unwrap()
        ;

        let tq_i16: i16 = hmap
            .get("trigquantization")
            .unwrap()
            .clone()
            .parse::<i16>()
            .unwrap()
        ;

        let tq_u32: u32 = u32
            ::try_from(tq_i16)
            .unwrap_or(255_u32)
        ;

        let trig_quantization_mode = SampleAttributeTrigQuantizationMode
            ::from_value(tq_u32)
            .unwrap()
        ;

        let gain = hmap
            .get("gain")
            .unwrap()
            .clone()
            .parse::<u8>()
            .unwrap()
        ;

        let bpm = hmap
            .get("bpm")
            .unwrap_or(&"2880".to_string())
            .clone()
            .parse::<u16>()
            .unwrap_or(2880) / 24_u16
        ;

        let sample_struct = Self {
            sample_type,
            slot_id,
            path,
            file_pair,
            trim_bars,
            timestrech_mode,
            loop_mode,
            trig_quantization_mode,
            gain,
            // bpm: hmap.get("bpm").unwrap().clone().parse::<u16>().unwrap(),
            bpm
        };

        Ok(sample_struct)

    }

    /// Load project 'samples' data from the raw project ASCII file.
    fn from_string(data: &String) -> Result<Vec<Self>, Box<dyn std::error::Error>> {

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

        let mut sample_structs: Vec<ProjectSampleActive> = Vec::new();
        for sample in samples {

            let mut hmap: HashMap<String, String> = HashMap::new();
            for key_value_pair in sample {
                hmap.insert(
                    key_value_pair[0].to_string().to_lowercase(),
                    key_value_pair[1].to_string(),
                );
            }

            let sample_struct = Self::from_hashmap(&hmap)?;

            sample_structs.push(sample_struct);
        }

        Ok(sample_structs)

    }
}


/// All samples related to the project

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectSamples {

    /// samples loaded into a project sample slot
    active: Vec<ProjectSampleActive>,

    /// smples in a project directory, but not loaded into a sample slot.
    inactive: Vec<OctatrackSampleFilePair>,
}

impl ProjectSamples {
    pub fn from_string() {

    }
}

/// A parsed representation of an Octatrack Project file (`project.work` or `project.strd`).

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Project {
    // has to be a vec because the length of the 
    // file depends on how many samples are added?
    pub meta: ProjectMetadata,
    pub settings: ProjectSettings,
    pub states: ProjectStates,
    pub samples: ProjectSamples,

    /// Name of this Project (directory basename)
    pub name: String,

    /// Explicit path to this Audio Pool
    pub path: PathBuf,
}

// TODO: Move to some utils file
// TODO: Error type
fn get_pathbuf_fname_as_string(path: &PathBuf) -> Result<String, ()> {

    let name = path
        .clone()
        .file_name()
        .unwrap_or(&OsStr::new("err"))
        .to_str()
        .unwrap_or("err")
        .to_string()
    ;

    if name == "err" {return Err(())};
    Ok(name)

}


impl Project {

    pub fn to_file(&self, file_path: &PathBuf) -> ! {
        todo!()
    }

    /// Read and parse an Octatrack project file (`project.work` or `project.strd`)
    
    pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {

        let s = std::fs::read_to_string(&path)?;

        let meta = ProjectMetadata::from_string(&s)?;
        println!("META: {:#?}", meta);

        let states = ProjectStates::from_string(&s)?;
        println!("STATES: {:#?}", states);

        let settings = ProjectSettings::from_string(&s)?;
        println!("SETTINGS: {:#?}", settings);

        // TODO: Get sample file pairs, pop the ones that are active, the rest are inactive.

        let active_samples = ProjectSampleActive::from_string(&s)?;
        println!("SAMPLES: {:#?}", &active_samples);

        // TODO
        let inactive_samples: Vec<OctatrackSampleFilePair> = vec![];

        let samples = ProjectSamples {
            active: active_samples,
            inactive: inactive_samples,
        };

        let name = get_pathbuf_fname_as_string(&path).unwrap();

        Ok(
            Self {
                meta,
                settings,
                states,
                samples,
                name,
                path,
            }
        )
    }
}



#[cfg(test)]
mod test_integration {
    use super::*;

    #[test]
    fn test_read_a_project() {
        assert!(
            ! Project
                ::from_file(
                    PathBuf
                        ::from_str("data/tests/index-cf/DEV-OTsm/FLEX-ONESTRTEND/project.work")
                        .unwrap()
                )
                .is_ok()
        );
    }
}
