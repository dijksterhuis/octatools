//! Current settings for the Project Tempo.
//! **NOTE**: This tempo setting works independently to arrangement mode tempo.

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use crate::projects::{parse_hashmap_string_value, parse_hashmap_string_value_bool, FromHashMap};

/// Global `TEMPO` UI menu.

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TempoMenu {
    /// BPM of the current project tempo setting.
    /// **NOTE 1**: This can be ignored by using the `pattern_tempo_enabled`.
    /// **NOTE 2**: Is multiplied by 24 on device.
    pub tempo: u32,

    /// Whether to use the current pattern's tempo or project tempo.
    /// - Pattern Tempo: `true`
    /// - Project Tempo: `false`
    pub pattern_tempo_enabled: bool,
}

impl FromHashMap for TempoMenu {
    type A = String;
    type B = String;
    type T = TempoMenu;

    fn from_hashmap(hmap: &HashMap<Self::A, Self::B>) -> crate::RBoxErr<Self::T> {
        Ok(Self {
            tempo: parse_hashmap_string_value::<u32>(hmap, "tempox24", None)? / 24,
            pattern_tempo_enabled: parse_hashmap_string_value_bool(
                hmap,
                "pattern_tempo_enabled",
                None,
            )?,
        })
    }
}
