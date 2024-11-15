//! Deserialization of `arr??.*` files to extract out and work with Arranger data.
//! Note that Serialization is not complete for arrangement files as there are some 
//! intricacies relate to how Arranger Row data is written in files (will require
//! custom Ser/De trait implementations).
//! 
//! ### How data is persisted in `*.work` and `*.strd` files
//! 
//! `arr??.work` files are updated by using the PROJECT -> SYNC TO CARD operation. 
//! The `arr??.strd` files are not changed by the PROJECT -> SYNC TO CARD operation. 
//! 
//! `arr??.strd` files are updated with the latest arrangement data 
//! when performing the ARRANGER -> SAVE operation. 
//! The `arr??.work` files are not changed by the ARRANGER -> SAVE operation.


use crate::common::{FromFileAtPathBuf, RBoxErr, ToFileAtPathBuf};
use bincode;
use log::{debug, error, info, trace, warn};
use serde::de::SeqAccess;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_big_array::BigArray;
use std::array::from_fn;
use std::{error::Error, fmt, fs::File, io::Read, io::Write, path::PathBuf, str};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ArrangeType {
    PatternSelect = 0,
    Looping = 1,
    Text = 2,
}

#[derive(Debug)]
pub enum ArrangeRow {
    PatternRow {
        // row_type: u8,
        pattern_id: u8,
        repetitions: u8,
        // unused_1: u8,
        mute_mask: u8,
        // unused_2: u8,
        tempo_1: u8,
        tempo_2: u8,
        scene_a: u8,
        scene_b: u8,
        // unused_3: u8,
        offset: u8,
        // unused_4: u8,
        length: u8,
        midi_transpose: [u8; 8],
        // midi_transpose_tr1: u8,
        // midi_transpose_tr2: u8,
        // midi_transpose_tr3: u8,
        // midi_transpose_tr4: u8,
        // midi_transpose_tr5: u8,
        // midi_transpose_tr6: u8,
        // midi_transpose_tr7: u8,
        // midi_transpose_tr8: u8,
    },
    LoopOrJumpOrHaltRow {
        // row_type: u8,
        // unused: u8,
        loop_count: u8,
        row_target: u8,
        // unused_1: u8,
        // unused_2: u8,
        // unused_3: u8,
        // unused_4: u8,
        // unused_5: u8,
        // unused_6: u8,
        // unused_7: u8,
        // unused_8: u8,
        // unused_9: u8,
        // unused_10: u8,
        // unused_11: u8,
        // unused_12: u8,
        // unused_13: u8,
        // unused_14: u8,
        // unused_15: u8,
        // unused_16: u8,
        // unused_17: u8,
        // unused_18: u8,
    },
    JumpRow {
        // row_type: u8,
        // unused: u8,
        // loop_count: u8,
        row_target: u8,
    },
    HaltRow {
        // row_type: u8,
        // unused: u8,
        loop_count: u8,
        row_target: u8,
    },
    ReminderRow(String),
    EmptyRow(),
}

// struct ObjVisitor <'a, 'de: 'a> {
//     de: &'a mut Deserializer<'de>,
//     first: bool,
// }

struct ObjVisitor;

impl<'de> Visitor<'de> for ObjVisitor {
    type Value = ArrangeRow;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("sequence of u8s")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // for some reason doing visit_seq deserialization skips like 5 bytes
        // at the start of the array of bytes that we want to deserialize.
        //
        // i have no idea why this is happening, but everything else about
        // this was working
        //
        // :/

        let mut v: Vec<u8> = vec![];
        for i in 0..=22 {
            let n = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(i + 1, &self))?;

