#[allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]
pub mod conversions;
#[allow(
    unused_variables,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    clippy::module_name_repetitions,
    // needed to pull inferred return type down to ()
    clippy::semicolon_if_nothing_returned, clippy::unit_arg,
    clippy::wildcard_imports,
    unused_mut,

)]
pub mod gl_core;
#[allow(non_snake_case, non_upper_case_globals, clippy::upper_case_acronyms)]
pub mod gl_types;
