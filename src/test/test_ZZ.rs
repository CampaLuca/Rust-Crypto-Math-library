use crate::numbers::numbers::*;
use crate::numbers::classes::ZZ::*;
use crate::numbers::instances::ZZ_instance::*;
use num_bigint::BigInt;

pub fn test() {
    test_sum();
    test_sub();
    test_mul();
    test_div();
}


/*
    ZZ op ZZ
*/
fn test_sum() {
    let zz_class = ZZ::new();
    let v1: ZZinstance = zz_class.apply(BigInt::from(3));
    //let v2: ZZ_instance = zz_class.apply(BigInt::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: ZZinstance = zz_class.apply(BigInt::parse_bytes(b"13254435464566445343233245346146452342324398292378293724254895732984734987322498749837432493873249873498327498734983498743", 10).unwrap());
    let v3: ZZinstance = v1 + v2;
    let value = v3.value.clone();
    assert_eq!(value, BigInt::parse_bytes(b"13254435464566445343233245346146452342324398292378293724254895732984734987322498749837432493873249873498327498734983498746", 10).unwrap());
    println!("{}", value);

}

fn test_sub() {
    let zz_class = ZZ::new();
    let v1: ZZinstance = zz_class.apply(BigInt::from(2));
    let v2: ZZinstance = zz_class.apply(BigInt::from(3));
    let v3: ZZinstance = v1 - v2;
    let value = v3.value.clone();
    assert_eq!(value, BigInt::from(-1));
    println!("{}", value);

}

fn test_mul() {
    let zz_class = ZZ::new();

    let v1: ZZinstance = zz_class.apply(BigInt::from(2));
    let v2: ZZinstance = zz_class.apply(BigInt::from(-3));
    let v3: ZZinstance = v1 * v2;
    let value = v3.value.clone();
    assert_eq!(value, BigInt::from(-6));
    println!("{}", value);

}

fn test_div() {
    let zz_class = ZZ::new();

    let v1: ZZinstance = zz_class.apply(BigInt::from(3));
    let v2: ZZinstance = zz_class.apply(BigInt::from(2));
    let v3: ZZinstance = v1 / v2;
    let value = v3.value.clone();
    assert_eq!(value, BigInt::from(1));
    println!("{}", value);
}