pub fn iter_str() {
    let my_str = "A brown dog jumps over the lazy moon.";

    for c in my_str.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("    Found a vowel: {}", c),
            ' ' | '.' | '?' | ',' => println!("    punct / space: {}", c),
            _ => println!("Rats! A consonant: {}", c),
        }
    }
}

pub fn iter_string() {
    let my_str = "A brown dog jumps over the lazy moon.".to_string();

    for c in my_str.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("    Found a vowel: {}", c),
            ' ' | '.' | '?' | ',' => println!("    punct / space: {}", c),
            _ => println!("Rats! A consonant: {}", c),
        }
    }
}

#[allow(unused_variables)]

pub fn iter_words() {
    // String version
    let sentence = "A brown dog jumps over the lazy moon.".to_string();
    let words: Vec<&str> = sentence.split_ascii_whitespace().collect();

    // &str version
    let sentence = "A brown dog jumps over the lazy moon.";
    let words = sentence.split_ascii_whitespace();

    for c in words.clone() {
        println!("word: {}", c);
    }
}
