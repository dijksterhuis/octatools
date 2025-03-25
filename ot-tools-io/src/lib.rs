//! # `ot-tools-io`
//!
//! Serialization and Deserialization library for Elektron Octatrack data files, including
//!
//! - arrangement files -- `arr??.*`
//! - bank files -- `bank??.*`
//! - project files -- `project.*`
//! - sample attribute files -- `*.ot`
//!
//! The code in this library is quite rough still.
//! Do not expect anything robust just yet.

pub mod arrangements;
pub mod banks;
pub mod constants;
pub mod projects;
pub mod samples;
pub mod utils;

use serde::{Deserialize, Serialize};
use serde_big_array::Array;
use std::array::from_fn;
use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{Read, Write},
    path::Path,
};

// todo: sized errors so not necessary to keep Boxing error enum varients
/// Shorthand type alias for a Result with a Boxed Error
type RBoxErr<T> = Result<T, Box<dyn Error>>;

/// Global error variants
#[derive(Debug, PartialEq, Eq)]
pub enum SerdeOctatrackErrors {
    /// An 'Options' Enum (e.g. `SampleAttributesLoopMode`) does not have a matching variant for this value
    NoMatchingOptionEnumValue,
    /// Could not parse a sample slot string data when loading a project
    ProjectSampleSlotParsingError,
    /// I know an error exists here, but I'm busy yak shaving something else at the moment.
    TodoError,
}
impl std::fmt::Display for SerdeOctatrackErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoMatchingOptionEnumValue => {
                write!(f, "no matching enum option for the provided value")
            }
            Self::ProjectSampleSlotParsingError => {
                write!(f, "count not load sample slot from project string data")
            }
            Self::TodoError => {
                write!(
                    f,
                    "this error is handled, but an error variant is not created yet"
                )
            }
        }
    }
}
impl Error for SerdeOctatrackErrors {}

// DO-NOT-DERIVE: Implementation details for each enum are always required.
/// Trait to convert between Enum option instances and their corresponding value.
trait OptionEnumValueConvert {
    /// One of the enum types within the `octatrack::options` module.
    type T;

    /// Input type for `from_value` and return type for `value` method.
    type V;

    /// Get an Enum instance from a numeric value.
    fn from_value(v: &Self::V) -> RBoxErr<Self::T>;

    /// Get a numeric value for an Enum instance.
    fn value(&self) -> RBoxErr<Self::V>;
}

pub trait Decode {
    fn decode(bytes: &[u8]) -> RBoxErr<Self>
    where
        Self: Sized,
        Self: for<'a> Deserialize<'a>,
    {
        let x: Self = bincode::deserialize(bytes)?;
        Ok(x)
    }
}

pub trait Encode {
    fn encode(&self) -> RBoxErr<Vec<u8>>
    where
        Self: Serialize,
    {
        Ok(bincode::serialize(&self)?)
    }
}

/*
Personal note: const generic parameters is one of those things that is making me
fall in love with Rust's type system.

A const generic parameter defined like this is automatically picked up by the
type system when creating a new instance of a struct.

So a struct like:
```rust
#[derive(DefaultsAsArray)]
struct MyStruct { pub arr: [u8; 42] }
impl Default for u8 { fn default() -> u8 { 0 } }
let x = MyStruct { arr: u8::defaults() };
```

The type system automatically works out that the method's const generic `N`
parameter is `22` for the `arr` field! No need to manually provide the length as
an argument if the array is already defined as having a specified length!

In non-struct code definitions, the same thing happens if a type hint is given
when declaring the variable:
```
let y: [u8; 42] = u8::defaults();
```

When there's no type hint -- that's where we have to define it:
```
let z = u8::defaults::<42>();
```
*/

