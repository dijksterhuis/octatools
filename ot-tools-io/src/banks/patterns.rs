//! Serialization and Deserialization of Pattern related data for Bank files.

use crate::{
    banks::parts::{
        AudioTrackAmpParamsValues, AudioTrackFxParamsValues, LfoParamsValues,
        MidiTrackArpParamsValues, MidiTrackCc1ParamsValues, MidiTrackCc2ParamsValues,
        MidiTrackLfoParamsValues, MidiTrackMidiParamsValues,
    },
    CheckHeader, DefaultsArrayBoxed, OptionEnumValueConvert, SerdeOctatrackErrors,
};
use ot_tools_derive::DefaultsAsBoxedBigArray;
use std::array::from_fn;

use crate::RBoxErr;
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

const HALF_PAGE_TRIG_BITMASK_VALUES: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const PATTERN_HEADER: [u8; 8] = [0x50, 0x54, 0x52, 0x4e, 0x00, 0x00, 0x00, 0x00];

/// Header array for a MIDI track section in binary data files: `MTRA`
const MIDI_TRACK_HEADER: [u8; 4] = [0x4d, 0x54, 0x52, 0x41];

/// Header array for a MIDI track section in binary data files: `TRAC`
const AUDIO_TRACK_HEADER: [u8; 4] = [0x54, 0x52, 0x41, 0x43];

/// Given a half-page trig bit mask, get an array of 8x boolean values
/// indicating whether each trig in the half-page is active or not
pub fn get_halfpage_trigs_from_bitmask_value(bitmask: &u8) -> RBoxErr<[bool; 8]> {
    let arr: [bool; 8] = HALF_PAGE_TRIG_BITMASK_VALUES
        .iter()
        .map(|x| (bitmask & x) > 0)
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();
    Ok(arr)
}

/// Given a half-page trig bit mask, get an array of 8x boolean values
/// indicating where each trig in the half-page is active or not
pub fn get_track_trigs_from_bitmasks(bitmasks: &[u8; 8]) -> RBoxErr<[bool; 64]> {
    let trigs: [bool; 64] = bitmasks
        .iter()
        .flat_map(|x: &u8| get_halfpage_trigs_from_bitmask_value(x).unwrap())
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();

    Ok(trigs)
}

/// A Trig's parameter locks on the Playback/Machine page for an Audio Track.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct AudioTrackParameterLockPlayback {
    pub param1: u8,
    pub param2: u8,
    pub param3: u8,
    pub param4: u8,
    pub param5: u8,
    pub param6: u8,
}

impl Default for AudioTrackParameterLocks {
    fn default() -> Self {
        // 255 -> disabled

        // NOTE: the `part.rs` `default` methods for each of these type has
        // fields all set to the correct defaults for the TRACK view, not p-lock
        // trigS. So don't try and use the type's `default` method here as you
        // will end up with a bunch of p-locks on trigs for all the default
        // values. (Although maybe that's a desired feature for some workflows).

        // Yes, this comment is duplicated below. It is to make sur you've seen
        // it.
        Self {
            machine: AudioTrackParameterLockPlayback {
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
            static_slot_id: 255,
            flex_slot_id: 255,
        }
    }
}

/// A single trig's parameter locks on an Audio Track.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsBoxedBigArray)]
pub struct AudioTrackParameterLocks {
    pub machine: AudioTrackParameterLockPlayback,
    pub lfo: LfoParamsValues,
    pub amp: AudioTrackAmpParamsValues,
    pub fx1: AudioTrackFxParamsValues,
    pub fx2: AudioTrackFxParamsValues,
    /// P-Lock to change an audio track's static machine sample slot assignment per trig
    pub static_slot_id: u8,
    /// P-Lock to change an audio track's flex machine sample slot assignment per trig
    pub flex_slot_id: u8,
}

/// MIDI Track parameter locks.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, DefaultsAsBoxedBigArray)]
pub struct MidiTrackParameterLocks {
    pub midi: MidiTrackMidiParamsValues,
    pub lfo: MidiTrackLfoParamsValues,
    pub arp: MidiTrackArpParamsValues,
    pub ctrl1: MidiTrackCc1ParamsValues,
    pub ctrl2: MidiTrackCc2ParamsValues,

    #[serde(with = "BigArray")]
    unknown: [u8; 2],
}

impl Default for MidiTrackParameterLocks {
    fn default() -> Self {
        // 255 -> disabled

        // NOTE: the `part.rs` `default` methods for each of these type has
        // fields all set to the correct defaults for the TRACK view, not p-lock
        // trigS. So don't try and use the type's `default` method here as you
        // will end up with a bunch of p-locks on trigs for all the default
        // values. (Although maybe that's a desired feature for some workflows).

        // Yes, this comment is duplicated above. It is to make sur you've seen
        // it.

        Self {
            midi: MidiTrackMidiParamsValues {
                note: 255,
                vel: 255,
                len: 255,
                not2: 255,
                not3: 255,
                not4: 255,
            },
            lfo: MidiTrackLfoParamsValues {
                spd1: 255,
                spd2: 255,
                spd3: 255,
                dep1: 255,
                dep2: 255,
                dep3: 255,
            },
            arp: MidiTrackArpParamsValues {
                tran: 255,
                leg: 255,
                mode: 255,
                spd: 255,
                rnge: 255,
                nlen: 255,
            },
            ctrl1: MidiTrackCc1ParamsValues {
                pb: 255,
                at: 255,
                cc1: 255,
                cc2: 255,
                cc3: 255,
                cc4: 255,
            },
            ctrl2: MidiTrackCc2ParamsValues {
                cc5: 255,
                cc6: 255,
                cc7: 255,
                cc8: 255,
                cc9: 255,
                cc10: 255,
            },
            unknown: [255, 255],
        }
    }
}

/// Audio & MIDI Track Pattern playback settings.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct TrackPatternSettings {
    /// Silence any existing audio playback on the Audio Track when switching Patterns.
    pub start_silent: u8,

    /// Trigger Audio Track playback without any quantization or syncing to other Audio Tracks.
    pub plays_free: u8,

