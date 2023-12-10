use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn fruit_salad() {
    println!("####### FRUIT SALAD #######");
    random_salad();
}

fn random_salad() {
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
        "jackfruit",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "pineapple",
        "quince",
        "raspberry",
        "strawberry",
        "tangerine",
        "ugli fruit",
        "victoria plum",
        "watermelon",
        "xigua",
        "yellow passionfruit",
        "star fruit",
    ];

    fruits.shuffle(&mut rng);
    let fruit_salad = &fruits[0..3];

    for (i,fruit) in fruit_salad.iter().enumerate() {
        if i == fruit_salad.len() - 1 {
            println!("{}", fruit);
        } else {
            print!("{}, ", fruit);
        }
    }
}
