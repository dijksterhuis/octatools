use ot_tools_io::banks::Bank;
use ot_tools_io::read_type_from_bin_file;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("data/tests/blank-project/bank01.work");
    let bank = read_type_from_bin_file::<Bank>(&path).unwrap();

    println!("bank header: {:?}", bank.header_data);
}