    /// Quantization when this Audio Track is Triggered for Playback.
    ///
    /// Options
    /// ```text
    /// N/A and ONE: 0 (Default)
    /// ONE2: 1
    /// HOLD: 2
    /// ```
    pub trig_mode: u8,

    /// Track Trigger Quantization.
    ///
    /// Options
    /// ```text
    /// N/A and TR.LEN: 0 (Default)
    /// 1/16: 1
    /// 2/16: 2
    /// 3/16: 3
    /// 4/16: 4
    /// 6/16: 5
    /// 8/16: 6
    /// 12/16: 7
    /// 16/16: 8
    /// 24/16: 9
    /// 32/16: 10
    /// 48/16: 11
    /// 64/16: 12
    /// 96/16: 13
    /// 128/16: 14
    /// 192/16: 15
    /// 256/16: 16
    /// DIRECT: 255
    /// ```
    pub trig_quant: u8,

    /// Whether to play the track as a `ONESHOT` track.
    pub oneshot_trk: u8,
}

impl Default for TrackPatternSettings {
    fn default() -> Self {
        Self {
            start_silent: 255,
            plays_free: 0,
            trig_mode: 0,
            trig_quant: 0,
            oneshot_trk: 0,
        }
    }
}

/// Trig bitmasks array for Audio Tracks.
/// Can be converted into an array of booleans using the `get_track_trigs_from_bitmasks` function.
///
/// Trig bitmask arrays have bitmasks stored in this order, which is slightly confusing (pay attention to the difference with 7 + 8!):
/// 1. 1st half of the 4th page
/// 2. 2nd half of the 4th page
/// 3. 1st half of the 3rd page
/// 4. 2nd half of the 3rd page
/// 5. 1st half of the 2nd page
/// 6. 2nd half of the 2nd page
/// 7. 2nd half of the 1st page
/// 8. 1st half of the 1st page
///
/// ### Bitmask values for trig positions
/// With single trigs in a half-page
/// ```text
/// positions
/// 1 2 3 4 5 6 7 8 | mask value
/// ----------------|-----------
/// - - - - - - - - | 0
/// x - - - - - - - | 1
/// - x - - - - - - | 2
/// - - x - - - - - | 4
/// - - - x - - - - | 8
/// - - - - x - - - | 16
/// - - - - - x - - | 32
/// - - - - - - x - | 64
/// - - - - - - - x | 128
/// ```
///
/// When there are multiple trigs in a half-page, the individual position values are summed together:
///
/// ```text
/// 1 2 3 4 5 6 7 8 | mask value
/// ----------------|-----------
/// x x - - - - - - | 1 + 2 = 3
/// x x x x - - - - | 1 + 2 + 4 + 8 = 15
/// ```
/// ### Fuller diagram of mask values
///
/// ```text
/// positions
/// 1 2 3 4 5 6 7 8 | mask value
/// ----------------|-----------
/// x - - - - - - - | 1
/// - x - - - - - - | 2
/// x x - - - - - - | 3
/// - - x - - - - - | 4
/// x - x - - - - - | 5
/// - x x - - - - - | 6
/// x x x - - - - - | 7
/// - - - x - - - - | 8
/// x - - x - - - - | 9
/// - x - x - - - - | 10
/// x x - x - - - - | 11
/// - - x x - - - - | 12
/// x - x x - - - - | 13
/// - x x x - - - - | 14
/// x x x x - - - - | 15
/// ................|....
/// x x x x x x - - | 63
/// ................|....
/// - - - - - - - x | 128
/// ................|....
/// - x - x - x - x | 170
/// ................|....
/// - - - - x x x x | 240
/// ................|....
/// x x x x x x x x | 255
/// ```
///
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackTrigMasks {
    /// Trigger Trig masks -- indicate which Trigger Trigs are active.
    /// Base track Trig masks are stored backwards, meaning
    /// the first 8 Trig positions are the last bytes in this section.
    #[serde(with = "BigArray")]
    pub trigger: [u8; 8],

    /// Envelope Trig masks -- indicate which Envelope Trigs are active.
    /// See the description of the `trig_trig_masks` field for an
    /// explanation of how the masking works.
    #[serde(with = "BigArray")]
    pub trigless: [u8; 8],

    /// Parameter-Lock Trig masks -- indicate which Parameter-Lock Trigs are active.
    /// See the description of the `trig_trig_masks` field for an
    /// explanation of how the masking works.    
    #[serde(with = "BigArray")]
    pub plock: [u8; 8],

    /// Hold Trig masks -- indicate which Hold Trigs are active.
    /// See the description of the `trig_trig_masks` field for an
    /// explanation of how the masking works.
    #[serde(with = "BigArray")]
    pub oneshot: [u8; 8],

    /// Recorder Trig masks -- indicate which Recorder Trigs are active.
    /// These seem to function differently to the main Track Trig masks.
    /// Filling up Recorder Trigs on a Pattern results in a 32 length array
    /// instead of 8 length.
    /// Possible that the Trig type is stored in this array as well.
    #[serde(with = "BigArray")]
    pub recorder: [u8; 32],

    /// Swing trigs Trig masks.
    #[serde(with = "BigArray")]
    pub swing: [u8; 8],

    /// Parameter Slide trigs Trig masks.
    #[serde(with = "BigArray")]
    pub slide: [u8; 8],
}

impl Default for AudioTrackTrigMasks {
    fn default() -> Self {
        Self {
            trigger: from_fn(|_| 0),
            trigless: from_fn(|_| 0),
            plock: from_fn(|_| 0),
            oneshot: from_fn(|_| 0),
            recorder: from_fn(|_| 0),
            swing: from_fn(|_| 170),
            slide: from_fn(|_| 0),
        }
    }
}

/// Audio Track custom scaling when the Pattern is in PER TRACK scale mode.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct TrackPerTrackModeScale {
    /// The Audio Track's Length when Pattern is in Per Track mode.
    /// Default: 16
    pub per_track_len: u8,

    /// The Audio Track's Scale when Pattern is in Per Track mode.
    ///
    /// Options
    /// ```text
    /// 0 -> 2x
    /// 1 -> 3/2x
    /// 2 -> 1x (Default)
    /// 3 -> 3/4x
    /// 4 -> 1/2x
    /// 5 -> 1/4x
    /// 6 -> 1/8x
    /// ```
    pub per_track_scale: u8,
}

