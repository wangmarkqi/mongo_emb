use super::rdb::Rdb;
use pyo3::exceptions::PyOSError;
use pyo3::{IntoPyObjectExt, prelude::*};
use std;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[pyclass]
pub struct PyRdb {
    inner: Arc<Mutex<Rdb>>,
}

#[pymethods]
impl PyRdb {
    #[new]
    fn new(dp: &str, tp: &str) -> PyResult<Self> {
        match Rdb::new(dp, tp) {
            Ok(db) => Ok(Self{
                inner: Arc::new(Mutex::new(db)),
            }),
            Err(e) => Err(PyOSError::new_err(e.to_string())),
        }
    }

    pub fn write(&self, k: &str, v: &str) -> PyResult<String> {
        let db = self.inner.lock().unwrap();
        let res = db.write(k, v);
        match res {
            Ok(res) => Ok("success".to_string()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error write db: {}",
                e
            ))),
        }
    }

    pub fn read(&self, k: &str) -> PyResult<HashMap<String, String>> {
        let db = self.inner.lock().unwrap();
        let res = db.read(k);
        match res {
            Ok(m) => Ok(m),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error read db : {}",
                e
            ))),
        }
    }
}
