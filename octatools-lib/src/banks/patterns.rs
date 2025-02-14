//! Serialization and Deserialization of Pattern related data for Bank files.

use crate::{
    banks::parts::{
        AudioTrackAmpParamsValues, AudioTrackFxParamsValues, LfoParamsValues,
        MidiTrackArpParamsValues, MidiTrackCc1ParamsValues, MidiTrackCc2ParamsValues,
        MidiTrackLfoParamsValues, MidiTrackMidiParamsValues,
    },
    projects::options::ProjectSampleSlotType,
    DefaultsArray, DefaultsArrayBoxed,
};

use octatools_derive::{DefaultsAsArray, DefaultsAsBoxedBigArray};

use crate::RBoxErr;
use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

const HALF_PAGE_TRIG_BITMASK_VALUES: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const PATTERN_HEADER: [u8; 8] = [0x50, 0x54, 0x52, 0x4e, 0x00, 0x00, 0x00, 0x00];

/// Header array for a MIDI track section in binary data files: `MTRA`
const MIDI_TRACK_HEADER: [u8; 4] = [0x4d, 0x54, 0x52, 0x41];

/// Header array for a MIDI track section in binary data files: `TRAC`
const AUDIO_TRACK_HEADER: [u8; 4] = [54, 0x52, 0x41, 0x43];

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
            trigger: [0, 0, 0, 0, 0, 0, 0, 0],
            trigless: [0, 0, 0, 0, 0, 0, 0, 0],
            plock: [0, 0, 0, 0, 0, 0, 0, 0],
            oneshot: [0, 0, 0, 0, 0, 0, 0, 0],
            recorder: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            swing: [170, 170, 170, 170, 170, 170, 170, 170],
            slide: [0, 0, 0, 0, 0, 0, 0, 0],
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

/// Track trigs assigned on an Audio Track within a Pattern
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, DefaultsAsArray)]
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
    pub unknown_1: [u8; 5],

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

    /// Unknown data.
    /// comes at the end, dunno what this block is yet
    /// mostly a bunch of zero values
    #[serde(with = "BigArray")]
    pub unknown_3: [u8; 192],
}

impl Default for AudioTrackTrigs {
    fn default() -> Self {
        Self {
            header: AUDIO_TRACK_HEADER,
            unknown_1: [0, 0, 0, 0, 0],
            trig_masks: AudioTrackTrigMasks::default(),
            scale_per_track_mode: TrackPerTrackModeScale::default(),
            swing_amount: 0,
            pattern_settings: TrackPatternSettings::default(),
            unknown_2: 0,
            plocks: AudioTrackParameterLocks::defaults(),
            unknown_3: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        }
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
            trigger: [0, 0, 0, 0, 0, 0, 0, 0],
            trigless: [0, 0, 0, 0, 0, 0, 0, 0],
            plock: [0, 0, 0, 0, 0, 0, 0, 0],
            swing: [170, 170, 170, 170, 170, 170, 170, 170],
            unknown: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}

/// Track trigs assigned on an Audio Track within a Pattern
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, DefaultsAsArray)]
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

    /// this is something to do with the trig masks but i don't know what it's referring to
    /// * when not active1 trigs on a track: 7
    /// * when all trigs on a track are trigger trigs: 7
    /// * when all trigs on a track are trigless trigs: 6
    /// * when all trigs on a track are plock trigs: 5
    pub unknown_2: u8,

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

    /// Unknown data.
    /// comes at the end, dunno what this block is yet
    /// mostly a bunch of zero values
    // note -- stack overflow if tring to use #[serde(with = "BigArray")]
    pub unknown_3: Box<Array<u8, 128>>,
}

impl Default for MidiTrackTrigs {
    fn default() -> Self {
        Self {
            header: MIDI_TRACK_HEADER,
            unknown_1: [0, 0, 0, 0],
            unknown_2: 0,
            trig_masks: MidiTrackTrigMasks::default(),
            scale_per_track_mode: TrackPerTrackModeScale::default(),
            swing_amount: 0,
            pattern_settings: TrackPatternSettings::default(),
            plocks: MidiTrackParameterLocks::defaults(),
            unknown_3: Box::new(Array([0; 128])),
        }
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

impl Pattern {
    pub fn update_plock_sample_slots(
        &mut self,
        sample_type: &ProjectSampleSlotType,
        old: &u8,
        new: &u8,
    ) -> RBoxErr<()> {
        for audio_track_trigs in self.audio_track_trigs.iter_mut() {
            for plock in audio_track_trigs.plocks.iter_mut() {
                match sample_type {
                    ProjectSampleSlotType::Static => {
                        if plock.static_slot_id == *old {
                            plock.static_slot_id = *new;
                        }
                    }
                    ProjectSampleSlotType::Flex => {
                        if plock.flex_slot_id == *old {
                            plock.flex_slot_id = *new;
                        }
                    }
                    ProjectSampleSlotType::RecorderBuffer => {}
                }
            }
        }
        Ok(())
    }
    pub fn update_flex_sample_plocks(&mut self, old: &u8, new: &u8) -> RBoxErr<()> {
        for audio_track_trigs in self.audio_track_trigs.iter_mut() {
            for plock in audio_track_trigs.plocks.iter_mut() {
                if plock.flex_slot_id == *old {
                    plock.flex_slot_id = *new;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
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
}
