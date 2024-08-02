use num_bigint::BigUint;
use pyo3::{pyfunction, PyResult};

fn factorial(n: u32) -> BigUint {
    return (1..=n).product()
}


#[pyfunction]
pub fn py_factorial(n: u32) -> PyResult<String> {
    let result = factorial(n);
    Ok(result.to_str_radix(36))
}