use ot_tools_lib::projects::Project;
use ot_tools_lib::read_type_from_bin_file;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("data/tests/blank-project/project.work");
    let project = read_type_from_bin_file::<Project>(&path).unwrap();

    println!(
        "project created with OS version: {:?}",
        project.metadata.os_version
    );
}
