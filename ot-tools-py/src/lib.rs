use ot_tools_io::{
    arrangements::ArrangementFile, banks::Bank, projects::Project, samples::SampleAttributes,
};
use ot_tools_io::{
    deserialize_bin_to_type, deserialize_json_to_type, read_type_from_bin_file,
    serialize_json_from_type, write_type_to_bin_file, Encode,
};
use ot_tools_ops::actions::banks::{batch_copy_banks, copy_bank_by_paths};
use ot_tools_ops::actions::samples::{
    batch_create_samplechains, create_equally_sliced_sample, create_randomly_sliced_sample,
    deconstruct_samplechain_from_paths,
};

use ot_tools_ops::actions::projects::slots::cmd_slots_deduplicate;
use ot_tools_ops::actions::projects::{
    consolidate_sample_slots_to_audio_pool, consolidate_sample_slots_to_project_pool,
    purge_project_pool,
};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::{
    pyfunction, pymodule, wrap_pyfunction, Bound, PyModule, PyModuleMethods, PyResult,
};
use pyo3::wrap_pymodule;
use std::path::PathBuf;
// arrangements

#[pyfunction]
pub fn arrangement_bytes_to_json(bytes: Vec<u8>) -> PyResult<String> {
    let x = deserialize_bin_to_type::<ArrangementFile>(&bytes)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let y = serialize_json_from_type::<ArrangementFile>(&x)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(y)
}

