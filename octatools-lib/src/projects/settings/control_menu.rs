//! Data structures for the Octatrack Project Settings 'Control Menu'.

use crate::RBoxErr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::projects::{
    options::ProjectMidiChannels, parse_hashmap_string_value, parse_hashmap_string_value_bool,
    FromHashMap,
};
use crate::OptionEnumValueConvert;

/// Convenience struct for all data related to the Octatrack Project Settings 'Control' Menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ControlMenu {
    /// 'Audio' page
    pub audio: AudioControlPage,

    /// 'Input' page
    pub input: InputControlPage,

    /// 'Sequencer' page
    pub sequencer: SequencerControlPage,

    /// 'MIDI Sequencer' page
    // TODO?!?!?!??!
    pub midi_sequencer: MidiSequencerControlPage,

    /// 'Memory' page
    pub memory: MemoryControlPage,

    /// 'Metronome' page
    pub metronome: MetronomeControlPage,

    /// 'Midi' sub menu
    pub midi: MidiSubMenu,
}

impl FromHashMap for ControlMenu {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            audio: AudioControlPage::from_hashmap(hmap)?,
            input: InputControlPage::from_hashmap(hmap)?,
            sequencer: SequencerControlPage::from_hashmap(hmap)?,
            midi_sequencer: MidiSequencerControlPage {},
            memory: MemoryControlPage::from_hashmap(hmap)?,
            metronome: MetronomeControlPage::from_hashmap(hmap)?,
            midi: MidiSubMenu::from_hashmap(hmap)?,
        })
    }
}

/// Convenience struct for all data related to the 'MIDI' sub-menu
/// within the Octatrack Project Settings 'Control' Menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MidiSubMenu {
    pub control: MidiControlMidiPage,
    pub sync: MidiSyncMidiPage,
    pub channels: MidiChannelsMidiPage,
    // TODO?!?!
    // control_midi_turbo: todo!(),
}

impl FromHashMap for MidiSubMenu {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            control: MidiControlMidiPage::from_hashmap(hmap)?,
            sync: MidiSyncMidiPage::from_hashmap(hmap)?,
            channels: MidiChannelsMidiPage::from_hashmap(hmap)?,
        })
    }
}

/// `PROJECT` -> `CONTROL` -> `AUDIO` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AudioControlPage {
    /// `TRACK 8` setting. Whether Track 8 is a master audio track or not:
    /// - **NORMAL**: `false`
    /// - **MASTER**: `true`
    pub master_track: bool,

    /// `CUE CFG` setting. Behaviour for audio routing to CUE outputs.
    /// - **NORMAL** -> **CUE+TRACK** button combo sends audio to CUE out.
    /// - **STUDIO** -> Individual track volume controls for CUE out (unable to **CUE+TRACK**).
    pub cue_studio_mode: bool,
}

impl FromHashMap for AudioControlPage {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            master_track: parse_hashmap_string_value_bool(hmap, "master_track", None)?,
            cue_studio_mode: parse_hashmap_string_value_bool(hmap, "cue_studio_mode", None)?,
        })
    }
}

/// `PROJECT` -> `CONTROL` -> `INPUT` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InputControlPage {
    /// dB level of noise gate for the AB external audio inputs.
    /// See Manual section 8.8 MIXER MENU
    pub gate_ab: u8, // 127 is default so i assume this is u8? midpoint?

    /// dB level of noise gate for the CD external audio inputs.
    /// See Manual section 8.8 MIXER MENU
    pub gate_cd: u8, // 127 is default so i assume this is u8? midpoint?

    /// See Manual section 8.6.2. INPUT.
    /// Adds a delay to incoming external audio signals. Controlled by the DIR setting on the MIXER page.
    pub input_delay_compensation: bool,
}

impl FromHashMap for InputControlPage {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            gate_ab: parse_hashmap_string_value::<u8>(hmap, "gate_ab", None)?,
            gate_cd: parse_hashmap_string_value::<u8>(hmap, "gate_cd", None)?,
            input_delay_compensation: parse_hashmap_string_value_bool(
                hmap,
                "input_delay_compensation",
                None,
            )?,
        })
    }
}

/// `PROJECT` -> `CONTROL` -> `SEQUENCER` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SequencerControlPage {
    /// `CHAIN AFTER` setting.
    /// When chained patterns start playing once the pattern is chosen.
    /// This is the global project level setting, but can be overidden for each pattern.
    /// Default setting is "PATTERN LENGTH".
    /// See Manual section 8.6.3. SEQUENCER.
    pub pattern_change_chain_behaviour: u8, // bool?

    /// `SILENCE TRACKS` setting
    /// Silence tracks when switching to a new pattern.
    /// See Manual section 8.6.3. SEQUENCER.
    pub pattern_change_auto_silence_tracks: bool,

    /// `LFO AUTO CHANGE` setting.
    /// Whether to retrigger LFOs when swtiching to a new pattern
    /// See Manual section 8.6.3. SEQUENCER.
    pub pattern_change_auto_trig_lfos: bool,
}

