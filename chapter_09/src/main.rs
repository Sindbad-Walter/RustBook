use std::fs::File;
use std::io::{self, ErrorKind, Read};

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", &value);
        }

        Guess { value: value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file {:?}", other_error)
    //         }
    //     },
    // };
    let my_guess = Guess::new(12);
    println!("{:?}", my_guess);

    let point1 = Point { x: 12, y: 50 };

    let point2 = Point {
        x: String::from("test"),
        y: 12.2,
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
