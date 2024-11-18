//! Serialization and Deserialization of Part related data for Bank files.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::RBoxErr;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OnOrOff {
    On = 1,
    Off = 0,
}

/// Audio Track MAIN and CUE volume.
/// Both are 108 by default.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackVolume {
    pub main: u8,
    pub cue: u8,
}

/// Scenes currently selected in the Part.
/// Whether Scenes are muted or not are controlled at the Project level.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveScenes {
    pub scene_a: u8,
    pub scene_b: u8,
}

/// An Audio Track's Setup values for the Static machine on the track (loop setting/slice setting/len setting etc.).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsSetupStatic {
    pub xloop: u8,
    pub slic: u8,
    pub len: u8,
    pub rate: u8,
    pub tstr: u8,
    pub tsns: u8,
}

/// An Audio Track's Setup values for the Flex machine on the track (loop setting/slice setting/len setting etc.).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsSetupFlex {
    pub xloop: u8,
    pub slic: u8,
    pub len: u8,
    pub rate: u8,
    pub tstr: u8,
    pub tsns: u8,
}

/// An Audio Track's Setup values for the Thru machine on the track. Should not contain any data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsSetupThru {
    unused_1: u8,
    unused_2: u8,
    unused_3: u8,
    unused_4: u8,
    unused_5: u8,
    unused_6: u8,
}

/// An Audio Track's Setup values for the Neighbor machine on the track. Should not contain any data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsSetupNeighbor {
    unused_1: u8,
    unused_2: u8,
    unused_3: u8,
    unused_4: u8,
    unused_5: u8,
    unused_6: u8,
}

/// An Audio Track's Setup values for the Pickup machine on the track.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsSetupPickup {
    unused_1: u8,
    unused_2: u8,
    unused_3: u8,
    unused_4: u8,
    pub tstr: u8,
    pub tsns: u8,
}

/// Audio Tracks Machine Setup pages.
/// As before, separate from other Audio Track parameter fields to be persisted,
/// allowing safer audio triggering.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachinesParamsSetup {
    pub static_machine: AudioTrackMachineParamsSetupStatic,
    pub flex_machine: AudioTrackMachineParamsSetupFlex,
    pub thru_machine: AudioTrackMachineParamsSetupThru,
    pub neighbor_machine: AudioTrackMachineParamsSetupNeighbor,
    pub pickup_machine: AudioTrackMachineParamsSetupPickup,
}

/// Audio Tracks Machine Slot assignments.
/// Sample Slots assigned for each machine.
/// Also tracks the recording buffer sample slot assignment.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct AudioTrackMachineSlot {
    pub static_slot_id: u8,
    pub flex_slot_id: u8,
    unused_1: u8,
    unused_2: u8,
    pub recorder_slot_id: u8,
}

/// Values of an Audio Track's parameters page for the Static machine on the track (pitch/slice/len etc).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsValuesStatic {
    pub ptch: u8,
    pub strt: u8,
    pub len: u8,
    pub rate: u8,
    pub rtrg: u8,
    pub rtim: u8,
}

/// Values of an Audio Track's parameters page for the Flex machine on the track (pitch/slice/len etc).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsValuesFlex {
    pub ptch: u8,
    pub strt: u8,
    pub len: u8,
    pub rate: u8,
    pub rtrg: u8,
    pub rtim: u8,
}

/// Values of an Audio Track's parameters page for the Flex machine on the track (in/vol etc).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsValuesThru {
    pub in_ab: u8,
    pub vol_ab: u8,
    pub unused_1: u8,
    pub in_cd: u8,
    pub vol_cd: u8,
    pub unused_2: u8,
}

/// Values of an Audio Track's parameters page for the Neighbor machine on the track. Should not contain any data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsValuesNeighbor {
    unused_1: u8,
    unused_2: u8,
    unused_3: u8,
    unused_4: u8,
    unused_5: u8,
    unused_6: u8,
}

