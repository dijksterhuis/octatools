//! Serialization and Deserialization of Part related data for Bank files.

use crate::{CheckHeader, DefaultsArray};
use ot_tools_derive::DefaultsAsArray;
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};
use std::array::from_fn;

/// Header data for Parts, indicating when a new Part data section starts in binary data files:
/// `PART`
const PART_HEADER: [u8; 4] = [0x50, 0x41, 0x52, 0x54];

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OnOrOff {
    On = 1,
    Off = 0,
}

/// Audio Track MAIN and CUE volume.
/// Both are 108 by default.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
pub struct AudioTrackVolume {
    pub main: u8,
    pub cue: u8,
}

// TODO: Double check values
impl Default for AudioTrackVolume {
    fn default() -> Self {
        Self {
            main: 0x6c, // 108
            cue: 0x6c,  // 108
        }
    }
}

/// Scenes currently selected in the Part.
/// Whether Scenes are muted or not are controlled at the Project level.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveScenes {
    pub scene_a: u8,
    pub scene_b: u8,
}

impl Default for ActiveScenes {
    fn default() -> Self {
        Self {
            scene_a: 0,
            scene_b: 8,
        }
    }
}

/// An Audio Track's Setup values for the Static machine on the track (loop setting/slice setting/len setting etc.).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackMachineParamsSetupStatic {
    pub xloop: u8,
    pub slic: u8,
    pub len: u8,
    pub rate: u8,
    pub tstr: u8,
    pub tsns: u8,
}

/// An Audio Track's Setup values for the Flex machine on the track (loop setting/slice setting/len setting etc.).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackMachineParamsSetupFlex {
    pub xloop: u8,
    pub slic: u8,
    pub len: u8,
    pub rate: u8,
    pub tstr: u8,
    pub tsns: u8,
}

/// An Audio Track's Setup values for the Thru machine on the track. Should not contain any data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackMachineParamsSetupThru {
    unused_1: u8,
    unused_2: u8,
    unused_3: u8,
    unused_4: u8,
    unused_5: u8,
    unused_6: u8,
}

/// An Audio Track's Setup values for the Neighbor machine on the track. Should not contain any data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackMachineParamsSetupNeighbor {
    unused_1: u8,
    unused_2: u8,
    unused_3: u8,
    unused_4: u8,
    unused_5: u8,
    unused_6: u8,
}

/// An Audio Track's Setup values for the Pickup machine on the track.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
pub struct AudioTrackMachinesParamsSetup {
    pub static_machine: AudioTrackMachineParamsSetupStatic,
    pub flex_machine: AudioTrackMachineParamsSetupFlex,
    pub thru_machine: AudioTrackMachineParamsSetupThru,
    pub neighbor_machine: AudioTrackMachineParamsSetupNeighbor,
    pub pickup_machine: AudioTrackMachineParamsSetupPickup,
}

impl Default for AudioTrackMachinesParamsSetup {
    fn default() -> Self {
        Self {
            static_machine: AudioTrackMachineParamsSetupStatic {
                xloop: 1,
                slic: 0,
                len: 0,
                rate: 0,
                tstr: 1,
                tsns: 64,
            },
            flex_machine: AudioTrackMachineParamsSetupFlex {
                xloop: 1,
                slic: 0,
                len: 0,
                rate: 0,
                tstr: 1,
                tsns: 64,
            },
            thru_machine: AudioTrackMachineParamsSetupThru {
                unused_1: 0,
                unused_2: 0,
                unused_3: 0,
                unused_4: 0,
                unused_5: 0,
                unused_6: 0,
            },
            neighbor_machine: AudioTrackMachineParamsSetupNeighbor {
                unused_1: 0,
                unused_2: 0,
                unused_3: 0,
                unused_4: 0,
                unused_5: 0,
                unused_6: 0,
            },
            pickup_machine: AudioTrackMachineParamsSetupPickup {
                unused_1: 0,
                unused_2: 0,
                unused_3: 0,
                unused_4: 0,
                tstr: 1,
                tsns: 64,
            },
        }
    }
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

impl AudioTrackMachineSlot {
    /// WARNING: This `defaults` method is not from the `Defaults` trait, as we
    /// cannot use a default struct instance to create an array/vector of
    /// machine slots data --> the individual default depends on their position
    /// in the final array!
    ///
    /// In the future, maybe it might be worth creating `default_with` and
    /// `defaults_with` methods to deal with this. But it's not clear they are
    /// needed just yet. 80/20.
    pub fn defaults<const N: usize>() -> [Self; N] {
        from_fn(|x| {
            let x_u8 = x as u8;
            Self {
                static_slot_id: x_u8,
                flex_slot_id: x_u8,
                unused_1: 0,
                unused_2: 0,
                recorder_slot_id: 128 + x_u8,
            }
        })
    }
}

/// Values of an Audio Track's parameters page for the Static machine on the track (pitch/slice/len etc).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackMachineParamsValuesStatic {
    pub ptch: u8,
    pub strt: u8,
    pub len: u8,
    pub rate: u8,
    pub rtrg: u8,
    pub rtim: u8,
}