/// Used when we need a collection of types as the default, e.g. when we want to
/// get a default sample slot list, we need to return a `Vec<SampleSlot>`
/// The `Default` trait doesn't work in this case, because `Default` is reserved
/// for a creating a default of a single `SampleSlot`.
pub trait DefaultsArray {
    /// Create an Array containing `N` default instances of `Self`.
    fn defaults<const N: usize>() -> [Self; N]
    where
        Self: Default,
    {
        from_fn(|_| Self::default())
    }
}

pub trait DefaultsArrayBoxed {
    /// Create a Boxed 'serde BigArray' Array containing `N` default instances
    /// of `Self`.
    fn defaults<const N: usize>() -> Box<Array<Self, N>>
    where
        Self: Default,
    {
        Box::new(Array(from_fn(|_| Self::default())))
    }
}

/* SER/DE GENERICS ============================================================================== */

/// TODO Serialize a JSON string to a data structure of type `T`
pub fn deserialize_bin_to_type<T>(bytes: &[u8]) -> RBoxErr<T>
where
    T: Decode,
    T: for<'a> Deserialize<'a>,
{
    let x: T = T::decode(bytes)?;
    Ok(x)
}

/// TODO Serialize a xxx from a data structure of type `T`
pub fn serialize_bin_from_type<T>(data: &T) -> RBoxErr<Vec<u8>>
where
    T: Encode,
    T: Serialize,
{
    data.encode()
}

/// Deserialize a JSON string to a data structure of type `T`
pub fn deserialize_json_to_type<T>(data: &str) -> RBoxErr<T>
where
    T: for<'a> Deserialize<'a>,
{
    let x: T = serde_json::from_str(data)?;
    Ok(x)
}

/// Serialize a JSON string from a data structure of type `T`
pub fn serialize_json_from_type<T>(data: &T) -> RBoxErr<String>
where
    T: Serialize,
{
    Ok(serde_json::to_string(&data)?)
}

/// Deserialize a YAML string to a data structure of type `T`
pub fn deserialize_yaml_to_type<T>(data: &str) -> RBoxErr<T>
where
    T: for<'a> Deserialize<'a>,
{
    let x: T = serde_yml::from_str(data)?;
    Ok(x)
}
/// Serialize a YAML string from a data structure of type `T`
pub fn serialize_yaml_from_type<T>(data: &T) -> RBoxErr<String>
where
    T: Serialize,
{
    Ok(serde_yml::to_string(&data)?)
}

/* UTILS ======================================================================================== */

/* NO TESTS BLOCK START */

pub fn yaml_file_to_type<T>(path: &Path) -> RBoxErr<T>
where
    T: for<'a> Deserialize<'a>,
{
    let string = read_str_file(path)?;
    let data = deserialize_yaml_to_type::<T>(&string)?;
    Ok(data)
}

pub fn type_to_yaml_file<T>(data: &T, path: &Path) -> RBoxErr<()>
where
    T: Serialize,
{
    let yaml = serialize_yaml_from_type::<T>(data)?;
    write_str_file(&yaml, path)?;
    Ok(())
}

pub fn json_file_to_type<T>(path: &Path) -> RBoxErr<T>
where
    T: for<'a> Deserialize<'a>,
{
    let string = read_str_file(path)?;
    let data = deserialize_json_to_type::<T>(&string)?;
    Ok(data)
}

pub fn type_to_json_file<T>(data: &T, path: &Path) -> RBoxErr<()>
where
    T: Serialize,
{
    let yaml = serialize_json_from_type::<T>(data)?;
    write_str_file(&yaml, path)?;
    Ok(())
}

/// Create a new type with default settings, and serialize the data to a file.
///
/// **NOTE**: The `Defaults` trait should never be used with this function.
///
/// `Defaults` are always used during the construction of underlying types
/// within a top level type, i.e. the default `Vec<SampleSlot>` only exists
/// inside a `Project` type. There's no reason for us to write `Vec<SampleSlot>`
/// to a binary data file.
pub fn default_type_to_bin_file<T>(outpath: &Path) -> RBoxErr<()>
where
    T: Encode,
    T: Default,
    T: Serialize,
{
    write_type_to_bin_file(&T::default(), outpath)?;
    Ok(())
}

