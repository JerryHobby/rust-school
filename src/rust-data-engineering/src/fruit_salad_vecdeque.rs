/// The `fruit_salad_vecdeque` module provides functionality for creating a random fruit salad.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use fruit_salad_vecdeque::fruit_salad_vecdeque;
///
/// fruit_salad_vecdeque();
/// ```
///
/// # Note
///
/// This module uses the `rand` crate for randomizing the order of fruits.
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

/// Prints a banner and calls the `random_salad` function to create a random fruit salad.
pub fn fruit_salad_vecdeque() {
    println!("####### FRUIT SALAD VECDEQUE #######");
    random_salad(1);
    random_salad(4);
    random_salad(999);
}

/// Creates a random fruit salad.
///
/// This function initializes a `VecDeque` with a list of fruits, shuffles them,
/// and then prints the first four fruits in the shuffled list.
///
/// You can specify how many fruits are included in the salad.
///
/// # Examples
///
/// ```
/// use fruit_salad::random_salad;
///
/// random_salad(salad_size: usize);
/// ```

fn random_salad(salad_size: usize) {
    let mut rng = thread_rng();
    let mut fruits: VecDeque<&str> = VecDeque::new();

    fruits.push_back("apple");
    fruits.push_back("banana");
    fruits.push_back("cherry");
    fruits.push_back("durian");
    fruits.push_back("elderberry");
    fruits.push_back("fig");
    fruits.push_back("grapefruit");
    fruits.push_back("honeydew");
    fruits.push_back("ice cream bean");

    let mut fruits: Vec<&str> = fruits.into_iter().collect();

    fruits.shuffle(&mut rng);

    let mut fruits: VecDeque<_> = fruits.into_iter().collect();

    fruits.push_front("pomegranate");
    fruits.push_back("jackfruit");
    fruits.push_back("kiwi");

    let max_count = if fruits.len() <= salad_size {
        fruits.len()
    } else {
        salad_size
    };

    for (i, fruit) in fruits.iter().enumerate() {
        if i == max_count {
            break;
        }

        if i == max_count - 1 {
            println!("{}", fruit);
        } else {
            print!("{}, ", fruit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_salad_0() {
        random_salad(0);
    }
    #[test]
    fn test_random_salad_5() {
        random_salad(5);
    }
    #[test]
    fn test_random_salad_999() {
        random_salad(999);
    }
}
