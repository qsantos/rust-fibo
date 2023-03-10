use fibo::{dynamic_programming, iterative, logarithmic, memoized, naive};
use fibo::{dynamic_programming_mod, iterative_mod, logarithmic_mod, memoized_mod, naive_mod};

use num_bigint::BigInt;

#[test]
fn test_naive() {
    assert_eq!(naive(0), BigInt::from(0));
    assert_eq!(naive(1), BigInt::from(1));
    assert_eq!(naive(2), BigInt::from(1));
    assert_eq!(naive(3), BigInt::from(2));
    assert_eq!(naive(4), BigInt::from(3));
    assert_eq!(naive(5), BigInt::from(5));
    assert_eq!(naive(6), BigInt::from(8));
    assert_eq!(naive(7), BigInt::from(13));
    assert_eq!(naive(30), BigInt::from(832040));
}

#[test]
fn test_naive_mod() {
    assert_eq!(naive_mod(0, 10_000), 0);
    assert_eq!(naive_mod(1, 10_000), 1);
    assert_eq!(naive_mod(2, 10_000), 1);
    assert_eq!(naive_mod(3, 10_000), 2);
    assert_eq!(naive_mod(4, 10_000), 3);
    assert_eq!(naive_mod(5, 10_000), 5);
    assert_eq!(naive_mod(6, 10_000), 8);
    assert_eq!(naive_mod(7, 10_000), 13);
    assert_eq!(naive_mod(30, 10_000), 2040);
}

#[test]
fn test_memoized() {
    assert_eq!(memoized(0), BigInt::from(0));
    assert_eq!(memoized(1), BigInt::from(1));
    assert_eq!(memoized(2), BigInt::from(1));
    assert_eq!(memoized(3), BigInt::from(2));
    assert_eq!(memoized(4), BigInt::from(3));
    assert_eq!(memoized(5), BigInt::from(5));
    assert_eq!(memoized(6), BigInt::from(8));
    assert_eq!(memoized(7), BigInt::from(13));
    assert_eq!(memoized(30), BigInt::from(832040));
    assert_eq!(
        memoized(1000),
        BigInt::parse_bytes(
            b"434665576869374564356885276750406258025646605173717804024817290895365\
            55417949051890403879840079255169295922593080322634775209689623239873322\
            471161642996440906533187938298969649928516003704476137795166849228875",
            10,
        )
        .unwrap(),
    );
    assert_eq!(memoized(10_000) % 1_000_000, BigInt::from(366875));
}

#[test]
fn test_memoized_mod() {
    assert_eq!(memoized_mod(0, 10_000), 0);
    assert_eq!(memoized_mod(1, 10_000), 1);
    assert_eq!(memoized_mod(2, 10_000), 1);
    assert_eq!(memoized_mod(3, 10_000), 2);
    assert_eq!(memoized_mod(4, 10_000), 3);
    assert_eq!(memoized_mod(5, 10_000), 5);
    assert_eq!(memoized_mod(6, 10_000), 8);
    assert_eq!(memoized_mod(7, 10_000), 13);
    assert_eq!(memoized_mod(30, 10_000), 2040);
    assert_eq!(memoized_mod(1000, 1_000_000), 228875);
    assert_eq!(memoized_mod(10_000, 1_000_000), 366875);
}

#[test]
fn test_dynamic_programming() {
    assert_eq!(dynamic_programming(0), BigInt::from(0));
    assert_eq!(dynamic_programming(1), BigInt::from(1));
    assert_eq!(dynamic_programming(2), BigInt::from(1));
    assert_eq!(dynamic_programming(3), BigInt::from(2));
    assert_eq!(dynamic_programming(4), BigInt::from(3));
    assert_eq!(dynamic_programming(5), BigInt::from(5));
    assert_eq!(dynamic_programming(6), BigInt::from(8));
    assert_eq!(dynamic_programming(7), BigInt::from(13));
    assert_eq!(dynamic_programming(30), BigInt::from(832040));
    assert_eq!(
        dynamic_programming(1000),
        BigInt::parse_bytes(
            b"434665576869374564356885276750406258025646605173717804024817290895365\
            55417949051890403879840079255169295922593080322634775209689623239873322\
            471161642996440906533187938298969649928516003704476137795166849228875",
            10,
        )
        .unwrap(),
    );
    assert_eq!(
        dynamic_programming(100_000) % 1_000_000,
        BigInt::from(746875)
    );
}

