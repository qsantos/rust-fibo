use fibo::naive;

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
