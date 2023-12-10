pub fn basic_array_types() {
    println!("####### SEQUENCE #######");
    sequence();
    println!("####### HASH MAP #######");
    hash_map();
    println!("####### SETS #######");
    sets();
}

fn sequence() {
    let mut sequence = vec![1, 1];

    for i in 2..10 {
        let next = sequence[i - 1] + sequence[i - 2];
        sequence.push(next);
    }

    println!("The sequence is: {:?}", sequence);
}

fn hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("The scores are: {:?}", scores);
}

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
