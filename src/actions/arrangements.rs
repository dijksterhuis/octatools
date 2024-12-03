use std::path::{Path, PathBuf};

use crate::{actions::get_bytes_slice, RBoxErr};
use serde_octatrack::{
    arrangements::{ArrangementFile, ArrangementFileRawBytes},
    FromPath,
};

/// Show deserialised representation of an Arrangement for a given arrangement file at `path`
pub fn show_arrangement(path: &PathBuf) -> RBoxErr<()> {
    let b = ArrangementFile::from_path(path).expect("Could not load arrangement file");
    println!("{b:#?}");
    Ok(())
}

/// Show bytes output as u8 values for an Arrangement file located at `path`
pub fn show_arrangement_bytes(
    path: &PathBuf,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let bytes = get_bytes_slice(
        ArrangementFileRawBytes::from_path(path)
            .expect("Could not load arrangement file")
            .data
            .to_vec(),
        start_idx,
        len,
    );
    println!("{:#?}", bytes);
    Ok(())
}

/// Load Arrangement file data from a YAML file
pub fn load_arrangement(_yaml_path: &Path, _outfile: &Path) -> RBoxErr<()> {
    unimplemented!("Need to deal with intermediate struct conversions.")
}

/// Dump Arrangement file data to a YAML file
pub fn dump_arrangement(_path: &Path, _yaml_path: &Path) -> RBoxErr<()> {
    unimplemented!("Need to deal with intermediate struct conversions.")
}


mod test {
    use super::*;

    #[test]
    fn test_show_ok() {

        let fp = PathBuf::from("data/tests/blank-project/arr01.work");
        let r = show_arrangement(&fp);
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_first_all_bytes_ok() {

        let fp = PathBuf::from("data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(
            &fp, 
            &None, 
            &None,
        );
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_first_100_bytes_ok() {

        let fp = PathBuf::from("data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(
            &fp, 
            &Some(0), 
            &Some(100),
        );
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_1_byte_ok() {

        let fp = PathBuf::from("data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(
            &fp, 
            &Some(0), 
            &Some(1),
        );
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_100_bytes_offset_100_ok() {

        let fp = PathBuf::from("data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(
            &fp, 
            &Some(100), 
            &Some(100),
        );
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_maxlen_ok() {

        let fp = PathBuf::from("data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(
            &fp, 
            &Some(0), 
            &Some(11336),
        );
        assert!(r.is_ok())
    }
}