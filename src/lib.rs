#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod mario;
mod falco;
mod statuslogger;
mod seph;
mod chrom;

#[skyline::main(name = "compiled_build")]
pub fn main() {
    //mario::install();
    //falco::install();
    //statuslogger::install();
    //seph::install();
    chrom::install();
}