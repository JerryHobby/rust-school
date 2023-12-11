// frequency_counter.rs
use std::collections::HashMap;
use std::collections::HashSet;
/// The `frequency_counter` module provides functionality for counting the frequency of items in a list.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use frequency_counter::frequency_counter;
///
/// frequency_counter();
/// ```
///
/// # Note
///
/// This module uses the `std::collections` for HashMap and HashSet.

/// Counts the frequency of items in a list and prints a sorted frequency map.
///
/// This function initializes a frequency map with the count of each item in the `items` list,
/// and then creates a sorted frequency map that is sorted in the order of `items` and does not contain duplicates.

pub fn frequency_counter() {
    let items = vec![9, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3, 5];
    let frequency_map = counter_logic(&items);

    let mut sorted_frequency_map = Vec::new();
    let mut seen = HashSet::new();

    for &item in &items {
        if !seen.contains(&item) {
            if let Some(&frequency) = frequency_map.get(&item) {
                sorted_frequency_map.push((item, frequency));
                seen.insert(item);
            }
        }
    }

    println!("{:?}", sorted_frequency_map);
}

/// Counts the frequency of items in a list.
///
/// This function initializes a frequency map with the count of each item in the `items` list.
///
/// # Examples
///
/// ```
/// use frequency_counter::counter_logic;
///
/// counter_logic(items: &[T]);
/// ```

fn counter_logic<T: Eq + std::hash::Hash + Copy>(
    items: &[T],
) -> HashMap<T, u32> {
    let mut frequency_map: HashMap<T, u32> = HashMap::new();
    for &item in items {
        let count = frequency_map.entry(item).or_insert(0);
        *count += 1;
    }
    frequency_map
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `counter_logic` function.
    ///
    /// This test checks that the `counter_logic` function correctly counts the frequency of items in a list.

    #[test]
    fn test_frequency_counter() {
        let items = vec![9, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3, 5];
        let frequency_map = counter_logic(&items);

        let mut sorted_frequency_map = Vec::new();
        let mut seen = HashSet::new();

        for &item in &items {
            if !seen.contains(&item) {
                if let Some(&frequency) = frequency_map.get(&item) {
                    sorted_frequency_map.push((item, frequency));
                    seen.insert(item);
                }
            }
        }

        let expected_sorted_frequency_map = vec![
            (9, 3),
            (1, 2),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 2),
            (6, 1),
            (7, 1),
            (8, 1),
        ];

        assert_eq!(sorted_frequency_map, expected_sorted_frequency_map);
        assert_eq!(sorted_frequency_map.len(), expected_sorted_frequency_map.len());
    }
}