impl FromHashMap for SequencerControlPage {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            pattern_change_chain_behaviour: parse_hashmap_string_value::<u8>(
                hmap,
                "pattern_change_chain_behavior",
                None,
            )?,
            pattern_change_auto_silence_tracks: parse_hashmap_string_value_bool(
                hmap,
                "pattern_change_auto_trig_lfos",
                None,
            )?,
            pattern_change_auto_trig_lfos: parse_hashmap_string_value_bool(
                hmap,
                "pattern_change_auto_trig_lfos",
                None,
            )?,
        })
    }
}

/// `PROJECT` -> `CONTROL` -> `MIDI SEQUENCER` UI menu.
// TODO: ?!?!?!?! Where is the value for this??!?!?!
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MidiSequencerControlPage {}

/// `PROJECT` -> `CONTROL` -> `MEMORY` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MemoryControlPage {
    /// Whether samples can be loaded in 24-bit depth (16 bit depth samples are always oaded as 16 bit).
    /// Setting this to false loads all samples as 16 bit depth.
    /// See Manual section 8.6.5. MEMORY.
    pub load_24bit_flex: bool,

    /// Disabled forces all recorders to use track recorder memory (16 seconds per track).
    /// When enabled, track recorders can use free Flex RAM memory.
    /// See Manual section 8.6.5. MEMORY.
    pub dynamic_recorders: bool,

    /// Whether to record in 24 bit depth (`true`) or 16 bit depth (`false`).
    /// See Manual section 8.6.5. MEMORY.
    pub record_24bit: bool,

    /// How many active track recorders are available in a project. Controls whether TR1 through to TR8 are enabled / disabled.
    /// See Manual section 8.6.5. MEMORY.
    pub reserved_recorder_count: u8,

    /// How many 'sequencer steps' should be reserved for track recorders in RAM.
    /// See Manual section 8.6.5. MEMORY.
    pub reserved_recorder_length: u32,
}

impl FromHashMap for MemoryControlPage {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            load_24bit_flex: parse_hashmap_string_value_bool(hmap, "load_24bit_flex", None)?,
            dynamic_recorders: parse_hashmap_string_value_bool(hmap, "dynamic_recorders", None)?,
            record_24bit: parse_hashmap_string_value_bool(hmap, "record_24bit", None)?,
            reserved_recorder_count: parse_hashmap_string_value::<u8>(
                hmap,
                "reserved_recorder_count",
                None,
            )?,
            reserved_recorder_length: parse_hashmap_string_value::<u32>(
                hmap,
                "reserved_recorder_length",
                None,
            )?,
        })
    }
}

/// `PROJECT` -> `CONTROL` -> `METRONOME` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MetronomeControlPage {
    /// `TIME SIG. NUMER` setting in `PROJECT` -> `CONTROL` -> `METRONOME` UI menu.
    /// Controls the numerator for time signature (the 3 in 3/4).
    /// See Manual section 8.6.6 METRONOME
    pub metronome_time_signature: u8, // i'm guessing 3 is actually 4/4? 0-indexed

    /// `TIME SIG. DENOM` setting in `PROJECT` -> `CONTROL` -> `METRONOME` UI menu.
    /// Controls the numerator for time signature (the 3 in 3/4).
    /// See Manual section 8.6.6 METRONOME
    pub metronome_time_signature_denominator: u8, // i'm guessing 2 is actually 4/4? 0-indexed

    /// `PREROLL` setting in `PROJECT` -> `CONTROL` -> `METRONOME` UI menu.
    /// How many bars to prerolls with the metronome before playing a pattern.
    /// See Manual section 8.6.6 METRONOME
    pub metronome_preroll: u8, // what is the maximum for this?

    /// How loud to play the metronome on CUE outputs. Default is 32.
    /// See Manual section 8.6.6 METRONOME
    pub metronome_cue_volume: u8,

    /// How loud to play the metronome on MAIN outputs. Default is 0.
    /// See Manual section 8.6.6 METRONOME
    pub metronome_main_volume: u8,

    /// Pitch of the metronome clicks. Default is 12.
    /// See Manual section 8.6.6 METRONOME
    pub metronome_pitch: u8,

    /// Whether the metronome click has tonal characteristics or not. Default is `true` (enabled).
    /// See Manual section 8.6.6 METRONOME
    pub metronome_tonal: bool,

    /// Whether the metronome is active. Default is `false`.
    /// See Manual section 8.6.6 METRONOME
    pub metronome_enabled: bool,
}

