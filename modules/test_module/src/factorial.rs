use num_bigint::BigUint;


pub fn factorial(n: u32) -> BigUint {
    return (1..=n).product()
}