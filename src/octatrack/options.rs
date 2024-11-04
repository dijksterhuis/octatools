//! Enums for various Octatrack settings.

// TODO: Break this up into options modules in the projects / samples directories.

use crate::common::RVoidError;

use crate::octatrack::common::OptionEnumValueConvert;
use serde::{Deserialize, Serialize};

/// Sample Slot options for Projects.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ProjectSampleSlotType {
    /// Static machine slot
    Static,

    /// Flex machine slot
    Flex,

    /// Recorder buffer slot
    RecorderBuffer,
}

impl OptionEnumValueConvert for ProjectSampleSlotType {
    type T = ProjectSampleSlotType;
    type V = String;

    fn from_value(v: Self::V) -> RVoidError<Self::T> {
        match v.to_ascii_uppercase().as_str() {
            "STATIC" => Ok(ProjectSampleSlotType::Static),
            "FLEX" => Ok(ProjectSampleSlotType::Flex),
            "RECORDER" => Ok(ProjectSampleSlotType::RecorderBuffer),
            _ => Err(()),
        }
    }

    fn value(&self) -> RVoidError<Self::V> {
        match self {
            ProjectSampleSlotType::Static => Ok("STATIC".to_string()),
            ProjectSampleSlotType::Flex => Ok("FLEX".to_string()),
            ProjectSampleSlotType::RecorderBuffer => Ok("RECORDER".to_string()),
        }
    }
}

/// Sample attributes Timestrech options.
/// See Octatrack Manaul section 13.2.4 ATTRIBUTES

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize, Copy)]
pub enum SampleAttributeTimestrechMode {
    /// No timestreching applied.
    #[default]
    Off,

    /// Regular timestreching.
    Normal,

    /// Drum / Rythmic specific algorithm
    Beat,
}

impl OptionEnumValueConvert for SampleAttributeTimestrechMode {
    type T = SampleAttributeTimestrechMode;
    type V = u32;

    fn from_value(v: Self::V) -> RVoidError<Self::T> {
        match v {
            0 => Ok(SampleAttributeTimestrechMode::Off),
            2 => Ok(SampleAttributeTimestrechMode::Normal),
            3 => Ok(SampleAttributeTimestrechMode::Beat),
            _ => Err(()),
        }
    }

    fn value(&self) -> RVoidError<Self::V> {
        match self {
            SampleAttributeTimestrechMode::Off => Ok(0),
            SampleAttributeTimestrechMode::Normal => Ok(2),
            SampleAttributeTimestrechMode::Beat => Ok(3),
        }
    }
}

/// Sample attributes Loop mode options.
/// See Octatrack Manaul section 13.2.4 ATTRIBUTES

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize, Copy)]
pub enum SampleAttributeLoopMode {
    /// Loop points are ignored and sample will never loop.
    #[default]
    Off,

    /// Loop by starting again at the loop start position once playback of the sample reaches loop end.
    Normal,

    /// Loop by continuously reversing once playback of the sample reaches loop end/loop start.
    PingPong,
}

impl OptionEnumValueConvert for SampleAttributeLoopMode {
    type T = SampleAttributeLoopMode;
    type V = u32;

    fn from_value(v: Self::V) -> RVoidError<Self::T> {
        match v {
            0 => Ok(SampleAttributeLoopMode::Off),
            1 => Ok(SampleAttributeLoopMode::Normal),
            2 => Ok(SampleAttributeLoopMode::PingPong),
            _ => Err(()),
        }
    }

    fn value(&self) -> RVoidError<Self::V> {
        match self {
            SampleAttributeLoopMode::Off => Ok(0),
            SampleAttributeLoopMode::Normal => Ok(1),
            SampleAttributeLoopMode::PingPong => Ok(2),
        }
    }
}

/// Sample attributes Trig Quantization options
/// (quantization when manually triggering samples via track buttons).
/// See Octatrack Manaul section 13.2.4 ATTRIBUTES

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize, Copy)]
pub enum SampleAttributeTrigQuantizationMode {
    /// Play back immediately, no quantization.
    #[default]
    Direct,

