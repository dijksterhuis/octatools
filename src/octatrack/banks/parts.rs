use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_big_array::BigArray;

// #[derive(Debug, Deserialize, Clone)]
// pub struct Header(String);

// fn deserialize_string<'de, D>(deserializer: D) -> Result<String, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let raw_header: Result<[u8; 4], <D as Deserializer<'_>>::Error> =
//         Deserialize::deserialize(deserializer);

//     let header_string: String = raw_header
//         .unwrap()
//         .to_vec()
//         .escape_ascii()
//         .to_string()
//         .split("\\")
//         .take(1)
//         .collect();

//     Ok(header_string)
// }

// fn serialize_string<'de, S>(s: &String, serializer: S) -> Result<<S as Serializer>::Ok, S::Error>
// where
//     S: Serializer,
// {
//     Serialize::serialize(&s.as_bytes(), serializer)
// }


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackMachineAttributes {
    pub static_slot_id: u8,
    pub flex_slot_id: u8,
    #[serde(with = "BigArray")]
    pub todo_unknown: [u8; 2],
    pub todo_recording_buffer_slot_id: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoMaybeMachineParameterPage {
    pub param_1: u8,
    pub param_2: u8,
    pub param_3: u8,
    pub param_4: u8,
    pub param_5: u8,
    pub param_6: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoTrackMachineParameters {
    pub page1: TodoMaybeMachineParameterPage,
    pub page2: TodoMaybeMachineParameterPage,
    pub page3: TodoMaybeMachineParameterPage,
    pub page4: TodoMaybeMachineParameterPage,
    pub page5: TodoMaybeMachineParameterPage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterPage {
    pub param_1: u8,
    pub param_2: u8,
    pub param_3: u8,
    pub param_4: u8,
    pub param_5: u8,
    pub param_6: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackParameters {
    pub lfo: ParameterPage,
    pub amp: ParameterPage,
    pub fx1: ParameterPage,
    pub fx2: ParameterPage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoTrackUnknownBlockA {
    #[serde(with = "BigArray")]
    pub a: [u8; 6],
    #[serde(with = "BigArray")]
    pub b: [u8; 6],
    #[serde(with = "BigArray")]
    pub c: [u8; 6],
    #[serde(with = "BigArray")]
    pub d: [u8; 6],
    #[serde(with = "BigArray")]
    pub e: [u8; 6],
    #[serde(with = "BigArray")]
    pub f: [u8; 6],
}

// TODO: For some reaosn there are EIGHT part sections in the data file...
// I do not know why ... previous states?

/// Parts in the bank, containing track data.
#[derive(Serialize, Deserialize, Clone)]

pub struct Part {
    // #[serde(deserialize_with = "deserialize_string", serialize_with="serialize_string")]
    // pub header: String,
    #[serde(with = "BigArray")]
    pub header: [u8; 4],

    /// All 0 values.
    #[serde(with = "BigArray")]
    pub data_block_1: [u8; 5],

    /// All 4 values.
    /// I'm betting this is FX1 type.
    #[serde(with = "BigArray")]
    pub audio_track_fx1: [u8; 8],

    /// All 8 values
    /// I'm betting this is FX2 type.
    #[serde(with = "BigArray")]
    pub audio_track_fx2: [u8; 8],

    /// 0 8
    /// Absolutely no idea.
    #[serde(with = "BigArray")]
    pub maybe_scene_selection: [u8; 2],

    /// Track MAIN volume.
    /// All 108 values by default.
    /// NOTE: It could also be MAIN_TR1, CUE_TR1, MAIN_TR2, CUE_TR1.
    #[serde(with = "BigArray")]
    pub audio_track_main_volumes: [u8; 8],

    /// Track CUE volume.
    /// All 108 values by default.
    /// NOTE: It could also be MAIN_TR1, CUE_TR1, MAIN_TR2, CUE_TR1.
    #[serde(with = "BigArray")]
    pub audio_track_cue_volumes: [u8; 8],

    /// Machine type for each track.
    /// Static = 0, Flex = 1, Thru = 2, Neighbour = 3, Pickup = 4.
    #[serde(with = "BigArray")]
    pub machine_types: [u8; 8],

    /// Looks like parameter settings pages?
    /// But the numbers don't match what I would expect.
    #[serde(with = "BigArray")]
    pub audio_track_machine_params: [TodoTrackMachineParameters; 8],

    /// Paramater Pages for a given Track.
    #[serde(with = "BigArray")]
    pub parameter_pages: [TrackParameters; 8], // 32

    /// Looks like LFO settings.
    /// 16 zero steps for each of the 8x custom LFOs in LFO designer.
    /// But could also be the Track Parameter Page Settings.
    #[serde(with = "BigArray")]
    pub todo_maybe_lfo_block: [[u8; 30]; 8],

    #[serde(with = "BigArray")]
    pub audio_machine_properties: [TrackMachineAttributes; 8],

    /// I think this is Track Parameter Page Setup
    #[serde(with = "BigArray")]
    pub data_block_2: [[u8; 30]; 8],

    #[serde(with = "BigArray")]
    pub data_block_3: [[u8; 32]; 8],

    #[serde(with = "BigArray")]
    pub data_block_4: [TodoTrackUnknownBlockA; 8],

    #[serde(with = "BigArray")]
    pub data_block_5: [[u8; 12]; 8],

    /// Massive 0xff block
    #[serde(with = "BigArray")]
    pub data_block_6: [u8; 4256],

    #[serde(with = "BigArray")]
    pub data_block_7: [u8; 432],
}

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        #[derive(Debug)]
        struct Part<'a> {
            pub header: &'a [u8; 4],
            pub data_block_1: &'a [u8; 5],
            pub audio_track_fx1: &'a [u8; 8],
            pub audio_track_fx2: &'a [u8; 8],
            pub maybe_scene_selection: &'a [u8; 2],
            pub audio_track_main_volumes: &'a [u8; 8],
            pub audio_track_cue_volumes: &'a [u8; 8],
            pub machine_types: &'a [u8; 8],
            pub audio_track_machine_params: &'a [TodoTrackMachineParameters; 8],
            pub parameter_pages: &'a [TrackParameters; 8],
            pub todo_maybe_lfo_block: &'a [[u8; 30]; 8],
            pub audio_machine_properties: &'a [TrackMachineAttributes; 8],
            pub data_block_2: &'a [[u8; 30]; 8],
            pub data_block_3: &'a [[u8; 32]; 8],
            pub data_block_4: &'a [TodoTrackUnknownBlockA; 8],
            pub data_block_5: &'a [[u8; 12]; 8],
            pub data_block_6: &'a [u8; 4256],
            pub data_block_7: &'a [u8; 432],
        }

        let Self {
            header,
            data_block_1,
            audio_track_fx1,
            audio_track_fx2,
            maybe_scene_selection,
            audio_track_main_volumes,
            audio_track_cue_volumes,
            machine_types,
            audio_track_machine_params,
            parameter_pages,
            todo_maybe_lfo_block,
            audio_machine_properties,
            data_block_2,
            data_block_3,
            data_block_4,
            data_block_5,
            data_block_6,
            data_block_7,
        } = self;

        fmt::Debug::fmt(
            &Part {
                header,
                data_block_1,
                audio_track_fx1,
                audio_track_fx2,
                maybe_scene_selection,
                audio_track_main_volumes,
                audio_track_cue_volumes,
                machine_types,
                audio_track_machine_params,
                parameter_pages,
                todo_maybe_lfo_block,
                audio_machine_properties,
                data_block_2,
                data_block_3,
                data_block_4,
                data_block_5,
                data_block_6,
                data_block_7,
            },
            f,
        )
    }
}
