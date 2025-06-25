pub mod domain;
pub mod mapping;
pub mod normalization;
pub mod punycode;
pub mod unicode;
pub mod unicode_tables;
pub mod validation;

pub use domain::{IdnaError, to_ascii, to_unicode};
pub use mapping::{ascii_map, map};
pub use normalization::normalize;
pub use punycode::{punycode_to_utf32, utf32_to_punycode, verify_punycode};
pub use unicode::{utf8_length_from_utf32, utf8_to_utf32, utf32_length_from_utf8, utf32_to_utf8};
pub use validation::{
    contains_forbidden_domain_code_point, is_ascii, is_label_valid, valid_name_code_point,
};