    /// Play once the pattern ends
    PatternLength,

    /// Play after 1 sequencer step(s).
    OneStep,

    /// Play after 2 sequencer step(s).
    TwoSteps,

    /// Play after 3 sequencer step(s).
    ThreeSteps,

    /// Play after 4 sequencer step(s).
    FourSteps,

    /// Play after 6 sequencer step(s).
    SixSteps,

    /// Play after 8 sequencer step(s).
    EightSteps,

    /// Play after 12 sequencer step(s).
    TwelveSteps,

    /// Play after 16 sequencer step(s).
    SixteenSteps,

    /// Play after 24 sequencer step(s).
    TwentyFourSteps,

    /// Play after 32 sequencer step(s).
    ThirtyTwoSteps,

    /// Play after 48 sequencer step(s).
    FourtyEightSteps,

    /// Play after 64 sequencer step(s).
    SixtyFourSteps,

    /// Play after 96 sequencer step(s).
    NinetySixSteps,

    /// Play after 128 sequencer step(s).
    OneTwentyEightSteps,

    /// Play after 192 sequencer step(s).
    OneNinetyTwoSteps,

    /// Play after 256 sequencer step(s).
    TwoFiveSixSteps,
}

impl OptionEnumValueConvert for SampleAttributeTrigQuantizationMode {
    type T = SampleAttributeTrigQuantizationMode;
    type V = u32;

    fn from_value(v: Self::V) -> RVoidError<Self::T> {
        match v {
            0 => Ok(SampleAttributeTrigQuantizationMode::Direct),
            255 => Ok(SampleAttributeTrigQuantizationMode::PatternLength),
            1 => Ok(SampleAttributeTrigQuantizationMode::OneStep),
            2 => Ok(SampleAttributeTrigQuantizationMode::TwoSteps),
            3 => Ok(SampleAttributeTrigQuantizationMode::ThreeSteps),
            4 => Ok(SampleAttributeTrigQuantizationMode::FourSteps),
            5 => Ok(SampleAttributeTrigQuantizationMode::SixSteps),
            6 => Ok(SampleAttributeTrigQuantizationMode::EightSteps),
            7 => Ok(SampleAttributeTrigQuantizationMode::TwelveSteps),
            8 => Ok(SampleAttributeTrigQuantizationMode::SixteenSteps),
            9 => Ok(SampleAttributeTrigQuantizationMode::TwentyFourSteps),
            10 => Ok(SampleAttributeTrigQuantizationMode::ThirtyTwoSteps),
            11 => Ok(SampleAttributeTrigQuantizationMode::FourtyEightSteps),
            12 => Ok(SampleAttributeTrigQuantizationMode::SixtyFourSteps),
            13 => Ok(SampleAttributeTrigQuantizationMode::NinetySixSteps),
            14 => Ok(SampleAttributeTrigQuantizationMode::OneTwentyEightSteps),
            15 => Ok(SampleAttributeTrigQuantizationMode::OneNinetyTwoSteps),
            16 => Ok(SampleAttributeTrigQuantizationMode::TwoFiveSixSteps),
            _ => Err(()),
        }
    }

