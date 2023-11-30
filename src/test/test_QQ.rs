
use crate::numbers::classes::QQ::*;
use crate::numbers::instances::QQ_instance::*;
use crate::numbers::numbers::Class;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
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
    let qq_class: QQ = QQ::new();
    let v1: QQinstance = qq_class.new_instance(BigInt::from(3),BigInt::from(4));
    //let v2: QQ_instance = QQ_class.new_instance(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: QQinstance = qq_class.new_instance(BigInt::from(5),BigInt::from(6));
    let v3: QQinstance = v1 + v2;
    
    println!("{0}/{1}", v3.numerator.clone() , v3.denominator.clone());

    let ff = qq_class.apply(BigDecimal::from_str("1.5").unwrap());
    println!("{0}/{1}", ff.numerator, ff.denominator);
   
}

fn test_sub() {
    let qq_class: QQ = QQ::new();

    let v1: QQinstance = qq_class.new_instance(BigInt::from(3),BigInt::from(4));
    //let v2: QQ_instance = QQ_class.new_instance(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: QQinstance = qq_class.new_instance(BigInt::from(5),BigInt::from(6));
    let v3: QQinstance = v1 - v2;
    
    println!("{0}/{1}", v3.numerator.clone() , v3.denominator.clone());

    let ff = qq_class.apply(BigDecimal::from_str("1.5").unwrap());
    println!("{0}/{1}", ff.numerator, ff.denominator);
   
}

fn test_mul() {
    let qq_class: QQ = QQ::new();

    let v1: QQinstance = qq_class.new_instance(BigInt::from(3),BigInt::from(4));
    //let v2: QQ_instance = QQ_class.new_instance(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: QQinstance = qq_class.new_instance(BigInt::from(5),BigInt::from(6));
    let v3: QQinstance = v1 * v2;
    
    println!("{0}/{1}", v3.numerator.clone() , v3.denominator.clone());

    let ff = qq_class.apply(BigDecimal::from_str("1.5").unwrap());
    println!("{0}/{1}", ff.numerator, ff.denominator);
   
}

fn test_div() {
    let qq_class: QQ = QQ::new();

    let v1: QQinstance = qq_class.new_instance(BigInt::from(3),BigInt::from(4));
    //let v2: QQ_instance = QQ_class.new_instance(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: QQinstance = qq_class.new_instance(BigInt::from(5),BigInt::from(6));
    let v3: QQinstance = v1 / v2;
    
    println!("{0}/{1}", v3.numerator.clone() , v3.denominator.clone());

    let ff = qq_class.apply(BigDecimal::from_str("1.5").unwrap());
    println!("{0}/{1}", ff.numerator, ff.denominator);
   
}