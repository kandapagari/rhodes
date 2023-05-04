use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub data: HashMap<String, String>,
}

impl Config {
    pub fn from_file(file_path: &str) -> PyResult<Self> {
        let path = Path::new(file_path);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                return Err(pyo3::exceptions::PyIOError::new_err(format!(
                    "Failed to open file '{}': {}",
                    file_path, err
                )));
            }
        };

        let mut contents = String::new();
        if let Err(err) = file.read_to_string(&mut contents) {
            return Err(pyo3::exceptions::PyIOError::new_err(format!(
                "Failed to read file '{}': {}",
                file_path, err
            )));
        }

        match path.extension() {
            Some(extension) if extension == "yaml" || extension == "yml" => {
                serde_yaml::from_str(&contents).map_err(|err| {
                    pyo3::exceptions::PyValueError::new_err(format!(
                        "Failed to parse YAML file '{}': {}",
                        file_path, err
                    ))
                })
            }
            Some(extension) if extension == "json" => {
                serde_json::from_str(&contents).map_err(|err| {
                    pyo3::exceptions::PyValueError::new_err(format!(
                        "Failed to parse JSON file '{}': {}",
                        file_path, err
                    ))
                })
            }
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unsupported configuration file format for file '{}'",
                file_path
            ))),
        }
    }
}

#[pyfunction]
fn load(file_path: &str) -> PyResult<HashMap<String, String>> {
    let config = Config::from_file(file_path)?;
    Ok(config.data)
}

#[pymodule]
fn rhodes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;

    Ok(())
}
