//! Custom deserialization of arrangement data files.

use crate::arrangements::{ArrangeRow, ArrangementBlock, ArrangementFile, ARRANGEMENT_FILE_HEADER};

use bincode;
use itertools::Itertools;
use serde::de::{
    self, Deserializer, Error as DeserializeErr, IgnoredAny, MapAccess, SeqAccess, Visitor,
};
use serde::Deserialize;
use serde_big_array::Array;
use std::array::from_fn;
use std::{fmt, str};

/// Helper function to convert a Serde Deserializer sequence argument into a `u8` vector.
fn get_bytes_from_seq<'de, A>(mut seq: A) -> Result<Vec<u8>, A::Error>
where
    A: SeqAccess<'de>,
{
    let mut v: Vec<u8> = vec![];
    while seq.size_hint().unwrap_or(0) > 0 {
        v.push(
            seq.next_element()?
                .ok_or_else(|| A::Error::custom("Could not access byte array sequence element"))?,
        );
    }

    Ok(v)
}

/// Custom Deserialize trait for `ArrangeBlock` struct data.
/// Needs to be customised so we can force the correct number of bytes be passed to the Deserializer
/// with `deserialize_tuple`, otherwise we end up with an `io::UnexpectedEof` error whenever we try
/// to deserialize.
impl<'de> Deserialize<'de> for ArrangementFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Header,
            Unk1,
            ArrangementStateCurrent,
            Unk2,
            ArrangementStatePrevious,
            ArrangementsActiveFlag,
            CheckSum,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("ArrangementFile fields")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "header" => Ok(Field::Header),
                            "unk1" => Ok(Field::Unk1),
                            "arrangement_state_current" => Ok(Field::ArrangementStateCurrent),
                            "unk2" => Ok(Field::Unk2),
                            "arrangement_state_previous" => Ok(Field::ArrangementStatePrevious),
                            "arrangements_active_flag" => Ok(Field::ArrangementsActiveFlag),
                            "check_sum" => Ok(Field::CheckSum),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ArrangementFileVisitor;

        impl<'de> Visitor<'de> for ArrangementFileVisitor {
            type Value = ArrangementFile;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ArrangementFile")
            }

            /// Raw binary deserialization is done via `deserializer.deserialize_tuple()`, which
            /// calls `deserializer.deserialize_seq()` under the hood.
            /// We have to use `deserializer.deserialize_tuple()` to ensure the number of bytes
            /// passed from parent deserializers is correct.
            fn visit_seq<V>(self, seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let v: Vec<u8> = get_bytes_from_seq::<V>(seq)?;

                let unk1: [u8; 2] = from_fn(|i| v[ARRANGEMENT_FILE_HEADER.len() + i]);
                let curr_arr_bytes: [u8; 5652] = from_fn(|i| v[i + 24]);

                let arrangement_state_current =
                    bincode::deserialize::<ArrangementBlock>(&curr_arr_bytes).unwrap();
                let unk2: [u8; 2] = from_fn(|i| v[i + 24 + 5650]);

                let prev_arr_bytes: [u8; 5652] = from_fn(|i| v[i + 24 + 5650 + 2]);
                let arrangement_state_previous =
                    bincode::deserialize::<ArrangementBlock>(&prev_arr_bytes).unwrap();

                let arrangements_active_flag: [u8; 8] = from_fn(|i| v[i + 24 + 5650 + 2 + 5650]);

                let check_sum: [u8; 2] = from_fn(|i| v[i + 24 + 5650 + 2 + 5650 + 8]);

                Ok(Self::Value {
                    header: ARRANGEMENT_FILE_HEADER,
                    unk1,
                    arrangement_state_current,
                    unk2,
                    arrangement_state_previous,
                    arrangements_active_flag,
                    check_sum,
                })
            }

            /// Human-readable deserialization for JSON/YAML etc. for ArrangementBlock type data.
            /// Uses the standard Serde custom struct deserialization pattern with Field enum
            /// deserialization.
            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut header = None;
                let mut unk1 = None;
                let mut arrangement_state_current = None;
                let mut unk2 = None;
                let mut arrangement_state_previous = None;
                let mut arrangements_active_flag = None;
                let mut check_sum = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Header => {
                            if header.is_some() {
                                return Err(de::Error::duplicate_field("header"));
                            }
                            // type argument required here, otherwise Serde gets confused about the
                            // header field being a sequence in yaml (likely the same for JSON, but
                            // not tested yet):
                            // ```
                            // Error("header: invalid type: sequence, expected unit", line: 2, column: 1)
                            // ```
                            header = Some(map.next_value::<[u8; 22]>()?);
                        }
                        Field::Unk1 => {
                            if unk1.is_some() {
                                return Err(de::Error::duplicate_field("unk1"));
                            }
                            unk1 = Some(map.next_value::<[u8; 2]>()?);
                        }
                        Field::ArrangementStateCurrent => {
                            if arrangement_state_current.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "arrangement_state_current",
                                ));
                            }
                            arrangement_state_current = Some(map.next_value()?);
                        }
                        Field::Unk2 => {
                            if unk2.is_some() {
                                return Err(de::Error::duplicate_field("unk2"));
                            }
                            unk2 = Some(map.next_value()?);
                        }
                        Field::ArrangementStatePrevious => {
                            if arrangement_state_previous.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "arrangement_state_previous",
                                ));
                            }
                            arrangement_state_previous = Some(map.next_value()?);
                        }
                        Field::ArrangementsActiveFlag => {
                            if arrangements_active_flag.is_some() {
                                return Err(de::Error::duplicate_field("arrangements_active_flag"));
                            }
                            arrangements_active_flag = Some(map.next_value()?);
                        }
                        Field::CheckSum => {
                            if check_sum.is_some() {
                                return Err(de::Error::duplicate_field("check_sum"));
                            }
                            check_sum = Some(map.next_value()?);
                        }
                    }
                }

                // run the check for header data for now... but can possibly allow the field to be
                // ignored in future as we just use the const value below.
                let _header = header.ok_or_else(|| de::Error::missing_field("header"))?;
                let unk1 = unk1.ok_or_else(|| de::Error::missing_field("unk1"))?;
                let arrangement_state_current = arrangement_state_current
                    .ok_or_else(|| de::Error::missing_field("arrangement_state_current"))?;
                let unk2 = unk2.ok_or_else(|| de::Error::missing_field("unk2"))?;
                let arrangement_state_previous = arrangement_state_previous
                    .ok_or_else(|| de::Error::missing_field("arrangement_state_previous"))?;
                let arrangements_active_flag = arrangements_active_flag
                    .ok_or_else(|| de::Error::missing_field("arrangements_active_flag"))?;
                let check_sum = check_sum.ok_or_else(|| de::Error::missing_field("check_sum"))?;

                Ok(Self::Value {
                    header: ARRANGEMENT_FILE_HEADER,
                    unk1,
                    arrangement_state_current,
                    unk2,
                    arrangement_state_previous,
                    arrangements_active_flag,
                    check_sum,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "header",
            "unk1",
            "arrangement_state_current",
            "unk2",
            "arrangement_state_previous",
            "arrangements_active_flag",
            "check_sum",
        ];

        match deserializer.is_human_readable() {
            true => deserializer.deserialize_map(ArrangementFileVisitor),
            false => deserializer.deserialize_tuple(11336, ArrangementFileVisitor),
        }
    }
}

