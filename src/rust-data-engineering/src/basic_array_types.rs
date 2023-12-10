/// The `basic_array_types` module provides functionality for demonstrating basic array types in Rust.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use basic_array_types::basic_array_types;
///
/// basic_array_types();
/// ```
///
/// # Note
///
/// This module uses the `std::collections` for HashMap and HashSet demonstrations.

pub fn basic_array_types() {
    println!("####### SEQUENCE #######");
    sequence();
    println!("####### HASH MAP #######");
    hash_map();
    println!("####### SETS #######");
    sets();
}

/// Demonstrates a sequence in Rust.
///
/// This function initializes a vector with the first two Fibonacci numbers,
/// and then generates the next eight numbers in the sequence.
///
/// # Examples
///
/// ```
/// use basic_array_types::sequence;
///
/// sequence();
/// ```

fn sequence() {
    let mut sequence = vec![1, 1];

    for i in 2..10 {
        let next = sequence[i - 1] + sequence[i - 2];
        sequence.push(next);
    }

    println!("The sequence is: {:?}", sequence);
}

/// Demonstrates a HashMap in Rust.
///
/// This function initializes a HashMap with two key-value pairs,
/// where the keys are strings and the values are integers.
///
/// # Examples
///
/// ```
/// use basic_array_types::hash_map;
///
/// hash_map();
/// ```

fn hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("The scores are: {:?}", scores);
}

/// Demonstrates a HashSet in Rust.
///
/// This function initializes a HashSet with five integers.
///
/// # Examples
///
/// ```
/// use basic_array_types::sets;
///
/// sets();
/// ```

fn sets() {
    use std::collections::HashSet;

    let mut set = HashSet::new();

    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);

    println!("The set is: {:?}", set);
}
