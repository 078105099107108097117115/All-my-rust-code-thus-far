#![allow(dead_code)]

use std::borrow::Cow;

fn triangle(num : i32) -> i32 {
    (1..=num).fold(1, | prod, elem | prod * elem)
}

fn main() {
    //let res = triangle(100);
    //println!("product =={}", res);
    let s = "Hello world!";
    let cow : Cow<str> = Cow::Owned(String::from(s));

}
