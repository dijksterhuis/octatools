//! Common functions/ traits / implementations across Octatrack data structures.

// LEARN: Must be imported to use these methods.
// TODO: from_value and value method generic type arguments

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::path::PathBuf;
use std::str::FromStr;

pub type RBoxErr<T> = Result<T, Box<dyn Error>>;
pub type RVoidError<T> = Result<T, ()>;

/// Trait to convert between Enum option instances and their corresponding value.
pub trait OptionEnumValueConvert {
    /// One of the enum types within the `octatrack::options` module.
    type T;

    /// Input type for `from_value` and return type for `value` method.
    type V;

    /// Get an Enum instance from a numeric value.
    fn from_value(v: Self::V) -> Result<Self::T, ()>;

    /// Get a numeric value for an Enum instance.
    fn value(&self) -> Result<Self::V, ()>;
}

/// Trait for adding the `.swap_bytes()` method.
pub trait SwapBytes {
    /// Type for `Self`
    type T;

    /// Swap the bytes of all struct fields.
    /// Must be applied to the `SampleAttributes` file to deal with litle-endian/big-endian systems.
    fn swap_bytes(&self) -> Result<Self::T, Box<dyn Error>>;
}

/// Trait to enable extracting a section of raw Octatrack Project file ASCII data
pub trait ParseHashMapValueAs {
    fn parse_hashmap_value<T: FromStr>(
        hmap: &HashMap<String, String>,
        key: &str,
    ) -> Result<T, Box<dyn Error>>
    where
        <T as FromStr>::Err: Debug,
    {
        Ok(hmap.get(key).unwrap().parse::<T>().unwrap())
    }

    // special case as boolean values are actually stored as 0 / 1 in the project data
    fn parse_hashmap_value_bool(
        hmap: &HashMap<String, String>,
        key: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut res = false;
        if Self::parse_hashmap_value::<u8>(&hmap, &key)? == 1 {
            res = true
        };
        Ok(res)
    }
}

/// Trait to use when a new struct can be created from some hashmap with all the necessary fields.
pub trait FromHashMap {
    /// Type for `HashMap` keys
    type A;

    /// Type for `HashMap` values
    type B;

    /// Type for `Self`
    type T;

    /// Crete a new struct from a `HashMap`.
    fn from_hashmap(hmap: &HashMap<Self::A, Self::B>) -> Result<Self::T, Box<dyn Error>>;
}

/// Trait to use when a new struct can be deserialised from some file located at the specified path.
pub trait FromFileAtPathBuf {
    /// Type for `Self`
    type T;

    /// Crete a new struct by reading a file located at `path`.
    fn from_pathbuf(path: &PathBuf) -> Result<Self::T, Box<dyn std::error::Error>>;
}

/// Trait to use when a new file can be written at the speciifed path by serializing a struct
pub trait ToFileAtPathBuf {
    /// Crete a new file at the path file location struct by serializing struct data.
    fn to_pathbuf(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>>;
}

/// Trait to use when a new struct can be created by reading a string.
pub trait ProjectFromString {
    /// Type for `Self`
    type T;

    /// Crete a new struct by parsing a `String`.
    fn from_string(data: &String) -> Result<Self::T, Box<dyn std::error::Error>>;
}

/// Trait to use when a new struct can be created by reading a string.
pub trait ProjectToString {
    /// Crete a new struct by parsing a `String`.
    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>>;
}
