// dev_bestia_string_utils lib.rs

#![doc=include_str!("../README.md")]

// internal private modules
mod macros_mod;
mod string_utils_mod;

// region: public interface

// export/re-export public functions, traits,...

// the macro `s!` is automatically exported at the root of the crate

pub use string_utils_mod::*;

// endregion: public interface