impl Default for TrackPerTrackModeScale {
    fn default() -> Self {
        Self {
            per_track_len: 16,
            per_track_scale: 2,
        }
    }
}

/// Sample Slot options for Projects.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum TrigCondition {
    None,
    /// > FILL is true (will activate the trig) when fill mode is active (see below).
    Fill,
    /// > ... true when FILL is not.
    NotFill,
    /// > PRE is true if the most recently evaluated trig condition on the same track was true.
    Pre,
    /// > ... true when PRE is not.
    NotPre,
    /// > true if the most recently evaluated trig condition on the neighbor track was true.
    /// > The neighbor track is the track before the one being edited.
    /// > For example, the neighbor track of track 4 is track 3. If no conditions exist on the
    /// > neighbor track, the condition is false.
    Nei,
    /// > ... true when NEI is not.
    NotNei,
    /// > only true the first time the pattern play (when looped).
    First,
    /// > ... true when 1st is not.
    NotFirst,
    /// > probability condition. 1% chance of being true.
    Percent1,
    /// > probability condition. 2% chance of being true.
    Percent2,
    /// > probability condition. 4% chance of being true.
    Percent4,
    /// > probability condition. 6% chance of being true.
    Percent6,
    /// > probability condition. 9% chance of being true.
    Percent9,
    /// > probability condition. 13% chance of being true.
    Percent13,
    /// > probability condition. 19% chance of being true.
    Percent19,
    /// > probability condition. 25% chance of being true.
    Percent25,
    /// > probability condition. 33% chance of being true.
    Percent33,
    /// > probability condition. 41% chance of being true.
    Percent41,
    /// > probability condition. 50% chance of being true.
    Percent50,
    /// > probability condition. 59% chance of being true.
    Percent59,
    /// > probability condition. 67% chance of being true.
    Percent67,
    /// > probability condition. 75% chance of being true.
    Percent75,
    /// > probability condition. 81% chance of being true.
    Percent81,
    /// > probability condition. 87% chance of being true.
    Percent87,
    /// > probability condition. 91% chance of being true.
    Percent91,
    /// > probability condition. 94% chance of being true.
    Percent94,
    /// > probability condition. 96% chance of being true.
    Percent96,
    /// > probability condition. 98% chance of being true.
    Percent98,
    /// > probability condition. 99% chance of being true.
    Percent99,
    /// pattern loop 1 triggers, pattern loop 2 resets
    PatternT1R2,
    /// pattern loop 2 triggers, pattern loop 2 resets
    PatternT2R2,
    /// pattern loop 1 triggers, pattern loop 3 resets
    PatternT1R3,
    /// pattern loop 2 triggers, pattern loop 3 resets
    PatternT2R3,
    /// pattern loop 3 triggers, pattern loop 3 resets
    PatternT3R3,
    /// pattern loop 1 triggers, pattern loop 4 resets
    PatternT1R4,
    /// pattern loop 2 triggers, pattern loop 4 resets
    PatternT2R4,
    /// pattern loop 3 triggers, pattern loop 4 resets
    PatternT3R4,
    /// pattern loop 4 triggers, pattern loop 4 resets
    PatternT4R4,
    /// pattern loop 1 triggers, pattern loop 5 resets
    PatternT1R5,
    /// pattern loop 2 triggers, pattern loop 5 resets
    PatternT2R5,
    /// pattern loop 3 triggers, pattern loop 5 resets
    PatternT3R5,
    /// pattern loop 4 triggers, pattern loop 5 resets
    PatternT4R5,
    /// pattern loop 5 triggers, pattern loop 5 resets
    PatternT5R5,
    /// pattern loop 1 triggers, pattern loop 6 resets
    PatternT1R6,
    /// pattern loop 2 triggers, pattern loop 6 resets
    PatternT2R6,
    /// pattern loop 3 triggers, pattern loop 6 resets
    PatternT3R6,
    /// pattern loop 4 triggers, pattern loop 6 resets
    PatternT4R6,
    /// pattern loop 5 triggers, pattern loop 6 resets
    PatternT5R6,
    /// pattern loop 6 triggers, pattern loop 6 resets
    PatternT6R6,
    /// pattern loop 1 triggers, pattern loop 7 resets
    PatternT1R7,
    /// pattern loop 2 triggers, pattern loop 7 resets
    PatternT2R7,
    /// pattern loop 3 triggers, pattern loop 7 resets
    PatternT3R7,
    /// pattern loop 4 triggers, pattern loop 7 resets
    PatternT4R7,
    /// pattern loop 5 triggers, pattern loop 7 resets
    PatternT5R7,
    /// pattern loop 6 triggers, pattern loop 7 resets
    PatternT6R7,
    /// pattern loop 7 triggers, pattern loop 7 resets
    PatternT7R7,
    /// pattern loop 1 triggers, pattern loop 8 resets
    PatternT1R8,
    /// pattern loop 2 triggers, pattern loop 8 resets
    PatternT2R8,
    /// pattern loop 3 triggers, pattern loop 8 resets
    PatternT3R8,
    /// pattern loop 4 triggers, pattern loop 8 resets
    PatternT4R8,
    /// pattern loop 5 triggers, pattern loop 8 resets
    PatternT5R8,
    /// pattern loop 6 triggers, pattern loop 8 resets
    PatternT6R8,
    /// pattern loop 7 triggers, pattern loop 8 resets
    PatternT7R8,
    /// pattern loop 8 triggers, pattern loop 8 resets
    PatternT8R8,
}

impl OptionEnumValueConvert for TrigCondition {
    type T = TrigCondition;
    type V = u8;