            v.push(n);
        }
        debug!("SSSSSS: {v:#?}");

        let data = match v[0] {
            0 => {
                let midi_transpose: [u8; 8] = from_fn(|x| v[x + 14]);
                let x = ArrangeRow::PatternRow {
                    // row_type: v[0],
                    pattern_id: v[1],
                    repetitions: v[2],
                    // unused_1: v[3],
                    mute_mask: v[4],
                    // unused_2: v[5],
                    tempo_1: v[6],
                    tempo_2: v[7],
                    scene_a: v[8],
                    scene_b: v[9],
                    // unused_3: v[10],
                    offset: v[11],
                    // unused_4: v[12],
                    length: v[13],
                    midi_transpose,
                    // midi_transpose_tr2: v[15],
                    // midi_transpose_tr3: v[16],
                    // midi_transpose_tr4: v[17],
                    // midi_transpose_tr5: v[18],
                    // midi_transpose_tr6: v[19],
                    // midi_transpose_tr7: v[20],
                    // midi_transpose_tr8: v[21],
                };
                Ok(x)
            }
            1 => {
                let x = ArrangeRow::LoopOrJumpOrHaltRow {
                    // row_type: v[0],
                    // unused: v[1],
                    loop_count: v[2],
                    row_target: v[3],
                };
                Ok(x)
            }
            2 => Ok(ArrangeRow::ReminderRow("something2".to_string())),
            _ => {
                // Ok(ArrangeRow::Rem(format!("bounds: {:#?}", v).to_string()))
                let x = ArrangeRow::LoopOrJumpOrHaltRow {
                    // row_type: v[0],
                    // unused: v[1],
                    loop_count: v[2],
                    row_target: v[3],
                };
                Ok(x)
            }
        };
        data
    }

    // fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    // where
    //     E: de::Error,
    // {
    //     Ok(FromStr::from_str(value).unwrap())
    // }

    // fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    //     where
    //         D: Deserializer<'de>, {

    // }<M>(self, visitor: M) -> Result<Self::Value, M::Error>
    // where
    //     M: MapAccess<'de>,
    // {
    //     let aux: ObjAux = Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))?;
    //     Ok(Obj { x: aux.x, y: aux.y })
    // }
}

impl<'de> Deserialize<'de> for ArrangeRow {
    fn deserialize<D>(deserializer: D) -> Result<ArrangeRow, D::Error>
    where
        D: Deserializer<'de>,
    {
        println!("sffs");
        let x = deserializer.deserialize_seq(ObjVisitor);
        println!("XXX {x:#?}");
        x
    }
}

// Note: a JUMP is just a shortcut for an INFINITE loop!

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct ArrangerRowIntermediate {
    /// Type of the arranger field
    ///
    /// | ROW TYPE | VALUE |
    /// | ---- | ---- |
    /// | PATTERN SELECT | 0 |
    /// | HALT/JUMP/LOOP | 1 |
    /// | REM | 2 |
    row_type: u8,

    /// ### PATTERN SELECT
    /// Pattern ID. Pattern `A01`: 0; Pattern `A04`: 3; Pattern `B01`: 16; and so on.
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_1: u8,

    /// ### PATTERN SELECT
    /// Number of Pattern repeats
    ///
    /// ### HALT/JUMP/LOOP
    /// HALT: 0
    /// JUMP: 0
    /// LOOP: Number of loops (0 is infinite looping)
    ///
    /// ### REM
    /// ASCII Character
    value_2: u8,

    /// ### PATTERN SELECT
    /// Not used?
    ///
    /// ### HALT/JUMP/LOOP
    /// Row number to skip to.
    ///
    /// ### PATTERN SELECT
    /// `2` when row is for A01... not sure... need to test.
    ///
    /// ### REM
    /// ASCII Character
    value_3: u8,

    /// ### PATTERN SELECT
    /// MUTE mask. 255 is only MIDI tracks enabled/
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_4: u8,

    /// ### PATTERN SELECT
    /// Not used?
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_5: u8,

    /// ### PATTERN SELECT
    /// Tempo Value A -- must be combined with Tempo Value B (value_7) to get the actual tempo.
    /// For a Tempo of 30 BPM: 2
    /// For a Tempo of 64 BPM: 6 ... ? (64 - 29 ) / 6 = 5.833
    /// For a Tempo of 300 BPM: 28
    ///
    /// ==> This is the 06-.- part of the tempo!
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_6: u8,

    /// ### PATTERN SELECT
    /// Tempo Value B -- must be combined with Tempo Value A (value_6) to get the actual tempo.
    /// For a Tempo of 30 BPM: 208
    /// For a Tempo of 64 BPM: 0
    /// For a Tempo of 300 BPM: 32
    ///
    /// ==> This is the --4.0 part of the tempo!
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_7: u8,

    /// ### PATTERN SELECT
    /// Scene A Assignment
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_8: u8,

    /// ### PATTERN SELECT
    /// Scene B Assignment
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_9: u8,

    /// ### PATTERN SELECT
    /// Not used?
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_10: u8,

    /// ### PATTERN SELECT
    /// Which trig to start the pattern playing on
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_11: u8,

    /// ### PATTERN SELECT
    /// Not used?
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_12: u8,

    /// ### PATTERN SELECT
    /// Number of trigs to play from the start trig (value_11).
    /// WARNING: When value_11 (starting trig) is non-zero, this value is increased by (value_11 + actual_n_trigs).
    /// So, for an ArrangerRow where the pattern starts at trig 32 and plays for 64 trigs, this field's value is 96.
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_13: u8,

    /// ### PATTERN SELECT
    /// midi transpose MIDI Track 1
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_14: u8,

    /// ### PATTERN SELECT
    /// midi transpose MIDI Track 2
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// ASCII Character
    value_15: u8,

    /// ### PATTERN SELECT
    /// midi transpose MIDI Track 3
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// Not used
    value_16: u8,

    /// ### PATTERN SELECT
    /// midi transpose MIDI Track 4
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// Not used
    value_17: u8,

    /// ### PATTERN SELECT
    /// midi transpose MIDI Track 5
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// Not used
    value_18: u8,

    /// ### PATTERN SELECT
    /// midi transpose MIDI Track 6
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// Not used
    value_19: u8,

    /// ### PATTERN SELECT
    /// midi transpose MIDI Track 7
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// Not used
    value_20: u8,

    /// ### PATTERN SELECT
    /// midi transpose MIDI Track 8
    ///
    /// ### HALT/JUMP/LOOP
    /// Not used?
    ///
    /// ### REM
    /// Not used
    value_21: u8,
}