/// Values of an Audio Track's parameters page for the Pickup machine on the track. (pitch/dir/len etc).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachineParamsValuesPickup {
    pub ptch: u8,
    pub dir: u8,
    pub len: u8,
    unused_1: u8,
    pub gain: u8,
    pub op: u8,
}

/// Audio Tracks Machine Parameter pages.
/// Machine parameter values persist after the machine type is changed,
/// meaning audio keeps playing until the new machine is triggered to play new audio.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackMachinesParamsValues {
    pub static_machine: AudioTrackMachineParamsValuesStatic,
    pub flex_machine: AudioTrackMachineParamsValuesFlex,
    pub thru_machine: AudioTrackMachineParamsValuesThru,
    pub neighbor_machine: AudioTrackMachineParamsValuesNeighbor,
    pub pickup_machine: AudioTrackMachineParamsValuesPickup,
}

/// Values of a Track's main LFO parameters page.
/// Note that the speed and depth settings in the LFO Setup menu are looking at these values too.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LfoParamsValues {
    pub spd1: u8,
    pub spd2: u8,
    pub spd3: u8,
    pub dep1: u8,
    pub dep2: u8,
    pub dep3: u8,
}

/// Values of a Track's main AMP parameters page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackAmpParamsValues {
    pub atk: u8,
    pub hold: u8,
    pub rel: u8,
    pub vol: u8,
    pub bal: u8,
    /// Reserved space for the \<F\> parameter used by Scenes and LFOs
    unused: u8,
}

/// Values of a Track's FX parameters page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackFxParamsValues {
    pub param_1: u8,
    pub param_2: u8,
    pub param_3: u8,
    pub param_4: u8,
    pub param_5: u8,
    pub param_6: u8,
}

/// Audio Tracks Paramater Page values. LFO, Amp, FX etc.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackParamsValues {
    pub lfo: LfoParamsValues,
    pub amp: AudioTrackAmpParamsValues,
    pub fx1: AudioTrackFxParamsValues,
    pub fx2: AudioTrackFxParamsValues,
}

/// First set of values for a Track's LFO Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LfoParamsSetup1 {
    pub lfo1_pmtr: u8,
    pub lfo2_pmtr: u8,
    pub lfo3_pmtr: u8,
    pub lfo1_wave: u8,
    pub lfo2_wave: u8,
    pub lfo3_wave: u8,
}

/// Second set of values for a Track's LFO Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LfoParamsSetup2 {
    pub lfo1_mult: u8,
    pub lfo2_mult: u8,
    pub lfo3_mult: u8,
    pub lfo1_trig: u8,
    pub lfo2_trig: u8,
    pub lfo3_trig: u8,
}

/// Values for a Track's AMP Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackAmpParamsSetup {
    pub amp: u8,
    pub sync: u8,
    pub atck: u8,
    pub fx1: u8,
    pub fx2: u8,
    // This is the hidden <F> parameter used in LFOs and Scenes etc.
    unused: u8,
}

/// Values for a Track's FX Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackFxParamsSetup {
    pub setting1: u8,
    pub setting2: u8,
    pub setting3: u8,
    pub setting4: u8,
    pub setting5: u8,
    pub setting6: u8,
}

/// Audio Tracks Paramater Setup pages for LFO, Amp, FX etc.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackParamsSetup {
    /// Parameter Target and Wave selection for the 3x LFOs on this Audio Track.
    pub lfo_setup_1: LfoParamsSetup1,
    /// Amplitude setup page
    pub amp: AudioTrackAmpParamsSetup,
    /// FX #1 setup page
    pub fx1: AudioTrackFxParamsSetup,
    /// FX #2 setup page
    pub fx2: AudioTrackFxParamsSetup,
    /// Multiplier and Trigger type selection for the 3x LFOs on this Audio Track.
    pub lfo_setup_2: LfoParamsSetup2,
}

/// Values for a MIDI Track's MIDI parameter page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackMidiParamsValues {
    pub note: u8,
    pub vel: u8,
    pub len: u8,
    pub not2: u8,
    pub not3: u8,
    pub not4: u8,
}

