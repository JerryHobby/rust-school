/// The `main` module is the entry point of the application.
///
/// It imports and uses various demo modules.
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
/// This module uses various demo modules for demonstrating basic array types.
mod basic_array_types;
mod fruit_salad_vec;
mod fruit_salad_vecdeque;
mod fruit_salad_linked_list;
mod frequency_counter;

/// The main function of the application.
///
/// It calls the various function to test and/or demonstrate
/// the functionality of the application.

fn main() {
    basic_array_types::basic_array_types();
    fruit_salad_vec::fruit_salad_vec();
    fruit_salad_vecdeque::fruit_salad_vecdeque();
    fruit_salad_linked_list::fruit_salad_linked_list();
    frequency_counter::frequency_counter();


}
