//! Custom serialization of arrangement data.
//!
//! The variant of an `ArrangeRow` is determined by
//! - the row's index in the `ArrangementBlock.rows` array versus number of rows in the arrangement `ArrangementBlock.n_rows`
//! - The value of the first byte for an `ArrangeRow`. See the table below
//!
//! | `ArrangeRow` Variant   | First Byte |
//! | ---------------------- | ---------- |
//! | `PatternRow`           | 0          |
//! | `ReminderRow`          | 0          |
//! | `LoopOrJumpOrHaltRow`  | 0          |
//! | `PatternRow`           | 0          |
//! | `EmptyRow`             | n/a        |

use crate::arrangements::{ArrangeRow, ArrangementBlock};
use itertools::Itertools;

use serde::ser::{Error as SerializeErr, SerializeMap, SerializeStruct, Serializer};
use serde::Serialize;

/// Custom serialization to ensure we can validate that the correct number of `ArrangeRow::EmptyRow`
/// variants will be present in the resulting data.
impl Serialize for ArrangementBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Validation for number of arrangement rows versus the first appearance of a EmptyRow
        // variant (position of first EmptyRow variant should match n_rows).
        //
        // NOTE: This is pure serialization. I'm not messing around with easier YAML config
        // interfaces for users just yet.
        //
        // That might come along later in octatrack-bin and would look something like:
        // - what do you want your arrangement rows to look like
        // - translate that to an ArrangementFile struct
        // - dump to work file (maybe copy existing state to previous state in work file if it
        // exists)

        let first_empty_row = self
            .rows
            .iter()
            .find_position(|x| **x == ArrangeRow::EmptyRow());
        if first_empty_row.is_none() && self.n_rows < 255_u8 {
            return Err(S::Error::custom(format![
                "No Empty Rows, but n_rows is less than 255: firstEmptyIdx={:?} nRows={:?}",
                first_empty_row.unwrap().0,
                self.n_rows,
            ]));
        }

        let first_empty_row = first_empty_row.unwrap_or((0, &ArrangeRow::EmptyRow())).0;
        if first_empty_row != self.n_rows as usize {
            return Err(S::Error::custom(format![
                "Index of first Empty Row does not match value for n_rows: idx={:?} nRows={:?}",
                first_empty_row, self.n_rows,
            ]));
        }

        let mut state = serializer.serialize_struct("ArrangementBlock", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("unknown_1", &self.unknown_1)?;
        state.serialize_field("n_rows", &self.n_rows)?;
        state.serialize_field("rows", &self.rows)?;
        state.end()
    }
}

