/// The `main` module is the entry point of the application.
///
/// It imports and uses the `basic_array_types` and `fruit_salad` modules.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use main::main;
///
/// main();
/// ```
///
/// # Note
///
/// This module uses the `basic_array_types` and `fruit_salad` modules for demonstrating basic array types and creating a random fruit salad respectively.
mod basic_array_types;
mod fruit_salad;

/// The main function of the application.
///
/// It calls the various function to test and/or demonstrate
/// the functionality of the application.

fn main() {
    basic_array_types::basic_array_types();
    fruit_salad::fruit_salad();
}
