use std::path::PathBuf;

use serde_octatrack::{get_bytes_slice, read_type_from_bin_file};

use crate::RBoxErr;
use serde_octatrack::arrangements::ArrangementFileRawBytes;

/// Show bytes output as u8 values for an Arrangement file located at `path`
pub fn show_arrangement_bytes(
    path: &PathBuf,
    start_idx: &Option<usize>,
    len: &Option<usize>,
) -> RBoxErr<()> {
    let raw: ArrangementFileRawBytes = read_type_from_bin_file::<ArrangementFileRawBytes>(&path)
        .expect("Could not read arrangement file");

    let bytes = get_bytes_slice(raw.data.to_vec(), start_idx, len);
    println!("{:#?}", bytes);
    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_show_bytes_first_all_bytes_ok() {
        let fp = PathBuf::from("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(&fp, &None, &None);
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_first_100_bytes_ok() {
        let fp = PathBuf::from("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(&fp, &Some(0), &Some(100));
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_1_byte_ok() {
        let fp = PathBuf::from("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(&fp, &Some(0), &Some(1));
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_100_bytes_offset_100_ok() {
        let fp = PathBuf::from("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(&fp, &Some(100), &Some(100));
        assert!(r.is_ok())
    }

    #[test]
    fn test_show_bytes_maxlen_ok() {
        let fp = PathBuf::from("../data/tests/blank-project/arr01.work");
        let r = show_arrangement_bytes(&fp, &Some(0), &Some(11336));
        assert!(r.is_ok())
    }
}
