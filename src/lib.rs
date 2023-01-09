#![feature(exclusive_range_pattern)]
#![feature(assert_matches)]
#![feature(test)]
#![feature(local_key_cell_methods)]
#![feature(async_closure)]

pub mod application;
pub mod emulator;
pub mod util;

#[cfg(test)]
pub mod test;
