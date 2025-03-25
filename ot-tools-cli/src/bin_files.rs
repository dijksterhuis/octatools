use crate::{print_err, RBoxErr};
use clap::{Subcommand, ValueEnum, ValueHint};
use ot_tools_io::arrangements::{ArrangementFile, ArrangementFileRawBytes};
use ot_tools_io::banks::{Bank, BankRawBytes};
use ot_tools_io::projects::{Project, ProjectToString};
use ot_tools_io::samples::{SampleAttributes, SampleAttributesRawBytes};
use ot_tools_io::{read_type_from_bin_file, Decode, Encode};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum CliBinFilesError {
    // it's a clap thang
    CreateDefaultSampleAttrUseOtherCommand,
}

impl std::fmt::Display for CliBinFilesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CreateDefaultSampleAttrUseOtherCommand => write!(
                f,
                "`create-default` not implemented for sample attributes files (requires a wav file)"
            ),
        }
    }
}
impl std::error::Error for CliBinFilesError {}

/// Available file formats for converting to/from human-readable data formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub(crate) enum HumanReadableFileFormat {
    Json,
    Yaml,
}

#[derive(Debug, PartialEq, Clone, ValueEnum)]
// #[group(required = false, multiple = false)]
pub(crate) enum BinTypes {
    /// Binary data file is a `project.work` or `project.strd`
    Project,
    /// Binary data file is a `bank??.work` or `bank??.strd`
    Bank,
    /// Binary data file is an `arr??.work` or `arr??.strd`
    Arrangement,
    /// Binary data file is an `*.ot` file
    SampleAttributes,
}

/// Commands for working with individual binary data files directly.
#[derive(Subcommand, Debug, PartialEq)]
pub(crate) enum SubCmds {
    /// Read a binary data file and print the deserialized output to stdout
    Inspect {
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path of the binary data file
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
    },
    /// Read a binary data file and print raw u8 byte values to stdout
    InspectBytes {
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path of the OctaTrack binary data file
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
        /// Index of starting byte range to inspect
        #[arg(value_hint = ValueHint::Other)]
        start: Option<usize>,
        /// Number of bytes to display after starting byte index
        #[arg(value_hint = ValueHint::Other)]
        len: Option<usize>,
    },
    /// Create a binary data file with default data
    CreateDefault {
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path of where to write the new binary data file to
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
    },
    /// Create a human-readable data file from a binary data file
    BinToHuman {
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path to the source binary data file
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
        /// Convert to this type of human-readable format
        #[arg(value_enum)]
        dest_type: HumanReadableFileFormat,
        /// Path to the human-readable output file
        #[arg(value_hint = ValueHint::FilePath)]
        dest_path: PathBuf,
    },
    /// Create a binary data file from a human-readable data file
    HumanToBin {
        /// Read from this type of human-readable format
        #[arg(value_enum)]
        source_type: HumanReadableFileFormat,
        /// Path to the human-readable source file
        #[arg(value_hint = ValueHint::FilePath)]
        source_path: PathBuf,
        /// Type of binary data file
        #[arg(value_enum)]
        bin_type: BinTypes,
        /// Path to the output OctaTrack data file
        #[arg(value_hint = ValueHint::FilePath)]
        bin_path: PathBuf,
    },
}

enum ConvertFromToCmd {
    BinToHuman,
    HumanToBin,
}

#[doc(hidden)]
/// Succinctly handle converting from binary to human-readable and vice versa
fn convert_from_to<T>(
    conversion_type: ConvertFromToCmd,
    human_type: HumanReadableFileFormat,
    human_path: PathBuf,
    bin_path: PathBuf,
) -> RBoxErr<()>
where
    T: Decode,
    T: Encode,
    T: Serialize,
    T: for<'a> Deserialize<'a>,
{
    match conversion_type {
        ConvertFromToCmd::BinToHuman => match human_type {
            HumanReadableFileFormat::Json => {
                ot_tools_io::bin_file_to_json_file::<T>(&bin_path, &human_path)
            }
            HumanReadableFileFormat::Yaml => {
                ot_tools_io::bin_file_to_yaml_file::<T>(&bin_path, &human_path)
            }
        },
        ConvertFromToCmd::HumanToBin => match human_type {
            HumanReadableFileFormat::Json => {
                ot_tools_io::json_file_to_bin_file::<T>(&human_path, &bin_path)
            }
            HumanReadableFileFormat::Yaml => {
                ot_tools_io::yaml_file_to_bin_file::<T>(&human_path, &bin_path)
            }
        },
    }
}