/// Values for a MIDI Track's LFO parameter page.
/// todo: combine with lfo params?
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackLfoParamsValues {
    pub spd1: u8,
    pub spd2: u8,
    pub spd3: u8,
    pub dep1: u8,
    pub dep2: u8,
    pub dep3: u8,
}

/// Values for a MIDI Track's Arp parameter page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackArpParamsValues {
    pub tran: u8,
    pub leg: u8,
    pub mode: u8,
    pub spd: u8,
    pub rnge: u8,
    pub nlen: u8,
}

/// Values for a MIDI Track's CC1 parameter page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackCc1ParamsValues {
    pub pb: u8,
    pub at: u8,
    pub cc1: u8,
    pub cc2: u8,
    pub cc3: u8,
    pub cc4: u8,
}

/// Values for a MIDI Track's CC2 parameter page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackCc2ParamsValues {
    pub cc5: u8,
    pub cc6: u8,
    pub cc7: u8,
    pub cc8: u8,
    pub cc9: u8,
    pub cc10: u8,
}

/// MIDI Tracks Paramater values. LFO, Amp, FX etc.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackParamsValues {
    pub midi: MidiTrackMidiParamsValues,
    pub arp: MidiTrackLfoParamsValues,
    pub lfo: MidiTrackArpParamsValues,
    pub ctrl1: MidiTrackCc1ParamsValues,
    pub ctrl2: MidiTrackCc2ParamsValues,
    #[serde(with = "BigArray")]
    pub unknown: [u8; 2],
}

/// Values for a MIDI Track's Note Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackNoteParamsSetup {
    pub chan: u8,
    pub bank: u8,
    pub prog: u8,
    unused_1: u8,
    pub sbank: u8,
    unused_2: u8,
}

/// Values for a MIDI Track's Arp Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackArpParamsSetup {
    unused_1: u8,
    unused_2: u8,
    /// Length of the arp sequence
    pub len: u8,
    unused_3: u8,
    unused_4: u8,
    /// maximum 24 -- B Min
    /// mininim 0 -- Off
    pub key: u8,
}

/// Values for a MIDI Track's CC1 Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackCc1ParamsSetup {
    unused_1: u8,
    unused_2: u8,
    pub cc1: u8,
    pub cc2: u8,
    pub cc3: u8,
    pub cc4: u8,
}

/// Values for a MIDI Track's CC2 Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackCc2ParamsSetup {
    pub cc5: u8,
    pub cc6: u8,
    pub cc7: u8,
    pub cc8: u8,
    pub cc9: u8,
    pub cc10: u8,
}

/// MIDI Tracks Paramater Setup pages for LFO, Amp, FX etc.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackParamsSetup {
    pub note: MidiTrackNoteParamsSetup,
    pub lfo1: LfoParamsSetup1,
    pub arp: MidiTrackArpParamsSetup,
    pub ctrl1: MidiTrackCc1ParamsSetup,
    pub ctrl2: MidiTrackCc2ParamsSetup,
    pub lfo2: LfoParamsSetup2,
}

/// A custom LFO Design -- array of 16 values.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomLfo([u8; 16]);

/// Audio Tracks Custom LFO designs.
/// 0 -> 127 values (above line) maps to 0 -> 127.
/// -1 -> -127 values (above line) seems to map to 255 -> 128.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomLfos {
    #[serde(with = "BigArray")]
    pub track_1: [u8; 16],

    #[serde(with = "BigArray")]
    pub track_2: [u8; 16],

    #[serde(with = "BigArray")]
    pub track_3: [u8; 16],

    #[serde(with = "BigArray")]
    pub track_4: [u8; 16],

    #[serde(with = "BigArray")]
    pub track_5: [u8; 16],

    #[serde(with = "BigArray")]
    pub track_6: [u8; 16],

    #[serde(with = "BigArray")]
    pub track_7: [u8; 16],

    #[serde(with = "BigArray")]
    pub track_8: [u8; 16],
}

