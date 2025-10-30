use pyo3::prelude::*;

mod mongo;
mod redb;

use redb::py_rdb::PyRdb;

use mongo::py_database::PyCollection;
use mongo::py_database::PyDatabase;

#[pymodule]
fn mongo_emb(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?);
    m.add_class::<PyDatabase>()?;

    m.add_class::<PyCollection>()?;
    m.add_class::<PyRdb>()?;

    Ok(())
}
