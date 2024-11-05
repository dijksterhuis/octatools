use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

/// Track trigs assigned on an Audio Track within a Pattern
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AudioTrackTrigs {
    /// Header data section
    ///
    /// example data:
    /// ```
    /// TRAC
    /// 54 52 41 43
    /// ```
    #[serde(with = "BigArray")]
    pub header: [u8; 4],

    /// main block of data, contains a bunch of stuff to parse
    /// trig mask might be in here?
    ///
    /// example data:
    /// ```
    /// 00 00 00 00 01 00 00 00 00 00 00 00 00 00 00 00
    /// 00 00 00 00 00 00 00 00 00 00 00 00 00 aa aa aa
    /// aa aa aa aa aa 00 00 00 00 00 00 00 00 10 02 00
    /// ff 00 00 00 00
    /// ```
    #[serde(with = "BigArray")]
    pub main_blob: [u8; 94],

    /// trig properties -- sample locks, p-locks etc.
    /// the big `0xff` value block within tracks basically.
    /// 32 bytes per trig from what I remember.
    #[serde(with = "BigArray")]
    pub trig_properties: [[u8; 32]; 64],

    /// comes at the end, dunno what this block is yet
    /// mostly a bunch of zero values
    #[serde(with = "BigArray")]
    pub unknown: [u8; 192],
}

/// Track trigs assigned on an Audio Track within a Pattern
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct MidiTrackTrigs {
    // I think these are to do with whether a pattern has been modified
    // or not but i'm not sure yet
    // #[serde(with = "BigArray")]
    // pub modified_bits: [u8; 2],
    /// Header data section
    ///
    /// example data:
    /// ```
    /// MTRA
    /// 4d 54 52 41
    /// ```
    #[serde(with = "BigArray")]
    pub header: [u8; 4],

    /// main block of data, contains a bunch of stuff to parse
    /// trig mask might be in here?
    ///
    /// example data:
    /// ```
    /// 00 00 00 00 01 00 00 00 00 00 00 00 00 00 00 00
    /// 00 00 00 00 00 00 00 00 00 00 00 00 00 aa aa aa
    /// aa aa aa aa aa 00 00 00 00 00 00 00 00 10 02 00
    /// ff 00 00 00 00
    /// ```
    #[serde(with = "BigArray")]
    pub main_blob: [u8; 53],

    /// trig properties -- sample locks, p-locks etc.
    /// the big `0xff` value block within tracks basically.
    /// 32 bytes per trig from what I remember.
    ///
    /// too much data to post a full example here, but here
    /// is one 32 byte block:
    /// ```
    /// ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff
    /// ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff
    /// ```
    #[serde(with = "BigArray")]
    pub trig_properties: [[u8; 32]; 64],

    /// comes at the end, dunno what this block is yet
    /// mostly a bunch of zero values
    #[serde(with = "BigArray")]
    pub unknown: [u8; 128],
}

/// A pattern of trigs stored in the bank.

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Pattern {
    /// Header indicating start of pattern section
    ///
    /// example data:
    /// ```
    /// PTRN....
    /// 50 54 52 4e 00 00 00 00
    /// ```
    #[serde(with = "BigArray")]
    pub header: [u8; 8],

    #[serde(with = "BigArray")]
    audio_track_trigs: [AudioTrackTrigs; 8],

    #[serde(with = "BigArray")]
    midi_track_trigs: [MidiTrackTrigs; 8],

    /// Some pattern level data at the end of the PTRN block
    // I think this is page size and possibly the trig mask? stuff like that
    #[serde(with = "BigArray")]
    other_data: [u8; 12],
}