#[pyfunction]
pub fn arrangement_file_to_json(path: PathBuf) -> PyResult<String> {
    let x = read_type_from_bin_file::<ArrangementFile>(&path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let y = serialize_json_from_type::<ArrangementFile>(&x)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(y)
}

#[pyfunction]
pub fn arrangement_json_to_bytes(json: String) -> PyResult<Vec<u8>> {
    let x = deserialize_json_to_type::<ArrangementFile>(&json)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let bytes = x
        .encode()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(bytes)
}

#[pyfunction]
pub fn arrangement_json_to_file(json: String, path: PathBuf) -> PyResult<()> {
    let x = deserialize_json_to_type::<ArrangementFile>(&json)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    write_type_to_bin_file::<ArrangementFile>(&x, &path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

// banks

#[pyfunction]
pub fn bank_bytes_to_json(bytes: Vec<u8>) -> PyResult<String> {
    let x = deserialize_bin_to_type::<Bank>(&bytes)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let y =
        serialize_json_from_type::<Bank>(&x).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(y)
}

#[pyfunction]
pub fn bank_file_to_json(path: PathBuf) -> PyResult<String> {
    let x = read_type_from_bin_file::<Bank>(&path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let y =
        serialize_json_from_type::<Bank>(&x).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(y)
}

#[pyfunction]
pub fn bank_json_to_bytes(json: String) -> PyResult<Vec<u8>> {
    let x = deserialize_json_to_type::<Bank>(&json)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let bytes = x
        .encode()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(bytes)
}

#[pyfunction]
pub fn bank_json_to_file(json: String, path: PathBuf) -> PyResult<()> {
    let x = deserialize_json_to_type::<Bank>(&json)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    write_type_to_bin_file::<Bank>(&x, &path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

// projects

#[pyfunction]
pub fn project_bytes_to_json(bytes: Vec<u8>) -> PyResult<String> {
    let x = deserialize_bin_to_type::<Project>(&bytes)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let y = serialize_json_from_type::<Project>(&x)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(y)
}

#[pyfunction]
pub fn project_file_to_json(path: PathBuf) -> PyResult<String> {
    let x = read_type_from_bin_file::<Project>(&path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let y = serialize_json_from_type::<Project>(&x)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(y)
}

#[pyfunction]
pub fn project_json_to_bytes(json: String) -> PyResult<Vec<u8>> {
    let x = deserialize_json_to_type::<Project>(&json)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let bytes = x
        .encode()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(bytes)
}

#[pyfunction]
pub fn project_json_to_file(json: String, path: PathBuf) -> PyResult<()> {
    let x = deserialize_json_to_type::<Project>(&json)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    write_type_to_bin_file::<Project>(&x, &path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

// samples

#[pyfunction]
pub fn sample_attributes_bytes_to_json(bytes: Vec<u8>) -> PyResult<String> {
    let x = deserialize_bin_to_type::<SampleAttributes>(&bytes)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let y = serialize_json_from_type::<SampleAttributes>(&x)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(y)
}

#[pyfunction]
pub fn sample_attributes_file_to_json(path: PathBuf) -> PyResult<String> {
    let x = read_type_from_bin_file::<SampleAttributes>(&path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let y = serialize_json_from_type::<SampleAttributes>(&x)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(y)
}

#[pyfunction]
pub fn sample_attributes_json_to_bytes(json: String) -> PyResult<Vec<u8>> {
    let x = deserialize_json_to_type::<SampleAttributes>(&json)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let bytes = x
        .encode()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(bytes)
}

#[pyfunction]
pub fn sample_attributes_json_to_file(json: String, path: PathBuf) -> PyResult<()> {
    let x = deserialize_json_to_type::<SampleAttributes>(&json)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    write_type_to_bin_file::<SampleAttributes>(&x, &path)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pymodule]
fn binfiles(m: &Bound<'_, PyModule>) -> PyResult<()> {
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

#[pyfunction]
pub fn copy_bank(
    src_project_dirpath: PathBuf,
    src_bank_id: usize,
    dest_project_dirpath: PathBuf,
    dest_bank_id: usize,
    force: bool,
) -> PyResult<()> {
    copy_bank_by_paths(
        &src_project_dirpath,
        &dest_project_dirpath,
        src_bank_id,
        dest_bank_id,
        force,
    )
    .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pyfunction]
pub fn copy_bank_yaml(yaml_conf_fpath: PathBuf) -> PyResult<()> {
    batch_copy_banks(&yaml_conf_fpath).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pyfunction]
pub fn project_samples_consolidate(project_dirpath: PathBuf) -> PyResult<()> {
    consolidate_sample_slots_to_project_pool(&project_dirpath)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pyfunction]
pub fn project_samples_centralize(project_dirpath: PathBuf) -> PyResult<()> {
    consolidate_sample_slots_to_audio_pool(&project_dirpath)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pyfunction]
pub fn sample_slots_purge(project_dirpath: PathBuf) -> PyResult<()> {
    purge_project_pool(&project_dirpath).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pyfunction]
pub fn sample_slots_deduplicate(project_dirpath: PathBuf) -> PyResult<()> {
    cmd_slots_deduplicate(&project_dirpath).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

/// Simple interface for creating a sample chain.
///
/// Parameters:
///   - wav_fps: list: wav file paths to chain together
///   - out_dirpath: str: path to output directory
///   - out_chainame: str: base name of the generated chain (will be suffixed with numbers based on number of wav files)
#[pyfunction]
pub fn create_sample_chain(
    wav_fps: Vec<PathBuf>,
    out_dirpath: PathBuf,
    out_chainname: String,
) -> PyResult<()> {
    batch_create_samplechains(&wav_fps, &out_dirpath, &out_chainname, None, None, None)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pyfunction]
pub fn split_sample_by_slices(
    wav_fp: PathBuf,
    ot_fp: PathBuf,
    out_dirpath: PathBuf,
) -> PyResult<()> {
    deconstruct_samplechain_from_paths(&wav_fp, &ot_fp, &out_dirpath)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pyfunction]
pub fn randomly_slice_sample(wav_fp: PathBuf, n_slices: usize) -> PyResult<()> {
    create_randomly_sliced_sample(&wav_fp, n_slices)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pyfunction]
pub fn linearly_slice_sample(wav_fp: PathBuf, n_slices: usize) -> PyResult<()> {
    create_equally_sliced_sample(&wav_fp, n_slices)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pymodule]
fn copy_ops(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(copy_bank, m)?)?;
    m.add_function(wrap_pyfunction!(copy_bank_yaml, m)?)?;
    Ok(())
}

// not in use
#[pymodule]
fn project_slot_ops(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sample_slots_deduplicate, m)?)?;
    m.add_function(wrap_pyfunction!(sample_slots_purge, m)?)?;
    Ok(())
}

// not in use
#[pymodule]
fn project_sample_ops(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(project_samples_centralize, m)?)?;
    m.add_function(wrap_pyfunction!(project_samples_consolidate, m)?)?;
    Ok(())
}

#[pymodule]
fn sample_files(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(linearly_slice_sample, m)?)?;
    m.add_function(wrap_pyfunction!(randomly_slice_sample, m)?)?;
    m.add_function(wrap_pyfunction!(split_sample_by_slices, m)?)?;
    m.add_function(wrap_pyfunction!(create_sample_chain, m)?)?;
    Ok(())
}

#[pymodule]
fn operations(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(copy_ops))?;
    // m.add_wrapped(wrap_pymodule!(project_slot_ops))?;
    // m.add_wrapped(wrap_pymodule!(project_sample_ops))?;
    Ok(())
}

#[pymodule]
fn ot_tools_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(binfiles))?;
    m.add_wrapped(wrap_pymodule!(operations))?;
    m.add_wrapped(wrap_pymodule!(sample_files))?;

    Ok(())
}
