#![feature(exclusive_range_pattern)]
#![feature(assert_matches)]

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

pub mod app;
pub mod emulator;
pub mod util;

#[cfg(test)]
pub mod test;
