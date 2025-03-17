//! Project settings from a parsed Octatrack Project file.
//! e.g. whether Track 8 is a master track or not.

pub mod control_menu;
pub mod mixer;
pub mod tempo;
pub mod trig_mode_midi_tracks;

use control_menu::{
    AudioControlPage, ControlMenu, InputControlPage, MemoryControlPage, MetronomeControlPage,
    MidiChannelsMidiPage, MidiControlMidiPage, MidiSequencerControlPage, MidiSubMenu,
    MidiSyncMidiPage, SequencerControlPage,
};

use mixer::MixerMenu;
use tempo::TempoMenu;
use trig_mode_midi_tracks::MidiTrackTrigModes;

use serde::{Deserialize, Serialize};

use crate::{OptionEnumValueConvert, RBoxErr};

use crate::projects::{
    options::ProjectMidiChannels, parse_hashmap_string_value_bool, string_to_hashmap, FromHashMap,
    ProjectFromString, ProjectRawFileSection, ProjectToString,
};

/*
Example data:
[SETTINGS]\r\nWRITEPROTECTED=0\r\nTEMPOx24=2880\r\nPATTERN_TEMPO_ENABLED=0\r\nMIDI_CLOCK_SEND=0\r\nMIDI_CLOCK_RECEIVE=0\r\nMIDI_TRANSPORT_SEND=0\r\nMIDI_TRANSPORT_RECEIVE=0\r\nMIDI_PROGRAM_CHANGE_SEND=0\r\nMIDI_PROGRAM_CHANGE_SEND_CH=-1\r\nMIDI_PROGRAM_CHANGE_RECEIVE=0\r\nMIDI_PROGRAM_CHANGE_RECEIVE_CH=-1\r\nMIDI_TRIG_CH1=0\r\nMIDI_TRIG_CH2=1\r\nMIDI_TRIG_CH3=2\r\nMIDI_TRIG_CH4=3\r\nMIDI_TRIG_CH5=4\r\nMIDI_TRIG_CH6=5\r\nMIDI_TRIG_CH7=6\r\nMIDI_TRIG_CH8=7\r\nMIDI_AUTO_CHANNEL=10\r\nMIDI_SOFT_THRU=0\r\nMIDI_AUDIO_TRK_CC_IN=1\r\nMIDI_AUDIO_TRK_CC_OUT=3\r\nMIDI_AUDIO_TRK_NOTE_IN=1\r\nMIDI_AUDIO_TRK_NOTE_OUT=3\r\nMIDI_MIDI_TRK_CC_IN=1\r\nPATTERN_CHANGE_CHAIN_BEHAVIOR=0\r\nPATTERN_CHANGE_AUTO_SILENCE_TRACKS=0\r\nPATTERN_CHANGE_AUTO_TRIG_LFOS=0\r\nLOAD_24BIT_FLEX=0\r\nDYNAMIC_RECORDERS=0\r\nRECORD_24BIT=0\r\nRESERVED_RECORDER_COUNT=8\r\nRESERVED_RECORDER_LENGTH=16\r\nINPUT_DELAY_COMPENSATION=0\r\nGATE_AB=127\r\nGATE_CD=127\r\nGAIN_AB=64\r\nGAIN_CD=64\r\nDIR_AB=0\r\nDIR_CD=0\r\nPHONES_MIX=64\r\nMAIN_TO_CUE=0\r\nMASTER_TRACK=0\r\nCUE_STUDIO_MODE=0\r\nMAIN_LEVEL=64\r\nCUE_LEVEL=64\r\nMETRONOME_TIME_SIGNATURE=3\r\nMETRONOME_TIME_SIGNATURE_DENOMINATOR=2\r\nMETRONOME_PREROLL=0\r\nMETRONOME_CUE_VOLUME=32\r\nMETRONOME_MAIN_VOLUME=0\r\nMETRONOME_PITCH=12\r\nMETRONOME_TONAL=1\r\nMETRONOME_ENABLED=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\nTRIG_MODE_MIDI=0\r\n[/SETTINGS]
----

[SETTINGS]
WRITEPROTECTED=0
TEMPOx24=2880
PATTERN_TEMPO_ENABLED=0
MIDI_CLOCK_SEND=0
MIDI_CLOCK_RECEIVE=0
MIDI_TRANSPORT_SEND=0
MIDI_TRANSPORT_RECEIVE=0
MIDI_PROGRAM_CHANGE_SEND=0
MIDI_PROGRAM_CHANGE_SEND_CH=-1
MIDI_PROGRAM_CHANGE_RECEIVE=0
MIDI_PROGRAM_CHANGE_RECEIVE_CH=-1
MIDI_TRIG_CH1=0\r\nMIDI_TRIG_CH2=1
MIDI_TRIG_CH3=2\r\nMIDI_TRIG_CH4=3
MIDI_TRIG_CH5=4\r\nMIDI_TRIG_CH6=5
MIDI_TRIG_CH7=6\r\nMIDI_TRIG_CH8=7
MIDI_AUTO_CHANNEL=10
MIDI_SOFT_THRU=0
MIDI_AUDIO_TRK_CC_IN=1
MIDI_AUDIO_TRK_CC_OUT=3
MIDI_AUDIO_TRK_NOTE_IN=1
MIDI_AUDIO_TRK_NOTE_OUT=3
MIDI_MIDI_TRK_CC_IN=1
PATTERN_CHANGE_CHAIN_BEHAVIOR=0
PATTERN_CHANGE_AUTO_SILENCE_TRACKS=0
PATTERN_CHANGE_AUTO_TRIG_LFOS=0
LOAD_24BIT_FLEX=0
DYNAMIC_RECORDERS=0
RECORD_24BIT=0
RESERVED_RECORDER_COUNT=8
RESERVED_RECORDER_LENGTH=16
INPUT_DELAY_COMPENSATION=0
GATE_AB=127
GATE_CD=127
GAIN_AB=64
GAIN_CD=64
DIR_AB=0
DIR_CD=0
PHONES_MIX=64
MAIN_TO_CUE=0
MASTER_TRACK=0
CUE_STUDIO_MODE=0
MAIN_LEVEL=64
CUE_LEVEL=64
METRONOME_TIME_SIGNATURE=3
METRONOME_TIME_SIGNATURE_DENOMINATOR=2
METRONOME_PREROLL=0
METRONOME_CUE_VOLUME=32
METRONOME_MAIN_VOLUME=0
METRONOME_PITCH=12
METRONOME_TONAL=1
METRONOME_ENABLED=0
TRIG_MODE_MIDI=0
TRIG_MODE_MIDI=0
TRIG_MODE_MIDI=0
TRIG_MODE_MIDI=0
TRIG_MODE_MIDI=0
TRIG_MODE_MIDI=0
TRIG_MODE_MIDI=0
TRIG_MODE_MIDI=0
[/SETTINGS]
*/