/// Values of an Audio Track's parameters page for the Flex machine on the track (pitch/slice/len etc).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackMachineParamsValuesFlex {
    pub ptch: u8,
    pub strt: u8,
    pub len: u8,
    pub rate: u8,
    pub rtrg: u8,
    pub rtim: u8,
}

/// Values of an Audio Track's parameters page for the Flex machine on the track (in/vol etc).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackMachineParamsValuesThru {
    pub in_ab: u8,
    pub vol_ab: u8,
    pub unused_1: u8,
    pub in_cd: u8,
    pub vol_cd: u8,
    pub unused_2: u8,
}

/// Values of an Audio Track's parameters page for the Neighbor machine on the track. Should not contain any data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackMachineParamsValuesNeighbor {
    unused_1: u8,
    unused_2: u8,
    unused_3: u8,
    unused_4: u8,
    unused_5: u8,
    unused_6: u8,
}

/// Values of an Audio Track's parameters page for the Pickup machine on the track. (pitch/dir/len etc).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
pub struct AudioTrackMachinesParamsValues {
    pub static_machine: AudioTrackMachineParamsValuesStatic,
    pub flex_machine: AudioTrackMachineParamsValuesFlex,
    pub thru_machine: AudioTrackMachineParamsValuesThru,
    pub neighbor_machine: AudioTrackMachineParamsValuesNeighbor,
    pub pickup_machine: AudioTrackMachineParamsValuesPickup,
}

// TODO: Set defaults
impl Default for AudioTrackMachinesParamsValues {
    fn default() -> Self {
        Self {
            static_machine: AudioTrackMachineParamsValuesStatic {
                ptch: 64,
                strt: 0,
                len: 0,
                rate: 127,
                rtrg: 0,
                rtim: 79,
            },
            flex_machine: AudioTrackMachineParamsValuesFlex {
                ptch: 64,
                strt: 0,
                len: 0,
                rate: 127,
                rtrg: 0,
                rtim: 79,
            },
            thru_machine: AudioTrackMachineParamsValuesThru {
                in_ab: 0,
                vol_ab: 64,
                unused_1: 0,
                in_cd: 0,
                vol_cd: 64,
                unused_2: 0,
            },
            neighbor_machine: AudioTrackMachineParamsValuesNeighbor {
                unused_1: 0,
                unused_2: 0,
                unused_3: 0,
                unused_4: 0,
                unused_5: 0,
                unused_6: 0,
            },
            pickup_machine: AudioTrackMachineParamsValuesPickup {
                ptch: 64,
                dir: 2,
                len: 1,
                unused_1: 127,
                gain: 64,
                op: 1,
            },
        }
    }
}

/// Values of a Track's main LFO parameters page.
/// Note that the speed and depth settings in the LFO Setup menu are looking at these values too.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct LfoParamsValues {
    pub spd1: u8,
    pub spd2: u8,
    pub spd3: u8,
    pub dep1: u8,
    pub dep2: u8,
    pub dep3: u8,
}

/// Values of a Track's main AMP parameters page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackAmpParamsValues {
    pub atk: u8,
    pub hold: u8,
    pub rel: u8,
    pub vol: u8,
    pub bal: u8,
    /// Reserved space for the \<F\> parameter used by Scenes and LFOs
    pub unused: u8,
}