impl FromHashMap for MetronomeControlPage {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            metronome_time_signature: parse_hashmap_string_value::<u8>(
                hmap,
                "metronome_time_signature",
                None,
            )?,
            metronome_time_signature_denominator: parse_hashmap_string_value::<u8>(
                hmap,
                "metronome_time_signature_denominator",
                None,
            )?,
            metronome_preroll: parse_hashmap_string_value::<u8>(hmap, "metronome_preroll", None)?,
            metronome_cue_volume: parse_hashmap_string_value::<u8>(
                hmap,
                "metronome_cue_volume",
                None,
            )?,
            metronome_main_volume: parse_hashmap_string_value::<u8>(
                hmap,
                "metronome_main_volume",
                None,
            )?,
            metronome_pitch: parse_hashmap_string_value::<u8>(hmap, "metronome_pitch", None)?,
            metronome_tonal: parse_hashmap_string_value_bool(hmap, "metronome_tonal", None)?,
            metronome_enabled: parse_hashmap_string_value_bool(hmap, "metronome_enabled", None)?,
        })
    }
}

/// `PROJECT` -> `CONTROL` -> `MIDI` -> `CONTROL` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MidiControlMidiPage {
    /// Whether samples can be loaded in 24-bit depth (16 bit depth samples are always oaded as 16 bit).
    /// `AUDIO CC IN` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CONTROL` UI menu.
    /// Whether audio tracks respond to MIDI CC IN messages.
    ///
    /// See manual section 8.7.1 CONTROL.
    pub midi_audio_track_cc_in: bool,

    /// `AUDIO CC OUT` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CONTROL` UI menu.
    /// Whether audio tracks send MIDI CC OUT messages.
    /// Three options:
    /// - `INT`: No messages sent, knobs only affect Octatrack settings.
    /// - `EXT`: Sends CC OUT messages but they don't alter any Octatrack settings.
    /// - `INT+EXT`: Simulataneously affects Octratack settings and sends CC OUT messages.
    ///
    /// See manual section 8.7.1 CONTROL.
    pub midi_audio_track_cc_out: u8,

    /// `AUDIO NOTE IN` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CONTROL` UI menu.
    /// Whether to receive MIDI NOTE IN messages on Audio tracks and how the audio tracks
    /// respond to those MIDI NOTE IN messages.
    /// - **OFF**: midi note has no effet.
    /// - **STANDARD**: standard note mapping (default).
    /// - **FOLLOW TM**: Track's current trig mode affects audio tracks (track/chromatic/slots).
    /// - **MAP/TRACK**: Uses MIDI MAP configuration on a per track basis (track/chromatic/slots
    ///     disconnected from user trig mode of track).
    pub midi_audio_track_note_in: u8,

    /// `AUDIO NOTE OUT` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CONTROL` UI menu.
    /// Whether audio tracks send MIDI NOTE OUT messages. Three options:
    /// - `INT`: No messages sent, knobs only affect Octatrack settings.
    /// - `EXT`: Sends NOTE OUT messages but they don't alter any Octatrack settings.
    /// - `INT+EXT`: Simulataneously affects Octratack settings and sends NOTE OUT messages.
    ///
    /// See manual section 8.7.1 CONTROL.
    pub midi_audio_track_note_out: u8,

    /// Unknown. MIDI channel to MIDI Track CC In messages n (1 - 16) ?
    pub midi_midi_track_cc_in: u8,
}

impl FromHashMap for MidiControlMidiPage {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            midi_audio_track_cc_in: parse_hashmap_string_value_bool(
                hmap,
                "midi_audio_trk_cc_in",
                None,
            )?,
            midi_audio_track_cc_out: parse_hashmap_string_value::<u8>(
                hmap,
                "midi_audio_trk_cc_out",
                None,
            )?,
            midi_audio_track_note_in: parse_hashmap_string_value::<u8>(
                hmap,
                "midi_audio_trk_note_in",
                None,
            )?,
            midi_audio_track_note_out: parse_hashmap_string_value::<u8>(
                hmap,
                "midi_audio_trk_note_out",
                None,
            )?,
            midi_midi_track_cc_in: parse_hashmap_string_value::<u8>(
                hmap,
                "midi_midi_trk_cc_in",
                None,
            )?,
        })
    }
}