    fn from_value(v: &Self::V) -> RBoxErr<Self::T> {
        // read the essay for `AudioTrackTrigs.trig_timings_repeats_conditions`
        // to understand why rem_euclid is used here
        match v.rem_euclid(128) {
            0 => Ok(TrigCondition::None),
            1 => Ok(TrigCondition::Fill),
            2 => Ok(TrigCondition::NotFill),
            3 => Ok(TrigCondition::Pre),
            4 => Ok(TrigCondition::NotPre),
            5 => Ok(TrigCondition::Nei),
            6 => Ok(TrigCondition::NotNei),
            7 => Ok(TrigCondition::First),
            8 => Ok(TrigCondition::NotFirst),
            //
            9 => Ok(TrigCondition::Percent1),
            10 => Ok(TrigCondition::Percent2),
            11 => Ok(TrigCondition::Percent4),
            12 => Ok(TrigCondition::Percent6),
            13 => Ok(TrigCondition::Percent9),
            14 => Ok(TrigCondition::Percent13),
            15 => Ok(TrigCondition::Percent19),
            16 => Ok(TrigCondition::Percent25),
            17 => Ok(TrigCondition::Percent33),
            18 => Ok(TrigCondition::Percent41),
            19 => Ok(TrigCondition::Percent50),
            20 => Ok(TrigCondition::Percent59),
            21 => Ok(TrigCondition::Percent67),
            22 => Ok(TrigCondition::Percent75),
            23 => Ok(TrigCondition::Percent81),
            24 => Ok(TrigCondition::Percent87),
            25 => Ok(TrigCondition::Percent91),
            26 => Ok(TrigCondition::Percent94),
            27 => Ok(TrigCondition::Percent96),
            28 => Ok(TrigCondition::Percent98),
            29 => Ok(TrigCondition::Percent99),
            //
            30 => Ok(TrigCondition::PatternT1R2),
            31 => Ok(TrigCondition::PatternT2R2),
            //
            32 => Ok(TrigCondition::PatternT1R3),
            33 => Ok(TrigCondition::PatternT2R3),
            34 => Ok(TrigCondition::PatternT3R3),
            //
            35 => Ok(TrigCondition::PatternT1R4),
            36 => Ok(TrigCondition::PatternT2R4),
            37 => Ok(TrigCondition::PatternT3R4),
            38 => Ok(TrigCondition::PatternT4R4),
            //
            39 => Ok(TrigCondition::PatternT1R5),
            40 => Ok(TrigCondition::PatternT2R5),
            41 => Ok(TrigCondition::PatternT3R5),
            42 => Ok(TrigCondition::PatternT4R5),
            43 => Ok(TrigCondition::PatternT5R5),
            //
            44 => Ok(TrigCondition::PatternT1R6),
            45 => Ok(TrigCondition::PatternT2R6),
            46 => Ok(TrigCondition::PatternT3R6),
            47 => Ok(TrigCondition::PatternT4R6),
            48 => Ok(TrigCondition::PatternT5R6),
            49 => Ok(TrigCondition::PatternT6R6),
            //
            50 => Ok(TrigCondition::PatternT1R7),
            51 => Ok(TrigCondition::PatternT2R7),
            52 => Ok(TrigCondition::PatternT3R7),
            53 => Ok(TrigCondition::PatternT4R7),
            54 => Ok(TrigCondition::PatternT5R7),
            55 => Ok(TrigCondition::PatternT6R7),
            56 => Ok(TrigCondition::PatternT7R7),
            //
            57 => Ok(TrigCondition::PatternT1R8),
            58 => Ok(TrigCondition::PatternT2R8),
            59 => Ok(TrigCondition::PatternT3R8),
            60 => Ok(TrigCondition::PatternT4R8),
            61 => Ok(TrigCondition::PatternT5R8),
            62 => Ok(TrigCondition::PatternT6R8),
            63 => Ok(TrigCondition::PatternT7R8),
            64 => Ok(TrigCondition::PatternT8R8),
            //
            _ => Err(SerdeOctatrackErrors::NoMatchingOptionEnumValue.into()),
        }
    }

    fn value(&self) -> RBoxErr<Self::V> {
        match self {
            Self::None => Ok(0),
            Self::Fill => Ok(1),
            Self::NotFill => Ok(2),
            Self::Pre => Ok(3),
            Self::NotPre => Ok(4),
            Self::Nei => Ok(5),
            Self::NotNei => Ok(6),
            Self::First => Ok(7),
            Self::NotFirst => Ok(8),
            Self::Percent1 => Ok(9),
            Self::Percent2 => Ok(10),
            Self::Percent4 => Ok(11),
            Self::Percent6 => Ok(12),
            Self::Percent9 => Ok(13),
            Self::Percent13 => Ok(14),
            Self::Percent19 => Ok(15),
            Self::Percent25 => Ok(16),
            Self::Percent33 => Ok(17),
            Self::Percent41 => Ok(18),
            Self::Percent50 => Ok(19),
            Self::Percent59 => Ok(20),
            Self::Percent67 => Ok(21),
            Self::Percent75 => Ok(22),
            Self::Percent81 => Ok(23),
            Self::Percent87 => Ok(24),
            Self::Percent91 => Ok(25),
            Self::Percent94 => Ok(26),
            Self::Percent96 => Ok(27),
            Self::Percent98 => Ok(28),
            Self::Percent99 => Ok(29),
            Self::PatternT1R2 => Ok(30),
            Self::PatternT2R2 => Ok(31),
            Self::PatternT1R3 => Ok(32),
            Self::PatternT2R3 => Ok(33),
            Self::PatternT3R3 => Ok(34),
            Self::PatternT1R4 => Ok(35),
            Self::PatternT2R4 => Ok(36),
            Self::PatternT3R4 => Ok(37),
            Self::PatternT4R4 => Ok(38),
            Self::PatternT1R5 => Ok(39),
            Self::PatternT2R5 => Ok(40),
            Self::PatternT3R5 => Ok(41),
            Self::PatternT4R5 => Ok(42),
            Self::PatternT5R5 => Ok(43),
            Self::PatternT1R6 => Ok(44),
            Self::PatternT2R6 => Ok(45),
            Self::PatternT3R6 => Ok(46),
            Self::PatternT4R6 => Ok(47),
            Self::PatternT5R6 => Ok(48),
            Self::PatternT6R6 => Ok(49),
            Self::PatternT1R7 => Ok(50),
            Self::PatternT2R7 => Ok(51),
            Self::PatternT3R7 => Ok(52),
            Self::PatternT4R7 => Ok(53),
            Self::PatternT5R7 => Ok(54),
            Self::PatternT6R7 => Ok(55),
            Self::PatternT7R7 => Ok(56),
            Self::PatternT1R8 => Ok(57),
            Self::PatternT2R8 => Ok(58),
            Self::PatternT3R8 => Ok(59),
            Self::PatternT4R8 => Ok(60),
            Self::PatternT5R8 => Ok(61),
            Self::PatternT6R8 => Ok(62),
            Self::PatternT7R8 => Ok(63),
            Self::PatternT8R8 => Ok(64),
        }
    }
}

