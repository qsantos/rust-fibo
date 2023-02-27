use memoize::memoize;
use num_bigint::BigInt;

pub fn naive(n: u64) -> BigInt {
    match n {
        0 => BigInt::from(0),
        1 => BigInt::from(1),
        n => naive(n - 1) + naive(n - 2),
    }
}

pub fn naive_mod(n: u64, m: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => (naive_mod(n - 1, m) + naive_mod(n - 2, m)) % m,
    }
}

#[memoize]
pub fn memoized(n: u64) -> BigInt {
    match n {
        0 => BigInt::from(0),
        1 => BigInt::from(1),
        n => memoized(n - 1) + memoized(n - 2),
    }
}

#[memoize]
pub fn memoized_mod(n: u64, m: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => (memoized_mod(n - 1, m) + memoized_mod(n - 2, m)) % m,
    }
}

pub fn dynamic_programming(n: u64) -> BigInt {
    let n = n as usize;
    let mut v = vec![BigInt::from(0), BigInt::from(1)];
    for i in 2..=n {
        v.push(&v[i - 2] + &v[i - 1]);
    }
    std::mem::take(&mut v[n])
}

pub fn dynamic_programming_mod(n: u64, m: u64) -> u64 {
    let n = n as usize;
    let mut v = vec![0, 1];
    for i in 2..=n {
        v.push((v[i - 2] + v[i - 1]) % m);
    }
    v[n]
}

pub fn iterative(n: u64) -> BigInt {
    let mut u = BigInt::from(0);
    let mut v = BigInt::from(1);
    for _ in 0..n {
        (u, v) = (&u + v, u);
    }
    u
}

pub fn iterative_mod(n: u64, m: u64) -> u64 {
    let mut u = 0;
    let mut v = 1;
    for _ in 0..n {
        (u, v) = ((u + v) % m, u);
    }
    u
}

pub fn logarithmic(n: u64) -> BigInt {
    if n == 0 {
        BigInt::from(0)
    } else if n == 1 {
        BigInt::from(1)
    } else if n % 2 == 0 {
        // n = 2k
        // F_n = (2 × F_{k-1} + F_k) × F_k
        let k = n / 2;
        let f_k = logarithmic(k);
        let f_km1 = logarithmic(k - 1);
        (2 * f_km1 + &f_k) * f_k
    } else {
        // n = 2k + 1
        // F_n = F_k² + F_{k+1}^2
        let k = n / 2;
        let f_k = logarithmic(k);
        let f_km1 = logarithmic(k + 1);
        f_k.pow(2) + f_km1.pow(2)
    }
}

pub fn logarithmic_mod(n: u64, m: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else if n % 2 == 0 {
        // n = 2k
        // F_n = (2 × F_{k-1} + F_k) × F_k
        let k = n / 2;
        let f_k = logarithmic_mod(k, m);
        let f_km1 = logarithmic_mod(k - 1, m);
        ((2 * f_km1 + f_k) * f_k) % m
    } else {
        // n = 2k + 1
        // F_n = F_k² + F_{k+1}^2
        let k = n / 2;
        let f_k = logarithmic_mod(k, m);
        let f_km1 = logarithmic_mod(k + 1, m);
        (f_k * f_k + f_km1 * f_km1) % m
    }
}
