extern crate libc;

pub mod add;

use crate::add::add_numbers;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn hello_world() -> * const libc::c_char {
    let a = "hello world";
    println!("{:?}", a);
    CString::new("hello world").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn flip(x: libc::c_int, y: libc::c_int) -> *const [libc::c_int; 2] {
    println!("flipping {:?} <-> {:?}", x, y);
    println!("fliped {:?} <-> {:?}", y, x);
    &[y, x]
}

#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> libc::c_int {
    add_numbers(x, y)
}