/// Custom serialization to ensure we can serialize both bytes and human-readable data formats
/// correctly.
/// Please note this currently abuses the `serialize_struct` pattern for writing binary/bytes to
/// ensure we get the correct number of bytes in the correct positions.
impl Serialize for ArrangeRow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // YAML / JSON
        match self {
            ArrangeRow::PatternRow {
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
            } => {
                // TODO: Is max 64 on device
                if repetitions > &63_u8 {
                    return Err(S::Error::custom(
                        "ArrangeRow::PatternRow: Repetitions cannot exceed 63 (64x)",
                    ));
                }

                if *scene_a != 255_u8 && scene_a > &15_u8 {
                    return Err(S::Error::custom("ArrangeRow::PatternRow: Scene A index cannot be greater than 15 (zero index; 16 length)"));
                }
                if *scene_b != 255_u8 && scene_b > &15_u8 {
                    return Err(S::Error::custom("ArrangeRow::PatternRow: Scene B index cannot be greater than 15 (zero index; 16 length)"));
                }

                if serializer.is_human_readable() {
                    let mut state = serializer.serialize_struct("PatternRow", 10)?;
                    state.serialize_field("pattern_id", pattern_id)?;
                    state.serialize_field("repetitions", repetitions)?;
                    state.serialize_field("mute_mask", mute_mask)?;
                    state.serialize_field("tempo_1", tempo_1)?;
                    state.serialize_field("tempo_2", tempo_2)?;
                    state.serialize_field("scene_a", scene_a)?;
                    state.serialize_field("scene_b", scene_b)?;
                    state.serialize_field("offset", offset)?;
                    state.serialize_field("length", length)?;
                    state.serialize_field("midi_transpose", midi_transpose)?;
                    state.end()
                } else {
                    let mut state = serializer.serialize_struct("PatternRow", 22)?;
                    state.serialize_field("row_type", &0_u8)?;
                    state.serialize_field("pattern_id", pattern_id)?;
                    state.serialize_field("repetitions", repetitions)?;
                    state.serialize_field("unused_1", &0_u8)?;
                    state.serialize_field("mute_mask", mute_mask)?;
                    state.serialize_field("unused_2", &0_u8)?;
                    state.serialize_field("tempo_1", tempo_1)?;
                    state.serialize_field("tempo_2", tempo_2)?;
                    state.serialize_field("scene_a", scene_a)?;
                    state.serialize_field("scene_b", scene_b)?;
                    state.serialize_field("unused_3", &0_u8)?;
                    state.serialize_field("offset", offset)?;
                    state.serialize_field("unused_4", &0_u8)?;
                    state.serialize_field("length", length)?;
                    state.serialize_field("midi_transpose", midi_transpose)?;
                    state.end()
                }
            }
            ArrangeRow::LoopOrJumpOrHaltRow {
                loop_count,
                row_target,
            } => {
                if loop_count > &100_u8 {
                    return Err(S::Error::custom(
                        "ArrangeRow::LoopOrJumpOrHaltRow: Loop count cannot exceed 100 (99x)",
                    ));
                }

                if serializer.is_human_readable() {
                    let mut state = serializer.serialize_struct("LoopOrJumpOrHaltRow", 2)?;
                    state.serialize_field("loop_count", loop_count)?;
                    state.serialize_field("row_target", row_target)?;
                    state.end()
                } else {
                    let mut state = serializer.serialize_struct("LoopOrJumpOrHaltRow", 22)?;
                    state.serialize_field("row_type", &1_u8)?;
                    state.serialize_field("loop_count", loop_count)?;
                    state.serialize_field("row_target", row_target)?;
                    state.serialize_field("unused_1", &0_u8)?;
                    state.serialize_field("unused_2", &0_u8)?;
                    state.serialize_field("unused_3", &0_u8)?;
                    state.serialize_field("unused_4", &0_u8)?;
                    state.serialize_field("unused_5", &0_u8)?;
                    state.serialize_field("unused_6", &0_u8)?;
                    state.serialize_field("unused_7", &0_u8)?;
                    state.serialize_field("unused_8", &0_u8)?;
                    state.serialize_field("unused_9", &0_u8)?;
                    state.serialize_field("unused_10", &0_u8)?;
                    state.serialize_field("unused_11", &0_u8)?;
                    state.serialize_field("unused_12", &0_u8)?;
                    state.serialize_field("unused_13", &0_u8)?;
                    state.serialize_field("unused_14", &0_u8)?;
                    state.serialize_field("unused_15", &0_u8)?;
                    state.serialize_field("unused_16", &0_u8)?;
                    state.serialize_field("unused_17", &0_u8)?;
                    state.serialize_field("unused_18", &0_u8)?;
                    state.serialize_field("unused_19", &0_u8)?;
                    state.end()
                }
            }
            ArrangeRow::ReminderRow(x) => {
                if x.len() > 15 {
                    return Err(S::Error::custom(format![
                        "ArrangeRow::ReminderRow: string length exceeds 15: str={:?}",
                        x,
                    ]));
                };
                if serializer.is_human_readable() {
                    let mut state = serializer.serialize_map(Some(1))?;
                    state.serialize_entry("reminder", x)?;
                    state.end()
                } else {
                    let mut state = serializer.serialize_struct("ReminderRow", 22)?;
                    state.serialize_field("row_type", &2_u8)?;
                    for c in x.as_bytes() {
                        state.serialize_field("char", &c)?;
                    }
                    for _ in x.len()..15 {
                        state.serialize_field("char", &0_u8)?;
                    }
                    state.serialize_field("unused_1", &0_u8)?;
                    state.serialize_field("unused_2", &0_u8)?;
                    state.serialize_field("unused_3", &0_u8)?;
                    state.serialize_field("unused_4", &0_u8)?;
                    state.serialize_field("unused_5", &0_u8)?;
                    state.serialize_field("unused_6", &0_u8)?;
                    state.end()
                }
            }
            ArrangeRow::EmptyRow() => {
                if serializer.is_human_readable() {
                    let mut state = serializer.serialize_map(Some(1))?;
                    state.serialize_entry("empty", "")?;
                    state.end()
                } else {
                    let mut state = serializer.serialize_struct("EmptyRow", 0)?;
                    state.serialize_field("unused_1", &0_u8)?;
                    state.serialize_field("unused_2", &0_u8)?;
                    state.serialize_field("unused_3", &0_u8)?;
                    state.serialize_field("unused_4", &0_u8)?;
                    state.serialize_field("unused_5", &0_u8)?;
                    state.serialize_field("unused_6", &0_u8)?;
                    state.serialize_field("unused_7", &0_u8)?;
                    state.serialize_field("unused_8", &0_u8)?;
                    state.serialize_field("unused_9", &0_u8)?;
                    state.serialize_field("unused_10", &0_u8)?;
                    state.serialize_field("unused_11", &0_u8)?;
                    state.serialize_field("unused_12", &0_u8)?;
                    state.serialize_field("unused_13", &0_u8)?;
                    state.serialize_field("unused_14", &0_u8)?;
                    state.serialize_field("unused_15", &0_u8)?;
                    state.serialize_field("unused_16", &0_u8)?;
                    state.serialize_field("unused_17", &0_u8)?;
                    state.serialize_field("unused_18", &0_u8)?;
                    state.serialize_field("unused_19", &0_u8)?;
                    state.serialize_field("unused_20", &0_u8)?;
                    state.serialize_field("unused_21", &0_u8)?;
                    state.serialize_field("unused_22", &0_u8)?;
                    state.end()
                }
            }
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    mod arrangement_file {

        #[test]
        // WARN: Currently depends on deserialization being functional.
        fn test_serialize_to_json() {
            let path = std::path::Path::new("../data/tests/blank-project/arr01.work");
            let r = crate::read_type_from_bin_file::<crate::arrangements::ArrangementFile>(&path)
                .unwrap();
            let json = crate::serialize_json_from_type::<crate::arrangements::ArrangementFile>(&r);
            assert!(json.is_ok());
        }

        #[test]
        // WARN: Currently depends on deserialization being functional.
        fn test_serialize_to_yaml() {
            let valid_yaml_path = std::path::Path::new("../data/tests/arrange/blank.yaml");
            let valid_yaml = crate::read_str_file(&valid_yaml_path);

            let bin_file_path = std::path::Path::new("../data/tests/blank-project/arr01.work");
            let arr = crate::read_type_from_bin_file::<crate::arrangements::ArrangementFile>(
                &bin_file_path,
            );
            let yaml = crate::serialize_yaml_from_type::<crate::arrangements::ArrangementFile>(
                &arr.unwrap(),
            );

            assert!(yaml.is_ok());

            let yaml_str = yaml.unwrap();
            assert_eq!(valid_yaml.unwrap(), yaml_str)
        }

        // re-enable this test to dump a yaml file to the serde_octatrack package directory.
        // useful for inspecting the current schema.
        #[test]
        #[ignore]
        fn test_serialize_custom_arrange_to_yaml() {
            let expected_rows: [super::ArrangeRow; 256] = std::array::from_fn(|i| {
                if i < 3 {
                    super::ArrangeRow::PatternRow {
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
                } else if i < 6 {
                    super::ArrangeRow::LoopOrJumpOrHaltRow {
                        loop_count: 2,
                        row_target: 0,
                    }
                } else if i < 10 {
                    super::ArrangeRow::ReminderRow("CCCCCCCCCCCCC".to_string())
                } else {
                    super::ArrangeRow::EmptyRow()
                }
            });

            let empty_rows: [super::ArrangeRow; 256] =
                std::array::from_fn(|_| super::ArrangeRow::EmptyRow());

            let rows_1 = Box::new(serde_big_array::Array(expected_rows));
            let rows_2 = Box::new(serde_big_array::Array(empty_rows));

            let arrangement_state_current = super::ArrangementBlock {
                name: [45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45],
                unknown_1: [99, 99],
                n_rows: 10,
                rows: rows_1,
            };
            let arrangement_state_previous = super::ArrangementBlock {
                name: [70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70],
                unknown_1: [99, 99],
                n_rows: 0,
                rows: rows_2,
            };

            let arr_f = crate::arrangements::ArrangementFile {
                header: crate::arrangements::ARRANGEMENT_FILE_HEADER,
                unk1: [0, 0],
                arrangement_state_current,
                unk2: [0, 0],
                arrangement_state_previous,
                arrangements_active_flag: [1, 0, 0, 0, 0, 0, 0, 0],
                check_sum: [0, 0],
            };

            let outyaml = std::path::Path::new("./test_thing.yaml");
            let _ =
                crate::type_to_yaml_file::<crate::arrangements::ArrangementFile>(&arr_f, outyaml)
                    .unwrap();
            assert!(false);
        }
    }

    mod arrangement_block {
        #[test]
        fn test_ok() {
            let expected_rows: [super::ArrangeRow; 256] = std::array::from_fn(|i| {
                if i < 10 {
                    super::ArrangeRow::PatternRow {
                        pattern_id: 1,
                        repetitions: 1,
                        mute_mask: 1,
                        tempo_1: 1,
                        tempo_2: 1,
                        scene_a: 1,
                        scene_b: 1,
                        offset: 1,
                        length: 1,
                        midi_transpose: [8, 1, 1, 1, 1, 1, 1, 8],
                    }
                } else {
                    super::ArrangeRow::EmptyRow()
                }
            });

            let rows = Box::new(serde_big_array::Array(expected_rows));

            let expected = super::ArrangementBlock {
                name: [10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10],
                unknown_1: [10, 9],
                n_rows: 10,
                rows,
            };

            // TODO: Need to do modulo on index to create pattern row data
            let _: [u8; 5652] = std::array::from_fn(|x| {
                match x {
                    // start name
                    0 => 10,
                    // end name
                    13 => 10,
                    // unk1 start
                    14 => 10,
                    // unk2 end
                    15 => 9,
                    // n rows
                    16 => 10,
                    // unk2 start
                    5650 => 10,
                    // unk2 end
                    5651 => 9,
                    _ => 0,
                }
            });
            let r = bincode::serialize(&expected);
            println!("{r:?}");
            assert!(r.is_ok());
            let v = r.unwrap();
            assert_eq!(5650, v.len());
        }
    }

    mod arrangement_row {
        use super::*;

        mod pattern_row {
            #[test]
            fn valid() {
                let x = super::ArrangeRow::PatternRow {
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
                let r = bincode::serialize(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap().len(), 22);
            }

            #[test]
            fn valid_yaml() {
                let x = super::ArrangeRow::PatternRow {
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
                let r = serde_yml::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "pattern_id: 0\nrepetitions: 0\nmute_mask: 0\ntempo_1: 0\ntempo_2: 0\nscene_a: 0\nscene_b: 0\noffset: 0\nlength: 0\nmidi_transpose:\n- 0\n- 0\n- 0\n- 0\n- 0\n- 0\n- 0\n- 0\n");
            }

            #[test]
            fn valid_json() {
                let x = super::ArrangeRow::PatternRow {
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
                let r = serde_json::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "{\"pattern_id\":0,\"repetitions\":0,\"mute_mask\":0,\"tempo_1\":0,\"tempo_2\":0,\"scene_a\":0,\"scene_b\":0,\"offset\":0,\"length\":0,\"midi_transpose\":[0,0,0,0,0,0,0,0]}");
            }

            #[test]
            fn invalid_repetitions() {
                let x = super::ArrangeRow::PatternRow {
                    pattern_id: 0,
                    repetitions: 64,
                    mute_mask: 0,
                    tempo_1: 0,
                    tempo_2: 0,
                    scene_a: 0,
                    scene_b: 0,
                    offset: 0,
                    length: 0,
                    midi_transpose: [0, 0, 0, 0, 0, 0, 0, 0],
                };
                let r = bincode::serialize(&x);
                assert!(r.is_err());
            }

            #[test]
            fn valid_scene_a_off() {
                let x = super::ArrangeRow::PatternRow {
                    pattern_id: 0,
                    repetitions: 0,
                    mute_mask: 0,
                    tempo_1: 0,
                    tempo_2: 0,
                    scene_a: 255,
                    scene_b: 0,
                    offset: 0,
                    length: 0,
                    midi_transpose: [0, 0, 0, 0, 0, 0, 0, 0],
                };
                let r = bincode::serialize(&x);
                println!("{r:?}");
                assert!(r.is_ok());
            }

            #[test]
            fn valid_scene_b_off() {
                let x = super::ArrangeRow::PatternRow {
                    pattern_id: 0,
                    repetitions: 0,
                    mute_mask: 0,
                    tempo_1: 0,
                    tempo_2: 0,
                    scene_a: 0,
                    scene_b: 255,
                    offset: 0,
                    length: 0,
                    midi_transpose: [0, 0, 0, 0, 0, 0, 0, 0],
                };
                let r = bincode::serialize(&x);
                println!("{r:?}");
                assert!(r.is_ok());
            }

            #[test]
            fn invalid_scene_a() {
                let x = super::ArrangeRow::PatternRow {
                    pattern_id: 0,
                    repetitions: 0,
                    mute_mask: 0,
                    tempo_1: 0,
                    tempo_2: 0,
                    scene_a: 16,
                    scene_b: 0,
                    offset: 0,
                    length: 0,
                    midi_transpose: [0, 0, 0, 0, 0, 0, 0, 0],
                };
                let r = bincode::serialize(&x);
                println!("{r:#?}");
                assert!(r.is_err());
            }

            #[test]
            fn invalid_scene_b() {
                let x = super::ArrangeRow::PatternRow {
                    pattern_id: 0,
                    repetitions: 0,
                    mute_mask: 0,
                    tempo_1: 0,
                    tempo_2: 0,
                    scene_a: 16,
                    scene_b: 16,
                    offset: 0,
                    length: 0,
                    midi_transpose: [0, 0, 0, 0, 0, 0, 0, 0],
                };
                let r = bincode::serialize(&x);
                assert!(r.is_err());
            }
        }

        mod loop_or_jump_or_halt {

            #[test]
            fn valid() {
                let x = super::ArrangeRow::LoopOrJumpOrHaltRow {
                    loop_count: 1,
                    row_target: 1,
                };
                let r = bincode::serialize(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap().len(), 22);
            }

            #[test]
            fn valid_yaml() {
                let x = super::ArrangeRow::LoopOrJumpOrHaltRow {
                    loop_count: 1,
                    row_target: 1,
                };
                let r = serde_yml::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "loop_count: 1\nrow_target: 1\n");
            }

            #[test]
            fn valid_json() {
                let x = super::ArrangeRow::LoopOrJumpOrHaltRow {
                    loop_count: 1,
                    row_target: 1,
                };
                let r = serde_json::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "{\"loop_count\":1,\"row_target\":1}");
            }

            #[test]
            fn invalid_loop_count() {
                let x = super::ArrangeRow::LoopOrJumpOrHaltRow {
                    loop_count: 101,
                    row_target: 1,
                };
                let r = bincode::serialize(&x);
                assert!(r.is_err());
            }
        }

        mod reminder_row {
            #[test]
            fn valid_string() {
                let x = super::ArrangeRow::ReminderRow(String::from("HELLO WORLD"));
                let r = bincode::serialize(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                println!("{r:?}");
                assert_eq!(r.unwrap().len(), 22);
            }

            #[test]
            fn valid_string_yaml() {
                let x = super::ArrangeRow::ReminderRow(String::from("HELLO WORLD"));
                let r = serde_yml::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "reminder: HELLO WORLD\n");
            }

            #[test]
            fn valid_string_json() {
                let x = super::ArrangeRow::ReminderRow(String::from("HELLO WORLD"));
                let r = serde_json::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "{\"reminder\":\"HELLO WORLD\"}");
            }

            #[test]
            fn empty_string() {
                let x = super::ArrangeRow::ReminderRow(String::new());
                let r = bincode::serialize(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap().len(), 22);
            }

            #[test]
            fn empty_string_yaml() {
                let x = super::ArrangeRow::ReminderRow(String::new());
                let r = serde_yml::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "reminder: ''\n");
            }

            #[test]
            fn empty_string_json() {
                let x = super::ArrangeRow::ReminderRow(String::new());
                let r = serde_json::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "{\"reminder\":\"\"}");
            }

            #[test]
            fn invalid() {
                // 16 character string
                let x = super::ArrangeRow::ReminderRow(String::from("1111111111111111"));
                let r = bincode::serialize(&x);
                assert!(r.is_err());
            }
        }

        mod empty_row {
            #[test]
            fn valid() {
                let x = super::ArrangeRow::EmptyRow();
                let r = bincode::serialize(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap().len(), 22);
            }

            #[test]
            fn valid_yaml() {
                let x = super::ArrangeRow::EmptyRow();
                let r = serde_yml::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "empty: ''\n");
            }

            #[test]
            fn valid_json() {
                let x = super::ArrangeRow::EmptyRow();
                let r = serde_json::to_string(&x);
                println!("{r:?}");
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), "{\"empty\":\"\"}");
            }
        }
    }
}