// (0x00001620 + 12 - (0x00000010 + 8)) / 16 = 353.25 ?
// 0x00001620 + 23 --> start of arrangement name block
// 0x00001620 + 23 --> start of second arrangement name block
#[derive(Debug, Serialize, Deserialize)]
struct ArrangementIntermediate {
    /// Name of the Arrangement in ASCII values, max length 15 characters
    #[serde(with = "BigArray")]
    name: [u8; 15],

    /// Unknown data
    #[serde(with = "BigArray")]
    unk1: [u8; 2],

    /// Number of active rows in the arrangement
    pub n_rows: u8,

    /// All possible arranger fields.
    #[serde(with = "BigArray")]
    rows: [ArrangerRowIntermediate; 256],

    #[serde(with = "BigArray")]
    unk2: [u8; 2],
}

#[derive(Debug)]
pub struct Arrangement {
    /// Name of the Arrangement in ASCII values, max length 15 characters
    pub name: String,

    /// Unknown data. No idea what this is. Usually [0, 0, 30].
    pub unk1: [u8; 2],

    /// Number of active rows in the arrangement
    pub n_rows: u8,

    /// Rows of the arrangement. Maximum 256 rows possible.
    pub rows: Vec<ArrangeRow>,

    /// Not sure. First Arrangement block in the file is [0, 1].
    /// Second Arrangement block in the file is [1, 0].
    pub unk2: [u8; 2],
}

// max length: 11336 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct ArrangementFileRawBytes {
    #[serde(with = "BigArray")]
    pub data: [u8; 11336],
}

impl FromFileAtPathBuf for ArrangementFileRawBytes {
    type T = ArrangementFileRawBytes;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: &PathBuf) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let new: Self = bincode::deserialize(&bytes[..])?;

        Ok(new)
    }
}

impl ToFileAtPathBuf for ArrangementFileRawBytes {
    fn to_pathbuf(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let bytes: Vec<u8> = bincode::serialize(&self)?;
        let mut file: File = File::create(path)?;
        let _: RBoxErr<()> = file.write_all(&bytes).map_err(|e| e.into());

        Ok(())
    }
}

// max length: 11336 bytes
#[derive(Debug, Serialize, Deserialize)]
struct ArrangementFileIntermediate {
    #[serde(with = "BigArray")]
    header: [u8; 22],

    #[serde(with = "BigArray")]
    unk1: [u8; 2],

    #[serde(with = "BigArray")]
    arrange_data: [ArrangementIntermediate; 2],

    #[serde(with = "BigArray")]
    unk2: [u8; 8],
}

// max length: 11336 bytes
#[derive(Debug)]
pub struct ArrangementFile {
    /// Header data:
    /// ```text
    /// ASCII: FORM....DPS1ARRA........
    /// Hex: 46 4f 52 4d 00 00 00 00 44 50 53 31 41 52 52 41 00 00 00 00 00 06  
    /// U8: [70, 79, 82, 77, 0, 0, 0, 0, 68, 80, 83, 49, 65, 82, 82, 65, 0, 0, 0, 0, 0, 6]
    /// ```
    pub header: [u8; 22],

