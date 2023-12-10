use std::ops::Add;

#[allow(dead_code)]

pub fn run() {
    let mut xi32 = 5;

    no_modify_int(xi32);
    println!("x is {}", xi32);

    xi32 = modify_int(xi32);
    println!("x is {}", xi32);

    let mut xstring = "hello".to_string();
    no_modify_str(&xstring);
    println!("x is {}", xstring);

    xstring = modify_str(&xstring);
    println!("x is {}", xstring);
}

fn no_modify_int(a: i32) {
    //  a = 4; //-- can't be done
    println!("nothing changes: {}.", a);
}

fn modify_int(mut a: i32) -> i32 {
    a -= 1;
    println!("decrementing: {}.", a);
    a
}

fn no_modify_str(a: &String) {
    //  a = 4; //-- can't be done
    println!("nothing changes: {}.", a);
}

fn modify_str(a: &String) -> String {
    let b = a.clone().add(" world"); //-- can't be done
    println!("string changes: {}.", b);
    b.to_string()
}
