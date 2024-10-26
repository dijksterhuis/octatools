//! Current settings for the Project Tempo. 
//! **NOTE**: This tempo setting works independently to arrangement mode tempo. 


use std::{
    collections::HashMap,
    error::Error,
};
use serde::{Deserialize, Serialize};

use crate::octatrack::common::{
    ParseHashMapValueAs,
    FromHashMap,
};


/// Global `TEMPO` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TempoMenu {

    /// BPM of the current project tempo setting.
    /// **NOTE 1**: This can be ignored by using the `pattern_tempo_enabled`.
    /// **NOTE 2**: Is multiplied by 24 on device.
    tempo: u32,
    
    /// Whether to use the current pattern's tempo or project tempo.
    /// - Pattern Tempo: `true`
    /// - Project Tempo: `false`
    pattern_tempo_enabled: bool,

}

impl ParseHashMapValueAs for TempoMenu {}

impl FromHashMap for TempoMenu {

    type A = String;
    type B = String;
    type T = TempoMenu;

    fn from_hashmap(hmap: &HashMap<Self::A, Self:: B>) -> Result<Self::T, Box<dyn Error>> {
        Ok(
            Self {
                tempo: Self::parse_hashmap_value::<u32>(&hmap, "tempox24")? / 24,
                pattern_tempo_enabled: Self::parse_hashmap_value_bool(&hmap, "pattern_tempo_enabled")?,
            },
        )
    }
}