    fn value(&self) -> RVoidError<Self::V> {
        match self {
            SampleAttributeTrigQuantizationMode::Direct => Ok(0),
            SampleAttributeTrigQuantizationMode::PatternLength => Ok(255),
            SampleAttributeTrigQuantizationMode::OneStep => Ok(1),
            SampleAttributeTrigQuantizationMode::TwoSteps => Ok(2),
            SampleAttributeTrigQuantizationMode::ThreeSteps => Ok(3),
            SampleAttributeTrigQuantizationMode::FourSteps => Ok(4),
            SampleAttributeTrigQuantizationMode::SixSteps => Ok(5),
            SampleAttributeTrigQuantizationMode::EightSteps => Ok(6),
            SampleAttributeTrigQuantizationMode::TwelveSteps => Ok(7),
            SampleAttributeTrigQuantizationMode::SixteenSteps => Ok(8),
            SampleAttributeTrigQuantizationMode::TwentyFourSteps => Ok(9),
            SampleAttributeTrigQuantizationMode::ThirtyTwoSteps => Ok(10),
            SampleAttributeTrigQuantizationMode::FourtyEightSteps => Ok(11),
            SampleAttributeTrigQuantizationMode::SixtyFourSteps => Ok(12),
            SampleAttributeTrigQuantizationMode::NinetySixSteps => Ok(13),
            SampleAttributeTrigQuantizationMode::OneTwentyEightSteps => Ok(14),
            SampleAttributeTrigQuantizationMode::OneNinetyTwoSteps => Ok(15),
            SampleAttributeTrigQuantizationMode::TwoFiveSixSteps => Ok(16),
        }
    }
}

// TODO: Specification tests

/// MIDI Channel options in Project Settings Menu
/// (only use when `Disabled` is an option).

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize, Copy)]
pub enum ProjectMidiChannels {
    /// No MIDI Channel selected -- Project Menu -> Control -> Midi -> Sync
    #[default]
    Disabled,

    /// MIDI CH 1
    One,

    /// MIDI CH 2
    Two,

    /// MIDI CH 3
    Three,

    /// MIDI CH 4
    Four,

    /// MIDI CH 5
    Five,

    /// MIDI CH 6
    Six,

    /// MIDI CH 7
    Seven,

    /// MIDI CH 8
    Eight,

    /// MIDI CH 9
    Nine,

    /// MIDI CH 10
    Ten,

    /// MIDI CH 11
    Eleven,

    /// MIDI CH 12
    Twelve,

    /// MIDI CH 13
    Thirteen,

    /// MIDI CH 14
    Fourteen,

    /// MIDI CH 15
    Fifteen,

    /// MIDI CH 16
    Sixteen,
}

impl OptionEnumValueConvert for ProjectMidiChannels {
    type T = ProjectMidiChannels;
    type V = i8;

    fn from_value(v: Self::V) -> RVoidError<Self::T> {
        match v {
            -1 => Ok(Self::Disabled),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            10 => Ok(Self::Ten),
            11 => Ok(Self::Eleven),
            12 => Ok(Self::Twelve),
            13 => Ok(Self::Thirteen),
            14 => Ok(Self::Fourteen),
            15 => Ok(Self::Fifteen),
            16 => Ok(Self::Sixteen),
            _ => Err(()),
        }
    }

    fn value(&self) -> RVoidError<Self::V> {
        match self {
            Self::Disabled => Ok(-1),
            Self::One => Ok(1),
            Self::Two => Ok(2),
            Self::Three => Ok(3),
            Self::Four => Ok(4),
            Self::Five => Ok(5),
            Self::Six => Ok(6),
            Self::Seven => Ok(7),
            Self::Eight => Ok(8),
            Self::Nine => Ok(9),
            Self::Ten => Ok(10),
            Self::Eleven => Ok(11),
            Self::Twelve => Ok(12),
            Self::Thirteen => Ok(13),
            Self::Fourteen => Ok(14),
            Self::Fifteen => Ok(15),
            Self::Sixteen => Ok(16),
        }
    }
}

/// "Specification" tests ... ie. guarantee that enum values match correct values.
#[cfg(test)]
mod test_spec {

    mod ot_sample_slot_type {

        mod value {

            // NOTE: @dijksterhuis: have to import the trait to use it
            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::ProjectSampleSlotType;

            #[test]
            fn test_static() {
                assert_eq!(ProjectSampleSlotType::Static.value().unwrap(), "STATIC",);
            }
            #[test]
            fn test_flex() {
                assert_eq!(ProjectSampleSlotType::Flex.value().unwrap(), "FLEX",);
            }
        }

        mod from_value {

