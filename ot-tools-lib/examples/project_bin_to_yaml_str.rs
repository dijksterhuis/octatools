use ot_tools_lib::projects::Project;
use ot_tools_lib::{read_type_from_bin_file, serialize_yaml_from_type};
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("data/tests/blank-project/project.work");

    let project_bin = read_type_from_bin_file::<Project>(&path).unwrap();
    let project_yaml = serialize_yaml_from_type::<Project>(&project_bin).unwrap();

    println!("project as yaml: {:?}", project_yaml);
}
