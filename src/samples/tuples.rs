pub fn my_tuples() {
    let tuple = (1, 2, 3);
    let (a, b, c) = tuple;
    println!("a = {}, b = {}, c = {}", a, b, c);

    let tuple = (1, 2, 3);
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;
    println!("a = {}, b = {}, c = {}", a, b, c);

    let tuple = (1, 2, 3);
    show_parts(tuple);
}

fn show_parts(props: (i32, i32, i32)) {
    println!("a = {}, b = {}, c = {}", props.0, props.1, props.2);
}

struct Props {
    a: i32,
    b: i32,
    c: i32,
}

pub fn destruct_struct() {
    let props = Props { a: 1, b: 2, c: 3 };
    let Props { a, b, c } = props;
    println!("a = {}, b = {}, c = {}", a, b, c);

    let props = Props { a: 1, b: 2, c: 3 };
    let a = props.a;
    let b = props.b;
    let c = props.c;
    println!("a = {}, b = {}, c = {}", a, b, c);

    let props = Props { a: 1, b: 2, c: 3 };
    show_props(props);
}

fn show_props(p0: Props) {
    println!("a = {}, b = {}, c = {}", p0.a, p0.b, p0.c);
}
