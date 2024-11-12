//! Parse Octatrack `project.*` data files.

pub mod common;
pub mod metadata;
pub mod options;
pub mod settings;
pub mod slots;
pub mod states;

use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf};

use crate::common::{
    FromFileAtPathBuf, ProjectFromString, ProjectToString, RBoxErr, ToFileAtPathBuf,
};

use crate::projects::{
    metadata::ProjectMetadata, options::ProjectSampleSlotType, settings::ProjectSettings,
    slots::ProjectSampleSlot, states::ProjectStates,
};

/// A parsed representation of an Octatrack Project file (`project.work` or `project.strd`).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Project {
    /// Metadata key-value pairs from a Project file.
    pub metadata: ProjectMetadata,

    /// Settings key-value pairs from a Project file.
    pub settings: ProjectSettings,

    /// States key-value pairs from a Project file.
    pub states: ProjectStates,

    /// Slots key-value pairs from a Project file.
    pub slots: Vec<ProjectSampleSlot>,
}

impl Project {
    pub fn update_sample_slot_id(
        &mut self,
        old_slot_id: &u8,
        new_slot_id: &u8,
        sample_type: Option<ProjectSampleSlotType>,
    ) -> () {
        use itertools::Itertools;
        let type_filt = sample_type.unwrap_or(ProjectSampleSlotType::Static);

        let sample_slot_find: Option<(usize, ProjectSampleSlot)> = self
            .slots
            .clone()
            .into_iter()
            .find_position(|x| x.slot_id == *old_slot_id as u16 && x.sample_type == type_filt);

        // no samples assigned to slots
        if !sample_slot_find.is_none() {
            println!("Found matchin slot id");
            let mut sample_slot = sample_slot_find.clone().unwrap().1;
            sample_slot.slot_id = *new_slot_id as u16;
            self.slots[sample_slot_find.unwrap().0] = sample_slot;
        }
    }
}

impl ProjectToString for Project {
    /// Turn a Project struct into a String configuration, ready for writing to binary data files

    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let states_header =
            "############################\r\n# Project States\r\n############################"
                .to_string();
        let settings_header =
            "############################\r\n# Project Settings\r\n############################"
                .to_string();
        let slots_header =
            "############################\r\n# Samples\r\n############################".to_string();
        let footer = "############################".to_string();

        let metadata_string: String = self.metadata.to_string()?;
        let states_string: String = self.states.to_string()?;
        let settings_string: String = self.settings.to_string()?;

        let slots_string_vec: Vec<String> =
            self.slots.iter().map(|x| x.to_string().unwrap()).collect();

        let mut v: Vec<String> = Vec::new();

        v.push(settings_header);
        v.push(metadata_string);
        v.push(settings_string);
        v.push(states_header);
        v.push(states_string);
        v.push(slots_header);
        for slot in slots_string_vec {
            v.push(slot);
        }
        v.push(footer);

        let mut project_string = v.join("\r\n\r\n");
        project_string.push_str("\r\n\r\n");
        Ok(project_string)
    }
}

impl FromFileAtPathBuf for Project {
    type T = Project;

    /// Read and parse an Octatrack project file (`project.work` or `project.strd`)
    fn from_pathbuf(path: PathBuf) -> RBoxErr<Self> {
        let s = std::fs::read_to_string(&path)?;

        let metadata = ProjectMetadata::from_string(&s)?;
        let states = ProjectStates::from_string(&s)?;
        let settings = ProjectSettings::from_string(&s)?;
        // todo? Get sample file pairs, pop the ones that are active, the rest are inactive.
        let slots = ProjectSampleSlot::from_string(&s)?;

        Ok(Self {
            metadata,
            settings,
            states,
            slots,
        })
    }
}

