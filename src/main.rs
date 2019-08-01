#[macro_use] extern crate lazy_static;

mod convert;
mod encoded_string;
mod io;
mod set_1_1;
mod set_1_2;
mod set_1_3;
mod set_1_4;
mod set_1_5;
mod set_1_6;
mod static_values;
mod tests;

fn main() {
    set_1_6::main();
}
