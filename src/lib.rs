// dev_bestia_string_utils lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # dev_bestia_string_utils
//!
//! **Library for string manipulation**  
//! ***[repository](https://github.com/lucianobestia/dev_bestia_string_utils); version: 0.1.19  date: 2021-10-23 authors: Luciano Bestia***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-102-green.svg)](https://github.com/LucianoBestia/dev_bestia_string_utils/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-61-blue.svg)](https://github.com/LucianoBestia/dev_bestia_string_utils/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-22-purple.svg)](https://github.com/LucianoBestia/dev_bestia_string_utils/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/dev_bestia_string_utils/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/dev_bestia_string_utils/)
//!
//! [![crates.io](https://img.shields.io/crates/v/dev_bestia_string_utils.svg)](https://crates.io/crates/dev_bestia_string_utils) [![Documentation](https://docs.rs/dev_bestia_string_utils/badge.svg)](https://docs.rs/dev_bestia_string_utils/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/dev_bestia_string_utils.svg)](https://web.crev.dev/rust-reviews/crate/dev_bestia_string_utils/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/dev_bestia_string_utils/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/dev_bestia_string_utils/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/dev_bestia_string_utils/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/dev_bestia_string_utils/)  
//!
//! ## Motivation
//!
//! There is always some functions around strings that are used very often.  
//! In Rust I use `x.to_string()` or `x.to_owned()` or `String::from(x)` or `format!("{}",x)` so much, that it deserves something shorter.  
//! For now my best bet is a macro `s!(x)`. I would much rather have a suffix macro like `x.s!()`, but that does not exist in Rust yet. It has a small probability that one day they will add it to Rust.  
//!
//! ## Development
//!
//! I use [cargo-auto](https://crates.io/crates/cargo-auto) for my automation tasks like `cargo auto build` or `cargo auto doc`, ...
//!
//! ## cargo crev reviews and advisory
//!
//! We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! You can also read reviews quickly on the web:  
//! <https://web.crev.dev/rust-reviews/crates/>  
//!
//! ## open-source free and free as a beer
//!
//! My open-source projects are free and free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful,  
//! please buy me a beer donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// internal private modules
mod macros_mod;
mod string_utils_mod;

// region: public interface

// export/re-export public functions, traits,...

// the macro `s!` is automatically exported at the root of the crate

pub use string_utils_mod::*;

// endregion: public interface