// allow the verbose implementation to keep things
// - (a) standardised across all types
// - (b) easier for non-rustaceans to follow when reading through data structures
#[allow(clippy::derivable_impls)]
impl Default for AudioTrackAmpParamsValues {
    fn default() -> Self {
        Self {
            atk: 0,
            hold: 127,
            rel: 127,
            vol: 64,
            bal: 64,
            unused: 127,
        }
    }
}

/// Values of a Track's FX parameters page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackFxParamsValues {
    pub param_1: u8,
    pub param_2: u8,
    pub param_3: u8,
    pub param_4: u8,
    pub param_5: u8,
    pub param_6: u8,
}

/// Audio Tracks Paramater Page values. LFO, Amp, FX etc.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
pub struct AudioTrackParamsValues {
    pub lfo: LfoParamsValues,
    pub amp: AudioTrackAmpParamsValues,
    pub fx1: AudioTrackFxParamsValues,
    pub fx2: AudioTrackFxParamsValues,
}

impl Default for AudioTrackParamsValues {
    fn default() -> Self {
        Self {
            lfo: LfoParamsValues {
                spd1: 32,
                spd2: 32,
                spd3: 32,
                dep1: 0,
                dep2: 0,
                dep3: 0,
            },
            amp: AudioTrackAmpParamsValues::default(),
            fx1: AudioTrackFxParamsValues {
                param_1: 0,
                param_2: 127,
                param_3: 0,
                param_4: 64,
                param_5: 0,
                param_6: 64,
            },
            fx2: AudioTrackFxParamsValues {
                param_1: 47,
                param_2: 0,
                param_3: 127,
                param_4: 0,
                param_5: 127,
                param_6: 0,
            },
        }
    }
}

/// First set of values for a Track's LFO Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct LfoParamsSetup1 {
    pub lfo1_pmtr: u8,
    pub lfo2_pmtr: u8,
    pub lfo3_pmtr: u8,
    pub lfo1_wave: u8,
    pub lfo2_wave: u8,
    pub lfo3_wave: u8,
}

/// Second set of values for a Track's LFO Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct LfoParamsSetup2 {
    pub lfo1_mult: u8,
    pub lfo2_mult: u8,
    pub lfo3_mult: u8,
    pub lfo1_trig: u8,
    pub lfo2_trig: u8,
    pub lfo3_trig: u8,
}

/// Values for a Track's AMP Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackFxParamsSetup {
    pub setting1: u8,
    pub setting2: u8,
    pub setting3: u8,
    pub setting4: u8,
    pub setting5: u8,
    pub setting6: u8,
}

/// Audio Tracks Paramater Setup pages for LFO, Amp, FX etc.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
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

impl Default for AudioTrackParamsSetup {
    fn default() -> Self {
        Self {
            lfo_setup_1: LfoParamsSetup1 {
                lfo1_pmtr: 0,
                lfo2_pmtr: 0,
                lfo3_pmtr: 0,
                lfo1_wave: 0,
                lfo2_wave: 0,
                lfo3_wave: 0,
            },
            amp: AudioTrackAmpParamsSetup {
                amp: 1,
                sync: 1,
                atck: 0,
                fx1: 0,
                fx2: 0,
                unused: 0,
            },
            fx1: AudioTrackFxParamsSetup {
                setting1: 0,
                setting2: 0,
                setting3: 1,
                setting4: 0,
                setting5: 3,
                setting6: 0,
            },
            fx2: AudioTrackFxParamsSetup {
                setting1: 0,
                setting2: 1,
                setting3: 127,
                setting4: 1,
                setting5: 0,
                setting6: 0,
            },
            lfo_setup_2: LfoParamsSetup2 {
                lfo1_mult: 0,
                lfo2_mult: 0,
                lfo3_mult: 0,
                lfo1_trig: 0,
                lfo2_trig: 0,
                lfo3_trig: 0,
            },
        }
    }
}

/// Values for a MIDI Track's MIDI parameter page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct MidiTrackLfoParamsValues {
    pub spd1: u8,
    pub spd2: u8,
    pub spd3: u8,
    pub dep1: u8,
    pub dep2: u8,
    pub dep3: u8,
}