/// Get a slice of a byte vector (`Vec<u8>`) -- mostly for reverse engineering utility purposes
fn get_bytes_slice(data: Vec<u8>, start_idx: &Option<usize>, len: &Option<usize>) -> Vec<u8> {
    let start = start_idx.unwrap_or(0);
    let end = match len {
        None => data.len(),
        _ => len.unwrap() + start,
    };
    data[start..end].to_vec()
}

#[cfg(test)]
mod test_get_byte_slice {
    use super::*;
    #[test]
    fn test_no_options() {
        let data: Vec<u8> = vec![1, 2, 3];
        let r = get_bytes_slice(data, &None, &None);
        assert_eq!(r, vec![1, 2, 3]);
    }
    #[test]
    fn test_no_options_one_byte_data() {
        let data: Vec<u8> = vec![1];
        let r = get_bytes_slice(data, &None, &None);
        assert_eq!(r, vec![1]);
    }
    #[test]
    fn test_non_zero_start() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5];
        let r = get_bytes_slice(data, &Some(1), &None);
        assert_eq!(r, vec![2, 3, 4, 5]);
    }
    #[test]
    fn test_non_zero_end() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5];
        let r = get_bytes_slice(data, &None, &Some(3));
        assert_eq!(r, vec![1, 2, 3]);
    }
    #[test]
    fn test_non_zero_start_and_end() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5];
        let r = get_bytes_slice(data, &Some(1), &Some(3));
        assert_eq!(r, vec![2, 3, 4]);
    }
}

/// Show bytes output as u8 values for a Sample Attributes file located at `path`
fn show_ot_file_bytes(path: &Path, start_idx: &Option<usize>, len: &Option<usize>) -> RBoxErr<()> {
    let raw = read_type_from_bin_file::<SampleAttributesRawBytes>(path)?;

    let bytes = get_bytes_slice(raw.data.to_vec(), start_idx, len);
    println!("{:#?}", bytes);
    Ok(())
}

/// Show bytes output as u8 values for a project file located at `path`
fn show_project_bytes(path: &Path, start_idx: &Option<usize>, len: &Option<usize>) -> RBoxErr<()> {
    let raw_project = read_type_from_bin_file::<Project>(path)?;

    let proj_bytes = raw_project.to_string()?.into_bytes();
    let bytes = get_bytes_slice(proj_bytes, start_idx, len);
    println!("{:#?}", bytes);
    Ok(())
}

/// Show bytes output as u8 values for a Sample Attributes file located at `path`
fn show_bank_bytes(path: &Path, start_idx: &Option<usize>, len: &Option<usize>) -> RBoxErr<()> {
    let raw_bank = read_type_from_bin_file::<BankRawBytes>(path)?;

    let bytes = get_bytes_slice(raw_bank.data.to_vec(), start_idx, len);
    println!("{:#?}", bytes);
    Ok(())
}

/// Show bytes output as u8 values for an Arrangement file located at `path`
fn show_arrangement_bytes(
    path: &Path,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let raw: ArrangementFileRawBytes = read_type_from_bin_file::<ArrangementFileRawBytes>(path)
        .expect("Could not read arrangement file");

    let bytes = get_bytes_slice(raw.data.to_vec(), start_idx, len);
    println!("{:#?}", bytes);
    Ok(())
}

#[cfg(test)]
mod test_arrangement_bytes {
    use super::*;