/// Custom Deserialize trait for `ArrangeBlock` struct data.
/// Needs to be customised so we can handle any rows after `n_rows` being deserialized into
/// `ArrangeRow::EmptyRow` variants.
impl<'de> Deserialize<'de> for ArrangementBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArrangementBlockVisitor;

        impl<'de> Visitor<'de> for ArrangementBlockVisitor {
            type Value = ArrangementBlock;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ArrangementBlock")
            }

            /// Raw binary deserialization is done via `deserializer.deserialize_tuple()`, which
            /// calls `deserializer.deserialize_seq()` under the hood.
            /// We have to use `deserializer.deserialize_tuple()` to ensure the number of bytes
            /// passed from parent deserializers is correct.
            fn visit_seq<V>(self, seq: V) -> Result<ArrangementBlock, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let v: Vec<u8> = get_bytes_from_seq::<V>(seq)?;

                let name: [u8; 15] = from_fn(|x| v[x]);
                let unknown_1: [u8; 2] = from_fn(|x| v[x + 15]);
                let n_rows = v[17];

                let rows: [ArrangeRow; 256] = from_fn(|i| {
                    /*
                    IMPORTANT: DONTFIX: @dijksterhuis: It's not possible to work out if a row should
                    be an EmptyRow  exclusively from the bytes for that row.

                    An EmptyRow variant is only used when the current row's index is greater than
                    the number of total rows in an ArrangementBlock.
                    */
                    match i >= n_rows as usize {
                        true => ArrangeRow::EmptyRow(),
                        false => {
                            let offset = 18;
                            let idx_start = offset + (i * 22);

                            let row_bytes: [u8; 22] = from_fn(|j| v[j + idx_start]);
                            bincode::deserialize::<ArrangeRow>(&row_bytes).unwrap()
                        }
                    }
                });

                let rows = Box::new(Array(rows));

                Ok(ArrangementBlock {
                    name,
                    unknown_1,
                    n_rows,
                    rows,
                })
            }

            /// Human-readable deserialization for JSON/YAML etc. for ArrangementBlock type data.
            /// *Does not* uses the standard Serde custom struct deserialization pattern with Field
            /// enum deserialization. Field names are matched manually for now.
            fn visit_map<V>(self, mut map: V) -> Result<ArrangementBlock, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut unknown_1 = None;
                let mut n_rows = None;
                let mut rows = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Some("name") => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Some("unknown_1") => {
                            if unknown_1.is_some() {
                                return Err(de::Error::duplicate_field("unknown_1"));
                            }
                            unknown_1 = Some(map.next_value()?);
                        }
                        Some("n_rows") => {
                            if n_rows.is_some() {
                                return Err(de::Error::duplicate_field("n_rows"));
                            }
                            n_rows = Some(map.next_value()?);
                        }
                        Some("rows") => {
                            if rows.is_some() {
                                return Err(de::Error::duplicate_field("rows"));
                            }
                            rows = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(de::Error::custom(format![
                                "Did not recognise ArrangementBlock key: {:?}",
                                key.unwrap(),
                            ]));
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let unknown_1 = unknown_1.ok_or_else(|| de::Error::missing_field("unknown_1"))?;
                let n_rows = n_rows.ok_or_else(|| de::Error::missing_field("n_rows"))?;
                let rows = rows.ok_or_else(|| de::Error::missing_field("rows"))?;

                Ok(ArrangementBlock {
                    name,
                    unknown_1,
                    n_rows,
                    rows,
                })
            }
        }

        match deserializer.is_human_readable() {
            true => deserializer.deserialize_map(ArrangementBlockVisitor),
            false => deserializer.deserialize_tuple(5650, ArrangementBlockVisitor),
        }
    }
}