/// Values for a MIDI Track's Arp parameter page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct MidiTrackArpParamsValues {
    pub tran: u8,
    pub leg: u8,
    pub mode: u8,
    pub spd: u8,
    pub rnge: u8,
    pub nlen: u8,
}

/// Values for a MIDI Track's CC1 parameter page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct MidiTrackCc1ParamsValues {
    pub pb: u8,
    pub at: u8,
    pub cc1: u8,
    pub cc2: u8,
    pub cc3: u8,
    pub cc4: u8,
}

/// Values for a MIDI Track's CC2 parameter page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct MidiTrackCc2ParamsValues {
    pub cc5: u8,
    pub cc6: u8,
    pub cc7: u8,
    pub cc8: u8,
    pub cc9: u8,
    pub cc10: u8,
}

/// MIDI Tracks Paramater values. LFO, Amp, FX etc.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
pub struct MidiTrackParamsValues {
    pub midi: MidiTrackMidiParamsValues,
    pub arp: MidiTrackLfoParamsValues,
    pub lfo: MidiTrackArpParamsValues,
    pub ctrl1: MidiTrackCc1ParamsValues,
    pub ctrl2: MidiTrackCc2ParamsValues,
    #[serde(with = "BigArray")]
    pub unknown: [u8; 2],
}

impl Default for MidiTrackParamsValues {
    fn default() -> Self {
        Self {
            midi: MidiTrackMidiParamsValues {
                note: 48,
                vel: 100,
                len: 6,
                not2: 64,
                not3: 64,
                not4: 64,
            },
            arp: MidiTrackLfoParamsValues {
                spd1: 32,
                spd2: 32,
                spd3: 32,
                dep1: 0,
                dep2: 0,
                dep3: 0,
            },
            lfo: MidiTrackArpParamsValues {
                tran: 64,
                leg: 0,
                mode: 0,
                spd: 5,
                rnge: 0,
                nlen: 6,
            },
            ctrl1: MidiTrackCc1ParamsValues {
                pb: 64,
                at: 0,
                cc1: 127,
                cc2: 0,
                cc3: 0,
                cc4: 64,
            },
            ctrl2: MidiTrackCc2ParamsValues {
                cc5: 0,
                cc6: 0,
                cc7: 0,
                cc8: 0,
                cc9: 0,
                cc10: 0,
            },
            unknown: [0, 0],
        }
    }
}

/// Values for a MIDI Track's Note Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct MidiTrackNoteParamsSetup {
    pub chan: u8,
    pub bank: u8,
    pub prog: u8,
    unused_1: u8,
    pub sbank: u8,
    unused_2: u8,
}

/// Values for a MIDI Track's Arp Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct MidiTrackCc1ParamsSetup {
    unused_1: u8,
    unused_2: u8,
    pub cc1: u8,
    pub cc2: u8,
    pub cc3: u8,
    pub cc4: u8,
}

/// Values for a MIDI Track's CC2 Setup page.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct MidiTrackCc2ParamsSetup {
    pub cc5: u8,
    pub cc6: u8,
    pub cc7: u8,
    pub cc8: u8,
    pub cc9: u8,
    pub cc10: u8,
}

/// MIDI Tracks Paramater Setup pages for LFO, Amp, FX etc.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
pub struct MidiTrackParamsSetup {
    pub note: MidiTrackNoteParamsSetup,
    pub lfo1: LfoParamsSetup1,
    pub arp: MidiTrackArpParamsSetup,
    pub ctrl1: MidiTrackCc1ParamsSetup,
    pub ctrl2: MidiTrackCc2ParamsSetup,
    pub lfo2: LfoParamsSetup2,
}

