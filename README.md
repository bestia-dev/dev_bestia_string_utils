<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# dev_bestia_string_utils

[//]: # (auto_cargo_toml_to_md start)

**Library for string manipulation**  
***version: 0.1.19 date: 2021-10-23 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/dev_bestia_string_utils)***  

[//]: # (auto_cargo_toml_to_md end)

 ![work_in_progress](https://img.shields.io/badge/work_in_progress-yellow)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-102-green.svg)](https://github.com/bestia-dev/dev_bestia_string_utils/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-61-blue.svg)](https://github.com/bestia-dev/dev_bestia_string_utils/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-22-purple.svg)](https://github.com/bestia-dev/dev_bestia_string_utils/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/dev_bestia_string_utils/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/dev_bestia_string_utils/)

[//]: # (auto_lines_of_code end)

[//]: # (auto_badges start)

 [![crates.io](https://img.shields.io/crates/v/dev_bestia_string_utils.svg)](https://crates.io/crates/dev_bestia_string_utils)
 [![Documentation](https://docs.rs/dev_bestia_string_utils/badge.svg)](https://docs.rs/dev_bestia_string_utils/)
 [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/dev_bestia_string_utils.svg)](https://web.crev.dev/rust-reviews/crate/dev_bestia_string_utils/)
 [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/dev_bestia_string_utils/)
 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/dev_bestia_string_utils/blob/master/LICENSE)
 [![Rust](https://github.com/bestia-dev/dev_bestia_string_utils/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/dev_bestia_string_utils/actions)
 ![dev_bestia_string_utils](https://bestia.dev/webpage_hit_counter/get_svg_image/307799587.svg)

[//]: # (auto_badges end)

Hashtags: #rustlang #tutorial  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Motivation

There is always some functions around strings that are used very often.  
In Rust I use `x.to_string()` or `x.to_owned()` or `String::from(x)` or `format!("{}",x)` so much, that it deserves something shorter.  
For now my best bet is a macro `s!(x)`. I would much rather have a suffix macro like `x.s!()`, but that does not exist in Rust yet. It has a small probability that one day they will add it to Rust.  

## Development

I use [cargo-auto](https://crates.io/crates/cargo-auto) for my automation tasks like `cargo auto build` or `cargo auto doc`, ...

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
