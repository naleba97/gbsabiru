#[macro_use]
extern crate enum_primitive;
extern crate num;

mod core;

use crate::core::cpu::SharpCpu;
use crate::core::regfile::Register::*;

fn main() {
    let cpu: SharpCpu = SharpCpu::default();
    println!("Hello World!");
}