impl Default for MidiTrackParamsSetup {
    fn default() -> Self {
        Self {
            note: MidiTrackNoteParamsSetup {
                chan: 0,
                bank: 128,
                prog: 128,
                unused_1: 0,
                sbank: 128,
                unused_2: 0,
            },
            lfo1: LfoParamsSetup1 {
                lfo1_pmtr: 0,
                lfo2_pmtr: 0,
                lfo3_pmtr: 0,
                lfo1_wave: 0,
                lfo2_wave: 0,
                lfo3_wave: 0,
            },
            arp: MidiTrackArpParamsSetup {
                unused_1: 0,
                unused_2: 0,
                len: 7,
                unused_3: 0,
                unused_4: 0,
                key: 0,
            },
            ctrl1: MidiTrackCc1ParamsSetup {
                unused_1: 0,
                unused_2: 0,
                cc1: 7,
                cc2: 1,
                cc3: 2,
                cc4: 10,
            },
            ctrl2: MidiTrackCc2ParamsSetup {
                cc5: 71,
                cc6: 72,
                cc7: 73,
                cc8: 74,
                cc9: 75,
                cc10: 76,
            },
            lfo2: LfoParamsSetup2 {
                lfo1_mult: 0,
                lfo2_mult: 0,
                lfo3_mult: 0,
                lfo1_trig: 0,
                lfo2_trig: 0,
                lfo3_trig: 0,
            },
        }
    }
}

/// A custom LFO Design -- array of 16 values.
/// 0 -> 127 values (above line) maps to 0 -> 127.
/// -1 -> -127 values (above line) seems to map to 255 -> 128.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, DefaultsAsArray)]
pub struct CustomLfoDesign(pub [u8; 16]);

/// Default is a 16 length array with zero values
impl Default for CustomLfoDesign {
    fn default() -> Self {
        Self(from_fn(|_| 0))
    }
}

/// LFO Interpolation mask.
/// Indicates which LFO steps should have values interpolated when LFO is triggered.
/// Not sure exactly how the calculation works yet.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, DefaultsAsArray)]
pub struct CustomLfoInterpolationMask(pub [u8; 2]);

impl Default for CustomLfoInterpolationMask {
    fn default() -> Self {
        Self(from_fn(|_| 0))
    }
}

/// First page of settings controlling an Audio Track's Recording Buffer configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct RecorderSetupProcessing {
    pub fin: u8,
    pub fout: u8,
    pub ab: u8,
    pub qrec: u8,
    pub qpl: u8,
    pub cd: u8,
}

/// Recorder Setup Pages
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
pub struct RecorderSetup {
    pub src: RecorderSetupSources,
    pub proc: RecorderSetupProcessing,
}

impl Default for RecorderSetup {
    fn default() -> Self {
        Self {
            src: RecorderSetupSources {
                in_ab: 1,
                in_cd: 1,
                rlen: 64,
                trig: 0,
                src3: 0,
                xloop: 1,
            },
            proc: RecorderSetupProcessing {
                fin: 0,
                fout: 0,
                ab: 0,
                qrec: 255,
                qpl: 255,
                cd: 0,
            },
        }
    }
}

// todo: merge with the Pattern Plock variant of this?
/// A scene's parameter assignments on the Playback/Machine page for an Audio Track.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackSceneLockPlayback {
    pub param1: u8,
    pub param2: u8,
    pub param3: u8,
    pub param4: u8,
    pub param5: u8,
    pub param6: u8,
}

/// Scene parameter assignments for a single audio track.
///
/// 255 is no assignment on the scene for that specific audio track's parameter
/// value.
///
/// Any other value is the Scene's assigned value for that control.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
pub struct AudioTrackScenesParamsAssignments {
    /// Scene assignments for the Audio Track's active playback machine
    pub machine: AudioTrackSceneLockPlayback,
    /// Scene assignments for the Audio Track's LFO parameters
    pub lfo: LfoParamsValues,
    /// Scene assignments for the Audio Track's AMP parameters
    pub amp: AudioTrackAmpParamsValues,
    /// Scene assignments for the Audio Track's FX1 parameters
    pub fx1: AudioTrackFxParamsValues,
    /// Scene assignments for the Audio Track's FX2 parameters
    pub fx2: AudioTrackFxParamsValues,
    /// Unknown, likely leftover sample slot assignment (it
    /// seems the underlying machine OS code re-uses the same
    /// data structure in several places).
    unknown_1: u8,
    /// Unknown, likely leftover sample slot assignment (it
    /// seems the underlying machine OS code re-uses the same
    /// data structure in several places).
    unknown_2: u8,
}