            // NOTE: @dijksterhuis: have to import the trait to use it
            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::ProjectSampleSlotType;

            #[test]
            fn test_error() {
                assert_eq!(
                    ProjectSampleSlotType::from_value("SOME INCORRECT STRING".to_string()),
                    Err(()),
                );
            }

            #[test]
            fn test_static_upper() {
                assert_eq!(
                    ProjectSampleSlotType::Static,
                    ProjectSampleSlotType::from_value("STATIC".to_string()).unwrap(),
                );
            }

            #[test]
            fn test_static_lower() {
                assert_eq!(
                    ProjectSampleSlotType::Static,
                    ProjectSampleSlotType::from_value("static".to_string()).unwrap(),
                );
            }

            #[test]
            fn test_flex_upper() {
                assert_eq!(
                    ProjectSampleSlotType::Flex,
                    ProjectSampleSlotType::from_value("FLEX".to_string()).unwrap(),
                );
            }

            #[test]
            fn test_flex_lower() {
                assert_eq!(
                    ProjectSampleSlotType::Flex,
                    ProjectSampleSlotType::from_value("flex".to_string()).unwrap(),
                );
            }
        }
    }

    mod ot_trig_quantize_mode {

        mod value {
            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::SampleAttributeTrigQuantizationMode;

            #[test]
            fn test_direct() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::Direct.value().unwrap(),
                    0
                );
            }
            #[test]
            fn test_patternlen() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::PatternLength
                        .value()
                        .unwrap(),
                    255
                );
            }
            #[test]
            fn test_1() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::OneStep
                        .value()
                        .unwrap(),
                    1
                );
            }
            #[test]
            fn test_2() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::TwoSteps
                        .value()
                        .unwrap(),
                    2
                );
            }
            #[test]
            fn test_3() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::ThreeSteps
                        .value()
                        .unwrap(),
                    3
                );
            }
            #[test]
            fn test_4() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::FourSteps
                        .value()
                        .unwrap(),
                    4
                );
            }
            #[test]
            fn test_6() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::SixSteps
                        .value()
                        .unwrap(),
                    5
                );
            }
            #[test]
            fn test_8() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::EightSteps
                        .value()
                        .unwrap(),
                    6
                );
            }
            #[test]
            fn test_12() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::TwelveSteps
                        .value()
                        .unwrap(),
                    7
                );
            }
            #[test]
            fn test_16() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::SixteenSteps
                        .value()
                        .unwrap(),
                    8
                );
            }
            #[test]
            fn test_24() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::TwentyFourSteps
                        .value()
                        .unwrap(),
                    9
                );
            }
            #[test]
            fn test_32() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::ThirtyTwoSteps
                        .value()
                        .unwrap(),
                    10
                );
            }
            #[test]
            fn test_48() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::FourtyEightSteps
                        .value()
                        .unwrap(),
                    11
                );
            }
            #[test]
            fn test_64() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::SixtyFourSteps
                        .value()
                        .unwrap(),
                    12
                );
            }
            #[test]
            fn test_96() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::NinetySixSteps
                        .value()
                        .unwrap(),
                    13
                );
            }
            #[test]
            fn test_128() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::OneTwentyEightSteps
                        .value()
                        .unwrap(),
                    14
                );
            }
            #[test]
            fn test_192() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::OneNinetyTwoSteps
                        .value()
                        .unwrap(),
                    15
                );
            }
            #[test]
            fn test_256() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::TwoFiveSixSteps
                        .value()
                        .unwrap(),
                    16
                );
            }
        }

        mod from_value {
            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::SampleAttributeTrigQuantizationMode;

            #[test]
            fn test_error() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::from_value(200),
                    Err(()),
                );
            }
            #[test]
            fn test_direct() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::Direct,
                    SampleAttributeTrigQuantizationMode::from_value(0).unwrap()
                );
            }
            #[test]
            fn test_patternlen() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::PatternLength,
                    SampleAttributeTrigQuantizationMode::from_value(255).unwrap()
                );
            }
            #[test]
            fn test_1() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::OneStep,
                    SampleAttributeTrigQuantizationMode::from_value(1).unwrap()
                );
            }
            #[test]
            fn test_2() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::TwoSteps,
                    SampleAttributeTrigQuantizationMode::from_value(2).unwrap()
                );
            }
            #[test]
            fn test_3() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::ThreeSteps,
                    SampleAttributeTrigQuantizationMode::from_value(3).unwrap()
                );
            }
            #[test]
            fn test_4() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::FourSteps,
                    SampleAttributeTrigQuantizationMode::from_value(4).unwrap()
                );
            }
            #[test]
            fn test_6() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::SixSteps,
                    SampleAttributeTrigQuantizationMode::from_value(5).unwrap()
                );
            }
            #[test]
            fn test_8() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::EightSteps,
                    SampleAttributeTrigQuantizationMode::from_value(6).unwrap()
                );
            }
            #[test]
            fn test_12() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::TwelveSteps,
                    SampleAttributeTrigQuantizationMode::from_value(7).unwrap()
                );
            }
            #[test]
            fn test_16() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::SixteenSteps,
                    SampleAttributeTrigQuantizationMode::from_value(8).unwrap()
                );
            }
            #[test]
            fn test_24() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::TwentyFourSteps,
                    SampleAttributeTrigQuantizationMode::from_value(9).unwrap()
                );
            }
            #[test]
            fn test_32() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::ThirtyTwoSteps,
                    SampleAttributeTrigQuantizationMode::from_value(10).unwrap()
                );
            }
            #[test]
            fn test_48() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::FourtyEightSteps,
                    SampleAttributeTrigQuantizationMode::from_value(11).unwrap()
                );
            }
            #[test]
            fn test_64() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::SixtyFourSteps,
                    SampleAttributeTrigQuantizationMode::from_value(12).unwrap()
                );
            }
            #[test]
            fn test_96() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::NinetySixSteps,
                    SampleAttributeTrigQuantizationMode::from_value(13).unwrap()
                );
            }
            #[test]
            fn test_128() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::OneTwentyEightSteps,
                    SampleAttributeTrigQuantizationMode::from_value(14).unwrap()
                );
            }
            #[test]
            fn test_192() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::OneNinetyTwoSteps,
                    SampleAttributeTrigQuantizationMode::from_value(15).unwrap()
                );
            }
            #[test]
            fn test_256() {
                assert_eq!(
                    SampleAttributeTrigQuantizationMode::TwoFiveSixSteps,
                    SampleAttributeTrigQuantizationMode::from_value(16).unwrap()
                );
            }
        }
    }

    mod ot_timestrech_mode {

        mod value {
            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::SampleAttributeTimestrechMode;

            #[test]
            fn test_off_value() {
                assert_eq!(SampleAttributeTimestrechMode::Off.value().unwrap(), 0);
            }
            #[test]
            fn test_normal_value() {
                assert_eq!(SampleAttributeTimestrechMode::Normal.value().unwrap(), 2);
            }
            #[test]
            fn test_beat_value() {
                assert_eq!(SampleAttributeTimestrechMode::Beat.value().unwrap(), 3);
            }
        }

        mod from_value {
            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::SampleAttributeTimestrechMode;

            #[test]
            fn test_error() {
                // not in a sequental range with other values
                // dunno why they implemented it to skip value of 1, possible bug or easter egg?
                assert_eq!(SampleAttributeTimestrechMode::from_value(1), Err(()),);
                // do a slightly exhausitve check, but don't test the whole u32 range
                // as it's not worth the performance drain
                for i in 4..u8::MAX {
                    assert_eq!(SampleAttributeTimestrechMode::from_value(i as u32), Err(()),);
                }
            }
            #[test]
            fn test_off_from_value() {
                assert_eq!(
                    SampleAttributeTimestrechMode::Off,
                    SampleAttributeTimestrechMode::from_value(0).unwrap()
                );
            }
            #[test]
            fn test_normal_from_value() {
                assert_eq!(
                    SampleAttributeTimestrechMode::Normal,
                    SampleAttributeTimestrechMode::from_value(2).unwrap()
                );
            }
            #[test]
            fn test_beat_from_value() {
                assert_eq!(
                    SampleAttributeTimestrechMode::Beat,
                    SampleAttributeTimestrechMode::from_value(3).unwrap()
                );
            }
        }
    }

    mod ot_loop_mode {

        mod value {
            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::SampleAttributeLoopMode;

            #[test]
            fn test_off_value() {
                assert_eq!(SampleAttributeLoopMode::Off.value().unwrap(), 0);
            }
            #[test]
            fn test_normal_value() {
                assert_eq!(SampleAttributeLoopMode::Normal.value().unwrap(), 1);
            }
            #[test]
            fn test_beat_value() {
                assert_eq!(SampleAttributeLoopMode::PingPong.value().unwrap(), 2);
            }
        }

        mod from_value {
            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::SampleAttributeLoopMode;

            #[test]
            fn test_error() {
                // do a slightly exhausitve check, but don't test the whole u32 range
                // as it's not worth the performance drain
                for i in 3..u8::MAX {
                    assert_eq!(SampleAttributeLoopMode::from_value(i as u32), Err(()),);
                }
            }
            #[test]
            fn test_off_from_value() {
                assert_eq!(
                    SampleAttributeLoopMode::Off,
                    SampleAttributeLoopMode::from_value(0).unwrap()
                );
            }
            #[test]
            fn test_normal_from_value() {
                assert_eq!(
                    SampleAttributeLoopMode::Normal,
                    SampleAttributeLoopMode::from_value(1).unwrap()
                );
            }
            #[test]
            fn test_beat_from_value() {
                assert_eq!(
                    SampleAttributeLoopMode::PingPong,
                    SampleAttributeLoopMode::from_value(2).unwrap()
                );
            }
        }
    }

    mod ot_proj_settings_midi_channels {

        mod value {
            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::ProjectMidiChannels;

            #[test]
            fn test_disabled() {
                assert_eq!(ProjectMidiChannels::Disabled.value().unwrap(), -1);
            }
            #[test]
            fn test_1() {
                assert_eq!(ProjectMidiChannels::One.value().unwrap(), 1);
            }
            #[test]
            fn test_2() {
                assert_eq!(ProjectMidiChannels::Two.value().unwrap(), 2);
            }
            #[test]
            fn test_3() {
                assert_eq!(ProjectMidiChannels::Three.value().unwrap(), 3);
            }
            #[test]
            fn test_4() {
                assert_eq!(ProjectMidiChannels::Four.value().unwrap(), 4);
            }
            #[test]
            fn test_5() {
                assert_eq!(ProjectMidiChannels::Five.value().unwrap(), 5);
            }
            #[test]
            fn test_6() {
                assert_eq!(ProjectMidiChannels::Six.value().unwrap(), 6);
            }
            #[test]
            fn test_7() {
                assert_eq!(ProjectMidiChannels::Seven.value().unwrap(), 7);
            }
            #[test]
            fn test_8() {
                assert_eq!(ProjectMidiChannels::Eight.value().unwrap(), 8);
            }
            #[test]
            fn test_9() {
                assert_eq!(ProjectMidiChannels::Nine.value().unwrap(), 9);
            }
            #[test]
            fn test_10() {
                assert_eq!(ProjectMidiChannels::Ten.value().unwrap(), 10);
            }
            #[test]
            fn test_11() {
                assert_eq!(ProjectMidiChannels::Eleven.value().unwrap(), 11);
            }
            #[test]
            fn test_12() {
                assert_eq!(ProjectMidiChannels::Twelve.value().unwrap(), 12);
            }
            #[test]
            fn test_13() {
                assert_eq!(ProjectMidiChannels::Thirteen.value().unwrap(), 13);
            }
            #[test]
            fn test_14() {
                assert_eq!(ProjectMidiChannels::Fourteen.value().unwrap(), 14);
            }
            #[test]
            fn test_15() {
                assert_eq!(ProjectMidiChannels::Fifteen.value().unwrap(), 15);
            }
            #[test]
            fn test_16() {
                assert_eq!(ProjectMidiChannels::Sixteen.value().unwrap(), 16);
            }
        }

        mod from_value {

            use crate::octatrack::common::OptionEnumValueConvert;
            use crate::octatrack::options::ProjectMidiChannels;

            #[test]
            fn test_error_1() {
                assert_eq!(ProjectMidiChannels::from_value(100), Err(()),);
            }
            #[test]
            fn test_error_2() {
                assert_eq!(ProjectMidiChannels::from_value(0), Err(()),);
            }
            #[test]
            fn test_disabled() {
                assert_eq!(
                    ProjectMidiChannels::Disabled,
                    ProjectMidiChannels::from_value(-1).unwrap()
                );
            }
            #[test]
            fn test_1() {
                assert_eq!(
                    ProjectMidiChannels::One,
                    ProjectMidiChannels::from_value(1).unwrap()
                );
            }
            #[test]
            fn test_2() {
                assert_eq!(
                    ProjectMidiChannels::Two,
                    ProjectMidiChannels::from_value(2).unwrap()
                );
            }
            #[test]
            fn test_3() {
                assert_eq!(
                    ProjectMidiChannels::Three,
                    ProjectMidiChannels::from_value(3).unwrap()
                );
            }
            #[test]
            fn test_4() {
                assert_eq!(
                    ProjectMidiChannels::Four,
                    ProjectMidiChannels::from_value(4).unwrap()
                );
            }
            #[test]
            fn test_5() {
                assert_eq!(
                    ProjectMidiChannels::Five,
                    ProjectMidiChannels::from_value(5).unwrap()
                );
            }
            #[test]
            fn test_6() {
                assert_eq!(
                    ProjectMidiChannels::Six,
                    ProjectMidiChannels::from_value(6).unwrap()
                );
            }
            #[test]
            fn test_7() {
                assert_eq!(
                    ProjectMidiChannels::Seven,
                    ProjectMidiChannels::from_value(7).unwrap()
                );
            }
            #[test]
            fn test_8() {
                assert_eq!(
                    ProjectMidiChannels::Eight,
                    ProjectMidiChannels::from_value(8).unwrap()
                );
            }
            #[test]
            fn test_9() {
                assert_eq!(
                    ProjectMidiChannels::Nine,
                    ProjectMidiChannels::from_value(9).unwrap()
                );
            }
            #[test]
            fn test_10() {
                assert_eq!(
                    ProjectMidiChannels::Ten,
                    ProjectMidiChannels::from_value(10).unwrap()
                );
            }
            #[test]
            fn test_11() {
                assert_eq!(
                    ProjectMidiChannels::Eleven,
                    ProjectMidiChannels::from_value(11).unwrap()
                );
            }
            #[test]
            fn test_12() {
                assert_eq!(
                    ProjectMidiChannels::Twelve,
                    ProjectMidiChannels::from_value(12).unwrap()
                );
            }
            #[test]
            fn test_13() {
                assert_eq!(
                    ProjectMidiChannels::Thirteen,
                    ProjectMidiChannels::from_value(13).unwrap()
                );
            }
            #[test]
            fn test_14() {
                assert_eq!(
                    ProjectMidiChannels::Fourteen,
                    ProjectMidiChannels::from_value(14).unwrap()
                );
            }
            #[test]
            fn test_15() {
                assert_eq!(
                    ProjectMidiChannels::Fifteen,
                    ProjectMidiChannels::from_value(15).unwrap()
                );
            }
            #[test]
            fn test_16() {
                assert_eq!(
                    ProjectMidiChannels::Sixteen,
                    ProjectMidiChannels::from_value(16).unwrap()
                );
            }
        }
    }
}