/// Project settings read from a parsed Octatrack Project file

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectSettings {
    /// Whether the project can be written to (is currently being read/written when `true`)
    pub write_protected: bool,

    /// Current settings in the `Project`'s control menu UI.
    pub control: ControlMenu,

    /// Unknown: Whether MIDI 'Thru' is enabled/disabled?
    pub midi_soft_thru: bool,

    /// Current state of the settings in the Mixer Menu overview
    pub mixer: MixerMenu,

    /// Current state of the settings in the Tempo menu
    pub tempo: TempoMenu,

    /// Current selections for MIDI Track Trig Mode
    pub midi_tracks_trig_mode: MidiTrackTrigModes,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
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
        }
    }
}

impl ProjectFromString for ProjectSettings {
    type T = Self;

    /// Load project 'state' data from the raw project ASCII file.
    fn from_string(s: &str) -> RBoxErr<Self> {
        let hmap = string_to_hashmap(s, &ProjectRawFileSection::Settings)?;

        Ok(Self {
            write_protected: parse_hashmap_string_value_bool(&hmap, "writeprotected", None)?,
            // Unknown: Whether MIDI 'Thru' is enabled/disabled?
            midi_soft_thru: parse_hashmap_string_value_bool(&hmap, "midi_soft_thru", None)?,
            //
            control: ControlMenu::from_hashmap(&hmap)?,
            mixer: MixerMenu::from_hashmap(&hmap)?,
            tempo: TempoMenu::from_hashmap(&hmap)?,
            midi_tracks_trig_mode: MidiTrackTrigModes::from_hashmap(&hmap)?,
        })
    }
}

