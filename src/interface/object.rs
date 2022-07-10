use pyo3::prelude::{pyfunction, PyResult, Python};
use pyo3::types::{PyAny, PyDict};
use pyo3::exceptions::PyLookupError;

use super::config::run_config;

#[pyfunction]
pub fn object_interface<'a>(input_object: &'a PyAny) -> PyResult<&'a PyAny> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    
    let config_dict: &PyDict = PyDict::new(py);

    match input_object.getattr("number") {
        Ok(data) => config_dict.set_item("number", data).unwrap(),
        Err(_) => Err(PyLookupError::new_err("attribute number is missing")).unwrap()
    }

    match input_object.getattr("numbers") {
        Ok(data) => config_dict.set_item("numbers", data).unwrap(),
        Err(_) => Err(PyLookupError::new_err("attribute numbers is missing")).unwrap()
    }
    let output_dict: &PyDict = run_config(config_dict).unwrap();

    input_object.setattr("number_results", output_dict.get_item("NUMBER RESULT").unwrap())?;
    input_object.setattr("numbers_results", output_dict.get_item("NUMBER RESULT").unwrap())?;

    Ok(input_object)
}