/* NO TESTS BLOCK ENDS */

/// Show deserialized representation of a binary data file of type `T` at `path`
pub fn show_type<T>(path: &Path, newlines: Option<bool>) -> RBoxErr<()>
where
    T: Debug,
    T: Decode,
    T: for<'a> Deserialize<'a>,
{
    let data = read_type_from_bin_file::<T>(path)?;
    if newlines.unwrap_or(true) {
        println!("{data:#?}")
    } else {
        println!("{data:?}")
    };

    Ok(())
}

/// Read a YAML file then write the data to a new `<T>` type file
pub fn yaml_file_to_bin_file<T>(yaml_filepath: &Path, bin_filepath: &Path) -> RBoxErr<()>
where
    T: Encode,
    T: Serialize,
    T: for<'a> Deserialize<'a>,
{
    let yaml = read_str_file(yaml_filepath)?;
    let data = deserialize_yaml_to_type::<T>(&yaml)?;
    write_type_to_bin_file::<T>(&data, bin_filepath)?;
    Ok(())
}

/// Read data of type `<T>` from  a binary data file and write it to a YAML file
pub fn bin_file_to_yaml_file<T>(bin_filepath: &Path, yaml_filepath: &Path) -> RBoxErr<()>
where
    T: Decode,
    T: Serialize,
    T: for<'a> Deserialize<'a>,
{
    let data = read_type_from_bin_file::<T>(bin_filepath)?;
    let yaml = serialize_yaml_from_type::<T>(&data)?;
    write_str_file(&yaml, yaml_filepath)?;
    Ok(())
}

/// Read a JSON file then write the data to a new `<T>` type file
pub fn json_file_to_bin_file<T>(json_filepath: &Path, bin_filepath: &Path) -> RBoxErr<()>
where
    T: Encode,
    T: Serialize,
    T: for<'a> Deserialize<'a>,
{
    let json = read_str_file(json_filepath)?;
    let data = deserialize_json_to_type::<T>(&json)?;
    write_type_to_bin_file::<T>(&data, bin_filepath)?;
    Ok(())
}

/// Read data of type `<T>` from  a binary data file and write it to a JSON file
pub fn bin_file_to_json_file<T>(bin_filepath: &Path, json_filepath: &Path) -> RBoxErr<()>
where
    T: Decode,
    T: Serialize,
    T: for<'a> Deserialize<'a>,
{
    let data = read_type_from_bin_file::<T>(bin_filepath)?;
    let yaml = serialize_json_from_type::<T>(&data)?;
    write_str_file(&yaml, json_filepath)?;
    Ok(())
}

/// Shorthand/helper for reading a type from a binary data file and deserializing it in one go.
pub fn read_type_from_bin_file<T>(path: &Path) -> RBoxErr<T>
where
    T: Decode,
    T: for<'a> Deserialize<'a>,
{
    let bytes = read_bin_file(path)?;
    let data = deserialize_bin_to_type::<T>(&bytes)?;
    Ok(data)
}

/// Shorthand/helper for writing a type to a binary data file while serializing it in one go.
pub fn write_type_to_bin_file<T>(data: &T, path: &Path) -> RBoxErr<()>
where
    T: Encode,
    T: Serialize,
{
    let bytes = serialize_bin_from_type::<T>(data)?;
    write_bin_file(&bytes, path)?;
    Ok(())
}

/// Read `bytes` from a file at `path`. Used for reading octatrack data files.
pub fn read_bin_file(path: &Path) -> RBoxErr<Vec<u8>> {
    let mut infile = File::open(path)?;
    let mut bytes: Vec<u8> = vec![];
    let _: usize = infile.read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// Write `bytes` to a file at `path`. Used for creating new octatrack data files.
pub fn write_bin_file(bytes: &[u8], path: &Path) -> RBoxErr<()> {
    let mut file: File = File::create(path)?;
    file.write_all(bytes)?;
    Ok(())
}

/// Read a file at `path` as a single string. Useful for reading from json and yaml files.
pub fn read_str_file(path: &Path) -> RBoxErr<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();
    let _ = file.read_to_string(&mut string)?;
    Ok(string)
}

