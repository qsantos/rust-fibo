use fibo::{memoized, naive};

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
}
