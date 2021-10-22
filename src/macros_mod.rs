// macros_mod.rs

/// short macro `s!()` instead of `&str.to_string` or `format!()`, because that is so common an verbose.  
/// Equivalents: `String::new()`, `x.to_string()`, `x.to_owned()`, `format!()`...  
#[macro_export]
macro_rules! s {
    () => {
        String::new()
    };
    ($my_str: expr) => {
        $my_str.to_string()
    };
    ($literal: expr, $str_1: expr) => {
        format!($literal, $str_1)
    };
    ($literal: expr, $str_1: expr, $str_2: expr) => {
        format!($literal, $str_1, $str_2)
    };
    ($literal: expr, $str_1: expr, $str_2: expr, $str_3: expr) => {
        format!($literal, $str_1, $str_2, $str_3)
    };
    ($literal: expr, $str_1: expr, $str_2: expr, $str_3: expr, $str_4: expr) => {
        format!($literal, $str_1, $str_2, $str_3, $str_4)
    };
}
