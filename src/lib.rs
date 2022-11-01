#![feature(slice_flatten)]
#![feature(arbitrary_enum_discriminant)]
#![feature(exclusive_range_pattern)]

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

pub mod app;
pub mod emulator;
pub mod util;