/// Custom Deserialize trait for `ArrangeRow` variants.
/// Ensures we can do
/// - conditional/dynamic deserialization for binary data based on the row type byte
/// - deserialize from both human-readable and raw binary formats
///
/// The variant of an `ArrangeRow` is determined by
/// - the row's index in the `ArrangementBlock.rows` array versus number of rows in the arrangement `ArrangementBlock.n_rows`
/// - The value of the first byte for an `ArrangeRow`. See the table below:
///
/// | `ArrangeRow` Variant   | First Byte |
/// | ---------------------- | ---------- |
/// | `PatternRow`           | 0          |
/// | `ReminderRow`          | 0          |
/// | `LoopOrJumpOrHaltRow`  | 0          |
/// | `PatternRow`           | 0          |
/// | `EmptyRow`             | n/a        |
impl<'de> Deserialize<'de> for ArrangeRow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArrangeRowVisitor;

        impl<'de> Visitor<'de> for ArrangeRowVisitor {
            type Value = ArrangeRow;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum ArrangeRow")
            }

            /// Raw binary deserialization is done via `deserializer.deserialize_tuple()`, which
            /// calls `deserializer.deserialize_seq()` under the hood.
            /// We have to use `deserializer.deserialize_tuple()` to ensure the number of bytes
            /// passed from parent deserializers is correct.
            fn visit_seq<V>(self, seq: V) -> Result<ArrangeRow, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let v: Vec<u8> = get_bytes_from_seq::<V>(seq)?;

                if v.len() != 22 {
                    return Err(de::Error::invalid_length(v.len(), &self));
                }

                let row_type = v[0];
                let row_data = v[1..].to_vec();

                if row_data.len() != 21 {
                    return Err(de::Error::custom(format![
                        "ArrangeRow: Invalid row data length, must be 21 bytes, received: {:?}",
                        row_data.len()
                    ]));
                }

                // TODO: Enum Variant with values to match `row_type`?
                //       (can't remember its official name)
                match row_type {
                    0 => {
                        let pattern_id = row_data[0];
                        let repetitions = row_data[1];
                        // TODO: mute mask across Audio and MIDI tracks (again)
                        let mute_mask = row_data[3];
                        // TODO: tempo mask (again)
                        let tempo_1 = row_data[5];
                        let tempo_2 = row_data[6];
                        let scene_a = row_data[7];
                        let scene_b = row_data[8];
                        // TODO: what's the max for this? 64?
                        let offset = row_data[10];
                        // TODO: this has a max of something like 512 somehow?
                        // TODO: The value for length changes on device depending on what the offset
                        //       value is. With a length of 256, adding a 64 offset changes the
                        //       length to 192.
                        let length = row_data[12];

                        let midi_transpose: [u8; 8] = from_fn(|x| row_data[x + 13]);

                        if repetitions > 63 {
                            return Err(de::Error::custom(format![
                                "ArrangeRow::PatternRow: Too many repetitions, must be <= 63: rep={:?}",
                                repetitions,
                            ]));
                        }

                        let x = ArrangeRow::PatternRow {
                            pattern_id,
                            repetitions,
                            mute_mask,
                            tempo_1,
                            tempo_2,
                            scene_a,
                            scene_b,
                            offset,
                            length,
                            midi_transpose,
                        };
                        Ok(x)
                    }
                    1 => {

                        let loop_count = row_data[0];
                        let row_target = row_data[1];

                        if loop_count > 100_u8 {
                            return Err(de::Error::custom(format![
                                "ArrangeRow::LoopOrJumpOrHaltRow: Loop count cannot exceed 100 (99x). loops={:?}",
                                loop_count,
                            ]));
                        }
                        let x = ArrangeRow::LoopOrJumpOrHaltRow {
                            loop_count,
                            row_target,
                        };
                        Ok(x)
                    }
                    2 => {
                        let mut row_data = row_data[0..=14].to_vec();
                        // ignore invalid ascii characters: https://www.asciitable.com
                        let first_invalid: Option<(usize, &u8)> = row_data.iter().find_position(|x| { **x < 32 || **x > 126 });
                        if first_invalid.is_some() {
                            row_data = row_data[..first_invalid.unwrap().0].to_vec();
                        }
                        let s = String::from_utf8(row_data)
                            .unwrap_or("ERROR".to_string())
                            .to_ascii_uppercase();

                        Ok(ArrangeRow::ReminderRow(s))
                    }
                    _ => {
                        Err(de::Error::custom(
                            format!["Invalid row type: {row_type:?} -- must be 0 (PatternRow/EmptyRow) / 1 (LoopOrHaltOrJump) / 2 (Reminder)"]
                        ))
                    }
                }
            }

            /// Human-readable deserialization for JSON/YAML etc. for ArrangeRow data.
            /// *Does not* uses the standard Serde custom struct deserialization pattern with Field
            /// enum deserialization. Field names are matched manually for now.
            fn visit_map<V>(self, mut map: V) -> Result<ArrangeRow, V::Error>
            where
                V: MapAccess<'de>,
            {
                // first key tells us which type of field this is
                let k = map.next_key()?;
                match k {
                    Some("empty") => {
                        let _ = map.next_value::<IgnoredAny>()?;
                        Ok(ArrangeRow::EmptyRow())
                    }
                    Some("reminder") => Ok(ArrangeRow::ReminderRow(map.next_value::<String>()?)),
                    Some("pattern_id") => {
                        let pattern_id = map.next_value::<u8>()?;
                        let repetitions = map.next_entry::<&str, u8>()?.unwrap_or(("", 0)).1;
                        let mute_mask = map.next_entry::<&str, u8>()?.unwrap_or(("", 0)).1;
                        let tempo_1 = map.next_entry::<&str, u8>()?.unwrap_or(("", 0)).1;
                        let tempo_2 = map.next_entry::<&str, u8>()?.unwrap_or(("", 0)).1;
                        let scene_a = map.next_entry::<&str, u8>()?.unwrap_or(("", 0)).1;
                        let scene_b = map.next_entry::<&str, u8>()?.unwrap_or(("", 0)).1;
                        let offset = map.next_entry::<&str, u8>()?.unwrap_or(("", 0)).1;
                        let length = map.next_entry::<&str, u8>()?.unwrap_or(("", 0)).1;
                        let midi_transpose = map
                            .next_entry()?
                            .unwrap_or(("", [0, 0, 0, 0, 0, 0, 0, 0]))
                            .1;

                        Ok(ArrangeRow::PatternRow {
                            pattern_id,
                            repetitions,
                            mute_mask,
                            tempo_1,
                            tempo_2,
                            scene_a,
                            scene_b,
                            offset,
                            length,
                            midi_transpose,
                        })
                    }
                    Some("loop_count") => {
                        let loop_count = map.next_value::<u8>()?;
                        let _ = map.next_key::<IgnoredAny>()?;
                        let row_target = map.next_value::<u8>()?;

                        Ok(ArrangeRow::LoopOrJumpOrHaltRow {
                            loop_count,
                            row_target,
                        })
                    }
                    _ => Err(de::Error::custom(format![
                        "Did not recognise ArrangeRow type based on first key: {:?}",
                        k.unwrap()
                    ])),
                }
            }
        }

        match deserializer.is_human_readable() {
            true => deserializer.deserialize_map(ArrangeRowVisitor),
            false => deserializer.deserialize_tuple(22, ArrangeRowVisitor),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod arrangement_file {

        #[test]
        fn read_binfile_blank() {
            let path = std::path::Path::new("../data/tests/arrange/blank.work");
            let r = crate::read_type_from_bin_file::<super::ArrangementFile>(path);
            println!("{r:?}");
            assert!(r.is_ok());
        }

        #[test]
        fn read_binfile_full_options() {
            let path = std::path::Path::new("../data/tests/arrange/full_options.work");
            let r = crate::read_type_from_bin_file::<super::ArrangementFile>(path);
            println!("{r:?}");
            assert!(r.is_ok());
        }

        #[test]
        fn read_binfile_one_reminder_row() {
            let path = std::path::Path::new("../data/tests/arrange/one_reminder_row.work");
            let r = crate::read_type_from_bin_file::<super::ArrangementFile>(path);
            println!("{r:?}");
            assert!(r.is_ok());
        }
    }

    mod arrangement_block {
        #[test]
        fn empty_rows() {
            use crate::arrangements::ArrangeRow;

            let expected_rows: [ArrangeRow; 256] = std::array::from_fn(|i| {
                if i < 10 {
                    ArrangeRow::PatternRow {
                        pattern_id: 0,
                        repetitions: 0,
                        mute_mask: 0,
                        tempo_1: 0,
                        tempo_2: 0,
                        scene_a: 0,
                        scene_b: 0,
                        offset: 0,
                        length: 0,
                        midi_transpose: [0, 0, 0, 0, 0, 0, 0, 0],
                    }
                } else {
                    ArrangeRow::EmptyRow()
                }
            });

            let rows = Box::new(serde_big_array::Array(expected_rows));

            let expected = super::ArrangementBlock {
                name: [10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10],
                unknown_1: [10, 9],
                n_rows: 10,
                rows,
            };

            let b: [u8; 5652] = std::array::from_fn(|x| {
                match x {
                    // start name
                    0 => 10,
                    // end name
                    14 => 10,
                    // unk1 start
                    15 => 10,
                    // unk1 end
                    16 => 9,
                    // n rows
                    17 => 10,
                    _ => 0,
                }
            });

            let r = bincode::deserialize::<super::ArrangementBlock>(&b);
            assert_eq!(expected, r.unwrap());
        }
    }

    mod arrange_row {
        use super::*;

        #[test]
        fn invalid_row_type() {
            let b = [
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let r = bincode::deserialize::<ArrangeRow>(&b);
            assert!(r.is_err());
        }

        mod empty_row {
            /*
            IMPORTANT: DONTFIX: @dijksterhuis: It's not possible to work out if a row should be an EmptyRow
            exclusively from the bytes for that row.

            An EmptyRow variant is only used when the current row's index is greater than the number of
            total rows in an ArrangementBlock.

            This test is to make sure this is true and no-one tries to fix it.
            */
            #[test]
            fn row_index_matters() {
                let not_expected = super::ArrangeRow::EmptyRow();
                let expected = super::ArrangeRow::PatternRow {
                    pattern_id: 0,
                    repetitions: 0,
                    mute_mask: 0,
                    tempo_1: 0,
                    tempo_2: 0,
                    scene_a: 0,
                    scene_b: 0,
                    offset: 0,
                    length: 0,
                    midi_transpose: [0, 0, 0, 0, 0, 0, 0, 0],
                };

                let b = [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                println!("{r:?}");
                assert!(r.is_ok());
                let x = r.unwrap();
                assert_ne!(not_expected, x);
                assert_eq!(expected, x);
            }

            #[test]
            fn valid_yaml() {
                let expected = super::ArrangeRow::EmptyRow();
                let s = "empty: ''\n";
                let r = serde_yml::from_str::<super::ArrangeRow>(s);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(expected, r.unwrap());
            }

            #[test]
            fn valid_json() {
                let expected = super::ArrangeRow::EmptyRow();
                let s = "{\"empty\":\"\"}";
                let r = serde_json::from_str::<super::ArrangeRow>(s);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(expected, r.unwrap());
            }
        }

        mod reminder_row {
            #[test]
            fn valid() {
                let expected = super::ArrangeRow::ReminderRow("CCCCCCCCCCCCCCC".to_string());
                let b = [
                    2, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 0, 0, 0, 0, 0, 0,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                assert_eq!(expected, r.unwrap());
            }

            #[test]
            fn valid_yaml() {
                let expected = super::ArrangeRow::ReminderRow("CCCCCCCCCCCCCCC".to_string());

                let s = "reminder: CCCCCCCCCCCCCCC\n";
                let r = serde_yml::from_str::<super::ArrangeRow>(s);
                assert_eq!(expected, r.unwrap());
            }

            #[test]
            fn valid_drop_excess_characters() {
                let expected = super::ArrangeRow::ReminderRow("CCCCCCCCCCCCCCC".to_string());
                let b = [
                    2, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
                    99, 99,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                assert_eq!(expected, r.unwrap());
            }
        }
        mod loop_jump_or_halt_row {
            #[test]
            fn valid() {
                let expected = super::ArrangeRow::LoopOrJumpOrHaltRow {
                    loop_count: 1,
                    row_target: 1,
                };
                let b = [
                    1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                assert_eq!(expected, r.unwrap());
            }

            #[test]
            #[ignore]
            fn valid_yaml() {
                let expected = super::ArrangeRow::LoopOrJumpOrHaltRow {
                    loop_count: 1,
                    row_target: 1,
                };

                let s = "loop_count: 1\nrow_target: 1\n";
                let r = serde_yml::from_str::<super::ArrangeRow>(s);
                assert_eq!(expected, r.unwrap());
            }

            #[test]
            fn invalid_loop_count() {
                let b = [
                    1, 101, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                assert!(r.is_err());
            }
        }

        mod pattern_row {

            #[test]
            fn valid_pattern_id_only() {
                let expected = super::ArrangeRow::PatternRow {
                    pattern_id: 1,
                    repetitions: 0,
                    mute_mask: 0,
                    tempo_1: 0,
                    tempo_2: 0,
                    scene_a: 0,
                    scene_b: 0,
                    offset: 0,
                    length: 0,
                    midi_transpose: [0, 0, 0, 0, 0, 0, 0, 0],
                };
                let b = [
                    0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                assert_eq!(expected, r.unwrap());
            }

            #[test]
            fn valid_last_midi_transpose_only() {
                let expected = super::ArrangeRow::PatternRow {
                    pattern_id: 0,
                    repetitions: 0,
                    mute_mask: 0,
                    tempo_1: 0,
                    tempo_2: 0,
                    scene_a: 0,
                    scene_b: 0,
                    offset: 0,
                    length: 0,
                    midi_transpose: [0, 0, 0, 0, 0, 0, 0, 8],
                };
                let b = [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                assert_eq!(expected, r.unwrap());
            }

            #[test]
            fn invalid_too_many_repetitions() {
                let b = [
                    0, 1, 64, 1, 1, 1, 1, 1, 15, 15, 1, 1, 1, 64, 3, 3, 3, 3, 3, 3, 3, 3,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                println!("{r:?}");
                assert!(r.is_err());
            }

            #[test]
            fn valid_all() {
                let expected = super::ArrangeRow::PatternRow {
                    pattern_id: 1,
                    repetitions: 10,
                    mute_mask: 1,
                    tempo_1: 1,
                    tempo_2: 1,
                    scene_a: 15,
                    scene_b: 15,
                    offset: 1,
                    length: 64,
                    midi_transpose: [3, 3, 3, 3, 3, 3, 3, 3],
                };
                let b = [
                    0, 1, 10, 1, 1, 1, 1, 1, 15, 15, 1, 1, 1, 64, 3, 3, 3, 3, 3, 3, 3, 3,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                assert_eq!(expected, r.unwrap());
            }

            #[test]
            fn zero_valued_is_still_pattern_row() {
                let expected = super::ArrangeRow::PatternRow {
                    pattern_id: 0,
                    repetitions: 0,
                    mute_mask: 0,
                    tempo_1: 0,
                    tempo_2: 0,
                    scene_a: 0,
                    scene_b: 0,
                    offset: 0,
                    length: 0,
                    midi_transpose: [0, 0, 0, 0, 0, 0, 0, 0],
                };
                let b = [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                let r = bincode::deserialize::<super::ArrangeRow>(&b);
                assert_eq!(expected, r.unwrap());
            }
        }
    }
}
