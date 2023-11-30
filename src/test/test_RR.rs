use crate::numbers::numbers::*;
use crate::numbers::classes::RR::*;
use crate::numbers::instances::RR_instance::*;
use bigdecimal::BigDecimal;
use std::str::FromStr;

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
    let rr_class: RR = RR::new();
    let v1: RRinstance = rr_class.apply(BigDecimal::from_str("123.45").unwrap());
    //let v2: RR_instance = rr_class.apply(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: RRinstance = rr_class.apply(BigDecimal::from_str("123.46").unwrap());
    let v3: RRinstance = v1 + v2;
    let value = v3.value.clone();
    assert_eq!(value, BigDecimal::from_str("246.91").unwrap());
}

fn test_sub() {
    let rr_class: RR = RR::new();

    let v1: RRinstance = rr_class.apply(BigDecimal::from_str("2.56").unwrap());
    let v2: RRinstance = rr_class.apply(BigDecimal::from_str("343565246334.32").unwrap());
    let v3: RRinstance = v1 - v2;
    let value = v3.value.clone();
    assert_eq!(value, BigDecimal::from_str("-343565246331.76").unwrap());
}

fn test_mul() {
    let rr_class: RR = RR::new();

    let v1: RRinstance = rr_class.apply(BigDecimal::from(2));
    let v2: RRinstance = rr_class.apply(BigDecimal::from(-3));
    let v3: RRinstance = v1 * v2;
    let value = v3.value.clone();
    assert_eq!(value, BigDecimal::from(-6));
}

fn test_div() {
    let rr_class: RR = RR::new();

    let v1: RRinstance = rr_class.apply(BigDecimal::from(3));
    let v2: RRinstance = rr_class.apply(BigDecimal::from(10));
    let v3: RRinstance = v1 / v2;
    let value = v3.value.clone();
    println!("{} is the result of 1/3", value);
    //assert_eq!(value, BigDecimal::from_str("1.5").unwrap());
}