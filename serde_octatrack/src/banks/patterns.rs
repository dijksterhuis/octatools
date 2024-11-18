//! Serialization and Deserialization of Pattern related data for Bank files.

use crate::banks::parts::{
    AudioTrackAmpParamsValues, AudioTrackFxParamsValues, LfoParamsValues, MidiTrackArpParamsValues,
    MidiTrackCc1ParamsValues, MidiTrackCc2ParamsValues, MidiTrackLfoParamsValues,
    MidiTrackMidiParamsValues,
};

use serde::{Deserialize, Serialize};
use serde_big_array::{Array, BigArray};

use crate::RBoxErr;

const HALF_PAGE_TRIG_BITMASK_VALUES: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

/// Given a half-page trig bit mask, get a arrary of 8x boolean values
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

/// Given a half-page trig bit mask, get a arrary of 8x boolean values
/// indicating where each trig in the half-page is active or not
pub fn get_track_trigs_from_bitmasks(bitmasks: &[u8; 8]) -> RBoxErr<[bool; 64]> {
    let trigs: [bool; 64] = bitmasks
        .iter()
        .map(|x: &u8| get_halfpage_trigs_from_bitmask_value(x).unwrap())
        .flatten()
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();

    Ok(trigs)
}

/// A Trig's parameter locks on the Playback/Machine page for an Audio Track.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackParameterLockPlayback {
    pub param1: u8,
    pub param2: u8,
    pub param3: u8,
    pub param4: u8,
    pub param5: u8,
    pub param6: u8,
}

/// A single trig's parameter locks on an Audio Track.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AudioTrackParameterLocks {
    pub machine: AudioTrackParameterLockPlayback,
    pub lfo: LfoParamsValues,
    pub amp: AudioTrackAmpParamsValues,
    pub fx1: AudioTrackFxParamsValues,
    pub fx2: AudioTrackFxParamsValues,
    pub sample_lock_static: u8,
    pub sample_lock_flex: u8,
}

/// MIDI Track parameter locks.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MidiTrackParameterLocks {
    pub midi: MidiTrackMidiParamsValues,
    pub lfo: MidiTrackLfoParamsValues,
    pub arp: MidiTrackArpParamsValues,
    pub ctrl1: MidiTrackCc1ParamsValues,
    pub ctrl2: MidiTrackCc2ParamsValues,

    #[serde(with = "BigArray")]
    unknown: [u8; 2],
}

/// Audio & MIDI Track Pattern playback settings.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    /// TODO -- better docs.
    pub oneshot_trk: u8,
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

/// Audio Track custom scaling when the Pattern is in PER TRACK scale mode.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    /// Minimum value is 2 when when the multiplier equals 0.
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

/// A pattern of trigs stored in the bank.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
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
    #[serde(with = "BigArray")]
    pub unknown: [u8; 2],

    /// Pattern setting for Tempo.
    ///
    /// The Tempo value is split across both `tempo_1` and `tempo_2`.
    /// As of yet to figure out how they relate to each other.
    ///
    /// Value of 120 BPM is 11 for this field.
    /// Value of 30 BPM is 2 for this field.
    pub tempo_1: u8,

    /// Pattern setting for Tempo.
    ///
    /// The Tempo value is split across both `tempo_1` and `tempo_2`.
    /// As of yet to figure out how they relate to each other.
    ///
    /// Value of 120 BPM is `64` for this field.
    /// Value of 30 BPM is `208` for this field.
    pub tempo_2: u8,
}

impl Pattern {
    pub fn update_static_sample_plocks(&mut self, old: &u8, new: &u8) -> RBoxErr<()> {
        for (_, audio_track_trigs) in self.audio_track_trigs.iter_mut().enumerate() {
            for (_, plock) in audio_track_trigs.plocks.iter_mut().enumerate() {
                if plock.sample_lock_static == *old {
                    plock.sample_lock_static = *new;
                }
            }
        }
        Ok(())
    }
    pub fn update_flex_sample_plocks(&mut self, old: &u8, new: &u8) -> RBoxErr<()> {
        for (_, audio_track_trigs) in self.audio_track_trigs.iter_mut().enumerate() {
            for (_, plock) in audio_track_trigs.plocks.iter_mut().enumerate() {
                if plock.sample_lock_flex == *old {
                    plock.sample_lock_flex = *new;
                }
            }
        }
        Ok(())
    }
}

mod test {
    mod trig_bitmasks {
        use crate::banks::patterns::get_halfpage_trigs_from_bitmask_value;

        #[test]
        fn test_halfpage_trig_bitmask_unmask_0() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&0).unwrap(),
                [false, false, false, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_1() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&1).unwrap(),
                [true, false, false, false, false, false, false, false],
            );
        }
        #[test]
        fn test_halfpage_trig_bitmask_unmask_2() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&2).unwrap(),
                [false, true, false, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_4() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&4).unwrap(),
                [false, false, true, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_8() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&8).unwrap(),
                [false, false, false, true, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_16() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&16).unwrap(),
                [false, false, false, false, true, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_32() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&32).unwrap(),
                [false, false, false, false, false, true, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_64() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&64).unwrap(),
                [false, false, false, false, false, false, true, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_128() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&128).unwrap(),
                [false, false, false, false, false, false, false, true],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_3() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&3).unwrap(),
                [true, true, false, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_7() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&7).unwrap(),
                [true, true, true, false, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_15() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&15).unwrap(),
                [true, true, true, true, false, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_31() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&31).unwrap(),
                [true, true, true, true, true, false, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_63() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&63).unwrap(),
                [true, true, true, true, true, true, false, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_127() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&127).unwrap(),
                [true, true, true, true, true, true, true, false],
            );
        }

        #[test]
        fn test_halfpage_trig_bitmask_unmask_255() {
            assert_eq!(
                get_halfpage_trigs_from_bitmask_value(&255).unwrap(),
                [true, true, true, true, true, true, true, true],
            );
        }
    }
}