    /// Dunno. Example data:
    /// ```text
    /// [0, 0]
    /// ```
    pub unk1: [u8; 2],

    /// Arrangement files each store 2x blocks of `Arrangement` data.
    ///
    /// The first block is what is currently being edited in the arranger view.
    /// This block is written the arrangement is edited in the Arranger view, then Project Menu -> SYNC TO CARD.
    ///
    /// The second block is the 'persisent' store, written when saving the arrangement via Arranger Menu -> SAVE.
    pub arrange_data: Vec<Arrangement>,

    /// This looks like some form of checksum.
    /// Adding more rows increases the values.
    ///
    /// Example data:
    /// ```text
    /// 30 rows: [0, 0, 0, 0, 0, 0, 188, 168]
    /// 31 rows: [0, 0, 0, 0, 0, 0, 196, 196]
    /// 0 rows (just names): [0, 0, 0, 0, 0, 0, 7, 70]
    /// ```
    ///
    pub unk2: [u8; 8],
}

impl FromFileAtPathBuf for ArrangementFile {
    type T = ArrangementFile;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: &PathBuf) -> Result<Self::T, Box<dyn Error>> {
        let mut infile = File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        let _: usize = infile.read_to_end(&mut bytes)?;

        let new: ArrangementFileIntermediate = bincode::deserialize(&bytes[..])?;

        let mut a: Vec<Arrangement> = vec![];

        for arr in new.arrange_data {
            let mut b: Vec<ArrangeRow> = vec![];
            for row in arr.rows {
                let midi_transpose: [u8; 8] = [
                    row.value_14,
                    row.value_15,
                    row.value_16,
                    row.value_17,
                    row.value_18,
                    row.value_19,
                    row.value_20,
                    row.value_21,
                ];

                let x = match row.row_type {
                    0 => {
                        // pattern select with zero value is a NOTHING row.
                        if row.value_13 == 0 {
                            ArrangeRow::EmptyRow()
                        } else {
                            ArrangeRow::PatternRow {
                                // row_type: row.row_type,
                                pattern_id: row.value_1,
                                repetitions: row.value_2,
                                // unused_1: row.value_3,
                                mute_mask: row.value_4,
                                // unused_2: row.value_5,
                                tempo_1: row.value_6,
                                tempo_2: row.value_7,
                                scene_a: row.value_8,
                                scene_b: row.value_9,
                                // unused_3: row.value_10,
                                offset: row.value_11,
                                // unused_4: row.value_12,
                                length: row.value_13,
                                midi_transpose,
                                // midi_transpose_tr1: row.value_14,
                                // midi_transpose_tr2: row.value_15,
                                // midi_transpose_tr3: row.value_16,
                                // midi_transpose_tr4: row.value_17,
                                // midi_transpose_tr5: row.value_18,
                                // midi_transpose_tr6: row.value_19,
                                // midi_transpose_tr7: row.value_20,
                                // midi_transpose_tr8: row.value_21,
                            }
                        }
                    }
                    1 => ArrangeRow::LoopOrJumpOrHaltRow {
                        loop_count: row.value_2,
                        row_target: row.value_3,
                    },
                    2 => {
                        let b = [
                            row.value_1,
                            row.value_2,
                            row.value_3,
                            row.value_4,
                            row.value_5,
                            row.value_6,
                            row.value_7,
                            row.value_8,
                            row.value_9,
                            row.value_10,
                            row.value_11,
                            row.value_12,
                            row.value_13,
                            row.value_14,
                            row.value_15,
                        ];
                        let s = str::from_utf8(&b)?;
                        ArrangeRow::ReminderRow(s.to_string())
                    }
                    _ => ArrangeRow::ReminderRow("ERROR!".to_string()),
                };

                b.push(x);
            }

            let d = Arrangement {
                name: str::from_utf8(&arr.name)?.to_string(),
                unk1: arr.unk1,
                n_rows: arr.n_rows,
                rows: b,
                unk2: arr.unk2,
            };

            a.push(d);
        }

        let decoded = ArrangementFile {
            header: new.header,
            unk1: new.unk1,
            arrange_data: a,
            unk2: new.unk2,
        };

        Ok(decoded)
    }
}

// impl ToFileAtPathBuf for ArrangementFile {
//     fn to_pathbuf(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
//         let bytes: Vec<u8> = bincode::serialize(&self)?;
//         let mut file: File = File::create(path)?;
//         let _: RBoxErr<()> = file.write_all(&bytes).map_err(|e| e.into());

//         Ok(())
//     }
// }
