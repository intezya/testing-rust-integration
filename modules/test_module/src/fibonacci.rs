use pyo3::prelude::pyfunction;
use pyo3::prelude::PyResult;


fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[pyfunction]
pub fn py_fibonacci(n: u32) -> PyResult<u32> {
    Ok(fibonacci(n))
}
