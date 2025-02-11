use pyo3::prelude::{
    pyfunction, pymodule, wrap_pyfunction, Bound, PyModule, PyModuleMethods, PyResult,
};
use octatools_lib::{
    arrangements::ArrangementFile, banks::Bank, projects::Project, samples::SampleAttributes,
};
use octatools_lib::{
    deserialize_bin_to_type, deserialize_json_to_type, read_type_from_bin_file,
    serialize_json_from_type, write_type_to_bin_file, Encode,
};

use std::path::PathBuf;

// arrangements

#[pyfunction]
pub fn arrangement_bytes_to_json(bytes: Vec<u8>) -> PyResult<String> {
    let x = deserialize_bin_to_type::<ArrangementFile>(&bytes)
        .expect("Could not deserialize arrangement bytes.");
    let y = serialize_json_from_type::<ArrangementFile>(&x)
        .expect("Could not serialize arrangement to JSON.");

    Ok(y)
}

#[pyfunction]
pub fn arrangement_file_to_json(path: PathBuf) -> PyResult<String> {
    let x = read_type_from_bin_file::<ArrangementFile>(&path)
        .expect("Could not read arrangement file.");
    let y = serialize_json_from_type::<ArrangementFile>(&x)
        .expect("Could not serialize arrangement to JSON.");

    Ok(y)
}

#[pyfunction]
pub fn arrangement_json_to_bytes(json: String) -> PyResult<Vec<u8>> {
    let x = deserialize_json_to_type::<ArrangementFile>(&json)
        .expect("Could not deserialize JSON into arrangement.");
    let bytes = x
        .encode()
        .expect("Could not encode arrangement into bytes.");
    Ok(bytes)
}

#[pyfunction]
pub fn arrangement_json_to_file(json: String, path: PathBuf) -> PyResult<()> {
    let x = deserialize_json_to_type::<ArrangementFile>(&json)
        .expect("Could not deserialize JSON to arrangement.");
    let _ = write_type_to_bin_file::<ArrangementFile>(&x, &path)
        .expect("Could not write arrangement to file.");
    Ok(())
}

// banks

#[pyfunction]
pub fn bank_bytes_to_json(bytes: Vec<u8>) -> PyResult<String> {
    let x = deserialize_bin_to_type::<Bank>(&bytes).expect("Could not deserialize bank bytes.");
    let y = serialize_json_from_type::<Bank>(&x).expect("Could not serialize bank to JSON.");

    Ok(y)
}

#[pyfunction]
pub fn bank_file_to_json(path: PathBuf) -> PyResult<String> {
    let x = read_type_from_bin_file::<Bank>(&path).expect("Could not read bank file.");
    let y = serialize_json_from_type::<Bank>(&x).expect("Could not serialize bank to JSON.");

    Ok(y)
}

#[pyfunction]
pub fn bank_json_to_bytes(json: String) -> PyResult<Vec<u8>> {
    let x = deserialize_json_to_type::<Bank>(&json).expect("Could not deserialize JSON into bank.");
    let bytes = x.encode().expect("Could not encode bank into bytes.");
    Ok(bytes)
}

#[pyfunction]
pub fn bank_json_to_file(json: String, path: PathBuf) -> PyResult<()> {
    let x = deserialize_json_to_type::<Bank>(&json).expect("Could not deserialize JSON to bank.");
    let _ = write_type_to_bin_file::<Bank>(&x, &path).expect("Could not write bank to file.");
    Ok(())
}

// projects

#[pyfunction]
pub fn project_bytes_to_json(bytes: Vec<u8>) -> PyResult<String> {
    let x =
        deserialize_bin_to_type::<Project>(&bytes).expect("Could not deserialize project bytes.");
    let y = serialize_json_from_type::<Project>(&x).expect("Could not serialize project to JSON.");

    Ok(y)
}

#[pyfunction]
pub fn project_file_to_json(path: PathBuf) -> PyResult<String> {
    let x = read_type_from_bin_file::<Project>(&path).expect("Could not read project file.");
    let y = serialize_json_from_type::<Project>(&x).expect("Could not serialize project to JSON.");

    Ok(y)
}

#[pyfunction]
pub fn project_json_to_bytes(json: String) -> PyResult<Vec<u8>> {
    let x = deserialize_json_to_type::<Project>(&json)
        .expect("Could not deserialize JSON into project.");
    let bytes = x.encode().expect("Could not encode project into bytes.");
    Ok(bytes)
}

#[pyfunction]
pub fn project_json_to_file(json: String, path: PathBuf) -> PyResult<()> {
    let x =
        deserialize_json_to_type::<Project>(&json).expect("Could not deserialize JSON to project.");
    let _ = write_type_to_bin_file::<Project>(&x, &path).expect("Could not write project to file.");
    Ok(())
}

// samples

#[pyfunction]
pub fn sample_attributes_bytes_to_json(bytes: Vec<u8>) -> PyResult<String> {
    let x = deserialize_bin_to_type::<SampleAttributes>(&bytes)
        .expect("Could not deserialize sample attributes bytes.");
    let y = serialize_json_from_type::<SampleAttributes>(&x)
        .expect("Could not serialize sample attributes to JSON.");

    Ok(y)
}

#[pyfunction]
pub fn sample_attributes_file_to_json(path: PathBuf) -> PyResult<String> {
    let x = read_type_from_bin_file::<SampleAttributes>(&path)
        .expect("Could not read sample attributes file.");
    let y = serialize_json_from_type::<SampleAttributes>(&x)
        .expect("Could not serialize sample attributes to JSON.");

    Ok(y)
}

#[pyfunction]
pub fn sample_attributes_json_to_bytes(json: String) -> PyResult<Vec<u8>> {
    let x = deserialize_json_to_type::<SampleAttributes>(&json)
        .expect("Could not deserialize JSON into sample attributes.");
    let bytes = x
        .encode()
        .expect("Could not encode sample attributes into bytes.");
    Ok(bytes)
}

#[pyfunction]
pub fn sample_attributes_json_to_file(json: String, path: PathBuf) -> PyResult<()> {
    let x = deserialize_json_to_type::<SampleAttributes>(&json)
        .expect("Could not deserialize JSON to sample attributes.");
    let _ = write_type_to_bin_file::<SampleAttributes>(&x, &path)
        .expect("Could not write sample attributes to file.");
    Ok(())
}

#[pymodule]
fn octatools_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(arrangement_bytes_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(arrangement_file_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(arrangement_json_to_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(arrangement_json_to_file, m)?)?;

    m.add_function(wrap_pyfunction!(bank_bytes_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(bank_file_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(bank_json_to_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(bank_json_to_file, m)?)?;

    m.add_function(wrap_pyfunction!(project_bytes_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(project_file_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(project_json_to_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(project_json_to_file, m)?)?;

    m.add_function(wrap_pyfunction!(sample_attributes_bytes_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(sample_attributes_file_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(sample_attributes_json_to_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(sample_attributes_json_to_file, m)?)?;

    Ok(())
}