impl Default for AudioTrackScenesParamsAssignments {
    fn default() -> Self {
        Self {
            machine: AudioTrackSceneLockPlayback {
                param1: 255,
                param2: 255,
                param3: 255,
                param4: 255,
                param5: 255,
                param6: 255,
            },
            lfo: LfoParamsValues {
                spd1: 255,
                spd2: 255,
                spd3: 255,
                dep1: 255,
                dep2: 255,
                dep3: 255,
            },
            amp: AudioTrackAmpParamsValues {
                atk: 255,
                hold: 255,
                rel: 255,
                vol: 255,
                bal: 255,
                unused: 255,
            },
            fx1: AudioTrackFxParamsValues {
                param_1: 255,
                param_2: 255,
                param_3: 255,
                param_4: 255,
                param_5: 255,
                param_6: 255,
            },
            fx2: AudioTrackFxParamsValues {
                param_1: 255,
                param_2: 255,
                param_3: 255,
                param_4: 255,
                param_5: 255,
                param_6: 255,
            },
            unknown_1: 255,
            unknown_2: 255,
        }
    }
}

/// Parameters for a specified Scene can be configured across all 8 tracks
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, DefaultsAsArray)]
pub struct SceneParams(pub [AudioTrackScenesParamsAssignments; 8]);

impl Default for SceneParams {
    fn default() -> Self {
        Self(from_fn(|_| AudioTrackScenesParamsAssignments::default()))
    }
}

/// XLV Scene Parameter assignments for an Audio Track,
/// plus two other parameters i don't know yet
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsArray)]
pub struct SceneXlvAssignments {
    /// Main Volume fade control (XLV) for each Audio Track
    /// `MAX` -> `127`.
    /// `MIN` -> `0`.
    #[serde(with = "BigArray")]
    pub track_xlvs: [u8; 8],

    /// Don't know what this data block is for yet.
    /// Possibly leftover from using the same struct in the machine code
    /// as for audio track parameter values.
    #[serde(with = "BigArray")]
    unknown: [u8; 2],
}

impl Default for SceneXlvAssignments {
    fn default() -> Self {
        Self {
            track_xlvs: from_fn(|_| 255),
            unknown: from_fn(|_| 255),
        }
    }
}

/// A MIDI Track's custom Arp sequence.
/// `0` -> `+63` values (above line) maps to `0` -> `63`.
/// `-1` -> `-63` values (below line) map to `255` -> `192`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, DefaultsAsArray)]
pub struct MidiArpSequence(pub [u8; 16]);

impl Default for MidiArpSequence {
    fn default() -> Self {
        Self(from_fn(|_| 0))
    }
}

/// Parts in the bank, containing track data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Part {
    #[serde(with = "BigArray")]
    pub header: [u8; 4],

    /// All 0 values.
    #[serde(with = "BigArray")]
    pub data_block_1: [u8; 4],

    /// Zero-indexed part number
    pub part_id: u8,

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
    pub audio_track_params_values: [AudioTrackParamsValues; 8],

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

    /// Scene parameter assignments. There are 16 Scenes in a bank. Each scene
    /// can be used to modify parameter page values across any of the audio
    /// tracks.
    #[serde(with = "BigArray")]
    pub scenes: [SceneParams; 16],

    #[serde(with = "BigArray")]
    pub scene_xlvs: [SceneXlvAssignments; 16],

    /// Custom LFO designs for Audio Tracks.
    #[serde(with = "BigArray")]
    pub audio_tracks_custom_lfo_designs: [CustomLfoDesign; 8],

    /// Interpolation of steps in custom LFOs for Audio Tracks.
    /// Indicates which LFO steps should have values interpolated when LFO is triggered.
    /// Not sure exactly how the calculation works yet.
    #[serde(with = "BigArray")]
    pub audio_tracks_custom_lfos_interpolation_masks: [CustomLfoInterpolationMask; 8],

    /// Custom LFO designs for MIDI Tracks.
    #[serde(with = "BigArray")]
    pub midi_tracks_custom_lfos: [CustomLfoDesign; 8],

    /// Interpolation of steps in custom LFOs for MIDI Tracks.
    /// Indicates which LFO steps should have values interpolated when LFO is triggered.
    /// Not sure exactly how the calculation works yet.
    #[serde(with = "BigArray")]
    pub midi_tracks_custom_lfos_interpolation_masks: [CustomLfoInterpolationMask; 8],

    /// Arp Sequence Mutes for MIDI tracks.
    /// Not sure how these work.
    /// Muting some of the notes on M-TR-8 has the last array element set to 7 instead of 255.
    #[serde(with = "BigArray")]
    pub midi_tracks_arp_mute_masks: [u8; 16],

    /// Arp Sequence Notes for MIDI tracks.
    pub midi_tracks_arp_seqs: [MidiArpSequence; 8],
}

