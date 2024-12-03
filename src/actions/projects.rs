use serde_octatrack::{projects::Project, FromPath, ToYamlFile};

use crate::{actions::load_from_yaml, RBoxErr};
use itertools::Itertools;
use std::path::{Path, PathBuf};

/// Show deserialised representation of a Project for a given project file at `path`
pub fn show_project(path: &PathBuf) -> RBoxErr<()> {
    let b = Project::from_path(path).expect("Could not load project file");
    println!("{b:#?}");
    Ok(())
}

/// List all the sample slots within an Octatrack Project, given a path to a Project data file
pub fn list_project_sample_slots(path: &PathBuf) -> RBoxErr<()> {
    for slot in Project::from_path(path)
        .expect("Could not load project file")
        .slots
        .iter()
        .sorted_by(|x, y| Ord::cmp(&x.slot_id, &y.slot_id))
    {
        println!("{:#?}", slot);
    }

    Ok(())
}

/// Load Project file data from a YAML file
pub fn load_project(yaml_path: &Path, outfile: &Path) -> RBoxErr<()> {
    load_from_yaml::<Project>(yaml_path, outfile)
}

/// Dump Project file data to a YAML file
pub fn dump_project(path: &Path, yaml_path: &Path) -> RBoxErr<()> {
    let b = Project::from_path(path).expect("Could not load project file");
    let _ = b.to_yaml(yaml_path);
    Ok(())
}




mod test {
    use super::*;

    #[test]
    fn test_show_ok() {

        let fp = PathBuf::from("data/tests/blank-project/project.work");
        let r = show_project(&fp);
        assert!(r.is_ok())
    }

    #[test]
    fn test_list_sample_slots_ok() {

        let fp = PathBuf::from("data/tests/blank-project/project.work");
        let r = list_project_sample_slots(&fp);
        assert!(r.is_ok())
    }

    #[test]
    fn test_load_project_ok() {
        let outfile = PathBuf::from("/tmp/octatools-actions-project-load-test-ok.work");
        let yaml = PathBuf::from("data/tests/projects/project.yaml");
        let r = load_project(&yaml, &outfile);

        let _ = std::fs::remove_file(&outfile);
        assert!(r.is_ok())
    }

    #[test]
    fn test_load_project_matches_blank() {
        let testfile = PathBuf::from("data/tests/projects/blank.work");
        let outfile = PathBuf::from("/tmp/octatools-actions-project-load-test-full.work");
        let yaml = PathBuf::from("data/tests/projects/project.yaml");

        let _ = load_project(&yaml, &outfile);

        let written = Project::from_path(&outfile).unwrap();
        let valid = Project::from_path(&testfile).unwrap();

        let _ = std::fs::remove_file(&outfile);
        assert_eq!(written, valid)
    }

    #[test]
    fn test_dump_project_ok() {
        let infile = PathBuf::from("data/tests/projects/blank.work");
        let outyaml = PathBuf::from("/tmp/project-test-dump-ok.yaml");
        let r = dump_project(&infile, &outyaml);

        let _ = std::fs::remove_file(&outyaml);
        assert!(r.is_ok())
    }

}