/// Track trigs assigned on an Audio Track within a Pattern
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct AudioTrackTrigs {
    /// Header data section
    ///
    /// example data:
    /// ```text
    /// TRAC
    /// 54 52 41 43
    /// ```
    #[serde(with = "BigArray")]
    pub header: [u8; 4],

    /// Unknown data.
    #[serde(with = "BigArray")]
    pub unknown_1: [u8; 4],

    /// The zero indexed track number
    pub track_id: u8,

    /// Trig masks contain the Trig step locations for different trig types
    pub trig_masks: AudioTrackTrigMasks,

    /// The scale of this Audio Track in Per Track Pattern mode.
    pub scale_per_track_mode: TrackPerTrackModeScale,

    /// Amount of swing when a Swing Trig is active for the Track.
    /// Maximum is `30` (`80` on device), minimum is `0` (`50` on device).
    pub swing_amount: u8,

    /// Pattern settings for this Audio Track
    pub pattern_settings: TrackPatternSettings,

    /// Unknown data.
    pub unknown_2: u8,

    /// Parameter-Lock data for all Trigs.
    // note -- stack overflow if tring to use #[serde(with = "BigArray")]
    pub plocks: Box<Array<AudioTrackParameterLocks, 64>>,

    /// What the hell is this field?!?!
    /// It **has to** be something to do with trigs, but i have no idea what it could be.
    #[serde(with = "BigArray")]
    pub unknown_3: [u8; 64],

    /// Trig Offsets, Trig Counts and Trig Conditions
    /// ====
    /// This is ..... slightly frustrating.
    ///
    /// This 64 length array consisting of a pair of bytes for each array element hold three
    /// data references... Trig Cunts and Trig Conditions use the two bytes independently,
    /// so they're easier to explain first
    ///
    /// Trig Counts and Trig Conditions
    /// ====
    ///
    /// Trig Counts and Trig Conditions data is interleaved for each trig.
    /// For Trig position 1, array index 0 is the count value and array index 1 is the Trig
    /// Condition.
    ///
    /// For trig counts (1st byte), the value (zero-indexed) is multiplied by 32.
    /// - 8 trig counts (7 repeats) --> 7 * 3 = 224
    /// - 4 trig counts (3 repeats) -- 3 * 32 = 96
    /// - 1 trig counts (0 repeats) -- 0 * 32 = 0
    ///
    /// For conditionals, see the `TrigCondition` enum and associated traits for more details.
    /// The maximum value for a Trig Condition byte is 64.
    ///
    /// ```rust
    /// // no trig micro-timings at all
    /// [
    ///     // trig 1
    ///     [
    ///         0,   // trig counts (number)
    ///         0,   // trig condition (enum rep)
    ///     ],
    ///     // trig 2
    ///     [
    ///         224, // trig counts (max value)
    ///         64,  // trig condition (max value)
    ///     ],
    ///     // trig 3
    ///     [
    ///         32,  // trig counts (minimum non-zero value)
    ///         1,   // trig condition (minimum non-zero value)
    ///     ],
    ///     // ... and so on
    /// ];
    /// ```
    ///
    /// Trig Offsets
    /// ====
    ///
    /// Trig Offset values use both of these interleaved bytes on top of the
    /// trig repeat and trig condition values... Which makes life more complex
    /// and somewhat frustrating.
    ///
    /// Inspected values
    /// - -23/384 -> 1st byte 20, 2nd byte 128
    /// - -1/32 -> 1st byte 26, 2nd byte 0
    /// - -1/64 -> 1st byte 29, 2nd byte 0
    /// - -1/128 -> 1st byte 30, 2nd byte 128
    /// - 1/128 -> 1st byte 1, 2nd byte 128
    /// - 1/64 -> 1st byte 3, 2nd byte 0
    /// - 1/32 -> 1st byte 6, 2nd byte 0
    /// - 23/384 -> 1st byte 11, 2nd byte 128
    ///
    /// #### 1st byte
    /// The 1st byte only has 31 possible values: 255 - 224 (trig count max) = 31.
    /// So it makes sense sort of that this is a mask? I guess?
    ///
    /// #### 2nd byte
    /// From what I can tell, the second offset byte is either 0 or 128.
    /// So a 2nd byte for an offset adjusted trig with a `8:8` trig condition is either
    /// - 128 + 64 = 192
    /// - 0 + 64 = 64
    ///
    /// So you will need to a `x.rem_euclid(128)` somewhere if you want to parse this.
    ///
    /// Combining the trig offset with trig count and trig conditions, we end up with
    /// ```rust
    /// [
    ///     // trig one, -23/384 offset with 1x trig count and None condition
    ///     [
    ///         20,  // 20 + (32 * 0)
    ///         128, // 128 + 0
    ///     ],
    ///     // trig two, -23/384 offset with 2x trig count and Fill condition
    ///     [
    ///         52,  // 20 + (32 * 1)
    ///         129, // 128 + 1
    ///     ],
    ///     // trig three, -23/384 offset with 3x trig count and Fill condition
    ///     [
    ///         84,  // 20 + (32 * 2)
    ///         129, // 128 + 1
    ///     ],
    ///     // trig four, -23/384 offset with 3x trig count and NotFill condition
    ///     [
    ///         84,  // 20 + (32 * 2)
    ///         130, // 128 + 2
    ///     ],
    ///     // trig five, +1/32 offset with 2x trig count and Fill condition
    ///     [
    ///         38,  // 6 + (32 * 1)
    ///         1,   // 0 + 1
    ///     ],
    ///     // trig six, +1/32 offset with 3x trig count and Fill condition
    ///     [
    ///         70,  // 6 + (32 * 2)
    ///         1,   // 0 + 1
    ///     ],
    ///     // trig seven, +1/32 offset with 3x trig count and NotFill condition
    ///     [
    ///         70,  // 6 + (32 * 2)
    ///         2,   // 0 + 2
    ///     ],
    ///     // .... and so on
    /// ];
    /// ```
    ///
    /// #### Extending pages and offsets
    ///
    /// If you have a trig offset on Trig 1 with only one pattern page activated,
    /// the trig offsets for Trig 1 are replicated over the relevant trig
    /// positions for each first trig in the inactive pages in this array.
    ///
    /// So, for a 1/32 offset on trig 1 with only one page active, you get the
    /// following values showing up in this array:
    /// - pair of bytes at array index 15 -> 1/32
    /// - pair of bytes at array index 31 -> 1/32
    /// - pair of bytes at array index 47 -> 1/32
    ///
    /// This does not happen for offset values at any other trig position
    /// (from what I can tell in my limited testing -- trig values 2-4 and 9-11
    /// inclusive are not replicated in the same way).
    ///
    /// This 'replicating trig offset values over unused pages' behaviour does
    /// not happen for trig counts. I haven't tested whether this applies to trig
    /// conditions yet.
    ///
    /// It seems that this behaviour could be to make sure the octatack plays
    /// correctly offset trigs when you extend a page live, i.e. when extending
    /// a one-page pattern to a two-page pattern, if there is a negative offset
    /// value there the octatrack will need to play the offset trig before the
    /// first page has completed.
    ///
    /// Or it could be a bug :shrug:
    #[serde(with = "BigArray")]
    pub trig_offsets_repeats_conditions: [[u8; 2]; 64],
}

