use std::io;
use std::io::Write;
use std::thread::park_timeout;

static MSG: [&str; 11] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];
static LOOP_SIZE: usize = 11;

pub fn for_loop_counter() {
    println!("\n#### for_loop_counter() ####");

    // for x in 0..LOOP_SIZE {
    // for x in 0..=LOOP_SIZE -1 {
    // for x in MSG {

    for x in 0..LOOP_SIZE {
        println!("for: x[{:02}] {}", x, MSG[x]);
    }

    for x in MSG.iter().rev() {
        println!("for MSG rev:  {}", x);
    }

    if MSG.iter().find(|x| *x == &"one") == Some(&"one") {
        println!("for MSG find:  {}", "one");
    }

    MSG.map(|x| println!("for MSG.map:  {}", x));
}

pub fn loop_counter() {
    let mut x = 0;
    println!("\n#### loop_counter() ####");
    loop {
        println!("loop: x[{:02}] {}", x, MSG[x]);
        x = x + 1;
        if x >= LOOP_SIZE {
            break;
        }
    }
}

pub fn while_counter() {
    let mut x = 0;
    println!("\n#### while_counter() ####");
    while x < LOOP_SIZE {
        println!("while: x[{:02}] {}", x, MSG[x]);
        x = x + 1;
    }
}

pub fn input_loop() {
    let mut input = String::new();
    println!("\n#### input_loop() ####");

    while input.trim() != "q" {
        input.clear();
        print!("Enter text.  (q) to quit: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You entered: {}", input.trim());
    }
}

pub fn match_demo() {
    let mut input = String::new();
    println!("\n#### match_demo() ####");

    while input.trim() != "q" {
        input.clear();
        print!("Enter 1 - 10.  (q) to quit: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        print!("You entered: ");

        match input.trim() {
            "1" => println!("one"),
            "2" => println!("two"),
            "3" => println!("three"),
            "4" => println!("four"),
            "5" => println!("five"),
            "q" => {
                println!("quitting");
                break;
            }
            _ => {
                println!("{}", input.trim());
                park_timeout(std::time::Duration::from_millis(1000));
            }
        }
    }
}
