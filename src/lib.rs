use num_bigint::BigInt;

pub fn naive(n: u64) -> BigInt {
    match n {
        0 => BigInt::from(0),
        1 => BigInt::from(1),
        n => naive(n - 1) + naive(n - 2),
    }
}