/// LFO Interpolation mask.
/// Indicates which LFO steps should have values interpolated when LFO is triggered.
/// Not sure exactly how the calculation works yet.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomLfoInterpolationMask {
    #[serde(with = "BigArray")]
    pub mask: [u8; 2],
}

/// First page of settings controlling an Audio Track's Recording Buffer configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RecorderSetupSources {
    pub in_ab: u8,
    pub in_cd: u8,
    /// `64` is MAX. `63` and lower are actual values.
    pub rlen: u8,
    pub trig: u8,
    /// internal recording source
    pub src3: u8,
    pub xloop: u8,
}

/// Second page of settings controlling an Audio Track's Recording Buffer configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RecorderSetupProcessing {
    pub fin: u8,
    pub fout: u8,
    pub ab: u8,
    pub qrec: u8,
    pub qpl: u8,
    pub cd: u8,
}

/// Recorder Setup Pages
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RecorderSetup {
    pub src: RecorderSetupSources,
    pub proc: RecorderSetupProcessing,
}

/// Scene Parameter assignments for an Audio Track.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SceneParamsTrack {
    /// Don't know what this data block is for yet :/
    #[serde(with = "BigArray")]
    pub ukn1: [u8; 340],

    /// The parameter assignments.
    #[serde(with = "BigArray")]
    pub params_values: [u8; 30],

    #[serde(with = "BigArray")]
    pub unk2: [u8; 159],

    /// Audio Track Main Volume fade
    /// `MAX` -> `127`.
    /// `MIN` -> `0`.
    pub xlv: u8,

    /// Don't know what this data block is for yet :/
    #[serde(with = "BigArray")]
    pub ukn9: [u8; 2],
}

/// A MIDI Track's custom Arp sequence.
/// `0` -> `+63` values (above line) maps to `0` -> `63`.
/// `-1` -> `-63` values (below line) map to `255` -> `192`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiArpSequence([u8; 16]);

// TODO: For some reaosn there are EIGHT part sections in the data file...
// I do not know why ... previous states?

