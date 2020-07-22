extern crate lazy_static;
extern crate gio;

// To import all needed traits.

#[macro_use] extern crate log;

mod rustman;

pub fn main() -> Result<(), String> {
    return rustman::rustman();
}