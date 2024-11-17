//! Enums for Octatrack values realted to Projects..

// TODO: Break this up into options modules in the projects / samples directories.

use crate::common::{OptionEnumValueConvert, SerdeOctatrackErrors, RBoxErr};
use serde::{Deserialize, Serialize};

/// Sample Slot options for Projects.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
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

    fn from_value(v: &Self::V) -> RBoxErr<Self::T> {
        match v.to_ascii_uppercase().as_str() {
            "STATIC" => Ok(ProjectSampleSlotType::Static),
            "FLEX" => Ok(ProjectSampleSlotType::Flex),
            "RECORDER" => Ok(ProjectSampleSlotType::RecorderBuffer),
            _ => Err(SerdeOctatrackErrors::NoMatchingOptionEnumValue.into()),
        }
    }

    fn value(&self) -> RBoxErr<Self::V> {
        match self {
            ProjectSampleSlotType::Static => Ok("STATIC".to_string()),
            ProjectSampleSlotType::Flex => Ok("FLEX".to_string()),
            ProjectSampleSlotType::RecorderBuffer => Ok("RECORDER".to_string()),
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

    fn from_value(v: &Self::V) -> RBoxErr<Self::T> {
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
            _ => Err(SerdeOctatrackErrors::NoMatchingOptionEnumValue.into()),
        }
    }

    fn value(&self) -> RBoxErr<Self::V> {
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
            use crate::common::OptionEnumValueConvert;
            use crate::projects::options::ProjectSampleSlotType;

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
            use crate::common::OptionEnumValueConvert;
            use crate::projects::options::ProjectSampleSlotType;

            #[test]
            fn test_error() {
                assert!(
                    ProjectSampleSlotType::from_value(&"SOME INCORRECT STRING".to_string()).is_err(),
                );
            }

            #[test]
            fn test_static_upper() {
                assert_eq!(
                    ProjectSampleSlotType::Static,
                    ProjectSampleSlotType::from_value(&"STATIC".to_string()).unwrap(),
                );
            }

            #[test]
            fn test_static_lower() {
                assert_eq!(
                    ProjectSampleSlotType::Static,
                    ProjectSampleSlotType::from_value(&"static".to_string()).unwrap(),
                );
            }

            #[test]
            fn test_flex_upper() {
                assert_eq!(
                    ProjectSampleSlotType::Flex,
                    ProjectSampleSlotType::from_value(&"FLEX".to_string()).unwrap(),
                );
            }

            #[test]
            fn test_flex_lower() {
                assert_eq!(
                    ProjectSampleSlotType::Flex,
                    ProjectSampleSlotType::from_value(&"flex".to_string()).unwrap(),
                );
            }

            #[test]
            fn test_recorder_upper() {
                assert_eq!(
                    ProjectSampleSlotType::RecorderBuffer,
                    ProjectSampleSlotType::from_value(&"RECORDER".to_string()).unwrap(),
                );
            }

            #[test]
            fn test_recorder_lower() {
                assert_eq!(
                    ProjectSampleSlotType::RecorderBuffer,
                    ProjectSampleSlotType::from_value(&"recorder".to_string()).unwrap(),
                );
            }
        }
    }

    mod ot_proj_settings_midi_channels {

        mod value {
            use crate::common::OptionEnumValueConvert;
            use crate::projects::options::ProjectMidiChannels;

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

            use crate::common::OptionEnumValueConvert;
            use crate::projects::options::ProjectMidiChannels;

            #[test]
            fn test_error_1() {
                assert!(ProjectMidiChannels::from_value(&100).is_err());
            }
            #[test]
            fn test_error_2() {
                assert!(ProjectMidiChannels::from_value(&0).is_err());
            }
            #[test]
            fn test_disabled() {
                assert_eq!(
                    ProjectMidiChannels::Disabled,
                    ProjectMidiChannels::from_value(&-1).unwrap()
                );
            }
            #[test]
            fn test_1() {
                assert_eq!(
                    ProjectMidiChannels::One,
                    ProjectMidiChannels::from_value(&1).unwrap()
                );
            }
            #[test]
            fn test_2() {
                assert_eq!(
                    ProjectMidiChannels::Two,
                    ProjectMidiChannels::from_value(&2).unwrap()
                );
            }
            #[test]
            fn test_3() {
                assert_eq!(
                    ProjectMidiChannels::Three,
                    ProjectMidiChannels::from_value(&3).unwrap()
                );
            }
            #[test]
            fn test_4() {
                assert_eq!(
                    ProjectMidiChannels::Four,
                    ProjectMidiChannels::from_value(&4).unwrap()
                );
            }
            #[test]
            fn test_5() {
                assert_eq!(
                    ProjectMidiChannels::Five,
                    ProjectMidiChannels::from_value(&5).unwrap()
                );
            }
            #[test]
            fn test_6() {
                assert_eq!(
                    ProjectMidiChannels::Six,
                    ProjectMidiChannels::from_value(&6).unwrap()
                );
            }
            #[test]
            fn test_7() {
                assert_eq!(
                    ProjectMidiChannels::Seven,
                    ProjectMidiChannels::from_value(&7).unwrap()
                );
            }
            #[test]
            fn test_8() {
                assert_eq!(
                    ProjectMidiChannels::Eight,
                    ProjectMidiChannels::from_value(&8).unwrap()
                );
            }
            #[test]
            fn test_9() {
                assert_eq!(
                    ProjectMidiChannels::Nine,
                    ProjectMidiChannels::from_value(&9).unwrap()
                );
            }
            #[test]
            fn test_10() {
                assert_eq!(
                    ProjectMidiChannels::Ten,
                    ProjectMidiChannels::from_value(&10).unwrap()
                );
            }
            #[test]
            fn test_11() {
                assert_eq!(
                    ProjectMidiChannels::Eleven,
                    ProjectMidiChannels::from_value(&11).unwrap()
                );
            }
            #[test]
            fn test_12() {
                assert_eq!(
                    ProjectMidiChannels::Twelve,
                    ProjectMidiChannels::from_value(&12).unwrap()
                );
            }
            #[test]
            fn test_13() {
                assert_eq!(
                    ProjectMidiChannels::Thirteen,
                    ProjectMidiChannels::from_value(&13).unwrap()
                );
            }
            #[test]
            fn test_14() {
                assert_eq!(
                    ProjectMidiChannels::Fourteen,
                    ProjectMidiChannels::from_value(&14).unwrap()
                );
            }
            #[test]
            fn test_15() {
                assert_eq!(
                    ProjectMidiChannels::Fifteen,
                    ProjectMidiChannels::from_value(&15).unwrap()
                );
            }
            #[test]
            fn test_16() {
                assert_eq!(
                    ProjectMidiChannels::Sixteen,
                    ProjectMidiChannels::from_value(&16).unwrap()
                );
            }
        }
    }
}