impl Part {
    /// WARNING: This `default` method is not from the `Default` trait, as we
    /// cannot use a default struct instance to create an array/vector of
    /// part data --> the individual default depends on their position
    /// in the final array!
    ///
    /// In the future, maybe it might be worth creating `default_with` and
    /// `defaults_with` methods to deal with this. But it's not clear they are
    /// needed just yet. 80/20.
    fn default(part_id: u8) -> Self {
        // TODO: create an ot-tools error
        assert!(part_id < 4);
        Self {
            header: PART_HEADER,
            data_block_1: from_fn(|_| 0),
            part_id,
            audio_track_fx1: from_fn(|_| 4),
            audio_track_fx2: from_fn(|_| 8),
            active_scenes: ActiveScenes::default(),
            audio_track_volumes: AudioTrackVolume::defaults(),
            audio_track_machine_types: from_fn(|_| 0),
            audio_track_machine_params: AudioTrackMachinesParamsValues::defaults(),
            audio_track_params_values: AudioTrackParamsValues::defaults(),
            audio_track_machine_setup: AudioTrackMachinesParamsSetup::defaults(),
            audio_track_machine_slots: AudioTrackMachineSlot::defaults(),
            audio_track_params_setup: AudioTrackParamsSetup::defaults(),
            midi_track_params_values: MidiTrackParamsValues::defaults(),
            midi_track_params_setup: MidiTrackParamsSetup::defaults(),
            recorder_setup: RecorderSetup::defaults(),
            scenes: SceneParams::defaults(),
            scene_xlvs: SceneXlvAssignments::defaults(),
            audio_tracks_custom_lfo_designs: CustomLfoDesign::defaults(),
            audio_tracks_custom_lfos_interpolation_masks: CustomLfoInterpolationMask::defaults(),
            midi_tracks_custom_lfos: CustomLfoDesign::defaults(),
            midi_tracks_custom_lfos_interpolation_masks: CustomLfoInterpolationMask::defaults(),
            midi_tracks_arp_mute_masks: from_fn(|_| 255),
            midi_tracks_arp_seqs: MidiArpSequence::defaults(),
        }
    }

    /// WARNING: This `defaults` method is not from the `Defaults` trait, as we
    /// cannot use a default struct instance to create an array/vector of
    /// machine slots data --> the individual default depends on their position
    /// in the final array!
    ///
    /// In the future, maybe it might be worth creating `default_with` and
    /// `defaults_with` methods to deal with this. But it's not clear they are
    /// needed just yet. 80/20.
    pub fn defaults<const N: usize>() -> Box<Array<Self, N>> {
        Array(from_fn(|x| Self::default(x as u8))).into()
    }
}

impl CheckHeader for Part {
    fn check_header(&self) -> bool {
        self.header == PART_HEADER
    }
}

/// Contains the two different types of Part data
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Parts {
    /// Unsaved Part data for a Bank.
    ///
    /// Part state prior to saving a Part via the Part menu (but after SYNC TO CARD).
    pub unsaved: Box<Array<Part, 4>>,
    /// Saved Part data for a Bank.
    ///
    /// Part state once the Part has been saved via the Part menu is stored here.
    pub saved: Box<Array<Part, 4>>,
}

impl CheckHeader for Parts {
    fn check_header(&self) -> bool {
        self.unsaved.iter().all(|x| x.check_header())
    }
}

impl Default for Parts {
    fn default() -> Self {
        Self {
            unsaved: Part::defaults(),
            saved: Part::defaults(),
        }
    }
}

#[cfg(test)]
mod test {
    mod integrity_check {
        mod part {}

        mod parts {}
    }
}
