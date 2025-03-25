use ot_tools_io::arrangements::ArrangementFile;
use ot_tools_io::read_type_from_bin_file;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("data/tests/blank-project/arr01.work");
    let arr = read_type_from_bin_file::<ArrangementFile>(&path).unwrap();

    println!("arrangement header: {:?}", arr.header);
}
