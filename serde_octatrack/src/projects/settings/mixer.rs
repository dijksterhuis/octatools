//! Current settings for the Project Mixer

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use crate::common::{FromHashMap, ParseHashMapValueAs};

/// Global `MIXER` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MixerMenu {
    /// Controls the incoming gain of external audio signal through AB inputs. -64 to +63 range.
    /// See Manual section 8.8 MIXER MENU
    pub gain_ab: u8, // 64 is default

    /// Controls the incoming gain of external audio signal through CD inputs. -64 to +63 range.
    /// See Manual section 8.8 MIXER MENU
    pub gain_cd: u8, // 64 is default

    /// Routes audio from AB inputs directly to mixer outputs. 0 to 127 range.
    /// See Manual section 8.8 MIXER MENU
    pub dir_ab: u8,

    /// Routes audio from CD inputs directly to mixer outputs. 0 to 127 range.
    /// See Manual section 8.8 MIXER MENU
    pub dir_cd: u8,

    /// How much to mix the master / cue outputs on the headphones output. 0 to 127 range with 64 the default (equal mix)
    /// See Manual section 8.8 MIXER MENU
    pub phones_mix: u8, // 64 is default, so 0 -> 127 with midpoint = 0 middle mix

    /// Unknown.
    /// See Manual section 8.8 MIXER MENU
    pub main_to_cue: u8,

    /// Final gain / output level of the main outputs. -64 to 63 range. 0 is default.
    /// See Manual section 8.8 MIXER MENU
    pub main_level: u8,

    /// Final gain / output level of the cue outputs. -64 to 63 range. 0 is default.
    /// See Manual section 8.8 MIXER MENU
    pub cue_level: u8, // no idea what params max mins are here
}

impl ParseHashMapValueAs for MixerMenu {}

impl FromHashMap for MixerMenu {
    type A = String;
    type B = String;
    type T = MixerMenu;

    fn from_hashmap(hmap: &HashMap<Self::A, Self::B>) -> Result<Self::T, Box<dyn Error>> {
        Ok(Self {
            gain_ab: Self::parse_hashmap_value::<u8>(&hmap, "gain_ab")?,
            gain_cd: Self::parse_hashmap_value::<u8>(&hmap, "gain_cd")?,
            dir_ab: Self::parse_hashmap_value::<u8>(&hmap, "dir_ab")?,
            dir_cd: Self::parse_hashmap_value::<u8>(&hmap, "dir_cd")?,
            phones_mix: Self::parse_hashmap_value::<u8>(&hmap, "phones_mix")?,
            main_to_cue: Self::parse_hashmap_value::<u8>(&hmap, "main_to_cue")?,
            main_level: Self::parse_hashmap_value::<u8>(&hmap, "main_level")?,
            cue_level: Self::parse_hashmap_value::<u8>(&hmap, "cue_level")?,
        })
    }
}