impl AudioTrackTrigs {
    /// WARNING: This `default` method is not from the `Default` trait, as we
    /// cannot use a default struct instance to create an array/vector of
    /// midi track trig data --> the individual default depends on their
    /// position in the final array!
    ///
    /// In the future, maybe it might be worth creating `default_with` and
    /// `defaults_with` methods to deal with this. But it's not clear they are
    /// needed just yet. 80/20.
    fn default(id: u8) -> Self {
        assert!(id < 8);
        Self {
            header: AUDIO_TRACK_HEADER,
            unknown_1: from_fn(|_| 0),
            track_id: id,
            trig_masks: AudioTrackTrigMasks::default(),
            scale_per_track_mode: TrackPerTrackModeScale::default(),
            swing_amount: 0,
            pattern_settings: TrackPatternSettings::default(),
            unknown_2: 0,
            plocks: AudioTrackParameterLocks::defaults(),
            unknown_3: from_fn(|_| 0),
            trig_offsets_repeats_conditions: from_fn(|_| [0, 0]),
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
    pub fn defaults<const N: usize>() -> [Self; N] {
        from_fn(|x| Self::default(x as u8))
    }
}

impl CheckHeader for AudioTrackTrigs {
    fn check_header(&self) -> bool {
        self.header == AUDIO_TRACK_HEADER
    }
}

/// MIDI Track Trig masks.
/// Can be converted into an array of booleans using the `get_track_trigs_from_bitmasks` function.
/// See `AudioTrackTrigMasks` for more information.
///
/// Trig mask arrays have data stored in this order, which is slightly confusing (pay attention to the difference with 7 + 8!):
/// 1. 1st half of the 4th page
/// 2. 2nd half of the 4th page
/// 3. 1st half of the 3rd page
/// 4. 2nd half of the 3rd page
/// 5. 1st half of the 2nd page
/// 6. 2nd half of the 2nd page
/// 7. 2nd half of the 1st page
/// 8. 1st half of the 1st page
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct MidiTrackTrigMasks {
    /// Note Trig masks.
    #[serde(with = "BigArray")]
    pub trigger: [u8; 8],

    /// Trigless Trig masks.
    #[serde(with = "BigArray")]
    pub trigless: [u8; 8],

    /// Parameter Lock Trig masks.
    /// Note this only stores data for exclusive parameter lock *trigs* (light green trigs).
    #[serde(with = "BigArray")]
    pub plock: [u8; 8],

    /// Swing trigs mask.
    #[serde(with = "BigArray")]
    pub swing: [u8; 8],

    /// this is a block of 8, so looks like a trig mask for tracks,
    /// but I can't think of what it could be.
    #[serde(with = "BigArray")]
    pub unknown: [u8; 8],
}

impl Default for MidiTrackTrigMasks {
    fn default() -> Self {
        Self {
            trigger: from_fn(|_| 0),
            trigless: from_fn(|_| 0),
            plock: from_fn(|_| 0),
            swing: from_fn(|_| 170),
            unknown: from_fn(|_| 0),
        }
    }
}

/// Track trigs assigned on an Audio Track within a Pattern
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackTrigs {
    /// Header data section
    ///
    /// example data:
    /// ```text
    /// MTRA
    /// 4d 54 52 41
    /// ```
    #[serde(with = "BigArray")]
    pub header: [u8; 4],

    /// Unknown data.
    #[serde(with = "BigArray")]
    pub unknown_1: [u8; 4],

    /// The zero indexed track number
    pub track_id: u8,

    /// MIDI Track Trig masks contain the Trig step locations for different trig types
    pub trig_masks: MidiTrackTrigMasks,

    /// The scale of this MIDI Track in Per Track Pattern mode.
    pub scale_per_track_mode: TrackPerTrackModeScale,

    /// Amount of swing when a Swing Trig is active for the Track.
    /// Maximum is `30` (`80` on device), minimum is `0` (`50` on device).
    pub swing_amount: u8,

    /// Pattern settings for this MIDI Track
    pub pattern_settings: TrackPatternSettings,

    /// trig properties -- p-locks etc.
    /// the big `0xff` value block within tracks basically.
    /// 32 bytes per trig -- 6x parameters for 5x pages plus 2x extra fields at the end.
    ///
    /// For audio tracks, the 2x extra fields at the end are for sample locks,
    /// but there's no such concept for MIDI tracks.
    /// It seems like Elektron devs reused their data structures for P-Locks on both Audio + MIDI tracks.
    // note -- stack overflow if tring to use #[serde(with = "BigArray")]
    pub plocks: Box<Array<MidiTrackParameterLocks, 64>>,

    /// See the documentation for `AudioTrackTrigs` on how this field works.
    #[serde(with = "BigArray")]
    pub trig_offsets_repeats_conditions: [[u8; 2]; 64],
}

impl MidiTrackTrigs {
    /// WARNING: This `default` method is not from the `Default` trait, as we
    /// cannot use a default struct instance to create an array/vector of
    /// midi track trig data --> the individual default depends on their
    /// position in the final array!
    ///
    /// In the future, maybe it might be worth creating `default_with` and
    /// `defaults_with` methods to deal with this. But it's not clear they are
    /// needed just yet. 80/20.
    fn default(id: u8) -> Self {
        // TODO: create an ot-tools error
        assert!(id < 8);
        Self {
            header: MIDI_TRACK_HEADER,
            unknown_1: from_fn(|_| 0),
            track_id: id,
            trig_masks: MidiTrackTrigMasks::default(),
            scale_per_track_mode: TrackPerTrackModeScale::default(),
            swing_amount: 0,
            pattern_settings: TrackPatternSettings::default(),
            plocks: MidiTrackParameterLocks::defaults(),
            trig_offsets_repeats_conditions: from_fn(|_| [0, 0]),
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
    pub fn defaults<const N: usize>() -> [Self; N] {
        from_fn(|x| Self::default(x as u8))
    }
}

impl CheckHeader for MidiTrackTrigs {
    fn check_header(&self) -> bool {
        self.header == MIDI_TRACK_HEADER
    }
}

/// Pattern level scaling settings.
/// Some of these settings still apply when the pattern is in Per-Track scaling mode.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PatternScaleSettings {
    /// Multiply this value by `master_len_per_track` to get
    /// the real Master Length in Per Track Pattern mode.
    ///
    /// This field must be set to `255` when Master Length in
    /// Per Track Pattern mode is set to `INF`.
    ///
    /// ```text
    /// 0: From 2 steps to 255 steps.
    /// 1: From 256 steps to 511 steps.
    /// 2: From 512 steps to 767 steps.
    /// 3: From 768 steps to 1023 steps.
    /// 4: 1024 steps only.
    /// 255: `INF`.
    /// ```
    pub master_len_per_track_multiplier: u8,

    /// Master Length in Per Track Pattern mode.
    /// Must multiply this by multiplier like this `(x + 1) * (mult + 1)` to get the real number.
    ///
    /// This field must be set to `255` when Master Length in
    /// Per Track Pattern mode is set to `INF`.
    ///
    /// Minimum value is 2 when the multiplier equals 0.
    pub master_len_per_track: u8,

    /// The Audio Track's Scale when Pattern is in Per Track mode.
    ///
    /// Options
    /// ```text
    /// 0 -> 2x
    /// 1 -> 3/2x
    /// 2 -> 1x (Default)
    /// 3 -> 3/4x
    /// 4 -> 1/2x
    /// 5 -> 1/4x
    /// 6 -> 1/8x
    /// ```
    pub master_scale_per_track: u8,

    /// Master Pattern Length.
    /// Determines Pattern Length for all Tracks when NOT in Per Track mode.
    pub master_len: u8,

    /// Master Pattern playback multiplier.
    ///
    /// Options
    /// ```text
    /// 0 -> 2x
    /// 1 -> 3/2x
    /// 2 -> 1x (Default)
    /// 3 -> 3/4x
    /// 4 -> 1/2x
    /// 5 -> 1/4x
    /// 6 -> 1/8x
    /// ```
    pub master_scale: u8,

    /// Scale mode for the Pattern.
    ///
    /// Options
    /// ```text
    /// NORMAL: 0 (Default)
    /// PER TRACK: 1
    /// ```
    pub scale_mode: u8,
}

impl Default for PatternScaleSettings {
    fn default() -> Self {
        Self {
            master_len_per_track_multiplier: 0,
            master_len_per_track: 16,
            master_scale_per_track: 2,
            master_len: 16,
            master_scale: 2,
            scale_mode: 0,
        }
    }
}

/// Chaining behaviour for the pattern.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PatternChainBehavior {
    /// When `use_project_setting` field is set to `1`/`true`
    /// this field should be set to `N/A` with a value of `255`.
    pub use_pattern_setting: u8,

    /// Pattern Chain Behaviour -- Use the Project level setting for chain
    /// behaviour and disable any Pattern level chaining behaviour.
    /// Numeric Boolean.
    /// When this is `1` the `use_pattern_setting` should be set to `255`.
    pub use_project_setting: u8,
}

// allow the verbose implementation to keep things
// - (a) standardised across all types
// - (b) easier for non-rustaceans to follow when reading through data structures
#[allow(clippy::derivable_impls)]
impl Default for PatternChainBehavior {
    fn default() -> Self {
        Self {
            use_pattern_setting: 0,
            use_project_setting: 0,
        }
    }
}

/// A pattern of trigs stored in the bank.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, DefaultsAsBoxedBigArray)]
pub struct Pattern {
    /// Header indicating start of pattern section
    ///
    /// example data:
    /// ```text
    /// PTRN....
    /// 50 54 52 4e 00 00 00 00
    /// ```
    #[serde(with = "BigArray")]
    pub header: [u8; 8],

    /// Audio Track data
    #[serde(with = "BigArray")]
    pub audio_track_trigs: [AudioTrackTrigs; 8],

    /// MIDI Track data
    #[serde(with = "BigArray")]
    pub midi_track_trigs: [MidiTrackTrigs; 8],

    /// Pattern scaling controls and settings
    pub scale: PatternScaleSettings,

    /// Pattern chaining behaviour and settings
    pub chain_behaviour: PatternChainBehavior,

    /// Unknown data.
    pub unknown: u8,

    /// The Part of a Bank assigned to a Pattern.
    /// Part 1 = 0; Part 2 = 1; Part 3 = 2; Part 4 = 3.
    /// Credit to [@sezare56 on elektronauts for catching this one](https://www.elektronauts.com/t/octalib-a-simple-octatrack-librarian/225192/27)
    pub part_assignment: u8,

    /// Pattern setting for Tempo.
    ///
    /// The Tempo value is split across both `tempo_1` and `tempo_2`.
    /// Yet to figure out how they relate to each other.
    ///
    /// Value of 120 BPM is 11 for this field.
    /// Value of 30 BPM is 2 for this field.
    pub tempo_1: u8,

    /// Pattern setting for Tempo.
    ///
    /// The Tempo value is split across both `tempo_1` and `tempo_2`.
    /// Tet to figure out how they relate to each other.
    ///
    /// Value of 120 BPM is `64` for this field.
    /// Value of 30 BPM is `208` for this field.
    pub tempo_2: u8,
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            header: PATTERN_HEADER,
            audio_track_trigs: AudioTrackTrigs::defaults(),
            midi_track_trigs: MidiTrackTrigs::defaults(),
            scale: PatternScaleSettings::default(),
            chain_behaviour: PatternChainBehavior::default(),
            unknown: 0,
            part_assignment: 0,
            // **I believe** these two mask values make the tempo 120.0 BPM
            // don't quote me on that though
            tempo_1: 11,
            tempo_2: 64,
        }
    }
}