/// `PROJECT` -> `CONTROL` -> `MIDI` -> `SYNC` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MidiSyncMidiPage {
    /// `CLOCK SEND` setting.
    /// Whether MIDI clock sending is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    pub midi_clock_send: bool,

    /// `CLOCK RECV` setting.
    /// Whether MIDI clock receiving is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    pub midi_clock_receive: bool,

    /// `TRANS SEND` setting.
    /// Whether MIDI transport sending is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    pub midi_transport_send: bool,

    /// `TRANS RECV` setting.
    /// Whether MIDI transport receiving is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    pub midi_transport_receive: bool,

    /// `PROG CH SEND` setting.
    /// Whether MIDI Program Change sending is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    pub midi_progchange_send: bool,

    /// `CHANNEL` setting.
    /// Channel to send MIDI Program Change messages on. (-1, or between 1 - 16).
    /// **NOTE**: should be set to `-1` when `midi_progchange_send` is disabled.
    /// See manual section 8.7.2 SYNC.
    pub midi_progchange_send_channel: ProjectMidiChannels,

    /// `PROG CH RECEIVE` setting.
    /// Whether MIDI Program Change receiveing is enabled/disabled
    /// See manual section 8.7.2 SYNC.
    pub midi_progchange_receive: bool,

    /// `CHANNEL` setting.
    /// Channel to receive MIDI Program Change messages on (-1 or between 1 - 16).
    /// **NOTE**: should be set to `-1` when `midi_progchange_receive` is disabled.
    /// See manual section 8.7.2 SYNC.
    pub midi_progchange_receive_channel: ProjectMidiChannels,
}

impl FromHashMap for MidiSyncMidiPage {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            midi_clock_send: parse_hashmap_string_value_bool(hmap, "midi_clock_send", None)?,
            midi_clock_receive: parse_hashmap_string_value_bool(hmap, "midi_clock_receive", None)?,
            midi_transport_send: parse_hashmap_string_value_bool(
                hmap,
                "midi_transport_send",
                None,
            )?,
            midi_transport_receive: parse_hashmap_string_value_bool(
                hmap,
                "midi_transport_receive",
                None,
            )?,
            midi_progchange_send: parse_hashmap_string_value_bool(
                hmap,
                "midi_program_change_send",
                None,
            )?,
            midi_progchange_send_channel: ProjectMidiChannels::from_value(
                &parse_hashmap_string_value::<i8>(hmap, "midi_program_change_send_ch", None)?,
            )?,
            midi_progchange_receive: parse_hashmap_string_value_bool(
                hmap,
                "midi_program_change_receive",
                None,
            )?,
            midi_progchange_receive_channel: ProjectMidiChannels::from_value(
                &parse_hashmap_string_value::<i8>(hmap, "midi_program_change_receive_ch", None)?,
            )?,
        })
    }
}

/// `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MidiChannelsMidiPage {
    /// `TRIG CH 1` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 1 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    pub midi_trig_ch1: u8,

    /// `TRIG CH 2` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 2 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    pub midi_trig_ch2: u8,

    /// `TRIG CH 3` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 3 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    pub midi_trig_ch3: u8,

    /// `TRIG CH 4` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 4 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    pub midi_trig_ch4: u8,

    /// `TRIG CH 5` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 5 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    pub midi_trig_ch5: u8,

    /// `TRIG CH 6` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 6 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    pub midi_trig_ch6: u8,

    /// `TRIG CH 7` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 7 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    pub midi_trig_ch7: u8,

    /// `TRIG CH 8` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// MIDI Channel to send MIDI Trig 8 messages to (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    pub midi_trig_ch8: u8,

    /// `AUTO CH` setting in `PROJECT` -> `CONTROL` -> `MIDI` -> `CHANNELS` UI menu.
    /// Auto MIDI Channel (1 - 16)
    /// See manual section 8.7.3 CHANNELS.
    pub midi_auto_channel: u8,
}

impl FromHashMap for MidiChannelsMidiPage {
    type A = String;
    type B = String;
    type T = Self;

    fn from_hashmap(hmap: &HashMap<String, String>) -> RBoxErr<Self> {
        Ok(Self {
            midi_trig_ch1: parse_hashmap_string_value::<u8>(hmap, "midi_trig_ch1", None)?,
            midi_trig_ch2: parse_hashmap_string_value::<u8>(hmap, "midi_trig_ch2", None)?,
            midi_trig_ch3: parse_hashmap_string_value::<u8>(hmap, "midi_trig_ch3", None)?,
            midi_trig_ch4: parse_hashmap_string_value::<u8>(hmap, "midi_trig_ch4", None)?,
            midi_trig_ch5: parse_hashmap_string_value::<u8>(hmap, "midi_trig_ch5", None)?,
            midi_trig_ch6: parse_hashmap_string_value::<u8>(hmap, "midi_trig_ch6", None)?,
            midi_trig_ch7: parse_hashmap_string_value::<u8>(hmap, "midi_trig_ch7", None)?,
            midi_trig_ch8: parse_hashmap_string_value::<u8>(hmap, "midi_trig_ch8", None)?,
            midi_auto_channel: parse_hashmap_string_value::<u8>(hmap, "midi_auto_channel", None)?,
        })
    }
}
