fn main() {
    let b = Box::new(6);
    println!("b = {}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
