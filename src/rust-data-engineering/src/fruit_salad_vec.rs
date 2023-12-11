/// The `fruit_salad_vec` module provides functionality for creating a random fruit salad.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use fruit_salad_vec::fruit_salad_vec;
///
/// fruit_salad_vec();
/// ```
///
/// # Note
///
/// This module uses the `rand` crate for randomizing the order of fruits.
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Prints a banner and calls the `random_salad` function to create a random fruit salad.
pub fn fruit_salad_vec() {
    println!("####### FRUIT SALAD VEC #######");
    random_salad(1);
    random_salad(4);
    random_salad(999);
}

/// Creates a random fruit salad.
///
/// This function initializes a `Vec` with a list of fruits, shuffles them,
/// and then prints the first four fruits in the shuffled list.
///
/// You can specify how many fruits are included in the salad.
///
/// # Examples
///
/// ```
/// use fruit_salad_vec::random_salad;
///
/// random_salad(salad_size: usize);
/// ```

fn random_salad(salad_size: usize) {
    let mut rng = thread_rng();
    let mut fruits = vec![
        "apple",
        "banana",
        "cherry",
        "durian",
        "elderberry",
        "fig",
        "grapefruit",
        "honeydew",
        "ice cream bean",
    ];

    fruits.push("jujube");
    fruits.push("pomegranate");
    fruits.push("jackfruit");
    fruits.push("kiwi");

    fruits.shuffle(&mut rng);


    let max_count = if fruits.len() <= salad_size {
        fruits.len()
    } else {
        salad_size
    };

    let salad = &fruits[0..max_count];

    for (i, fruit) in salad.iter().enumerate() {
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
