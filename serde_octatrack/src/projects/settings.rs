//! Project settings from a parsed Octatrack Project file.
//! e.g. whether Track 8 is a master track or not.

pub mod control_menu;
pub mod mixer;
pub mod tempo;
pub mod trig_mode_midi_tracks;

use serde::{Deserialize, Serialize};

use crate::common::{
    FromHashMap, OptionEnumValueConvert, ParseHashMapValueAs, ProjectFromString, ProjectToString,
};

use crate::projects::{
    common::string_to_hashmap,
    common::ProjectRawFileSection,
    settings::{
        control_menu::ControlMenu, mixer::MixerMenu, tempo::TempoMenu,
        trig_mode_midi_tracks::MidiTrackTrigModes,
    },
};

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

impl ParseHashMapValueAs for ProjectSettings {}

impl ProjectFromString for ProjectSettings {
    type T = Self;

    /// Load project 'state' data from the raw project ASCII file.
    fn from_string(s: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let hmap = string_to_hashmap(&s, &ProjectRawFileSection::Settings)?;

        Ok(Self {
            write_protected: Self::parse_hashmap_value_bool(&hmap, "writeprotected")?,
            // Unknown: Whether MIDI 'Thru' is enabled/disabled?
            midi_soft_thru: Self::parse_hashmap_value_bool(&hmap, "midi_soft_thru")?,
            //
            control: ControlMenu::from_hashmap(&hmap).unwrap(),
            mixer: MixerMenu::from_hashmap(&hmap)?,
            tempo: TempoMenu::from_hashmap(&hmap)?,
            midi_tracks_trig_mode: MidiTrackTrigModes::from_hashmap(&hmap)?,
        })
    }
}

impl ProjectToString for ProjectSettings {
    /// Extract `OctatrackProjectMetadata` fields from the project file's ASCII data

    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
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
                    .value()
                    .unwrap()
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
                    .value()
                    .unwrap()
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
            format!(
                "MIDI_AUDIO_TRK_NOTE_IN={}",
                self.control.midi.control.midi_audio_track_note_in as u8
            )
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
