use pyo3::prelude::pyfunction;
use pyo3::prelude::PyResult;


fn factorial(n: u32) -> u32 {
    return (1..=n).product()
}


#[pyfunction]
pub fn py_factorial(n: u32) -> PyResult<u32> {
    Ok(factorial(n))
}
