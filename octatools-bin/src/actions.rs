//! Module containing code related to running commands

pub mod arrangements;
pub mod banks;
pub mod drive;
pub mod parts;
pub mod patterns;
pub mod projects;
pub mod samples;

// TODO: T needs a Where and for a trait or something with a `data()` method to ensure the generic
//       function definition knows we can get the `data` field from it
// /// Show raw bytes representation of a binary data file of type `T` at `path`
// fn show_type_bytes<T>(
//     path: &PathBuf,
//     start_idx: &Option<usize>,
//     len: &Option<usize>,
// ) -> RBoxErr<()> {
//
//     let bytes = read_bin_file(path)?;
//
//     let raw_data = deserialize_bin_to_type::<T>(&bytes)?;
//
//
//     let bytes = get_bytes_slice(
//         raw_data.data.to_vec(),
//         start_idx,
//         len,
//     );
//     println!("{:#?}", bytes);
//     Ok(())
// }
//