impl CheckHeader for Pattern {
    fn check_header(&self) -> bool {
        self.header == PATTERN_HEADER
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {

    mod track_trig_defaults {

        mod audio {
            use crate::banks::patterns::AudioTrackTrigs;
            use crate::RBoxErr;

            fn wrap_err(track_id: u8) -> RBoxErr<AudioTrackTrigs> {
                Ok(AudioTrackTrigs::default(track_id))
            }

            #[test]
            fn ok_track_id_zero() {
                assert!(wrap_err(0).is_ok());
            }

            #[test]
            fn ok_track_id_seven() {
                assert!(wrap_err(7).is_ok());
            }

            // todo: proper error handling (don't use an assert!() in the default method)
            #[test]
            #[should_panic]
            fn err_default_track_id_eight() {
                assert!(wrap_err(8).is_err());
            }
        }
        mod midi {
            use crate::banks::patterns::MidiTrackTrigs;
            use crate::RBoxErr;

            fn wrap_err(track_id: u8) -> RBoxErr<MidiTrackTrigs> {
                Ok(MidiTrackTrigs::default(track_id))
            }

            #[test]
            fn ok_track_id_zero() {
                assert!(wrap_err(0).is_ok());
            }

            #[test]
            fn ok_track_id_seven() {
                assert!(wrap_err(7).is_ok());
            }

            // todo: proper error handling (don't use an assert!() in the default method)
            #[test]
            #[should_panic]
            fn err_default_track_id_eight() {
                assert!(wrap_err(8).is_err());
            }
        }
    }
    mod trig_bitmasks {

        #[test]
        fn test_all_track_trigs_from_bitmasks() {
            let x: [u8; 8] = std::array::from_fn(|_| 255);
            let y: [bool; 64] = std::array::from_fn(|_| true);

            assert_eq!(
                crate::banks::patterns::get_track_trigs_from_bitmasks(&x).unwrap(),
                y
            )
        }

        #[test]
        fn test_no_track_trigs_from_bitmasks() {
            let x: [u8; 8] = std::array::from_fn(|_| 0);
            let y: [bool; 64] = std::array::from_fn(|_| false);

            assert_eq!(
                crate::banks::patterns::get_track_trigs_from_bitmasks(&x).unwrap(),
                y
            )
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_0() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&0).unwrap(),
                [false, false, false, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_1() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&1).unwrap(),
                [true, false, false, false, false, false, false, false],
            );
        }
        #[test]
        fn test_halfpage_trig_bitmask_unmask_2() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&2).unwrap(),
                [false, true, false, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_4() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&4).unwrap(),
                [false, false, true, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_8() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&8).unwrap(),
                [false, false, false, true, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_16() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&16).unwrap(),
                [false, false, false, false, true, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_32() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&32).unwrap(),
                [false, false, false, false, false, true, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_64() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&64).unwrap(),
                [false, false, false, false, false, false, true, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_128() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&128).unwrap(),
                [false, false, false, false, false, false, false, true],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_3() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&3).unwrap(),
                [true, true, false, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_7() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&7).unwrap(),
                [true, true, true, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_15() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&15).unwrap(),
                [true, true, true, true, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_31() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&31).unwrap(),
                [true, true, true, true, true, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_63() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&63).unwrap(),
                [true, true, true, true, true, true, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_127() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&127).unwrap(),
                [true, true, true, true, true, true, true, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_255() {
            assert_eq!(
                crate::banks::patterns::get_halfpage_trigs_from_bitmask_value(&255).unwrap(),
                [true, true, true, true, true, true, true, true],
            );
        }
    }

    mod integrity {
        mod pattern {
            // valid header: [0x50, 0x54, 0x52, 0x4e, 0x00, 0x00, 0x00, 0x00];
            use crate::banks::patterns::Pattern;
            use crate::CheckHeader;

            #[test]
            fn true_valid_header() {
                let pattern = Pattern::default();
                assert!(pattern.check_header());
            }

            #[test]
            fn false_invalid_header() {
                let mut pattern = Pattern::default();
                pattern.header[0] = 0x01;
                pattern.header[1] = 0x01;
                pattern.header[7] = 0x50;
                assert!(!pattern.check_header());
            }
        }
        mod audio_track_trigs {
            use crate::banks::patterns::AudioTrackTrigs;
            use crate::CheckHeader;

            #[test]
            fn true_valid_header() {
                let trigs = AudioTrackTrigs::default(0);
                assert!(trigs.check_header());
            }

            #[test]
            fn false_invalid_header() {
                let mut trigs = AudioTrackTrigs::default(0);
                trigs.header[0] = 0x01;
                trigs.header[1] = 0x01;
                trigs.header[2] = 0x50;
                assert!(!trigs.check_header());
            }
        }
        mod midi_track_trigs {
            use crate::banks::patterns::MidiTrackTrigs;
            use crate::CheckHeader;

            #[test]
            fn true_valid_header() {
                let trigs = MidiTrackTrigs::default(0);
                assert!(trigs.check_header());
            }

            #[test]
            fn false_invalid_header() {
                let mut trigs = MidiTrackTrigs::default(0);
                trigs.header[0] = 0x01;
                trigs.header[1] = 0x01;
                trigs.header[2] = 0x50;
                assert!(!trigs.check_header());
            }
        }
    }
}
