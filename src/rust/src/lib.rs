use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn greeting_n_times() -> String {
    let n = 10;
    (0..n).map(|_| "Hello world!\n").collect::<String>()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rpackageUsingRust;
    fn greeting_n_times;
}