impl ProjectToString for ProjectSettings {
    /// Extract `OctatrackProjectMetadata` fields from the project file's ASCII data
    fn to_string(&self) -> RBoxErr<String> {
        let mut s = "".to_string();
        s.push_str("[SETTINGS]");
        s.push_str("\r\n");

        s.push_str(format!("WRITEPROTECTED={}", self.write_protected as u8).as_str());
        s.push_str("\r\n");
        s.push_str(format!("TEMPOx24={}", self.tempo.tempo * 24).as_str());
        s.push_str("\r\n");
        s.push_str(
            format!(
                "PATTERN_TEMPO_ENABLED={}",
                self.tempo.pattern_tempo_enabled as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");

        s.push_str(
            format!(
                "MIDI_CLOCK_SEND={}",
                self.control.midi.sync.midi_clock_send as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_CLOCK_RECEIVE={}",
                self.control.midi.sync.midi_clock_receive as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_TRANSPORT_SEND={}",
                self.control.midi.sync.midi_transport_send as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_TRANSPORT_RECEIVE={}",
                self.control.midi.sync.midi_transport_receive as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_PROGRAM_CHANGE_SEND={}",
                self.control.midi.sync.midi_progchange_send as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_PROGRAM_CHANGE_SEND_CH={}",
                self.control
                    .midi
                    .sync
                    .midi_progchange_send_channel
                    .value()?
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_PROGRAM_CHANGE_RECEIVE={}",
                self.control.midi.sync.midi_progchange_receive as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_PROGRAM_CHANGE_RECEIVE_CH={}",
                self.control
                    .midi
                    .sync
                    .midi_progchange_receive_channel
                    .value()?
            )
            .as_str(),
        );
        s.push_str("\r\n");

        s.push_str(format!("MIDI_TRIG_CH1={}", self.control.midi.channels.midi_trig_ch1).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_TRIG_CH2={}", self.control.midi.channels.midi_trig_ch2).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_TRIG_CH3={}", self.control.midi.channels.midi_trig_ch3).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_TRIG_CH4={}", self.control.midi.channels.midi_trig_ch4).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_TRIG_CH5={}", self.control.midi.channels.midi_trig_ch5).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_TRIG_CH6={}", self.control.midi.channels.midi_trig_ch6).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_TRIG_CH7={}", self.control.midi.channels.midi_trig_ch7).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MIDI_TRIG_CH8={}", self.control.midi.channels.midi_trig_ch8).as_str());
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_AUTO_CHANNEL={}",
                self.control.midi.channels.midi_auto_channel
            )
            .as_str(),
        );
        s.push_str("\r\n");

        s.push_str(format!("MIDI_SOFT_THRU={}", self.midi_soft_thru as u8).as_str());
        s.push_str("\r\n");

        s.push_str(
            format!(
                "MIDI_AUDIO_TRK_CC_IN={}",
                self.control.midi.control.midi_audio_track_cc_in as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_AUDIO_TRK_CC_OUT={}",
                self.control.midi.control.midi_audio_track_cc_out
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!("MIDI_AUDIO_TRK_NOTE_IN={}", {
                self.control.midi.control.midi_audio_track_note_in
            })
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_AUDIO_TRK_NOTE_OUT={}",
                self.control.midi.control.midi_audio_track_note_out
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "MIDI_MIDI_TRK_CC_IN={}",
                self.control.midi.control.midi_midi_track_cc_in
            )
            .as_str(),
        );
        s.push_str("\r\n");

        s.push_str(
            format!(
                "PATTERN_CHANGE_CHAIN_BEHAVIOR={}",
                self.control.sequencer.pattern_change_chain_behaviour
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "PATTERN_CHANGE_AUTO_SILENCE_TRACKS={}",
                self.control.sequencer.pattern_change_auto_silence_tracks as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "PATTERN_CHANGE_AUTO_TRIG_LFOS={}",
                self.control.sequencer.pattern_change_auto_trig_lfos as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");

        s.push_str(
            format!(
                "LOAD_24BIT_FLEX={}",
                self.control.memory.load_24bit_flex as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "DYNAMIC_RECORDERS={}",
                self.control.memory.dynamic_recorders as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(format!("RECORD_24BIT={}", self.control.memory.record_24bit as u8).as_str());
        s.push_str("\r\n");
        s.push_str(
            format!(
                "RESERVED_RECORDER_COUNT={}",
                self.control.memory.reserved_recorder_count
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "RESERVED_RECORDER_LENGTH={}",
                self.control.memory.reserved_recorder_length
            )
            .as_str(),
        );
        s.push_str("\r\n");

        s.push_str(
            format!(
                "INPUT_DELAY_COMPENSATION={}",
                self.control.input.input_delay_compensation as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");

        s.push_str(format!("GATE_AB={}", self.control.input.gate_ab).as_str());
        s.push_str("\r\n");
        s.push_str(format!("GATE_CD={}", self.control.input.gate_cd).as_str());
        s.push_str("\r\n");
        s.push_str(format!("GAIN_AB={}", self.mixer.gain_ab).as_str());
        s.push_str("\r\n");
        s.push_str(format!("GAIN_CD={}", self.mixer.gain_cd).as_str());
        s.push_str("\r\n");
        s.push_str(format!("DIR_AB={}", self.mixer.dir_ab).as_str());
        s.push_str("\r\n");
        s.push_str(format!("DIR_CD={}", self.mixer.dir_cd).as_str());
        s.push_str("\r\n");
        s.push_str(format!("PHONES_MIX={}", self.mixer.phones_mix).as_str());
        s.push_str("\r\n");
        s.push_str(format!("MAIN_TO_CUE={}", self.mixer.main_to_cue).as_str());
        s.push_str("\r\n");

        s.push_str(format!("MASTER_TRACK={}", self.control.audio.master_track as u8).as_str());
        s.push_str("\r\n");
        s.push_str(
            format!(
                "CUE_STUDIO_MODE={}",
                self.control.audio.cue_studio_mode as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");

        s.push_str(format!("MAIN_LEVEL={}", self.mixer.main_level).as_str());
        s.push_str("\r\n");
        s.push_str(format!("CUE_LEVEL={}", self.mixer.cue_level).as_str());
        s.push_str("\r\n");

        s.push_str(
            format!(
                "METRONOME_TIME_SIGNATURE={}",
                self.control.metronome.metronome_time_signature
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "METRONOME_TIME_SIGNATURE_DENOMINATOR={}",
                self.control.metronome.metronome_time_signature_denominator
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "METRONOME_PREROLL={}",
                self.control.metronome.metronome_preroll
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "METRONOME_CUE_VOLUME={}",
                self.control.metronome.metronome_cue_volume
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "METRONOME_MAIN_VOLUME={}",
                self.control.metronome.metronome_main_volume
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(format!("METRONOME_PITCH={}", self.control.metronome.metronome_pitch).as_str());
        s.push_str("\r\n");
        s.push_str(
            format!(
                "METRONOME_TONAL={}",
                self.control.metronome.metronome_tonal as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "METRONOME_ENABLED={}",
                self.control.metronome.metronome_enabled as u8
            )
            .as_str(),
        );
        s.push_str("\r\n");

        s.push_str(
            format!(
                "TRIG_MODE_MIDI={}",
                self.midi_tracks_trig_mode.trig_mode_midi_track_1
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "TRIG_MODE_MIDI={}",
                self.midi_tracks_trig_mode.trig_mode_midi_track_2
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "TRIG_MODE_MIDI={}",
                self.midi_tracks_trig_mode.trig_mode_midi_track_3
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "TRIG_MODE_MIDI={}",
                self.midi_tracks_trig_mode.trig_mode_midi_track_4
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "TRIG_MODE_MIDI={}",
                self.midi_tracks_trig_mode.trig_mode_midi_track_5
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "TRIG_MODE_MIDI={}",
                self.midi_tracks_trig_mode.trig_mode_midi_track_6
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "TRIG_MODE_MIDI={}",
                self.midi_tracks_trig_mode.trig_mode_midi_track_7
            )
            .as_str(),
        );
        s.push_str("\r\n");
        s.push_str(
            format!(
                "TRIG_MODE_MIDI={}",
                self.midi_tracks_trig_mode.trig_mode_midi_track_8
            )
            .as_str(),
        );

        s.push_str("\r\n[/SETTINGS]");

        Ok(s)
    }
}