    #[test]
    fn test_show_bytes_first_all_bytes_ok() {
        let fp = Path::new("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(fp, &None, &None);
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_first_100_bytes_ok() {
        let fp = Path::new("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(fp, &Some(0), &Some(100));
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_1_byte_ok() {
        let fp = Path::new("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(fp, &Some(0), &Some(1));
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_100_bytes_offset_100_ok() {
        let fp = Path::new("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(fp, &Some(100), &Some(100));
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_maxlen_ok() {
        let fp = Path::new("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(fp, &Some(0), &Some(11336));
        assert!(r.is_ok())
    }
}

#[doc(hidden)]
pub(crate) fn subcmd_runner(x: SubCmds) {
    match x {
        SubCmds::Inspect { bin_type, bin_path } => match bin_type {
            BinTypes::Arrangement => {
                print_err(|| ot_tools_io::show_type::<ArrangementFile>(&bin_path, None));
            }
            BinTypes::Bank => {
                print_err(|| ot_tools_io::show_type::<Bank>(&bin_path, None));
            }
            BinTypes::Project => {
                print_err(|| ot_tools_io::show_type::<Project>(&bin_path, None));
            }
            BinTypes::SampleAttributes => {
                print_err(|| ot_tools_io::show_type::<SampleAttributes>(&bin_path, None));
            }
        },
        SubCmds::InspectBytes {
            bin_type,
            bin_path,
            start,
            len,
        } => match bin_type {
            BinTypes::Arrangement => {
                print_err(|| show_arrangement_bytes(&bin_path, &start, &len));
            }
            BinTypes::Bank => {
                print_err(|| show_bank_bytes(&bin_path, &start, &len));
            }
            BinTypes::Project => {
                print_err(|| show_project_bytes(&bin_path, &start, &len));
            }
            BinTypes::SampleAttributes => {
                print_err(|| show_ot_file_bytes(&bin_path, &start, &len));
            }
        },
        SubCmds::CreateDefault { bin_type, bin_path } => {
            match bin_type {
                BinTypes::Arrangement => {
                    print_err(|| {
                        ot_tools_io::default_type_to_bin_file::<ArrangementFile>(&bin_path)
                    });
                }
                BinTypes::Bank => {
                    print_err(|| ot_tools_io::default_type_to_bin_file::<Bank>(&bin_path));
                }
                BinTypes::Project => {
                    print_err(|| ot_tools_io::default_type_to_bin_file::<Project>(&bin_path));
                }
                BinTypes::SampleAttributes => {
                    // it's a clap thang
                    print_err(|| {
                        let e: RBoxErr<()> =
                            Err(CliBinFilesError::CreateDefaultSampleAttrUseOtherCommand.into());
                        e
                    });
                }
            }
        }
        SubCmds::HumanToBin {
            source_type,
            source_path,
            bin_type,
            bin_path,
        } => match bin_type {
            BinTypes::Arrangement => {
                print_err(|| {
                    convert_from_to::<ArrangementFile>(
                        ConvertFromToCmd::HumanToBin,
                        source_type,
                        source_path,
                        bin_path,
                    )
                });
            }
            BinTypes::Bank => {
                print_err(|| {
                    convert_from_to::<Bank>(
                        ConvertFromToCmd::HumanToBin,
                        source_type,
                        source_path,
                        bin_path,
                    )
                });
            }
            BinTypes::Project => {
                print_err(|| {
                    convert_from_to::<Project>(
                        ConvertFromToCmd::HumanToBin,
                        source_type,
                        source_path,
                        bin_path,
                    )
                });
            }
            BinTypes::SampleAttributes => {
                print_err(|| {
                    convert_from_to::<SampleAttributes>(
                        ConvertFromToCmd::HumanToBin,
                        source_type,
                        source_path,
                        bin_path,
                    )
                });
            }
        },
        SubCmds::BinToHuman {
            bin_type,
            bin_path,
            dest_type,
            dest_path,
        } => match bin_type {
            BinTypes::Arrangement => {
                print_err(|| {
                    convert_from_to::<ArrangementFile>(
                        ConvertFromToCmd::BinToHuman,
                        dest_type,
                        dest_path,
                        bin_path,
                    )
                });
            }
            BinTypes::Bank => {
                print_err(|| {
                    convert_from_to::<Bank>(
                        ConvertFromToCmd::BinToHuman,
                        dest_type,
                        dest_path,
                        bin_path,
                    )
                });
            }
            BinTypes::Project => {
                print_err(|| {
                    convert_from_to::<Project>(
                        ConvertFromToCmd::BinToHuman,
                        dest_type,
                        dest_path,
                        bin_path,
                    )
                });
            }
            BinTypes::SampleAttributes => {
                print_err(|| {
                    convert_from_to::<SampleAttributes>(
                        ConvertFromToCmd::BinToHuman,
                        dest_type,
                        dest_path,
                        bin_path,
                    )
                });
            }
        },
    }
}
