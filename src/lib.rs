use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rustc_demangle;

/// De-mangles a Rust symbol.
#[pyfunction]
fn demangle(a: &str) -> PyResult<String> {
    Ok(rustc_demangle::demangle(a).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_demangle(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(demangle, m)?)?;

    Ok(())
}
