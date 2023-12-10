// these functions demonstrate some rust data type features
pub fn declarations() {
    let x = 5;
    let y = 6;
    let z = x + y;
    println!("z = {}", z);
    // Declare array, initialize all values, compiler infers length = 7
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    println!("days = {:?}", &days);

    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];
    println!("bytes = {:?}", bytes);
}

pub fn check_none() {
    println!("\n#### check_none() ####");

    let absent_number: Option<i32> = None;

    println!("absent_number: {:?}", absent_number);
}

pub fn check_some() {
    println!("\n#### check_some() ####");

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
}

pub fn some_number() {
    // let maybe_number = Some(42);
    let maybe_number: Option<Option<()>> = Some(None);

    if let Some(number) = maybe_number {
        println!("The number is: {:?}", number);
    } else {
        println!("The number is: None");
    }
}

pub fn hash_maps() {
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );

    println!("reviews: {:#?}", reviews);
    println!("reviews: {:?}", reviews.get("Ancient Roman History"));
}

pub fn arrays_vectors_etc() {
    // tuples .. fixed size on the stack
    let tuple = (1, "hello", 4.5, true);
    println!("tuple: {:?}", tuple);

    // arrays .. fixed size on the stack
    let array = [1, 2, 3, 4, 5];
    println!("array: {:?}", array);

    // vectors .. variable size on the heap
    let vector = vec![1, 2, 3, 4, 5];
    println!("vector: {:?}", vector);

    // slices .. reference to a subset of a vector
    let slice = &vector[1..3];
    println!("slice: {:?}", slice);

    // hash maps .. variable size on the heap
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("one", 1);
    hash_map.insert("two", 2);
    hash_map.insert("three", 3);
    println!("hash_map: {:?}", hash_map);

    // multi-dimensional tuples
    let tuple = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    println!("tuple: {:?}", tuple);

    // multi-dimensional arrays
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("matrix: {:?}", matrix);

    // multi-dimensional vectors
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("matrix: {:?}", matrix);

    // multi-dimensional hash maps
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("one", vec![1, 2, 3]);
    hash_map.insert("two", vec![4, 5, 6]);
    hash_map.insert("three", vec![7, 8, 9]);
    println!("hash_map: {:?}", hash_map);
}
