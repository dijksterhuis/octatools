use std::path::PathBuf;

pub(crate) fn get_blank_proj_dirpath() -> PathBuf {
    PathBuf::from("..")
        .join("data")
        .join("tests")
        .join("blank-project")
}

pub(crate) fn get_misc_dirpath() -> PathBuf {
    PathBuf::from("..").join("data").join("tests").join("misc")
}

pub(crate) fn get_arrange_dirpath() -> PathBuf {
    PathBuf::from("..")
        .join("data")
        .join("tests")
        .join("arrange")
}

pub(crate) fn get_bank_dirpath() -> PathBuf {
    PathBuf::from("..").join("data").join("tests").join("bank")
}

pub(crate) fn get_samples_dirpath() -> PathBuf {
    PathBuf::from("..")
        .join("data")
        .join("tests")
        .join("samples")
}

pub(crate) fn get_project_dirpath() -> PathBuf {
    PathBuf::from("..")
        .join("data")
        .join("tests")
        .join("projects")
}
