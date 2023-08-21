#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod mario;
mod falco;
mod custom;
mod seph;

#[skyline::main(name = "compiled_build")]
pub fn main() {
    //mario::install();
    //falco::install();
    //custom::install();
    seph::install();
}