/// Write a single `string` to a file at `path`. Useful for writing to json and yaml files.
pub fn write_str_file(string: &str, path: &Path) -> RBoxErr<()> {
    let mut file: File = File::create(path)?;
    write!(file, "{}", string)?;
    Ok(())
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;
    use crate::arrangements::ArrangementFile;
    use crate::banks::Bank;
    use crate::projects::Project;
    use crate::samples::SampleAttributes;

    mod show_ok {
        use super::*;

        #[test]
        fn test_arrangement() {
            let fp = Path::new("../data/tests/blank-project/arr01.work");
            let r = show_type::<ArrangementFile>(fp, None);
            assert!(r.is_ok())
        }

        #[test]
        fn test_bank() {
            let fp = Path::new("../data/tests/blank-project/bank01.work");
            let r = show_type::<Bank>(fp, None);
            assert!(r.is_ok())
        }

        #[test]
        fn test_project() {
            let fp = Path::new("../data/tests/blank-project/project.work");
            let r = show_type::<Project>(fp, None);
            assert!(r.is_ok())
        }

        #[test]
        fn test_sample() {
            let fp = Path::new("../data/tests/misc/pair.ot");
            let r = show_type::<SampleAttributes>(fp, None);
            assert!(r.is_ok())
        }
    }

    // TODO: Add more cases
    mod yaml_file_to_bin_file_ok {
        use super::*;
        use std::path::PathBuf;

        #[test]
        fn test_arrangement_blank() {
            let testfile = PathBuf::from("../data/tests/arrange/blank.work");
            let outfile = std::env::temp_dir().join("ot-tools-io-arrangement-load-test-blank.work");
            let yaml = PathBuf::from("../data/tests/arrange/blank.yaml");

            let r = yaml_file_to_bin_file::<ArrangementFile>(&yaml, &outfile);

            let written = read_type_from_bin_file::<ArrangementFile>(&outfile).unwrap();
            let valid = read_type_from_bin_file::<ArrangementFile>(&testfile).unwrap();

            let _ = std::fs::remove_file(&outfile);
            println!("{r:?}");
            assert!(r.is_ok());
            assert_eq!(written, valid)
        }

        #[test]
        fn test_arrangement_full_options() {
            let testfile = PathBuf::from("../data/tests/arrange/full_options.work");
            let outfile =
                std::env::temp_dir().join("ot-tools-io-arrangement-load-test-full_options.work");
            let yaml = PathBuf::from("../data/tests/arrange/full_options.yaml");

            let r = yaml_file_to_bin_file::<ArrangementFile>(&yaml, &outfile);

            let written = read_type_from_bin_file::<ArrangementFile>(&outfile);
            let valid = read_type_from_bin_file::<ArrangementFile>(&testfile);

            let _ = std::fs::remove_file(&outfile);
            println!("{r:?}");
            assert!(r.is_ok());
            assert_eq!(written.unwrap(), valid.unwrap())
        }

        #[test]
        fn test_arrangement_one_reminder_row() {
            let testfile = PathBuf::from("../data/tests/arrange/one_reminder_row.work");
            let outfile = std::env::temp_dir()
                .join("ot-tools-io-arrangement-load-test-one_reminder_row.work");
            let yaml = PathBuf::from("../data/tests/arrange/one_reminder_row.yaml");

            let r = yaml_file_to_bin_file::<ArrangementFile>(&yaml, &outfile);

            let written = read_type_from_bin_file::<ArrangementFile>(&outfile).unwrap();
            let valid = read_type_from_bin_file::<ArrangementFile>(&testfile).unwrap();

            let _ = std::fs::remove_file(&outfile);
            println!("{r:?}");
            assert!(r.is_ok());
            assert_eq!(written, valid)
        }

        #[test]
        fn test_project() {
            let outfile = std::env::temp_dir().join("ot-tools-actions-project-load-test-ok.work");
            let yaml = PathBuf::from("../data/tests/projects/project.yaml");
            let r = yaml_file_to_bin_file::<Project>(&yaml, &outfile);
            let _ = std::fs::remove_file(&outfile);
            println!("{r:?}");
            assert!(r.is_ok())
        }

        #[test]
        fn test_project_matches_blank() {
            let testfile = PathBuf::from("../data/tests/projects/blank.work");
            let outfile = std::env::temp_dir().join("ot-tools-actions-project-load-test-full.work");
            let yaml = PathBuf::from("../data/tests/projects/project.yaml");

            let r = yaml_file_to_bin_file::<Project>(&yaml, &outfile);

            let written = read_type_from_bin_file::<Project>(&outfile).unwrap();
            let valid = read_type_from_bin_file::<Project>(&testfile).unwrap();

            let _ = std::fs::remove_file(&outfile);
            println!("{r:?}");
            assert!(r.is_ok());
            assert_eq!(written, valid)
        }

        #[test]
        fn test_bank() {
            let outfile = std::env::temp_dir().join("ot-tools-actions-bank-load-test-ok.work");
            let yaml = PathBuf::from("../data/tests/bank/blank.yaml");
            let r = yaml_file_to_bin_file::<Bank>(&yaml, &outfile);
            let _ = std::fs::remove_file(&outfile);
            println!("{r:?}");
            assert!(r.is_ok())
        }

        #[test]
        fn test_sample() {
            let outfile = std::env::temp_dir().join("ot-tools-actions-sample-load-test-ok.work");
            let yaml = PathBuf::from("../data/tests/samples/chain.yaml");
            let r = yaml_file_to_bin_file::<SampleAttributes>(&yaml, &outfile);
            let _ = std::fs::remove_file(&outfile);
            println!("{r:?}");
            assert!(r.is_ok());
        }
    }

    mod bin_file_to_yaml_file_ok {
        use super::*;

        #[test]
        // Windows will add on carriage returns...
        #[cfg(not(target_os = "windows"))]
        fn arrangement_blank() {
            let valid_yaml_path = std::path::Path::new("../data/tests/arrange/blank.yaml");
            let binpath = std::path::Path::new("../data/tests/arrange/blank.work");
            let outyaml = std::env::temp_dir().join("serde-ot-bin2yaml-arrange-blank.yaml");

            let r = crate::bin_file_to_yaml_file::<super::ArrangementFile>(binpath, &outyaml);
            let written = crate::read_str_file(&outyaml).unwrap();
            let valid = crate::read_str_file(valid_yaml_path).unwrap();

            let _ = std::fs::remove_file(&outyaml);
            println!("{r:?}");
            assert!(r.is_ok());
            assert_eq!(valid, written);
        }

        #[test]
        // Windows will add on carriage returns...
        #[cfg(not(target_os = "windows"))]
        fn arrangement_full_options() {
            let valid_yaml_path = std::path::Path::new("../data/tests/arrange/full_options.yaml");
            let binpath = std::path::Path::new("../data/tests/arrange/full_options.work");
            let outyaml = std::env::temp_dir().join("serde-ot-bin2yaml-arrange-fulloptions.yaml");

            let r = crate::bin_file_to_yaml_file::<super::ArrangementFile>(binpath, &outyaml);
            let written = crate::read_str_file(&outyaml).unwrap();
            let valid = crate::read_str_file(valid_yaml_path).unwrap();

            let _ = std::fs::remove_file(&outyaml);
            println!("{r:?}");
            assert!(r.is_ok());
            assert_eq!(valid, written);
        }

        #[test]
        // Windows will add on carriage returns...
        #[cfg(not(target_os = "windows"))]
        fn arrangement_one_reminder_row() {
            let valid_yaml_path =
                std::path::Path::new("../data/tests/arrange/one_reminder_row.yaml");
            let binpath = std::path::Path::new("../data/tests/arrange/one_reminder_row.work");
            let outyaml =
                std::env::temp_dir().join("serde-ot-bin2yaml-arrange-onereminderrow.yaml");

            let r = crate::bin_file_to_yaml_file::<super::ArrangementFile>(binpath, &outyaml);
            let written = crate::read_str_file(&outyaml).unwrap();
            let valid = crate::read_str_file(valid_yaml_path).unwrap();

            let _ = std::fs::remove_file(&outyaml);
            println!("{r:?}");
            assert!(r.is_ok());
            assert_eq!(valid, written);
        }

        #[test]
        fn test_bank() {
            let outfile = std::env::temp_dir().join("ot-tools-actions-bin2yaml-bank-ok.yaml");
            let binfile = Path::new("../data/tests/blank-project/bank01.work");
            let r = bin_file_to_yaml_file::<Bank>(binfile, &outfile);
            let _ = std::fs::remove_file(&outfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_project() {
            let outfile = std::env::temp_dir().join("ot-tools-actions-bin2yaml-project-ok.yaml");
            let binfile = Path::new("../data/tests/blank-project/project.work");
            let r = bin_file_to_yaml_file::<Project>(binfile, &outfile);
            let _ = std::fs::remove_file(&outfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_sample() {
            let outfile = std::env::temp_dir().join("ot-tools-actions-bin2yaml-sample-ok.yaml");
            let binfile = Path::new("../data/tests/misc/pair.ot");
            let r = bin_file_to_yaml_file::<SampleAttributes>(binfile, &outfile);
            let _ = std::fs::remove_file(&outfile);
            assert!(r.is_ok())
        }
    }

    mod bin_file_to_json_file_ok {
        use super::*;

        #[test]
        fn arrangement_blank() {
            // let valid_json_path = std::path::Path::new("TODO");
            let binpath = std::path::Path::new("../data/tests/arrange/blank.work");
            let outjson = std::env::temp_dir().join("serde-ot-bin2yaml-arrange-blank.json");

            let r = crate::bin_file_to_json_file::<super::ArrangementFile>(binpath, &outjson);
            let written = crate::read_str_file(&outjson);
            // let valid = crate::read_str_file(&valid_json_path).unwrap();

            let _ = std::fs::remove_file(&outjson);
            println!("{r:?}");
            assert!(r.is_ok());
            assert!(written.is_ok());
            // assert_eq!(valid, written);
        }

        #[test]
        fn arrangement_full_options() {
            // let valid_json_path = std::path::Path::new("TODO");
            let binpath = std::path::Path::new("../data/tests/arrange/full_options.work");
            let outjson = std::env::temp_dir().join("serde-ot-bin2yaml-arrange-full.json");

            let r = crate::bin_file_to_json_file::<super::ArrangementFile>(binpath, &outjson);
            let written = crate::read_str_file(&outjson);
            // let valid = crate::read_str_file(&valid_json_path).unwrap();

            let _ = std::fs::remove_file(&outjson);
            println!("{r:?}");
            assert!(r.is_ok());
            assert!(written.is_ok());
            // assert_eq!(valid, written);
        }

        #[test]
        fn arrangement_one_reminder_row() {
            // let valid_json_path = std::path::Path::new("TODO");
            let binpath = std::path::Path::new("../data/tests/arrange/one_reminder_row.work");
            let outjson =
                std::env::temp_dir().join("serde-ot-bin2yaml-arrange-one_reminder_row.json");

            let r = crate::bin_file_to_json_file::<super::ArrangementFile>(binpath, &outjson);
            let written = crate::read_str_file(&outjson);
            // let valid = crate::read_str_file(&valid_json_path).unwrap();

            let _ = std::fs::remove_file(&outjson);
            println!("{r:?}");
            assert!(r.is_ok());
            assert!(written.is_ok());
            // assert_eq!(valid, written);
        }

        #[test]
        fn test_bank() {
            let outfile = std::env::temp_dir().join("ot-tools-actions-bin2yaml-bank-ok.json");
            let binfile = Path::new("../data/tests/blank-project/bank01.work");
            let r = bin_file_to_json_file::<Bank>(binfile, &outfile);
            let _ = std::fs::remove_file(&outfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_project() {
            let outfile = std::env::temp_dir().join("ot-tools-actions-bin2yaml-project-ok.json");
            let binfile = Path::new("../data/tests/blank-project/project.work");
            let r = bin_file_to_json_file::<Project>(binfile, &outfile);
            let _ = std::fs::remove_file(&outfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_sample() {
            let outfile = std::env::temp_dir().join("ot-tools-actions-bin2yaml-sample-ok.json");
            let binfile = Path::new("../data/tests/misc/pair.ot");
            let r = bin_file_to_json_file::<SampleAttributes>(binfile, &outfile);
            let _ = std::fs::remove_file(&outfile);
            assert!(r.is_ok())
        }
    }

    // TODO: Add more cases!
    mod json_file_to_bin_file_ok {
        // #[test]
        // fn test_arrangement() {
        //     let outfile = std::env::temp_dir().join("ot-tools-actions-arrangement-load-test-ok.work");
        //     let yaml = PathBuf::from("TODO");
        //     // TODO!
        //     let r = yaml_file_to_bin_file::<ArrangementFile>(&yaml, &outfile);
        //     let _ = std::fs::remove_file(&outfile);
        //     assert!(r.is_ok())
        // }

        // #[test]
        // fn test_bank() {
        //     let outfile = std::env::temp_dir().join("ot-tools-actions-bank-load-test-ok.work");
        //     let yaml = PathBuf::from("TODO");
        //     let r = yaml_file_to_bin_file::<Bank>(&yaml, &outfile);
        //     let _ = std::fs::remove_file(&outfile);
        //     assert!(r.is_ok())
        // }

        // #[test]
        // fn test_project() {
        //     let outfile = std::env::temp_dir().join("ot-tools-actions-project-load-test-ok.work");
        //     let yaml = PathBuf::from("../data/tests/projects/project.yaml");
        //     let r = yaml_file_to_bin_file::<Project>(&yaml, &outfile);
        //     let _ = std::fs::remove_file(&outfile);
        //     assert!(r.is_ok())
        // }

        // #[test]
        // fn test_sample() {
        //     let outfile = std::env::temp_dir().join("ot-tools-actions-sample-load-test-ok.work");
        //     let yaml = PathBuf::from("TODO");
        //     let r = yaml_file_to_bin_file::<SampleAttributes>(&yaml, &outfile);
        //     let _ = std::fs::remove_file(&outfile);
        //     assert!(r.is_ok())
        // }
    }

    mod read_type_from_bin_file_ok {
        use super::*;

        #[test]
        fn arrangement_blank() {
            let binfile = Path::new("../data/tests/arrange/blank.work");
            let r = read_type_from_bin_file::<ArrangementFile>(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn arrangement_full_options() {
            let binfile = Path::new("../data/tests/arrange/full_options.work");
            let r = read_type_from_bin_file::<ArrangementFile>(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn arrangement_one_reminder() {
            let binfile = Path::new("../data/tests/arrange/one_reminder_row.work");
            let r = read_type_from_bin_file::<ArrangementFile>(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_read_type_from_bin_file_bank() {
            let binfile = Path::new("../data/tests/blank-project/bank01.work");
            let r = read_type_from_bin_file::<Bank>(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_read_type_from_bin_file_project() {
            let binfile = Path::new("../data/tests/blank-project/project.work");
            let r = read_type_from_bin_file::<Project>(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_read_type_from_bin_file_sample() {
            let binfile = Path::new("../data/tests/misc/pair.ot");
            let r = read_type_from_bin_file::<SampleAttributes>(binfile);
            assert!(r.is_ok())
        }
    }

    mod write_type_from_bin_file_ok {
        // TODO
    }

    mod read_bin_file_ok {
        use super::*;

        #[test]
        fn arrangement_blank() {
            let binfile = Path::new("../data/tests/arrange/blank.work");
            let r = read_bin_file(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn arrangement_full_options() {
            let binfile = Path::new("../data/tests/arrange/full_options.work");
            let r = read_bin_file(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn arrangement_one_reminder() {
            let binfile = Path::new("../data/tests/arrange/one_reminder_row.work");
            let r = read_bin_file(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_read_bin_file_bank() {
            let binfile = Path::new("../data/tests/blank-project/bank01.work");
            let r = read_bin_file(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_read_bin_file_project() {
            let binfile = Path::new("../data/tests/blank-project/project.work");
            let r = read_bin_file(binfile);
            assert!(r.is_ok())
        }

        #[test]
        fn test_read_bin_file_sample() {
            let binfile = Path::new("../data/tests/misc/pair.ot");
            let r = read_bin_file(binfile);
            assert!(r.is_ok())
        }
    }

    mod write_bin_file_ok {
        // TODO
    }

    mod read_str_file_ok {
        // TODO
    }

    mod write_str_file_ok {
        // TODO
    }

    // TODO: This probably shouldn't be here...
    mod project_read {
        use super::*;
        use crate::projects::metadata::ProjectMetadata;
        use crate::projects::settings::ProjectSettings;
        use crate::projects::slots::ProjectSampleSlot;
        use crate::projects::states::ProjectStates;
        use crate::projects::Project;
        use std::path::PathBuf;

        // can read a project file without errors
        #[test]
        fn test_read_default_project_work_file() {
            let infile = PathBuf::from("../data/tests/blank-project/project.work");
            assert!(read_type_from_bin_file::<Project>(&infile).is_ok());
        }

        // test that the metadata section is correct
        #[test]
        fn test_read_default_project_work_file_metadata() {
            let infile = PathBuf::from("../data/tests/blank-project/project.work");
            let p = read_type_from_bin_file::<Project>(&infile).unwrap();

            let correct = ProjectMetadata::default();

            assert_eq!(p.metadata, correct);
        }

        // test that the states section is correct
        #[test]
        fn test_read_default_project_work_file_states() {
            let infile = PathBuf::from("../data/tests/blank-project/project.work");
            let p = read_type_from_bin_file::<Project>(&infile).unwrap();

            let correct = ProjectStates::default();

            assert_eq!(p.states, correct);
        }

        // test that the states section is correct
        #[test]
        fn test_read_default_project_work_file_settings() {
            let infile = PathBuf::from("../data/tests/blank-project/project.work");
            let p = read_type_from_bin_file::<Project>(&infile).unwrap();

            let correct = ProjectSettings::default();

            assert_eq!(p.settings, correct);
        }

        // test that the states section is correct
        #[test]
        fn test_read_default_project_work_file_sslots() {
            let infile = PathBuf::from("../data/tests/blank-project/project.work");
            let p = read_type_from_bin_file::<Project>(&infile).unwrap();
            let default_sslots = ProjectSampleSlot::defaults();

            assert_eq!(p.slots, default_sslots);
        }

        // test that reading and writing a single project gives the same outputs
        #[test]
        fn test_read_write_default_project_work_file() {
            let infile = PathBuf::from("../data/tests/blank-project/project.work");
            let outfile = std::env::temp_dir().join("default_1.work");
            let p = read_type_from_bin_file::<Project>(&infile).unwrap();
            let _ = write_type_to_bin_file::<Project>(&p, &outfile);

            let p_reread = read_type_from_bin_file::<Project>(&infile).unwrap();

            assert_eq!(p, p_reread)
        }
    }
}