/// Parts in the bank, containing track data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Part {
    // #[serde(deserialize_with = "deserialize_string", serialize_with="serialize_string")]
    // pub header: String,
    #[serde(with = "BigArray")]
    pub header: [u8; 4],

    /// All 0 values.
    #[serde(with = "BigArray")]
    pub data_block_1: [u8; 5],

    /// Audio Tracks active FX for FX1.
    /// ```text
    /// OFF -> 0
    /// FILTER -> 4
    /// EQ -> 12
    /// DJ EQ -> 13
    /// PHASER -> 16
    /// FLANGER -> 17
    /// CHORUS -> 18
    /// SPATIALIZER -> 5
    /// COMB FILTER -> 19
    /// COMPRESSOR -> 24
    /// LOFI -> 24
    /// ```
    #[serde(with = "BigArray")]
    pub audio_track_fx1: [u8; 8],

    /// Audio Tracks active FX for FX2.
    /// ```text
    /// OFF -> 0
    /// FILTER -> 4
    /// EQ -> 12
    /// DJ EQ -> 13
    /// PHASER -> 16
    /// FLANGER -> 17
    /// CHORUS -> 18
    /// SPATIALIZER -> 5
    /// COMB FILTER -> 19
    /// COMPRESSOR -> 24
    /// LOFI -> 24
    /// DELAY -> 8
    /// PLATE REVERB -> 20
    /// SPRING REVERB -> 21
    /// DARK REVERB -> 22
    /// ```
    #[serde(with = "BigArray")]
    pub audio_track_fx2: [u8; 8],

    /// Currently selected Scenes for a Part.
    pub active_scenes: ActiveScenes,

    /// Volumes for Audio Tracks.
    #[serde(with = "BigArray")]
    pub audio_track_volumes: [AudioTrackVolume; 8],

    /// Audio Tracks Machine types.
    /// Static = 0, Flex = 1, Thru = 2, Neighbour = 3, Pickup = 4.
    #[serde(with = "BigArray")]
    pub audio_track_machine_types: [u8; 8],

    /// Parameters for Audio Track machines:
    /// Static, Flex, Thru, Neighbor and Pickup machines.
    #[serde(with = "BigArray")]
    pub audio_track_machine_params: [AudioTrackMachinesParamsValues; 8],

    /// Audio Track audio processing parameter values:
    /// Amplitude, LFOs, fx1 and fx2.
    #[serde(with = "BigArray")]
    pub audio_track_params_values: [AudioTrackParamsValues; 8], // 32

    /// Setup values for Audio Track machines:
    /// Static, Flex, Thru, Neighbor and Pickup machines.
    #[serde(with = "BigArray")]
    pub audio_track_machine_setup: [AudioTrackMachinesParamsSetup; 8],

    /// Audio Track sample slot assignments.
    #[serde(with = "BigArray")]
    pub audio_track_machine_slots: [AudioTrackMachineSlot; 8],

    /// Audio Track audio processing setup values:
    /// Amplitude, LFOs, fx1 and fx2.
    #[serde(with = "BigArray")]
    pub audio_track_params_setup: [AudioTrackParamsSetup; 8],

    /// MIDI Track processing values:
    /// Amplitude, LFOs, fx1 and fx2.
    #[serde(with = "BigArray")]
    pub midi_track_params_values: [MidiTrackParamsValues; 8],

    /// MIDI Track processing setup values:
    /// Amplitude, LFOs, fx1 and fx2.
    #[serde(with = "BigArray")]
    pub midi_track_params_setup: [MidiTrackParamsSetup; 8],

    /// Audio Track Recorder settings.
    #[serde(with = "BigArray")]
    pub recorder_setup: [RecorderSetup; 8],

    /// Parameter assignments for Scenes.
    ///
    /// 255 is no assignment on the scene.
    /// Any other value is the Scene's assigned value for that control.
    #[serde(with = "BigArray")]
    pub scene_params: [SceneParamsTrack; 8],

    /// Custom LFO designs for Audio Tracks.
    pub audio_track_custom_lfo_designs: CustomLfos,

    /// Interpolation of steps in custom LFOs for Audio Tracks.
    #[serde(with = "BigArray")]
    pub audio_track_custom_lfos_interpolation_masks: [CustomLfoInterpolationMask; 8],

    /// Custom LFO designs for MIDI Tracks.
    pub midi_track_custom_lfos: CustomLfos,

    /// Interpolation of steps in custom LFOs for MIDI Tracks.
    #[serde(with = "BigArray")]
    pub midi_track_custom_lfos_interpolation_masks: [CustomLfoInterpolationMask; 8],

    /// Arp Sequence Mutes for MIDI tracks.
    /// Not sure how these work.
    /// Muting some of the notes on M-TR-8 has the last array element set to 7 instead of 255.
    #[serde(with = "BigArray")]
    pub midi_track_arp_mute_masks: [u8; 16],

    /// Arp Sequence Notes for MIDI tracks.
    pub midi_track_arp_seqs: [MidiArpSequence; 8],
}

impl Part {
    pub fn update_static_machine_slot(&mut self, old: &u8, new: &u8) -> RBoxErr<()> {
        for audio_track_slots in self.audio_track_machine_slots.iter_mut() {
            if audio_track_slots.static_slot_id == *old {
                audio_track_slots.static_slot_id = *new;
            }
        }

        Ok(())
    }

    pub fn update_flex_machine_slot(&mut self, old: &u8, new: &u8) -> RBoxErr<()> {
        for audio_track_slots in self.audio_track_machine_slots.iter_mut() {
            if audio_track_slots.flex_slot_id == *old {
                audio_track_slots.flex_slot_id = *new;
            }
        }

        Ok(())
    }
}