#[test]
fn test_dynamic_programming_mod() {
    assert_eq!(dynamic_programming_mod(0, 10_000), 0);
    assert_eq!(dynamic_programming_mod(1, 10_000), 1);
    assert_eq!(dynamic_programming_mod(2, 10_000), 1);
    assert_eq!(dynamic_programming_mod(3, 10_000), 2);
    assert_eq!(dynamic_programming_mod(4, 10_000), 3);
    assert_eq!(dynamic_programming_mod(5, 10_000), 5);
    assert_eq!(dynamic_programming_mod(6, 10_000), 8);
    assert_eq!(dynamic_programming_mod(7, 10_000), 13);
    assert_eq!(dynamic_programming_mod(30, 10_000), 2040);
    assert_eq!(dynamic_programming_mod(1000, 1_000_000), 228875);
    assert_eq!(dynamic_programming_mod(1_000_000, 1_000_000), 546875);
}

#[test]
fn test_iterative() {
    assert_eq!(iterative(0), BigInt::from(0));
    assert_eq!(iterative(1), BigInt::from(1));
    assert_eq!(iterative(2), BigInt::from(1));
    assert_eq!(iterative(3), BigInt::from(2));
    assert_eq!(iterative(4), BigInt::from(3));
    assert_eq!(iterative(5), BigInt::from(5));
    assert_eq!(iterative(6), BigInt::from(8));
    assert_eq!(iterative(7), BigInt::from(13));
    assert_eq!(iterative(30), BigInt::from(832040));
    assert_eq!(
        iterative(1000),
        BigInt::parse_bytes(
            b"434665576869374564356885276750406258025646605173717804024817290895365\
            55417949051890403879840079255169295922593080322634775209689623239873322\
            471161642996440906533187938298969649928516003704476137795166849228875",
            10,
        )
        .unwrap(),
    );
    assert_eq!(iterative(1_000_000) % 1_000_000, BigInt::from(546875));
}

#[test]
fn test_iterative_mod() {
    assert_eq!(iterative_mod(0, 10_000), 0);
    assert_eq!(iterative_mod(1, 10_000), 1);
    assert_eq!(iterative_mod(2, 10_000), 1);
    assert_eq!(iterative_mod(3, 10_000), 2);
    assert_eq!(iterative_mod(4, 10_000), 3);
    assert_eq!(iterative_mod(5, 10_000), 5);
    assert_eq!(iterative_mod(6, 10_000), 8);
    assert_eq!(iterative_mod(7, 10_000), 13);
    assert_eq!(iterative_mod(30, 10_000), 2040);
    assert_eq!(iterative_mod(1000, 1_000_000), 228875);
    assert_eq!(iterative_mod(100_000_000, 1_000_000), 546875);
}

#[test]
fn test_logarithmic() {
    assert_eq!(logarithmic(0), BigInt::from(0));
    assert_eq!(logarithmic(1), BigInt::from(1));
    assert_eq!(logarithmic(2), BigInt::from(1));
    assert_eq!(logarithmic(3), BigInt::from(2));
    assert_eq!(logarithmic(4), BigInt::from(3));
    assert_eq!(logarithmic(5), BigInt::from(5));
    assert_eq!(logarithmic(6), BigInt::from(8));
    assert_eq!(logarithmic(7), BigInt::from(13));
    assert_eq!(logarithmic(30), BigInt::from(832040));
    assert_eq!(
        logarithmic(1000),
        BigInt::parse_bytes(
            b"434665576869374564356885276750406258025646605173717804024817290895365\
            55417949051890403879840079255169295922593080322634775209689623239873322\
            471161642996440906533187938298969649928516003704476137795166849228875",
            10,
        )
        .unwrap(),
    );
    assert_eq!(logarithmic(10_000_000) % 1_000_000, BigInt::from(546875));
}

#[test]
fn test_logarithmic_mod() {
    assert_eq!(logarithmic_mod(0, 10_000), 0);
    assert_eq!(logarithmic_mod(1, 10_000), 1);
    assert_eq!(logarithmic_mod(2, 10_000), 1);
    assert_eq!(logarithmic_mod(3, 10_000), 2);
    assert_eq!(logarithmic_mod(4, 10_000), 3);
    assert_eq!(logarithmic_mod(5, 10_000), 5);
    assert_eq!(logarithmic_mod(6, 10_000), 8);
    assert_eq!(logarithmic_mod(7, 10_000), 13);
    assert_eq!(logarithmic_mod(30, 10_000), 2040);
    assert_eq!(logarithmic_mod(1000, 1_000_000), 228875);
    assert_eq!(logarithmic_mod(1_000_000_000, 1_000_000), 546875);
}