impl ToFileAtPathBuf for Project {
    fn to_pathbuf(&self, path: PathBuf) -> RBoxErr<()> {
        let data = self.to_string()?;
        let mut f = File::create(path)?;
        f.write_all(data.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod test_integration {
    use super::*;

    // can read a project file without errors
    #[test]
    fn test_read_default_project_work_file() {
        let infile = PathBuf::from("./data/tests/projects/blank.work");
        assert!(Project::from_pathbuf(infile).is_ok());
    }

    // test that the metadata section is correct
    #[test]
    fn test_read_default_project_work_file_metadata() {
        let infile = PathBuf::from("./data/tests/projects/blank.work");
        let p = Project::from_pathbuf(infile).unwrap();

        let correct = ProjectMetadata {
            filetype: "OCTATRACK DPS-1 PROJECT".to_string(),
            project_version: 19,
            os_version: "R0177     1.40B".to_string(),
        };

        assert_eq!(p.metadata, correct);
    }

    // test that the states section is correct
    #[test]
    fn test_read_default_project_work_file_states() {
        let infile = PathBuf::from("./data/tests/projects/blank.work");
        let p = Project::from_pathbuf(infile).unwrap();

        let correct = ProjectStates {
            bank: 0,
            pattern: 0,
            arrangement: 0,
            arrangement_mode: 0,
            part: 0,
            track: 0,
            track_othermode: 0,
            scene_a_mute: false,
            scene_b_mute: false,
            track_cue_mask: 0,
            track_mute_mask: 0,
            track_solo_mask: 0,
            midi_track_mute_mask: 0,
            midi_track_solo_mask: 0,
            midi_mode: 0,
        };

        assert_eq!(p.states, correct);
    }

    // test that the states section is correct
    #[test]
    fn test_read_default_project_work_file_settings() {
        use crate::projects::options::ProjectMidiChannels;
        use crate::projects::settings::control_menu::{
            AudioControlPage, ControlMenu, InputControlPage, MemoryControlPage,
            MetronomeControlPage, MidiChannelsMidiPage, MidiControlMidiPage,
            MidiSequencerControlPage, MidiSubMenu, MidiSyncMidiPage, SequencerControlPage,
        };
        use crate::projects::settings::{
            mixer::MixerMenu, tempo::TempoMenu, trig_mode_midi_tracks::MidiTrackTrigModes,
        };

        let infile = PathBuf::from("./data/tests/projects/blank.work");
        let p = Project::from_pathbuf(infile).unwrap();

        let correct = ProjectSettings {
            write_protected: false,
            control: ControlMenu {
                audio: AudioControlPage {
                    master_track: false,
                    cue_studio_mode: false,
                },
                input: InputControlPage {
                    gate_ab: 127,
                    gate_cd: 127,
                    input_delay_compensation: false,
                },
                sequencer: SequencerControlPage {
                    pattern_change_chain_behaviour: 0,
                    pattern_change_auto_silence_tracks: false,
                    pattern_change_auto_trig_lfos: false,
                },
                midi_sequencer: MidiSequencerControlPage {},
                memory: MemoryControlPage {
                    load_24bit_flex: false,
                    dynamic_recorders: false,
                    record_24bit: false,
                    reserved_recorder_count: 8,
                    reserved_recorder_length: 16,
                },
                metronome: MetronomeControlPage {
                    metronome_time_signature: 3,
                    metronome_time_signature_denominator: 2,
                    metronome_preroll: 0,
                    metronome_cue_volume: 32,
                    metronome_main_volume: 0,
                    metronome_pitch: 12,
                    metronome_tonal: true,
                    metronome_enabled: false,
                },
                midi: MidiSubMenu {
                    control: MidiControlMidiPage {
                        midi_audio_track_cc_in: true,
                        midi_audio_track_cc_out: 3,
                        midi_audio_track_note_in: 1,
                        midi_audio_track_note_out: 3,
                        midi_midi_track_cc_in: 1,
                    },
                    sync: MidiSyncMidiPage {
                        midi_clock_send: false,
                        midi_clock_receive: false,
                        midi_transport_send: false,
                        midi_transport_receive: false,
                        midi_progchange_send: false,
                        midi_progchange_send_channel: ProjectMidiChannels::Disabled,
                        midi_progchange_receive: false,
                        midi_progchange_receive_channel: ProjectMidiChannels::Disabled,
                    },
                    channels: MidiChannelsMidiPage {
                        midi_trig_ch1: 0,
                        midi_trig_ch2: 1,
                        midi_trig_ch3: 2,
                        midi_trig_ch4: 3,
                        midi_trig_ch5: 4,
                        midi_trig_ch6: 5,
                        midi_trig_ch7: 6,
                        midi_trig_ch8: 7,
                        midi_auto_channel: 10,
                    },
                },
            },
            midi_soft_thru: false,
            mixer: MixerMenu {
                gain_ab: 64,
                gain_cd: 64,
                dir_ab: 0,
                dir_cd: 0,
                phones_mix: 64,
                main_to_cue: 0,
                main_level: 64,
                cue_level: 64,
            },
            tempo: TempoMenu {
                tempo: 120,
                pattern_tempo_enabled: false,
            },
            midi_tracks_trig_mode: MidiTrackTrigModes {
                trig_mode_midi_track_1: 0,
                trig_mode_midi_track_2: 0,
                trig_mode_midi_track_3: 0,
                trig_mode_midi_track_4: 0,
                trig_mode_midi_track_5: 0,
                trig_mode_midi_track_6: 0,
                trig_mode_midi_track_7: 0,
                trig_mode_midi_track_8: 0,
            },
        };

        assert_eq!(p.settings, correct);
    }

    // test that the states section is correct
    #[test]
    fn test_read_default_project_work_file_sslots() {
        use crate::projects::options::ProjectSampleSlotType;
        use crate::samples::options::{
            SampleAttributeLoopMode, SampleAttributeTimestrechMode,
            SampleAttributeTrigQuantizationMode,
        };

        let infile = PathBuf::from("./data/tests/projects/blank.work");
        let p = Project::from_pathbuf(infile).unwrap();

        let correct: Vec<ProjectSampleSlot> = [
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::Flex,
                slot_id: 129,
                path: PathBuf::from("../AUDIO/flex.wav"),
                trim_bars: 1.73,
                timestrech_mode: SampleAttributeTimestrechMode::Normal,
                loop_mode: SampleAttributeLoopMode::Normal,
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::PatternLength,
                gain: 48,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 130,
                path: PathBuf::from(""),
                trim_bars: 0.0,
                timestrech_mode: SampleAttributeTimestrechMode::Normal,
                loop_mode: SampleAttributeLoopMode::Off,
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::PatternLength,
                gain: 72,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 131,
                path: PathBuf::from(""),
                trim_bars: 0.0,
                timestrech_mode: SampleAttributeTimestrechMode::Normal,
                loop_mode: SampleAttributeLoopMode::Off,
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::PatternLength,
                gain: 72,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 132,
                path: PathBuf::from(""),
                trim_bars: 0.0,
                timestrech_mode: SampleAttributeTimestrechMode::Normal,
                loop_mode: SampleAttributeLoopMode::Off,
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::PatternLength,
                gain: 72,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 133,
                path: PathBuf::from(""),
                trim_bars: 0.0,
                timestrech_mode: SampleAttributeTimestrechMode::Normal,
                loop_mode: SampleAttributeLoopMode::Off,
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::PatternLength,
                gain: 72,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 134,
                path: PathBuf::from(""),
                trim_bars: 0.0,
                timestrech_mode: SampleAttributeTimestrechMode::Normal,
                loop_mode: SampleAttributeLoopMode::Off,
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::PatternLength,
                gain: 72,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 135,
                path: PathBuf::from(""),
                trim_bars: 0.0,
                timestrech_mode: SampleAttributeTimestrechMode::Normal,
                loop_mode: SampleAttributeLoopMode::Off,
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::PatternLength,
                gain: 72,
                bpm: 120,
            },
            ProjectSampleSlot {
                sample_type: ProjectSampleSlotType::RecorderBuffer,
                slot_id: 136,
                path: PathBuf::from(""),
                trim_bars: 0.0,
                timestrech_mode: SampleAttributeTimestrechMode::Normal,
                loop_mode: SampleAttributeLoopMode::Off,
                trig_quantization_mode: SampleAttributeTrigQuantizationMode::PatternLength,
                gain: 72,
                bpm: 120,
            },
        ]
        .to_vec();

        assert_eq!(p.slots, correct);
    }

    // test that reading and writing a single project gives the same outputs
    #[test]
    fn test_read_write_default_project_work_file() {
        let infile = PathBuf::from("./data/tests/projects/blank.work");
        let outfile = PathBuf::from("/tmp/default_1.work");
        let p = Project::from_pathbuf(infile).unwrap();
        let _ = p.to_pathbuf(outfile.clone());

        let p_reread = Project::from_pathbuf(outfile).unwrap();

        assert_eq!(p, p_reread)
    }

    #[test]
    fn test_read_a_project_work_file() {
        let test_file_pathbuf =
            PathBuf::from("data/tests/index-cf/DEV-OTsm/FLEX-ONESTRTEND/project.work");
        assert!(Project::from_pathbuf(test_file_pathbuf).is_ok());
    }

    #[test]
    fn test_read_a_project_strd_file() {
        let test_file_pathbuf =
            PathBuf::from("data/tests/index-cf/DEV-OTsm/FLEX-ONESTRTEND/project.strd");
        assert!(Project::from_pathbuf(test_file_pathbuf).is_ok());
    }
}
