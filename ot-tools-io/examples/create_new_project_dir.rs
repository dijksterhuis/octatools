use ot_tools_io::write_type_to_bin_file;
use ot_tools_io::{arrangements::ArrangementFile, banks::Bank, projects::Project};
use std::path::PathBuf;

fn bank_fname_from_id(id: usize) -> String {
    format!["bank{id:0>2}.work"].to_string()
}

fn arr_fname_from_id(id: usize) -> String {
    format!["arr{id:0>2}.work"].to_string()
}

fn main() {
    let project_dirpath = PathBuf::from("example-new-project");

    std::fs::create_dir_all(&project_dirpath).unwrap();

    let proj_fpath = project_dirpath.join("project.work");
    write_type_to_bin_file::<Project>(&Project::default(), &proj_fpath).unwrap();

    for i in 1..=16 {
        let bank_fpath = project_dirpath.join(bank_fname_from_id(i));
        write_type_to_bin_file::<Bank>(&Bank::default(), &bank_fpath).unwrap();
    }

    for i in 1..=8 {
        let arr_fpath = project_dirpath.join(arr_fname_from_id(i));
        write_type_to_bin_file::<ArrangementFile>(&ArrangementFile::default(), &arr_fpath).unwrap();
    }

    println!("New project created: {project_dirpath:?}");
}
